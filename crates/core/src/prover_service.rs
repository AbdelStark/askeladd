use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::prover::ProvingError;
use stwo_prover::examples::fibonacci::Fibonacci;
use stwo_prover::examples::poseidon::{PoseidonAir, PoseidonComponent};

use crate::dvm::types::{
    ContractUploadType, FibonnacciProvingRequest, FibonnacciProvingResponse,
    GenericProvingResponse, PoseidonProvingRequest, ProgramInternalContractName, ProgramParams,
    ProgramRequestType,
};

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
        request_str: &str,
        program_params: Option<ProgramParams>,
    ) -> Result<GenericProvingResponse, ProvingError> {
        println!("generate_proof_by_program type {:?}", request);

        // TODO
        // Refacto & clean
        // Add not internal program check
        if let Some(program_params) = program_params {
            println!("program_params: {:?}", program_params);
            // TODO: bring others enum to publish your program and upload it
            // IPFS, BACKEND, URL
            match program_params.contract_reached {
                ContractUploadType::InternalAskeladd => {
                    match program_params.internal_contract_name {
                        // TODO: add other internal contract
                        Some(internal_contract) => {
                            match internal_contract {
                                ProgramInternalContractName::FibonnacciProvingRequest => {
                                    let fib_req: FibonnacciProvingRequest =
                                        serde_json::from_str(request_str).unwrap();
                                    let fib = Fibonacci::new(
                                        fib_req.log_size,
                                        BaseField::from(fib_req.claim),
                                    );
                                    match fib.prove() {
                                        Ok(proof) => {
                                            // let data: ProgramRequestType =
                                            // let data:Value =
                                            // serde_json::from_str(request).unwrap();
                                            // Ok(GenericProvingResponse::new(data.clone(), proof))
                                            Ok(GenericProvingResponse::new(request.clone(), proof))
                                        }
                                        Err(e) => Err(e),
                                    }
                                }
                                ProgramInternalContractName::PoseidonProvingRequest => {
                                    Err(ProvingError::ConstraintsNotSatisfied)
                                }
                            }
                        }
                        None => Err(ProvingError::ConstraintsNotSatisfied),
                    }
                }
            }
        } else {
            Err(ProvingError::ConstraintsNotSatisfied)
        }
    }
}
