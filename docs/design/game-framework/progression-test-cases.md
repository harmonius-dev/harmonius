# Progression and Condition Systems — Test Cases

Companion test cases for [progression.md](progression.md).

## Unit Tests

### TC-13.6.1.1 Graph Validate No Cycles

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |
| 2 | R-13.6.1    |

1. **#1** — DAG with Start->A->B->End
   - **Expected:** `validate() == Ok(())`
2. **#2** — Graph with A->B->C->A cycle
   - **Expected:** `Err(CycleDetected { path: [A, B, C] })`

### TC-13.6.1.2 Graph Validate Orphan Node

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |

1. **#1** — DAG with unreachable node X (no incoming edges from Start)
   - **Expected:** `Err(UnreachableNode { node: X })`

### TC-13.6.1.3 Graph Validate Start End Counts

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |
| 2 | R-13.6.1    |
| 3 | R-13.6.1    |

1. **#1** — Graph with zero Start nodes
   - **Expected:** `Err(MissingStartNode)`
2. **#2** — Graph with two Start nodes
   - **Expected:** `Err(MultipleStartNodes)`
3. **#3** — Graph with zero End nodes
   - **Expected:** `Err(MissingEndNode)`

### TC-13.6.1.4 Graph Successors and Predecessors

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |
| 2 | R-13.6.1    |

1. **#1** — Node A with edges to B and C
   - **Expected:** `successors(A) == [B, C]`
2. **#2** — Node C with edges from A and B
   - **Expected:** `predecessors(C) == [A, B]`

### TC-13.6.1.5 Graph Node Count

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |

1. **#1** — Graph with Start + 3 Objective + End
   - **Expected:** `node_count() == 5`

### TC-13.6.1.6 Graph Node Lookup

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |
| 2 | R-13.6.1    |

1. **#1** — `node(valid_id)`
   - **Expected:** `Some(&ProgressionNode)`
2. **#2** — `node(invalid_id)`
   - **Expected:** `None`

### TC-13.6.1.7 Node Status Valid Transitions

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |
| 2 | F-13.6.1    |
| 3 | F-13.6.1    |
| 4 | F-13.6.1    |

1. **#1** — `transition(node, Locked -> Available)`
   - **Expected:** `Ok(())`
2. **#2** — `transition(node, Available -> Active)`
   - **Expected:** `Ok(())`
3. **#3** — `transition(node, Active -> Completed)`
   - **Expected:** `Ok(())`
4. **#4** — `transition(node, Active -> Failed)`
   - **Expected:** `Ok(())`

### TC-13.6.1.8 Node Status Invalid Transitions

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |
| 2 | F-13.6.1    |
| 3 | F-13.6.1    |

1. **#1** — `transition(node, Locked -> Active)`
   - **Expected:** `Err(InvalidTransition)`
2. **#2** — `transition(node, Completed -> Active)`
   - **Expected:** `Err(InvalidTransition)`
3. **#3** — `transition(node, Locked -> Completed)`
   - **Expected:** `Err(InvalidTransition)`

### TC-13.6.1.9 Node Skip On Branch Alternate

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Branch node completes, edge to B chosen, edge to C not
   - **Expected:** `status(C) == Skipped`

### TC-13.6.1.10 State From Graph Initialization

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — `ProgressionState::from_graph(graph, tick=100)`
   - **Expected:** Start node `Available`, all others `Locked`, `started_at == 100`,
     `completed_at == None`

### TC-13.6.1.11 State Is Complete

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |
| 2 | F-13.6.1    |

1. **#1** — All End nodes `Completed`
   - **Expected:** `is_complete() == true`
2. **#2** — One End node still `Active`
   - **Expected:** `is_complete() == false`

### TC-13.6.1.12 State Nodes With Status

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — 3 nodes `Completed`, 2 nodes `Locked`
   - **Expected:** `nodes_with_status(Completed).len() == 3`

### TC-13.6.2.1 Edge Condition Unconditional

| # | Requirement |
|---|-------------|
| 1 | R-13.6.2    |

1. **#1** — Edge with `condition: None`, source node `Completed`
   - **Expected:** Destination transitions to `Available`

### TC-13.6.2.2 Edge Condition Gated Pass

| # | Requirement |
|---|-------------|
| 1 | R-13.6.2    |

1. **#1** — Edge with `HasTag("quest_complete")`, entity has tag
   - **Expected:** Destination transitions to `Available`

### TC-13.6.2.3 Edge Condition Gated Fail

| # | Requirement |
|---|-------------|
| 1 | R-13.6.2    |

1. **#1** — Edge with `HasTag("quest_complete")`, entity lacks tag
   - **Expected:** Destination remains `Locked`

### TC-13.6.3.1 Journal Active Count

| # | Requirement |
|---|-------------|
| 1 | R-13.6.3    |

1. **#1** — 3 graphs in `active`, 2 in `completed`
   - **Expected:** `active_count() == 3`

### TC-13.6.3.2 Journal Filter By Category

| # | Requirement |
|---|-------------|
| 1 | R-13.6.3    |

1. **#1** — 2 Quest graphs + 1 SkillTree in active
   - **Expected:** `by_category(Quest).len() == 2`

### TC-13.6.3.3 Journal Search

| # | Requirement |
|---|-------------|
| 1 | R-13.6.3    |

1. **#1** — Graphs named "Dragon Slayer", "Dragon Egg", "Goblin"
   - **Expected:** `search("Dragon").len() == 2`

### TC-13.6.4.1 Branch Mutual Exclusivity

| # | Requirement |
|---|-------------|
| 1 | R-13.6.4    |

1. **#1** — Branch node with 2 outgoing edges, both conditions true
   - **Expected:** Only first matching edge activates (by weight); other destination set to
     `Skipped`

### TC-13.6.5.1 Active Graphs Add

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — `add(state)` when below max_active limit
   - **Expected:** `Ok(())`; `active_count()` incremented by 1

### TC-13.6.5.2 Active Graphs Max Limit

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — `add(state)` when at max_active_per_entity = 5
   - **Expected:** `Err(GraphLimitError)`

### TC-13.6.5.3 Active Graphs Remove

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |
| 2 | F-13.6.1    |

1. **#1** — `remove(existing_graph_id)`
   - **Expected:** `true`; `active_count()` decremented
2. **#2** — `remove(nonexistent_graph_id)`
   - **Expected:** `false`

### TC-13.6.6.1 Reward GameplayEffect Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — Node completes with `RewardType::GameplayEffect(id)`
   - **Expected:** `ApplyGameplayEffect` event emitted with correct ID

### TC-13.6.6.2 Reward ItemGrant Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — Node completes with `RewardType::ItemGrant(row)`, qty 3
   - **Expected:** `GrantItem` event with `row_ref == row`, `qty == 3`

### TC-13.6.6.3 Reward CurrencyGrant Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — Node with `CurrencyGrant { currency: gold, amount: 500 }`
   - **Expected:** `GrantCurrency` event, `amount == 500`

### TC-13.6.6.4 Reward ExperienceGrant Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — Node with `ExperienceGrant(1000)`
   - **Expected:** `GrantExperience` event, `amount == 1000`

### TC-13.6.6.5 Reward ReputationGrant Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — Node with `ReputationGrant { faction: elves, delta: 50 }`
   - **Expected:** `GrantReputation` event, `delta == 50`

### TC-13.6.6.6 Reward GraphUnlock Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — Node with `GraphUnlock(next_quest_id)`
   - **Expected:** `GraphStarted` event for `next_quest_id`

### TC-13.6.6.7 Reward Conditional Grant

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |
| 2 | R-13.6.6    |

1. **#1** — `grant_condition = HasTag("premium")`, entity has tag
   - **Expected:** Reward granted; `RewardGranted` event emitted
2. **#2** — Same condition, entity lacks tag
   - **Expected:** Reward skipped; no `RewardGranted` event

### TC-13.6.6.8 Reward CustomEvent Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — Node with `CustomEvent("quest_fanfare")`
   - **Expected:** Custom event `"quest_fanfare"` emitted

### TC-13.12.1.1 Predicate And

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1a  |
| 2 | R-13.12.1a  |

1. **#1** — `And([HasTag("a"), HasTag("b")])`, entity has both
   - **Expected:** `evaluate() == true`
2. **#2** — Same predicate, entity has only tag "a"
   - **Expected:** `evaluate() == false`

### TC-13.12.1.2 Predicate Or

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1a  |
| 2 | R-13.12.1a  |

1. **#1** — `Or([HasTag("a"), HasTag("b")])`, entity has only "b"
   - **Expected:** `evaluate() == true`
2. **#2** — Same predicate, entity has neither
   - **Expected:** `evaluate() == false`

### TC-13.12.1.3 Predicate Not

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1a  |

1. **#1** — `Not(HasTag("dead"))`, entity lacks tag
   - **Expected:** `evaluate() == true`

### TC-13.12.1.4 Predicate Comparison

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1a  |
| 2 | R-13.12.1a  |
| 3 | R-13.12.1a  |

1. **#1** — `Comparison { stat: str, op: GreaterThan, threshold: 10 }`, stat value = 15
   - **Expected:** `evaluate() == true`
2. **#2** — Same predicate, stat value = 5
   - **Expected:** `evaluate() == false`
3. **#3** — `Comparison { stat: str, op: Equal, threshold: 10 }`, stat value = 10
   - **Expected:** `evaluate() == true`

### TC-13.12.1.5 Predicate HasComponent

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1a  |
| 2 | R-13.12.1a  |

1. **#1** — `HasComponent(Health)`, entity has `Health` component
   - **Expected:** `evaluate() == true`
2. **#2** — Same predicate, entity lacks `Health`
   - **Expected:** `evaluate() == false`

### TC-13.12.1.6 Predicate MeterThreshold

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2a  |
| 2 | R-13.12.2a  |

1. **#1** — `MeterThreshold { meter: xp, op: >=, value: 1000 }`, meter = 1500
   - **Expected:** `evaluate() == true`
2. **#2** — Same predicate, meter = 500
   - **Expected:** `evaluate() == false`

### TC-13.12.1.7 Predicate EventOccurred

| # | Requirement |
|---|-------------|
| 1 | R-13.12.6a  |
| 2 | R-13.12.6a  |

1. **#1** — `EventOccurred { event: "boss_kill", min_count: 1 }`, count = 3
   - **Expected:** `evaluate() == true`
2. **#2** — Same predicate, count = 0
   - **Expected:** `evaluate() == false`

### TC-13.12.1.8 Predicate CountReached

| # | Requirement |
|---|-------------|
| 1 | R-13.12.6a  |

1. **#1** — `CountReached { counter: "kills", target: 100 }`, counter = 100
   - **Expected:** `evaluate() == true`

### TC-13.12.1.9 Predicate TimeElapsed

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1a  |

1. **#1** — `TimeElapsed { ref: ObserverCreation, duration: 600 }`, elapsed = 700 ticks
   - **Expected:** `evaluate() == true`

### TC-13.12.1.10 Predicate GraphNodeStatus

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1a  |
| 2 | R-13.12.1a  |

1. **#1** — `GraphNodeStatus { graph, node, expected: Completed }`, node is `Completed`
   - **Expected:** `evaluate() == true`
2. **#2** — Same predicate, node is `Active`
   - **Expected:** `evaluate() == false`

### TC-13.12.1.11 Predicate Nested Composition

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1a  |

1. **#1** — `And([Or([HasTag("a"), HasTag("b")]), Not(HasTag("c"))])` with entity having tags {a, d}
   - **Expected:** `evaluate() == true`

### TC-13.12.2.1 Compiled Predicate Bytecode Eval

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2a  |

1. **#1** — Compile `And([HasTag("a"), HasTag("b")])` to bytecode, evaluate with entity having both
   tags
   - **Expected:** `evaluate() == true`; result matches interpreted evaluation

### TC-13.12.2.2 Compiled Predicate Leaf Count

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2a  |

1. **#1** — Compile `And([HasTag("a"), Or([HasTag("b"), HasTag("c")])])`
   - **Expected:** `leaf_count() == 3`

### TC-13.12.3.1 Observer OneShot Mode

| # | Requirement |
|---|-------------|
| 1 | R-13.12.6a  |

1. **#1** — OneShot observer, predicate becomes true twice
   - **Expected:** Exactly 1 `ObserverTrigger` emitted; observer removed after first fire

### TC-13.12.3.2 Observer Persistent Mode

| # | Requirement |
|---|-------------|
| 1 | R-13.12.5   |

1. **#1** — Persistent observer, predicate toggles false->true->false->true
   - **Expected:** 2 `ObserverTrigger` events (one per false->true edge)

### TC-13.12.3.3 Observer CountLimited Mode

| # | Requirement |
|---|-------------|
| 1 | R-13.12.6a  |

1. **#1** — `CountLimited(3)` observer, predicate becomes true 5 times
   - **Expected:** Exactly 3 `ObserverTrigger` events; observer removed after 3rd fire

### TC-13.12.3.4 Observer Edge Detection False To True

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1a  |

1. **#1** — Predicate true on first eval (was_true = false)
   - **Expected:** `ObserverTrigger` emitted; `was_true` set to true

### TC-13.12.3.5 Observer No Trigger On True To True

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1a  |

1. **#1** — Predicate remains true across two evals
   - **Expected:** Only 1 trigger total (from initial false->true)

### TC-13.12.3.6 Observer Reset On True To False

| # | Requirement |
|---|-------------|
| 1 | R-13.12.5   |

1. **#1** — Predicate goes true then false
   - **Expected:** `was_true` reset to false; no extra trigger

### TC-13.12.4.1 Trigger Action AdvanceNode

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2b  |

1. **#1** — `TriggerAction::AdvanceNode { graph, node }` dispatched
   - **Expected:** Target node transitions from `Available` to `Active`

### TC-13.12.4.2 Trigger Action CompleteNode

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2b  |

1. **#1** — `TriggerAction::CompleteNode { graph, node }` dispatched
   - **Expected:** Target node transitions to `Completed`; `NodeCompleted` event emitted

### TC-13.12.4.3 Trigger Action StartGraph

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1c  |

1. **#1** — `TriggerAction::StartGraph(graph_id)` dispatched
   - **Expected:** New `ProgressionState` added to entity's `ActiveGraphs`; `GraphStarted` event
     emitted

### TC-13.12.4.4 Trigger Action GrantReward

| # | Requirement |
|---|-------------|
| 1 | R-13.12.6b  |

1. **#1** — `TriggerAction::GrantReward(entry)` dispatched
   - **Expected:** `RewardGranted` event emitted with matching reward type and quantity

### TC-13.12.4.5 Trigger Action EmitEvent

| # | Requirement |
|---|-------------|
| 1 | R-13.12.5   |

1. **#1** — `TriggerAction::EmitEvent("faction_tier_up")` dispatched
   - **Expected:** Custom event `"faction_tier_up"` emitted

### TC-13.12.5.1 Graph Category Accessor

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Graph with `category: Quest`
   - **Expected:** `category() == Quest`

### TC-13.12.5.2 Graph Repeatable Flag

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |
| 2 | F-13.6.1    |

1. **#1** — Graph with `repeatable: true`
   - **Expected:** `is_repeatable() == true`
2. **#2** — Graph with `repeatable: false`
   - **Expected:** `is_repeatable() == false`

### TC-13.12.5.3 Graph Cooldown Enforcement

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Repeatable graph with `cooldown_ticks: 100`, re-start at tick 50 after completion
   - **Expected:** `Err(CooldownActive { remaining: 50 })`

### TC-13.12.6.1 Reward Node Auto Complete

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Reward node transitions to `Available`
   - **Expected:** Auto-transitions to `Active` then `Completed`; rewards dispatched

### TC-13.12.6.2 Merge Node Any Incoming

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Merge node with 3 incoming edges, 1 source `Completed`
   - **Expected:** Merge node transitions to `Available`

### TC-13.12.6.3 Checkpoint Node Save

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Checkpoint node completes
   - **Expected:** `save_progression_system` triggers for owning entity

### TC-13.12.7.1 State Serialization Round Trip

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Serialize `ProgressionState` with 10 nodes in mixed statuses, deserialize
   - **Expected:** All node statuses, `current_node`, `started_at`, `completed_at` match original

### TC-13.12.7.2 Active Graphs Serialization Round Trip

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Serialize `ActiveGraphs` with 3 graphs, deserialize
   - **Expected:** `active_count() == 3`; all graph states match

### TC-13.12.7.3 Observer Serialization Round Trip

| # | Requirement |
|---|-------------|
| 1 | R-13.12.6a  |

1. **#1** — Serialize `ConditionObserver` with nested predicate, deserialize
   - **Expected:** `predicate`, `mode`, `fire_count`, `was_true` all match original

### TC-13.6.7.1 Progression Events Emitted

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |
| 2 | F-13.6.1    |
| 3 | F-13.6.1    |
| 4 | F-13.6.1    |

1. **#1** — Start a graph
   - **Expected:** Exactly 1 `GraphStarted` event
2. **#2** — Activate a node
   - **Expected:** Exactly 1 `NodeActivated` event
3. **#3** — Complete a node
   - **Expected:** Exactly 1 `NodeCompleted` event
4. **#4** — Complete all End nodes
   - **Expected:** Exactly 1 `GraphCompleted` event

### TC-13.6.7.2 Failed Node Event

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Transition node to `Failed`
   - **Expected:** Exactly 1 `NodeFailed` event with correct entity, graph_id, node_id

### TC-13.6.7.3 Failed Node Retry Repeatable

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Failed node in repeatable graph, retry
   - **Expected:** Node transitions `Failed -> Available`

## Integration Tests

### TC-13.6.1.I1 Quest Full Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |
| 2 | R-13.6.2    |
| 3 | R-13.6.6    |

1. **#1** — Quest graph: Start->Objective(kill 5)->Reward->End. Spawn entity, start graph, simulate
   5 kill events
   - **Expected:** Nodes advance Start->Objective->Reward->End; `RewardGranted` emitted;
     `GraphCompleted` emitted; journal moves graph from active to completed

### TC-13.6.4.I1 Dialogue Branch Traversal

| # | Requirement |
|---|-------------|
| 1 | R-13.6.4    |
| 2 | R-13.6.2    |

1. **#1** — DialogueTree graph with 2 branches gated by `HasTag("high_rep")`. Entity has tag,
   selects response
   - **Expected:** Branch A path activates; Branch B path nodes set to `Skipped`; side-effect
     rewards granted

### TC-13.12.2.I1 Skill Tree Allocate And Respec

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2a  |
| 2 | R-13.12.2b  |

1. **#1** — SkillTree graph with tier-gated talents. Allocate 3 points into tier 1, then allocate
   tier 2 talent
   - **Expected:** Tier 1 nodes `Completed`; tier 2 node `Available` after gate met;
     `GameplayEffect` rewards applied
2. **#2** — Respec: reset all nodes to `Locked`, refund points
   - **Expected:** All nodes `Locked` except Start; talent points restored; gameplay effects removed

### TC-13.23.1.I1 Battle Pass Advance

| # | Requirement |
|---|-------------|
| 1 | R-13.23.1   |
| 2 | R-13.12.6b  |

1. **#1** — Linear BattlePass graph with 5 tiers. Observer watches season XP meter. Grant XP past
   tier 3 threshold
   - **Expected:** Tiers 1-3 `Completed` with rewards; tier 4 `Available`; free rewards granted to
     all, premium rewards only if entity has `HasTag("premium")`

### TC-13.12.6.I1 Achievement OneShot Fire

| # | Requirement |
|---|-------------|
| 1 | R-13.12.6a  |
| 2 | R-13.12.6b  |

1. **#1** — OneShot observer: `CountReached("kills", 100)`. Increment kill counter to 100
   - **Expected:** `ObserverTrigger` fires once; `RewardGranted` emitted; observer entity removed

### TC-13.6.1.I2 Multi Graph Simultaneous

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |
| 2 | R-13.12.2a  |

1. **#1** — Entity with Quest + SkillTree + BattlePass active. Simulate events that advance all 3
   graphs
   - **Expected:** All 3 graphs advance independently; no cross-contamination of node states;
     `active_count() == 3`

### TC-13.12.5.I1 Reputation Tier Transitions

| # | Requirement |
|---|-------------|
| 1 | R-13.12.5   |

1. **#1** — 3 Persistent observers at reputation thresholds 100, 500, 1000. Increase reputation from
   0 to 600, then decrease to 400
   - **Expected:** Triggers at 100 and 500 on increase; trigger at 500 again when crossing back down
     then up (edge detection)

### TC-13.23.2.I1 Daily Challenge Rotation

| # | Requirement |
|---|-------------|
| 1 | R-13.23.2   |

1. **#1** — Spawn 3 OneShot observers for daily challenges. Complete 2, advance time past window
   - **Expected:** 2 `RewardGranted` events for completed; expired observer removed on cleanup

### TC-13.6.7.I1 World Event Zone Phase

| # | Requirement |
|---|-------------|
| 1 | R-13.6.7a   |
| 2 | R-13.6.7b   |

1. **#1** — Zone-entity graph with 3 phases. Observers watch aggregate kill count. Advance through
   all phases
   - **Expected:** Phase nodes advance in order; `NodeActivated` events trigger sub-level streaming;
     `GraphCompleted` fires at final phase

### TC-13.12.1.I1 Class Progression Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1b  |
| 2 | R-13.12.1c  |

1. **#1** — ClassProgression graph: level milestones at 5, 10, 15 with ability unlock rewards. Grant
   XP past level 10
   - **Expected:** Milestone nodes 5 and 10 `Completed`; ability rewards granted; milestone 15 still
     `Locked`
2. **#2** — Multi-class: add second ClassProgression graph
   - **Expected:** `active_count` includes both; both progress independently

### TC-13.12.3.I1 Profession Crafting Gate

| # | Requirement |
|---|-------------|
| 1 | R-13.12.3a  |
| 2 | R-13.12.4   |

1. **#1** — Profession graph with Checkpoint node gating tier 2 recipes behind crafting station
   type. Advance profession XP, attempt tier 2 without station
   - **Expected:** Checkpoint blocks advancement; with correct station, checkpoint completes and
     recipe `GraphUnlock` rewards dispatch

### TC-13.12.7.I1 Save Load Round Trip

| # | Requirement |
|---|-------------|
| 1 | F-13.6.1    |

1. **#1** — Entity with 2 active graphs + 3 observers. Serialize via save system, despawn, reload
   - **Expected:** All graph states restored; all observer states restored with `fire_count` and
     `was_true` intact; progression continues

## Benchmarks

### TC-13.12.NF2.B1 Observer Evaluation Throughput

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | 1,000 observers, each with 5-leaf predicate | Total eval time | < 0.1 ms | R-13.12.NF2 |

### TC-13.12.NF1.B1 Talent Validation Latency

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Talent tree with 50 nodes, validate allocation | Wall-clock latency | < 1 ms | R-13.12.NF1 |

### TC-13.6.NF1.B1 Graph Traversal Latency

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | 50-node DAG, full traversal with edge eval | Wall-clock latency | < 0.5 ms | R-13.6.NF1 |

### TC-13.12.NF2.B2 Simultaneous Graphs Throughput

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | 100 entities, 3 graphs each | Advance time/frame | < 1 ms | R-13.12.NF2 |

### TC-13.12.NF1.B3 Predicate Bytecode Evaluation

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | 1,000 compiled predicates, 10 leaves | Bytecode eval time | < 0.01 ms | R-13.12.NF1 |
