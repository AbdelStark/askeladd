//! `dvm_customer` — a client for the Askeladd proving network.
//!
//! Submits verifiable computation jobs to Nostr (NIP-90), waits for a prover
//! agent to answer with a STARK proof, and verifies the proof locally.
//!
//! ## Examples
//!
//! ```bash
//! dvm_customer fibonacci --log-size 5 --claim 443693538
//! dvm_customer poseidon --log-n-instances 8
//! dvm_customer wide-fibonacci --log-fibonacci-size 5 --log-n-instances 5
//! dvm_customer demo
//! ```

use std::collections::HashMap;
use std::io::Write;

use askeladd::config::Settings;
use askeladd::dvm::customer::{Customer, CustomerError};
use askeladd::dvm::types::{
    ContractUploadType, FibonacciProvingRequest, GenerateZKPJobRequest,
    MultiFibonacciProvingRequest, PoseidonProvingRequest, ProgramInternalContractName,
    ProgramParams, WideFibonacciProvingRequest,
};
use clap::{Parser, Subcommand};
use colored::*;
use dotenv::dotenv;
use env_logger::Env;
use log::info;

/// Submit proving jobs to the Askeladd network and verify the STARK proofs
/// that come back. Don't trust — verify.
#[derive(Parser)]
#[command(name = "dvm_customer", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Prove a Fibonacci-squared sequence claim.
    Fibonacci {
        /// log₂ of the sequence length.
        #[arg(long, default_value_t = 5)]
        log_size: u32,
        /// The claimed final value of the sequence.
        #[arg(long, default_value_t = 443693538)]
        claim: u32,
        /// Seconds to wait for a prover before giving up.
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
    /// Prove Poseidon hash permutations.
    Poseidon {
        /// log₂ of the number of permutations (minimum 9).
        #[arg(long, default_value_t = 9)]
        log_n_instances: u32,
        /// Seconds to wait for a prover before giving up.
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
    /// Prove many Fibonacci instances in a single proof.
    WideFibonacci {
        /// log₂ of each sequence's length (minimum 8).
        #[arg(long, default_value_t = 8)]
        log_fibonacci_size: u32,
        /// log₂ of the number of sequences.
        #[arg(long, default_value_t = 8)]
        log_n_instances: u32,
        /// Seconds to wait for a prover before giving up.
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
    /// Prove several Fibonacci claims in a single proof.
    MultiFibonacci {
        /// Comma-separated log₂ sequence lengths (e.g. "5,5").
        #[arg(long, default_value = "5,5")]
        log_sizes: String,
        /// Comma-separated claims (e.g. "443693538,443693538").
        #[arg(long, default_value = "443693538,443693538")]
        claims: String,
        /// Seconds to wait for a prover before giving up.
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
    /// The classic tour: a Fibonacci job and a Poseidon job, end to end.
    Demo {
        /// Seconds to wait for a prover before giving up.
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let cli = Cli::parse();

    println!("{}", "=".repeat(72).green());
    println!(
        "{}",
        "Askeladd DVM Customer — don't trust, verify."
            .bold()
            .green()
    );
    println!("{}", "=".repeat(72).green());

    let mut customer = Customer::new(Settings::new().expect("Failed to load settings"))?;
    customer.init().await?;

    match cli.command {
        Command::Fibonacci {
            log_size,
            claim,
            timeout_secs,
        } => {
            run_job(
                &customer,
                ProgramInternalContractName::FibonacciProvingRequest,
                serde_json::to_value(FibonacciProvingRequest { log_size, claim })?,
                HashMap::from([
                    ("log_size".to_owned(), log_size.to_string()),
                    ("claim".to_owned(), claim.to_string()),
                ]),
                timeout_secs,
            )
            .await?;
        }
        Command::Poseidon {
            log_n_instances,
            timeout_secs,
        } => {
            run_job(
                &customer,
                ProgramInternalContractName::PoseidonProvingRequest,
                serde_json::to_value(PoseidonProvingRequest { log_n_instances })?,
                HashMap::from([("log_n_instances".to_owned(), log_n_instances.to_string())]),
                timeout_secs,
            )
            .await?;
        }
        Command::WideFibonacci {
            log_fibonacci_size,
            log_n_instances,
            timeout_secs,
        } => {
            run_job(
                &customer,
                ProgramInternalContractName::WideFibonacciProvingRequest,
                serde_json::to_value(WideFibonacciProvingRequest {
                    log_fibonacci_size,
                    log_n_instances,
                })?,
                HashMap::from([
                    (
                        "log_fibonacci_size".to_owned(),
                        log_fibonacci_size.to_string(),
                    ),
                    ("log_n_instances".to_owned(), log_n_instances.to_string()),
                ]),
                timeout_secs,
            )
            .await?;
        }
        Command::MultiFibonacci {
            log_sizes,
            claims,
            timeout_secs,
        } => {
            let log_sizes = parse_u32_list(&log_sizes)?;
            let claims = parse_u32_list(&claims)?;
            let inputs = HashMap::from([
                ("log_sizes".to_owned(), log_sizes.clone()),
                ("claims".to_owned(), claims.clone()),
            ]);
            let log_sizes: Vec<u32> = serde_json::from_str(&log_sizes)?;
            let claims: Vec<u32> = serde_json::from_str(&claims)?;
            run_job(
                &customer,
                ProgramInternalContractName::MultiFibonacciProvingRequest,
                serde_json::to_value(MultiFibonacciProvingRequest { log_sizes, claims })?,
                inputs,
                timeout_secs,
            )
            .await?;
        }
        Command::Demo { timeout_secs } => {
            println!("\n{}", "Job 1/2: Fibonacci".bold().cyan());
            run_job(
                &customer,
                ProgramInternalContractName::FibonacciProvingRequest,
                serde_json::to_value(FibonacciProvingRequest {
                    log_size: 5,
                    claim: 443693538,
                })?,
                HashMap::from([
                    ("log_size".to_owned(), "5".to_owned()),
                    ("claim".to_owned(), "443693538".to_owned()),
                ]),
                timeout_secs,
            )
            .await?;

            println!("\n{}", "Job 2/2: Poseidon".bold().cyan());
            run_job(
                &customer,
                ProgramInternalContractName::PoseidonProvingRequest,
                serde_json::to_value(PoseidonProvingRequest { log_n_instances: 9 })?,
                HashMap::from([("log_n_instances".to_owned(), "9".to_owned())]),
                timeout_secs,
            )
            .await?;
        }
    }

    Ok(())
}

/// Parses a comma-separated list of u32s ("5,5") into a JSON array string
/// ("[5,5]"), which both the typed request and the string inputs map accept.
fn parse_u32_list(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let values: Vec<u32> = input
        .split(',')
        .map(|part| part.trim().parse::<u32>())
        .collect::<Result<_, _>>()?;
    Ok(serde_json::to_string(&values)?)
}

/// The full verifiable-computation loop: submit a job, wait for the result,
/// verify the STARK proof attached to it.
async fn run_job(
    customer: &Customer,
    program: ProgramInternalContractName,
    request: serde_json::Value,
    mut inputs: HashMap<String, String>,
    timeout_secs: u64,
) -> Result<(), CustomerError> {
    println!("\n{}", "Submitting job...".cyan());
    inputs.insert("output".to_owned(), "text/json".to_owned());
    let job_request = GenerateZKPJobRequest::new(
        request,
        Some(ProgramParams {
            event_id: None,
            unique_id: None,
            pubkey_application: None,
            inputs: Some(inputs),
            inputs_types: None,
            inputs_encrypted: None,
            contract_reached: ContractUploadType::InternalAskeladd,
            contract_name: None,
            internal_contract_name: Some(program.clone()),
            tags: None,
        }),
    );

    let job_id = customer.submit_job(job_request).await?;
    info!("Job ID: {}", job_id);

    println!("{}", "Waiting for a prover to pick it up...".cyan());
    let job_result = match customer.wait_for_job_result(&job_id, timeout_secs).await {
        Ok(result) => result,
        Err(e) => {
            if matches!(program, ProgramInternalContractName::PoseidonProvingRequest) {
                eprintln!(
                    "{}",
                    "Hint: Poseidon proofs are ~160 KB — many public relays reject events that large. \
                     Use a relay with raised limits (see config/nostr-rs-relay/config.toml)."
                        .yellow()
                );
            }
            return Err(e);
        }
    };
    println!("{}", "Result received. Verifying proof...".cyan());

    match customer.verify_proof(&job_result, &program) {
        Ok(()) => {
            println!("{}", "┌─────────────────────────────────────┐".green());
            println!("{}", "│                                     │".green());
            println!("{}", "│  Proof verification: SUCCESS        │".green());
            println!("{}", "│                                     │".green());
            println!("{}", "└─────────────────────────────────────┘".green());
            println!(
                "{}",
                "The prover's claim checked out — no trust was required.".green()
            );
            Ok(())
        }
        Err(e) => {
            println!("{}", "┌─────────────────────────────────────┐".red());
            println!("{}", "│                                     │".red());
            println!("{}", "│  Proof verification: FAILED         │".red());
            println!("{}", "│                                     │".red());
            println!("{}", "└─────────────────────────────────────┘".red());
            Err(e)
        }
    }
}
