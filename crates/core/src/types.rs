use stwo_prover::core::prover::StarkProof;

/// A request to generate a proof for a Fibonnacci sequence.
pub struct FibonnacciProvingRequest {
    /// The size of the log to generate.
    pub log_size: u32,
    /// The claim to be proved.
    pub claim: u32,
}

pub struct FibonnacciProvingResponse {
    /// The size of the log to generate.
    pub log_size: u32,
    /// The claim to be proved.
    pub claim: u32,
    /// The proof generated for the request.
    pub proof: StarkProof,
}
