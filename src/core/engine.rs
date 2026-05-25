use anyhow::Result;
use std::{thread, time::Duration};

use crate::actions::ActionIntent;
use crate::chronicle::Chronicle;
use crate::config::EngineConfig;
use crate::dfhack::DfHackBridge;
use crate::executor::Executor;
use crate::narrator::Narrator;
use crate::observe::{DfHackObserver, MockObserver, Observer};
use crate::planner::Planner;

pub struct ObsidianEngine {
    config: EngineConfig,
    chronicle: Chronicle,
    planner: Planner,
    executor: Executor,
    narrator: Narrator,
    dfhack: DfHackBridge,
    observer: Box<dyn Observer>,
}

impl ObsidianEngine {
    pub fn new(config_path: &str) -> Result<Self> {
        let config = EngineConfig::load(config_path)?;
        let observer = Box::new(MockObserver::from_config(&config));

        Ok(Self {
            chronicle: Chronicle::new(&config.chronicle_path),
            planner: Planner::new(),
            executor: Executor::new(config.dry_run),
            narrator: Narrator::new(),
            dfhack: DfHackBridge::new(&config.dfhack_command, config.dry_run),
            observer,
            config,
        })
    }

    pub fn boot(&mut self) -> Result<()> {
        println!("Obsidian Engine — Autonomous Fortress Intelligence");
        println!("Let the mountain think. Watch the fortress fall.");
        println!("Mode: {:?}", self.config.mode);
        println!("Dry run: {}", self.config.dry_run);
        println!("Future live observer source: {}", DfHackObserver::source().label());

        let dfhack_status = if self.dfhack.is_available() {
            "available"
        } else if self.config.dry_run {
            "not detected, but allowed in dry-run mode"
        } else {
            "not detected"
        };

        println!("DFHack bridge: {dfhack_status}");

        self.chronicle
            .record("engine_boot", "Obsidian Engine awakened.")?;

        self.chronicle.record("dfhack_status", dfhack_status)?;

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
        let snapshot = self.observer.observe(cycle)?;
        let state = &snapshot.state;
        let plan = self.planner.plan(state);
        let intent = ActionIntent::from_directive(&plan.directive);
        let narration = self.narrator.describe(state, &plan);

        println!();
        println!("=== CYCLE {cycle} ===");

        println!();
        println!("OBSERVATION SNAPSHOT:");
        println!("Cycle: {}", snapshot.cycle);
        println!("Source: {}", snapshot.source.label());
        println!("Raw events: {:?}", snapshot.raw_events);

        println!();
        println!("FORTRESS STATE:");
        println!("{state:#?}");

        println!();
        println!("OBSIDIAN DIRECTIVE:");
        println!("{plan:#?}");

        println!();
        println!("ACTION INTENT:");
        println!("{intent:#?}");

        println!();
        println!("CHRONICLE:");
        println!("{narration}");

        self.executor.execute(&intent, &self.dfhack)?;
        self.chronicle
            .record("observation", &format!("{snapshot:?}"))?;
        self.chronicle.record("directive", &format!("{plan:?}"))?;
        self.chronicle.record("action_intent", &format!("{intent:?}"))?;
        self.chronicle.record("narration", &narration)?;

        Ok(())
    }
}
