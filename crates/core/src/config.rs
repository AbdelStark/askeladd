//! Layered application settings.
//!
//! Configuration is resolved from, in increasing order of precedence:
//!
//! 1. `config/default.toml` (checked in, safe defaults)
//! 2. `config/{RUN_MODE}.toml` (e.g. `RUN_MODE=production`)
//! 3. `config/local.toml` (gitignored, machine-local)
//! 4. Environment variables prefixed with `APP_` (e.g. `APP_SUBSCRIBED_RELAYS`)

use std::env;
use std::path::PathBuf;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

/// A single relay URL, or a list of them (TOML array vs. `APP_` env string).
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StringOrVec {
    String(String),
    Vec(Vec<String>),
}

/// Runtime settings shared by the customer and the service provider.
#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Nostr relays to connect to (websocket URLs).
    #[serde(deserialize_with = "deserialize_subscribed_relays")]
    pub subscribed_relays: Vec<String>,
    /// Subscription ID for proving job requests.
    pub proving_req_sub_id: String,
    /// Subscription ID for proving job results.
    pub proving_resp_sub_id: String,
    /// Customer secret key (`nsec…`, bech32). Use a throwaway key for demos.
    pub user_bech32_sk: String,
    /// Prover agent secret key (`nsec…`, bech32).
    pub prover_agent_sk: String,
    /// Prover agent public key (`npub…`, bech32); customers filter results by it.
    pub prover_agent_pk: String,
    /// Path of the SQLite job ledger used by the prover agent.
    #[serde(default = "default_db_path")]
    pub db_path: PathBuf,
    /// Subscription ID for program-launch requests (experimental).
    pub launch_program_req_id: String,
}

fn deserialize_subscribed_relays<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = StringOrVec::deserialize(deserializer)?;
    Ok(match value {
        StringOrVec::String(s) => vec![s],
        StringOrVec::Vec(v) => v,
    })
}

fn default_db_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_owned());
    PathBuf::from(home)
        .join(".askeladd")
        .join("prover_agent.db")
}

impl Settings {
    /// Loads settings from the layered sources described in the module docs,
    /// and ensures the ledger's parent directory exists.
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        let settings: Settings = s.try_deserialize()?;

        // Ensure the directory for the database exists.
        if let Some(parent) = settings.db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                ConfigError::Message(format!("Failed to create directory for database: {}", e))
            })?;
        }

        Ok(settings)
    }
}
