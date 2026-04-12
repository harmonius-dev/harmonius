# Networking ↔ ECS Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.4.1.1 | Changed component detected | Modify replicated component | Changed<T> query returns entity | IR-4.4.1 |
| TC-IR-4.4.1.2 | Unchanged component skipped | No modification | Changed<T> query empty | IR-4.4.1 |
| TC-IR-4.4.2.1 | Delta packet contains only diff | Change 1 of 5 fields | Packet has 1 field delta | IR-4.4.2 |
| TC-IR-4.4.2.2 | Baseline advances on ACK | Client ACKs tick 100 | Baseline = 100 | IR-4.4.2 |
| TC-IR-4.4.3.1 | Nearby entity is relevant | Entity within AOI radius | Included in replication | IR-4.4.3 |
| TC-IR-4.4.3.2 | Distant entity filtered out | Entity beyond AOI radius | Excluded from replication | IR-4.4.3 |
| TC-IR-4.4.4.1 | Entity spawn replicated | Server spawns entity | Client receives full snapshot | IR-4.4.4 |
| TC-IR-4.4.4.2 | Entity despawn tombstoned | Server despawns entity | Client receives tombstone | IR-4.4.4 |
| TC-IR-4.4.4.3 | Tombstone TTL expires | 2x max RTT elapsed | Client removes entity | IR-4.4.4 |
| TC-IR-4.4.5.1 | Snapshot stores N ticks | Run 128 ticks | Buffer holds 128 snapshots | IR-4.4.5 |
| TC-IR-4.4.5.2 | Snapshot restore correct | Restore tick 50 | World matches tick 50 state | IR-4.4.5 |
| TC-IR-4.4.6.1 | Entity becomes dormant | No changes for threshold | DormancyManager marks dormant | IR-4.4.6 |
| TC-IR-4.4.6.2 | Dormant entity wakes | Component changed | Dormancy cleared, replication resumes | IR-4.4.6 |
| TC-IR-4.4.7.1 | Authority transfer completes | 3-phase protocol | New authority owns entity | IR-4.4.7 |
| TC-IR-4.4.7.2 | Transfer timeout aborts | No ACK within timeout | Original authority retained | IR-4.4.7 |
| TC-IR-4.4.8.1 | Reconciliation replays inputs | Mismatch at tick T | State matches after replay | IR-4.4.8 |
| TC-IR-4.4.8.2 | Correct prediction skips replay | Match at tick T | No replay, inputs discarded | IR-4.4.8 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.4.1.B1 | Change detection 10k entities | < 0.2 ms | IR-4.4.1 |
| TC-IR-4.4.2.B1 | Delta compression 1000 entities | < 0.3 ms | IR-4.4.2 |
| TC-IR-4.4.3.B1 | Interest management 64 clients | < 0.5 ms | IR-4.4.3 |
| TC-IR-4.4.5.B1 | Snapshot capture 10k entities | < 0.5 ms | IR-4.4.5 |
| TC-IR-4.4.8.B1 | Reconciliation replay 10 inputs | < 1 ms | IR-4.4.8 |
