use std::collections::HashMap;

use nostr_sdk::prelude::*;

/// Extracts parameters and output format from a vector of Tags.
///
/// # Arguments
///
/// * `tags` - A vector of Tag structs
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
