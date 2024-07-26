use serde::{Deserialize, Serialize};
use stwo_prover::core::prover::StarkProof;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FibonnacciProvingRequest {
    pub log_size: u32,
    pub claim: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FibonnacciProvingResponse {
    pub log_size: u32,
    pub claim: u32,
    pub proof: StarkProof,
}

impl FibonnacciProvingResponse {
    pub fn new(log_size: u32, claim: u32, proof: StarkProof) -> Self {
        Self {
            log_size,
            claim,
            proof,
        }
    }
}

impl Clone for FibonnacciProvingResponse {
    fn clone(&self) -> Self {
        // Temporarily use serde for a dirty clone
        // TODO: Implement a proper clone or find a better design that does not require cloning the
        // proof
        let proof_json = serde_json::to_string(&self.proof).unwrap();
        let proof = serde_json::from_str(&proof_json).unwrap();
        Self {
            log_size: self.log_size,
            claim: self.claim,
            proof,
        }
    }
}
