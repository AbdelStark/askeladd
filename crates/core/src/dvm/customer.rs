use std::time::Duration;

use log::{debug, error, info};
use nostr_sdk::prelude::*;
use thiserror::Error;
use tokio::time::timeout;

use crate::config::Settings;
use crate::dvm::types::{GenerateZKPJobRequest, GenerateZKPJobResult};
use crate::verifier_service::VerifierService;

/// Represents a customer in the Askeladd system.
///
/// The `Customer` struct is responsible for interacting with the Nostr network,
/// submitting job requests, waiting for job results, and verifying proofs.
pub struct Customer {
    /// Application settings
    settings: Settings,
    /// Nostr client for network communication
    nostr_client: Client,
    /// Service for verifying proofs
    verifier_service: VerifierService,
}

/// Errors that can occur during Customer operations
#[derive(Error, Debug)]
pub enum CustomerError {
    #[error("Failed to connect to Nostr relay: {0}")]
    NostrConnectionError(String),
    #[error("Failed to subscribe to Nostr events: {0}")]
    NostrSubscriptionError(String),
    #[error("Nostr client error: {0}")]
    NostrClientError(#[from] nostr_sdk::client::Error),
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Proof verification error: {0}")]
    VerificationError(String),
    #[error("Job timed out: {0}")]
    JobTimeout(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Customer {
    /// Creates a new Customer instance
    pub fn new(settings: Settings) -> Result<Self, CustomerError> {
        let user_keys = Keys::new(SecretKey::from_bech32(&settings.user_bech32_sk).unwrap());
        let opts = Options::new().wait_for_send(false);
        let client = Client::with_opts(&user_keys, opts);

        Ok(Self {
            settings,
            nostr_client: client,
            verifier_service: Default::default(),
        })
    }

    /// Initializes the Customer by connecting to Nostr relays
    pub async fn init(&mut self) -> Result<(), CustomerError> {
        for relay in &self.settings.subscribed_relays {
            self.nostr_client
                .add_relay(relay)
                .await
                .map_err(|e| CustomerError::NostrConnectionError(e.to_string()))?;
        }
        self.nostr_client.connect().await;
        debug!("Nostr client connected to relays.");
        Ok(())
    }

    /// Submits a job request to the Nostr network
    pub async fn submit_job(&self, job: GenerateZKPJobRequest) -> Result<(), CustomerError> {
        let request_json = serde_json::to_string(&job)?;
        debug!("Publishing proving request...");
        let event_id = self
            .nostr_client
            .publish_text_note(request_json, [])
            .await?;
        info!("Proving request published [{}]", event_id.to_string());
        Ok(())
    }

    /// Waits for a job result from the Nostr network
    pub async fn wait_for_job_result(
        &self,
        job_id: &str,
        timeout_secs: u64,
    ) -> Result<GenerateZKPJobResult, CustomerError> {
        let proving_resp_sub_id = SubscriptionId::new(&self.settings.proving_resp_sub_id);
        let prover_agent_public_key = PublicKey::from_bech32(&self.settings.prover_agent_pk)
            .map_err(|e| CustomerError::Unknown(format!("Failed to parse public key: {}", e)))?;

        // Set up a filter for the job result events
        let filter = Filter::new()
            .kind(Kind::TextNote)
            .author(prover_agent_public_key)
            .since(Timestamp::now() - Duration::from_secs(60));

        // Subscribe to the Nostr events
        self.nostr_client
            .subscribe_with_id(proving_resp_sub_id.clone(), vec![filter], None)
            .await
            .map_err(|e| CustomerError::NostrSubscriptionError(e.to_string()))?;

        // Wait for the job result with a timeout
        let timeout_duration = Duration::from_secs(timeout_secs);
        timeout(
            timeout_duration,
            self.listen_for_job_result(job_id, proving_resp_sub_id),
        )
        .await
        .map_err(|_| CustomerError::JobTimeout(job_id.to_string()))?
    }

    /// Listens for a specific job result from the Nostr network
    async fn listen_for_job_result(
        &self,
        job_id: &str,
        subscription_id: SubscriptionId,
    ) -> Result<GenerateZKPJobResult, CustomerError> {
        let job_id = job_id.to_string();
        let subscription_id = subscription_id.clone();

        // Handle incoming Nostr notifications
        self.nostr_client
            .handle_notifications(|notification| {
                let job_id = job_id.clone();
                let subscription_id = subscription_id.clone();
                async move {
                    if let RelayPoolNotification::Event {
                        subscription_id: sub_id,
                        event,
                        ..
                    } = notification
                    {
                        if sub_id == subscription_id {
                            if let Ok(result) =
                                serde_json::from_str::<GenerateZKPJobResult>(&event.content)
                            {
                                if result.job_id == job_id {
                                    info!("Job result found for job_id: {}", job_id);
                                    return Ok(true);
                                }
                            }
                        }
                    }
                    Ok(false)
                }
            })
            .await
            .map_err(CustomerError::NostrClientError)?;

        // Fetch recent events to find the job result
        let events = self
            .nostr_client
            .get_events_of(
                vec![Filter::new()
                    .kind(Kind::TextNote)
                    .author(PublicKey::from_bech32(&self.settings.prover_agent_pk).unwrap())
                    .since(Timestamp::now() - Duration::from_secs(60))],
                None,
            )
            .await
            .map_err(CustomerError::NostrClientError)?;

        // Find and return the matching job result
        for event in events {
            if let Ok(job_result) = serde_json::from_str::<GenerateZKPJobResult>(&event.content) {
                if job_result.job_id == job_id {
                    return Ok(job_result);
                }
            }
        }

        Err(CustomerError::Unknown("Job result not found".to_string()))
    }

    /// Verifies the proof in a job result
    pub fn verify_proof(&self, job_result: &GenerateZKPJobResult) -> Result<bool, CustomerError> {
        info!("Verifying proof...");
        self.verifier_service
            .verify_proof(job_result.response.clone())
            .map(|_| true)
            .map_err(|e| CustomerError::VerificationError(e.to_string()))
    }
}
