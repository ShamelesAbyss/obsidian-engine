use anyhow::Result;
use crate::planner::Plan;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, plan: &Plan) -> Result<()> {
        println!();
        println!("EXECUTOR:");
        println!("Dry-run only. Would execute directive: {:?}", plan.directive);
        Ok(())
    }
}
