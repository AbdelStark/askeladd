//! Turns proving job requests into STARK proofs.
//!
//! The service dispatches on the program named in the job and delegates to
//! the STWO-based provers in `stwo_wasm`. Every failure mode is a typed
//! error — a malformed job must never panic the agent.

use serde::de::DeserializeOwned;
use serde_json::Value;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_wasm::fibonacci::multi_fibonacci::MultiFibonacci;
use stwo_wasm::fibonacci::Fibonacci;
use stwo_wasm::poseidon::{PoseidonStruct, MIN_LOG_N_ROWS, N_LOG_INSTANCES_PER_ROW};
use stwo_wasm::wide_fibonacci::WideFibStruct;
use thiserror::Error;

use crate::dvm::types::{
    ContractUploadType, FibonacciProvingRequest, GenericProvingResponse,
    MultiFibonacciProvingRequest, PoseidonProvingRequest, ProgramInternalContractName,
    ProgramParams, WideFibonacciProvingRequest,
};
use crate::utils::convert_inputs_to_run_program;

/// Errors a prover agent can hit while executing a job.
#[derive(Error, Debug)]
pub enum ProverServiceError {
    /// The job request carried no program parameters at all.
    #[error("job request carries no program parameters")]
    MissingProgramParams,
    /// The requested program source is not supported (only built-ins are).
    #[error("unsupported program source: {0:?} (only built-in programs are available)")]
    UnsupportedContractSource(ContractUploadType),
    /// The requested program is unknown or not implemented yet.
    #[error("unsupported program: {0}")]
    UnsupportedProgram(String),
    /// Inputs failed to deserialize or violate the program's size limits.
    #[error("invalid inputs for {program}: {reason}")]
    InvalidInputs {
        program: &'static str,
        reason: String,
    },
    /// The STARK prover itself failed.
    #[error("proving failed: {0}")]
    ProvingFailed(String),
}

/// Stateless dispatcher from job requests to proofs.
#[derive(Debug, Default)]
pub struct ProverService {}

impl ProverService {
    /// Executes the program selected in `program_params` on `request`,
    /// returning the echoed inputs together with the STARK proof.
    pub fn generate_proof_by_program(
        &self,
        request: Value,
        program_params: Option<ProgramParams>,
    ) -> Result<GenericProvingResponse, ProverServiceError> {
        let params = program_params.ok_or(ProverServiceError::MissingProgramParams)?;
        match params.contract_reached {
            ContractUploadType::InternalAskeladd => self.prove_internal(request, &params),
            other => Err(ProverServiceError::UnsupportedContractSource(other)),
        }
    }

    fn prove_internal(
        &self,
        request: Value,
        params: &ProgramParams,
    ) -> Result<GenericProvingResponse, ProverServiceError> {
        let program = params
            .internal_contract_name
            .clone()
            .ok_or_else(|| ProverServiceError::UnsupportedProgram("unspecified".to_owned()))?;

        match program {
            ProgramInternalContractName::FibonacciProvingRequest => {
                let req: FibonacciProvingRequest = parse_inputs(&request, params, "fibonacci")?;
                let fib = Fibonacci::new(req.log_size, BaseField::from(req.claim));
                let proof = fib
                    .prove()
                    .map_err(|e| ProverServiceError::ProvingFailed(e.to_string()))?;
                Ok(GenericProvingResponse::new(request, proof))
            }
            ProgramInternalContractName::MultiFibonacciProvingRequest => {
                let req: MultiFibonacciProvingRequest =
                    parse_inputs(&request, params, "multi-fibonacci")?;
                if req.log_sizes.len() != req.claims.len() {
                    return Err(ProverServiceError::InvalidInputs {
                        program: "multi-fibonacci",
                        reason: "log_sizes and claims must have the same length".to_owned(),
                    });
                }
                let claims = req.claims.into_iter().map(BaseField::from).collect();
                let multi = MultiFibonacci::new(req.log_sizes, claims);
                let proof = multi
                    .prove()
                    .map_err(|e| ProverServiceError::ProvingFailed(e.to_string()))?;
                Ok(GenericProvingResponse::new(request, proof))
            }
            ProgramInternalContractName::PoseidonProvingRequest => {
                let req: PoseidonProvingRequest = parse_inputs(&request, params, "poseidon")?;
                validate_poseidon_inputs(req.log_n_instances)?;
                let poseidon = PoseidonStruct::new(req.log_n_instances).map_err(|reason| {
                    ProverServiceError::InvalidInputs {
                        program: "poseidon",
                        reason,
                    }
                })?;
                let proof = poseidon
                    .prove::<Blake2sMerkleHasher>()
                    .map_err(|e| ProverServiceError::ProvingFailed(e.to_string()))?;
                Ok(GenericProvingResponse::new(request, proof))
            }
            ProgramInternalContractName::WideFibonacciProvingRequest => {
                let req: WideFibonacciProvingRequest =
                    parse_inputs(&request, params, "wide-fibonacci")?;
                stwo_wasm::wide_fibonacci::validate_inputs(
                    req.log_fibonacci_size,
                    req.log_n_instances,
                )
                .map_err(|reason| ProverServiceError::InvalidInputs {
                    program: "wide-fibonacci",
                    reason,
                })?;
                let wide_fib = WideFibStruct::new(req.log_fibonacci_size, req.log_n_instances);
                let proof = wide_fib
                    .prove::<Blake2sMerkleHasher>()
                    .map_err(|e| ProverServiceError::ProvingFailed(e.to_string()))?;
                Ok(GenericProvingResponse::new(request, proof))
            }
            ProgramInternalContractName::Custom(name) => {
                Err(ProverServiceError::UnsupportedProgram(name))
            }
        }
    }
}

/// Deserializes program inputs, first from the structured `request` payload,
/// falling back to the string inputs map (which is also what NIP-90 tags carry).
fn parse_inputs<T: DeserializeOwned>(
    request: &Value,
    params: &ProgramParams,
    program: &'static str,
) -> Result<T, ProverServiceError> {
    if let Ok(parsed) = serde_json::from_value::<T>(request.clone()) {
        return Ok(parsed);
    }
    if let Some(inputs) = &params.inputs {
        let converted = convert_inputs_to_run_program(inputs.clone());
        if let Ok(parsed) =
            serde_json::from_value::<T>(serde_json::to_value(converted).map_err(|e| {
                ProverServiceError::InvalidInputs {
                    program,
                    reason: e.to_string(),
                }
            })?)
        {
            return Ok(parsed);
        }
    }
    Err(ProverServiceError::InvalidInputs {
        program,
        reason: "inputs do not match the program's expected shape".to_owned(),
    })
}

/// Enforces the trace-size limits of the Poseidon AIR.
fn validate_poseidon_inputs(log_n_instances: u32) -> Result<(), ProverServiceError> {
    let invalid = |reason: String| ProverServiceError::InvalidInputs {
        program: "poseidon",
        reason,
    };
    if (log_n_instances as usize) < N_LOG_INSTANCES_PER_ROW {
        return Err(invalid(format!(
            "log_n_instances must be at least {N_LOG_INSTANCES_PER_ROW}"
        )));
    }
    let log_n_rows = log_n_instances - N_LOG_INSTANCES_PER_ROW as u32;
    if log_n_rows < MIN_LOG_N_ROWS {
        return Err(invalid(format!(
            "log_n_rows ({log_n_rows}) must be at least {MIN_LOG_N_ROWS}; increase log_n_instances"
        )));
    }
    Ok(())
}
