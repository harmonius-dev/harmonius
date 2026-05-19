# Graph Runtime Test Cases

Companion test cases for [graph-runtime.md](graph-runtime.md).

## Unit Tests

### TC-1.15.1.1 DagValidator Accepts Linear Graph

| # | Requirement |
|---|-------------|
| 1 | F-1.15.1    |
| 2 | F-1.15.1    |

1. **#1** — Build 3-node chain: A -> B -> C, validate
   - **Expected:** Returns `Ok(GraphIr)` with `sorted == [A, B, C]`
2. **#2** — `ir.roots`
   - **Expected:** `[A]`

### TC-1.15.1.2 DagValidator Rejects Self-Edge Cycle

| # | Requirement |
|---|-------------|
| 1 | F-1.15.1    |

1. **#1** — Node A with edge A -> A, validate
   - **Expected:** `Err(GraphError::Cycle { nodes: [A] })`

### TC-1.15.1.3 DagValidator Rejects Two-Node Cycle

| # | Requirement |
|---|-------------|
| 1 | F-1.15.1    |

1. **#1** — A -> B, B -> A
   - **Expected:** `Err(GraphError::Cycle { nodes: [A, B] })`

### TC-1.15.1.4 DagValidator Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | F-1.15.1    |

1. **#1** — Node A emits `f32`, Node B expects `i32`, edge A -> B
   - **Expected:** `Err(GraphError::TypeMismatch { .. })`

### TC-1.15.1.5 PinCardinality One Violated

| # | Requirement |
|---|-------------|
| 1 | F-1.15.1    |

1. **#1** — Node B has `Cardinality::One` input, two edges connect to it
   - **Expected:** `Err(GraphError::PinCardinality { .. })`

### TC-1.15.1.6 TopologicalSorter Diamond Graph

| # | Requirement |
|---|-------------|
| 1 | F-1.15.1    |

1. **#1** — A -> B, A -> C, B -> D, C -> D
   - **Expected:** `sorted` begins with `A` and ends with `D`; `B` and `C` between

### TC-1.15.1.7 CycleDetector Disconnected Cycle

| # | Requirement |
|---|-------------|
| 1 | F-1.15.1    |

1. **#1** — Acyclic subgraph X -> Y and cyclic subgraph P -> Q -> P
   - **Expected:** `detect()` returns `Some([P, Q])`

### TC-1.15.4.1 Constant Fold Binary Op

| # | Requirement |
|---|-------------|
| 1 | F-1.15.4    |
| 2 | F-1.15.4    |

1. **#1** — Add(Const(3), Const(4)) input to Multiply node
   - **Expected:** After fold, Add replaced with `Value::Int(7)`
2. **#2** — Remaining graph has one fewer node
   - **Expected:** `ir.nodes.len()` decreased by 1

### TC-1.15.4.2 Dead Node Elimination

| # | Requirement |
|---|-------------|
| 1 | F-1.15.4    |

1. **#1** — 5 nodes authored, only 3 reachable from roots
   - **Expected:** After DCE, `ir.nodes.len() == 3`

### TC-1.15.2.1 RustBackend Emits Module

| # | Requirement |
|---|-------------|
| 1 | F-1.15.2    |

1. **#1** — Canonical 3-node arithmetic IR through RustBackend
   - **Expected:** `EmitOutput.artifact` contains `pub fn evaluate(...)`

### TC-1.15.2.2 GlslBackend Emits Shader

| # | Requirement |
|---|-------------|
| 1 | F-1.15.2    |

1. **#1** — Canonical material IR through GlslBackend
   - **Expected:** `EmitOutput.artifact` contains `float4 main(` header

### TC-1.15.2.3 TypeDescriptorBackend Emits Layout

| # | Requirement |
|---|-------------|
| 1 | F-1.15.2    |

1. **#1** — Struct-describing IR through TypeDescriptorBackend
   - **Expected:** `EmitOutput.symbols` contains entries for each field

### TC-1.15.2.4 Backend Determinism

| # | Requirement |
|---|-------------|
| 1 | F-1.15.2    |
| 2 | F-1.15.2    |

1. **#1** — Same IR emitted twice
   - **Expected:** Byte-identical `artifact`
2. **#2** — IR with pin order permuted and canonicalized
   - **Expected:** Identical output after canonicalization pass

### TC-1.15.3.1 Stable NodeId After Reorder

| # | Requirement |
|---|-------------|
| 1 | F-1.15.3    |

1. **#1** — Add node, remove node at lower index, re-validate
   - **Expected:** Remaining node keeps its original `NodeId`

## Integration Tests

### TC-1.15.1.8 Scripting Client Round-Trip

| # | Requirement |
|---|-------------|
| 1 | F-13.4.1    |

1. **#1** — Scripting subsystem compiles "add two numbers" graph via GraphRuntime
   - **Expected:** RustBackend artifact compiles successfully

### TC-2.7.1.1 Material Client Round-Trip

| # | Requirement |
|---|-------------|
| 1 | F-2.7.1     |

1. **#1** — Material subsystem compiles "color + normal" graph
   - **Expected:** GlslBackend artifact passes glslc compile

### TC-7.2.3.1 Behavior Tree Client Round-Trip

| # | Requirement |
|---|-------------|
| 1 | F-7.2.3     |

1. **#1** — BT subsystem compiles "sequence(cond, act)" graph
   - **Expected:** RustBackend output executes via bt runtime

### TC-13.4.3.1 Hot Reload Preserves State

| # | Requirement |
|---|-------------|
| 1 | F-13.4.3    |
| 2 | F-13.4.3    |

1. **#1** — Compile v1 graph, run, swap to v2 via ReloadBarrier
   - **Expected:** Running instances migrated via `StateMigrationFn`
2. **#2** — Inflight instances not matching new shape
   - **Expected:** Rolled back to v1

## Benchmarks

### TC-1.15.1.9 Compile 1K Nodes Under 10 ms

| # | Requirement |
|---|-------------|
| 1 | R-1.15.1a   |

1. **#1** — Random DAG, 1000 nodes / 3000 edges, RustBackend
   - **Expected:** `compile()` completes under 10 ms on reference workstation

### TC-1.15.1.10 Topological Sort 50K Edges Under 1 ms

| # | Requirement |
|---|-------------|
| 1 | R-1.15.1a   |

1. **#1** — 10K nodes / 50K edges
   - **Expected:** `TopologicalSorter::sort` completes under 1 ms

### TC-1.15.4.3 Constant Fold 10K Nodes

| # | Requirement |
|---|-------------|
| 1 | R-1.15.4a   |

1. **#1** — Arithmetic tree of 10K nodes with all leaves constant
   - **Expected:** Fold completes under 5 ms; result is single leaf
