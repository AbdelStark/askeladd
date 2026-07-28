//! Multi-Fibonacci: several Fibonacci-squared sequences proven in a single
//! STARK proof, one trace column per sequence.

use std::iter::zip;

use stwo_prover::core::backend::cpu::CpuCircleEvaluation;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::{self, BaseField};
use stwo_prover::core::fields::IntoSlice;
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::prover::{ProvingError, StarkProof, VerificationError};
use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::trace_generation::{commit_and_prove, commit_and_verify};
use wasm_bindgen::prelude::*;

use super::air::MultiFibonacciAir;
use super::Fibonacci;
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

/// A multi-instance Fibonacci prover/verifier: `log_sizes[i]` and `claims[i]`
/// describe the i-th sequence.
pub struct MultiFibonacci {
    log_sizes: Vec<u32>,
    claims: Vec<BaseField>,
}

impl MultiFibonacci {
    pub fn new(log_sizes: Vec<u32>, claims: Vec<BaseField>) -> Self {
        assert!(!log_sizes.is_empty());
        assert_eq!(log_sizes.len(), claims.len());
        Self { log_sizes, claims }
    }

    /// One trace column per sequence, in the same order as the AIR's components.
    pub fn get_trace(&self) -> Vec<CpuCircleEvaluation<BaseField, BitReversedOrder>> {
        zip(&self.log_sizes, &self.claims)
            .map(|(log_size, claim)| Fibonacci::new(*log_size, *claim).get_trace())
            .collect()
    }

    fn air(&self) -> MultiFibonacciAir {
        MultiFibonacciAir::new(&self.log_sizes, &self.claims)
    }

    /// Both sides seed the Fiat–Shamir channel with all claims, in order.
    fn channel(&self) -> Blake2sChannel {
        Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&self.claims)))
    }

    pub fn prove(&self) -> Result<StarkProof<Blake2sMerkleHasher>, ProvingError> {
        commit_and_prove(&self.air(), &mut self.channel(), self.get_trace())
    }

    pub fn verify(&self, proof: StarkProof<Blake2sMerkleHasher>) -> Result<(), VerificationError> {
        commit_and_verify(proof, &self.air(), &mut self.channel())
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
            console_log!("Proof generated successfully");
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
            console_log!("Proof generation failed: {:?}", e);
            StwoResult {
                success: false,
                message: format!("Proof generation failed: {:?}", e),
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
        .map(m31::M31::from_u32_unchecked)
        .collect();
    let multi_fibo = MultiFibonacci::new(log_sizes, claims);
    let stark_proof: Result<StarkProof<Blake2sMerkleHasher>, serde_json::Error> =
        serde_json::from_str(stark_proof_str);
    match stark_proof {
        Ok(proof) => match multi_fibo.verify(proof) {
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
            console_log!("Failed to deserialize proof: {:?}", e);
            StwoResult {
                success: false,
                message: format!("Failed to deserialize proof: {:?}", e),
            }
        }
    }
}
