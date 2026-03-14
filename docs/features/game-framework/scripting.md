# 13.4 — Scripting

## Visual Scripting

### F-13.4.1 Visual Scripting Graph

A node-based visual scripting system where designers author gameplay logic as directed graphs of
typed nodes connected by data and execution flow edges. Node categories include events, actions,
conditions, math, ECS queries, and ability/quest triggers. Graphs compile to an optimized bytecode
representation for runtime execution, avoiding interpretation overhead per tick. Designed for
quest scripting, encounter mechanics, and world event orchestration at MMO scale.

- **Requirements:** R-13.4.1
- **Dependencies:** F-1.5.1 (Typed Event Channels), F-1.3.1 (Type Registry)
- **Platform notes:** None

### F-13.4.2 Visual Script Debugging

Provides breakpoints, step-through execution, node-level data inspection, and execution flow
highlighting for visual script graphs in the editor. Designers can pause a running graph, inspect
variable values at each node, and watch execution propagate through branches. Supports remote
debugging against a running game server so encounter scripts can be diagnosed without local
reproduction.

- **Requirements:** R-13.4.2
- **Dependencies:** F-13.4.1, F-15.6.1 (Debugger)
- **Platform notes:** None

## Script Runtime

### F-13.4.3 Embedded Script Runtime

Embeds a lightweight scripting language runtime (Rhai) for text-based gameplay scripting alongside
the visual scripting system. Scripts can query and mutate ECS components, send events, invoke
abilities, and access game services through a sandboxed API. The runtime enforces memory and
instruction-count budgets per script invocation to prevent runaway scripts from stalling the
server tick.

- **Requirements:** R-13.4.3
- **Dependencies:** F-1.1.5 (Archetype Queries), F-1.5.1 (Typed Event Channels)
- **Platform notes:** None

### F-13.4.4 Script-to-ECS Bridge

Exposes ECS world access to scripts through a typed bridge layer that maps script-side types to
Rust component types using the reflection system. Scripts read and write components, spawn and
despawn entities, register systems, and subscribe to events using the same access patterns as
native Rust code. The bridge validates all operations against the ECS access rules to prevent
data races in parallel script execution.

- **Requirements:** R-13.4.4
- **Dependencies:** F-13.4.3, F-1.3.1 (Type Registry), F-1.3.3 (Property System)
- **Platform notes:** None

## Hot Reload and Async

### F-13.4.5 Script Hot Reload

Detects script file changes on disk (or pushed from the editor) and hot-reloads them into the
running game without restart. Reloading preserves live state by serializing script-owned variables
before unloading and restoring them into the new script instance. For visual scripts, graph asset
reloading triggers recompilation to bytecode and seamless replacement of running graph instances.

- **Requirements:** R-13.4.5
- **Dependencies:** F-13.4.3, F-13.4.1, F-12.1.1 (Asset Import)
- **Platform notes:** File watch uses platform-native mechanisms (ReadDirectoryChangesW on
  Windows, FSEvents on macOS, inotify on Linux).

### F-13.4.6 Script Coroutines and Async Tasks

Supports coroutines within scripts that can yield execution across frames using `await`-style
syntax for delays, animation completion, event arrival, or network responses. Coroutines are
scheduled cooperatively within the ECS system scheduler so they do not block the game thread.
Enables natural authoring of multi-step quest sequences, phased boss encounters, and timed
world events without manual state machine boilerplate.

- **Requirements:** R-13.4.6
- **Dependencies:** F-13.4.3, F-1.1.9 (Run Criteria)
- **Platform notes:** None

## Performance and Safety

### F-13.4.7 Script Performance Sandboxing

Enforces per-script execution budgets (instruction count, memory allocation, ECS query result
limits) to prevent misbehaving scripts from degrading server performance. Scripts that exceed
their budget are suspended with a diagnostic warning and resume on the next tick or are
terminated after repeated violations. Budget thresholds are configurable per script category
(quest, encounter, world event) to match their expected complexity.

- **Requirements:** R-13.4.7
- **Dependencies:** F-13.4.3
- **Platform notes:** None
