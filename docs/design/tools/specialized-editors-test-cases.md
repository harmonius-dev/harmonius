# Specialized Editors Test Cases

Companion test cases for [specialized-editors.md](specialized-editors.md).

## Unit Tests

### Graph Editor Widget Framework

#### TC-15.SE.1 Graph Add Node

| # | Requirement |
|---|-------------|
| 1 | R-15.8.7    |
| 2 | R-15.8.7    |

1. **#1** — `add_node(BlendNode, [100, 200])`
   - **Expected:** Returns valid `NodeId`, node at position `[100, 200]`
2. **#2** — `add_node(Output, [300, 300])`
   - **Expected:** Returns valid `NodeId`, node has no output pins

#### TC-15.SE.2 Graph Remove Node

| # | Requirement |
|---|-------------|
| 1 | R-15.8.7    |

1. **#1** — Add nodes A, B with edge, `remove_node(A)`
   - **Expected:** A removed, edge removed, B still exists

#### TC-15.SE.3 Graph Connect Pins

| # | Requirement |
|---|-------------|
| 1 | R-15.8.2    |
| 2 | R-15.8.2    |

1. **#1** — Connect pose output to pose input
   - **Expected:** `Ok(EdgeId)`
2. **#2** — Connect float output to pose input
   - **Expected:** `Err(PinTypeError::Incompatible)`

#### TC-15.SE.4 Graph Disconnect Edge

| # | Requirement |
|---|-------------|
| 1 | R-15.8.7    |

1. **#1** — Connect A to B, `disconnect(edge_id)`
   - **Expected:** Edge removed, pins marked unconnected

#### TC-15.SE.5 Graph Validate

| # | Requirement |
|---|-------------|
| 1 | R-15.8.3    |
| 2 | R-15.8.3    |

1. **#1** — Valid graph with all inputs connected
   - **Expected:** `validate()` returns empty `Vec`
2. **#2** — Graph with disconnected input pin
   - **Expected:** Returns `ValidationError::DisconnectedInput`

#### TC-15.SE.6 Graph Copy Paste

| # | Requirement |
|---|-------------|
| 1 | R-15.8.7    |

1. **#1** — Select 3 nodes, `copy_selection()`, `paste([50, 50])`
   - **Expected:** 3 new nodes at offset, internal edges preserved

#### TC-15.SE.7 Graph Pan and Zoom

| # | Requirement |
|---|-------------|
| 1 | R-15.8.7    |
| 2 | R-15.8.7    |
| 3 | R-15.8.7    |

1. **#1** — `set_pan([100, 50])`
   - **Expected:** `pan_offset == [100, 50]`
2. **#2** — `set_zoom(2.0)`
   - **Expected:** `zoom_level == 2.0`
3. **#3** — `frame_all()` with 10 nodes
   - **Expected:** All nodes visible in viewport

#### TC-15.SE.8 Graph CRDT Ops Generated

| # | Requirement |
|---|-------------|
| 1 | R-15.12.8   |
| 2 | R-15.12.8   |

1. **#1** — `add_node()`, `pending_crdt_ops()`
   - **Expected:** Returns 1 `GraphOp::AddNode` operation
2. **#2** — `connect()`, `pending_crdt_ops()`
   - **Expected:** Returns 1 `GraphOp::Connect` operation

### Table Editor Widget Framework

#### TC-15.SE.9 Table Add Row

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — `add_row()`
   - **Expected:** Returns valid `RowId`, `row_count()` incremented by 1

#### TC-15.SE.10 Table Remove Row

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Add row, `remove_row(id)`
   - **Expected:** `row_count()` decremented, row absent

#### TC-15.SE.11 Table Set Cell with Validation

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |
| 2 | R-15.15.1   |

1. **#1** — Set float cell to `3.14`
   - **Expected:** `Ok(())`, `get_cell() == Float(3.14)`
2. **#2** — Set integer cell to negative when validator rejects
   - **Expected:** `Err(validation_message)`

#### TC-15.SE.12 Table Sort By Column

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |
| 2 | R-15.15.1   |

1. **#1** — 3 rows with weights `[5, 1, 3]`, `sort_by(weight, Ascending)`
   - **Expected:** `visible_rows()` in order `[1, 3, 5]`
2. **#2** — `sort_by(weight, Descending)`
   - **Expected:** `visible_rows()` in order `[5, 3, 1]`

#### TC-15.SE.13 Table Filter By Column

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |
| 2 | R-15.15.1   |

1. **#1** — 5 rows, `filter_by(type, TextContains("sword"))`
   - **Expected:** Only rows with "sword" in type column visible
2. **#2** — `clear_filters()`
   - **Expected:** All 5 rows visible

#### TC-15.SE.14 Table Copy Paste Rows

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Select 2 rows, `copy_selection()`, `paste(0)`
   - **Expected:** 2 new rows at top, values match copied

#### TC-15.SE.15 Table CRDT Ops Generated

| # | Requirement |
|---|-------------|
| 1 | R-15.12.8   |

1. **#1** — `set_cell()`, `pending_crdt_ops()`
   - **Expected:** Returns 1 `MapOp::Set` operation

### Animation Graph Editor

#### TC-15.SE.16 Anim Graph Add Blend Node

| # | Requirement |
|---|-------------|
| 1 | R-15.8.7    |

1. **#1** — `add_anim_node(BlendNode { Linear }, [100, 100])`
   - **Expected:** Node has 2 pose inputs, 1 pose output, 1 weight input

#### TC-15.SE.17 Anim Graph Add IK Node

| # | Requirement |
|---|-------------|
| 1 | R-15.8.7    |

1. **#1** — `add_anim_node(IkSolver { chain_length: 3 }, pos)`
   - **Expected:** Node has pose input, target input, pose output

### Behavior Tree Editor

#### TC-15.SE.18 BT Add Sequence Node

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |

1. **#1** — `add_bt_node(Sequence, Some(root))`
   - **Expected:** Node appears as child of root in tree layout

#### TC-15.SE.19 BT Debug Overlay

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |

1. **#1** — `attach_debug(entity)`, tick returns `Success` on node 3
   - **Expected:** `debug_state.active_node == 3`, `node_results[3] == Success`

#### TC-15.SE.20 BT Decorator Node

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |

1. **#1** — `add_bt_node(Decorator { Inverter }, parent)`
   - **Expected:** Node inverts child result in preview

### State Machine Editor

#### TC-15.SE.21 SM Add State and Transition

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |

1. **#1** — `add_state("Idle", pos)`, `add_state("Run", pos)`,
   `add_transition(idle, run, conditions)`
   - **Expected:** Two state nodes, one directed edge

#### TC-15.SE.22 SM Set Default State

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |

1. **#1** — `set_default_state(idle)`
   - **Expected:** Idle node has entry arrow indicator

#### TC-15.SE.23 SM Debug Active State

| # | Requirement |
|---|-------------|
| 1 | R-15.13.2   |

1. **#1** — `attach_debug(entity)` where entity is in "Run" state
   - **Expected:** `debug_state.active_state == run_id`

### Quest Editor

#### TC-15.SE.24 Quest Add Objective Node

| # | Requirement |
|---|-------------|
| 1 | R-15.14.1   |

1. **#1** — `add_quest_node(Objective { "Collect 5 gems", false }, pos)`
   - **Expected:** Node renders with objective description

#### TC-15.SE.25 Quest Validate No Start

| # | Requirement |
|---|-------------|
| 1 | R-15.14.1   |

1. **#1** — Graph with objectives but no `QuestStart`
   - **Expected:** `validate_quest()` returns `NoStartNode`

#### TC-15.SE.26 Quest Validate Unreachable

| # | Requirement |
|---|-------------|
| 1 | R-15.14.1   |

1. **#1** — Objective node with no incoming edges from start
   - **Expected:** Returns `UnreachableObjective { node }`

### Loot Table Editor

#### TC-15.SE.27 Loot Simulation Distribution

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — 2 items: weight 1 and weight 3, `run_simulation(10000)`
   - **Expected:** Item B ~75% drops, item A ~25% drops (within 5% tolerance)

#### TC-15.SE.28 Loot Simulation Zero Weight

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Item with weight 0, `run_simulation(1000)`
   - **Expected:** Item never appears in results

### Ability Ledger

#### TC-15.SE.29 Ability Filter By Tag

| # | Requirement |
|---|-------------|
| 1 | R-15.15.2   |

1. **#1** — 5 abilities, 2 tagged "fire", `filter_by_tag("fire")`
   - **Expected:** 2 rows visible

#### TC-15.SE.30 Ability Compare Stats

| # | Requirement |
|---|-------------|
| 1 | R-15.15.2   |

1. **#1** — `compare(fireball, ice_bolt)`
   - **Expected:** Returns `StatDelta` for damage, cooldown, cost

### Equipment Stat Table

#### TC-15.SE.31 Equipment Comparison Totals

| # | Requirement |
|---|-------------|
| 1 | R-15.15.3   |

1. **#1** — Set A: sword(atk:10) + shield(def:5), `compute_totals()`
   - **Expected:** `{"attack": 10.0, "defense": 5.0}`

#### TC-15.SE.32 Equipment Set Comparison

| # | Requirement |
|---|-------------|
| 1 | R-15.15.3   |

1. **#1** — `set_comparison(set_a, set_b)`
   - **Expected:** `stat_deltas` contains difference per stat

### Price Ledger

#### TC-15.SE.33 Inflation Simulation

| # | Requirement |
|---|-------------|
| 1 | R-15.15.4   |

1. **#1** — Buy price 100, `run_inflation_sim(0.10, 5)`
   - **Expected:** Projected buy ~161.05 (100 * 1.1^5)

#### TC-15.SE.34 Inflation Zero Rate

| # | Requirement |
|---|-------------|
| 1 | R-15.15.4   |

1. **#1** — `run_inflation_sim(0.0, 10)`
   - **Expected:** Projected prices equal original prices

### Entity Editor

#### TC-15.SE.35 Entity Add Component

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |

1. **#1** — `add_component(HealthComponent)`
   - **Expected:** Component appears in panel list, undo available

#### TC-15.SE.36 Entity Remove Component

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |

1. **#1** — `remove_component(HealthComponent)`
   - **Expected:** Component removed from panels, undo restores it

#### TC-15.SE.37 Entity Hierarchy Reparent

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |

1. **#1** — Entity C child of A, `reparent(C, B)`
   - **Expected:** C now child of B, undo moves back to A

#### TC-15.SE.38 Entity Prefab Override Revert

| # | Requirement |
|---|-------------|
| 1 | R-15.2.1    |

1. **#1** — Prefab hp=100, instance hp=200, `revert_override("hp")`
   - **Expected:** Instance hp=100, override removed

#### TC-15.SE.39 Entity Inspector Custom Editor Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |

1. **#1** — Plugin registers widget for `PhysicsBody`, select entity with `PhysicsBody`
   - **Expected:** Inspector renders custom widget, not auto-generated

#### TC-15.SE.40 Entity Hierarchy Search

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |

1. **#1** — 100 entities, 3 named "Light*", `search("Light")`
   - **Expected:** Returns 3 matching entities

#### TC-15.SE.41 Prefab Apply to Source

| # | Requirement |
|---|-------------|
| 1 | R-15.2.1    |

1. **#1** — Instance with hp override, `apply_to_prefab()`
   - **Expected:** Prefab asset updated, all instances reflect new value

#### TC-15.SE.42 Prefab Revert All

| # | Requirement |
|---|-------------|
| 1 | R-15.2.1    |

1. **#1** — Instance with 3 overrides, `revert_all()`
   - **Expected:** All overrides removed, instance matches prefab

## Integration Tests

### TC-15.SE.50 Graph Edit End-to-End with Undo

| # | Requirement        |
|---|--------------------|
| 1 | R-15.8.7, R-15.1.3 |

1. **#1** — Add 3 nodes, connect, undo, redo
   - **Expected:** Graph matches state after redo

### TC-15.SE.51 Table Edit End-to-End with Undo

| # | Requirement         |
|---|---------------------|
| 1 | R-15.15.1, R-15.1.3 |

1. **#1** — Add row, set cell, undo
   - **Expected:** Row removed after undo

### TC-15.SE.52 Graph CRDT Collaborative Sync

| # | Requirement |
|---|-------------|
| 1 | R-15.12.8   |

1. **#1** — Editor A adds node, Editor B receives CRDT op
   - **Expected:** Both editors show same graph

### TC-15.SE.53 Table CRDT Collaborative Sync

| # | Requirement |
|---|-------------|
| 1 | R-15.12.8   |

1. **#1** — Editor A sets cell, Editor B receives CRDT op
   - **Expected:** Both editors show same cell value

### TC-15.SE.54 Entity Editor Plugin Widget Integration

| # | Requirement       |
|---|-------------------|
| 1 | R-15.1.8, R-1.6.7 |

1. **#1** — Load plugin, select entity, verify custom widget, unload plugin, verify fallback
   - **Expected:** Widget dispatch switches correctly

### TC-15.SE.55 Behavior Tree Debug with Running Agent

| # | Requirement |
|---|-------------|
| 1 | R-15.13.1   |

1. **#1** — Start play mode, attach BT debug to agent, observe node highlighting
   - **Expected:** Active node updates each AI tick

### TC-15.SE.56 Quest Editor Full Validation

| # | Requirement |
|---|-------------|
| 1 | R-15.14.1   |
| 2 | R-15.14.1   |

1. **#1** — Build complete quest graph with branches, validate
   - **Expected:** No validation errors
2. **#2** — Remove start node, validate
   - **Expected:** Returns `NoStartNode`

### TC-15.SE.57 Loot Table Full Authoring Cycle

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Add items, set weights, run simulation, verify distribution
   - **Expected:** Simulation matches expected probability distribution

### TC-15.SE.58 Equipment Comparison Workflow

| # | Requirement |
|---|-------------|
| 1 | R-15.15.3   |

1. **#1** — Create two sets, compare, verify deltas
   - **Expected:** Stat deltas match manual calculation

### TC-15.SE.59 Price Inflation Full Workflow

| # | Requirement |
|---|-------------|
| 1 | R-15.15.4   |

1. **#1** — Set prices, run inflation at 5% for 10 periods
   - **Expected:** Projections match compound interest formula

## Benchmarks

### TC-15.SE.70 Graph with 500 Nodes

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Pan/zoom on 500-node graph | < 16 ms per frame | US-15.8.7.5 |

### TC-15.SE.71 Graph Validate 500 Nodes

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| `validate()` on 500-node graph | < 50 ms | US-15.8.3.3 |

### TC-15.SE.72 Table with 10k Rows

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Sort 10,000 rows | < 100 ms | US-15.15.1.4 |

### TC-15.SE.73 Table Filter 10k Rows

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Text filter on 10,000 rows | < 50 ms | US-15.15.1.4 |

### TC-15.SE.74 Inspector Rebuild 50 Components

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| `rebuild_panels()` with 50 components on entity | < 16 ms | US-15.1.NF1.1 |

### TC-15.SE.75 Loot Simulation 100k Draws

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| `run_simulation(100_000)` with 50 items | < 500 ms | US-15.15.1.6 |
