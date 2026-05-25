use crate::df::state::FortressState;
use crate::planner::Plan;

pub struct Narrator;

impl Narrator {
    pub fn new() -> Self {
        Self
    }

    pub fn describe(&self, state: &FortressState, plan: &Plan) -> String {
        format!(
            "Within {}, {} souls gather beneath uncertain stone. The Engine has chosen {:?}: {}",
            state.fortress_name,
            state.population,
            plan.directive,
            plan.reason
        )
    }
}
