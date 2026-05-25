use anyhow::Result;
use std::{thread, time::Duration};

use crate::chronicle::Chronicle;
use crate::config::EngineConfig;
use crate::df::state::FortressState;
use crate::executor::Executor;
use crate::narrator::Narrator;
use crate::planner::Planner;

pub struct ObsidianEngine {
    config: EngineConfig,
    chronicle: Chronicle,
    planner: Planner,
    executor: Executor,
    narrator: Narrator,
}

impl ObsidianEngine {
    pub fn new(config_path: &str) -> Result<Self> {
        let config = EngineConfig::load(config_path)?;

        Ok(Self {
            chronicle: Chronicle::new(&config.chronicle_path),
            planner: Planner::new(),
            executor: Executor::new(config.dry_run),
            narrator: Narrator::new(),
            config,
        })
    }

    pub fn boot(&mut self) -> Result<()> {
        println!("Obsidian Engine — Autonomous Fortress Intelligence");
        println!("Let the mountain think. Watch the fortress fall.");
        println!("Mode: {:?}", self.config.mode);
        println!("Dry run: {}", self.config.dry_run);

        self.chronicle
            .record("engine_boot", "Obsidian Engine awakened.")?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        for cycle in 1..=self.config.max_cycles {
            self.run_cycle(cycle)?;

            if cycle < self.config.max_cycles {
                thread::sleep(Duration::from_millis(self.config.loop_delay_ms));
            }
        }

        self.chronicle
            .record("engine_sleep", "Obsidian Engine completed configured cycles.")?;

        Ok(())
    }

    fn run_cycle(&mut self, cycle: u64) -> Result<()> {
        let state = FortressState::mock(&self.config.fortress_name, cycle);
        let plan = self.planner.plan(&state);
        let narration = self.narrator.describe(&state, &plan);

        println!();
        println!("=== CYCLE {cycle} ===");

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
