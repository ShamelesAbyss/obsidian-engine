#[derive(Debug, Clone)]
pub struct FortressState {
    pub fortress_name: String,
    pub cycle: u64,
    pub population: u32,
    pub food: u32,
    pub booze: u32,
    pub threats: Vec<String>,
    pub recent_events: Vec<String>,
}

impl FortressState {
    pub fn mock(fortress_name: &str, cycle: u64) -> Self {
        let booze = match cycle {
            1 => 35,
            2 => 75,
            _ => 120,
        };

        let threats = if cycle >= 3 {
            vec!["A possible hostile presence has been reported beyond the map edge.".to_string()]
        } else {
            vec![]
        };

        Self {
            fortress_name: fortress_name.to_string(),
            cycle,
            population: 7,
            food: 80,
            booze,
            threats,
            recent_events: vec![
                "The expedition has struck the earth.".to_string(),
                "A temporary shelter is required before nightfall.".to_string(),
                format!("The Engine completed observation cycle {cycle}."),
            ],
        }
    }
}
