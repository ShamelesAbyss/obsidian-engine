use crate::planner::Directive;

#[derive(Debug, Clone)]
pub struct ActionIntent {
    pub label: String,
    pub dfhack_command: String,
    pub safety_note: String,
}

impl ActionIntent {
    pub fn from_directive(directive: &Directive) -> Self {
        match directive {
            Directive::BrewAlcohol => Self {
                label: "brew_alcohol".to_string(),
                dfhack_command: "job list".to_string(),
                safety_note: "Observation only. Later this will inspect still jobs and drink stock.".to_string(),
            },
            Directive::SecureFood => Self {
                label: "secure_food".to_string(),
                dfhack_command: "stocks show".to_string(),
                safety_note: "Observation only. Later this will inspect food stocks and farming jobs.".to_string(),
            },
            Directive::BuildShelter => Self {
                label: "build_shelter".to_string(),
                dfhack_command: "buildingplan list".to_string(),
                safety_note: "Observation only. Later this will inspect planned shelter construction.".to_string(),
            },
            Directive::PrepareDefense => Self {
                label: "prepare_defense".to_string(),
                dfhack_command: "squad list".to_string(),
                safety_note: "Observation only. Later this will inspect squads, burrows, and threats.".to_string(),
            },
            Directive::ExpandIndustry => Self {
                label: "expand_industry".to_string(),
                dfhack_command: "workorder list".to_string(),
                safety_note: "Observation only. Later this will inspect workshops and work orders.".to_string(),
            },
            Directive::Observe => Self {
                label: "observe".to_string(),
                dfhack_command: "ls".to_string(),
                safety_note: "Observation only. Passive bridge probe.".to_string(),
            },
        }
    }
}
