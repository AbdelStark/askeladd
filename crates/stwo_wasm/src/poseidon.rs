use stwo_prover::constraint_framework::constant_columns::gen_is_first;
// lib.rs
use stwo_prover::constraint_framework::logup::LookupElements;
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
use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
use stwo_prover::core::vcs::hasher::Hasher;
use stwo_prover::core::InteractionElements;
use stwo_prover::examples::poseidon::{
    self,
    gen_interaction_trace,
    gen_trace,
    PoseidonAir,
    PoseidonComponent, //  PoseidonComponent,
};
use wasm_bindgen::prelude::*;

use crate::StwoResult;

pub const N_LOG_INSTANCES_PER_ROW: usize = 3;
pub const LOG_N_ROWS: u32 = 8;
pub const LOG_EXPAND: u32 = 2;
pub const LOG_N_LANES: u32 = 4;

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

#[derive(Clone)]
pub struct PoseidonStruct {
    pub air: PoseidonAir,
}

impl PoseidonStruct {
    pub fn new(log_n_instances: u32) -> Result<Self, String> {
        if log_n_instances < N_LOG_INSTANCES_PER_ROW as u32 {
            return Err("log_n_instances < N_LOG_INSTANCES_PER_ROW".to_string());
        }

        let log_n_rows = log_n_instances - N_LOG_INSTANCES_PER_ROW as u32;

        if log_n_rows < LOG_N_LANES {
            return Err("log_n_rows < LOG_N_LANES".to_string());
        }

        if log_n_rows < MIN_FFT_LOG_SIZE && log_n_instances < MIN_FFT_LOG_SIZE {
            println!(
                "log_n_elements >= MIN_FFT_LOG_SIZE as usize {}",
                log_n_rows >= LOG_N_LANES
            );
            return Err("llog_n_elements >= MIN_FFT_LOG_SIZE as usize".to_string());
        }

        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));

        let log_n_rows = log_n_instances - N_LOG_INSTANCES_PER_ROW as u32;

        // Draw lookup element.
        let lookup_elements = LookupElements::draw(channel);
        // let component = PoseidonComponent {
        // Precompute twiddles.
        let twiddles = SimdBackend::precompute_twiddles(
            CanonicCoset::new(log_n_rows + LOG_EXPAND + LOG_BLOWUP_FACTOR)
                .circle_domain()
                .half_coset,
        );

        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let commitment_scheme = &mut CommitmentSchemeProver::new(LOG_BLOWUP_FACTOR, &twiddles);

        // Trace.
        let (trace, lookup_data) = gen_trace(log_n_rows);
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(trace);
        tree_builder.commit(channel);

        // let (trace0, interaction_data) = gen_trace(LOG_N_ROWS);
        // let (trace1, claimed_sum) =
        //     gen_interaction_trace(LOG_N_ROWS, interaction_data, lookup_elements);
        let (trace, claimed_sum) = gen_interaction_trace(log_n_rows, lookup_data, lookup_elements);

        let component = PoseidonComponent {
            log_n_rows,
            lookup_elements,
            claimed_sum, // claimed_sum,
        };
        let air = PoseidonAir { component };

        Ok(Self { air })
    }
    pub fn prove(&self) -> Result<StarkProof, ProvingError> {
        // let (trace, lookup_data) = gen_trace(self.air.component.log_n_rows);
        let res = PoseidonStruct::prove_poseidon(self.air.component.log_n_rows);

        match res {
            Ok(proof) => Ok(proof),
            Err(e) => Err(ProvingError::ConstraintsNotSatisfied),
        }

        // if let Some(proof) = PoseidonStruct::prove_poseidon(self.air.component.log_n_rows) {
        //     Ok(proof)
        // } else {
        //     Err(ProvingError::ConstraintsNotSatisfied)
        // }
    }

    // @TODO handle correctly error to not crash
    pub fn prove_poseidon(log_n_instances: u32) -> Result<StarkProof, String> {
        if log_n_instances < N_LOG_INSTANCES_PER_ROW as u32 {
            return Err("log_n_rows < LOG_N_LANES".to_string());
        }

        let log_n_rows = log_n_instances - N_LOG_INSTANCES_PER_ROW as u32;

        if log_n_rows < LOG_N_LANES {
            return Err("log_n_rows < LOG_N_LANES".to_string());
        }

        if log_n_rows < MIN_FFT_LOG_SIZE && log_n_instances < MIN_FFT_LOG_SIZE {
            println!(
                "log_n_elements >= MIN_FFT_LOG_SIZE as usize {}",
                log_n_rows >= LOG_N_LANES
            );
            return Err("llog_n_elements >= MIN_FFT_LOG_SIZE as usize".to_string());
        }

        // Precompute twiddles.
        // let span = span!(Level::INFO, "Precompute twiddles").entered();
        let twiddles = SimdBackend::precompute_twiddles(
            CanonicCoset::new(log_n_rows + LOG_EXPAND + LOG_BLOWUP_FACTOR)
                .circle_domain()
                .half_coset,
        );

        // Setup protocol.
        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let commitment_scheme = &mut CommitmentSchemeProver::new(LOG_BLOWUP_FACTOR, &twiddles);

        // Trace.
        let (trace, lookup_data) = gen_trace(log_n_rows);
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(trace);
        tree_builder.commit(channel);

        // Draw lookup element.
        let lookup_elements = LookupElements::draw(channel);

        // Interaction trace.
        let (trace, claimed_sum) = gen_interaction_trace(log_n_rows, lookup_data, lookup_elements);
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(trace);
        tree_builder.commit(channel);

        // Constant trace.
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(vec![gen_is_first(log_n_rows)]);
        tree_builder.commit(channel);

        // Prove constraints.
        let component = PoseidonComponent {
            log_n_rows,
            lookup_elements,
            claimed_sum,
        };
        let air = PoseidonAir { component };
        let proof = prove::<SimdBackend>(
            &air,
            channel,
            &InteractionElements::default(),
            commitment_scheme,
        )
        .unwrap();

        Ok(proof)
    }

    pub fn verify(&self, proof: StarkProof) -> Result<(), VerificationError> {
        let verifier_channel =
            &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let commitment_scheme = &mut CommitmentSchemeVerifier::new();

        verify(
            &self.air,
            verifier_channel,
            &InteractionElements::default(),
            commitment_scheme,
            proof,
        )
    }
}

#[wasm_bindgen]
pub fn prove_stark_proof_poseidon(log_n_instances: u32) -> StwoResult {
    let poseidon = PoseidonStruct::new(log_n_instances);
    match poseidon {
        Ok(poseidon) => match poseidon.prove() {
            Ok(proof) => {
                console_log!("Proof deserialized successfully");
                match poseidon.verify(proof) {
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
                console_log!("Failed to deserialize proof: {:?}", e);
                StwoResult {
                    success: false,
                    message: format!("Failed to deserialize proof: {:?}", e),
                }
            }
        },
        Err(e) => StwoResult {
            success: false,
            message: format!("Failed to deserialize proof: {:?}", e),
        },
    }
}

#[wasm_bindgen]
pub fn verify_stark_proof_poseidon(log_n_instances: u32, stark_proof_str: &str) -> StwoResult {
    let poseidon = PoseidonStruct::new(log_n_instances);

    let stark_proof: Result<StarkProof, serde_json::Error> = serde_json::from_str(stark_proof_str);
    if let p = poseidon.unwrap() {
        match p.verify(stark_proof.unwrap()) {
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
    } else {
        StwoResult {
            success: false,
            message: "Proof verified successfully".to_string(),
        }
    }
}
