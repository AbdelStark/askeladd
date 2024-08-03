use std::collections::HashMap;

use serde_json::{Result as SerdeResult, Value};
use stwo_prover::core::fields::m31::{self, BaseField};
use stwo_prover::core::prover::ProvingError;
use stwo_prover::examples::fibonacci::{Fibonacci, MultiFibonacci};
use stwo_wasm::poseidon::PoseidonStruct;
use stwo_wasm::wide_fibonnacci::WideFibStruct;

use crate::dvm::types::{
    ContractUploadType, FibonnacciProvingRequest, FibonnacciProvingResponse, GenericProvingResponse, MultiFibonnacciProvingRequest, PoseidonProvingRequest, ProgramInternalContractName, ProgramParams, ProgramRequestType, WideFibonnacciProvingRequest
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
        let mut successful_parses = HashMap::new();
        if let Some(program_params) = program_params.clone() {
            for (key, value) in program_params.clone().params_map.iter() {
                if let Ok(num) = value.parse::<u32>() {
                    successful_parses.insert(key.clone(), num);
                    println!("The value for '{}' is a valid u32: {}", key, num);
                } else {
                    println!("The value for '{}' is not a valid u32.", key);
                }
            }
        }

        let serialized_request = serde_json::to_string(&successful_parses).unwrap();
        // TODO
        // Refacto & clean
        // Add not internal program check
        if let Some(program_params) = program_params {
            println!("program_params: {:?}", program_params);
            // TODO: bring others enum to publish your program and upload it
            // IPFS, BACKEND, URL
            match program_params.contract_reached {
                ContractUploadType::InternalAskeladd => {
                    match program_params.clone().internal_contract_name {
                        // TODO: add other internal contract
                        Some(internal_contract) => {
                            match internal_contract {
                                ProgramInternalContractName::FibonnacciProvingRequest => {
                                    println!("try check request fib");
                                    // let fib_req_res: SerdeResult<FibonnacciProvingRequest> =
                                    //     serde_json::from_str(request_str);

                                    let fib_req_res: SerdeResult<FibonnacciProvingRequest> =
                                        serde_json::from_str(&serialized_request);

                                    // fib_req_res.map_err(|e| e.to_string()).as_ref();
                                    let fib_req: FibonnacciProvingRequest;

                                    match fib_req_res.as_ref() {
                                        Ok(req) => {
                                            fib_req = req.clone();
                                        }
                                        Err(e) => return Err(e.to_string()),
                                    }
                                    println!("fiq_req: {:?}", fib_req);

                                    let fib = Fibonacci::new(
                                        fib_req.log_size,
                                        BaseField::from(fib_req.claim),
                                    );
                                    match fib.prove() {
                                        Ok(proof) => {
                                            Ok(GenericProvingResponse::new(request.clone(), proof))
                                        }
                                        Err(e) => Err(e.to_string()),
                                    }
                                }
                                ProgramInternalContractName::PoseidonProvingRequest => {
                                    let poseidon_serde_req: SerdeResult<PoseidonProvingRequest> =
                                        serde_json::from_str(&serialized_request);

                                    // fib_req_res.map_err(|e| e.to_string()).as_ref();
                                    let poseidon_req: PoseidonProvingRequest;
                                    match poseidon_serde_req.as_ref() {
                                        Ok(req) => {
                                            poseidon_req = req.clone();
                                        }
                                        Err(e) => return Err(e.to_string()),
                                    }
                                    let poseidon = PoseidonStruct::new(poseidon_req.log_n_rows);
                                    match poseidon.prove() {
                                        Ok(proof) => {
                                            Ok(GenericProvingResponse::new(request.clone(), proof))
                                        }
                                        Err(e) => Err(e.to_string()),
                                    }
                                    
                                }
                                ProgramInternalContractName::WideFibonnaciProvingRequest => {
                                    let wide_fib_serde: SerdeResult<WideFibonnacciProvingRequest> =
                                        serde_json::from_str(&serialized_request);

                                    // fib_req_res.map_err(|e| e.to_string()).as_ref();
                                    let wide_fib_req: WideFibonnacciProvingRequest;

                                    match wide_fib_serde.as_ref() {
                                        Ok(req) => {
                                            wide_fib_req = req.clone();
                                        }
                                        Err(e) => return Err(e.to_string()),
                                    }

                                    println!("WideFib create component");

                                    let wide_fib = WideFibStruct::new(
                                        wide_fib_req.log_fibonacci_size,
                                        wide_fib_req.log_n_instances,
                                    );

                                    match wide_fib.prove() {
                                        Ok(proof) => {
                                            Ok(GenericProvingResponse::new(request.clone(), proof))
                                        }
                                        Err(e) => Err(e.to_string()),
                                    }
                                }
                                ProgramInternalContractName::MultiFibonnaciProvingRequest => {
                                    let multi_fibo_res: SerdeResult<MultiFibonnacciProvingRequest> =
                                        serde_json::from_str(&serialized_request);

                                    // fib_req_res.map_err(|e| e.to_string()).as_ref();
                                    let mul_fib_req: MultiFibonnacciProvingRequest;
                                    match multi_fibo_res.as_ref() {
                                        Ok(req) => {
                                            mul_fib_req = req.clone();
                                        }
                                        Err(e) => return Err(e.to_string()),
                                    }
                                    let poseidon = MultiFibonacci::new(mul_fib_req.log_sizes, mul_fib_req.claims);
                                    match poseidon.prove() {
                                        Ok(proof) => {
                                            Ok(GenericProvingResponse::new(request.clone(), proof))
                                        }
                                        Err(e) => Err(e.to_string()),
                                    }
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
