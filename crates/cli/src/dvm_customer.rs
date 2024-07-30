use std::io::Write;
use std::thread;
use std::time::Duration;

use askeladd::config::Settings;
use askeladd::dvm::customer::Customer;
use askeladd::dvm::types::{FibonnacciProvingRequest, GenerateZKPJobRequest};
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
    let job_request = GenerateZKPJobRequest {
        request: FibonnacciProvingRequest {
            log_size: 5,
            claim: 443693538,
        },
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
