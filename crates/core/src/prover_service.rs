use std::collections::HashMap;
use std::fmt;

use serde_json::Result as SerdeResult;
use stwo_prover::core::backend::simd::fft::MIN_FFT_LOG_SIZE;
use stwo_prover::core::circle::M31_CIRCLE_LOG_ORDER;
use stwo_prover::core::fields::m31::{self, BaseField};
use stwo_prover::core::prover::ProvingError;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::examples::fibonacci::{Fibonacci, MultiFibonacci};
use stwo_wasm::poseidon::{PoseidonStruct, LOG_N_LANES, N_LOG_INSTANCES_PER_ROW};
use stwo_wasm::wide_fibonnacci::WideFibStruct;
use thiserror::Error;

use crate::dvm::types::{
    ContractUploadType, FibonnacciProvingRequest, FibonnacciProvingResponse,
    GenericProvingResponse, MultiFibonnacciProvingRequest, PoseidonProvingRequest,
    ProgramInternalContractName, ProgramParams, WideFibonnacciProvingRequest,
};
use crate::utils::convert_inputs_to_run_program;

#[derive(Error, Debug, Clone)]
pub enum ProverServiceError {
    // #[error("No program param")]
    NoProgramParam,
    Custom(String),
}

impl fmt::Display for ProverServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProverServiceError::NoProgramParam => write!(f, "NO PROGRAM PARAM"),
            ProverServiceError::Custom(ref err) => write!(f, "ProverServiceError {}", err),
        }
    }
}

#[derive(Debug, Default)]
pub struct ProverService {}

impl ProverService {
    pub fn generate_proof(
        &self,
        request: FibonnacciProvingRequest,
    ) -> Result<FibonnacciProvingResponse, ProvingError> {
        let fib = Fibonacci::new(request.log_size, BaseField::from(request.claim));
        match fib.prove() {
            Ok(proof) => Ok(FibonnacciProvingResponse::new(
                request.log_size,
                request.claim,
                proof,
            )),
            Err(e) => Err(e),
        }
    }

    pub fn generate_proof_by_program(
        &self,
        request: serde_json::Value,
        program_params: Option<ProgramParams>,
    ) -> Result<GenericProvingResponse, String> {
        println!("generate_proof_by_program type {:?}", request);
        let mut successful_parses = HashMap::new();
        if let Some(program_params) = program_params.clone() {
            successful_parses = convert_inputs_to_run_program(program_params.inputs);
        }
        let serialized_request = serde_json::to_string(&successful_parses).unwrap();
        // TODO
        // - Refacto & clean
        // -Different type of program launched: NIP-78 andNIP-94 + NIP-96 to handle program not
        // internal
        self.check_and_generate_proof(request, serialized_request.clone().as_str(), program_params)
    }

    pub fn check_and_generate_proof(
        &self,
        request: serde_json::Value,
        request_str: &str,
        program_params: Option<ProgramParams>,
    ) -> Result<GenericProvingResponse, String> {
        // TODO: bring others enum to publish your program and upload it
        // IPFS, BACKEND, URL
        if let Some(p) = program_params {
            match p.contract_reached {
                ContractUploadType::InternalAskeladd => {
                    self.internal_program(request, request_str, p)
                } //  => Err(ProverServiceError::NoProgramParam.to_string()),
            }
        } else {
            Err(ProverServiceError::NoProgramParam.to_string())
        }
    }

    pub fn internal_program(
        &self,
        request: serde_json::Value,
        serialized_request: &str,
        program_params: ProgramParams,
    ) -> Result<GenericProvingResponse, String> {
        match program_params.clone().internal_contract_name {
            None => {
                println!("No internal contract name");
                Err(ProverServiceError::NoProgramParam.to_string())
            }
            // TODO: add other internal contract
            Some(internal_contract) => match internal_contract {
                ProgramInternalContractName::FibonnacciProvingRequest => {
                    println!("try check request fib");
                    let fib_req_res: SerdeResult<FibonnacciProvingRequest> =
                        serde_json::from_str(&serialized_request);
                    let fib_req: FibonnacciProvingRequest;
                    match fib_req_res.as_ref() {
                        Ok(req) => {
                            fib_req = req.clone();
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                    let fib = Fibonacci::new(fib_req.log_size, BaseField::from(fib_req.claim));
                    match fib.prove() {
                        Ok(proof) => Ok(GenericProvingResponse::new(request.clone(), proof)),
                        Err(e) => Err(e.to_string()),
                    }
                }
                ProgramInternalContractName::MultiFibonnaciProvingRequest => {
                    let multi_fibo_res: SerdeResult<MultiFibonnacciProvingRequest> =
                        serde_json::from_str(&serialized_request);
                    let mul_fib_req: MultiFibonnacciProvingRequest;
                    match multi_fibo_res.as_ref() {
                        Ok(req) => {
                            mul_fib_req = req.clone();
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                    let claims: Vec<BaseField> = mul_fib_req
                        .claims
                        .into_iter()
                        .map(|f| m31::M31::from_u32_unchecked(f))
                        .collect();
                    let multi_fibo = MultiFibonacci::new(mul_fib_req.log_sizes, claims);
                    match multi_fibo.prove() {
                        Ok(proof) => Ok(GenericProvingResponse::new(request.clone(), proof)),
                        Err(e) => Err(e.to_string()),
                    }
                }
                ProgramInternalContractName::Custom(_) => {
                    println!("Custom internal contract");
                    Err(ProvingError::ConstraintsNotSatisfied.to_string())
                }
                ProgramInternalContractName::PoseidonProvingRequest => {
                    // Err(ProvingError::ConstraintsNotSatisfied.to_string())
                    let poseidon_serde_req: SerdeResult<PoseidonProvingRequest> =
                        serde_json::from_str(&serialized_request);
                    let poseidon_req: PoseidonProvingRequest;
                    match poseidon_serde_req.as_ref() {
                        Ok(req) => {
                            poseidon_req = req.clone();
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                    // TODO
                    //  add requirements in inputs_requirements
                    if poseidon_req.log_n_instances < N_LOG_INSTANCES_PER_ROW as u32 {
                        return Err("OVERFLOW".to_string());
                    }

                    assert!(poseidon_req.log_n_instances >= N_LOG_INSTANCES_PER_ROW as u32);
                    let log_n_rows = poseidon_req.log_n_instances - N_LOG_INSTANCES_PER_ROW as u32;

                    println!(
                        "log_n_rows {} >= LOG_N_LANES {} == {}",
                        log_n_rows,
                        LOG_N_LANES,
                        log_n_rows >= LOG_N_LANES,
                    );
                    println!("log_n_rows {}", log_n_rows);
                    if log_n_rows < LOG_N_LANES {
                        println!(
                            "failed log_n_rows >= LOG_N_LANES  {}",
                            log_n_rows >= LOG_N_LANES
                        );
                        return Err("log_size >= LOG_N_LANES".to_string());
                    }
                    println!("MIN_FFT_LOG_SIZE as usize {}", MIN_FFT_LOG_SIZE);

                    println!(
                        "poseidon_req.log_n_instances  as usize {}",
                        poseidon_req.log_n_instances
                    );

                    println!(
                        "poseidon_req.log_n_instances < MIN_FFT_LOG_SIZE{}",
                        poseidon_req.log_n_instances < MIN_FFT_LOG_SIZE
                    );

                    if poseidon_req.log_n_instances < MIN_FFT_LOG_SIZE as u32
                        || log_n_rows < MIN_FFT_LOG_SIZE as u32
                    {
                        println!(
                            "log_n_elements >= MIN_FFT_LOG_SIZE as usize {}",
                            log_n_rows >= LOG_N_LANES
                        );
                        return Err("llog_n_elements >= MIN_FFT_LOG_SIZE as usize".to_string());
                    }

                    if poseidon_req.log_n_instances >= M31_CIRCLE_LOG_ORDER {
                        return Err("log_n_instances >= M31_CIRCLE_LOG_ORDER as usize".to_string());
                    }

                    let poseidon = PoseidonStruct::new(poseidon_req.log_n_instances);

                    // TODO fix prove poseidon with inputs_requirements
                    match poseidon {
                        Ok(poseidon) => match poseidon.prove::<Blake2sMerkleHasher>() {
                            Ok(proof) => Ok(GenericProvingResponse::new(request.clone(), proof)),
                            Err(e) => Err(e.to_string()),
                        },
                        Err(_) => Err(ProvingError::ConstraintsNotSatisfied.to_string()),
                    }
                    // Err(ProvingError::ConstraintsNotSatisfied.to_string())
                }
                ProgramInternalContractName::WideFibonnaciProvingRequest => {
                    // Err(ProvingError::ConstraintsNotSatisfied.to_string())

                    let wide_fib_serde: SerdeResult<WideFibonnacciProvingRequest> =
                        serde_json::from_str(&serialized_request);
                    let wide_fib_req: WideFibonnacciProvingRequest;
                    match wide_fib_serde.as_ref() {
                        Ok(req) => {
                            wide_fib_req = req.clone();
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                    let wide_fib = WideFibStruct::new(
                        wide_fib_req.log_fibonacci_size,
                        wide_fib_req.log_n_instances,
                    );
                    match wide_fib.prove::<Blake2sMerkleHasher>() {
                        Ok(proof) => Ok(GenericProvingResponse::new(request.clone(), proof)),
                        Err(e) => Err(e.to_string()),
                    }
                } // _ => Err(ProvingError::ConstraintsNotSatisfied.to_string()),
            },
        }
    }
}
