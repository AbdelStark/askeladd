//! The customer side of the DVM: submit jobs, collect results, verify proofs.

use std::sync::Arc;
use std::time::Duration;

use log::{debug, info};
use nostr_sdk::prelude::*;
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::time::timeout;

use crate::config::Settings;
use crate::dvm::constants::*;
use crate::dvm::types::{GenerateZKPJobRequest, GenerateZKPJobResult, ProgramInternalContractName};
use crate::verifier_service::VerifierService;

/// A customer of the Askeladd proving network.
///
/// Submits proving jobs as NIP-90 requests, waits for a prover agent to
/// answer, and verifies the STARK proof attached to the result.
pub struct Customer {
    /// Application settings
    settings: Settings,
    /// User keys
    user_keys: Keys,
    /// Nostr client for network communication
    nostr_client: Client,
    /// Service for verifying proofs
    verifier_service: VerifierService,
}

/// Errors that can occur during Customer operations
#[derive(Error, Debug)]
pub enum CustomerError {
    #[error("invalid user secret key: {0}")]
    InvalidSecretKey(String),
    #[error("failed to connect to Nostr relay: {0}")]
    NostrConnectionError(String),
    #[error("failed to subscribe to Nostr events: {0}")]
    NostrSubscriptionError(String),
    #[error("Nostr client error: {0}")]
    NostrClientError(#[from] nostr_sdk::client::Error),
    #[error("Nostr event builder error: {0}")]
    EventBuilderError(String),
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("proof verification error: {0}")]
    VerificationError(String),
    #[error("job timed out: {0}")]
    JobTimeout(String),
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl Customer {
    /// Creates a new Customer instance
    pub fn new(settings: Settings) -> Result<Self, CustomerError> {
        let secret_key = SecretKey::from_bech32(&settings.user_bech32_sk)
            .map_err(|e| CustomerError::InvalidSecretKey(e.to_string()))?;
        let user_keys = Keys::new(secret_key);
        // Wait for relay acknowledgment so rejected events (e.g. oversized
        // proofs refused by relay policy) surface as errors instead of
        // vanishing silently.
        let opts = Options::new()
            .wait_for_send(true)
            .send_timeout(Some(Duration::from_secs(15)));
        let client = Client::with_opts(&user_keys, opts);

        Ok(Self {
            settings,
            user_keys,
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
        crate::nostr_utils::wait_for_relay_connection(&self.nostr_client, Duration::from_secs(10))
            .await
            .map_err(CustomerError::NostrConnectionError)?;
        debug!("Nostr client connected to relays.");
        Ok(())
    }

    /// Submits a proving job request to the Nostr network.
    ///
    /// Program inputs are mirrored as NIP-90 `param` tags so that any
    /// NIP-90-compatible tooling can inspect the job; the JSON content
    /// carries the full, self-describing request.
    ///
    /// Returns the event ID of the published request — the job ID.
    pub async fn submit_job(&self, job: GenerateZKPJobRequest) -> Result<String, CustomerError> {
        debug!("Publishing proving request...");

        let mut tags: Vec<Tag> = Vec::new();
        if let Some(program) = &job.program {
            if let Some(inputs) = &program.inputs {
                for (key, value) in inputs {
                    let tag = Tag::parse(&["param", key.as_str(), value.as_str()])
                        .map_err(|e| CustomerError::EventBuilderError(e.to_string()))?;
                    tags.push(tag);
                }
            }
            if let Some(extra_tags) = &program.tags {
                tags.extend(extra_tags.iter().cloned());
            }
        }

        let content = serde_json::to_string(&job)?;
        let event = EventBuilder::new(Kind::Custom(JOB_REQUEST_KIND), content, tags)
            .to_event(&self.user_keys)
            .map_err(|e| CustomerError::EventBuilderError(e.to_string()))?;

        let job_id = event.id.to_string();
        self.nostr_client.send_event(event).await?;

        info!("Proving request published [{}]", job_id);
        Ok(job_id)
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
            .kind(Kind::Custom(JOB_RESULT_KIND))
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

        let result = Arc::new(Mutex::new(None));
        let result_clone = Arc::clone(&result);

        // Handle incoming Nostr notifications
        self.nostr_client
            .handle_notifications(move |notification| {
                let job_id = job_id.clone();
                let subscription_id = subscription_id.clone();
                let result = Arc::clone(&result_clone);
                async move {
                    if let RelayPoolNotification::Event {
                        subscription_id: sub_id,
                        event,
                        ..
                    } = notification
                    {
                        if sub_id == subscription_id {
                            if let Ok(job_result) =
                                serde_json::from_str::<GenerateZKPJobResult>(&event.content)
                            {
                                if job_result.job_id == job_id {
                                    let mut result_guard = result.lock().await;
                                    *result_guard = Some(event.content.clone());
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

        // Check if we found a result
        let result_guard = result.lock().await;
        match result_guard.clone() {
            Some(content) => Ok(serde_json::from_str(&content)?),
            None => Err(CustomerError::Unknown("Job result not found".to_string())),
        }
    }

    /// Verifies the proof in a job result.
    ///
    /// `program` must be the selector used when submitting the job; the
    /// echoed inputs inside the result provide the public statement.
    /// Verification is local and trustless — an invalid proof is an error,
    /// not a `false`.
    pub fn verify_proof(
        &self,
        job_result: &GenerateZKPJobResult,
        program: &ProgramInternalContractName,
    ) -> Result<(), CustomerError> {
        info!("Verifying proof...");
        self.verifier_service
            .verify_job_result(job_result, program)
            .map_err(|e| CustomerError::VerificationError(e.to_string()))
    }
}
