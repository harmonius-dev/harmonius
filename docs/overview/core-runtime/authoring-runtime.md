# Authoring Runtime

Graphs, plugins, hot reload, and configurable variables that are shared by every domain.

## What it covers

- Visual node-based graphs compiled to native code; all visual authoring (logic, materials,
  AI, VFX) outputs actual code.
- Plugins extending the engine with new systems, components, and events; hot-reloadable during
  development.
- Strong type registry enabling runtime type queries and reflection.
- Deterministic procedural randomness with forking for reproducible simulations.
- Condition expression trees (boolean logic) evaluated against world state.
- Distance-based falloff curves shared by audio, lighting, damage, and effects.
- Per-frame budgeted work queues with priority and deferral.
- Console variables (tunable parameters) stored, queried, and edited in real time.

## Concepts

### Graph Compilation

Designers author graphs visually. Each node maps to a language construct (math op, branching,
iteration, ECS query). The engine compiles graphs to native code with full type checking, then
hot-reloads into the running game during development. No bytecode VM; one language, one compiler.

### Plugins

Plugins declare the systems, components, and events they contribute. The engine builds a topological
load order, runs startup validation, and detects conflicts. During development, gameplay plugins
hot-reload: the engine serializes world state, shuts down the old plugin, loads the new one, and
restores the world.

### Dynamic Types

A global type registry maps type IDs to metadata (size, alignment, drop logic, properties).
Reflection enables reading/writing typed data by path, serialization round-tripping, and network
replication without code generation for every data type.

## How it fits

- See [data-and-io](./data-and-io.md) for how plugins preserve state across hot reload via
  serialization.
- See [simulation-loop](./simulation-loop.md) for when plugins initialize and shut down.
- Integrates with [tools](../tools/editor-framework.md) for graph editing and live preview.
