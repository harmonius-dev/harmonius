# Save System ↔ Profiler Integration Test Cases

All tests are CI-runnable. Save writes use an in-memory `FakeFileSystem` that reports elapsed
simulated time and byte counts. RSS values use a fake sampler that lets tests inject known values.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-8.1.1.1 | Gather phase event emitted | IR-8.1.1 |
| TC-IR-8.1.1.2 | Archive phase event carries bytes | IR-8.1.1 |
| TC-IR-8.1.1.3 | Compress phase event emitted | IR-8.1.1 |
| TC-IR-8.1.1.4 | Finalize event closes save_id | IR-8.1.1 |
| TC-IR-8.1.2.1 | IoWriteEvent correlated by save_id | IR-8.1.2 |
| TC-IR-8.1.2.2 | Duration matches fake I/O cost | IR-8.1.2 |
| TC-IR-8.1.3.1 | MemorySnapshotEvent before < after | IR-8.1.3 |
| TC-IR-8.1.3.2 | Peak recorded during archive | IR-8.1.3 |
| TC-IR-8.1.4.1 | Phase 8 total includes save events | IR-8.1.4 |
| TC-IR-8.1.4.2 | Budget breach raised when over 0.5 ms | IR-8.1.4 |
| TC-IR-8.1.5.1 | Success increments `total_saves` | IR-8.1.5 |
| TC-IR-8.1.5.2 | Failure increments `total_failures` | IR-8.1.5 |
| TC-IR-8.1.5.3 | P99 rolling window correct | IR-8.1.5 |
| TC-IR-8.1.6.1 | Schema breakdown sums equal archive bytes | IR-8.1.6 |
| TC-IR-8.1.6.2 | Sunburst entity counts match archive | IR-8.1.6 |

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-8.1.1.N1 | CH-24 full drops oldest (FM-1) | IR-8.1.1 |
| TC-IR-8.1.3.N1 | RSS syscall failure keeps last (FM-2) | IR-8.1.3 |
| TC-IR-8.1.2.N1 | I/O write error recorded (FM-3) | IR-8.1.2 |
| TC-IR-8.1.1.N2 | Profiler off skips events (FM-4) | IR-8.1.1 |
| TC-IR-8.1.6.N1 | Orphaned schema event logged (FM-5) | IR-8.1.6 |

### Test case details

1. **TC-IR-8.1.1.1** -- Input: save with fake gather cost 2 ms. Expected: `SaveProfileEvent` with
   `phase=Gather`, `duration_ms=2.0`, `save_id` matching save call.
2. **TC-IR-8.1.1.2** -- Input: archive produces 1 MiB. Expected: `phase=Archive`, `bytes=1<<20`.
3. **TC-IR-8.1.1.3** -- Input: compress step runs. Expected: `phase=Compress` event in stream.
4. **TC-IR-8.1.1.4** -- Input: save completes successfully. Expected: `phase=Finalize` event last;
   profiler marks save_id complete.
5. **TC-IR-8.1.2.1** -- Input: I/O write for save_id=42. Expected: `IoWriteEvent.save_id == 42`
   correlated in profiler aggregation.
6. **TC-IR-8.1.2.2** -- Input: fake I/O returns 5 ms. Expected: `duration_ms=5.0`.
7. **TC-IR-8.1.3.1** -- Input: fake RSS sampler returns `before=100 MiB`, `after=110 MiB`. Expected:
   `MemorySnapshotEvent.rss_before=100<<20`, `rss_after=110<<20`.
8. **TC-IR-8.1.3.2** -- Input: peak RSS during archive is 120 MiB. Expected: `peak_during=120<<20`.
9. **TC-IR-8.1.4.1** -- Input: save with total 0.45 ms (gather 0.1, archive 0.2, compress 0.15).
   Expected: Phase 8 total includes 0.45 ms; no breach.
10. **TC-IR-8.1.4.2** -- Input: save with total 0.7 ms. Expected: profiler emits budget-breach
    marker; `DebugFlags::show_profiler_hud` shows red Phase 8 bar.
11. **TC-IR-8.1.5.1** -- Input: 3 successful saves. Expected: `total_saves=3`, `total_failures=0`.
12. **TC-IR-8.1.5.2** -- Input: 2 successful saves and 1 with I/O error. Expected: `total_saves=2`,
    `total_failures=1`.
13. **TC-IR-8.1.5.3** -- Input: 100 saves with durations 1..100 ms. Expected: P99 in rolling window
    equals 99 ms ±1.
14. **TC-IR-8.1.6.1** -- Input: archive split across 3 schema subtrees (player 100 B, world 400 B,
    ai 500 B); total archive bytes = 1000. Expected: sum of `SchemaBreakdownEvent.bytes` equals
    1000.
15. **TC-IR-8.1.6.2** -- Input: player subtree contains 10 entities. Expected: corresponding
    `SchemaBreakdownEvent.entity_count = 10`.
16. **TC-IR-8.1.1.N1** -- Input: burst 64 events into CH-24 cap=32. Expected: 32 oldest dropped;
    `FM-1` counter increments.
17. **TC-IR-8.1.3.N1** -- Input: RSS sampler returns error. Expected: `MemorySnapshotEvent` uses
    last-known values; `FM-2` counter increments.
18. **TC-IR-8.1.2.N1** -- Input: write fails with `IoError::DiskFull`. Expected:
    `IoWriteEvent.error = DiskFull`; profiler surfaces in HUD; `total_failures += 1`.
19. **TC-IR-8.1.1.N2** -- Input: profiler disabled. Expected: no events emitted; save still runs.
20. **TC-IR-8.1.6.N1** -- Input: schema event for a save_id the profiler has already finalized.
    Expected: event logged as orphan; `FM-5` counter increments.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-8.1.1.B1 | Event emission overhead | < 0.005 ms per event | IR-8.1.1 |
| TC-IR-8.1.4.B1 | Profiler aggregation 32 events | < 0.02 ms | IR-8.1.4 |
| TC-IR-8.1.6.B1 | Schema breakdown 256 nodes | < 0.05 ms | IR-8.1.6 |

All benchmarks run under `cargo bench` in CI; thresholds enforced via the benchmark harness.
