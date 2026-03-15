# R-7.5 -- Goal-Oriented Action Planning Requirements

## R-7.5.1 World State Representation

The engine **SHALL** model AI-relevant world state as a fixed-size bitset of named boolean and
integer properties that is copyable, comparable, and diffable in constant time, enabling fast
forward-search planning for thousands of NPCs per tick.

- **Derived from:** [F-7.5.1](../../features/ai/goap.md)
- **Rationale:** Planner performance depends on cheap state operations; a fixed-size bitset
  enables O(1) copy, compare, and diff without heap allocation.
- **Verification:** Create a world state with 64 properties. Measure copy, compare, and diff
  operations and verify each completes in constant time (no scaling with property count up to
  the fixed size). Verify named property access returns correct values after set/clear
  operations. Verify two independently constructed identical states compare as equal.

## R-7.5.2 GOAP Forward-Search Planner

The engine **SHALL** perform A* search over the action space from the current world state toward
a goal state, finding a minimal-cost sequence of actions whose preconditions are satisfied and
whose cumulative effects produce the goal state.

- **Derived from:** [F-7.5.2](../../features/ai/goap.md)
- **Rationale:** Forward-search planning lets NPCs dynamically compose behavior sequences
  instead of requiring hand-authored paths for every situation.
- **Verification:** Define 5 actions with known costs, preconditions, and effects. Set a goal
  state reachable by two valid action sequences (costs 10 and 15). Verify the planner returns
  the cost-10 sequence. Verify the plan's cumulative effects transform the initial state into
  the goal state. Verify an unreachable goal returns a planning failure within the configured
  iteration limit.

## R-7.5.3 Action Preconditions and Effects

The engine **SHALL** require each GOAP action to declare a set of world-state preconditions that
must hold before execution and a set of effects that modify the world state upon completion,
along with a cost used by the planner to prefer cheaper plans.

- **Derived from:** [F-7.5.3](../../features/ai/goap.md)
- **Rationale:** Explicit preconditions and effects enable the planner to reason about action
  applicability and chain actions into valid sequences automatically.
- **Verification:** Define an action "equip_weapon" with precondition (has_weapon_in_inventory
  = true) and effect (has_weapon = true, cost = 2). Verify the planner considers this action
  only when the precondition holds. Execute the action and verify the world state reflects the
  declared effects. Verify the cost is accumulated correctly across a multi-action plan.

## R-7.5.4 Plan Caching and Reuse

The engine **SHALL** cache recently computed plans keyed by (goal, initial-state-hash) and reuse
them for identical planning requests across NPCs of the same archetype, invalidating cache
entries when registered actions change or a configurable TTL expires.

- **Derived from:** [F-7.5.4](../../features/ai/goap.md)
- **Rationale:** Many NPCs of the same archetype plan for identical goals from similar states;
  caching avoids redundant A* searches and reduces CPU cost.
- **Verification:** Issue identical planning requests from 10 NPCs with the same archetype,
  goal, and initial state. Verify the planner executes A* only once and the remaining 9
  requests return the cached plan. Modify a registered action and verify the cache entry is
  invalidated. Wait for the TTL to expire and verify the next request triggers a fresh search.

## R-7.5.5 Replanning Triggers

The engine **SHALL** monitor executing plans for invalidation conditions (action preconditions
no longer hold, goal changed, high-priority blackboard event) and trigger partial or full
replan with a cooldown to prevent replanning thrash.

- **Derived from:** [F-7.5.5](../../features/ai/goap.md)
- **Rationale:** Plans become invalid when world state changes unexpectedly; replanning
  maintains coherent NPC behavior while cooldowns prevent CPU waste from rapid state churn.
- **Verification:** Execute a plan and invalidate the current action's precondition mid-
  execution. Verify a replan triggers within one tick. Set a replan cooldown of 1 second and
  rapidly invalidate conditions 5 times within that window. Verify only one replan occurs
  during the cooldown period. Verify a goal change triggers immediate replanning regardless
  of cooldown.

## R-7.5.6 Goal Prioritization

The engine **SHALL** maintain a scored list of goals per agent with dynamic priorities derived
from utility considerations or blackboard values, driving the planner with the highest-priority
unsatisfied goal and replanning when a higher-priority goal emerges.

- **Derived from:** [F-7.5.6](../../features/ai/goap.md)
- **Rationale:** NPCs must dynamically switch goals based on changing circumstances (combat
  overrides exploration, survival overrides combat) without manual scripting.
- **Verification:** Configure an agent with goals "explore" (priority 0.3), "fight" (priority
  0.7), and "survive" (priority 0.9, condition: health < 25%). Verify the agent plans for
  "fight" when health is full. Reduce health below 25% and verify the agent abandons the
  current plan and replans for "survive" within one tick. Satisfy the "survive" goal and
  verify the agent resumes planning for "fight."
