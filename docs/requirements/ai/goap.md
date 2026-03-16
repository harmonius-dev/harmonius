# R-7.5 -- Goal-Oriented Action Planning Requirements

## World State

### R-7.5.1 World State Representation

The engine **SHALL** model AI-relevant world state as a fixed-size bitset of named boolean and
integer properties, with copy, compare, and diff operations completing in O(1) time regardless of
property count.

- **Derived from:**
  [F-7.5.1](../../features/ai/goap.md)
- **Rationale:** The GOAP planner copies and compares world states thousands of times per search;
  O(1) bitset operations keep planning cost bounded.
- **Verification:** Create a world state with 64 properties. Benchmark copy, compare, and diff
  operations and verify each completes in constant time. Double the property count to 128 and verify
  operation time does not increase.

## Planner

### R-7.5.2 GOAP Forward-Search Planner

The engine **SHALL** perform A* search over the action space from the current world state toward a
goal state, finding a minimal-cost sequence of actions that satisfies the goal preconditions.

- **Derived from:**
  [F-7.5.2](../../features/ai/goap.md)
- **Rationale:** Forward search enables NPCs to compose multi-step plans dynamically, producing
  intelligent behavior without hand-authored action sequences.
- **Verification:** Define 5 actions with varying costs that can satisfy a goal. Verify the planner
  returns the cheapest valid sequence. Define an alternative 2-action sequence with lower total cost
  and verify the planner prefers it. Verify the planner returns failure when no valid sequence
  exists.

### R-7.5.3 Action Preconditions and Effects

The engine **SHALL** define per-action preconditions (world state properties that must hold before
execution) and effects (world state modifications applied upon completion), with a cost value per
action used by the planner for plan optimization.

- **Derived from:**
  [F-7.5.3](../../features/ai/goap.md)
- **Rationale:** Preconditions and effects are the formal mechanism by which the planner reasons
  about action applicability and chaining.
- **Verification:** Define an action with precondition "has_weapon = true." Verify the planner only
  considers it when has_weapon is true in the current state. Verify the action's effects modify the
  world state correctly after execution. Verify lower-cost actions are preferred when multiple
  actions achieve the same effect.

## Plan Management

### R-7.5.4 Plan Caching and Reuse

The engine **SHALL** cache computed plans keyed by (goal, initial-state-hash), reusing cached plans
for identical requests across multiple agents of the same archetype, with cache invalidation on
action set changes or TTL expiration.

- **Derived from:**
  [F-7.5.4](../../features/ai/goap.md)
- **Rationale:** NPCs of the same archetype with the same goal and state should not each pay the
  full A* search cost; caching amortizes planning across the population.
- **Verification:** Issue 10 identical planning requests (same goal, same state hash) and verify
  only 1 A* search executes. Modify the action set and verify the cache is invalidated. Verify a
  cached plan expires after the configured TTL.

### R-7.5.5 Replanning Triggers

The engine **SHALL** trigger replanning when an executing action's preconditions become invalid,
when the active goal changes, or when a high-priority blackboard event fires, with a configurable
cooldown that limits replanning frequency to prevent thrashing.

- **Derived from:**
  [F-7.5.5](../../features/ai/goap.md)
- **Rationale:** Plans become stale when the world changes; replanning ensures agents adapt, while
  the cooldown prevents wasting CPU on rapid re-searches during volatile situations.
- **Verification:** Invalidate an executing action's precondition and verify replanning occurs
  within 1 tick. Trigger 5 replan events within the cooldown period and verify only 1 replan
  executes. Verify a goal change triggers immediate replan regardless of cooldown.

### R-7.5.6 Goal Prioritization

The engine **SHALL** maintain a scored list of goals per agent with dynamic priorities, where the
highest-priority unsatisfied goal drives the planner, and a higher-priority goal emerging causes
immediate replan within one tick.

- **Derived from:**
  [F-7.5.6](../../features/ai/goap.md)
- **Rationale:** Agents must switch goals when circumstances change (e.g., abandoning combat to flee
  when near death) without waiting for the current plan to complete.
- **Verification:** Assign 3 goals with priorities 0.3, 0.6, and 0.9. Verify the planner targets the
  0.9-priority goal. Satisfy that goal and verify the planner switches to the 0.6-priority goal.
  Introduce a new goal with priority 1.0 and verify replan occurs within 1 tick.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/ai/goap.md](../../user-stories/ai/goap.md).
Requirements in this document are derived from those
user stories.
