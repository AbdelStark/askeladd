// lib.rs
use stwo_prover::core::backend::cpu::CpuCircleEvaluation;
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::{self, BaseField};
use stwo_prover::core::fields::IntoSlice;
use stwo_prover::core::poly::circle::CanonicCoset;
use stwo_prover::core::prover::{prove, ProvingError, StarkProof, VerificationError};
use stwo_prover::core::vcs::blake2_hash::{Blake2sHash, Blake2sHasher};
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::core::vcs::ops::MerkleHasher;
use stwo_prover::core::InteractionElements;
use stwo_prover::examples::wide_fibonacci::component::{
    Input, WideFibAir, WideFibComponent, LOG_N_COLUMNS,
};
use stwo_prover::examples::wide_fibonacci::constraint_eval::gen_trace;
use stwo_prover::trace_generation::{commit_and_prove, commit_and_verify};
use wasm_bindgen::prelude::*;

use crate::StwoResult;

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

pub trait WideFairImpl {
    fn verify<H: MerkleHasher>(proof: StarkProof<H>) -> Result<(), VerificationError>;
    fn prove<H: MerkleHasher>(
        log_fibonacci_size: u32,
        log_n_instances: u32,
    ) -> Result<StarkProof<H>, ProvingError>;
}

#[derive(Clone)]
pub struct WideFibStruct {
    pub air: WideFibAir,
}

impl WideFibStruct {
    pub fn new(log_fibonacci_size: u32, log_n_instances: u32) -> Self {
        let component = WideFibComponent {
            log_fibonacci_size: log_fibonacci_size + LOG_N_COLUMNS as u32,
            log_n_instances,
        };
        let wide_fib = WideFibAir {
            component: component.clone(),
        };
        Self { air: wide_fib }
    }
    pub fn prove<H: MerkleHasher<Hash = Blake2sHash>>(
        &self,
    ) -> Result<StarkProof<Blake2sMerkleHasher>, ProvingError> {
        let private_input = (0..(1 << self.air.component.log_n_instances))
            .map(|i| Input {
                a: m31::M31::from_u32_unchecked(i),
                b: m31::M31::from_u32_unchecked(i),
                // b: m31!(i),
            })
            .collect();
        // let trace = wide_fib.get_trace();
        let trace = gen_trace(&self.air.component.clone(), private_input);
        let trace_domain = CanonicCoset::new(self.air.component.log_column_size());
        let trace = trace
            .into_iter()
            .map(|eval| CpuCircleEvaluation::new_canonical_ordered(trace_domain, eval))
            .collect();
        let prover_channel =
            &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        // let res_proof = commit_and_prove::<CpuBackend>(&self.air, prover_channel, trace);
        let res_proof: Result<StarkProof<Blake2sMerkleHasher>, ProvingError> =
            commit_and_prove(&self.air, prover_channel, trace);
        // let res_proof = prove(
        //     &self.air,
        //     prover_channel,
        //     &InteractionElements::default(),
        //     None,
        // )
        // .map_err(|op| Err::<StarkProof<Blake2sMerkleHasher>, ProvingError>(op));

        match res_proof {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
        // res_proof
    }

    pub fn verify<H: MerkleHasher>(
        &self,
        proof: StarkProof<Blake2sMerkleHasher>,
    ) -> Result<(), VerificationError> {
        let verifier_channel =
            &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        commit_and_verify(proof, &self.air, verifier_channel)
    }
}

#[wasm_bindgen]
pub fn stark_proof_wide_fibo(log_fibonacci_size: u32, log_n_instances: u32) -> StwoResult {
    let component = WideFibComponent {
        log_fibonacci_size: log_fibonacci_size + LOG_N_COLUMNS as u32,
        log_n_instances,
    };

    let wide_fib_air = WideFibAir {
        component: component.clone(),
    };

    let wide_fib = WideFibStruct { air: wide_fib_air };
    match wide_fib.prove::<Blake2sMerkleHasher>() {
        Ok(proof) => {
            console_log!("Proof deserialized successfully");
            match wide_fib.verify::<Blake2sMerkleHasher>(proof) {
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

#[wasm_bindgen]
pub fn verify_stark_proof_wide_fibo(
    log_fibonacci_size: u32,
    log_n_instances: u32,
    stark_proof_str: &str,
) -> StwoResult {
    let component = WideFibComponent {
        log_fibonacci_size: log_fibonacci_size + LOG_N_COLUMNS as u32,
        log_n_instances,
    };

    let wide_fib_air = WideFibAir {
        component: component.clone(),
    };

    let wide_fib = WideFibStruct { air: wide_fib_air };

    let stark_proof: Result<StarkProof<Blake2sMerkleHasher>, serde_json::Error> =
        serde_json::from_str(stark_proof_str);
    match wide_fib.verify::<Blake2sMerkleHasher>(stark_proof.unwrap()) {
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
