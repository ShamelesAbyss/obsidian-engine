use anyhow::Result;

use crate::config::EngineConfig;
use crate::df::state::FortressState;

#[derive(Debug, Clone)]
pub struct ObservationSnapshot {
    pub cycle: u64,
    pub source: ObservationSource,
    pub raw_events: Vec<String>,
    pub state: FortressState,
}

#[derive(Debug, Clone)]
pub enum ObservationSource {
    Mock,
    DfHack,
}

impl ObservationSource {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Mock => "mock",
            Self::DfHack => "dfhack",
        }
    }
}

pub trait Observer {
    fn observe(&mut self, cycle: u64) -> Result<ObservationSnapshot>;
}

pub struct MockObserver {
    fortress_name: String,
    source: ObservationSource,
}

impl MockObserver {
    pub fn from_config(config: &EngineConfig) -> Self {
        Self {
            fortress_name: config.fortress_name.clone(),
            source: ObservationSource::Mock,
        }
    }
}

impl Observer for MockObserver {
    fn observe(&mut self, cycle: u64) -> Result<ObservationSnapshot> {
        let state = FortressState::mock(&self.fortress_name, cycle);

        Ok(ObservationSnapshot {
            cycle,
            source: self.source.clone(),
            raw_events: state.recent_events.clone(),
            state,
        })
    }
}

pub struct DfHackObserver;

impl DfHackObserver {
    pub fn source() -> ObservationSource {
        ObservationSource::DfHack
    }
}
