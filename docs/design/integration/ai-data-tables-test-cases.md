# AI Behavior ↔ Data Tables Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.1.1.1 | BT leaf reads aggro radius | RowRef to NPC table, col=aggro | BB key = 15.0 | IR-2.1.1 |
| TC-IR-2.1.1.2 | BT leaf reads missing row | Invalid RowRef | BB key = default | IR-2.1.1 |
| TC-IR-2.1.2.1 | Utility reads stat column | RowRef + col=strength | Score uses 25.0 | IR-2.1.2 |
| TC-IR-2.1.2.2 | Utility reads null column | Nullable col, null val | Score = 0.0 | IR-2.1.2 |
| TC-IR-2.1.3.1 | GOAP cost from table | Ability row, cost=5.0 | Action cost = 5.0 | IR-2.1.3 |
| TC-IR-2.1.3.2 | GOAP cost formula col | Formula col codegen'd | Computed cost value | IR-2.1.3 |
| TC-IR-2.1.4.1 | Ability RowRef resolves | FK to ability table | Cooldown, range read | IR-2.1.4 |
| TC-IR-2.1.4.2 | Ability FK chain | Row inherits parent | Merged values read | IR-2.1.4 |
| TC-IR-2.1.5.1 | BB key binds to column | DatabaseRow + col | BB synced on spawn | IR-2.1.5 |
| TC-IR-2.1.5.2 | BB rebinds on reload | TableReloaded event | BB key updated | IR-2.1.5 |
| TC-IR-2.1.6.1 | Hot reload invalidates | Swap table version+1 | AI re-reads rows | IR-2.1.6 |
| TC-IR-2.1.6.2 | Hot reload mid-tick | Reload during Phase 4 | Deferred to next frame | IR-2.1.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.1.1.B1 | 1000 BT leaf lookups | < 0.5 ms | IR-2.1.1 |
| TC-IR-2.1.2.B1 | 500 utility considerations | < 0.3 ms | IR-2.1.2 |
| TC-IR-2.1.4.B1 | 200 ability RowRef resolves | < 0.1 ms | IR-2.1.4 |
| TC-IR-2.1.6.B1 | Hot reload 10k-row table | < 500 ms | IR-2.1.6 |
