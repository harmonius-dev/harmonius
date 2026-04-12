# Scripting ↔ Data Tables Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.9.1.1 | Formula col is logic graph | "damage" formula col | DirectedGraph stored | IR-2.9.1 |
| TC-IR-2.9.1.2 | Formula reads same-row cols | base_dmg + bonus cols | Sum of both columns | IR-2.9.1 |
| TC-IR-2.9.2.1 | Formula compiles to Rust | Formula graph asset | fn in .dylib FnPtrTable | IR-2.9.2 |
| TC-IR-2.9.2.2 | Formula output type matches | f32 formula, f32 column | Value::F32 returned | IR-2.9.2 |
| TC-IR-2.9.3.1 | Formula reads FK row | FK to material table | Reads material.weight | IR-2.9.3 |
| TC-IR-2.9.3.2 | Formula resolves FK chain | FK→FK nested ref | Resolved 2 levels deep | IR-2.9.3 |
| TC-IR-2.9.4.1 | Logic graph reads table | TableLookupNode in graph | Value from DataTable | IR-2.9.4 |
| TC-IR-2.9.4.2 | Logic graph reads via entity | EntityBinding row source | Row from DatabaseRow comp | IR-2.9.4 |
| TC-IR-2.9.5.1 | Table reload re-evals | Change base_dmg, reload | Formula recomputed | IR-2.9.5 |
| TC-IR-2.9.5.2 | Formula reload re-bakes | New formula .dylib | Table cells updated | IR-2.9.5 |
| TC-IR-2.9.6.1 | Cycle detected at bake | A.formula reads B.formula | Rejected with error | IR-2.9.6 |
| TC-IR-2.9.6.2 | Type mismatch rejected | f32 formula, i32 column | Compile error reported | IR-2.9.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.9.2.B1 | 10k formula evaluations | < 1 ms | IR-2.9.2 |
| TC-IR-2.9.3.B1 | 5k FK resolutions in formulas | < 0.5 ms | IR-2.9.3 |
| TC-IR-2.9.4.B1 | 1000 table lookup nodes | < 0.5 ms | IR-2.9.4 |
| TC-IR-2.9.5.B1 | Hot reload 10k-row table | < 500 ms | IR-2.9.5 |
