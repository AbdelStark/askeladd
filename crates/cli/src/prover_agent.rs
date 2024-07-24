use askeladd::config::Settings;
use askeladd::prover_service::ProverService;
use askeladd::types::FibonnacciProvingRequest;
use dotenv::dotenv;
use nostr_sdk::prelude::*;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Load configuration from .env file
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
                        // Generate the proof
                        match proving_service.generate_proof(request) {
                            Ok(response) => {
                                // Serialize the response to JSON
                                let response_json = serde_json::to_string(&response)?;

                                // Publish the proving response
                                let tags = vec![];
                                let event_id =
                                    client.publish_text_note(response_json, tags).await?;
                                info!("Proving response published [{}]", event_id.to_string());
                            }
                            Err(e) => error!("Proof generation failed: {}", e),
                        }
                    }
                }
            }
            Ok(false)
        })
        .await?;

    Ok(())
}
