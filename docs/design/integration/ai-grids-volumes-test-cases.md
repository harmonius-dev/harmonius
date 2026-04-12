# AI Behavior ↔ Grids/Volumes Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.3.1.1 | BT samples influence | Agent at (10,10), val=0.8 | BB threat=0.8 | IR-2.3.1 |
| TC-IR-2.3.1.2 | BT samples outside grid | Agent at (-5,-5) | BB threat=default | IR-2.3.1 |
| TC-IR-2.3.2.1 | AI reads flow direction | Cell has Vec2(1,0) | Move east | IR-2.3.2 |
| TC-IR-2.3.2.2 | AI reads unreachable cell | Flow field = zero vec | Fallback path | IR-2.3.2 |
| TC-IR-2.3.3.1 | BT checks visible cell | FogState::Visible | Condition = true | IR-2.3.3 |
| TC-IR-2.3.3.2 | BT checks unexplored | FogState::Unexplored | Condition = false | IR-2.3.3 |
| TC-IR-2.3.4.1 | Utility scores from grid | Cell val=0.5, linear | Score = 0.5 | IR-2.3.4 |
| TC-IR-2.3.4.2 | Utility scores empty cell | Cell val=0.0 | Score = 0.0 | IR-2.3.4 |
| TC-IR-2.3.5.1 | GOAP reads safe zone | Influence > 0.7 | in_safe_zone=true | IR-2.3.5 |
| TC-IR-2.3.5.2 | GOAP reads danger zone | Influence < 0.3 | in_safe_zone=false | IR-2.3.5 |
| TC-IR-2.3.6.1 | AI writes influence | Combat at (5,5) | Grid cell += 0.5 | IR-2.3.6 |
| TC-IR-2.3.6.2 | AI influence propagates | Write + 1 tick | Neighbors get spread | IR-2.3.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.3.1.B1 | 1000 influence samples | < 0.1 ms | IR-2.3.1 |
| TC-IR-2.3.2.B1 | 1000 flow field reads | < 0.1 ms | IR-2.3.2 |
| TC-IR-2.3.3.B1 | 500 fog state checks | < 0.05 ms | IR-2.3.3 |
| TC-IR-2.3.6.B1 | 256x256 propagation | < 1 ms | IR-2.3.6 |
