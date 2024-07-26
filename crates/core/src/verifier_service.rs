use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::prover::VerificationError;
use stwo_prover::examples::fibonacci::Fibonacci;

use crate::types::FibonnacciProvingResponse;

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
}
