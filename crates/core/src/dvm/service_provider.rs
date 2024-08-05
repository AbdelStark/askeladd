// use std::collections::HashMap;
use std::error::Error;

use colored::*;
use log::{debug, error, info};
use nostr_sdk::prelude::*;
use serde_json::Result as SerdeResult;
use thiserror::Error;

use crate::config::Settings;
use crate::db::{Database, RequestStatus};
use crate::dvm::constants::{JOB_LAUNCH_PROGRAM_KIND, JOB_REQUEST_KIND};
use crate::dvm::types::{GenerateZKPJobRequest, GenerateZKPJobResult, ProgramParams};
// use crate::nostr_utils::extract_params_from_tags;
use crate::prover_service::ProverService;
use crate::utils::convert_inputs_to_run_program;

/// ServiceProvider is the main component of the Askeladd prover agent.
/// It manages the lifecycle of proving requests, from receiving them via Nostr,
/// to generating proofs and publishing the results.
///
/// The ServiceProvider integrates with a Nostr client for communication,
/// a database for persistence, and a proving service for generating proofs.
pub struct ServiceProvider {
    /// Application settings
    settings: Settings,
    /// Prover Agent Nostr keys
    prover_agent_keys: Keys,
    /// Service for generating proofs
    proving_service: ProverService,
    /// Nostr client for communication
    nostr_client: Client,
    /// Database for persisting request states
    db: Database,
}

/// Errors that can occur during ServiceProvider operations
#[derive(Error, Debug)]
pub enum ServiceProviderError {
    #[error("Failed to connect to Nostr relay: {0}")]
    NostrConnectionError(String),
    #[error("Failed to subscribe to Nostr events: {0}")]
    NostrSubscriptionError(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Nostr client error: {0}")]
    NostrClientError(#[from] nostr_sdk::client::Error),
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Unknown error")]
    Unknown,
    #[error("No program param")]
    NoProgramParam,
}

impl ServiceProvider {
    /// Creates a new ServiceProvider instance
    ///
    /// # Arguments
    ///
    /// * `settings` - Application settings
    ///
    /// # Returns
    ///
    /// A Result containing the new ServiceProvider or an error
    pub fn new(settings: Settings) -> Result<Self, ServiceProviderError> {
        // Initialize Nostr keys and client
        let prover_agent_keys =
            Keys::new(SecretKey::from_bech32(&settings.prover_agent_sk).unwrap());
        let opts = Options::new().wait_for_send(false);
        let client = Client::with_opts(&prover_agent_keys, opts);

        // Initialize database
        let db = Database::new(settings.db_path.to_str().unwrap())?;

        Ok(Self {
            settings,
            prover_agent_keys,
            proving_service: Default::default(),
            nostr_client: client,
            db,
        })
    }

    /// Initializes the ServiceProvider by connecting to Nostr relays
    pub async fn init(&mut self) -> Result<(), ServiceProviderError> {
        // Connect to all configured relays
        for relay in &self.settings.subscribed_relays {
            self.nostr_client
                .add_relay(relay)
                .await
                .map_err(|e| ServiceProviderError::NostrConnectionError(e.to_string()))?;
        }
        self.nostr_client.connect().await;
        debug!("Nostr client connected to relays.");
        Ok(())
    }

    /// Runs the main loop of the ServiceProvider
    ///
    /// This method subscribes to Nostr events and handles incoming proving requests
    pub async fn run(&self) -> Result<(), ServiceProviderError> {
        print_banner();
        let proving_req_sub_id = SubscriptionId::new(&self.settings.proving_req_sub_id);
        let filter = Filter::new()
            .kind(Kind::Custom(JOB_REQUEST_KIND))
            .since(Timestamp::now());

        // Subscribe to Nostr events
        self.nostr_client
            .subscribe_with_id(proving_req_sub_id.clone(), vec![filter], None)
            .await
            .map_err(|e| ServiceProviderError::NostrSubscriptionError(e.to_string()))?;

        info!("Subscribed to proving requests, waiting for requests...");

        // Start handling Nostr notifications
        self.nostr_client
            .handle_notifications(|notification| async {
                match self.handle_notification(notification).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(Box::new(e) as Box<dyn Error>),
                }
            })
            .await?;

        // Start JOB LAUNCH PROGRAM subscription
        let launch_program_req_id = SubscriptionId::new(&self.settings.launch_program_req_id);
        let filter_launch_program = Filter::new()
            .kind(Kind::Custom(JOB_LAUNCH_PROGRAM_KIND))
            .since(Timestamp::now());

        // Subscribe to LAUCH_PROGRAM DVM KIND event
        self.nostr_client
            .subscribe_with_id(
                launch_program_req_id.clone(),
                vec![filter_launch_program],
                None,
            )
            .await
            .map_err(|e| ServiceProviderError::NostrSubscriptionError(e.to_string()))?;

        // Start handling LAUNCH_PROGRAM
        self.nostr_client
            .handle_notifications(|notification| async {
                match self.handle_notification(notification).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(Box::new(e) as Box<dyn Error>),
                }
            })
            .await?;

        Ok(())
    }

    /// Handles incoming Nostr notifications
    async fn handle_notification(
        &self,
        notification: RelayPoolNotification,
    ) -> Result<bool, ServiceProviderError> {
        if let RelayPoolNotification::Event {
            subscription_id,
            event,
            ..
        } = notification
        {
            if subscription_id == SubscriptionId::new(&self.settings.proving_req_sub_id) {
                let _ = self.handle_event(event).await;
            } else if subscription_id == SubscriptionId::new(&self.settings.launch_program_req_id) {
                let _ = self.handle_event_launch_program(event).await;
            }
        }
        Ok(false)
    }

    fn deserialize_zkp_request_data(
        json_data: &str,
    ) -> Result<GenerateZKPJobRequest, ServiceProviderError> {
        let zkp_request: SerdeResult<GenerateZKPJobRequest> = serde_json::from_str(json_data);
        zkp_request.map_err(ServiceProviderError::SerializationError)
    }

    /// Handles a single proving request event
    async fn handle_event(&self, event: Box<Event>) -> Result<(), ServiceProviderError> {
        info!("Proving request received [{}]", event.id);

        let job_id = event.id.to_string();
        // let tags = &event.tags;
        // let params = extract_params_from_tags(tags);

        let zkp_request = ServiceProvider::deserialize_zkp_request_data(&event.content.to_owned())?;
        // println!("request value {:?}", request_value);
        println!("zkp_request {:?}", zkp_request);
        let params_program: Option<ProgramParams> = zkp_request.program.clone();
        let params_inputs;
        // let mut successful_parses = HashMap::new();
        // let mut successful_parses;

        // TODO Check strict if user have sent a good request
        if let Some(program_params) = params_program.clone() {
            println!("params_program {:?}", params_program);

            let successful_parses = convert_inputs_to_run_program(program_params.inputs);
            // params_inputs = program_params.inputs.clone();
            params_inputs = successful_parses.clone();
            println!("params_inputs {:?}", params_inputs);
        } else {
            println!("program_params {:?}", params_program);
        }

        // for (key, value) in params_inputs.into_iter() {
        //     println!("{} / {}", key, value);
        //     let tag = Tag::parse(&["param", &key.to_owned(), &value.to_owned()]);
        //     tags.push(tag.unwrap())
        //     // map.remove(key);
        // }

        // let log_size = params
        //     .get("log_size")
        //     .and_then(|s| s.parse::<u32>().ok())
        //     .unwrap();
        // let claim = params
        //     .get("claim")
        //     .and_then(|s| s.parse::<u32>().ok())
        //     .unwrap();

        // let request = FibonnacciProvingRequest { log_size, claim };
        let request_str = serde_json::to_string(&zkp_request.request).unwrap();
        // let request_str = serde_json::to_string(&request).unwrap();
        let request_value = serde_json::from_str(&request_str).unwrap();

        println!("request_str {:?}", request_str);

        if let Some(status) = self.db.get_request_status(&job_id)? {
            match status {
                RequestStatus::Completed => {
                    info!("Request {} already processed, skipping", &job_id);
                    return Ok(());
                }
                RequestStatus::Failed => {
                    info!("Request {} failed before, retrying", &job_id);
                }
                RequestStatus::Pending => {
                    info!("Request {} is already pending, skipping", &job_id);
                    return Ok(());
                }
            }
        } else {
            self.db.insert_request(&job_id, &request_value)?;
        }

        match self
            .proving_service
            .generate_proof_by_program(request_value, params_program)
        {
            Ok(response) => {
                let serialized_proof = serde_json::to_string(&response.proof)?;
                println!("Generated proof: {:?}", serialized_proof);
                let answer_string = serde_json::to_string(&response).unwrap();
                let value_answer: Value = serde_json::from_str(&answer_string)?;

                let job_result = GenerateZKPJobResult {
                    job_id: job_id.clone(),
                    response: value_answer,
                    // response:serde_json::from_value(response.clone()).unwrap(),
                    proof: response.proof,
                };

                let response_json = serde_json::to_string(&job_result)?;
                println!("Response JSON: {:?}", response_json);

                let job_result_event: Event =
                    EventBuilder::job_result(*event, response_json, 0, None)
                        .unwrap()
                        .to_event(&self.prover_agent_keys)
                        .unwrap();

                let event_id = self.nostr_client.send_event(job_result_event).await?;
                info!("Proving response published [{}]", event_id.to_string());

                self.db.update_request(
                    &job_id,
                    Some(&job_result.response),
                    RequestStatus::Completed,
                )?;
            }
            Err(e) => {
                error!("Proof generation failed: {}", e);
                self.db
                    .update_request(&job_id, None, RequestStatus::Failed)?;
            }
        }

        Ok(())
    }

    // @TODO finish implement launch program with NIP-78, 94 and 96
    async fn handle_event_launch_program(
        &self,
        event: Box<Event>,
    ) -> Result<(), ServiceProviderError> {
        info!("LAUNCH_PROGRAM request received [{}]", event.id);

        let job_id = event.id.to_string();
        // let tags = &event.tags;
        // let params = extract_params_from_tags(tags);

        // Deserialze content
        let zkp_request = ServiceProvider::deserialize_zkp_request_data(&event.content.to_owned())?;
        // let params_program: Option<ProgramParams> = zkp_request.program.clone();

        // Request on the content
        // Check request of the launch_program
        let request_str = serde_json::to_string(&zkp_request.request).unwrap();
        // let request_str = serde_json::to_string(&request).unwrap();
        let request_value = serde_json::from_str(&request_str).unwrap();

        // TAGS
        let program_str = serde_json::to_string(&zkp_request.program).unwrap();
        let program_value = serde_json::from_str(&program_str).unwrap();

        // Look if this program is already launched and save
        if let Some(status) = self.db.get_program_status(&job_id)? {
            match status {
                RequestStatus::Completed => {
                    info!("Request {} already processed, skipping", &job_id);
                    return Ok(());
                }
                RequestStatus::Failed => {
                    info!("Request {} failed before, retrying", &job_id);
                }
                RequestStatus::Pending => {
                    info!("Request {} is already pending, skipping", &job_id);
                    return Ok(());
                }
            }
        } else {
            self.db
                .insert_program_launched(&job_id, &request_value, &program_value)?;
        }

        // Look program param

        // Get URL and verify:
        // @TODO NIP-78 and NIP-94 and 96 to be implemented
        // Backend endpoint
        // WASM program
        // Maybe other way to do it
        Ok(())
    }
}

fn print_banner() {
    let askeladd = text_to_ascii_art::to_art("Askeladd".to_string(), "standard", 0, 0, 0).unwrap();
    let zk_proof = text_to_ascii_art::to_art("ZK proof DVM".to_string(), "small", 0, 0, 0).unwrap();

    println!("{}", "*".repeat(80).green());
    println!("\n{}", askeladd.green());
    println!("{}", zk_proof.green());
    println!("{}", "Censorship global proving network".green());
    println!("{}", "Powered by Nostr and Circle STARKs.".green());
    println!("{}", "*".repeat(80).green());
}
