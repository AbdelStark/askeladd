use askeladd::config::Settings;
use askeladd::db::{Database, RequestStatus};
use askeladd::prover_service::ProverService;
use askeladd::types::{FibonnacciProvingRequest, FibonnacciProvingResponse};
use dotenv::dotenv;
use nostr_sdk::prelude::*;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger and set default level to info
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Load configuration from .env filed
    dotenv().ok();
    let settings = Settings::new().expect("Failed to load settings");

    let user_secret_key = SecretKey::from_bech32(&settings.user_bech32_sk)?;
    let user_keys = Keys::new(user_secret_key);
    let user_public_key = user_keys.public_key();

    let prover_agent_keys = Keys::new(SecretKey::from_bech32(&settings.prover_agent_sk).unwrap());

    let opts = Options::new().wait_for_send(false);
    let client = Client::with_opts(&prover_agent_keys, opts);

    for relay in settings.subscribed_relays {
        client.add_relay(&relay).await?;
    }

    client.connect().await;
    debug!("Nostr client connected to relays.");

    let proving_req_sub_id = SubscriptionId::new(settings.proving_req_sub_id);
    let filter = Filter::new().kind(Kind::TextNote).author(user_public_key);

    client
        .subscribe_with_id(proving_req_sub_id.clone(), vec![filter], None)
        .await
        .expect("Failed to subscribe to proving requests");

    let db =
        Database::new(settings.db_path.to_str().unwrap()).expect("Failed to initialize database");

    let proving_service: ProverService = Default::default();

    info!("Subscribed to proving requests, waiting for requests...");
    client
        .handle_notifications(|notification| async {
            if let RelayPoolNotification::Event {
                subscription_id,
                event,
                ..
            } = notification
            {
                if subscription_id == proving_req_sub_id {
                    info!("Proving request received [{}]", event.id.to_string());

                    // Deserialize the request
                    if let Ok(request) =
                        serde_json::from_str::<FibonnacciProvingRequest>(&event.content)
                    {
                        // Check if request has already been processed
                        if let Ok(Some(status)) = db.get_request_status(&request.request_id) {
                            match status {
                                RequestStatus::Completed => {
                                    info!(
                                        "Request {} already processed, skipping",
                                        request.request_id
                                    );
                                    return Ok(false);
                                }
                                RequestStatus::Failed => {
                                    info!("Request {} failed before, retrying", request.request_id);
                                }
                                RequestStatus::Pending => {
                                    info!(
                                        "Request {} is already pending, skipping",
                                        request.request_id
                                    );
                                    return Ok(false);
                                }
                            }
                        } else {
                            // New request, insert into database
                            if let Err(e) = db.insert_request(&request) {
                                error!("Failed to insert request into database: {}", e);
                                return Ok(false);
                            }
                        }

                        // Generate the proof
                        match proving_service.generate_proof(request.clone()) {
                            Ok(response) => {
                                // Serialize the response to JSON
                                let response_json = serde_json::to_string(&response)?;

                                // Publish the proving response
                                let tags = vec![];
                                let event_id =
                                    client.publish_text_note(response_json, tags).await?;
                                info!("Proving response published [{}]", event_id.to_string());

                                // Update database
                                if let Err(e) = db.update_request(
                                    &response.request_id,
                                    &response,
                                    RequestStatus::Completed,
                                ) {
                                    error!("Failed to update request in database: {}", e);
                                }
                            }
                            Err(e) => {
                                error!("Proof generation failed: {}", e);
                                // Update database with failed status
                                if let Err(db_err) = db.update_request(
                                    &request.request_id,
                                    &FibonnacciProvingResponse::default(),
                                    RequestStatus::Failed,
                                ) {
                                    error!("Failed to update request in database: {}", db_err);
                                }
                            }
                        }
                    }
                }
            }
            Ok(false)
        })
        .await?;

    Ok(())
}
