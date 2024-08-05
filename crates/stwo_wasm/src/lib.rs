// lib.rs
pub mod multi_fibonacci;
pub mod poseidon;
pub mod types;
pub mod wide_fibonnacci;

use serde::{Deserialize, Serialize};
use stwo_prover::core::fields::m31::{self, BaseField};
use stwo_prover::core::prover::StarkProof;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::examples::fibonacci::Fibonacci;
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

fn process_data<T>(data: T) -> Result<T, String>
where
    T: Clone + std::fmt::Debug, // Example constraints
{
    println!("Processing data: {:?}", data);
    Ok(data.clone())
}

#[wasm_bindgen]
pub fn verify_stark_proof (log_size: u32, claim: u32, stark_proof_str: &str) -> StwoResult {
    console_log!(
        "Starting verify_stark_proof with log_size: {}, claim: {}",
        log_size,
        claim
    );
    console_log!("Received proof string length: {}", stark_proof_str.len());

    let fib = Fibonacci::new(log_size, BaseField::from(claim));

    let stark_proof: Result<StarkProof<Blake2sMerkleHasher>, serde_json::Error> = serde_json::from_str(stark_proof_str);

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
