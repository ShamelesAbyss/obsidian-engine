use anyhow::Result;
use crate::planner::Plan;

pub struct Executor {
    dry_run: bool,
}

impl Executor {
    pub fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }

    pub fn execute(&self, plan: &Plan) -> Result<()> {
        println!();
        println!("EXECUTOR:");

        if self.dry_run {
            println!("Dry-run only. Would execute directive: {:?}", plan.directive);
        } else {
            println!("Live execution is not implemented yet. Refusing unsafe action.");
        }

        Ok(())
    }
}
