pub mod customer;
pub mod service_provider;

pub mod constants {

    pub const DVM_NAME: &str = "askeladd";
    pub const DVM_DESCRIPTION: &str = "Censorship-resistant global proving network.";
    pub const SERVICE_NAME: &str = "generate-zk-proof";
    pub const VERSION: &str = "0.1.0";
    pub const JOB_REQUEST_KIND: u64 = 5600;
    pub const JOB_RESULT_KIND: u64 = 6600;
}

pub mod types {
    use serde::{Deserialize, Serialize};
    use stwo_prover::core::prover::StarkProof;
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct GenerateZKPJobRequest {
        pub job_id: String,
        pub request: FibonnacciProvingRequest,
    }

    impl GenerateZKPJobRequest {
        pub fn new(request: FibonnacciProvingRequest) -> Self {
            Self {
                job_id: Self::new_job_id(),
                request,
            }
        }

        pub fn new_job_id() -> String {
            Uuid::new_v4().to_string()
        }
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
        pub response: FibonnacciProvingResponse,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FibonnacciProvingRequest {
        pub log_size: u32,
        pub claim: u32,
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
}
