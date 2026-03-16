# Visual Logic Graph Test Cases

Companion test cases for [logic-graph.md](logic-graph.md).

## Unit Tests

### TC-15.8.2.1 Type Inference -- Simple Match

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect `Int` output to `Int` input | `infer()` returns `Ok`, both pins resolve to `Int` | R-15.8.2 |
| 2 | Connect `Float` output to `Float` input | `infer()` returns `Ok`, both pins resolve to `Float` | R-15.8.2 |

### TC-15.8.2.2 Type Inference -- Generic Binding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect `Int` output to `Generic(G)` input | G resolves to `Int` in `InferenceResult.bindings` | R-15.8.2 |
| 2 | Connect `Generic(G)` output to `Bool` input | G resolves to `Bool` | R-15.8.2 |
| 3 | Connect `Generic(G)` output to `Generic(H)` input | G and H unify to same type | R-15.8.2 |

### TC-15.8.2.3 Type Inference -- Bidirectional

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generic output -> typed `Vec3` input | generic parameter resolves to `Vec3` via backward propagation | R-15.8.2 |

### TC-15.8.2.4 Type Mismatch Rejection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect `Float` output to `Bool` input | `Err(TypeDiagnostic)` with `Mismatch { expected: Bool, found: Float }` | R-15.8.2 |
| 2 | Connect `Execution` pin to `Data(Int)` pin | `Err(TypeDiagnostic)` with `IncompatibleCategory` | R-15.8.2 |

### TC-15.8.2.5 No Implicit Coercion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect `Int` output to `Float` input (no conversion node) | `Err(TypeDiagnostic)` with `Mismatch` | R-15.8.2 |

### TC-15.8.3.1 Validation -- Missing Input

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add node with required input pin, leave unconnected | `MissingRequiredInput` diagnostic with correct `pin_id` | R-15.8.3 |

### TC-15.8.3.2 Validation -- Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create cycle A -> B -> C -> A in pure dataflow | `CycleDetected` diagnostic with `cycle == [A, B, C]` | R-15.8.3 |

### TC-15.8.3.3 Validation -- Dangling Pin

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create output pin connected to nothing | `UnusedOutput` warning with correct `pin_id` | R-15.8.3 |

### TC-15.8.3.4 Validation -- Save Blocking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with `MissingRequiredInput` error, attempt save | save rejected, `ValidationResult.is_valid() == false` | R-15.8.3 |
| 2 | Graph with no errors, attempt save | save succeeds, `ValidationResult.is_valid() == true` | R-15.8.3 |

### TC-15.8.12.1 Dead Node Elimination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with 10 nodes, 3 unreachable from any output | after optimization, 3 nodes removed | R-15.8.12 |

### TC-15.8.12.2 Constant Folding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `Add(Constant(3), Constant(4))` | folded to single `Constant(7)` node | R-15.8.12 |
| 2 | `Multiply(Constant(2.0), Constant(3.0))` | folded to `Constant(6.0)` | R-15.8.12 |

### TC-15.8.12.3 Subgraph Inlining

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SubgraphCall to 3-node subgraph | after optimization, no SubgraphCall node; 3 nodes inlined | R-15.8.12 |

### TC-15.8.12.4 Monomorphization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generic `Add<T>` used with `Float` | specialized to `Add_f32`, no generic params remain | R-15.8.12 |

### TC-15.8.4.1 Coroutine Lowering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with 3 `Yield` nodes | `CoroutineStateDesc` with 4 variants (3 phases + Complete) | R-15.8.4 |
| 2 | Graph with 0 `Yield` nodes | no `CoroutineStateDesc` generated | R-15.8.4 |

### TC-15.8.4.2 Event System Trigger

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with `Event(DamageTaken)` entry | `CompiledSystem.trigger == Event(DamageTaken)` | R-15.8.4 |

### TC-15.8.4.3 Tick System Trigger

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with `Tick(Update)` entry | `CompiledSystem.trigger == Tick(Update)` | R-15.8.4 |
| 2 | Graph with `Tick(FixedUpdate)` entry | `CompiledSystem.trigger == Tick(FixedUpdate)` | R-15.8.4 |

### TC-15.8.1.1 Variable Scope -- Local

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Variable with `scope == Local` | compiled as local binding in system fn, not as ECS component | R-15.8.1 |

### TC-15.8.1.2 Variable Scope -- Graph

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Variable with `scope == Graph` | compiled as ECS component field on owning entity | R-15.8.1 |

### TC-15.8.1.3 Variable Scope -- Entity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Variable with `scope == Entity` | compiled as component query access in system params | R-15.8.1 |

### TC-15.8.13.1 Diff -- Added Nodes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base graph with 5 nodes, head graph with 7 nodes (2 added) | `diff.added_nodes.len() == 2` | R-15.8.13 |

### TC-15.8.13.2 Diff -- Removed Edges

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base graph with 4 edges, head graph with 3 edges (1 removed) | `diff.removed_edges.len() == 1` | R-15.8.13 |

### TC-15.8.13.3 Merge -- No Conflict

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base graph, ours adds node A, theirs adds node B | `MergeResult::Clean`, merged graph contains both A and B | R-15.8.13 |

### TC-15.8.13.4 Merge -- Conflict

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base graph, ours modifies node X position, theirs modifies node X kind | `MergeResult::Conflicted` with conflict on node X | R-15.8.13 |

### TC-15.8.14.1 Find Usages

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Function used in 3 of 5 graphs | `find_usages` returns 3 `UsageLocation` entries | R-15.8.14 |
| 2 | Function not used in any graph | `find_usages` returns 0 entries | R-15.8.14 |

### TC-15.8.14.2 Rename Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rename function used in 3 graphs | returns `3`, all 3 graphs updated with new ID/name | R-15.8.14 |

### TC-15.8.10.1 Node Auto-Generation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register component type `Health` | `NodeRegistry` contains Get, Set, Add, Remove nodes for `Health` | R-15.8.10 |
| 2 | Register resource type `Time` | `NodeRegistry` contains Read, Write nodes for `Time` | R-15.8.10 |
| 3 | Register event type `DamageTaken` | `NodeRegistry` contains Send, Receive nodes for `DamageTaken` | R-15.8.10 |

## Integration Tests

### TC-15.8.4.I1 Compile and Run Damage Graph

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Damage graph: Event(DamageTaken) -> GetHealth -> Subtract -> SetHealth; entity HP=100, damage=30, resist=0.0 | HP == 70 after one event | R-15.8.4 |
| 2 | Same graph, entity HP=100, damage=30, resist=0.5 | HP == 85 after one event | R-15.8.4 |

### TC-15.8.4.I2 Compile and Run Coroutine

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-phase coroutine, run 3 frames | state transitions: Phase1 -> Phase2 -> Phase3 -> Complete | R-15.8.4 |

### TC-15.8.4.I3 Event-Driven Chain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph A sends EventX, Graph B reacts to EventX | both execute in same frame, B's side effects visible | R-15.8.4 |

### TC-15.8.5.I1 Shader Graph to HLSL

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fragment shader graph: SampleTexture2D -> Multiply -> Output | valid HLSL containing `Texture2D.Sample` call | R-15.8.5b |

### TC-15.8.5.I2 Shader Stage Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Vertex shader graph missing `SV_Position` output | `Err(MissingStageOutput { stage: Vertex, required: "SV_Position" })` | R-15.8.5a |

### TC-15.8.5.I3 Material Graph PBR

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Material graph with BaseColor, Metallic, Roughness, Normal inputs | HLSL output contains PBR struct with all 4 fields | R-15.8.5c |

### TC-15.8.11.I1 Live Edit Hot Swap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play mode active, modify graph node, apply hot-patch | new system active, entity state preserved, no restart | R-15.8.11 |

### TC-15.8.11.I2 Breakpoint Pause

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set breakpoint on node_3, run graph | `DebugState::Paused { at_node: node_3 }`, pin values inspectable | R-15.8.11 |

### TC-15.8.11.I3 Step-Through Execution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Breakpoint at node_3, `step()` 3 times | advances to node_4, node_5, node_6 in order | R-15.8.11 |

### TC-15.8.11.I4 Execution Trace

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run 5-node graph, read `execution_trace()` | 5 `ExecutionTraceEntry` in correct topological order, all `duration_us > 0` | R-15.8.11 |

### TC-15.8.13.I1 Git Merge Driver

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two `.logicgraph` files with non-overlapping edits, invoke merge driver | clean merged output, valid graph | R-15.8.13 |

### TC-15.8.14.I1 Pattern Replace

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Replace deprecated node in 5 graphs | all 5 graphs updated, all pass validation | R-15.8.14 |

### TC-15.8.10.I1 Subgraph Call Compilation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph calls 3-node subgraph | compiled output includes inlined subgraph logic, no call overhead | R-15.8.10 |

### TC-15.8.12.I1 Incremental Recompile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change 1 node in 100-node graph | only affected subgraph recompiles, total time < full compile | R-15.8.12 |

## Benchmarks

### TC-15.8.1.B1 Compiled vs Hand-Written Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Compiled damage graph vs hand-written equivalent, 10k entities | throughput ratio | within 5% | R-15.8.1 |

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
