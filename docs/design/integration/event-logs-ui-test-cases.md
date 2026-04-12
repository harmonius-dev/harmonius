# Event Logs ↔ UI Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.10.1.1 | Combat log shows entry | Push damage event | Line in ScrollView | IR-2.10.1 |
| TC-IR-2.10.1.2 | Combat log formats text | 50 dmg, source=NPC | "NPC dealt 50 damage" | IR-2.10.1 |
| TC-IR-2.10.1.3 | Combat log empty state | No events in log | "No entries" displayed | IR-2.10.1 |
| TC-IR-2.10.2.1 | Activity feed recent | 5 entries, window=60 | 5 lines displayed | IR-2.10.2 |
| TC-IR-2.10.2.2 | Activity feed multi-log | 2 entity logs | Merged chronologically | IR-2.10.2 |
| TC-IR-2.10.3.1 | FCT spawns on damage | LogEntryAdded (damage) | FCT at target world pos | IR-2.10.3 |
| TC-IR-2.10.3.2 | FCT despawns after dur | FCT + 2 second timer | Entity despawned | IR-2.10.3 |
| TC-IR-2.10.4.1 | Filter by event type | Toggle "healing only" | Only healing entries shown | IR-2.10.4 |
| TC-IR-2.10.4.2 | Filter by source | Filter source=player | Only player entries shown | IR-2.10.4 |
| TC-IR-2.10.4.3 | Filter by min_accuracy | min_accuracy=0.5 | Only accuracy>=0.5 shown | IR-2.10.4 |
| TC-IR-2.10.5.1 | Low accuracy fades | Entry accuracy=0.3 | Opacity = 0.3 | IR-2.10.5 |
| TC-IR-2.10.5.2 | Full accuracy opaque | Entry accuracy=1.0 | Opacity = 1.0 | IR-2.10.5 |
| TC-IR-2.10.6.1 | Auto-scroll at bottom | Scrolled to end + new | Scroll to new entry | IR-2.10.6 |
| TC-IR-2.10.6.2 | No auto-scroll if up | Scrolled up + new | Scroll position unchanged | IR-2.10.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.10.1.B1 | 200 log lines rendered | < 1 ms | IR-2.10.1 |
| TC-IR-2.10.3.B1 | 50 FCT entities active | < 0.5 ms | IR-2.10.3 |
| TC-IR-2.10.4.B1 | Filter 1000-entry log | < 0.1 ms | IR-2.10.4 |
| TC-IR-2.10.6.B1 | 200 msg/s throughput | No dropped entries | IR-2.10.6 |
| TC-IR-2.10.6.B2 | Throttle 500 msg burst | max_per_frame=32 respected | IR-2.10.6 |
