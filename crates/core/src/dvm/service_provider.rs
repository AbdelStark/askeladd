use std::error::Error;

use colored::*;
use log::{debug, error, info};
use nostr_sdk::prelude::*;
use thiserror::Error;

use crate::config::Settings;
use crate::db::{Database, RequestStatus};
use crate::dvm::constants::JOB_REQUEST_KIND;
use crate::dvm::types::{FibonnacciProvingRequest, GenerateZKPJobResult};
use crate::nostr_utils::extract_params_from_tags;
use crate::prover_service::ProverService;

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
                self.handle_event(event).await?;
            }
        }
        Ok(false)
    }

    /// Handles a single proving request event
    async fn handle_event(&self, event: Box<Event>) -> Result<(), ServiceProviderError> {
        info!("Proving request received [{}]", event.id);

        let job_id = event.id.to_string();
        let tags = &event.tags;
        let params = extract_params_from_tags(tags);
        let log_size = params
            .get("log_size")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap();
        let claim = params
            .get("claim")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap();

        let request = FibonnacciProvingRequest { log_size, claim };

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
            self.db.insert_request(&job_id, &request)?;
        }

        match self.proving_service.generate_proof(request) {
            Ok(response) => {
                let job_result = GenerateZKPJobResult {
                    job_id: job_id.clone(),
                    response,
                };
                let response_json = serde_json::to_string(&job_result)?;

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
