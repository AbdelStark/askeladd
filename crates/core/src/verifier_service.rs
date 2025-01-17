use serde::{Deserialize, Serialize};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::prover::VerificationError;
use stwo_wasm::fibonacci::Fibonacci;

use crate::dvm::types::{FibonacciProvingResponse, GenericProvingResponse};
// Define an enum to encapsulate possible deserialized types
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ProgramType {
    Fibonacci(FibonacciProvingResponse),
    Poseidon(GenericProvingResponse),
    Generic(GenericProvingResponse),
}

#[derive(Debug, Default)]
pub struct VerifierService {}

impl VerifierService {
    pub fn verify_proof(
        &self,
        response: FibonacciProvingResponse,
    ) -> Result<(), VerificationError> {
        let fib = Fibonacci::new(response.log_size, BaseField::from(response.claim));
        fib.verify(response.proof)
    }

    pub fn verify_proof_generic(
        &self,
        response: serde_json::Value,
    ) -> Result<(), VerificationError> {
        let data: ProgramType = serde_json::from_value(response).unwrap();
        match data {
            ProgramType::Fibonacci(fib_answer) => {
                let fib = Fibonacci::new(fib_answer.log_size, BaseField::from(fib_answer.claim));
                fib.verify(fib_answer.proof)
            }
            ProgramType::Poseidon(_) => Ok(()),
            ProgramType::Generic(_) => Ok(()),
            // Err(e) => e,
        }
    }
}
