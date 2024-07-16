use askeladd_core::prover_service::ProverService;
use askeladd_core::types::FibonnacciProvingRequest;
use nostr_sdk::prelude::*;

const SUBSCRIBED_RELAYS: &[&str] = &[
    "wss://nostr.oxtr.dev",
    "wss://relay.damus.io",
    "wss://nostr.openchain.fr",
];
const PROVING_REQ_SUB_ID: &str = "askeladd_proving_request";

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Options::new().wait_for_send(false);
    let client = Client::builder().opts(opts).build();

    for relay in SUBSCRIBED_RELAYS {
        client.add_relay(Url::parse(relay).unwrap()).await?;
    }

    client.connect().await;

    let proving_req_sub_id = SubscriptionId::new(PROVING_REQ_SUB_ID);
    let filter = Filter::new().kind(Kind::TextNote);

    client
        .subscribe_with_id(proving_req_sub_id.clone(), vec![filter], None)
        .await;

    let proving_service: ProverService = Default::default();

    client
        .handle_notifications(|notification| async {
            if let RelayPoolNotification::Event {
                subscription_id,
                event,
                ..
            } = notification
            {
                if subscription_id == proving_req_sub_id {
                    println!("Proving request received: {:?}", event);

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
                                println!("Proving response published with event ID: {}", event_id);
                            }
                            Err(e) => println!("Proof generation failed: {}", e),
                        }
                    }
                }
            }
            Ok(false)
        })
        .await?;

    Ok(())
}
