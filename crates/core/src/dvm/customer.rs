use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use log::{debug, error, info};
use nostr_sdk::prelude::*;
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::time::timeout;

use crate::config::Settings;
use crate::dvm::constants::*;
use crate::dvm::types::{GenerateZKPJobRequest, GenerateZKPJobResult};
use crate::nostr_utils::extract_params_from_tags;
use crate::verifier_service::VerifierService;

/// Represents a customer in the Askeladd system.
///
/// The `Customer` struct is responsible for interacting with the Nostr network,
/// submitting job requests, waiting for job results, and verifying proofs.
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
        debug!("Nostr client connected to relays.");
        Ok(())
    }

    /// Submits a job request to the Nostr network
    // pub async fn submit_job<T>(
    pub async fn submit_job(&self, job: GenerateZKPJobRequest) -> Result<String, CustomerError> {
        debug!("Publishing proving request...");

        let program = job.clone().program;
        let mut params_inputs: HashMap<String, String> = HashMap::new();
        let mut tags = vec![];
        if let Some(p) = program {
            if let Some(inputs) = p.inputs {
                params_inputs = inputs;
            }
            else {
                let successful_parses = extract_params_from_tags(&tags);
                // let inputs_values:HashMap<String,Value>= successful_parses
                //     .into_iter()
                //     .map(|(k, v)| {
                //         let val:Value= serde_json::to_value(&v).unwrap();
                //         // params_inputs.insert(k.clone(), val.clone());
                //         return (k, val)
                //     })
                //     .collect();
                params_inputs = successful_parses;
            }
        }
        // OLD TAGS creation
        // let tags = vec![
        //     Tag::parse(&[
        //         "param",
        //         "log_size",
        //         job.request.log_size.to_string().as_str(),
        //     ])
        //     .unwrap(),
        //     Tag::parse(&["param", "claim", job.request.claim.to_string().as_str()]).unwrap(),
        //     Tag::parse(&["output", "text/json"]).unwrap(),
        // ];

        for (key, value) in params_inputs.into_iter() {
            println!("{} / {}", key, value);
            let tag = Tag::parse(&["param", &key.to_owned(), &value.to_owned()]);
            tags.push(tag.unwrap())
        }

        // Send JSON into the content of the JOB_REQUEST:
        // Request: Params of the program
        // Program: Pamaters to select a specific program
        let content = serde_json::to_string(&job).unwrap();
        let event_builder = EventBuilder::new(Kind::Custom(JOB_REQUEST_KIND), content, tags);
        let event: Event = event_builder.to_event(&self.user_keys).unwrap();

        // let event: Event = EventBuilder::job_request(Kind::Custom(JOB_REQUEST_KIND), tags)
        //     .unwrap()
        //     .to_event(&self.user_keys)
        //     .unwrap();

        let event_id = self.nostr_client.send_event(event).await?;

        info!("Proving request published [{}]", event_id.to_string());
        Ok(event_id.to_string())
    }

    /// Waits for a job result from the Nostr network
    // pub async fn wait_for_job_result<T: Clone + serde::Deserialize<'static>>(
    pub async fn wait_for_job_result(
        &self,
        job_id: &str,
        timeout_secs: u64,
    ) -> Result<GenerateZKPJobResult, CustomerError> {
        // )  -> Result<GenerateZKPJobResult<T>, CustomerError> {
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
    // async fn listen_for_job_result<T:Clone + serde::Deserialize<'static>>(
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
        if let Some(job_result) = result_guard.clone() {
            // Convert the string to a GenerateZKPJobResult
            Ok(serde_json::from_str(&job_result).unwrap())
        } else {
            Err(CustomerError::Unknown("Job result not found".to_string()))
        }
    }

    /// Verifies the proof in a job result
    // pub fn verify_proof<T: Clone + serde::Deserialize<'static>>(
    pub fn verify_proof(
        &self,
        job_result: &GenerateZKPJobResult,
        // job_result: &GenerateZKPJobResult<T>,
    ) -> Result<bool, CustomerError> {
        info!("Verifying proof...");
        info!(
            "Proof: {}",
            serde_json::to_string(&job_result.proof).unwrap()
        );
        self.verifier_service
            .verify_proof_generic(job_result.response.clone())
            .map(|_| true)
            .map_err(|e| CustomerError::VerificationError(e.to_string()))
    }
}
#[cfg(test)]
mod tests {

    use nostr_sdk::prelude::*;

    use crate::nostr_utils::extract_params_from_tags;

    #[test]
    fn test_submit_job() {
        let tags = vec![
            Tag::parse(&["param", "log_size", "5"]).unwrap(),
            Tag::parse(&["param", "claim", "443693538"]).unwrap(),
            Tag::parse(&["output", "text/json"]).unwrap(),
        ];
        let params = extract_params_from_tags(&tags);

        assert_eq!(params.get("log_size"), Some(&"5".to_string()));
        assert_eq!(params.get("claim"), Some(&"443693538".to_string()));
        assert_eq!(params.get("output"), Some(&"text/json".to_string()));

        // Convert and check numeric parameters
        let log_size = params
            .get("log_size")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap();
        let claim = params
            .get("claim")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap();

        assert_eq!(log_size, 5);
        assert_eq!(claim, 443693538);

        // Print extracted parameters for debugging
        println!("Extracted parameters: {:?}", params);
    }
}
