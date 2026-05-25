mod chronicle;
mod core;
mod df;
mod executor;
mod narrator;
mod planner;

use anyhow::Result;
use crate::core::engine::ObsidianEngine;

fn main() -> Result<()> {
    let mut engine = ObsidianEngine::new()?;
    engine.boot()?;
    engine.run_once()?;
    Ok(())
}
