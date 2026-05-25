use anyhow::Result;

use crate::config::EngineConfig;
use crate::df::state::FortressState;
use crate::dfhack::DfHackBridge;

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

pub struct DfHackObserver {
    fortress_name: String,
    bridge: DfHackBridge,
}

impl DfHackObserver {
    pub fn from_config(config: &EngineConfig) -> Self {
        Self {
            fortress_name: config.fortress_name.clone(),
            bridge: DfHackBridge::new(&config.dfhack_command, true),
        }
    }

    pub fn source() -> ObservationSource {
        ObservationSource::DfHack
    }
}

impl Observer for DfHackObserver {
    fn observe(&mut self, cycle: u64) -> Result<ObservationSnapshot> {
        let units = self.bridge.run("units")?;
        let stocks = self.bridge.run("stocks show")?;
        let jobs = self.bridge.run("job list")?;

        let raw_events = vec![
            format!("dfhack units probe: {}", units.summary()),
            format!("dfhack stocks probe: {}", stocks.summary()),
            format!("dfhack jobs probe: {}", jobs.summary()),
        ];

        let state = FortressState {
            fortress_name: self.fortress_name.clone(),
            cycle,
            population: 0,
            food: 0,
            booze: 0,
            threats: vec![],
            recent_events: raw_events.clone(),
        };

        Ok(ObservationSnapshot {
            cycle,
            source: ObservationSource::DfHack,
            raw_events,
            state,
        })
    }
}
