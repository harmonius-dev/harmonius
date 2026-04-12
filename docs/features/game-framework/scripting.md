# 13.4 — Gameplay Scripting Integration

## Visual Logic

| ID       | Feature                            |
|----------|------------------------------------|
| F-13.4.1 | Gameplay Logic Graph Integration   |
| F-13.4.2 | Logic Graph Debugging for Gameplay |
| F-13.4.3 | Logic Graph Hot Reload             |

1. **F-13.4.1** — All gameplay scripting is authored through the universal logic graph system
   (F-15.8.4). This feature defines the gameplay-specific integration points: access to ECS world
   state (entities, components, resources), gameplay event emission and handling, ability activation
   and state machine transitions, and AI blackboard read/write. Gameplay graphs are attached to
   entities as components and execute in the ECS system schedule. No text-based scripting language
   is provided — all logic is visual.
   - **Deps:** F-15.8.4 (Gameplay Logic Graphs), F-15.8.1 (Universal Logic Graph Runtime), F-1.1.1
     (ECS)
2. **F-13.4.2** — Visual debugging of gameplay logic graphs using the graph debugger (F-15.8.11).
   Gameplay-specific debug features include: ECS query result inspection, component value watch
   expressions, event flow visualization (which events triggered which graph executions), and
   per-entity graph instance state comparison. Breakpoints pause game simulation while allowing
   editor interaction. The debugger supports remote debugging — connect to a running game instance
   over the network to debug logic graphs in real time.
   - **Deps:** F-13.4.1, F-15.8.11 (Graph Debugging)
3. **F-13.4.3** — Modify gameplay logic graphs in the editor while the game is running, with changes
   applied immediately without restarting the game session. The hot reload system detects graph
   asset changes, recompiles affected graphs through the shared build cache (F-15.11.3), and patches
   running graph instances with the new bytecode. Persistent state (local variables, coroutine
   positions) is preserved across reloads when the variable layout is compatible. Incompatible
   changes trigger a clean restart of the affected graph instance with a warning.
   - **Deps:** F-13.4.1, F-15.11.3 (Logic Graph Cache), F-12.4.1 (File Watcher)

## Sandbox, Scoping, and Lifecycle

| ID       | Feature                                           |
|----------|---------------------------------------------------|
| F-13.4.4 | Compile-Time Sandbox Enforcement                  |
| F-13.4.5 | Three-Scope Variable Model                        |
| F-13.4.6 | Graceful Runtime Error Handling                   |
| F-13.4.7 | Graph Execution Lifecycle Events                  |

1. **F-13.4.4** — Enforces sandbox constraints at compile time on all user-authored logic graphs.
   Generated modules compile with `#![forbid(unsafe_code)]`. The compiler rejects graphs containing
   raw pointer operations or unbounded loops. Budget checks are inserted at loop back-edges to
   terminate runaway execution with a BudgetExhausted error. Sandbox rules are non-configurable to
   prevent accidental engine corruption.
   - **Deps:** F-15.8.12 (Graph Compilation)
2. **F-13.4.5** — Three variable scopes for logic graph variables: local (reset on each graph
   execution), graph (persistent per graph instance across frames), and entity (mapped to ECS
   components, shared with other systems). Scope is selected per variable in the graph editor.
   Entity-scoped variables are visible in the entity inspector and participate in save
   serialization.
   - **Deps:** F-13.4.1, F-1.1.1 (ECS)
3. **F-13.4.6** — Handles runtime errors in graph execution gracefully. When a graph encounters a
   missing component, division by zero, budget exhaustion, type mismatch, or empty query, the graph
   instance transitions to an error state. The error is logged with the node ID and a diagnostic
   message. The entity continues to exist; other systems are unaffected. The error state is visible
   in the graph debugger (F-13.4.2) and clearable via hot-reload.
   - **Deps:** F-13.4.1, F-13.4.2 (Graph Debugging)
4. **F-13.4.7** — Emits ECS events when a graph instance completes execution, enters an error state,
   or hot-reloads. Events carry the entity ID, graph asset reference, and error details (if any).
   Gameplay systems subscribe to these events to implement retry logic, cleanup, or UI feedback tied
   to graph execution state.
   - **Deps:** F-13.4.1, F-1.5.1 (Typed Event Channels)
