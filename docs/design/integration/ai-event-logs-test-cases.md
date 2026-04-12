# AI Behavior ↔ Event Logs Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.2.1.1 | BT reads hostile memory | Log with 3 hostile entries | BB threat_count = 3 | IR-2.2.1 |
| TC-IR-2.2.1.2 | BT reads decayed memory | Entries below min_accuracy | BB threat_count = 0 | IR-2.2.1 |
| TC-IR-2.2.1.3 | BT reads empty log | No entries in log | BB threat_count = 0 | IR-2.2.1 |
| TC-IR-2.2.2.1 | Utility scores from log | 5 events, curve=linear | Score = 0.5 | IR-2.2.2 |
| TC-IR-2.2.2.2 | Utility scores zero events | Empty window | Score = 0.0 | IR-2.2.2 |
| TC-IR-2.2.3.1 | GOAP state from threshold | 3+ hostile in 60 ticks | has_threat = true | IR-2.2.3 |
| TC-IR-2.2.3.2 | GOAP state below threshold | 1 hostile in 60 ticks | has_threat = false | IR-2.2.3 |
| TC-IR-2.2.4.1 | Threshold fires alert | Count >= trigger count | ThresholdFired emitted | IR-2.2.4 |
| TC-IR-2.2.4.2 | Threshold window expires | Events outside window | No trigger fires | IR-2.2.4 |
| TC-IR-2.2.5.1 | Gossip updates blackboard | Propagated entry arrives | BB updated with data | IR-2.2.5 |
| TC-IR-2.2.5.2 | Gossip accuracy degrades | 2-hop propagation | accuracy *= 0.7^2 | IR-2.2.5 |
| TC-IR-2.2.6.1 | AI writes decision event | BT selects "flee" | Log entry with flee data | IR-2.2.6 |
| TC-IR-2.2.6.2 | AI decision recalled later | Flee event + 30 ticks | BT reads own flee memory | IR-2.2.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.2.1.B1 | 500 BT log queries | < 0.5 ms | IR-2.2.1 |
| TC-IR-2.2.2.B1 | 500 utility log scores | < 0.5 ms | IR-2.2.2 |
| TC-IR-2.2.4.B1 | 1000 threshold checks | < 0.1 ms | IR-2.2.4 |
| TC-IR-2.2.5.B1 | Propagation to 50 NPCs | < 0.5 ms | IR-2.2.5 |
