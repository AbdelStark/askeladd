// lib.rs

use stwo_prover::core::fields::m31::{self, BaseField};
use stwo_prover::core::prover::StarkProof;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::examples::fibonacci::MultiFibonacci;
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

#[wasm_bindgen]
pub fn stark_proof_multi_fibo(log_sizes: Vec<u32>, claims_int: Vec<u32>) -> StwoResult {
    let claims: Vec<BaseField> = claims_int
        .into_iter()
        .map(|f| m31::M31::from_u32_unchecked(f))
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

#[wasm_bindgen]
pub fn verify_stark_proof_multi_fibo(
    log_sizes: Vec<u32>,
    claims_int: Vec<u32>,
    stark_proof_str: &str,
) -> StwoResult {
    let claims: Vec<BaseField> = claims_int
        .into_iter()
        .map(|f| m31::M31::from_u32_unchecked(f))
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

// #[wasm_bindgen]
// pub fn verify_stark_proof_multi_fibo(
//     log_sizes: Vec<u32>,
//     claims_int: Vec<u32>,
//     stark_proof_str: &str,
// ) -> StwoResult {
//     let claims: Vec<BaseField> = claims_int
//         .into_iter()
//         .map(|f| m31::M31::from_u32_unchecked(f))
//         .collect();
//     let multi_fibo = MultiFibonacci::new(log_sizes, claims);
//     StwoResult {
//         success: false,
//         message: format!("Proof verification failed: {:?}", "no generic value"),
//     }
//     // let stark_proof: Result<StarkProof<H>, serde_json::Error> =
// serde_json::from_str(stark_proof_str);     // match multi_fibo.verify(stark_proof.unwrap()) {
//     //     Ok(()) => {
//     //         console_log!("Proof verified successfully");
//     //         StwoResult {
//     //             success: true,
//     //             message: "Proof verified successfully".to_string(),
//     //         }
//     //     }
//     //     Err(e) => {
//     //         console_log!("Proof verification failed: {:?}", e);
//     //         StwoResult {
//     //             success: false,
//     //             message: format!("Proof verification failed: {:?}", e),
//     //         }
//     //     }
//     // }
// }
