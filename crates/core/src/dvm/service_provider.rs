use std::error::Error;

use log::{debug, error, info};
use nostr_sdk::prelude::*;
use thiserror::Error;

use crate::config::Settings;
use crate::db::{Database, RequestStatus};
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
        let proving_req_sub_id = SubscriptionId::new(&self.settings.proving_req_sub_id);
        let filter = Filter::new().kind(Kind::TextNote).since(Timestamp::now());

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
        debug!("Event received [{}]", event.id);
        if let Ok(job_request) = serde_json::from_str::<GenerateZKPJobRequest>(&event.content) {
            info!("Proving request received [{}]", event.id);
            let request = job_request.request;

            if let Some(status) = self.db.get_request_status(&job_request.job_id)? {
                match status {
                    RequestStatus::Completed => {
                        info!("Request {} already processed, skipping", job_request.job_id);
                        return Ok(());
                    }
                    RequestStatus::Failed => {
                        info!("Request {} failed before, retrying", job_request.job_id);
                    }
                    RequestStatus::Pending => {
                        info!(
                            "Request {} is already pending, skipping",
                            job_request.job_id
                        );
                        return Ok(());
                    }
                }
            } else {
                self.db.insert_request(&job_request.job_id, &request)?;
            }

            match self.proving_service.generate_proof(request) {
                Ok(response) => {
                    let job_result = GenerateZKPJobResult {
                        job_id: job_request.job_id.clone(),
                        response,
                    };
                    let response_json = serde_json::to_string(&job_result)?;
                    let event_id = self
                        .nostr_client
                        .publish_text_note(response_json, vec![])
                        .await?;
                    info!("Proving response published [{}]", event_id.to_string());

                    self.db.update_request(
                        &job_request.job_id,
                        Some(&job_result.response),
                        RequestStatus::Completed,
                    )?;
                }
                Err(e) => {
                    error!("Proof generation failed: {}", e);
                    self.db
                        .update_request(&job_request.job_id, None, RequestStatus::Failed)?;
                }
            }
        } else {
            debug!("Received non-request event, ignoring");
        }

        Ok(())
    }
}
