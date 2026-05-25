use crate::df::state::FortressState;
use crate::planner::Plan;

pub struct Narrator;

impl Narrator {
    pub fn new() -> Self {
        Self
    }

    pub fn describe(&self, state: &FortressState, plan: &Plan) -> String {
        let events = if state.recent_events.is_empty() {
            "No recent events were recorded.".to_string()
        } else {
            state.recent_events.join(" ")
        };

        format!(
            "Within {}, {} souls gather beneath uncertain stone. Recent omens: {} The Engine has chosen {:?}: {}",
            state.fortress_name,
            state.population,
            events,
            plan.directive,
            plan.reason
        )
    }
}
