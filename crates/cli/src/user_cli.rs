use askeladd_core::config::Settings;
use askeladd_core::types::{FibonnacciProvingRequest, FibonnacciProvingResponse};
use askeladd_core::verifier_service::VerifierService;
use dotenv::dotenv;
use nostr_sdk::prelude::*;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    println!("User agent starting...");
    println!("Waiting 5 seconds before submitting proving request...");
    // Add a delay before connecting
    sleep(Duration::from_secs(5)).await;

    // Load configuration from .env file
    dotenv().ok();
    let settings = Settings::new().expect("Failed to load settings");

    let prover_agent_secret_key = SecretKey::from_bech32(&settings.prover_agent_sk)?;
    let prover_agent_keys = Keys::new(prover_agent_secret_key);
    let prover_agent_public_key = prover_agent_keys.public_key();
    let user_keys = Keys::new(SecretKey::from_bech32(&settings.user_bech32_sk).unwrap());

    let opts = Options::new().wait_for_send(false);
    let client = Client::with_opts(&user_keys, opts);

    for relay in settings.subscribed_relays {
        client.add_relay(&relay).await?;
    }

    client.connect().await;
    println!("Client connected to relays.");

    // Generate a unique request ID
    let request_id = Uuid::new_v4().to_string();

    // Create a proving request
    let proving_request = FibonnacciProvingRequest {
        request_id: request_id.clone(),
        log_size: 5,
        claim: 443693538,
    };

    // Serialize the request to JSON
    let request_json = serde_json::to_string(&proving_request)?;

    // Publish the proving request
    println!("Publishing proving request...");
    let event_id = client.publish_text_note(request_json, []).await?;

    println!("Proving request published with event ID: {}", event_id);

    // Subscribe to proving responses
    let proving_resp_sub_id = SubscriptionId::new(settings.proving_resp_sub_id);
    let filter = Filter::new()
        .kind(Kind::TextNote)
        .author(prover_agent_public_key)
        .since(Timestamp::now());

    client
        .subscribe_with_id(proving_resp_sub_id.clone(), vec![filter], None)
        .await;

    // Handle subscription notifications
    client
        .handle_notifications(|notification| async {
            if let RelayPoolNotification::Event {
                subscription_id,
                event,
                ..
            } = notification
            {
                if subscription_id == proving_resp_sub_id {
                    println!("Proving response received: {:?}", event);

                    // Deserialize the response
                    if let Ok(response) =
                        serde_json::from_str::<FibonnacciProvingResponse>(&event.content)
                    {
                        // Verify the proof
                        let verifier_service: VerifierService = Default::default();
                        println!("Verifying proof...");
                        match verifier_service.verify_proof(response) {
                            Ok(_) => println!("Proof successfully verified"),
                            Err(e) => println!("Proof verification failed: {}", e),
                        }
                        // Stop listening after receiving and verifying the response
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        })
        .await?;

    Ok(())
}
