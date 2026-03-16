# Specialized Editors Test Cases

Companion test cases for [specialized-editors.md](specialized-editors.md).

## Unit Tests

### Graph Editor Widget Framework

#### TC-15.SE.1 Graph Add Node

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_node(BlendNode, [100, 200])` | Returns valid `NodeId`, node at position `[100, 200]` | R-15.8.7 |
| 2 | `add_node(Output, [300, 300])` | Returns valid `NodeId`, node has no output pins | R-15.8.7 |

#### TC-15.SE.2 Graph Remove Node

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add nodes A, B with edge, `remove_node(A)` | A removed, edge removed, B still exists | R-15.8.7 |

#### TC-15.SE.3 Graph Connect Pins

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect pose output to pose input | `Ok(EdgeId)` | R-15.8.2 |
| 2 | Connect float output to pose input | `Err(PinTypeError::Incompatible)` | R-15.8.2 |

#### TC-15.SE.4 Graph Disconnect Edge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect A to B, `disconnect(edge_id)` | Edge removed, pins marked unconnected | R-15.8.7 |

#### TC-15.SE.5 Graph Validate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Valid graph with all inputs connected | `validate()` returns empty `Vec` | R-15.8.3 |
| 2 | Graph with disconnected input pin | Returns `ValidationError::DisconnectedInput` | R-15.8.3 |

#### TC-15.SE.6 Graph Copy Paste

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select 3 nodes, `copy_selection()`, `paste([50, 50])` | 3 new nodes at offset, internal edges preserved | R-15.8.7 |

#### TC-15.SE.7 Graph Pan and Zoom

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_pan([100, 50])` | `pan_offset == [100, 50]` | R-15.8.7 |
| 2 | `set_zoom(2.0)` | `zoom_level == 2.0` | R-15.8.7 |
| 3 | `frame_all()` with 10 nodes | All nodes visible in viewport | R-15.8.7 |

#### TC-15.SE.8 Graph CRDT Ops Generated

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_node()`, `pending_crdt_ops()` | Returns 1 `GraphOp::AddNode` operation | R-15.12.8 |
| 2 | `connect()`, `pending_crdt_ops()` | Returns 1 `GraphOp::Connect` operation | R-15.12.8 |

### Table Editor Widget Framework

#### TC-15.SE.9 Table Add Row

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_row()` | Returns valid `RowId`, `row_count()` incremented by 1 | R-15.15.1 |

#### TC-15.SE.10 Table Remove Row

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add row, `remove_row(id)` | `row_count()` decremented, row absent | R-15.15.1 |

#### TC-15.SE.11 Table Set Cell with Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set float cell to `3.14` | `Ok(())`, `get_cell() == Float(3.14)` | R-15.15.1 |
| 2 | Set integer cell to negative when validator rejects | `Err(validation_message)` | R-15.15.1 |

#### TC-15.SE.12 Table Sort By Column

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 rows with weights `[5, 1, 3]`, `sort_by(weight, Ascending)` | `visible_rows()` in order `[1, 3, 5]` | R-15.15.1 |
| 2 | `sort_by(weight, Descending)` | `visible_rows()` in order `[5, 3, 1]` | R-15.15.1 |

#### TC-15.SE.13 Table Filter By Column

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 rows, `filter_by(type, TextContains("sword"))` | Only rows with "sword" in type column visible | R-15.15.1 |
| 2 | `clear_filters()` | All 5 rows visible | R-15.15.1 |

#### TC-15.SE.14 Table Copy Paste Rows

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select 2 rows, `copy_selection()`, `paste(0)` | 2 new rows at top, values match copied | R-15.15.1 |

#### TC-15.SE.15 Table CRDT Ops Generated

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_cell()`, `pending_crdt_ops()` | Returns 1 `MapOp::Set` operation | R-15.12.8 |

### Animation Graph Editor

#### TC-15.SE.16 Anim Graph Add Blend Node

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_anim_node(BlendNode { Linear }, [100, 100])` | Node has 2 pose inputs, 1 pose output, 1 weight input | R-15.8.7 |

#### TC-15.SE.17 Anim Graph Add IK Node

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_anim_node(IkSolver { chain_length: 3 }, pos)` | Node has pose input, target input, pose output | R-15.8.7 |

### Behavior Tree Editor

#### TC-15.SE.18 BT Add Sequence Node

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_bt_node(Sequence, Some(root))` | Node appears as child of root in tree layout | R-15.13.1 |

#### TC-15.SE.19 BT Debug Overlay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `attach_debug(entity)`, tick returns `Success` on node 3 | `debug_state.active_node == 3`, `node_results[3] == Success` | R-15.13.1 |

#### TC-15.SE.20 BT Decorator Node

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_bt_node(Decorator { Inverter }, parent)` | Node inverts child result in preview | R-15.13.1 |

### State Machine Editor

#### TC-15.SE.21 SM Add State and Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_state("Idle", pos)`, `add_state("Run", pos)`, `add_transition(idle, run, conditions)` | Two state nodes, one directed edge | R-15.13.2 |

#### TC-15.SE.22 SM Set Default State

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_default_state(idle)` | Idle node has entry arrow indicator | R-15.13.2 |

#### TC-15.SE.23 SM Debug Active State

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `attach_debug(entity)` where entity is in "Run" state | `debug_state.active_state == run_id` | R-15.13.2 |

### Quest Editor

#### TC-15.SE.24 Quest Add Objective Node

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_quest_node(Objective { "Collect 5 gems", false }, pos)` | Node renders with objective description | R-15.14.1 |

#### TC-15.SE.25 Quest Validate No Start

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with objectives but no `QuestStart` | `validate_quest()` returns `NoStartNode` | R-15.14.1 |

#### TC-15.SE.26 Quest Validate Unreachable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Objective node with no incoming edges from start | Returns `UnreachableObjective { node }` | R-15.14.1 |

### Loot Table Editor

#### TC-15.SE.27 Loot Simulation Distribution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2 items: weight 1 and weight 3, `run_simulation(10000)` | Item B ~75% drops, item A ~25% drops (within 5% tolerance) | R-15.15.1 |

#### TC-15.SE.28 Loot Simulation Zero Weight

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Item with weight 0, `run_simulation(1000)` | Item never appears in results | R-15.15.1 |

### Ability Ledger

#### TC-15.SE.29 Ability Filter By Tag

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 abilities, 2 tagged "fire", `filter_by_tag("fire")` | 2 rows visible | R-15.15.2 |

#### TC-15.SE.30 Ability Compare Stats

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `compare(fireball, ice_bolt)` | Returns `StatDelta` for damage, cooldown, cost | R-15.15.2 |

### Equipment Stat Table

#### TC-15.SE.31 Equipment Comparison Totals

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set A: sword(atk:10) + shield(def:5), `compute_totals()` | `{"attack": 10.0, "defense": 5.0}` | R-15.15.3 |

#### TC-15.SE.32 Equipment Set Comparison

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_comparison(set_a, set_b)` | `stat_deltas` contains difference per stat | R-15.15.3 |

### Price Ledger

#### TC-15.SE.33 Inflation Simulation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Buy price 100, `run_inflation_sim(0.10, 5)` | Projected buy ~161.05 (100 * 1.1^5) | R-15.15.4 |

#### TC-15.SE.34 Inflation Zero Rate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `run_inflation_sim(0.0, 10)` | Projected prices equal original prices | R-15.15.4 |

### Entity Editor

#### TC-15.SE.35 Entity Add Component

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_component(HealthComponent)` | Component appears in panel list, undo available | R-15.1.4 |

#### TC-15.SE.36 Entity Remove Component

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `remove_component(HealthComponent)` | Component removed from panels, undo restores it | R-15.1.4 |

#### TC-15.SE.37 Entity Hierarchy Reparent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity C child of A, `reparent(C, B)` | C now child of B, undo moves back to A | R-15.1.4 |

#### TC-15.SE.38 Entity Prefab Override Revert

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Prefab hp=100, instance hp=200, `revert_override("hp")` | Instance hp=100, override removed | R-15.2.1 |

#### TC-15.SE.39 Entity Inspector Custom Editor Dispatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plugin registers widget for `PhysicsBody`, select entity with `PhysicsBody` | Inspector renders custom widget, not auto-generated | R-15.1.8 |

#### TC-15.SE.40 Entity Hierarchy Search

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 entities, 3 named "Light*", `search("Light")` | Returns 3 matching entities | R-15.1.4 |

#### TC-15.SE.41 Prefab Apply to Source

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Instance with hp override, `apply_to_prefab()` | Prefab asset updated, all instances reflect new value | R-15.2.1 |

#### TC-15.SE.42 Prefab Revert All

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Instance with 3 overrides, `revert_all()` | All overrides removed, instance matches prefab | R-15.2.1 |

## Integration Tests

### TC-15.SE.50 Graph Edit End-to-End with Undo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add 3 nodes, connect, undo, redo | Graph matches state after redo | R-15.8.7, R-15.1.3 |

### TC-15.SE.51 Table Edit End-to-End with Undo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add row, set cell, undo | Row removed after undo | R-15.15.1, R-15.1.3 |

### TC-15.SE.52 Graph CRDT Collaborative Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Editor A adds node, Editor B receives CRDT op | Both editors show same graph | R-15.12.8 |

### TC-15.SE.53 Table CRDT Collaborative Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Editor A sets cell, Editor B receives CRDT op | Both editors show same cell value | R-15.12.8 |

### TC-15.SE.54 Entity Editor Plugin Widget Integration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin, select entity, verify custom widget, unload plugin, verify fallback | Widget dispatch switches correctly | R-15.1.8, R-1.6.7 |

### TC-15.SE.55 Behavior Tree Debug with Running Agent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start play mode, attach BT debug to agent, observe node highlighting | Active node updates each AI tick | R-15.13.1 |

### TC-15.SE.56 Quest Editor Full Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build complete quest graph with branches, validate | No validation errors | R-15.14.1 |
| 2 | Remove start node, validate | Returns `NoStartNode` | R-15.14.1 |

### TC-15.SE.57 Loot Table Full Authoring Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add items, set weights, run simulation, verify distribution | Simulation matches expected probability distribution | R-15.15.1 |

### TC-15.SE.58 Equipment Comparison Workflow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create two sets, compare, verify deltas | Stat deltas match manual calculation | R-15.15.3 |

### TC-15.SE.59 Price Inflation Full Workflow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set prices, run inflation at 5% for 10 periods | Projections match compound interest formula | R-15.15.4 |

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
