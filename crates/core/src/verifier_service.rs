use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::prover::VerificationError;
use stwo_prover::examples::fibonacci::Fibonacci;

use crate::types::FibonnacciProvingResponse;
use crate::wrappers::ConversionError;

#[derive(Debug, thiserror::Error)]
pub enum VerifierError {
    #[error(transparent)]
    Verification(#[from] VerificationError),
    #[error(transparent)]
    Conversion(#[from] ConversionError),
}

#[derive(Debug, Default)]
pub struct VerifierService {}

impl VerifierService {
    pub fn verify_proof(&self, response: FibonnacciProvingResponse) -> Result<(), VerifierError> {
        let (_, log_size, claim, proof) = response.into_stark_proof()?;
        let fib = Fibonacci::new(log_size, BaseField::from(claim));
        fib.verify(proof).map_err(VerifierError::Verification)
    }
}
