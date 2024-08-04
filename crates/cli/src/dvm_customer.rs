use std::collections::HashMap;
use std::io::Write;
use std::thread;
use std::time::Duration;

use askeladd::config::Settings;
use askeladd::dvm::customer::{Customer, CustomerError};
use askeladd::dvm::types::{
    ContractUploadType, FibonnacciProvingRequest, GenerateZKPJobRequest, PoseidonProvingRequest,
    ProgramInternalContractName, ProgramParams,
};
use colored::*;
use dotenv::dotenv;
use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ******************************************************
    // ****************** SETUP *****************************
    // ******************************************************
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
                    .blue(),
                record.level().to_string().yellow(),
                record.args()
            )
        })
        .init();

    println!("{}", "=".repeat(80).green());
    println!("{}", "Askeladd DVM Customer".bold().green());
    println!("{}", "=".repeat(80).green());

    let settings = Settings::new().expect("Failed to load settings");

    // ******************************************************
    // ****************** INIT CUSTOMER *********************
    // ******************************************************
    println!("\n{}", "Initializing Customer...".cyan());
    let mut customer = Customer::new(settings)?;
    customer.init().await?;
    println!("{}", "Customer initialized successfully.".green());

    // ******************************************************
    // ****************** PREPARE JOB ***********************
    // ******************************************************
    println!("\n{}", "Preparing job...".cyan());

    let mut map_inputs = HashMap::<String, String>::new();
    map_inputs.insert("log_size".to_owned(), "5".to_owned());
    map_inputs.insert("claim".to_owned(), "443693538".to_owned());
    map_inputs.insert("output".to_owned(), "text/json".to_owned());
    let req_value = serde_json::to_value(FibonnacciProvingRequest {
        log_size: 5,
        claim: 443693538,
    })
    .unwrap();
    let job_request = GenerateZKPJobRequest {
        request: req_value,
        program: Some(ProgramParams {
            pubkey_application: None,
            inputs: map_inputs,
            inputs_encrypted: None,
            inputs_types: None,
            unique_id: None,
            event_id: None,
            contract_reached: ContractUploadType::InternalAskeladd,
            contract_name: Some("FibonacciProvingRequest".to_owned()),
            internal_contract_name: Some(ProgramInternalContractName::FibonnacciProvingRequest),
        }),
    };
    println!("{}", "Job prepared successfully.".green());

    // ******************************************************
    // ****************** SUBMIT JOB ************************
    // ******************************************************
    println!("\n{}", "Submitting job...".cyan());
    let job_id = customer.submit_job(job_request.clone()).await?;
    println!("{}", "Job submitted successfully.".green());
    info!("Job ID: {}", job_id);

    // ******************************************************
    // ****************** WAIT FOR JOB RESULT ***************
    // ******************************************************
    println!("\n{}", "Waiting for job result...".cyan());
    let job_result = customer.wait_for_job_result(&job_id, 60).await?;
    println!("{}", "Job result received.".green());

    // ******************************************************
    // ****************** VERIFY PROOF **********************
    // ******************************************************
    println!("\n{}", "Preparing to verify proof...".cyan());
    for i in (1..=3).rev() {
        print!("\rVerifying proof in {} seconds...", i);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!("\n");

    let is_valid = customer.verify_proof(&job_result)?;

    if is_valid {
        println!("{}", "┌─────────────────────────────────────┐".green());
        println!("{}", "│                                     │".green());
        println!("{}", "│  Proof Verification: SUCCESSFUL     │".green());
        println!("{}", "│                                     │".green());
        println!("{}", "└─────────────────────────────────────┘".green());
    } else {
        println!("{}", "┌─────────────────────────────────────┐".red());
        println!("{}", "│                                     │".red());
        println!("{}", "│   Proof Verification: FAILED        │".red());
        println!("{}", "│                                     │".red());
        println!("{}", "└─────────────────────────────────────┘".red());
    }

    // /// Add poseidon
    // let settings = Settings::new().expect("Failed to load settings");

    // let mut customer = Customer::new(settings)?;
    // customer.init().await?;

    // poseidon_program(customer).await?;

    Ok(())
}

pub async fn poseidon_program(customer: Customer) -> Result<(), CustomerError> {
    println!("poseidon program");
    let mut map_inputs = HashMap::<String, String>::new();

    let log_n_instances = 5;
    map_inputs.insert("log_n_instances".to_owned(), log_n_instances.to_string());
    map_inputs.insert("output".to_owned(), "text/json".to_owned());

    let req_value = serde_json::to_value(PoseidonProvingRequest { log_n_instances }).unwrap();

    let job_request = GenerateZKPJobRequest {
        request: req_value,
        program: Some(ProgramParams {
            inputs: map_inputs,
            pubkey_application: None,
            inputs_encrypted: None,
            inputs_types: None,
            unique_id: None,
            event_id: None,
            contract_reached: ContractUploadType::InternalAskeladd,
            contract_name: Some("PoseidonProvingRequest".to_owned()),
            internal_contract_name: Some(ProgramInternalContractName::PoseidonProvingRequest),
        }),
    };
    println!("{}", "Job prepared successfully.".green());

    // ******************************************************
    // ****************** SUBMIT JOB ************************
    // ******************************************************
    println!("\n{}", "Submitting job...".cyan());
    let job_id = customer.submit_job(job_request.clone()).await?;
    println!("{}", "Job submitted successfully.".green());
    info!("Job ID: {}", job_id);

    // ******************************************************
    // ****************** WAIT FOR JOB RESULT ***************
    // ******************************************************
    println!("\n{}", "Waiting for job result...".cyan());
    let job_result = customer.wait_for_job_result(&job_id, 60).await?;
    println!("{}", "Job result received.".green());

    // ******************************************************
    // ****************** VERIFY PROOF **********************
    // ******************************************************
    println!("\n{}", "Preparing to verify proof...".cyan());
    for i in (1..=3).rev() {
        print!("\rVerifying proof in {} seconds...", i);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!("\n");

    let is_valid = customer.verify_proof(&job_result)?;

    if is_valid {
        println!("{}", "┌─────────────────────────────────────┐".green());
        println!("{}", "│                                     │".green());
        println!("{}", "│  Proof Verification: SUCCESSFUL     │".green());
        println!("{}", "│                                     │".green());
        println!("{}", "└─────────────────────────────────────┘".green());
    } else {
        println!("{}", "┌─────────────────────────────────────┐".red());
        println!("{}", "│                                     │".red());
        println!("{}", "│   Proof Verification: FAILED        │".red());
        println!("{}", "│                                     │".red());
        println!("{}", "└─────────────────────────────────────┘".red());
    }

    Ok(())
}
