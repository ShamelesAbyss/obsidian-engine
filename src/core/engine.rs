use anyhow::Result;

use crate::chronicle::Chronicle;
use crate::df::state::FortressState;
use crate::executor::Executor;
use crate::narrator::Narrator;
use crate::planner::Planner;

pub struct ObsidianEngine {
    chronicle: Chronicle,
    planner: Planner,
    executor: Executor,
    narrator: Narrator,
}

impl ObsidianEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            chronicle: Chronicle::new("chronicle.jsonl"),
            planner: Planner::new(),
            executor: Executor::new(),
            narrator: Narrator::new(),
        })
    }

    pub fn boot(&mut self) -> Result<()> {
        println!("Obsidian Engine — Autonomous Fortress Intelligence");
        println!("Let the mountain think. Watch the fortress fall.");
        self.chronicle.record("engine_boot", "Obsidian Engine awakened.")?;
        Ok(())
    }

    pub fn run_once(&mut self) -> Result<()> {
        let state = FortressState::mock();
        let plan = self.planner.plan(&state);
        let narration = self.narrator.describe(&state, &plan);

        println!();
        println!("FORTRESS STATE:");
        println!("{state:#?}");

        println!();
        println!("OBSIDIAN DIRECTIVE:");
        println!("{plan:#?}");

        println!();
        println!("CHRONICLE:");
        println!("{narration}");

        self.executor.execute(&plan)?;
        self.chronicle.record("directive", &format!("{plan:?}"))?;
        self.chronicle.record("narration", &narration)?;

        Ok(())
    }
}
