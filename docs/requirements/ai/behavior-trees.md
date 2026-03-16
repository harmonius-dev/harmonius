# R-7.3 -- Behavior Tree Requirements

## Node Types

### R-7.3.1 Core Composite and Leaf Nodes

The engine **SHALL** provide Sequence (runs children in order, fails on first failure), Selector
(runs children in order, succeeds on first success), Parallel (runs all children concurrently with
configurable success/failure policies), and Leaf (executes a single action or condition check) node
types as the structural backbone of behavior trees.

- **Derived from:**
  [F-7.3.1](../../features/ai/behavior-trees.md)
- **Rationale:** These four composites are the minimal set required to express all standard behavior
  tree patterns; they enable designers to structure complex NPC behavior as composable trees without
  code.
- **Verification:** Build a Sequence with 3 children where the second fails. Verify children 1 and 2
  execute but child 3 does not, and the Sequence returns failure. Build a Parallel with policy
  "succeed on all" and verify it succeeds only when every child succeeds.

### R-7.3.2 Decorator Nodes

The engine **SHALL** provide Inverter (negates result), Repeater (loops N times or until failure),
Succeeder (always returns success), Rate Limiter (throttles tick frequency), and Cooldown (blocks
re-entry for a configurable duration) decorator nodes that wrap a single child.

- **Derived from:**
  [F-7.3.2](../../features/ai/behavior-trees.md)
- **Rationale:** Decorators modify child behavior without subtree duplication, enabling reusable
  patterns like throttled expensive checks and cooldown-gated abilities.
- **Verification:** Attach a Cooldown decorator with 3-second duration. Verify the child executes on
  first tick, then verify re-entry is blocked for exactly 3 seconds. Attach a Rate Limiter at 2 Hz
  and verify the child ticks at most twice per second regardless of the tree's tick rate.

### R-7.3.3 Conditional Aborts

The engine **SHALL** support conditional aborts that allow composite nodes to re-evaluate conditions
while a lower-priority branch is running, with self-abort, lower-priority abort, and combined modes,
interrupting the active branch within the same tick when conditions change.

- **Derived from:**
  [F-7.3.3](../../features/ai/behavior-trees.md)
- **Rationale:** Without aborts, behavior trees cannot react to high-priority events until the
  current branch completes, making AI unresponsive to phase transitions and sudden threats.
- **Verification:** Build a tree with a lower-priority "patrol" branch active and a higher-priority
  "combat" branch with a conditional abort. Change the condition to true and verify the abort fires
  within the same tick, interrupting patrol and activating combat.

## Blackboard

### R-7.3.4 Blackboard System

The engine **SHALL** provide a typed key-value store per AI agent with scoped keys (self, group,
global), where self-scoped keys are invisible to other agents and group- scoped keys are shared
among squad members. Change- notification observers **SHALL** be supported for triggering
conditional aborts on value changes.

- **Derived from:**
  [F-7.3.4](../../features/ai/behavior-trees.md)
- **Rationale:** Behavior tree nodes need shared memory for passing data (target entity, rally
  point, threat level) without tight coupling; scoped keys prevent information leakage between
  agents.
- **Verification:** Set a self-scoped key on agent A and verify agent B cannot read it. Set a
  group-scoped key and verify all squad members can read it. Register a change observer on a key,
  modify the value, and verify the observer fires within the same tick.

## Assets and Serialization

### R-7.3.5 Behavior Tree Assets and Serialization

The engine **SHALL** load behavior trees from declarative data assets (RON or JSON format) at
runtime, with hot-reload support during development that applies updated behavior within 1 second of
file modification.

- **Derived from:**
  [F-7.3.5](../../features/ai/behavior-trees.md)
- **Rationale:** Data-driven behavior trees let designers iterate on NPC behavior without code
  changes or server restarts, accelerating the design loop.
- **Verification:** Load a behavior tree from a RON asset and verify it executes correctly. Modify
  the asset file and verify the running tree reflects the changes within 1 second. Verify hot-reload
  is stripped from shipping builds.

### R-7.3.6 Subtree References and Reuse

The engine **SHALL** support referencing shared subtrees by handle, expanding them inline at load
time or executing them as nested scopes, with circular reference detection at load time that reports
a diagnostic error.

- **Derived from:**
  [F-7.3.6](../../features/ai/behavior-trees.md)
- **Rationale:** Common patterns (patrol, flee, call for help) should be defined once and reused
  across many NPC archetypes; circular references must be caught to prevent infinite loops.
- **Verification:** Create a subtree "flee_behavior" and reference it from 3 different NPC trees.
  Modify the subtree and verify all 3 trees reflect the change. Create a circular reference (A
  references B, B references A) and verify a diagnostic error is reported at load time.

## Debugging

### R-7.3.7 Runtime Debugging and Visualization

The engine **SHALL** provide a per-agent trace log recording every node visit and its return status
per tick, and an editor overlay rendering the tree structure with color-coded node states (running,
success, failure) and live blackboard contents.

- **Derived from:**
  [F-7.3.7](../../features/ai/behavior-trees.md)
- **Rationale:** Debugging behavior trees requires visibility into the execution path; trace logs
  and visual overlays let designers diagnose incorrect NPC behavior quickly.
- **Verification:** Select an agent and verify the trace log records every node visited during a
  tick with the correct return status. Verify the editor overlay shows color-coded node states
  matching the trace log. Verify live blackboard values are displayed and update in real time.

---

## User Stories

## US-7.3.1 Core Composite & Leaf Nodes

### US-7.3.1.1

As a **designer (P-5)**, I want Sequence, Selector, Parallel, and Leaf node types so that I can
structure NPC behavior as composable trees.

### US-7.3.1.2

As a **player (P-23)**, I want NPCs to exhibit varied behaviors so that encounters feel dynamic
rather than scripted.

### US-7.3.1.3

As an **engine dev (P-26)**, I want Parallel with configurable success/failure policies so that
concurrent behaviors complete based on tunable rules.

### US-7.3.1.4

As an **engine tester (P-27)**, I want to verify Sequence fails on first child failure so that
composite node semantics are regression-tested.

---

## US-7.3.2 Decorator Nodes

### US-7.3.2.1

As a **designer (P-5)**, I want Inverter, Repeater, Succeeder, Rate Limiter, and Cooldown decorators
so that I can modify child node behavior without duplicating subtrees.

### US-7.3.2.2

As a **designer (P-5)**, I want Rate Limiter to throttle expensive subtrees so that costly behaviors
run at reduced frequency.

### US-7.3.2.3

As an **engine tester (P-27)**, I want to verify Cooldown blocks re-entry for the configured
duration so that decorator timing is regression-tested.

---

## US-7.3.3 Conditional Aborts

### US-7.3.3.1

As a **designer (P-5)**, I want conditional aborts that interrupt lower-priority branches so that AI
reacts immediately to phase transitions or threats.

### US-7.3.3.2

As a **player (P-23)**, I want boss NPCs to react instantly to phase changes so that encounters feel
responsive and challenging.

### US-7.3.3.3

As an **engine tester (P-27)**, I want to verify lower-priority abort fires within the same tick
when conditions change so that abort responsiveness is regression-tested.

---

## US-7.3.4 Blackboard System

### US-7.3.4.1

As a **designer (P-5)**, I want a typed key-value store per agent for shared memory so that behavior
tree nodes can pass data without tight coupling.

### US-7.3.4.2

As a **designer (P-5)**, I want scoped keys (self, group, global) so that squad members can share
tactical data like rally points.

### US-7.3.4.3

As an **engine dev (P-26)**, I want change-notification observers on blackboard keys so that
conditional aborts trigger on value changes.

### US-7.3.4.4

As an **engine tester (P-27)**, I want to verify self-scoped keys are invisible to other agents so
that scope isolation is regression-tested.

---

## US-7.3.5 Behavior Tree Assets & Serialization

### US-7.3.5.1

As a **designer (P-5)**, I want behavior trees authored as data assets (RON or JSON) so that I can
edit NPC behavior without code changes.

### US-7.3.5.2

As a **designer (P-5)**, I want hot-reload during development so that I can iterate on behavior
without restarting the server.

### US-7.3.5.3

As an **engine tester (P-27)**, I want to verify hot-reload applies updated behavior within 1 second
so that hot-reload latency is regression-tested.

---

## US-7.3.6 Subtree References & Reuse

### US-7.3.6.1

As a **designer (P-5)**, I want to reference shared subtrees (patrol, flee) by handle so that common
patterns are defined once and reused.

### US-7.3.6.2

As a **designer (P-5)**, I want shared subtree updates to propagate to all referencing trees so that
fixing a behavior fixes it everywhere.

### US-7.3.6.3

As an **engine tester (P-27)**, I want to verify circular subtree references are detected at load
time so that reference cycle detection is regression-tested.

---

## US-7.3.7 Runtime Debugging & Visualization

### US-7.3.7.1

As a **designer (P-5)**, I want a trace log of node visits and results per tick so that I can
diagnose incorrect NPC behavior.

### US-7.3.7.2

As a **designer (P-5)**, I want an editor overlay with color-coded node states so that I can see
active behavior visually during testing.

### US-7.3.7.3

As a **designer (P-5)**, I want live blackboard contents in the debug overlay so that I can inspect
data values driving behavior decisions.

### US-7.3.7.4

As an **engine tester (P-27)**, I want to verify the trace log records every node visit with correct
return status so that trace log accuracy is regression-tested.
