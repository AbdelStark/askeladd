// lib.rs

use std::iter::zip;

use stwo_prover::core::air::{Air, Component};
use stwo_prover::core::backend::cpu::CpuCircleEvaluation;
use stwo_prover::core::fields::m31::{self, BaseField};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::prover::{ProvingError, StarkProof, VerificationError};
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::examples::wide_fibonacci::component::WideFibComponent;
// use stwo_prover::examples::fibonacci::MultiFibonacci;
// use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
// use stwo_prover::core::channel::{Blake2sChannel, Channel};
// use stwo_prover::core::fields::IntoSlice;
use wasm_bindgen::prelude::*;

use crate::fibonnaci::Fibonacci;
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

pub struct MultiFibonacci {
    log_sizes: Vec<u32>,
    claims: Vec<BaseField>,
}
#[derive(Clone)]
pub struct WideFibAir {
    pub component: WideFibComponent,
}

impl Air for WideFibAir {
    fn components(&self) -> Vec<&dyn Component> {
        vec![&self.component]
    }
}

impl MultiFibonacci {
    pub fn new(log_sizes: Vec<u32>, claims: Vec<BaseField>) -> Self {
        assert!(!log_sizes.is_empty());
        assert_eq!(log_sizes.len(), claims.len());
        Self { log_sizes, claims }
    }

    pub fn get_trace(&self) -> Vec<CpuCircleEvaluation<BaseField, BitReversedOrder>> {
        zip(&self.log_sizes, &self.claims)
            .map(|(log_size, claim)| {
                let fib = Fibonacci::new(*log_size, *claim);
                fib.get_trace()
            })
            .collect()
    }

    pub fn prove(&self) -> Result<StarkProof<Blake2sMerkleHasher>, ProvingError> {
        println!("try proof of multi fibo");

        // let channel =
        //     &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&self.claims)));
        // let trace = self.get_trace();
        Err(ProvingError::ConstraintsNotSatisfied)
    }

    pub fn verify(&self, proof: StarkProof<Blake2sMerkleHasher>) -> Result<(), VerificationError> {
        // println!("try verify proof of multi fibo");
        println!("try verify proof of multi fibo");
        println!("stark proof {:?}", proof);
        // println!("stark proof {:?}", proof.commitment_scheme_proof.proof_of_work.nonce);
        // let channel =
        //     &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&self.claims)));
        // commit_and_verify(proof, &self, channel)
        Err(VerificationError::OodsNotMatching)
    }
}

#[wasm_bindgen]
pub fn stark_proof_multi_fibo(log_sizes: Vec<u32>, claims_int: Vec<u32>) -> StwoResult {
    let claims: Vec<BaseField> = claims_int
        .into_iter()
        .map(m31::M31::from_u32_unchecked)
        .collect();
    let multi_fibo = MultiFibonacci::new(log_sizes, claims);

    match multi_fibo.prove() {
        Ok(proof) => {
            console_log!("Proof deserialized successfully");
            match multi_fibo.verify(proof) {
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

// #[wasm_bindgen]
// pub fn stark_proof_multi_fibo(log_sizes: Vec<u32>, claims_int: Vec<u32>) -> StwoResult {
//     let claims: Vec<BaseField> = claims_int
//         .into_iter()
//         .map(m31::M31::from_u32_unchecked)
//         .collect();
//     let multi_fibo = MultiFibonacci::new(log_sizes, claims);

//     match multi_fibo.prove() {
//         Ok(proof) => {
//             console_log!("Proof deserialized successfully");
//             match multi_fibo.verify(proof) {
//                 Ok(()) => {
//                     console_log!("Proof verified successfully");
//                     StwoResult {
//                         success: true,
//                         message: "Proof verified successfully".to_string(),
//                     }
//                 }
//                 Err(e) => {
//                     console_log!("Proof verification failed: {:?}", e);
//                     StwoResult {
//                         success: false,
//                         message: format!("Proof verification failed: {:?}", e),
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             console_log!("Failed to deserialize proof: {:?}", e);
//             StwoResult {
//                 success: false,
//                 message: format!("Failed to deserialize proof: {:?}", e),
//             }
//         }
//     }
// }

// #[wasm_bindgen]
// pub fn verify_stark_proof_multi_fibo(
//     log_sizes: Vec<u32>,
//     claims_int: Vec<u32>,
//     stark_proof_str: &str,
// ) -> StwoResult {
//     let claims: Vec<BaseField> = claims_int
//         .into_iter()
//         .map(m31::M31::from_u32_unchecked)
//         .collect();
//     let multi_fibo = MultiFibonacci::new(log_sizes, claims);
//     // StwoResult {
//     //     success: false,
//     //     message: format!("Proof verification failed: {:?}", "no generic value"),
//     // }
//     let stark_proof: Result<StarkProof<Blake2sMerkleHasher>, serde_json::Error> =
//         serde_json::from_str(stark_proof_str);
//     match multi_fibo.verify(stark_proof.unwrap()) {
//         Ok(()) => {
//             console_log!("Proof verified successfully");
//             StwoResult {
//                 success: true,
//                 message: "Proof verified successfully".to_string(),
//             }
//         }
//         Err(e) => {
//             console_log!("Proof verification failed: {:?}", e);
//             StwoResult {
//                 success: false,
//                 message: format!("Proof verification failed: {:?}", e),
//             }
//         }
//     }
// }

// #[wasm_bindgen]
pub fn verify_stark_proof_multi_fibo(
    log_sizes: Vec<u32>,
    claims_int: Vec<u32>,
    stark_proof_str: &str,
) -> StwoResult {
    let claims: Vec<BaseField> = claims_int
        .into_iter()
        .map(m31::M31::from_u32_unchecked)
        .collect();
    let multi_fibo = MultiFibonacci::new(log_sizes, claims);
    // StwoResult {
    //     success: false,
    //     message: format!("Proof verification failed: {:?}", "no generic value"),
    // }
    let stark_proof: Result<StarkProof<Blake2sMerkleHasher>, serde_json::Error> =
        serde_json::from_str(stark_proof_str);
    match multi_fibo.verify(stark_proof.unwrap()) {
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
