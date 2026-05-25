Obsidian Engine
Autonomous Fortress Intelligence for Dwarf Fortress

"Let the mountain think. Watch the fortress fall."

Status:
Early autonomous systems scaffold written entirely in Rust.

Current systems:
- Config-driven engine loop
- Mock observer system
- DFHack bridge scaffold
- Strategic memory
- Policy gate
- Action intent mapping
- Chronicle logging
- Dry-run execution safety

Pipeline:
Observer
-> Observation Snapshot
-> Strategic Memory
-> Planner
-> Policy Gate
-> Action Intent
-> Executor
-> DFHack Bridge

Goals:
- Fortress Mode oversight
- Adventure Mode support
- Long-term memory
- Narrative fortress chronicles
- Autonomous planning
- Safe DFHack execution
- Local AI integration

Safety:
Live execution is intentionally restricted.
Default behavior is dry-run observation.

Architecture:
src/
  actions/
  chronicle/
  config/
  core/
  df/
  dfhack/
  executor/
  memory/
  narrator/
  observe/
  planner/
  policy/

Roadmap:
v0.x
- architecture scaffold
- observer abstraction
- strategic memory
- policy gate

v1.x
- real DFHack observer
- population parsing
- stocks parsing
- job parsing
- threat parsing

v2.x
- strategic fortress planning
- persistent cognition
- narrative summaries

v3.x
- autonomous fortress play
- safe live execution
- local AI models
