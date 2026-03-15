# R-7.3 -- Behavior Tree Requirements

## R-7.3.1 Core Composite and Leaf Nodes

The engine **SHALL** provide Sequence (fails on first child failure), Selector (succeeds on first
child success), Parallel (runs all children with configurable success/failure policies), and Leaf
(executes a single action or condition check) node types as the structural backbone of behavior
trees.

- **Derived from:** [F-7.3.1](../../features/ai/behavior-trees.md)
- **Rationale:** These four node types form the minimum complete set for expressing arbitrary
  NPC behavior as tree-structured logic.
- **Verification:** Build a test tree with a Sequence of three leaves (success, success,
  failure) and verify the Sequence returns failure after the third child. Build a Selector of
  three leaves (failure, success, failure) and verify it returns success after the second
  child. Build a Parallel with a require-all policy and verify it returns failure when any
  child fails. Verify Leaf nodes execute their action exactly once per tick.

## R-7.3.2 Decorator Nodes

The engine **SHALL** provide decorator nodes that wrap a single child with modifiers: Inverter
(negates result), Repeater (loops N times or until failure), Succeeder (always returns success),
Rate Limiter (throttles tick frequency), and Cooldown (blocks re-entry for a configured
duration).

- **Derived from:** [F-7.3.2](../../features/ai/behavior-trees.md)
- **Rationale:** Decorators enable reusable behavior patterns (retry logic, rate limiting)
  without duplicating subtrees or adding special-case logic to leaf nodes.
- **Verification:** (1) Inverter wrapping a success leaf returns failure. (2) Repeater(3)
  executes its child exactly 3 times. (3) Succeeder wrapping a failure leaf returns success.
  (4) Rate Limiter with 500 ms interval skips ticks within the interval. (5) Cooldown with
  2 s duration blocks re-entry for 2 seconds after the child completes, measured via tick
  count.

## R-7.3.3 Conditional Aborts

The engine **SHALL** allow composite nodes to re-evaluate conditions while a lower-priority
branch is running, supporting self-abort, lower-priority abort, and both modes combined, to
enable immediate reaction to state changes.

- **Derived from:** [F-7.3.3](../../features/ai/behavior-trees.md)
- **Rationale:** Without conditional aborts, a running low-priority branch blocks the tree
  from reacting to high-priority events like phase transitions or threat changes.
- **Verification:** Build a Selector with two branches: branch A (condition: health < 50%,
  action: heal) and branch B (action: patrol). Start branch B running with health at 80%.
  Set health to 40% and verify branch B is aborted and branch A begins executing within the
  same tick. Verify self-abort re-checks its own condition and aborts when the condition
  becomes false.

## R-7.3.4 Blackboard System

The engine **SHALL** provide a typed key-value store per AI agent supporting scoped keys (self,
group, global) and change-notification observers, serving as shared memory between behavior tree
nodes.

- **Derived from:** [F-7.3.4](../../features/ai/behavior-trees.md)
- **Rationale:** Behavior tree nodes need shared state for passing data (targets, positions,
  flags) without tight coupling; scoped keys enable squad-level data sharing.
- **Verification:** Set a self-scoped key "target_entity" on agent A and verify agent B cannot
  read it. Set a group-scoped key "rally_point" and verify all agents in the group read the
  same value. Register an observer on a key and verify the callback fires within the same tick
  when the value changes. Verify type safety by attempting to read a float key as an integer
  and confirming an error is returned.

## R-7.3.5 Behavior Tree Assets and Serialization

The engine **SHALL** load behavior trees from declarative data assets (RON or JSON format) at
runtime, resolving node types by registered name, and support hot-reload during development
without restarting the server.

- **Derived from:** [F-7.3.5](../../features/ai/behavior-trees.md)
- **Rationale:** Data-driven behavior trees let designers iterate on NPC logic without code
  changes or server restarts, accelerating content development.
- **Verification:** Load a behavior tree from a RON asset file and verify it executes correctly.
  Modify the asset file on disk, trigger hot-reload, and verify the running agent switches to
  the updated behavior within 1 second. Verify that a tree referencing an unregistered node
  type fails to load with a descriptive error.

## R-7.3.6 Subtree References and Reuse

The engine **SHALL** support a node type that references another behavior tree asset by handle,
expanding it inline at load time or executing it as a nested scope, enabling modular authoring
of reusable behavior patterns.

- **Derived from:** [F-7.3.6](../../features/ai/behavior-trees.md)
- **Rationale:** Common patterns (patrol, flee, call for help) should be defined once and
  reused across NPC archetypes to avoid duplication and inconsistency.
- **Verification:** Create a "patrol" subtree asset and reference it from two different NPC
  behavior trees. Verify both NPCs execute identical patrol behavior. Modify the shared subtree
  and hot-reload; verify both NPCs reflect the change. Verify that circular subtree references
  are detected at load time and produce an error.

## R-7.3.7 Runtime Debugging and Visualization

The engine **SHALL** produce a server-side trace log of node visits, results, and blackboard
mutations per tick for a selected agent, and provide an optional editor overlay rendering the
tree structure with color-coded node states (running, success, failure) and live blackboard
contents.

- **Derived from:** [F-7.3.7](../../features/ai/behavior-trees.md)
- **Rationale:** Behavior tree debugging requires visibility into per-tick execution flow and
  state changes to diagnose incorrect NPC behavior.
- **Verification:** Select an agent and enable tracing. Verify the trace log records every node
  visit with its return status and any blackboard mutations for that tick. Verify the editor
  overlay renders all nodes with correct color coding matching their current state. Confirm
  the trace log is available in non-editor builds and the overlay is editor-only.
