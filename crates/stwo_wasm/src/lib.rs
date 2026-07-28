//! STWO proving and verification, for native Rust and WebAssembly.
//!
//! This crate wraps STWO AIRs as self-contained prover/verifier units:
//!
//! - [`fibonacci`] — the classic single-column Fibonacci-squared AIR.
//! - [`wide_fibonacci`] — many Fibonacci instances in one proof.
//! - [`poseidon`] — Poseidon hash permutations.
//!
//! The same code compiles to WASM (`wasm-pack build --target web`), which is
//! how Askeladd's browser frontends prove and verify STARKs with zero server
//! trust.

pub mod fibonacci;
pub mod poseidon;
pub mod wide_fibonacci;

use poseidon::PoseidonStruct;
use serde::{Deserialize, Serialize};
use stwo_prover::core::prover::StarkProof;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use wasm_bindgen::prelude::*;

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

/// Result type handed back to JavaScript: a success flag plus a message
/// (the serialized proof on success, the error otherwise).
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

#[wasm_bindgen]
pub fn prove_and_verify(log_n_instances: u32) -> StwoResult {
    console_log!(
        "Starting prove_and_verify with log_n_instances: {}",
        log_n_instances,
    );
    let poseidon = PoseidonStruct::new(log_n_instances);
    match poseidon {
        Err(e) => StwoResult {
            success: false,
            message: format!("Failed to initialize Poseidon: {:?}", e),
        },
        Ok(p) => match p.prove::<Blake2sMerkleHasher>() {
            Ok(proof) => {
                console_log!("Proof generated successfully");
                let serialized = match serde_json::to_string(&proof) {
                    Ok(s) => s,
                    Err(e) => {
                        return StwoResult {
                            success: false,
                            message: format!("Failed to serialize proof: {:?}", e),
                        }
                    }
                };

                match p.verify::<Blake2sMerkleHasher>(proof) {
                    Ok(_) => {
                        console_log!("Proof verified successfully");
                        StwoResult {
                            success: true,
                            message: serialized,
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
    }
}

#[wasm_bindgen]
pub fn verify_stark_proof(log_n_instances: u32, stark_proof_str: &str) -> StwoResult {
    console_log!(
        "Starting verify_stark_proof with log_n_instances: {}",
        log_n_instances
    );
    console_log!("Received proof string length: {}", stark_proof_str.len());

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

    let poseidon = match PoseidonStruct::new(log_n_instances) {
        Ok(p) => p,
        Err(e) => {
            console_log!("Failed to initialize Poseidon: {:?}", e);
            return StwoResult {
                success: false,
                message: format!("Failed to initialize Poseidon: {:?}", e),
            };
        }
    };

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
