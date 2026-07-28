//! Poseidon hash permutations, wrapped from STWO's `examples::poseidon` AIR.

use stwo_prover::constraint_framework::logup::LookupElements;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::simd::fft::MIN_FFT_LOG_SIZE;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::IntoSlice;
use stwo_prover::core::pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{
    prove, verify, ProvingError, StarkProof, VerificationError, LOG_BLOWUP_FACTOR,
};
use stwo_prover::core::vcs::blake2_hash::{Blake2sHash, Blake2sHasher};
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::core::vcs::ops::MerkleHasher;
use stwo_prover::core::InteractionElements;
use stwo_prover::examples::poseidon::{gen_interaction_trace, gen_trace, PoseidonComponent};
use wasm_bindgen::prelude::*;

use crate::StwoResult;

/// Instances are packed `2^N_LOG_INSTANCES_PER_ROW` per trace row.
pub const N_LOG_INSTANCES_PER_ROW: usize = 3;
/// Blowup of the constraint evaluation domain.
pub const LOG_EXPAND: u32 = 2;
/// SIMD lane count of the Poseidon AIR.
pub const LOG_N_LANES: u32 = 4;

/// Minimum `log_n_rows` accepted by the AIR: wide enough for SIMD lanes,
/// the FFT, and the interaction trace.
pub const MIN_LOG_N_ROWS: u32 = {
    let lanes_floor = LOG_N_LANES + 2;
    if lanes_floor > MIN_FFT_LOG_SIZE {
        lanes_floor
    } else {
        MIN_FFT_LOG_SIZE
    }
};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// A self-contained Poseidon prover/verifier for `2^log_n_instances` permutations.
#[derive(Clone)]
pub struct PoseidonStruct {
    pub component: PoseidonComponent,
}

impl PoseidonStruct {
    /// Builds the AIR for `2^log_n_instances` Poseidon permutations.
    ///
    /// `log_n_rows = log_n_instances - N_LOG_INSTANCES_PER_ROW` must be at
    /// least [`MIN_LOG_N_ROWS`].
    ///
    /// The public statement (the LogUp claimed sum) is reconstructed by
    /// replaying the prover's Fiat–Shamir transcript, so a verifier can build
    /// the exact same AIR from `log_n_instances` alone.
    pub fn new(log_n_instances: u32) -> Result<Self, String> {
        if (log_n_instances as usize) < N_LOG_INSTANCES_PER_ROW {
            return Err(format!(
                "log_n_instances must be at least {N_LOG_INSTANCES_PER_ROW}"
            ));
        }
        let log_n_rows = log_n_instances - N_LOG_INSTANCES_PER_ROW as u32;
        if log_n_rows < MIN_LOG_N_ROWS {
            return Err(format!(
                "log_n_rows ({log_n_rows}) must be at least {MIN_LOG_N_ROWS}; \
                 increase log_n_instances"
            ));
        }

        // Replay the prover's transcript: commit the base trace, then draw the
        // lookup elements, so the claimed sum matches the one the prover proves.
        let twiddles = SimdBackend::precompute_twiddles(
            CanonicCoset::new(log_n_rows + LOG_EXPAND + LOG_BLOWUP_FACTOR)
                .circle_domain()
                .half_coset,
        );
        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let commitment_scheme = &mut CommitmentSchemeProver::new(LOG_BLOWUP_FACTOR, &twiddles);
        let (trace, lookup_data) = gen_trace(log_n_rows);
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(trace);
        tree_builder.commit(channel);
        let lookup_elements = LookupElements::draw(channel);
        let (_, claimed_sum) = gen_interaction_trace(log_n_rows, lookup_data, &lookup_elements);

        Ok(Self {
            component: PoseidonComponent {
                log_n_rows,
                lookup_elements,
                claimed_sum,
            },
        })
    }

    /// Generates the STARK proof for the Poseidon trace.
    pub fn prove<H: MerkleHasher<Hash = Blake2sHash>>(
        &self,
    ) -> Result<StarkProof<Blake2sMerkleHasher>, ProvingError> {
        let log_n_rows = self.component.log_n_rows;

        // Precompute twiddles.
        let twiddles = SimdBackend::precompute_twiddles(
            CanonicCoset::new(log_n_rows + LOG_EXPAND + LOG_BLOWUP_FACTOR)
                .circle_domain()
                .half_coset,
        );

        // Setup protocol.
        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let commitment_scheme = &mut CommitmentSchemeProver::new(LOG_BLOWUP_FACTOR, &twiddles);

        // Base trace.
        let (trace, lookup_data) = gen_trace(log_n_rows);
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(trace);
        tree_builder.commit(channel);

        // Draw lookup elements — same transcript point as in `new`.
        let lookup_elements = LookupElements::draw(channel);

        // Interaction trace.
        let (trace, claimed_sum) = gen_interaction_trace(log_n_rows, lookup_data, &lookup_elements);
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(trace);
        tree_builder.commit(channel);

        let component = PoseidonComponent {
            log_n_rows,
            lookup_elements,
            claimed_sum,
        };
        prove(
            &[&component],
            channel,
            &InteractionElements::default(),
            commitment_scheme,
        )
    }

    /// Verifies a proof generated by [`PoseidonStruct::prove`], driving the
    /// verifier's commitment scheme through the same transcript as the prover.
    pub fn verify<H: MerkleHasher<Hash = Blake2sHash>>(
        &self,
        proof: StarkProof<H>,
    ) -> Result<(), VerificationError> {
        let log_n_rows = self.component.log_n_rows;
        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let commitment_scheme = &mut CommitmentSchemeVerifier::new();

        // Commit the base trace root from the proof, then draw the lookup
        // elements at the same transcript point as the prover.
        let sizes = self.component.trace_log_degree_bounds();
        commitment_scheme.commit(proof.commitments[0], &sizes[0], channel);
        let lookup_elements = LookupElements::draw(channel);
        commitment_scheme.commit(proof.commitments[1], &sizes[1], channel);

        let component = PoseidonComponent {
            log_n_rows,
            lookup_elements,
            claimed_sum: self.component.claimed_sum,
        };
        verify(
            &[&component],
            channel,
            &InteractionElements::default(),
            commitment_scheme,
            proof,
        )
    }
}

#[wasm_bindgen]
pub fn prove_stark_proof_poseidon(log_n_instances: u32) -> StwoResult {
    match PoseidonStruct::new(log_n_instances) {
        Ok(poseidon) => match poseidon.prove::<Blake2sMerkleHasher>() {
            Ok(proof) => {
                console_log!("Proof generated successfully");
                match poseidon.verify::<Blake2sMerkleHasher>(proof) {
                    Ok(()) => {
                        console_log!("Proof verified successfully");
                        StwoResult {
                            success: true,
                            message: "Proof verified successfully".to_string(),
                        }
                    }
                    Err(e) => {
                        console_log!("Proof verification failed: {:?}", e);
                        StwoResult {
                            success: false,
                            message: format!("Proof verification failed: {:?}", e),
                        }
                    }
                }
            }
            Err(e) => {
                console_log!("Proof generation failed: {:?}", e);
                StwoResult {
                    success: false,
                    message: format!("Proof generation failed: {:?}", e),
                }
            }
        },
        Err(e) => {
            console_log!("Invalid inputs: {:?}", e);
            StwoResult {
                success: false,
                message: format!("Invalid inputs: {:?}", e),
            }
        }
    }
}

#[wasm_bindgen]
pub fn verify_stark_proof_poseidon(log_n_instances: u32, stark_proof_str: &str) -> StwoResult {
    let stark_proof: Result<StarkProof<Blake2sMerkleHasher>, serde_json::Error> =
        serde_json::from_str(stark_proof_str);
    let proof = match stark_proof {
        Ok(proof) => proof,
        Err(e) => {
            console_log!("Failed to deserialize proof: {:?}", e);
            return StwoResult {
                success: false,
                message: format!("Failed to deserialize proof: {:?}", e),
            };
        }
    };

    match PoseidonStruct::new(log_n_instances) {
        Ok(poseidon) => match poseidon.verify::<Blake2sMerkleHasher>(proof) {
            Ok(()) => {
                console_log!("Proof verified successfully");
                StwoResult {
                    success: true,
                    message: "Proof verified successfully".to_string(),
                }
            }
            Err(e) => {
                console_log!("Proof verification failed: {:?}", e);
                StwoResult {
                    success: false,
                    message: format!("Proof verification failed: {:?}", e),
                }
            }
        },
        Err(e) => {
            console_log!("Invalid inputs: {:?}", e);
            StwoResult {
                success: false,
                message: format!("Invalid inputs: {:?}", e),
            }
        }
    }
}
