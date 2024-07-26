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
    use uuid::Uuid;

    use crate::types::{FibonnacciProvingRequest, FibonnacciProvingResponse};

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
}
