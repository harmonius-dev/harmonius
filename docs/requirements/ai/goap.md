# R-7.5 -- Goal-Oriented Action Planning Requirements

## World State

1. **R-7.5.1** -- The engine **SHALL** model AI-relevant world state as a fixed-size bitset of named
   boolean and integer properties with O(1) copy, compare, and diff operations.
   - **Rationale:** Compact trivially-copyable world states enable fast forward-search planning for
     thousands of NPCs without heap allocation.
   - **Verification:** Encode boolean and integer properties and verify lossless round-trip. Diff
     two states and verify all changed properties are identified. Benchmark copy and diff for 1000+
     concurrent agents.

## Planner

1. **R-7.5.2** -- The engine **SHALL** perform A* forward search from the current world state toward
   a goal state over the action space, finding minimal-cost action sequences within a per-tick
   iteration cap.
   - **Rationale:** A* over actions produces optimal plans; the iteration cap prevents planning from
     exceeding the per-tick AI budget.
   - **Verification:** Verify the planner finds the lowest-cost sequence for reference scenarios.
     Verify graceful no-plan return when the goal is unreachable. Benchmark plans per tick across
     platforms.

2. **R-7.5.3** -- The engine **SHALL** define each GOAP action with world-state preconditions,
   effects stored as deltas, and a numeric cost, with per-archetype action registration.
   - **Rationale:** Precondition-effect pairs drive the planner's search; per-archetype action sets
     produce personality-specific plans.
   - **Verification:** Verify an action cannot execute when preconditions are unmet. Verify
     executing an action applies its effects correctly. Verify total plan cost equals the sum of
     constituent action costs.

## Plan Management

1. **R-7.5.4** -- The engine **SHALL** cache computed plans keyed by goal and initial state hash,
   with invalidation on action registry changes or TTL expiration.
   - **Rationale:** Plan caching eliminates redundant search for identical planning requests across
     multiple NPCs of the same archetype.
   - **Verification:** Verify a cache hit returns a plan identical to fresh search. Modify the
     action registry and verify affected cache entries are invalidated. Benchmark throughput with
     and without caching.

2. **R-7.5.5** -- The engine **SHALL** monitor executing plans for invalidation conditions and
   trigger partial or full replan with a configurable cooldown, where high-priority events bypass
   the cooldown.
   - **Rationale:** Plans become stale when world state changes; replanning with a cooldown prevents
     thrashing while critical events demand immediate response.
   - **Verification:** Verify a plan is invalidated when preconditions no longer hold. Verify the
     cooldown prevents multiple replans within the window. Verify high-priority events bypass the
     cooldown.

3. **R-7.5.6** -- The engine **SHALL** maintain a scored goal list per agent with dynamic
   priorities, planning for the highest-priority unsatisfied goal and replanning when a
   higher-priority goal emerges.
   - **Rationale:** NPCs need to switch between goals like patrol, combat, and survival based on
     changing context.
   - **Verification:** Verify the planner always plans for the highest-priority unsatisfied goal.
     Verify a satisfied goal is skipped. Verify changing a blackboard value updates priorities and
     triggers a goal switch.
