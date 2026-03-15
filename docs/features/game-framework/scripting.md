# 13.4 — Gameplay Scripting Integration

## Visual Logic

### F-13.4.1 Gameplay Logic Graph Integration

All gameplay scripting is authored through the universal logic graph system (F-15.8.4). This feature
defines the gameplay-specific integration points: access to ECS world state (entities, components,
resources), gameplay event emission and handling, ability activation and state machine transitions,
and AI blackboard read/write. Gameplay graphs are attached to entities as components and execute in
the ECS system schedule. No text-based scripting language is provided — all logic is visual.

- **Requirements:** R-13.4.1
- **Dependencies:** F-15.8.4 (Gameplay Logic Graphs), F-15.8.1 (Universal Logic Graph Runtime),
  F-1.1.1 (ECS)
- **Platform notes:** None

### F-13.4.2 Logic Graph Debugging for Gameplay

Visual debugging of gameplay logic graphs using the graph debugger (F-15.8.11). Gameplay-specific
debug features include: ECS query result inspection, component value watch expressions, event flow
visualization (which events triggered which graph executions), and per-entity graph instance state
comparison. Breakpoints pause game simulation while allowing editor interaction. The debugger
supports remote debugging — connect to a running game instance over the network to debug logic
graphs in real time.

- **Requirements:** R-13.4.2
- **Dependencies:** F-13.4.1, F-15.8.11 (Graph Debugging)
- **Platform notes:** None

### F-13.4.3 Logic Graph Hot Reload

Modify gameplay logic graphs in the editor while the game is running, with changes applied
immediately without restarting the game session. The hot reload system detects graph asset changes,
recompiles affected graphs through the shared build cache (F-15.11.3), and patches running graph
instances with the new bytecode. Persistent state (local variables, coroutine positions) is
preserved across reloads when the variable layout is compatible. Incompatible changes trigger a
clean restart of the affected graph instance with a warning.

- **Requirements:** R-13.4.3
- **Dependencies:** F-13.4.1, F-15.11.3 (Logic Graph Cache), F-12.4.1 (File Watcher)
- **Platform notes:** None
