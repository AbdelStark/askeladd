//! Helpers for reading NIP-90 tags and managing relay connections.

use std::collections::HashMap;
use std::time::Duration;

use nostr_sdk::prelude::*;

/// Extracts parameters and output format from a vector of Tags.
///
/// NIP-90 carries job inputs as `["param", "<key>", "<value>"]` tags and the
/// requested output MIME type as `["output", "<type>"]`.
///
/// # Arguments
///
/// * `tags` - A slice of Tag structs
///
/// # Returns
///
/// A HashMap where:
/// - Keys are parameter names or "output" for the output format
/// - Values are the corresponding parameter values or output format
pub fn extract_params_from_tags(tags: &[Tag]) -> HashMap<String, String> {
    let mut params = HashMap::new();

    for tag in tags {
        let tag_vec = tag.as_vec();
        if tag_vec.len() >= 3 && tag_vec[0] == "param" {
            params.insert(tag_vec[1].to_string(), tag_vec[2].to_string());
        } else if tag_vec.len() >= 2 && tag_vec[0] == "output" {
            params.insert("output".to_string(), tag_vec[1].to_string());
        }
    }

    params
}

/// Waits until at least one relay reports `Connected`, or `timeout` elapses.
///
/// `Client::connect` returns as soon as connection attempts are *initiated*;
/// publishing immediately afterwards races the handshake and intermittently
/// fails with "relay not connected". Call this first.
pub async fn wait_for_relay_connection(client: &Client, timeout: Duration) -> Result<(), String> {
    let start = std::time::Instant::now();
    loop {
        for relay in client.relays().await.values() {
            if relay.status().await == RelayStatus::Connected {
                return Ok(());
            }
        }
        if start.elapsed() > timeout {
            return Err(format!("no relay connected within {}s", timeout.as_secs()));
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_params_and_output() {
        let tags = vec![
            Tag::parse(&["param", "log_size", "5"]).unwrap(),
            Tag::parse(&["param", "claim", "443693538"]).unwrap(),
            Tag::parse(&["output", "text/json"]).unwrap(),
        ];
        let params = extract_params_from_tags(&tags);

        assert_eq!(params.get("log_size"), Some(&"5".to_string()));
        assert_eq!(params.get("claim"), Some(&"443693538".to_string()));
        assert_eq!(params.get("output"), Some(&"text/json".to_string()));
    }

    #[test]
    fn ignores_unrelated_tags() {
        let tags = vec![
            Tag::parse(&["e", "some-event-id", "", "reply"]).unwrap(),
            Tag::parse(&["param"]).unwrap(),
        ];
        assert!(extract_params_from_tags(&tags).is_empty());
    }
}
