# Directed Graphs ↔ Scripting Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.7.1.1 | Logic graph stores as DG | 5-node visual graph | DirectedGraph with 5 nodes | IR-2.7.1 |
| TC-IR-2.7.1.2 | Edge payloads carry types | f32 output → f32 input | EdgePayload.data_type=F32 | IR-2.7.1 |
| TC-IR-2.7.2.1 | Topo sort sets eval order | A→B→C chain | Codegen: A then B then C | IR-2.7.2 |
| TC-IR-2.7.2.2 | Diamond topo sort | A→B,A→C,B→D,C→D | D evaluated last | IR-2.7.2 |
| TC-IR-2.7.3.1 | Conditional edge compiles | ConditionExpr on edge | if-branch in Rust output | IR-2.7.3 |
| TC-IR-2.7.3.2 | Static condition eliminated | Always-true condition | No branch in output | IR-2.7.3 |
| TC-IR-2.7.4.1 | Traversal maps to coroutine | current_node=NodeId(3) | resume_variant=3 | IR-2.7.4 |
| TC-IR-2.7.4.2 | Traversal reset on reload | Version mismatch | State reset to start | IR-2.7.4 |
| TC-IR-2.7.5.1 | Cycle detected pre-compile | A→B→C→A cycle | GraphError::CycleDetected | IR-2.7.5 |
| TC-IR-2.7.5.2 | Self-loop rejected | A→A edge | GraphError::SelfLoop | IR-2.7.5 |
| TC-IR-2.7.6.1 | Ordered children preserved | Siblings [C,A,B] | Codegen order: C,A,B | IR-2.7.6 |
| TC-IR-2.7.6.2 | Reorder updates codegen | Reorder to [A,B,C] | Codegen order: A,B,C | IR-2.7.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.7.2.B1 | Topo sort 1000-node graph | < 1 ms | IR-2.7.2 |
| TC-IR-2.7.3.B1 | 500 conditional edge evals | < 0.1 ms | IR-2.7.3 |
| TC-IR-2.7.5.B1 | Validate 1000-node DAG | < 2 ms | IR-2.7.5 |
| TC-IR-2.7.4.B1 | 1000 traversal state updates | < 0.1 ms | IR-2.7.4 |
