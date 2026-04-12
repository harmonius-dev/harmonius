# AI Behavior Systems Test Cases

Companion test cases for [behavior.md](behavior.md).

## Unit Tests

### TC-7.3.1.1 Sequence Fail-Fast

| # | Requirement |
|---|-------------|
| 1 | R-7.3.1     |
| 2 | R-7.3.1     |

1. **#1** — Sequence with children [Success, Failure, Success]
   - **Expected:** Failure after 2nd child; 3rd child never ticked
2. **#2** — Sequence with children [Success, Success, Success]
   - **Expected:** Success after all children ticked

### TC-7.3.1.2 Selector Succeed-Fast

| # | Requirement |
|---|-------------|
| 1 | R-7.3.1     |
| 2 | R-7.3.1     |

1. **#1** — Selector with children [Failure, Success, Failure]
   - **Expected:** Success after 2nd child; 3rd child never ticked
2. **#2** — Selector with children [Failure, Failure, Failure]
   - **Expected:** Failure after all children ticked

### TC-7.3.1.3 Parallel RequireAll

| # | Requirement |
|---|-------------|
| 1 | R-7.3.1     |
| 2 | R-7.3.1     |

1. **#1** — Parallel(RequireAll) with children [Success, Success, Success]
   - **Expected:** Success
2. **#2** — Parallel(RequireAll) with children [Success, Failure, Success]
   - **Expected:** Failure

### TC-7.3.1.4 Parallel RequireOne

| # | Requirement |
|---|-------------|
| 1 | R-7.3.1     |
| 2 | R-7.3.1     |

1. **#1** — Parallel(RequireOne) with children [Failure, Success, Failure]
   - **Expected:** Success
2. **#2** — Parallel(RequireOne) with children [Failure, Failure, Failure]
   - **Expected:** Failure

### TC-7.3.2.1 Inverter Negation

| # | Requirement |
|---|-------------|
| 1 | R-7.3.2     |
| 2 | R-7.3.2     |
| 3 | R-7.3.2     |

1. **#1** — Inverter wrapping child that returns Success
   - **Expected:** Failure
2. **#2** — Inverter wrapping child that returns Failure
   - **Expected:** Success
3. **#3** — Inverter wrapping child that returns Running
   - **Expected:** Running (no inversion)

### TC-7.3.2.2 Repeater Count

| # | Requirement |
|---|-------------|
| 1 | R-7.3.2     |
| 2 | R-7.3.2     |
| 3 | R-7.3.2     |

1. **#1** — Repeater(count=3) wrapping Success child
   - **Expected:** Child ticked 3 times; Repeater returns Success
2. **#2** — Repeater(count=5) wrapping Success child
   - **Expected:** Child ticked 5 times; Repeater returns Success
3. **#3** — Repeater(count=3) wrapping Failure child
   - **Expected:** Child ticked 1 time; Repeater returns Failure

### TC-7.3.2.3 Cooldown Blocks Re-Entry

| # | Requirement |
|---|-------------|
| 1 | R-7.3.2     |
| 2 | R-7.3.2     |

1. **#1** — Cooldown(2.0s) child ticked at t=0, re-ticked at t=1.0
   - **Expected:** Failure (blocked by cooldown)
2. **#2** — Cooldown(2.0s) child ticked at t=0, re-ticked at t=2.5
   - **Expected:** Child ticked normally (cooldown expired)

### TC-7.3.2.4 Rate Limiter Frequency

| # | Requirement |
|---|-------------|
| 1 | R-7.3.2     |
| 2 | R-7.3.2     |

1. **#1** — RateLimiter(2 Hz), 10 ticks at dt=0.1s over 1.0s
   - **Expected:** Child ticked exactly 2 times
2. **#2** — RateLimiter(10 Hz), 20 ticks at dt=0.1s over 2.0s
   - **Expected:** Child ticked exactly 20 times

### TC-7.3.3.1 Self-Abort

| # | Requirement |
|---|-------------|
| 1 | R-7.3.3     |
| 2 | R-7.3.3     |

1. **#1** — Self-abort condition fails while child is Running
   - **Expected:** Child interrupted; node returns Failure
2. **#2** — Self-abort condition remains true while child Running
   - **Expected:** Child continues Running normally

### TC-7.3.3.2 Lower-Priority Abort

| # | Requirement |
|---|-------------|
| 1 | R-7.3.3     |
| 2 | R-7.3.3     |

1. **#1** — Selector: branch A (priority 0, abort) becomes true while branch B (priority 1) Running
   - **Expected:** Branch B interrupted; branch A starts
2. **#2** — Selector: branch B condition true, branch A abort inactive
   - **Expected:** Branch B continues Running

### TC-7.3.3.3 Abort No State Leak

| # | Requirement |
|---|-------------|
| 1 | R-7.3.3     |
| 2 | R-7.3.3     |

1. **#1** — Abort child that wrote key "temp" to blackboard during execution
   - **Expected:** Key "temp" removed from blackboard after abort
2. **#2** — Abort nested subtree with Running children
   - **Expected:** All descendant node states reset to Idle

### TC-7.3.4.1 Blackboard Self Scope

| # | Requirement |
|---|-------------|
| 1 | R-7.3.4     |
| 2 | R-7.3.4     |

1. **#1** — Agent A sets self-scoped key "hp" = 50; Agent B queries "hp"
   - **Expected:** Agent B returns None (key invisible)
2. **#2** — Agent A sets self-scoped key "hp" = 50; Agent A queries "hp"
   - **Expected:** Returns Some(50)

### TC-7.3.4.2 Blackboard Group Scope

| # | Requirement |
|---|-------------|
| 1 | R-7.3.4     |
| 2 | R-7.3.4     |

1. **#1** — Agent A (group 1) sets group-scoped key "alert" = true; Agent B (group 1) queries
   "alert"
   - **Expected:** Returns Some(true)
2. **#2** — Agent A (group 1) sets group-scoped key "alert" = true; Agent C (group 2) queries
   "alert"
   - **Expected:** Returns None

### TC-7.3.4.3 Blackboard Observer

| # | Requirement |
|---|-------------|
| 1 | R-7.3.4     |
| 2 | R-7.3.4     |

1. **#1** — Register observer on key "target"; set key to 5, then set again to 5
   - **Expected:** Observer fires exactly once (first write only)
2. **#2** — Register observer on key "target"; set key to 5, then to 10
   - **Expected:** Observer fires twice (value changed both times)

### TC-7.3.5.1 BT Serialization Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-7.3.5     |
| 2 | R-7.3.5     |

1. **#1** — Serialize BT with 10 nodes to RON; deserialize back
   - **Expected:** Deserialized tree identical to original (node types, params, structure)
2. **#2** — Serialize BT with 10 nodes to JSON; deserialize back
   - **Expected:** Deserialized tree identical to original

### TC-7.3.6.1 Subtree Circular Reference Detection

| # | Requirement |
|---|-------------|
| 1 | R-7.3.6     |
| 2 | R-7.3.6     |

1. **#1** — BT asset A references subtree B; B references subtree A
   - **Expected:** Error::CircularReference at load time
2. **#2** — BT asset A references subtree B; B references subtree C (no cycle)
   - **Expected:** Load succeeds

### TC-7.3.7.1 Trace Log Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-7.3.7     |
| 2 | R-7.3.7     |

1. **#1** — BT with Sequence[Leaf(Success), Leaf(Failure)]; tick once
   - **Expected:** Trace log contains 3 entries: Sequence, Leaf(Success), Leaf(Failure)
2. **#2** — BT with Selector[Leaf(Failure), Leaf(Success)]; tick once
   - **Expected:** Trace log contains 3 entries: Selector, Leaf(Failure), Leaf(Success)

### TC-7.4.1.1 Linear Curve

| # | Requirement |
|---|-------------|
| 1 | R-7.4.1     |
| 2 | R-7.4.1     |

1. **#1** — Linear curve (slope=1.0, intercept=0.0), input=0.5
   - **Expected:** Output = 0.5
2. **#2** — Linear curve (slope=2.0, intercept=-0.5), input=0.75
   - **Expected:** Output = 1.0 (clamped)

### TC-7.4.1.2 Logistic Curve

| # | Requirement |
|---|-------------|
| 1 | R-7.4.1     |
| 2 | R-7.4.1     |
| 3 | R-7.4.1     |

1. **#1** — Logistic curve (k=10, midpoint=0.5), input=0.5
   - **Expected:** Output = 0.5 (midpoint)
2. **#2** — Logistic curve (k=10, midpoint=0.5), input=1.0
   - **Expected:** Output > 0.99 (saturated)
3. **#3** — Logistic curve (k=10, midpoint=0.5), input=0.0
   - **Expected:** Output < 0.01 (near zero)

### TC-7.4.1.3 Curve Clamping

| # | Requirement |
|---|-------------|
| 1 | R-7.4.1     |
| 2 | R-7.4.1     |
| 3 | R-7.4.1     |

1. **#1** — Linear curve producing raw value 1.5
   - **Expected:** Output clamped to 1.0
2. **#2** — Linear curve producing raw value -0.3
   - **Expected:** Output clamped to 0.0
3. **#3** — Quadratic curve producing raw value 2.0
   - **Expected:** Output clamped to 1.0

### TC-7.4.2.1 Compensation Fairness

| # | Requirement |
|---|-------------|
| 1 | R-7.4.2     |
| 2 | R-7.4.2     |

1. **#1** — Action A: 2 considerations scoring [0.8, 0.8]; Action B: 5 considerations scoring
   [0.8, 0.8, 0.8, 0.8, 0.8]
   - **Expected:** Compensated scores within 10% of each other
2. **#2** — Action A: 1 consideration scoring [0.5]; Action B: 3 considerations scoring
   [0.5, 0.5, 0.5]
   - **Expected:** Compensated scores within 10% of each other

### TC-7.4.2.2 Highest Selection Strategy

| # | Requirement |
|---|-------------|
| 1 | R-7.4.2     |
| 2 | R-7.4.2     |

1. **#1** — Actions with final scores [0.3, 0.9, 0.6, 0.1]
   - **Expected:** Selected action index = 1 (score 0.9)
2. **#2** — Actions with final scores [0.5, 0.5, 0.8]
   - **Expected:** Selected action index = 2 (score 0.8)

### TC-7.4.2.3 Weighted Random Distribution

| # | Requirement |
|---|-------------|
| 1 | R-7.4.2     |
| 2 | R-7.4.2     |

1. **#1** — Actions with scores [0.75, 0.25], 10000 selections
   - **Expected:** Action 0 selected ~75% of the time (within 5%)
2. **#2** — Actions with scores [0.5, 0.5], 10000 selections
   - **Expected:** Each selected ~50% (within 5%)

### TC-7.4.5.1 Context Hysteresis

| # | Requirement |
|---|-------------|
| 1 | R-7.4.5     |
| 2 | R-7.4.5     |

1. **#1** — Context A active (score 0.6); context B score oscillates between 0.58 and 0.62 with
   hysteresis margin 0.1
   - **Expected:** No switch occurs; context A remains active
2. **#2** — Context A active (score 0.6); context B score rises to 0.75 (exceeds hysteresis)
   - **Expected:** Switch to context B

### TC-7.4.4.1 Dual-Axis Category Priority

| # | Requirement |
|---|-------------|
| 1 | R-7.4.4     |
| 2 | R-7.4.4     |

1. **#1** — Survival category action (score 0.3); Social category action (score 0.9)
   - **Expected:** Survival action selected (higher category priority)
2. **#2** — Two Survival category actions (scores 0.4 and 0.8)
   - **Expected:** Score 0.8 action selected (same category, higher score)

### TC-7.4.3.1 Reusable Considerations For Distance LOS Health

| # | Requirement |
|---|-------------|
| 1 | R-7.4.3     |
| 2 | R-7.4.3     |

1. **#1** — Instantiate built-in considerations: Distance, LOS, Health, Threat, Time, Resources
   - **Expected:** All six types return scores in [0,1] for a sample agent + target
2. **#2** — Register a custom consideration implementing the trait interface
   - **Expected:** Custom consideration is callable by utility scorer exactly like built-ins

### TC-7.5.1.1 World State Satisfies

| # | Requirement |
|---|-------------|
| 1 | R-7.5.1     |
| 2 | R-7.5.1     |
| 3 | R-7.5.1     |

1. **#1** — State: bits=[has_weapon=true, has_ammo=true]; Goal: bits=[has_weapon=true]
   - **Expected:** satisfies() = true
2. **#2** — State: bits=[has_weapon=false]; Goal: bits=[has_weapon=true]
   - **Expected:** satisfies() = false
3. **#3** — State: ints=[health=80]; Goal: ints=[health>=50]
   - **Expected:** satisfies() = true

### TC-7.5.1.2 World State Apply

| # | Requirement |
|---|-------------|
| 1 | R-7.5.1     |
| 2 | R-7.5.1     |

1. **#1** — State: bits=[has_weapon=false]; Effect: set has_weapon=true
   - **Expected:** Result: bits=[has_weapon=true]
2. **#2** — State: ints=[ammo=10]; Effect: ammo += 5
   - **Expected:** Result: ints=[ammo=15]

### TC-7.5.1.3 World State Heuristic

| # | Requirement |
|---|-------------|
| 1 | R-7.5.1     |
| 2 | R-7.5.1     |

1. **#1** — State: [false, true, false]; Goal: [true, true, true]
   - **Expected:** Heuristic = 2 (two unsatisfied)
2. **#2** — State: [true, true, true]; Goal: [true, true, true]
   - **Expected:** Heuristic = 0

### TC-7.5.2.1 Planner Finds Optimal Path

| # | Requirement |
|---|-------------|
| 1 | R-7.5.2     |
| 2 | R-7.5.2     |

1. **#1** — Goal: has_weapon=true; Actions: [FindWeapon(cost=2), StealWeapon(cost=5)]
   - **Expected:** Plan = [FindWeapon]; total_cost = 2
2. **#2** — Goal: enemy_dead=true; Actions: [GetWeapon(cost=1), Attack(cost=2, requires has_weapon)]
   - **Expected:** Plan = [GetWeapon, Attack]; total_cost = 3

### TC-7.5.2.2 Planner Unsolvable Goal

| # | Requirement |
|---|-------------|
| 1 | R-7.5.2     |
| 2 | R-7.5.2     |

1. **#1** — Goal: fly=true; Actions: [Walk, Run] (none produce fly=true)
   - **Expected:** Plan = None (no panic)
2. **#2** — Goal: escape=true; Actions: [OpenDoor(requires has_key)] with no GetKey action
   - **Expected:** Plan = None

### TC-7.5.3.1 Preconditions Gate

| # | Requirement |
|---|-------------|
| 1 | R-7.5.3     |
| 2 | R-7.5.3     |

1. **#1** — Action Attack requires has_weapon=true; state has_weapon=false
   - **Expected:** Attack never appears in plan
2. **#2** — Action Heal requires has_potion=true; state has_potion=true
   - **Expected:** Heal may appear in plan

### TC-7.5.3.2 Plan Cost Sum

| # | Requirement |
|---|-------------|
| 1 | R-7.5.3     |
| 2 | R-7.5.3     |

1. **#1** — Plan = [GetWeapon(cost=1), Move(cost=3), Attack(cost=2)]
   - **Expected:** total_cost = 6
2. **#2** — Plan = [Heal(cost=5)]
   - **Expected:** total_cost = 5

### TC-7.5.4.1 Cache Hit Identical

| # | Requirement |
|---|-------------|
| 1 | R-7.5.4     |
| 2 | R-7.5.4     |

1. **#1** — Plan for (goal=G, state_hash=H); query again with same (G, H)
   - **Expected:** Cache hit; returned plan identical to first
2. **#2** — Plan for (goal=G, state_hash=H1); query with (G, H2) where H2 != H1
   - **Expected:** Cache miss; fresh A* search

### TC-7.5.4.2 Cache Invalidation

| # | Requirement |
|---|-------------|
| 1 | R-7.5.4     |
| 2 | R-7.5.4     |

1. **#1** — Populate cache; increment ActionRegistry version
   - **Expected:** All cache entries cleared
2. **#2** — Populate cache; no version change; query same key
   - **Expected:** Cache hit returned

### TC-7.5.5.1 Replan on Precondition Failure

| # | Requirement |
|---|-------------|
| 1 | R-7.5.5     |
| 2 | R-7.5.5     |

1. **#1** — Plan step requires has_ammo=true; state changes to has_ammo=false
   - **Expected:** Replan triggered
2. **#2** — Plan step requires has_ammo=true; state has_ammo=true
   - **Expected:** No replan; step proceeds

### TC-7.5.5.2 Replan Cooldown

| # | Requirement |
|---|-------------|
| 1 | R-7.5.5     |
| 2 | R-7.5.5     |

1. **#1** — Replan triggered at t=0, cooldown=1.0s; precondition fails at t=0.5
   - **Expected:** Replan suppressed until t=1.0
2. **#2** — Replan triggered at t=0, cooldown=1.0s; precondition fails at t=1.5
   - **Expected:** Replan proceeds (cooldown expired)

### TC-7.5.6.1 Goal Priority Ordering

| # | Requirement |
|---|-------------|
| 1 | R-7.5.6     |
| 2 | R-7.5.6     |

1. **#1** — Goals: [Survive(priority=10, unsatisfied), Eat(priority=3, unsatisfied)]
   - **Expected:** Planner plans for Survive first
2. **#2** — Goals: [Survive(priority=10, satisfied), Eat(priority=3, unsatisfied)]
   - **Expected:** Planner plans for Eat (highest unsatisfied)

### TC-7.5.6.2 Goal Satisfaction Stops Replan

| # | Requirement |
|---|-------------|
| 1 | R-7.5.6     |
| 2 | R-7.5.6     |

1. **#1** — Goal Survive satisfied; world state unchanged
   - **Expected:** No replan triggered for Survive
2. **#2** — Goal Survive becomes unsatisfied due to state change
   - **Expected:** Replan triggered for Survive

## Integration Tests

### TC-7.3.5.I1 BT Hot Reload Safety

| # | Requirement |
|---|-------------|
| 1 | R-7.3.5     |
| 2 | R-7.3.5     |

1. **#1** — Agent mid-tick on BT node 5; hot-reload replaces BT asset
   - **Expected:** No crash; agent restarts from root on next tick
2. **#2** — 50 agents running same BT; hot-reload triggers
   - **Expected:** All 50 agents observe updated tree on next tick

### TC-7.3.6.I1 Subtree Propagation

| # | Requirement |
|---|-------------|
| 1 | R-7.3.6     |
| 2 | R-7.3.6     |

1. **#1** — Shared subtree S referenced by trees T1 and T2; modify S
   - **Expected:** Both T1 and T2 use updated subtree on next tick
2. **#2** — Shared subtree S hot-reloaded while agent in T1 is inside S
   - **Expected:** Agent completes current tick; uses new S next tick

### TC-7.3.1.I1 1000 Agents BT Tick

| # | Requirement |
|---|-------------|
| 1 | R-7.3.1     |
| 2 | R-7.3.1     |

1. **#1** — 1000 agents with 20-node BT, parallel tick on thread pool
   - **Expected:** All agents tick; total time < 2 ms
2. **#2** — 1000 agents with varying BT depths (5-50 nodes)
   - **Expected:** All agents produce valid status (no NaN, no panic)

### TC-7.5.4.I1 GOAP Cache Deduplication

| # | Requirement |
|---|-------------|
| 1 | R-7.5.4     |
| 2 | R-7.5.4     |

1. **#1** — 10 agents request plan for identical (goal, state_hash)
   - **Expected:** Exactly 1 A* search performed; 10 cache hits
2. **#2** — 10 agents request plans for 5 distinct (goal, state_hash) pairs
   - **Expected:** Exactly 5 A* searches performed

### TC-7.4.2.I1 Utility Mobile Limits

| # | Requirement |
|---|-------------|
| 1 | R-7.4.2     |
| 2 | R-7.4.2     |

1. **#1** — Mobile platform config; register 16 actions
   - **Expected:** Only first 8 actions evaluated; rest skipped
2. **#2** — Desktop platform config; register 16 actions
   - **Expected:** All 16 actions evaluated

### TC-7.3.1.I2 Budget Carry Over

| # | Requirement |
|---|-------------|
| 1 | R-7.3.1     |
| 2 | R-7.3.1     |

1. **#1** — Budget exhausted mid-frame; agents 50-100 deferred
   - **Expected:** Agents 50-100 ticked first in next frame
2. **#2** — Budget sufficient for all agents
   - **Expected:** No deferral; all agents tick same frame

### TC-7.3.1.I3 Parallel Agent Evaluation

| # | Requirement |
|---|-------------|
| 1 | R-7.3.1     |
| 2 | R-7.3.1     |

1. **#1** — 500 agents evaluated in parallel (4 threads)
   - **Expected:** Results identical to serial evaluation of same agents
2. **#2** — 500 agents with shared group blackboard, parallel eval
   - **Expected:** No data races (ThreadSanitizer clean)

### TC-7.3.1.I4 Game Designer Authors Behavior Tree

| # | Requirement |
|---|-------------|
| 1 | US-7.3.1    |
| 2 | US-7.3.1    |

1. **#1** — Game designer assembles a 20-node behavior tree from sequences, selectors, and leaves
   - **Expected:** Tree ticks cleanly, transitions between states respect sequence/selector
     semantics
2. **#2** — Designer enables parallel composite
   - **Expected:** Children evaluated concurrently, requireAll/requireOne semantics respected

### TC-7.3.4.I1 Game Designer Uses Blackboard With Scopes

| # | Requirement |
|---|-------------|
| 1 | US-7.3.4    |
| 2 | US-7.3.4    |

1. **#1** — Game designer reads/writes blackboard keys in self scope and group scope from BT nodes
   - **Expected:** Self writes visible only to owning entity; group writes visible to all members
2. **#2** — Register observer on key `"target"`
   - **Expected:** Observer fires when key mutates, not when unchanged

### TC-7.3.7.I1 Engine Tester Traces Behavior Tree Execution

| # | Requirement |
|---|-------------|
| 1 | US-7.3.7    |
| 2 | US-7.3.7    |

1. **#1** — Engine tester enables BT trace log on failing agent for 60 frames
   - **Expected:** Trace records every tick with node name, status, and timestamp
2. **#2** — Inspect log
   - **Expected:** Node transitions match expected BT execution path

### TC-7.4.1.I1 Game Designer Uses Utility Curves For Action Scoring

| # | Requirement |
|---|-------------|
| 1 | US-7.4.1    |
| 2 | US-7.4.1    |

1. **#1** — Game designer assigns logistic response curve to "Attack" consideration and linear curve
   to "Flee"
   - **Expected:** Agent scores reflect curve shapes; high-threat triggers logistic saturation
2. **#2** — Clamp consideration input outside [0,1]
   - **Expected:** Curve output stays in [0,1]

### TC-7.4.2.I1 Game Designer Selects Utility Action Strategy

| # | Requirement |
|---|-------------|
| 1 | US-7.4.2    |
| 2 | US-7.4.2    |

1. **#1** — Game designer configures 8 actions and selects "highest score" strategy
   - **Expected:** Highest-scoring action picked every tick
2. **#2** — Switch to "weighted random" strategy, run 10000 ticks
   - **Expected:** Selection distribution matches weights within 5 % statistical tolerance

### TC-7.5.1.I1 Game Designer Defines GOAP World State

| # | Requirement |
|---|-------------|
| 1 | US-7.5.1    |
| 2 | US-7.5.1    |

1. **#1** — Game designer declares world state facts for hunger, ammo, threat
   - **Expected:** Facts stored, planner reads current state correctly
2. **#2** — Modify fact and call `satisfies(goal)`
   - **Expected:** Returns true/false matching hand-verified comparison

### TC-7.5.2.I1 Game Designer Uses GOAP Planner To Achieve Goal

| # | Requirement |
|---|-------------|
| 1 | US-7.5.2    |
| 2 | US-7.5.2    |

1. **#1** — Game designer defines 10 actions and goal "Survive with ammo > 0"
   - **Expected:** Planner returns a sequence of actions whose application yields goal state
2. **#2** — Set unreachable goal
   - **Expected:** Planner returns NoPlan cleanly within time budget

### TC-7.5.4.I1 Game Designer Benefits From Plan Cache

| # | Requirement |
|---|-------------|
| 1 | US-7.5.4    |
| 2 | US-7.5.4    |

1. **#1** — Game designer runs 100 agents with identical world state and goals
   - **Expected:** Planner hits plan cache on repeat queries, total planning time within budget
2. **#2** — Mutate one fact affecting the plan
   - **Expected:** Cache entry invalidates; next request triggers fresh plan

## Benchmarks

### TC-7.3.1.B1 BT Tick Per Agent

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 20-node BT, single agent tick | Wall time | < 5 us | R-7.3.1 |
| 2 | 50-node BT, single agent tick | Wall time | < 15 us | R-7.3.1 |

### TC-7.4.1.B1 Utility Score Per Action

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 3-consideration action, score evaluation | Wall time | < 1 us | R-7.4.1 |
| 2 | 6-consideration action, score evaluation | Wall time | < 2 us | R-7.4.1 |

### TC-7.4.2.B1 Utility Selection (32 Actions)

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 32 actions, Highest strategy | Wall time | < 10 us | R-7.4.2 |
| 2 | 32 actions, WeightedRandom strategy | Wall time | < 10 us | R-7.4.2 |

### TC-7.5.2.B1 GOAP Planning

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8-depth plan, 16 available actions | Wall time | < 50 us | R-7.5.2 |
| 2 | 4-depth plan, 8 available actions | Wall time | < 20 us | R-7.5.2 |

### TC-7.5.1.B1 WorldState Copy and Compare

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Copy 128-bit state + compare | Wall time | < 50 ns | R-7.5.1 |

### TC-7.3.4.B1 Blackboard Get/Set

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Set and get typed value | Wall time | < 100 ns | R-7.3.4 |

### TC-7.5.4.B1 Plan Cache Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Cache with 64 entries, single lookup | Wall time | < 200 ns | R-7.5.4 |

### TC-7.3.1.B2 1000 Agents Full AI Tick

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 agents, mixed BT/Utility/GOAP, parallel | Wall time | < 2 ms | R-7.3.1 |
