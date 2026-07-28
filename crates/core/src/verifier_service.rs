//! Verifies the STARK proofs attached to job results.
//!
//! Verification is local and trustless: given the echoed public inputs and
//! the proof, the verifier reconstructs the AIR for the chosen program and
//! checks the proof. No network, no faith in the prover — just mathematics.

use serde_json::Value;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_wasm::fibonacci::multi_fibonacci::MultiFibonacci;
use stwo_wasm::fibonacci::Fibonacci;
use stwo_wasm::poseidon::PoseidonStruct;
use stwo_wasm::wide_fibonacci::WideFibStruct;
use thiserror::Error;

use crate::dvm::types::{
    FibonacciProvingRequest, GenerateZKPJobResult, GenericProvingResponse,
    MultiFibonacciProvingRequest, PoseidonProvingRequest, ProgramInternalContractName,
    WideFibonacciProvingRequest,
};

/// Errors verification can fail with — including "the proof is invalid".
#[derive(Error, Debug)]
pub enum VerifierServiceError {
    /// The job result is missing the echoed public inputs.
    #[error("malformed job result: {0}")]
    MalformedResult(String),
    /// The program named by the caller is not verifiable here.
    #[error("unsupported program: {0}")]
    UnsupportedProgram(String),
    /// The proof does not verify. Do not trust the result.
    #[error("proof verification failed: {0}")]
    InvalidProof(String),
}

/// Stateless proof verifier.
#[derive(Debug, Default)]
pub struct VerifierService {}

impl VerifierService {
    /// Verifies the proof in `job_result` against `program`.
    ///
    /// `program` must be the same selector the customer used in the request;
    /// the echoed inputs embedded in the result provide the public statement.
    pub fn verify_job_result(
        &self,
        job_result: &GenerateZKPJobResult,
        program: &ProgramInternalContractName,
    ) -> Result<(), VerifierServiceError> {
        // The result's `response` field carries the echoed inputs and the
        // proof, serialized once. Deserialize it a single time.
        let generic: GenericProvingResponse =
            serde_json::from_value(job_result.response.clone())
                .map_err(|e| VerifierServiceError::MalformedResult(e.to_string()))?;
        let inputs = &generic.response;
        let proof = generic.proof;

        match program {
            ProgramInternalContractName::FibonacciProvingRequest => {
                let req: FibonacciProvingRequest = parse_echoed(inputs, "fibonacci")?;
                let fib = Fibonacci::new(req.log_size, BaseField::from(req.claim));
                fib.verify(proof)
                    .map_err(|e| VerifierServiceError::InvalidProof(e.to_string()))
            }
            ProgramInternalContractName::MultiFibonacciProvingRequest => {
                let req: MultiFibonacciProvingRequest = parse_echoed(inputs, "multi-fibonacci")?;
                let claims = req.claims.into_iter().map(BaseField::from).collect();
                let multi = MultiFibonacci::new(req.log_sizes, claims);
                multi
                    .verify(proof)
                    .map_err(|e| VerifierServiceError::InvalidProof(e.to_string()))
            }
            ProgramInternalContractName::PoseidonProvingRequest => {
                let req: PoseidonProvingRequest = parse_echoed(inputs, "poseidon")?;
                let poseidon = PoseidonStruct::new(req.log_n_instances)
                    .map_err(VerifierServiceError::MalformedResult)?;
                poseidon
                    .verify::<Blake2sMerkleHasher>(proof)
                    .map_err(|e| VerifierServiceError::InvalidProof(e.to_string()))
            }
            ProgramInternalContractName::WideFibonacciProvingRequest => {
                let req: WideFibonacciProvingRequest = parse_echoed(inputs, "wide-fibonacci")?;
                let wide_fib = WideFibStruct::new(req.log_fibonacci_size, req.log_n_instances);
                wide_fib
                    .verify::<Blake2sMerkleHasher>(proof)
                    .map_err(|e| VerifierServiceError::InvalidProof(e.to_string()))
            }
            ProgramInternalContractName::Custom(name) => {
                Err(VerifierServiceError::UnsupportedProgram(name.clone()))
            }
        }
    }
}

/// Extracts the echoed request inputs from a job result's `response` field,
/// which carries a serialized `GenericProvingResponse { response, proof }`.
fn parse_echoed<T: serde::de::DeserializeOwned>(
    inputs: &Value,
    program: &'static str,
) -> Result<T, VerifierServiceError> {
    serde_json::from_value(inputs.clone()).map_err(|e| {
        VerifierServiceError::MalformedResult(format!("cannot parse {program} inputs: {e}"))
    })
}
