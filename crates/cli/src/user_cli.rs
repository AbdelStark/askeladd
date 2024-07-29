use askeladd::config::Settings;
use askeladd::dvm::customer::Customer;
use askeladd::dvm::types::{FibonnacciProvingRequest, GenerateZKPJobRequest};
use dotenv::dotenv;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ******************************************************
    // ****************** SETUP *****************************
    // ******************************************************
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    dotenv().ok();
    let settings = Settings::new().expect("Failed to load settings");

    // ******************************************************
    // ****************** INIT CUSTOMER *********************
    // ******************************************************
    let mut customer = Customer::new(settings)?;
    customer.init().await?;

    // ******************************************************
    // ****************** PREPARE JOB ***********************
    // ******************************************************
    let job_request = GenerateZKPJobRequest {
        request: FibonnacciProvingRequest {
            log_size: 5,
            claim: 443693538,
        },
    };

    // ******************************************************
    // ****************** SUBMIT JOB ************************
    // ******************************************************
    info!("Submitting job");
    let job_id = customer.submit_job(job_request.clone()).await?;
    info!("Job submitted with id: {}", job_id);

    // ******************************************************
    // ****************** WAIT FOR JOB RESULT ***************
    // ******************************************************
    info!("Waiting for job result with id: {}", job_id);
    let job_result = customer.wait_for_job_result(&job_id, 60).await?;

    // ******************************************************
    // ****************** VERIFY PROOF **********************
    // ******************************************************
    info!("Verifying proof with id: {}", job_id);
    let is_valid = customer.verify_proof(&job_result)?;
    info!("Proof is valid: {}", is_valid);

    Ok(())
}
