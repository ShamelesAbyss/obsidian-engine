use anyhow::{Context, Result};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct DfHackBridge {
    command: String,
    dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct DfHackResult {
    pub command: String,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl DfHackResult {
    pub fn summary(&self) -> String {
        if self.stderr.is_empty() {
            format!(
                "command='{}' success={} stdout='{}'",
                self.command, self.success, self.stdout
            )
        } else {
            format!(
                "command='{}' success={} stdout='{}' stderr='{}'",
                self.command, self.success, self.stdout, self.stderr
            )
        }
    }
}

impl DfHackBridge {
    pub fn new(command: impl Into<String>, dry_run: bool) -> Self {
        Self {
            command: command.into(),
            dry_run,
        }
    }

    pub fn is_available(&self) -> bool {
        Command::new(&self.command)
            .arg("--version")
            .output()
            .is_ok()
    }

    pub fn run(&self, dfhack_command: &str) -> Result<DfHackResult> {
        if self.dry_run {
            return Ok(DfHackResult {
                command: dfhack_command.to_string(),
                success: true,
                stdout: format!("dry-run: would invoke {} {}", self.command, dfhack_command),
                stderr: String::new(),
            });
        }

        let output = Command::new(&self.command)
            .arg(dfhack_command)
            .output()
            .with_context(|| format!("failed to invoke {}", self.command))?;

        Ok(DfHackResult {
            command: dfhack_command.to_string(),
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}
