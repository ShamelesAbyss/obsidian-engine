use crate::observe::ObservationSnapshot;

#[derive(Debug, Clone)]
pub struct StrategicContext {
    pub cycles_seen: u64,
    pub last_population: u32,
    pub last_food: u32,
    pub last_booze: u32,
    pub repeated_threats: u32,
    pub recent_summary: String,
}

impl StrategicContext {
    pub fn resource_line(&self) -> String {
        format!(
            "population={} food={} booze={} threats_seen={}",
            self.last_population, self.last_food, self.last_booze, self.repeated_threats
        )
    }
}

pub struct MemoryCore {
    snapshots_seen: u64,
    repeated_threats: u32,
    last_population: u32,
    last_food: u32,
    last_booze: u32,
}

impl MemoryCore {
    pub fn new() -> Self {
        Self {
            snapshots_seen: 0,
            repeated_threats: 0,
            last_population: 0,
            last_food: 0,
            last_booze: 0,
        }
    }

    pub fn absorb(&mut self, snapshot: &ObservationSnapshot) -> StrategicContext {
        self.snapshots_seen += 1;

        let state = &snapshot.state;

        if !state.threats.is_empty() {
            self.repeated_threats += 1;
        }

        self.last_population = state.population;
        self.last_food = state.food;
        self.last_booze = state.booze;

        let recent_summary = if self.repeated_threats >= 2 {
            "Repeated hostile pressure has been observed. The fortress should become more defensive."
                .to_string()
        } else if state.booze < state.population * 10 {
            "Alcohol pressure is active. Dwarven morale may degrade if brewing does not improve."
                .to_string()
        } else if state.food < state.population * 8 {
            "Food pressure is active. The fortress economy may become unstable.".to_string()
        } else {
            "No persistent crisis has dominated the recent memory window.".to_string()
        };

        StrategicContext {
            cycles_seen: self.snapshots_seen,
            last_population: self.last_population,
            last_food: self.last_food,
            last_booze: self.last_booze,
            repeated_threats: self.repeated_threats,
            recent_summary,
        }
    }
}
