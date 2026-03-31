# R-7.3 -- Behavior Tree Requirements

## Node Types

1. **R-7.3.1** -- The engine **SHALL** provide Sequence (fail-fast), Selector (succeed-fast),
   Parallel (configurable success/failure policies), and Leaf (action or condition) node types as
   the structural backbone of behavior trees.
   - **Rationale:** These four composites are the minimal set required to express all standard
     behavior tree patterns without code.
   - **Verification:** Build a Sequence with 3 children where the second fails. Verify children 1
     and 2 execute but child 3 does not, and the Sequence returns failure. Build a Parallel with
     policy "succeed on all" and verify it succeeds only when every child succeeds.

2. **R-7.3.2** -- The engine **SHALL** provide Inverter, Repeater, Succeeder, Rate Limiter, and
   Cooldown decorator nodes that wrap a single child and modify its evaluation.
   - **Rationale:** Decorators modify child behavior without subtree duplication, enabling reusable
     patterns like throttled checks and cooldown-gated abilities.
   - **Verification:** Attach a Cooldown decorator with 3 s duration. Verify re-entry is blocked for
     exactly 3 s. Attach a Rate Limiter at 2 Hz and verify the child ticks at most twice per second
     regardless of the tree's tick rate.

3. **R-7.3.3** -- The engine **SHALL** support conditional aborts with self-abort, lower-priority
   abort, and combined modes, interrupting the active branch within the same tick when conditions
   change.
   - **Rationale:** Without aborts, behavior trees cannot react to high-priority events until the
     current branch completes, making AI unresponsive.
   - **Verification:** Build a tree with a lower-priority patrol branch active and a higher-priority
     combat branch with a conditional abort. Change the condition to true and verify the abort fires
     within the same tick, interrupting patrol.

## Blackboard

1. **R-7.3.4** -- The engine **SHALL** provide a typed key-value store per AI agent with scoped keys
   (self, group, global), where self-scoped keys are invisible to other agents and group-scoped keys
   are shared among squad members, with change-notification observers.
   - **Rationale:** Behavior tree nodes need shared memory for passing data without tight coupling;
     scoped keys prevent information leakage between agents.
   - **Verification:** Set a self-scoped key on agent A and verify agent B cannot read it. Set a
     group-scoped key and verify all squad members read it. Register an observer, modify the value,
     and verify the observer fires within the same tick.

## Assets and Serialization

1. **R-7.3.5** -- The engine **SHALL** load behavior trees from declarative data assets at runtime,
   with hot-reload support during development that applies updates within 1 second of file
   modification.
   - **Rationale:** Data-driven behavior trees let designers iterate on NPC behavior without code
     changes or restarts.
   - **Verification:** Load a BT from a data asset and verify correct execution. Modify the asset
     and verify the running tree reflects changes within 1 s. Verify hot-reload is stripped from
     shipping builds.

2. **R-7.3.6** -- The engine **SHALL** support referencing shared subtrees by handle, expanding them
   inline or as nested scopes, with circular reference detection at load time.
   - **Rationale:** Common patterns should be defined once and reused; circular references must be
     caught to prevent infinite loops.
   - **Verification:** Reference a shared subtree from 3 NPC trees and verify all reflect changes
     when the subtree is modified. Create a circular reference and verify a diagnostic error at load
     time.

## Debugging

1. **R-7.3.7** -- The engine **SHALL** provide a per-agent trace log recording every node visit and
   return status per tick, and an editor overlay rendering the tree with color-coded node states and
   live blackboard contents.
   - **Rationale:** Debugging behavior trees requires visibility into the execution path; trace logs
     and visual overlays let designers diagnose incorrect NPC behavior.
   - **Verification:** Select an agent and verify the trace log records every node visited with
     correct return status. Verify the editor overlay shows matching color-coded states and live
     blackboard values.
