# Event Logs ↔ UI Integration Test Cases

All tests are CI-runnable. Integration tests use real `EventLog<CombatEvent>`, real
`CombatLogBinding`, real `ScrollView`, and real `DataBindingComponent` instances — no mocks.
Negative and failure-mode tests run under `cargo test`; benchmarks run under `cargo bench`.

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.10.1.1 | Combat log shows entry | Push damage event | Line in ScrollView | IR-2.10.1 |
| TC-IR-2.10.1.2 | Log formats text | 50 dmg from NPC | "NPC dealt 50 damage" | IR-2.10.1 |
| TC-IR-2.10.1.3 | Log empty state | No events in log | "No entries" displayed | IR-2.10.1 |
| TC-IR-2.10.1.4 | Locale swap updates | Change Locale resource | Lines re-rendered | IR-2.10.1 |
| TC-IR-2.10.2.1 | Activity feed recent | 5 entries, window=60 | 5 lines displayed | IR-2.10.2 |
| TC-IR-2.10.2.2 | Activity feed multi-log | 2 entity logs | Merged chronologically | IR-2.10.2 |
| TC-IR-2.10.3.1 | FCT spawns on damage | LogEntryAdded(damage) | FCT at target pos | IR-2.10.3 |
| TC-IR-2.10.3.2 | FCT despawns after dur | FCT + 2s timer | Entity despawned | IR-2.10.3 |
| TC-IR-2.10.4.1 | Filter by event type | Toggle "healing only" | Only healing shown | IR-2.10.4 |
| TC-IR-2.10.4.2 | Filter by source | Filter source=player | Only player shown | IR-2.10.4 |
| TC-IR-2.10.4.3 | Filter by min_accuracy | min_accuracy=0.5 | Only accuracy>=0.5 | IR-2.10.4 |
| TC-IR-2.10.5.1 | Low accuracy fades | Entry accuracy=0.3 | Opacity = 0.3 | IR-2.10.5 |
| TC-IR-2.10.5.2 | Full accuracy opaque | Entry accuracy=1.0 | Opacity = 1.0 | IR-2.10.5 |
| TC-IR-2.10.6.1 | Auto-scroll at bottom | Scrolled to end + new | Scrolls to new | IR-2.10.6 |
| TC-IR-2.10.6.2 | No auto-scroll if up | Scrolled up + new | Position unchanged | IR-2.10.6 |
| TC-IR-2.10.6.3 | Derived auto-scroll | No auto_scroll field | Grep zero hits | IR-2.10.6 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.10.N1 | No async/await tokens | Grep async fn in module | Zero matches | IR-2.10.1 |
| TC-IR-2.10.N2 | No SPSC channels | Grep spsc in module | Zero matches | IR-2.10.1 |
| TC-IR-2.10.N3 | No Arc on mutable state | Grep Arc<Mutex in module | Zero matches | IR-2.10.1 |
| TC-IR-2.10.N4 | No mocks in tests | Grep mockall/mock_ | Zero matches | IR-2.10.1 |
| TC-IR-2.10.N5 | rkyv derives present | Grep rkyv::Archive types | All four hit | IR-2.10.1 |
| TC-IR-2.10.N6 | All enums enumerated | Parse CombatEventKind | 8 variants | IR-2.10.4 |
| TC-IR-2.10.N7 | Preformatted text gone | Grep `text: SmolStr` | Zero matches | IR-2.10.1 |
| TC-IR-2.10.N8 | DataBindingComponent | Grep bare `DataBinding` | Zero renamed hits | IR-2.10.1 |
| TC-IR-2.10.N9 | Canonical Commands | Grep `CommandBuffer` | Zero matches | IR-2.10.3 |
| TC-IR-2.10.N10 | EventReader lifetime | Grep `EventReader<Log` | Zero unlifetimed | IR-2.10.1 |

## Failure Mode Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.10.FM1 | Log entity despawned | Despawn mid-frame | Widget cleared, warn | FM-1 |
| TC-IR-2.10.FM2 | Log at capacity evicts | Push beyond capacity | Oldest line gone | FM-2 |
| TC-IR-2.10.FM3 | FCT target no Transform | Spawn on bodiless entity | Skip, warn logged | FM-3 |
| TC-IR-2.10.FM4 | Filter matches nothing | EventLogQuery misses | Placeholder line shown | FM-4 |
| TC-IR-2.10.FM5 | High rate throttle | 500 push/frame burst | max_per_frame=32 hit | FM-5 |
| TC-IR-2.10.FM6 | Event queue saturated | 2048 events in frame | Drops logged, no panic | FM-6 |
| TC-IR-2.10.FM7 | Off-screen FCT culled | Target behind camera | No warn, layout culls | FM-3 |

## Debug Toggle Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.10.D1 | trace_binding toggle | Flip trace_binding | Pass logged/frame | IR-2.10.1 |
| TC-IR-2.10.D2 | trace_fct toggle | Flip trace_fct | Spawn logged/call | IR-2.10.3 |
| TC-IR-2.10.D3 | overlay toggle | Flip overlay flag | Counter drawn on HUD | IR-2.10.6 |
| TC-IR-2.10.D4 | Release flags live | Run release + toggle | Flags still usable | IR-2.10.1 |
| TC-IR-2.10.D5 | No restart required | Flip mid-frame | Next frame honors | IR-2.10.1 |

## rkyv Persistence Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.10.R1 | Archive CombatLogBinding | rkyv serialize + load | Round-trip equal | IR-2.10.1 |
| TC-IR-2.10.R2 | Archive CombatLogLine | rkyv serialize + load | Round-trip equal | IR-2.10.1 |
| TC-IR-2.10.R3 | Archive CombatEventKind | rkyv round-trip | 8 variants survive | IR-2.10.4 |
| TC-IR-2.10.R4 | Archive CombatLogArg | rkyv round-trip | 4 variants survive | IR-2.10.1 |
| TC-IR-2.10.R5 | mmap load binding | Archived on disk | Zero-copy works | IR-2.10.1 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.10.1.B1 | 200 log lines rendered | < 1 ms | IR-2.10.1 |
| TC-IR-2.10.1.B2 | Locale swap rebuild 200 | < 1 ms | IR-2.10.1 |
| TC-IR-2.10.3.B1 | 50 FCT entities active | < 0.5 ms | IR-2.10.3 |
| TC-IR-2.10.3.B2 | 50 FCT world-to-screen | < 0.2 ms | IR-2.10.3 |
| TC-IR-2.10.4.B1 | Filter 1000-entry log | < 0.1 ms | IR-2.10.4 |
| TC-IR-2.10.4.B2 | min_accuracy filter 1000 | < 0.1 ms | IR-2.10.4 |
| TC-IR-2.10.6.B1 | 200 msg/s throughput | No dropped entries | IR-2.10.6 |
| TC-IR-2.10.6.B2 | Throttle 500 msg burst | max_per_frame=32 respected | IR-2.10.6 |
| TC-IR-2.10.6.B3 | Auto-scroll derivation | < 50 us per binding | IR-2.10.6 |

Benchmark numeric targets match the "Performance Budget" table in
[event-logs-ui.md](event-logs-ui.md). All benchmarks run under `cargo bench` on the reference CI
runner; no external hardware required.
