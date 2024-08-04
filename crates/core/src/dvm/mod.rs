pub mod customer;
pub mod service_provider;

pub mod constants {

    pub const DVM_NAME: &str = "askeladd";
    pub const DVM_DESCRIPTION: &str = "Censorship-resistant global proving network.";
    pub const SERVICE_NAME: &str = "generate-zk-proof";
    pub const VERSION: &str = "0.1.0";
    pub const JOB_REQUEST_KIND: u16 = 5600;
    pub const JOB_RESULT_KIND: u16 = 6600;
    pub const JOB_LAUNCH_PROGRAM_KIND: u16 = 5700;
}

pub mod types {
    use std::collections::HashMap;

    use nostr_sdk::EventId;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use stwo_prover::core::prover::StarkProof;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct GenerateZKPJobRequest {
        pub request: serde_json::Value,
        // pub program: ProgramParams,
        pub program: Option<ProgramParams>,
    }

    impl GenerateZKPJobRequest {
        pub fn new(request: Value, program: Option<ProgramParams>) -> Self {
            Self { request, program }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum ContractUploadType {
        InternalAskeladd,
        // URl,
        // BackendEndpoint,
        // Ipfs,
    }

    // Enum for internal_name program on ASKELADD
    #[derive(Serialize, Deserialize, Debug, Clone)]
    // #[serde(untagged)]
    pub enum ProgramInternalContractName {
        FibonnacciProvingRequest,
        PoseidonProvingRequest,
        WideFibonnaciProvingRequest,
        MultiFibonnaciProvingRequest,
        Custom(String),
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ProgramParams {
        pub event_id: Option<EventId>,
        pub unique_id: Option<String>,
        pub pubkey_application: Option<String>, /* Use for one to one marketplace => difficult
                                                 * on the archi of the DVM */
        pub inputs: HashMap<String, String>,
        pub inputs_types: Option<HashMap<String, String>>,
        pub inputs_encrypted: Option<HashMap<String, String>>,
        pub contract_reached: ContractUploadType,
        pub contract_name: Option<String>,
        pub internal_contract_name: Option<ProgramInternalContractName>,
        // For External program
        // pub endpoint:Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum JobStatus {
        Pending,
        Completed,
        Failed,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GenerateZKPJobResult {
        pub job_id: String,
        // pub response: T,
        pub response: serde_json::Value,
        pub proof: StarkProof,
    }

    impl GenerateZKPJobResult {
        pub fn new(job_id: String, response: Value, proof: StarkProof) -> Self {
            Self {
                job_id,
                response,
                proof,
            }
        }
        pub fn deserialize_container<'a>(
            json_data: &'static str,
        ) -> Result<GenerateZKPJobResult, serde_json::Error> {
            serde_json::from_str(json_data)
        }
    }

    /// Generic type for proving response

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GenericProvingResponse {
        pub response: Value,
        pub proof: StarkProof,
    }

    impl GenericProvingResponse {
        pub fn new(response: Value, proof: StarkProof) -> Self {
            Self { proof, response }
        }
    }

    impl Clone for GenericProvingResponse {
        fn clone(&self) -> Self {
            // Temporarily use serde for a dirty clone
            // TODO: Implement a proper clone or find a better design that does not require cloning
            // the proof
            let proof_json = serde_json::to_string(&self.proof).unwrap();
            let proof = serde_json::from_str(&proof_json).unwrap();
            Self {
                proof,
                response: self.response.clone(),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FibonnacciProvingRequest {
        pub log_size: u32,
        pub claim: u32,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct MultiFibonnacciProvingRequest {
        pub log_sizes: Vec<u32>,
        pub claims: Vec<u32>,
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
            // TODO: Implement a proper clone or find a better design that does not require cloning
            // the proof
            let proof_json = serde_json::to_string(&self.proof).unwrap();
            let proof = serde_json::from_str(&proof_json).unwrap();
            Self {
                log_size: self.log_size,
                claim: self.claim,
                proof,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct WideFibonnacciProvingRequest {
        pub log_fibonacci_size: u32,
        pub log_n_instances: u32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct WideFibonnacciProvingResponse {
        pub log_size: u32,
        pub claim: u32,
        pub proof: StarkProof,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PoseidonProvingRequest {
        pub log_n_instances: u32,
        // pub lookup_elements: stwo_prover::constraint_framework::logup::LookupElements,
        // pub claimed_sum: stwo_prover::core::fields::qm31::SecureField,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PoseidonProvingResponse {
        pub response: Value,
        pub proof: StarkProof,
    }
}
