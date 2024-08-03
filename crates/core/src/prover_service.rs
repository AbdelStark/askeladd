use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Result as SerdeResult, Value};
use stwo_prover::core::air::AirProver;
use stwo_prover::core::backend::cpu::CpuCircleEvaluation;
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::{self, BaseField};
use stwo_prover::core::fields::IntoSlice;
use stwo_prover::core::poly::circle::CanonicCoset;
use stwo_prover::core::prover::{ProvingError, StarkProof};
use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
use stwo_prover::core::vcs::hasher::Hasher;
use stwo_prover::examples::fibonacci::Fibonacci;
use stwo_prover::examples::poseidon::{PoseidonAir, PoseidonComponent};
use stwo_prover::examples::wide_fibonacci::component::{
    Input, WideFibAir, WideFibComponent, LOG_N_COLUMNS,
};
use stwo_prover::examples::wide_fibonacci::constraint_eval::gen_trace;
// use stwo_prover::examples::wide_fibonacci::simd::gen_trace;
use stwo_prover::trace_generation::{commit_and_prove, commit_and_verify};

use crate::dvm::types::{
    ContractUploadType, FibonnacciProvingRequest, FibonnacciProvingResponse,
    GenericProvingResponse, PoseidonProvingRequest, ProgramInternalContractName, ProgramParams,
    ProgramRequestType, WideFibonnacciProvingRequest,
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
                                    Err(ProvingError::ConstraintsNotSatisfied.to_string())
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
                                    let component = WideFibComponent {
                                        log_fibonacci_size: wide_fib_req.clone().log_fibonacci_size
                                            + LOG_N_COLUMNS as u32,
                                        log_n_instances: wide_fib_req.log_n_instances,
                                    };
                                    println!("WideFib Air");

                                    let wide_fib = WideFibAir {
                                        component: component.clone(),
                                    };
                                    println!("private_input");

                                    let private_input = (0..(1 << wide_fib_req.log_n_instances))
                                        .map(|i| Input {
                                            a: m31::M31::from_u32_unchecked(i),
                                            b: m31::M31::from_u32_unchecked(i),
                                            // b: m31!(i),
                                        })
                                        .collect();

                                    // let trace = wide_fib.get_trace();
                                    println!("trace");

                                    let trace = gen_trace(&component, private_input);
                                    println!("trace_domain");

                                    let trace_domain =
                                        CanonicCoset::new(component.log_column_size());
                                    println!("trace again");

                                    let trace = trace
                                        .into_iter()
                                        .map(|eval| {
                                            CpuCircleEvaluation::new_canonical_ordered(
                                                trace_domain,
                                                eval,
                                            )
                                        })
                                        .collect();
                                    println!("Create prover channel");

                                    let prover_channel = &mut Blake2sChannel::new(
                                        Blake2sHasher::hash(BaseField::into_slice(&[])),
                                    );
                                    println!("Commit and prove");
                                    let res_proof = commit_and_prove::<CpuBackend>(
                                        &wide_fib,
                                        prover_channel,
                                        trace,
                                    );

                                    match res_proof {
                                        Ok(p) => {
                                            Ok(GenericProvingResponse::new(request.clone(), p))
                                        }
                                        Err(e) => Err(e.to_string()),
                                    }

                                    // if let Some(p) = res_proof.unwrap() {
                                    //     // let cloned = Rc::clone(&p);
                                    //     let verifier_channel = &mut Blake2sChannel::new(
                                    //         Blake2sHasher::hash(BaseField::into_slice(&[])),
                                    //     );
                                    //     commit_and_verify(p, &wide_fib,
                                    // verifier_channel).unwrap();

                                    //     Ok(GenericProvingResponse::new(request.clone(), p))

                                    // } else {
                                    //     Err(ProvingError::ConstraintsNotSatisfied.to_string())

                                    // }

                                    // match wide_fib.prover_components() {
                                    //     Ok(proof) => {
                                    //         Ok(GenericProvingResponse::new(request.clone(),
                                    // proof))     }
                                    //     Err(e) => Err(e.to_string()),
                                    // }

                                    // Err(ProvingError::ConstraintsNotSatisfied.to_string())
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
