use askeladd::config::Settings;
use askeladd::types::{FibonnacciProvingRequest, FibonnacciProvingResponse};
use nostr_sdk::prelude::*;
use std::time::Duration;

#[tokio::test]
async fn test_e2e_flow() {
    let settings = Settings::new().expect("Failed to load settings");

    let secret_key = SecretKey::from_bech32(&settings.user_bech32_sk).unwrap();
    let keys = Keys::new(secret_key);

    let client = Client::new(&keys);
    client.add_relay("ws://nostr-relay:8080").await.unwrap();
    client.connect().await;
    // Create and publish a proving request
    let request = FibonnacciProvingRequest {
        request_id: "test-request-id".to_string(),
        log_size: 5,
        claim: 443693538,
    };
    let request_json = serde_json::to_string(&request).unwrap();
    let event_id = client.publish_text_note(request_json, &[]).await.unwrap();

    // Wait for the response
    let filter = Filter::new()
        .kind(Kind::TextNote)
        .custom_tag("e", vec![event_id.to_string()]);
    let mut notifications = client.notifications();

    let mut response_received = false;
    for _ in 0..30 {  // Wait up to 30 seconds
        if let Ok(notification) = tokio::time::timeout(Duration::from_secs(1), notifications.next()).await {
            if let Some(RelayPoolNotification::Event { event, .. }) = notification {
                if let Ok(response) = serde_json::from_str::<FibonnacciProvingResponse>(&event.content) {
                    assert_eq!(response.request_id, request.request_id);
                    assert!(response.proof.is_some());
                    response_received = true;
                    break;
                }
            }
        }
    }

    assert!(response_received, "Did not receive a response within the timeout period");
}