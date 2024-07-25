use std::env;
use std::path::PathBuf;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StringOrVec {
    String(String),
    Vec(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(deserialize_with = "deserialize_subscribed_relays")]
    pub subscribed_relays: Vec<String>,
    pub proving_req_sub_id: String,
    pub proving_resp_sub_id: String,
    pub user_bech32_sk: String,
    pub prover_agent_sk: String,
    #[serde(default = "default_db_path")]
    pub db_path: PathBuf,
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
    let home = env::var("HOME").expect("HOME environment variable not set");
    PathBuf::from(home)
        .join(".askeladd")
        .join("prover_agent.db")
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        let settings: Settings = s.try_deserialize()?;

        // Ensure the directory for the database exists
        if let Some(parent) = settings.db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                ConfigError::Message(format!("Failed to create directory for database: {}", e))
            })?;
        }

        Ok(settings)
    }
}
