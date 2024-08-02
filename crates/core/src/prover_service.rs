use std::collections::HashMap;

use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Result as SerdeResult, Value};
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
        // ) -> Result<GenericProvingResponse, ProvingError> {
    ) -> Result<GenericProvingResponse, String> {
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
                                    let mut successful_parses = HashMap::new();

                                    for (key, value) in program_params.params_map {
                                        if let Ok(num) = value.parse::<u32>() {
                                            successful_parses.insert(key.clone(), num);
                                            println!(
                                                "The value for '{}' is a valid u32: {}",
                                                key, num
                                            );
                                        } else {
                                            println!("The value for '{}' is not a valid u32.", key);
                                        }
                                    }
                                    println!("try check request fib");
                                    // Serialize the HashMap into a JSON string
                                    let serialized = serde_json::to_string(&successful_parses).unwrap();
                                    // let parsed: Value = serde_json::from_str(serialized)?;
                                    println!("Parsed JSON Value: {}", serialized);

                                    let fib_req_res: SerdeResult<FibonnacciProvingRequest> =
                                        serde_json::from_str(&serialized);

                                    // println!("try check request fib");
                                    // let fib_req_res: SerdeResult<FibonnacciProvingRequest> =
                                    //     serde_json::from_str(request_str);

                                    // fib_req_res.map_err(|e| e.to_string()).as_ref();
                                    let fiq_req: FibonnacciProvingRequest;

                                    match fib_req_res.as_ref() {
                                        Ok(req) => {
                                            fiq_req = req.clone();
                                        }
                                        Err(e) => return Err(e.to_string()),
                                    }
                                    println!("fiq_req: {:?}", fiq_req);

                                    let fib = Fibonacci::new(
                                        fiq_req.log_size,
                                        BaseField::from(fiq_req.claim),
                                    );
                                    match fib.prove() {
                                        Ok(proof) => {
                                            Ok(GenericProvingResponse::new(request.clone(), proof))
                                        }
                                        Err(e) => Err(e.to_string()),
                                    }
                                }
                                ProgramInternalContractName::PoseidonProvingRequest => {
                                    Err(ProvingError::ConstraintsNotSatisfied.to_string())
                                }
                                ProgramInternalContractName::Custom(s) => {
                                    println!("Custom internal contract");
                                    Err(ProvingError::ConstraintsNotSatisfied.to_string())
                                }
                                _ => Err(ProvingError::ConstraintsNotSatisfied.to_string()),
                            }
                        }
                        None => {
                            println!("No internal contract name");
                            Err(ProvingError::ConstraintsNotSatisfied.to_string())

                            // let fib_req: FibonnacciProvingRequest =
                            //     serde_json::from_str(request_str).unwrap();
                            // let fib =
                            //     Fibonacci::new(fib_req.log_size, BaseField::from(fib_req.claim));
                            // match fib.prove() {
                            //     Ok(proof) => {
                            //         // let data: ProgramRequestType =
                            //         // let data:Value =
                            //         // serde_json::from_str(request)R.unwrap();
                            //         // Ok(GenericProvingResponse::new(data.clone(), proof))
                            //         Ok(GenericProvingResponse::new(request.clone(), proof))
                            //     }
                            //     Err(e) => Err(e),
                            // }
                        }
                    }
                }
            }
        } else {
            Err(ProvingError::ConstraintsNotSatisfied.to_string())
        }
    }
}
