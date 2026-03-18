# Visual Logic Graph Test Cases

Companion test cases for [logic-graph.md](logic-graph.md).

## Unit Tests

### TC-15.8.2.1 Type Inference -- Simple Match

| # | Requirement |
|---|-------------|
| 1 | R-15.8.2    |
| 2 | R-15.8.2    |

1. **#1** — Connect `Int` output to `Int` input
   - **Expected:** `infer()` returns `Ok`, both pins resolve to `Int`
2. **#2** — Connect `Float` output to `Float` input
   - **Expected:** `infer()` returns `Ok`, both pins resolve to `Float`

### TC-15.8.2.2 Type Inference -- Generic Binding

| # | Requirement |
|---|-------------|
| 1 | R-15.8.2    |
| 2 | R-15.8.2    |
| 3 | R-15.8.2    |

1. **#1** — Connect `Int` output to `Generic(G)` input
   - **Expected:** G resolves to `Int` in `InferenceResult.bindings`
2. **#2** — Connect `Generic(G)` output to `Bool` input
   - **Expected:** G resolves to `Bool`
3. **#3** — Connect `Generic(G)` output to `Generic(H)` input
   - **Expected:** G and H unify to same type

### TC-15.8.2.3 Type Inference -- Bidirectional

| # | Requirement |
|---|-------------|
| 1 | R-15.8.2    |

1. **#1** — Generic output -> typed `Vec3` input
   - **Expected:** generic parameter resolves to `Vec3` via backward propagation

### TC-15.8.2.4 Type Mismatch Rejection

| # | Requirement |
|---|-------------|
| 1 | R-15.8.2    |
| 2 | R-15.8.2    |

1. **#1** — Connect `Float` output to `Bool` input
   - **Expected:** `Err(TypeDiagnostic)` with `Mismatch { expected: Bool, found: Float }`
2. **#2** — Connect `Execution` pin to `Data(Int)` pin
   - **Expected:** `Err(TypeDiagnostic)` with `IncompatibleCategory`

### TC-15.8.2.5 No Implicit Coercion

| # | Requirement |
|---|-------------|
| 1 | R-15.8.2    |

1. **#1** — Connect `Int` output to `Float` input (no conversion node)
   - **Expected:** `Err(TypeDiagnostic)` with `Mismatch`

### TC-15.8.3.1 Validation -- Missing Input

| # | Requirement |
|---|-------------|
| 1 | R-15.8.3    |

1. **#1** — Add node with required input pin, leave unconnected
   - **Expected:** `MissingRequiredInput` diagnostic with correct `pin_id`

### TC-15.8.3.2 Validation -- Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-15.8.3    |

1. **#1** — Create cycle A -> B -> C -> A in pure dataflow
   - **Expected:** `CycleDetected` diagnostic with `cycle == [A, B, C]`

### TC-15.8.3.3 Validation -- Dangling Pin

| # | Requirement |
|---|-------------|
| 1 | R-15.8.3    |

1. **#1** — Create output pin connected to nothing
   - **Expected:** `UnusedOutput` warning with correct `pin_id`

### TC-15.8.3.4 Validation -- Save Blocking

| # | Requirement |
|---|-------------|
| 1 | R-15.8.3    |
| 2 | R-15.8.3    |

1. **#1** — Graph with `MissingRequiredInput` error, attempt save
   - **Expected:** save rejected, `ValidationResult.is_valid() == false`
2. **#2** — Graph with no errors, attempt save
   - **Expected:** save succeeds, `ValidationResult.is_valid() == true`

### TC-15.8.12.1 Dead Node Elimination

| # | Requirement |
|---|-------------|
| 1 | R-15.8.12   |

1. **#1** — Graph with 10 nodes, 3 unreachable from any output
   - **Expected:** after optimization, 3 nodes removed

### TC-15.8.12.2 Constant Folding

| # | Requirement |
|---|-------------|
| 1 | R-15.8.12   |
| 2 | R-15.8.12   |

1. **#1** — `Add(Constant(3), Constant(4))`
   - **Expected:** folded to single `Constant(7)` node
2. **#2** — `Multiply(Constant(2.0), Constant(3.0))`
   - **Expected:** folded to `Constant(6.0)`

### TC-15.8.12.3 Subgraph Inlining

| # | Requirement |
|---|-------------|
| 1 | R-15.8.12   |

1. **#1** — SubgraphCall to 3-node subgraph
   - **Expected:** after optimization, no SubgraphCall node; 3 nodes inlined

### TC-15.8.12.4 Monomorphization

| # | Requirement |
|---|-------------|
| 1 | R-15.8.12   |

1. **#1** — Generic `Add<T>` used with `Float`
   - **Expected:** specialized to `Add_f32`, no generic params remain

### TC-15.8.4.1 Coroutine Lowering

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |
| 2 | R-15.8.4    |

1. **#1** — Graph with 3 `Yield` nodes
   - **Expected:** `CoroutineStateDesc` with 4 variants (3 phases + Complete)
2. **#2** — Graph with 0 `Yield` nodes
   - **Expected:** no `CoroutineStateDesc` generated

### TC-15.8.4.2 Event System Trigger

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |

1. **#1** — Graph with `Event(DamageTaken)` entry
   - **Expected:** `CompiledSystem.trigger == Event(DamageTaken)`

### TC-15.8.4.3 Tick System Trigger

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |
| 2 | R-15.8.4    |

1. **#1** — Graph with `Tick(Update)` entry
   - **Expected:** `CompiledSystem.trigger == Tick(Update)`
2. **#2** — Graph with `Tick(FixedUpdate)` entry
   - **Expected:** `CompiledSystem.trigger == Tick(FixedUpdate)`

### TC-15.8.1.1 Variable Scope -- Local

| # | Requirement |
|---|-------------|
| 1 | R-15.8.1    |

1. **#1** — Variable with `scope == Local`
   - **Expected:** compiled as local binding in system fn, not as ECS component

### TC-15.8.1.2 Variable Scope -- Graph

| # | Requirement |
|---|-------------|
| 1 | R-15.8.1    |

1. **#1** — Variable with `scope == Graph`
   - **Expected:** compiled as ECS component field on owning entity

### TC-15.8.1.3 Variable Scope -- Entity

| # | Requirement |
|---|-------------|
| 1 | R-15.8.1    |

1. **#1** — Variable with `scope == Entity`
   - **Expected:** compiled as component query access in system params

### TC-15.8.13.1 Diff -- Added Nodes

| # | Requirement |
|---|-------------|
| 1 | R-15.8.13   |

1. **#1** — Base graph with 5 nodes, head graph with 7 nodes (2 added)
   - **Expected:** `diff.added_nodes.len() == 2`

### TC-15.8.13.2 Diff -- Removed Edges

| # | Requirement |
|---|-------------|
| 1 | R-15.8.13   |

1. **#1** — Base graph with 4 edges, head graph with 3 edges (1 removed)
   - **Expected:** `diff.removed_edges.len() == 1`

### TC-15.8.13.3 Merge -- No Conflict

| # | Requirement |
|---|-------------|
| 1 | R-15.8.13   |

1. **#1** — Base graph, ours adds node A, theirs adds node B
   - **Expected:** `MergeResult::Clean`, merged graph contains both A and B

### TC-15.8.13.4 Merge -- Conflict

| # | Requirement |
|---|-------------|
| 1 | R-15.8.13   |

1. **#1** — Base graph, ours modifies node X position, theirs modifies node X kind
   - **Expected:** `MergeResult::Conflicted` with conflict on node X

### TC-15.8.14.1 Find Usages

| # | Requirement |
|---|-------------|
| 1 | R-15.8.14   |
| 2 | R-15.8.14   |

1. **#1** — Function used in 3 of 5 graphs
   - **Expected:** `find_usages` returns 3 `UsageLocation` entries
2. **#2** — Function not used in any graph
   - **Expected:** `find_usages` returns 0 entries

### TC-15.8.14.2 Rename Propagation

| # | Requirement |
|---|-------------|
| 1 | R-15.8.14   |

1. **#1** — Rename function used in 3 graphs
   - **Expected:** returns `3`, all 3 graphs updated with new ID/name

### TC-15.8.10.1 Node Auto-Generation

| # | Requirement |
|---|-------------|
| 1 | R-15.8.10   |
| 2 | R-15.8.10   |
| 3 | R-15.8.10   |

1. **#1** — Register component type `Health`
   - **Expected:** `NodeRegistry` contains Get, Set, Add, Remove nodes for `Health`
2. **#2** — Register resource type `Time`
   - **Expected:** `NodeRegistry` contains Read, Write nodes for `Time`
3. **#3** — Register event type `DamageTaken`
   - **Expected:** `NodeRegistry` contains Send, Receive nodes for `DamageTaken`

## Integration Tests

### TC-15.8.4.I1 Compile and Run Damage Graph

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |
| 2 | R-15.8.4    |

1. **#1** — Damage graph: Event(DamageTaken) -> GetHealth -> Subtract -> SetHealth; entity HP=100,
   damage=30, resist=0.0
   - **Expected:** HP == 70 after one event
2. **#2** — Same graph, entity HP=100, damage=30, resist=0.5
   - **Expected:** HP == 85 after one event

### TC-15.8.4.I2 Compile and Run Coroutine

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |

1. **#1** — 3-phase coroutine, run 3 frames
   - **Expected:** state transitions: Phase1 -> Phase2 -> Phase3 -> Complete

### TC-15.8.4.I3 Event-Driven Chain

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |

1. **#1** — Graph A sends EventX, Graph B reacts to EventX
   - **Expected:** both execute in same frame, B's side effects visible

### TC-15.8.5.I1 Shader Graph to HLSL

| # | Requirement |
|---|-------------|
| 1 | R-15.8.5b   |

1. **#1** — Fragment shader graph: SampleTexture2D -> Multiply -> Output
   - **Expected:** valid HLSL containing `Texture2D.Sample` call

### TC-15.8.5.I2 Shader Stage Validation

| # | Requirement |
|---|-------------|
| 1 | R-15.8.5a   |

1. **#1** — Vertex shader graph missing `SV_Position` output
   - **Expected:** `Err(MissingStageOutput { stage: Vertex, required: "SV_Position" })`

### TC-15.8.5.I3 Material Graph PBR

| # | Requirement |
|---|-------------|
| 1 | R-15.8.5c   |

1. **#1** — Material graph with BaseColor, Metallic, Roughness, Normal inputs
   - **Expected:** HLSL output contains PBR struct with all 4 fields

### TC-15.8.11.I1 Live Edit Hot Swap

| # | Requirement |
|---|-------------|
| 1 | R-15.8.11   |

1. **#1** — Play mode active, modify graph node, apply hot-patch
   - **Expected:** new system active, entity state preserved, no restart

### TC-15.8.11.I2 Breakpoint Pause

| # | Requirement |
|---|-------------|
| 1 | R-15.8.11   |

1. **#1** — Set breakpoint on node_3, run graph
   - **Expected:** `DebugState::Paused { at_node: node_3 }`, pin values inspectable

### TC-15.8.11.I3 Step-Through Execution

| # | Requirement |
|---|-------------|
| 1 | R-15.8.11   |

1. **#1** — Breakpoint at node_3, `step()` 3 times
   - **Expected:** advances to node_4, node_5, node_6 in order

### TC-15.8.11.I4 Execution Trace

| # | Requirement |
|---|-------------|
| 1 | R-15.8.11   |

1. **#1** — Run 5-node graph, read `execution_trace()`
   - **Expected:** 5 `ExecutionTraceEntry` in correct topological order, all `duration_us > 0`

### TC-15.8.13.I1 Git Merge Driver

| # | Requirement |
|---|-------------|
| 1 | R-15.8.13   |

1. **#1** — Two `.logicgraph` files with non-overlapping edits, invoke merge driver
   - **Expected:** clean merged output, valid graph

### TC-15.8.14.I1 Pattern Replace

| # | Requirement |
|---|-------------|
| 1 | R-15.8.14   |

1. **#1** — Replace deprecated node in 5 graphs
   - **Expected:** all 5 graphs updated, all pass validation

### TC-15.8.10.I1 Subgraph Call Compilation

| # | Requirement |
|---|-------------|
| 1 | R-15.8.10   |

1. **#1** — Graph calls 3-node subgraph
   - **Expected:** compiled output includes inlined subgraph logic, no call overhead

### TC-15.8.12.I1 Incremental Recompile

| # | Requirement |
|---|-------------|
| 1 | R-15.8.12   |

1. **#1** — Change 1 node in 100-node graph
   - **Expected:** only affected subgraph recompiles, total time < full compile

## Benchmarks

### TC-15.8.1.B1 Compiled vs Hand-Written Throughput

| # | Metric           | Target    | Requirement |
|---|------------------|-----------|-------------|
| 1 | throughput ratio | within 5% | R-15.8.1    |

1. **1** — Compiled damage graph vs hand-written equivalent, 10k entities

### TC-15.8.2.B1 Type Inference Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full inference on 500-node graph | latency | < 10 ms | R-15.8.2 |
| 2 | Incremental inference after 1 edge add | latency | < 1 ms | R-15.8.2 |

### TC-15.8.3.B1 Full Validation Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full validation on 500-node graph | latency | < 50 ms | R-15.8.3 |

### TC-15.8.12.B1 Full Compilation Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full compilation of 100-node gameplay graph | latency | < 200 ms | R-15.8.12 |

### TC-15.8.11.B1 Live Hot-Patch Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Modify node during play, measure time to system swap | latency | < 100 ms | R-15.8.11 |

### TC-15.8.10.B1 Node Auto-Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Generate nodes for 1000 registered types | latency | < 500 ms | R-15.8.10 |

### TC-15.8.5.B1 Shader Graph to HLSL

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50-node shader graph compiled to HLSL | latency | < 50 ms | R-15.8.5b |

### TC-15.8.14.B1 Project-Wide Search

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Find usages across 1000 graphs | latency | < 500 ms | R-15.8.14 |

### TC-15.8.13.B1 Three-Way Merge

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Three-way merge of 200-node graphs | latency | < 100 ms | R-15.8.13 |
