# R-7.3 -- Behavior Tree Requirements

## Node Types

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-7.3.1 | The engine **SHALL** provide Sequence (runs children in order, fails on first failure), Selector (runs children in order, succeeds on first success), Parallel (runs all children concurrently with configurable success/failure policies), and Leaf (executes a single action or condition check) node types as the structural backbone of behavior trees. | [F-7.3.1](../../features/ai/behavior-trees.md) | These four composites are the minimal set required to express all standard behavior tree patterns; they enable designers to structure complex NPC behavior as composable trees without code. | Build a Sequence with 3 children where the second fails. Verify children 1 and 2 execute but child 3 does not, and the Sequence returns failure. Build a Parallel with policy "succeed on all" and verify it succeeds only when every child succeeds. |
| R-7.3.2 | The engine **SHALL** provide Inverter (negates result), Repeater (loops N times or until failure), Succeeder (always returns success), Rate Limiter (throttles tick frequency), and Cooldown (blocks re-entry for a configurable duration) decorator nodes that wrap a single child. | [F-7.3.2](../../features/ai/behavior-trees.md) | Decorators modify child behavior without subtree duplication, enabling reusable patterns like throttled expensive checks and cooldown-gated abilities. | Attach a Cooldown decorator with 3-second duration. Verify the child executes on first tick, then verify re-entry is blocked for exactly 3 seconds. Attach a Rate Limiter at 2 Hz and verify the child ticks at most twice per second regardless of the tree's tick rate. |
| R-7.3.3 | The engine **SHALL** support conditional aborts that allow composite nodes to re-evaluate conditions while a lower-priority branch is running, with self-abort, lower-priority abort, and combined modes, interrupting the active branch within the same tick when conditions change. | [F-7.3.3](../../features/ai/behavior-trees.md) | Without aborts, behavior trees cannot react to high-priority events until the current branch completes, making AI unresponsive to phase transitions and sudden threats. | Build a tree with a lower-priority "patrol" branch active and a higher-priority "combat" branch with a conditional abort. Change the condition to true and verify the abort fires within the same tick, interrupting patrol and activating combat. |

## Blackboard

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-7.3.4 | The engine **SHALL** provide a typed key-value store per AI agent with scoped keys (self, group, global), where self-scoped keys are invisible to other agents and group-scoped keys are shared among squad members. Change-notification observers **SHALL** be supported for triggering conditional aborts on value changes. | [F-7.3.4](../../features/ai/behavior-trees.md) | Behavior tree nodes need shared memory for passing data (target entity, rally point, threat level) without tight coupling; scoped keys prevent information leakage between agents. | Set a self-scoped key on agent A and verify agent B cannot read it. Set a group-scoped key and verify all squad members can read it. Register a change observer on a key, modify the value, and verify the observer fires within the same tick. |

## Assets and Serialization

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-7.3.5 | The engine **SHALL** load behavior trees from declarative data assets (RON or JSON format) at runtime, with hot-reload support during development that applies updated behavior within 1 second of file modification. | [F-7.3.5](../../features/ai/behavior-trees.md) | Data-driven behavior trees let designers iterate on NPC behavior without code changes or server restarts, accelerating the design loop. | Load a behavior tree from a RON asset and verify it executes correctly. Modify the asset file and verify the running tree reflects the changes within 1 second. Verify hot-reload is stripped from shipping builds. |
| R-7.3.6 | The engine **SHALL** support referencing shared subtrees by handle, expanding them inline at load time or executing them as nested scopes, with circular reference detection at load time that reports a diagnostic error. | [F-7.3.6](../../features/ai/behavior-trees.md) | Common patterns (patrol, flee, call for help) should be defined once and reused across many NPC archetypes; circular references must be caught to prevent infinite loops. | Create a subtree "flee_behavior" and reference it from 3 different NPC trees. Modify the subtree and verify all 3 trees reflect the change. Create a circular reference (A references B, B references A) and verify a diagnostic error is reported at load time. |

## Debugging

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-7.3.7 | The engine **SHALL** provide a per-agent trace log recording every node visit and its return status per tick, and an editor overlay rendering the tree structure with color-coded node states (running, success, failure) and live blackboard contents. | [F-7.3.7](../../features/ai/behavior-trees.md) | Debugging behavior trees requires visibility into the execution path; trace logs and visual overlays let designers diagnose incorrect NPC behavior quickly. | Select an agent and verify the trace log records every node visited during a tick with the correct return status. Verify the editor overlay shows color-coded node states matching the trace log. Verify live blackboard values are displayed and update in real time. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/ai/behavior-trees.md](../../user-stories/ai/behavior-trees.md). Requirements in this
document are derived from those user stories.
