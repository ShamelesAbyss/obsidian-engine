use crate::actions::ActionIntent;
use crate::memory::StrategicContext;
use crate::planner::{Directive, Plan};

#[derive(Debug, Clone)]
pub enum PolicyDecision {
    Approved,
    Blocked(String),
    Downgraded {
        reason: String,
        replacement: ActionIntent,
    },
}

pub struct PolicyCore {
    dry_run: bool,
}

impl PolicyCore {
    pub fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }

    pub fn evaluate(
        &self,
        plan: &Plan,
        intent: &ActionIntent,
        context: &StrategicContext,
    ) -> PolicyDecision {
        if intent.dfhack_command.trim().is_empty() {
            return PolicyDecision::Blocked(format!(
                "Directive {:?} produced an empty DFHack command.",
                plan.directive
            ));
        }

        if self.dry_run {
            return PolicyDecision::Approved;
        }

        match plan.directive {
            Directive::PrepareDefense if context.repeated_threats >= 2 => PolicyDecision::Approved,
            Directive::Observe => PolicyDecision::Approved,
            _ => PolicyDecision::Downgraded {
                reason: format!(
                    "Live mode refuses non-critical directive {:?} until execution policies mature.",
                    plan.directive
                ),
                replacement: ActionIntent {
                    label: "observe".to_string(),
                    dfhack_command: "ls".to_string(),
                    safety_note: format!(
                        "Downgraded from {}. Observation-only fallback.",
                        intent.label
                    ),
                },
            },
        }
    }
}
