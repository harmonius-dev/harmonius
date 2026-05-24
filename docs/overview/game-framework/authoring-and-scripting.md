# Authoring and Scripting

Script systems, visual scripting, and event-driven gameplay.

## What it covers

- Scripting languages: integration with Lua, Python, or custom DSLs.
- Hot reload: reloading scripts during development without restarting.
- Visual scripting: node-based behavior graphs and state machines.
- Events and messaging: decoupled communication between systems.
- Cutscenes and cinematics: scripted camera and animation sequences.
- Dialogue systems: branching conversations and NPC interactions.
- Progression triggers: unlocking content based on conditions.
- Debugging tools: script inspection and breakpoints.
- Script sandboxing: security isolation for untrusted scripts.
- Community modding: exposing APIs for player-created mods.

## Concepts

### Scripting Languages

Games integrate scripting languages for flexible behavior authoring. Lua is lightweight and fast;
Python is familiar to many; custom DSLs are domain-specific. Scripts access game APIs: query entity
position, spawn objects, play sounds. Hot reload enables rapid iteration: change script, reload,
test immediately without restarting editor.

### Visual Scripting

Visual scripting uses node graphs: action nodes (move, attack), condition nodes (is health low),
and flows connecting them. This enables non-programmers to author behavior. State machines organize
visual scripts hierarchically: global state machine contains per-state scripts. Transitions connect
states via conditions.

### Event Systems

Event-driven architecture decouples systems: health system broadcasts damage-taken event; any
script listening for it responds. Messaging systems route events: death event triggers quest
complete, plays audio, spawns loot. This enables modular, reusable scripts without tight coupling.

### Cutscenes and Cinematics

Cutscenes use timeline systems: events (play animation, move camera, show UI) scheduled on timeline.
Playback advances timeline, triggering events at specified times. Director tools enable in-editor
cutscene authoring: place camera, set target, scrub timeline, see results immediately. Exported
cutscenes play back during gameplay.

### Dialogue and Branching

Dialogue systems organize conversations as branching trees: player sees dialogue options, selections
lead to different conversation paths. Dialogue can trigger events (flag quest complete, spawn NPC).
Localization integrates: dialogue text and audio load based on game language.

## How it fits

- See [world-management.md](./world-management.md) for world state and entity management.
- See [gameplay-features.md](./gameplay-features.md) for gameplay systems and progression.
- See [../core-runtime/authoring-runtime.md](../core-runtime/authoring-runtime.md) for graph
  systems and plugins.
