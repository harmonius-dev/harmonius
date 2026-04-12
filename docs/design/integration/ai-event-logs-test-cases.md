# AI Behavior ↔ Event Logs Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.2.1.1 | BT reads hostile memory | Log w/ 3 hostile entries | BB threat_count = 3 | IR-2.2.1 |
| TC-IR-2.2.1.2 | BT reads decayed memory | Entries below min_accuracy | BB threat_count = 0 | IR-2.2.1 |
| TC-IR-2.2.1.3 | BT reads empty log | No entries in log | BB threat_count = 0 | IR-2.2.1 |
| TC-IR-2.2.2.1 | Utility scores from log | 5 events, curve=linear | Score = 0.5 | IR-2.2.2 |
| TC-IR-2.2.2.2 | Utility scores zero events | Empty window | Score = 0.0 | IR-2.2.2 |
| TC-IR-2.2.3.1 | GOAP state from threshold | 3+ hostile in 60 ticks | has_threat = true | IR-2.2.3 |
| TC-IR-2.2.3.2 | GOAP state below threshold | 1 hostile in 60 ticks | has_threat = false | IR-2.2.3 |
| TC-IR-2.2.4.1 | Threshold fires alert | Count >= trigger count | ThresholdFired emitted | IR-2.2.4 |
| TC-IR-2.2.4.2 | Threshold window expires | Events outside window | No trigger fires | IR-2.2.4 |
| TC-IR-2.2.5.1 | Gossip updates blackboard | Propagated entry arrives | BB updated after flush | IR-2.2.5 |
| TC-IR-2.2.5.2 | Gossip accuracy degrades | 2-hop propagation | accuracy *= 0.7^2 | IR-2.2.5 |
| TC-IR-2.2.5.3 | Flush ordering deterministic | 2 writers same key | Sorted last-writer wins | IR-2.2.5 |
| TC-IR-2.2.6.1 | AI writes decision event | BT selects "flee" | Log entry w/ flee data | IR-2.2.6 |
| TC-IR-2.2.6.2 | AI decision recalled later | Flee event + 30 ticks | BT reads own flee memory | IR-2.2.6 |
| TC-IR-2.2.6.3 | Write visible next tick | Phase 4 write | Phase 3 next sees entry | IR-2.2.6 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.2.N1 | Same-tick write invisible | Phase 4 write then read | Same tick excludes entry | IR-2.2.6 |
| TC-IR-2.2.N2 | No Phase 3↔4 overlap | Race detector run | No race flagged | IR-2.2.4 |
| TC-IR-2.2.N3 | HashMap forbidden in BB | Grep BlackboardScope | No HashMap symbol | IR-2.2.1 |
| TC-IR-2.2.N4 | No Arc on mutable state | Grep Arc<EventLog | Zero matches | IR-2.2.6 |
| TC-IR-2.2.N5 | No async/await tokens | Grep async.*fn in impl | Zero matches | IR-2.2.4 |
| TC-IR-2.2.N6 | Non-MPSC forbidden | Grep spsc channel | Zero matches | IR-2.2.4 |

## Failure Mode Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.2.FM1 | Empty log fallback | Query empty log | Empty SmallVec | FM-1 |
| TC-IR-2.2.FM2 | All entries decayed | Advance ticks past decay | Empty SmallVec, patrol | FM-2 |
| TC-IR-2.2.FM3 | Propagation overflow FIFO | Full log + propagation | Oldest evicted, new kept | FM-3 |
| TC-IR-2.2.FM4 | Predicate mismatch empty | Wrong predicate vs log | Empty SmallVec, no panic | FM-4 |
| TC-IR-2.2.FM5 | Unconsumed threshold drop | ThresholdFired, no AI | Channel drained, warn | FM-5 |
| TC-IR-2.2.FM6 | Channel overflow drop | 65+ ThresholdFired/tick | 64 delivered, rest drop | FM-6 |
| TC-IR-2.2.FM7 | Missing predicate id | Stale PredicateId | Empty result + warn log | FM-7 |

## Debug Toggle Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.2.D1 | Trace queries runtime on | Flip trace_queries=true | Log line per query | IR-2.2.1 |
| TC-IR-2.2.D2 | Trace thresholds toggle | Flip trace_thresholds | Fired events logged | IR-2.2.4 |
| TC-IR-2.2.D3 | No cfg gating of flags | Release build run | All flags usable | IR-2.2.1 |

## rkyv Persistence Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.2.R1 | Archive AiDecisionEntry | rkyv serialize entry | Round-trip equal | IR-2.2.6 |
| TC-IR-2.2.R2 | mmap load EventLog | Archived log on disk | Zero-copy access works | IR-2.2.6 |
| TC-IR-2.2.R3 | Archived predicate call | Pass &Archived to pred | Predicate returns bool | IR-2.2.1 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.2.1.B1 | 500 BT log queries | < 0.5 ms | IR-2.2.1 |
| TC-IR-2.2.2.B1 | 500 utility log scores | < 0.5 ms | IR-2.2.2 |
| TC-IR-2.2.4.B1 | 1000 threshold checks | < 0.1 ms | IR-2.2.4 |
| TC-IR-2.2.5.B1 | Propagation to 50 NPCs | < 0.5 ms | IR-2.2.5 |
| TC-IR-2.2.5.B2 | PropagationBuffer flush 1k | < 0.2 ms | IR-2.2.5 |
| TC-IR-2.2.6.B1 | AI decision write 1k/tick | < 0.3 ms | IR-2.2.6 |
| TC-IR-2.2.1.B2 | Blackboard BTreeMap read | < 50 ns per get | IR-2.2.1 |

All benchmarks run under `cargo bench` on the CI runner; no external hardware required. Negative and
failure-mode tests run under `cargo test`.
