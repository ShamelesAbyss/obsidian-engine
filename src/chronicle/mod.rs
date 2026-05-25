use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
struct ChronicleEntry<'a> {
    timestamp: String,
    kind: &'a str,
    text: &'a str,
}

pub struct Chronicle {
    path: PathBuf,
}

impl Chronicle {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn record(&self, kind: &str, text: &str) -> Result<()> {
        let entry = ChronicleEntry {
            timestamp: Utc::now().to_rfc3339(),
            kind,
            text,
        };

        let encoded = serde_json::to_string(&entry)?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        writeln!(file, "{encoded}")?;
        Ok(())
    }
}
