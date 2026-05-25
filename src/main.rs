mod actions;
mod chronicle;
mod config;
mod core;
mod df;
mod dfhack;
mod executor;
mod narrator;
mod planner;

use anyhow::Result;
use crate::core::engine::ObsidianEngine;

fn main() -> Result<()> {
    let mut engine = ObsidianEngine::new("obsidian.json")?;
    engine.boot()?;
    engine.run()?;
    Ok(())
}
