# AI Behavior Systems Test Cases

Companion test cases for [behavior.md](behavior.md).

## Unit Tests

### TC-7.3.1.1 Sequence Fail-Fast

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sequence with children [Success, Failure, Success] | Failure after 2nd child; 3rd child never ticked | R-7.3.1 |
| 2 | Sequence with children [Success, Success, Success] | Success after all children ticked | R-7.3.1 |

### TC-7.3.1.2 Selector Succeed-Fast

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Selector with children [Failure, Success, Failure] | Success after 2nd child; 3rd child never ticked | R-7.3.1 |
| 2 | Selector with children [Failure, Failure, Failure] | Failure after all children ticked | R-7.3.1 |

### TC-7.3.1.3 Parallel RequireAll

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parallel(RequireAll) with children [Success, Success, Success] | Success | R-7.3.1 |
| 2 | Parallel(RequireAll) with children [Success, Failure, Success] | Failure | R-7.3.1 |

### TC-7.3.1.4 Parallel RequireOne

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parallel(RequireOne) with children [Failure, Success, Failure] | Success | R-7.3.1 |
| 2 | Parallel(RequireOne) with children [Failure, Failure, Failure] | Failure | R-7.3.1 |

### TC-7.3.2.1 Inverter Negation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inverter wrapping child that returns Success | Failure | R-7.3.2 |
| 2 | Inverter wrapping child that returns Failure | Success | R-7.3.2 |
| 3 | Inverter wrapping child that returns Running | Running (no inversion) | R-7.3.2 |

### TC-7.3.2.2 Repeater Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Repeater(count=3) wrapping Success child | Child ticked 3 times; Repeater returns Success | R-7.3.2 |
| 2 | Repeater(count=5) wrapping Success child | Child ticked 5 times; Repeater returns Success | R-7.3.2 |
| 3 | Repeater(count=3) wrapping Failure child | Child ticked 1 time; Repeater returns Failure | R-7.3.2 |

### TC-7.3.2.3 Cooldown Blocks Re-Entry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cooldown(2.0s) child ticked at t=0, re-ticked at t=1.0 | Failure (blocked by cooldown) | R-7.3.2 |
| 2 | Cooldown(2.0s) child ticked at t=0, re-ticked at t=2.5 | Child ticked normally (cooldown expired) | R-7.3.2 |

### TC-7.3.2.4 Rate Limiter Frequency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | RateLimiter(2 Hz), 10 ticks at dt=0.1s over 1.0s | Child ticked exactly 2 times | R-7.3.2 |
| 2 | RateLimiter(10 Hz), 20 ticks at dt=0.1s over 2.0s | Child ticked exactly 20 times | R-7.3.2 |

### TC-7.3.3.1 Self-Abort

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Self-abort condition fails while child is Running | Child interrupted; node returns Failure | R-7.3.3 |
| 2 | Self-abort condition remains true while child Running | Child continues Running normally | R-7.3.3 |

### TC-7.3.3.2 Lower-Priority Abort

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Selector: branch A (priority 0, abort) becomes true while branch B (priority 1) Running | Branch B interrupted; branch A starts | R-7.3.3 |
| 2 | Selector: branch B condition true, branch A abort inactive | Branch B continues Running | R-7.3.3 |

### TC-7.3.3.3 Abort No State Leak

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Abort child that wrote key "temp" to blackboard during execution | Key "temp" removed from blackboard after abort | R-7.3.3 |
| 2 | Abort nested subtree with Running children | All descendant node states reset to Idle | R-7.3.3 |

### TC-7.3.4.1 Blackboard Self Scope

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Agent A sets self-scoped key "hp" = 50; Agent B queries "hp" | Agent B returns None (key invisible) | R-7.3.4 |
| 2 | Agent A sets self-scoped key "hp" = 50; Agent A queries "hp" | Returns Some(50) | R-7.3.4 |

### TC-7.3.4.2 Blackboard Group Scope

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Agent A (group 1) sets group-scoped key "alert" = true; Agent B (group 1) queries "alert" | Returns Some(true) | R-7.3.4 |
| 2 | Agent A (group 1) sets group-scoped key "alert" = true; Agent C (group 2) queries "alert" | Returns None | R-7.3.4 |

### TC-7.3.4.3 Blackboard Observer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register observer on key "target"; set key to 5, then set again to 5 | Observer fires exactly once (first write only) | R-7.3.4 |
| 2 | Register observer on key "target"; set key to 5, then to 10 | Observer fires twice (value changed both times) | R-7.3.4 |

### TC-7.3.5.1 BT Serialization Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serialize BT with 10 nodes to RON; deserialize back | Deserialized tree identical to original (node types, params, structure) | R-7.3.5 |
| 2 | Serialize BT with 10 nodes to JSON; deserialize back | Deserialized tree identical to original | R-7.3.5 |

### TC-7.3.6.1 Subtree Circular Reference Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | BT asset A references subtree B; B references subtree A | Error::CircularReference at load time | R-7.3.6 |
| 2 | BT asset A references subtree B; B references subtree C (no cycle) | Load succeeds | R-7.3.6 |

### TC-7.3.7.1 Trace Log Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | BT with Sequence[Leaf(Success), Leaf(Failure)]; tick once | Trace log contains 3 entries: Sequence, Leaf(Success), Leaf(Failure) | R-7.3.7 |
| 2 | BT with Selector[Leaf(Failure), Leaf(Success)]; tick once | Trace log contains 3 entries: Selector, Leaf(Failure), Leaf(Success) | R-7.3.7 |

### TC-7.4.1.1 Linear Curve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Linear curve (slope=1.0, intercept=0.0), input=0.5 | Output = 0.5 | R-7.4.1 |
| 2 | Linear curve (slope=2.0, intercept=-0.5), input=0.75 | Output = 1.0 (clamped) | R-7.4.1 |

### TC-7.4.1.2 Logistic Curve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Logistic curve (k=10, midpoint=0.5), input=0.5 | Output = 0.5 (midpoint) | R-7.4.1 |
| 2 | Logistic curve (k=10, midpoint=0.5), input=1.0 | Output > 0.99 (saturated) | R-7.4.1 |
| 3 | Logistic curve (k=10, midpoint=0.5), input=0.0 | Output < 0.01 (near zero) | R-7.4.1 |

### TC-7.4.1.3 Curve Clamping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Linear curve producing raw value 1.5 | Output clamped to 1.0 | R-7.4.1 |
| 2 | Linear curve producing raw value -0.3 | Output clamped to 0.0 | R-7.4.1 |
| 3 | Quadratic curve producing raw value 2.0 | Output clamped to 1.0 | R-7.4.1 |

### TC-7.4.2.1 Compensation Fairness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Action A: 2 considerations scoring [0.8, 0.8]; Action B: 5 considerations scoring [0.8, 0.8, 0.8, 0.8, 0.8] | Compensated scores within 10% of each other | R-7.4.2 |
| 2 | Action A: 1 consideration scoring [0.5]; Action B: 3 considerations scoring [0.5, 0.5, 0.5] | Compensated scores within 10% of each other | R-7.4.2 |

### TC-7.4.2.2 Highest Selection Strategy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Actions with final scores [0.3, 0.9, 0.6, 0.1] | Selected action index = 1 (score 0.9) | R-7.4.2 |
| 2 | Actions with final scores [0.5, 0.5, 0.8] | Selected action index = 2 (score 0.8) | R-7.4.2 |

### TC-7.4.2.3 Weighted Random Distribution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Actions with scores [0.75, 0.25], 10000 selections | Action 0 selected ~75% of the time (within 5%) | R-7.4.2 |
| 2 | Actions with scores [0.5, 0.5], 10000 selections | Each selected ~50% (within 5%) | R-7.4.2 |

### TC-7.4.5.1 Context Hysteresis

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Context A active (score 0.6); context B score oscillates between 0.58 and 0.62 with hysteresis margin 0.1 | No switch occurs; context A remains active | R-7.4.5 |
| 2 | Context A active (score 0.6); context B score rises to 0.75 (exceeds hysteresis) | Switch to context B | R-7.4.5 |

### TC-7.4.4.1 Dual-Axis Category Priority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Survival category action (score 0.3); Social category action (score 0.9) | Survival action selected (higher category priority) | R-7.4.4 |
| 2 | Two Survival category actions (scores 0.4 and 0.8) | Score 0.8 action selected (same category, higher score) | R-7.4.4 |

### TC-7.5.1.1 World State Satisfies

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | State: bits=[has_weapon=true, has_ammo=true]; Goal: bits=[has_weapon=true] | satisfies() = true | R-7.5.1 |
| 2 | State: bits=[has_weapon=false]; Goal: bits=[has_weapon=true] | satisfies() = false | R-7.5.1 |
| 3 | State: ints=[health=80]; Goal: ints=[health>=50] | satisfies() = true | R-7.5.1 |

### TC-7.5.1.2 World State Apply

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | State: bits=[has_weapon=false]; Effect: set has_weapon=true | Result: bits=[has_weapon=true] | R-7.5.1 |
| 2 | State: ints=[ammo=10]; Effect: ammo += 5 | Result: ints=[ammo=15] | R-7.5.1 |

### TC-7.5.1.3 World State Heuristic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | State: [false, true, false]; Goal: [true, true, true] | Heuristic = 2 (two unsatisfied) | R-7.5.1 |
| 2 | State: [true, true, true]; Goal: [true, true, true] | Heuristic = 0 | R-7.5.1 |

### TC-7.5.2.1 Planner Finds Optimal Path

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Goal: has_weapon=true; Actions: [FindWeapon(cost=2), StealWeapon(cost=5)] | Plan = [FindWeapon]; total_cost = 2 | R-7.5.2 |
| 2 | Goal: enemy_dead=true; Actions: [GetWeapon(cost=1), Attack(cost=2, requires has_weapon)] | Plan = [GetWeapon, Attack]; total_cost = 3 | R-7.5.2 |

### TC-7.5.2.2 Planner Unsolvable Goal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Goal: fly=true; Actions: [Walk, Run] (none produce fly=true) | Plan = None (no panic) | R-7.5.2 |
| 2 | Goal: escape=true; Actions: [OpenDoor(requires has_key)] with no GetKey action | Plan = None | R-7.5.2 |

### TC-7.5.3.1 Preconditions Gate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Action Attack requires has_weapon=true; state has_weapon=false | Attack never appears in plan | R-7.5.3 |
| 2 | Action Heal requires has_potion=true; state has_potion=true | Heal may appear in plan | R-7.5.3 |

### TC-7.5.3.2 Plan Cost Sum

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plan = [GetWeapon(cost=1), Move(cost=3), Attack(cost=2)] | total_cost = 6 | R-7.5.3 |
| 2 | Plan = [Heal(cost=5)] | total_cost = 5 | R-7.5.3 |

### TC-7.5.4.1 Cache Hit Identical

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plan for (goal=G, state_hash=H); query again with same (G, H) | Cache hit; returned plan identical to first | R-7.5.4 |
| 2 | Plan for (goal=G, state_hash=H1); query with (G, H2) where H2 != H1 | Cache miss; fresh A* search | R-7.5.4 |

### TC-7.5.4.2 Cache Invalidation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Populate cache; increment ActionRegistry version | All cache entries cleared | R-7.5.4 |
| 2 | Populate cache; no version change; query same key | Cache hit returned | R-7.5.4 |

### TC-7.5.5.1 Replan on Precondition Failure

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plan step requires has_ammo=true; state changes to has_ammo=false | Replan triggered | R-7.5.5 |
| 2 | Plan step requires has_ammo=true; state has_ammo=true | No replan; step proceeds | R-7.5.5 |

### TC-7.5.5.2 Replan Cooldown

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Replan triggered at t=0, cooldown=1.0s; precondition fails at t=0.5 | Replan suppressed until t=1.0 | R-7.5.5 |
| 2 | Replan triggered at t=0, cooldown=1.0s; precondition fails at t=1.5 | Replan proceeds (cooldown expired) | R-7.5.5 |

### TC-7.5.6.1 Goal Priority Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Goals: [Survive(priority=10, unsatisfied), Eat(priority=3, unsatisfied)] | Planner plans for Survive first | R-7.5.6 |
| 2 | Goals: [Survive(priority=10, satisfied), Eat(priority=3, unsatisfied)] | Planner plans for Eat (highest unsatisfied) | R-7.5.6 |

### TC-7.5.6.2 Goal Satisfaction Stops Replan

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Goal Survive satisfied; world state unchanged | No replan triggered for Survive | R-7.5.6 |
| 2 | Goal Survive becomes unsatisfied due to state change | Replan triggered for Survive | R-7.5.6 |

## Integration Tests

### TC-7.3.5.I1 BT Hot Reload Safety

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Agent mid-tick on BT node 5; hot-reload replaces BT asset | No crash; agent restarts from root on next tick | R-7.3.5 |
| 2 | 50 agents running same BT; hot-reload triggers | All 50 agents observe updated tree on next tick | R-7.3.5 |

### TC-7.3.6.I1 Subtree Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shared subtree S referenced by trees T1 and T2; modify S | Both T1 and T2 use updated subtree on next tick | R-7.3.6 |
| 2 | Shared subtree S hot-reloaded while agent in T1 is inside S | Agent completes current tick; uses new S next tick | R-7.3.6 |

### TC-7.3.1.I1 1000 Agents BT Tick

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 agents with 20-node BT, parallel tick on thread pool | All agents tick; total time < 2 ms | R-7.3.1 |
| 2 | 1000 agents with varying BT depths (5-50 nodes) | All agents produce valid status (no NaN, no panic) | R-7.3.1 |

### TC-7.5.4.I1 GOAP Cache Deduplication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 agents request plan for identical (goal, state_hash) | Exactly 1 A* search performed; 10 cache hits | R-7.5.4 |
| 2 | 10 agents request plans for 5 distinct (goal, state_hash) pairs | Exactly 5 A* searches performed | R-7.5.4 |

### TC-7.4.2.I1 Utility Mobile Limits

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile platform config; register 16 actions | Only first 8 actions evaluated; rest skipped | R-7.4.2 |
| 2 | Desktop platform config; register 16 actions | All 16 actions evaluated | R-7.4.2 |

### TC-7.3.1.I2 Budget Carry Over

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Budget exhausted mid-frame; agents 50-100 deferred | Agents 50-100 ticked first in next frame | R-7.3.1 |
| 2 | Budget sufficient for all agents | No deferral; all agents tick same frame | R-7.3.1 |

### TC-7.3.1.I3 Parallel Agent Evaluation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 agents evaluated in parallel (4 threads) | Results identical to serial evaluation of same agents | R-7.3.1 |
| 2 | 500 agents with shared group blackboard, parallel eval | No data races (ThreadSanitizer clean) | R-7.3.1 |

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
