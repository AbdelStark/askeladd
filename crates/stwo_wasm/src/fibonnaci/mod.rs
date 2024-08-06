// mod.rs fibonnaci
use self::component::FibonacciComponent;
pub mod air;
pub mod component;
pub mod multi_fibonacci;

use air::FibonacciAir;
use num_traits::One;
use serde::{Deserialize, Serialize};
use stwo_prover::core::backend::cpu::CpuCircleEvaluation;
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::{FieldExpOps, IntoSlice};
use stwo_prover::core::pcs::CommitmentSchemeProver;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation, PolyOps};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::prover::{
    prove,
    // verify,
    ProvingError,
    StarkProof,
    VerificationError,
    LOG_BLOWUP_FACTOR,
};
use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::core::InteractionElements;
use stwo_prover::trace_generation::{
    // commit_and_prove,
    commit_and_verify,
};
// use stwo_prover::core::pcs::{ CommitmentSchemeVerifier};

// use stwo_prover::trace_generation::{commit_and_prove, commit_and_verify};
use wasm_bindgen::prelude::*;

// use num_traits::One;

// use self::air::{FibonacciAir, MultiFibonacciAir};

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

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct StwoResult {
    success: bool,
    message: String,
}

#[wasm_bindgen]
impl StwoResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

#[derive(Clone)]
pub struct Fibonacci {
    pub component: FibonacciComponent,
    pub air: FibonacciAir,
}

impl Fibonacci {
    pub fn new(log_size: u32, claim: BaseField) -> Self {
        let component = FibonacciComponent::new(log_size, claim);
        let air = FibonacciAir {
            component: component.clone(),
        };
        Self {
            component: component.clone(),
            air,
        }
    }

    pub fn get_trace(&self) -> CpuCircleEvaluation<BaseField, BitReversedOrder> {
        // Trace.
        let trace_domain = CanonicCoset::new(self.component.log_size);
        let mut trace = Vec::with_capacity(trace_domain.size());

        // Fill trace with fibonacci squared.
        let mut a = BaseField::one();
        let mut b = BaseField::one();
        for _ in 0..trace_domain.size() {
            trace.push(a);
            let tmp = a.square() + b.square();
            a = b;
            b = tmp;
        }

        // Returns as a CircleEvaluation.
        CircleEvaluation::new_canonical_ordered(trace_domain, trace)
    }

    pub fn prove(&self) -> Result<StarkProof<Blake2sMerkleHasher>, ProvingError> {
        println!("channel");

        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[self
            .component
            .claim])));
        println!("twiddles");

        let twiddles = CpuBackend::precompute_twiddles(
            CanonicCoset::new(self.component.log_size)
                .circle_domain()
                .half_coset,
        );
        println!("commitment_scheme");

        let commitment_scheme = &mut CommitmentSchemeProver::new(LOG_BLOWUP_FACTOR, &twiddles);
        println!("get trace");

        // let trace = self.get_trace();

        println!("trace_domain");
        // let trace_domain = CanonicCoset::new(self.component.log_size, self.component.claim);

        // let trace = trace
        // .into_iter()
        // .map(|eval| CpuCircleEvaluation::new_canonical_ordered(trace_domain, eval))
        // .collect_vec();

        // let trace = self.get_trace();
        // let trace = self.get_trace();
        // let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[self
        //     .air
        //     .component
        //     .claim])));

        // commit_and_prove(&self.air, channel, trace);
        // commit_and_prove(&self.air, channel, vec![trace]);

        let proof = prove(
            &[&self.component],
            channel,
            &InteractionElements::default(),
            commitment_scheme, // vec![trace],
        )
        .map_err(Err::<StarkProof<Blake2sMerkleHasher>, ProvingError>);

        match proof {
            Ok(p) => Ok(p),
            Err(_) => Err(ProvingError::ConstraintsNotSatisfied),
        }
    }

    pub fn verify(&self, proof: StarkProof<Blake2sMerkleHasher>) -> Result<(), VerificationError> {
        // let verifier_channel =
        //     &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        // let commitment_scheme = &mut CommitmentSchemeVerifier::new();
        // verify(
        //     &[&self.component],
        //     verifier_channel,
        //     &InteractionElements::default(),
        //     commitment_scheme,
        //     proof,
        // )

        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[self
            .air
            .component
            .claim])));
        commit_and_verify(proof, &self.air, channel)
    }
}

#[wasm_bindgen]
pub fn prove_and_verify(log_size: u32, claim: u32) -> StwoResult {
    console_log!(
        "Starting prove_and_verify with log_size: {}, claim: {}",
        log_size,
        claim
    );
    let fib = Fibonacci::new(log_size, BaseField::from(claim));

    match fib.prove() {
        Ok(proof) => {
            console_log!("Proof generated successfully");
            let serialized = serde_json::to_string(&proof).unwrap();
            console_log!("Serialized proof: {}", serialized);

            match fib.verify(proof) {
                Ok(_) => {
                    console_log!("Proof verified successfully");
                    StwoResult {
                        success: true,
                        message: serialized.to_string(),
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
    }
}

#[wasm_bindgen]
pub fn verify_stark_proof(log_size: u32, claim: u32, stark_proof_str: &str) -> StwoResult {
    console_log!(
        "Starting verify_stark_proof with log_size: {}, claim: {}",
        log_size,
        claim
    );
    console_log!("Received proof string length: {}", stark_proof_str.len());

    let fib = Fibonacci::new(log_size, BaseField::from(claim));

    let stark_proof: Result<StarkProof<Blake2sMerkleHasher>, serde_json::Error> =
        serde_json::from_str(stark_proof_str);

    match stark_proof {
        Ok(proof) => {
            console_log!("Proof deserialized successfully");
            match fib.verify(proof) {
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
    }
}
