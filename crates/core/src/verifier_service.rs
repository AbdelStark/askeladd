use serde::{Deserialize, Serialize};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::prover::VerificationError;
use stwo_prover::examples::fibonacci::Fibonacci;

use crate::dvm::types::{FibonnacciProvingResponse, GenericProvingResponse};
// Define an enum to encapsulate possible deserialized types
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ProgramType {
    FibonnacciProvingResponse(FibonnacciProvingResponse),
    PoseidonProvingResponse(GenericProvingResponse),
    GenericProvingResponse(GenericProvingResponse),
}

#[derive(Debug, Default)]
pub struct VerifierService {}

impl VerifierService {
    pub fn verify_proof(
        &self,
        response: FibonnacciProvingResponse,
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
            ProgramType::FibonnacciProvingResponse(fib_answer) => {
                let fib = Fibonacci::new(fib_answer.log_size, BaseField::from(fib_answer.claim));
                fib.verify(fib_answer.proof)
            }
            ProgramType::PoseidonProvingResponse(_) => Ok(()),
            ProgramType::GenericProvingResponse(_) => Ok(()),
            // Err(e) => e,
        }
    }
}
