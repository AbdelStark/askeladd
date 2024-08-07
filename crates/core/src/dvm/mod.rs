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

    use nostr_sdk::{EventId, Tag};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use stwo_prover::core::prover::StarkProof;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;

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
        Ipfs,
        // URl,
        // BackendEndpoint,
    }

    // Enum for internal_name program on ASKELADD
    #[derive(Serialize, Deserialize, Debug, Clone)]
    // #[serde(untagged)]
    pub enum ProgramInternalContractName {
        FibonnacciProvingRequest,
        PoseidonProvingRequest,
        WideFibonacciProvingRequest,
        MultiFibonacciProvingRequest,
        Custom(String),
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ProgramParams {
        pub event_id: Option<EventId>,
        pub unique_id: Option<String>,
        pub pubkey_application: Option<String>, /* Use for one to one marketplace => difficult
                                                 * on the archi of the DVM */
        pub inputs: Option<HashMap<String, String>>,
        pub inputs_types: Option<HashMap<String, String>>,
        pub inputs_encrypted: Option<HashMap<String, String>>,
        pub contract_reached: ContractUploadType,
        pub contract_name: Option<String>,
        pub internal_contract_name: Option<ProgramInternalContractName>,
        pub tags: Option<Vec<Tag>>,
        // todo config payment and minimal sats
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
        pub response: serde_json::Value,
        pub proof: StarkProof<Blake2sMerkleHasher>,
    }

    impl GenerateZKPJobResult {
        pub fn new(
            job_id: String,
            response: Value,
            proof: StarkProof<Blake2sMerkleHasher>,
        ) -> Self {
            Self {
                job_id,
                response,
                proof,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GenericProvingResponse {
        pub response: Value,
        pub proof: StarkProof<Blake2sMerkleHasher>,
    }

    impl GenericProvingResponse {
        pub fn new(response: Value, proof: StarkProof<Blake2sMerkleHasher>) -> Self {
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
    pub struct FibonacciProvingRequest {
        pub log_size: u32,
        pub claim: u32,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct MultiFibonacciProvingRequest {
        pub log_sizes: Vec<u32>,
        pub claims: Vec<u32>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FibonacciProvingResponse {
        pub log_size: u32,
        pub claim: u32,
        pub proof: StarkProof<Blake2sMerkleHasher>,
    }

    impl FibonacciProvingResponse {
        pub fn new(log_size: u32, claim: u32, proof: StarkProof<Blake2sMerkleHasher>) -> Self {
            Self {
                log_size,
                claim,
                proof,
            }
        }
    }

    impl Clone for FibonacciProvingResponse {
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
    pub struct WideFibonacciProvingRequest {
        pub log_fibonacci_size: u32,
        pub log_n_instances: u32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct WideFibonacciProvingResponse {
        pub log_size: u32,
        pub claim: u32,
        pub proof: StarkProof<Blake2sMerkleHasher>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PoseidonProvingRequest {
        pub log_n_instances: u32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PoseidonProvingResponse {
        pub response: Value,
        pub proof: StarkProof<Blake2sMerkleHasher>,
    }
}
