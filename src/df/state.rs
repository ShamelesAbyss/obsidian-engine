#[derive(Debug, Clone)]
pub struct FortressState {
    pub fortress_name: String,
    pub population: u32,
    pub food: u32,
    pub booze: u32,
    pub threats: Vec<String>,
    pub recent_events: Vec<String>,
}

impl FortressState {
    pub fn mock() -> Self {
        Self {
            fortress_name: "The First Black Hall".to_string(),
            population: 7,
            food: 80,
            booze: 35,
            threats: vec![],
            recent_events: vec![
                "The expedition has struck the earth.".to_string(),
                "A temporary shelter is required before nightfall.".to_string(),
            ],
        }
    }
}
