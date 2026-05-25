use crate::df::state::FortressState;
use crate::memory::StrategicContext;

#[derive(Debug, Clone)]
pub enum Directive {
    SecureFood,
    BrewAlcohol,
    BuildShelter,
    PrepareDefense,
    ExpandIndustry,
    Observe,
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub directive: Directive,
    pub reason: String,
}

pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self
    }

    pub fn plan(&self, state: &FortressState, context: &StrategicContext) -> Plan {
        if context.repeated_threats >= 2 {
            return Plan {
                directive: Directive::PrepareDefense,
                reason: format!(
                    "Memory reports repeated hostile pressure across {} observed cycles.",
                    context.cycles_seen
                ),
            };
        }

        if !state.threats.is_empty() {
            return Plan {
                directive: Directive::PrepareDefense,
                reason: "Hostile pressure detected near the fortress.".to_string(),
            };
        }

        if state.booze < state.population * 10 {
            return Plan {
                directive: Directive::BrewAlcohol,
                reason: "Booze reserves are below safe dwarven survival threshold.".to_string(),
            };
        }

        if state.food < state.population * 8 {
            return Plan {
                directive: Directive::SecureFood,
                reason: "Food stores are too low for sustained fortress growth.".to_string(),
            };
        }

        if state.cycle == 2 {
            return Plan {
                directive: Directive::ExpandIndustry,
                reason: "Basic survival looks stable enough to begin workshop expansion.".to_string(),
            };
        }

        if state.cycle > 3 {
            return Plan {
                directive: Directive::Observe,
                reason: context.recent_summary.clone(),
            };
        }

        Plan {
            directive: Directive::BuildShelter,
            reason: "The founding expedition needs protected internal space.".to_string(),
        }
    }
}
