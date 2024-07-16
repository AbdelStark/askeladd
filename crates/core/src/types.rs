use serde::{Deserialize, Serialize};
use stwo_prover::core::prover::StarkProof;

use crate::wrappers::{ConversionError, StarkProofWrapper};

#[derive(Debug, Serialize, Deserialize)]
pub struct FibonnacciProvingRequest {
    pub request_id: String,
    pub log_size: u32,
    pub claim: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FibonnacciProvingResponse {
    pub request_id: String,
    pub log_size: u32,
    pub claim: u32,
    pub proof: StarkProofWrapper,
}

impl FibonnacciProvingResponse {
    pub fn new(request_id: String, log_size: u32, claim: u32, proof: StarkProof) -> Self {
        Self {
            request_id,
            log_size,
            claim,
            proof: proof.into(),
        }
    }

    pub fn into_stark_proof(self) -> Result<(String, u32, u32, StarkProof), ConversionError> {
        let proof = self.proof.try_into().unwrap();
        Ok((self.request_id, self.log_size, self.claim, proof))
    }
}
