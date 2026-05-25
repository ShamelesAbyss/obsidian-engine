use anyhow::Result;

use crate::actions::ActionIntent;
use crate::dfhack::DfHackBridge;

pub struct Executor {
    dry_run: bool,
}

impl Executor {
    pub fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }

    pub fn execute(&self, intent: &ActionIntent, dfhack: &DfHackBridge) -> Result<()> {
        println!();
        println!("EXECUTOR:");
        println!("Intent: {}", intent.label);
        println!("Safety: {}", intent.safety_note);

        if self.dry_run {
            let result = dfhack.run(&intent.dfhack_command)?;
            println!("Dry-run command: {}", result.summary());
        } else {
            println!("Live execution is not implemented yet. Refusing unsafe action.");
        }

        Ok(())
    }
}
