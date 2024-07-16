use askeladd_core::data_fixture::{self, PROVING_RESP_SUB_ID, SUBSCRIBED_RELAYS};
use askeladd_core::types::{FibonnacciProvingRequest, FibonnacciProvingResponse};
use askeladd_core::verifier_service::VerifierService;
use nostr_sdk::prelude::*;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let prover_agent_secret_key = SecretKey::from_bech32(data_fixture::PROVER_AGENT_SK)?;
    let prover_agent_keys = Keys::new(prover_agent_secret_key);
    let prover_agent_public_key = prover_agent_keys.public_key();
    let user_keys = Keys::new(SecretKey::from_bech32(data_fixture::USER_BECH32_SK).unwrap());

    let opts = Options::new().wait_for_send(false);
    let client = Client::with_opts(&user_keys, opts);

    for relay in SUBSCRIBED_RELAYS {
        client.add_relay(Url::parse(relay).unwrap()).await?;
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
    let proving_resp_sub_id = SubscriptionId::new(PROVING_RESP_SUB_ID);
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
                        return Ok(true); // Stop listening after receiving and verifying the
                                         // response
                    }
                }
            }
            Ok(false)
        })
        .await?;

    Ok(())
}
