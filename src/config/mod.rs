use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct EngineConfig {
    pub fortress_name: String,
    pub mode: EngineMode,
    pub observer: ObserverMode,
    pub dry_run: bool,
    pub loop_delay_ms: u64,
    pub max_cycles: u64,
    pub chronicle_path: String,
    pub dfhack_command: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EngineMode {
    Fortress,
    Adventure,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObserverMode {
    Mock,
    Dfhack,
}

impl EngineConfig {
    pub fn load(path: &str) -> Result<Self> {
        let raw = fs::read_to_string(path)?;
        let config = serde_json::from_str(&raw)?;
        Ok(config)
    }
}
