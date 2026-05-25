use anyhow::Result;
use std::{thread, time::Duration};

use crate::actions::ActionIntent;
use crate::chronicle::Chronicle;
use crate::config::{EngineConfig, ObserverMode};
use crate::dfhack::DfHackBridge;
use crate::executor::Executor;
use crate::memory::MemoryCore;
use crate::narrator::Narrator;
use crate::observe::{DfHackObserver, MockObserver, Observer};
use crate::planner::Planner;
use crate::policy::{PolicyCore, PolicyDecision};

pub struct ObsidianEngine {
    config: EngineConfig,
    chronicle: Chronicle,
    planner: Planner,
    policy: PolicyCore,
    executor: Executor,
    narrator: Narrator,
    dfhack: DfHackBridge,
    observer: Box<dyn Observer>,
    memory: MemoryCore,
}

impl ObsidianEngine {
    pub fn new(config_path: &str) -> Result<Self> {
        let config = EngineConfig::load(config_path)?;

        let observer: Box<dyn Observer> = match config.observer {
            ObserverMode::Mock => Box::new(MockObserver::from_config(&config)),
            ObserverMode::Dfhack => Box::new(DfHackObserver::from_config(&config)),
        };

        Ok(Self {
            chronicle: Chronicle::new(&config.chronicle_path),
            planner: Planner::new(),
            policy: PolicyCore::new(config.dry_run),
            executor: Executor::new(config.dry_run),
            narrator: Narrator::new(),
            dfhack: DfHackBridge::new(&config.dfhack_command, config.dry_run),
            observer,
            memory: MemoryCore::new(),
            config,
        })
    }

    pub fn boot(&mut self) -> Result<()> {
        println!("Obsidian Engine — Autonomous Fortress Intelligence");
        println!("Let the mountain think. Watch the fortress fall.");
        println!("Mode: {:?}", self.config.mode);
        println!("Observer: {:?}", self.config.observer);
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
        let context = self.memory.absorb(&snapshot);
        let state = &snapshot.state;
        let plan = self.planner.plan(state, &context);
        let proposed_intent = ActionIntent::from_directive(&plan.directive);

        let policy_decision = self.policy.evaluate(&plan, &proposed_intent, &context);
        let final_intent = match &policy_decision {
            PolicyDecision::Approved => proposed_intent.clone(),
            PolicyDecision::Blocked(_) => ActionIntent {
                label: "blocked".to_string(),
                dfhack_command: "ls".to_string(),
                safety_note: "Policy blocked directive. Observation-only fallback.".to_string(),
            },
            PolicyDecision::Downgraded { replacement, .. } => replacement.clone(),
        };

        let narration = self.narrator.describe(state, &plan);

        println!();
        println!("=== CYCLE {cycle} ===");

        println!();
        println!("OBSERVATION SNAPSHOT:");
        println!("Cycle: {}", snapshot.cycle);
        println!("Source: {}", snapshot.source.label());
        println!("Raw events: {:?}", snapshot.raw_events);

        println!();
        println!("STRATEGIC MEMORY:");
        println!("{context:#?}");
        println!("Memory line: {}", context.resource_line());

        println!();
        println!("FORTRESS STATE:");
        println!("{state:#?}");

        println!();
        println!("OBSIDIAN DIRECTIVE:");
        println!("{plan:#?}");

        println!();
        println!("PROPOSED ACTION INTENT:");
        println!("{proposed_intent:#?}");

        println!();
        println!("POLICY DECISION:");
        println!("{policy_decision:#?}");
        match &policy_decision {
            PolicyDecision::Blocked(reason) => println!("Policy block reason: {reason}"),
            PolicyDecision::Downgraded { reason, .. } => println!("Policy downgrade reason: {reason}"),
            PolicyDecision::Approved => {}
        }

        println!();
        println!("FINAL ACTION INTENT:");
        println!("{final_intent:#?}");

        println!();
        println!("CHRONICLE:");
        println!("{narration}");

        self.executor.execute(&final_intent, &self.dfhack)?;
        self.chronicle
            .record("observation", &format!("{snapshot:?}"))?;
        self.chronicle
            .record("strategic_context", &format!("{context:?}"))?;
        self.chronicle
            .record("resource_line", &context.resource_line())?;
        self.chronicle.record("directive", &format!("{plan:?}"))?;
        self.chronicle
            .record("proposed_action_intent", &format!("{proposed_intent:?}"))?;
        self.chronicle
            .record("policy_decision", &format!("{policy_decision:?}"))?;
        self.chronicle
            .record("final_action_intent", &format!("{final_intent:?}"))?;
        self.chronicle.record("narration", &narration)?;

        Ok(())
    }
}
