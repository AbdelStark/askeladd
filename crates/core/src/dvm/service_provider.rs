//! The prover agent: a NIP-90 service provider that answers proving jobs
//! with STARK proofs.
//!
//! The agent subscribes to job requests on the configured relays, executes
//! the requested program, and publishes the result — output plus proof —
//! back to the network. A local SQLite ledger makes completed jobs sticky,
//! so events redelivered by relay gossip are never executed twice.

use std::error::Error;
use std::time::Duration;

use colored::*;
use log::{debug, error, info, warn};
use nostr_sdk::prelude::*;
use thiserror::Error;

use crate::config::Settings;
use crate::db::{Database, RequestStatus};
use crate::dvm::constants::{JOB_LAUNCH_PROGRAM_KIND, JOB_REQUEST_KIND};
use crate::dvm::types::{GenerateZKPJobRequest, GenerateZKPJobResult};
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
    #[error("invalid prover agent secret key: {0}")]
    InvalidSecretKey(String),
    #[error("invalid database path")]
    InvalidDatabasePath,
    #[error("failed to connect to Nostr relay: {0}")]
    NostrConnectionError(String),
    #[error("failed to subscribe to Nostr events: {0}")]
    NostrSubscriptionError(String),
    #[error("Nostr event builder error: {0}")]
    EventBuilderError(String),
    #[error("database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Nostr client error: {0}")]
    NostrClientError(#[from] nostr_sdk::client::Error),
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
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
        let secret_key = SecretKey::from_bech32(&settings.prover_agent_sk)
            .map_err(|e| ServiceProviderError::InvalidSecretKey(e.to_string()))?;
        let prover_agent_keys = Keys::new(secret_key);
        // Wait for relay acknowledgment so rejected events (e.g. oversized
        // proofs refused by relay policy) surface as errors instead of
        // vanishing silently.
        let opts = Options::new()
            .wait_for_send(true)
            .send_timeout(Some(Duration::from_secs(15)));
        let client = Client::with_opts(&prover_agent_keys, opts);

        // Initialize database
        let db_path = settings
            .db_path
            .to_str()
            .ok_or(ServiceProviderError::InvalidDatabasePath)?;
        let db = Database::new(db_path)?;

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
        crate::nostr_utils::wait_for_relay_connection(&self.nostr_client, Duration::from_secs(10))
            .await
            .map_err(ServiceProviderError::NostrConnectionError)?;
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

        // Subscribe to proving job requests
        self.nostr_client
            .subscribe_with_id(proving_req_sub_id.clone(), vec![filter], None)
            .await
            .map_err(|e| ServiceProviderError::NostrSubscriptionError(e.to_string()))?;

        info!("Subscribed to proving requests, waiting for requests...");

        // Subscribe to program-launch requests (experimental, kind 5700)
        let launch_program_req_id = SubscriptionId::new(&self.settings.launch_program_req_id);
        let filter_launch_program = Filter::new()
            .kind(Kind::Custom(JOB_LAUNCH_PROGRAM_KIND))
            .since(Timestamp::now());

        self.nostr_client
            .subscribe_with_id(
                launch_program_req_id.clone(),
                vec![filter_launch_program],
                None,
            )
            .await
            .map_err(|e| ServiceProviderError::NostrSubscriptionError(e.to_string()))?;

        info!("Subscribed to program launch requests (experimental).");

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
                if let Err(e) = self.handle_proving_request(event).await {
                    error!("Failed to handle proving request: {}", e);
                }
            } else if subscription_id == SubscriptionId::new(&self.settings.launch_program_req_id) {
                Self::handle_launch_request(&event);
            }
        }
        Ok(false)
    }

    /// Handles a single proving request event: parse, deduplicate, prove, publish.
    async fn handle_proving_request(&self, event: Box<Event>) -> Result<(), ServiceProviderError> {
        info!("Proving request received [{}]", event.id);

        let job_id = event.id.to_string();
        let zkp_request: GenerateZKPJobRequest = serde_json::from_str(&event.content)?;

        // Skip jobs already completed; retry jobs that failed before.
        match self.db.get_request_status(&job_id)? {
            Some(RequestStatus::Completed) | Some(RequestStatus::Pending) => {
                info!("Request {} already handled, skipping", &job_id);
                return Ok(());
            }
            Some(RequestStatus::Failed) => {
                info!("Request {} failed before, retrying", &job_id);
            }
            None => {
                self.db.insert_request(&job_id, &zkp_request.request)?;
            }
        }

        match self
            .proving_service
            .generate_proof_by_program(zkp_request.request, zkp_request.program)
        {
            Ok(response) => {
                let job_result =
                    GenerateZKPJobResult::new(job_id.clone(), serde_json::to_value(&response)?);

                let tags = vec![Tag::parse(&["e", job_id.as_str(), "", "reply"])
                    .map_err(|e| ServiceProviderError::EventBuilderError(e.to_string()))?];

                let result_json = serde_json::to_string(&job_result)?;
                let job_result_event = EventBuilder::job_result(*event, result_json, 0, None)
                    .map_err(|e| ServiceProviderError::EventBuilderError(e.to_string()))?
                    .add_tags(tags)
                    .to_event(&self.prover_agent_keys)
                    .map_err(|e| ServiceProviderError::EventBuilderError(e.to_string()))?;

                let result_event_id = job_result_event.id;
                self.nostr_client.send_event(job_result_event).await?;
                info!("Proving response published [{}]", result_event_id);

                self.db.update_request(
                    &job_id,
                    Some(&job_result.response),
                    RequestStatus::Completed,
                )?;
            }
            Err(e) => {
                error!("Proof generation failed for job {}: {}", &job_id, e);
                self.db
                    .update_request(&job_id, None, RequestStatus::Failed)?;
            }
        }

        Ok(())
    }

    /// Program launch (kind 5700) is reserved for the pluggable-program roadmap:
    /// provers will fetch WASM programs published as Nostr events (NIP-94/96)
    /// or from IPFS. Until then, launch requests are acknowledged and ignored.
    fn handle_launch_request(event: &Event) {
        warn!(
            "Program launch request [{}] ignored: uploaded programs are not supported yet.",
            event.id
        );
    }
}

fn print_banner() {
    let askeladd = text_to_ascii_art::to_art("Askeladd".to_string(), "standard", 0, 0, 0)
        .unwrap_or_else(|_| "Askeladd".to_string());
    let zk_proof = text_to_ascii_art::to_art("ZK proof DVM".to_string(), "small", 0, 0, 0)
        .unwrap_or_else(|_| "ZK proof DVM".to_string());

    println!("{}", "*".repeat(80).green());
    println!("\n{}", askeladd.green());
    println!("{}", zk_proof.green());
    println!("{}", "Censorship-resistant global proving network.".green());
    println!("{}", "Powered by Nostr and Circle STARKs.".green());
    println!("{}", "*".repeat(80).green());
}
