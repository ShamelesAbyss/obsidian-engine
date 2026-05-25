use anyhow::Result;

use crate::config::EngineConfig;
use crate::df::state::FortressState;

pub trait Observer {
    fn observe(&mut self, cycle: u64) -> Result<FortressState>;
}

pub struct MockObserver {
    fortress_name: String,
}

impl MockObserver {
    pub fn from_config(config: &EngineConfig) -> Self {
        Self {
            fortress_name: config.fortress_name.clone(),
        }
    }
}

impl Observer for MockObserver {
    fn observe(&mut self, cycle: u64) -> Result<FortressState> {
        Ok(FortressState::mock(&self.fortress_name, cycle))
    }
}
