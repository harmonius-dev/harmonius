# Editor ↔ Asset Pipeline Integration Test Cases

All tests are CI-runnable in a headless editor harness with a fake file system and a fake GPU
uploader. Imports use a simple `FakeImporter` that produces deterministic rkyv blobs.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-9.2.1.1 | File drop enqueues import request | IR-9.2.1 |
| TC-IR-9.2.1.2 | Import success publishes reload event | IR-9.2.1 |
| TC-IR-9.2.1.3 | Import failure publishes banner | IR-9.2.1 |
| TC-IR-9.2.2.1 | Reimport source invalidates downstream | IR-9.2.2 |
| TC-IR-9.2.2.2 | Dep graph update removes stale edges | IR-9.2.2 |
| TC-IR-9.2.2.3 | Cycle detected, import aborts chain | IR-9.2.2 |
| TC-IR-9.2.3.1 | PIE hot reload swaps texture | IR-9.2.3 |
| TC-IR-9.2.3.2 | Edit-mode hot reload swaps mesh | IR-9.2.3 |
| TC-IR-9.2.3.3 | Swap atomic across frame boundary | IR-9.2.3 |
| TC-IR-9.2.4.1 | Progress messages delivered in order | IR-9.2.4 |
| TC-IR-9.2.4.2 | ETA decreases over time | IR-9.2.4 |
| TC-IR-9.2.4.3 | Progress for multi-stage import | IR-9.2.4 |
| TC-IR-9.2.5.1 | Source change fires watcher event | IR-9.2.5 |
| TC-IR-9.2.5.2 | `only_if_stale=true` skips unchanged | IR-9.2.5 |
| TC-IR-9.2.6.1 | Folder drop batches all files | IR-9.2.6 |
| TC-IR-9.2.6.2 | Batch id groups progress | IR-9.2.6 |

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-9.2.1.N1 | Source file read error (FM-1) | IR-9.2.1 |
| TC-IR-9.2.1.N2 | Parse error keeps previous (FM-2) | IR-9.2.1 |
| TC-IR-9.2.2.N1 | Dependency cycle (FM-3) | IR-9.2.2 |
| TC-IR-9.2.3.N1 | Hot reload timeout (FM-4) | IR-9.2.3 |
| TC-IR-9.2.1.N3 | `CH-20` full backpressure (FM-5) | IR-9.2.1 |
| TC-IR-9.2.4.N1 | ProgressCh drops oldest (FM-6) | IR-9.2.4 |
| TC-IR-9.2.6.N1 | Partial batch failure (FM-7) | IR-9.2.6 |

### Test case details

1. **TC-IR-9.2.1.1** -- Input: editor calls `drop_file("tex.png")`. Expected: `AssetImportRequest`
   enqueued with `kind=Texture`.
2. **TC-IR-9.2.1.2** -- Input: import succeeds. Expected: `AssetImportResult` with
   `outcome=Success`; `AssetReloadEvent { scope: EditorOnly }` delivered.
3. **TC-IR-9.2.1.3** -- Input: import fails. Expected: `outcome=Failed`; editor banner raised.
4. **TC-IR-9.2.2.1** -- Input: material depends on texture; re-import texture. Expected: material
   re-imported automatically; `AssetReloadEvent` delivered for both.
5. **TC-IR-9.2.2.2** -- Input: remove a dependency during re-import. Expected: old edge removed from
   `DependencyGraph`; old parent no longer reimports on the child change.
6. **TC-IR-9.2.2.3** -- Input: A depends on B; B depends on A. Expected: cycle detected, both
   imports aborted with a user-visible banner plus `FM-3` counter increment.
7. **TC-IR-9.2.3.1** -- Input: PIE running; re-import texture. Expected: `HotReloadBarrier` parks
   game and editor; `Arc<Texture>` swapped; game resumes with new texture next frame.
8. **TC-IR-9.2.3.2** -- Input: edit mode; re-import mesh. Expected: barrier, mesh `Arc` swap,
   resume.
9. **TC-IR-9.2.3.3** -- Input: during swap, no frame observes a half-swapped asset. Expected:
   registry invariant holds (registry hash before != after; no intermediate state).
10. **TC-IR-9.2.4.1** -- Input: import emits `Read`, `Parse`, `Process`, `Compress`, then multi-tick
    `Bake`, then `Link`, `Upload`. Expected: editor receives ordered `AssetImportProgress` messages
    matching that stage sequence (bake may emit multiple percent ticks).
11. **TC-IR-9.2.4.2** -- Input: measure `eta_s` over progress stream. Expected: monotonically
    non-increasing.
12. **TC-IR-9.2.4.3** -- Input: import with long bake stage. Expected: progress percent crosses each
    stage threshold in order.
13. **TC-IR-9.2.5.1** -- Input: source file mtime changes. Expected: `SourceChangedEvent` fired;
    editor enqueues `AssetImportRequest` with `only_if_stale=true`.
14. **TC-IR-9.2.5.2** -- Input: `only_if_stale=true` but asset is up to date. Expected:
    `outcome=AlreadyUpToDate`; no reimport performed.
15. **TC-IR-9.2.6.1** -- Input: drop folder of 100 files. Expected: 100 `AssetImportRequest`s
    enqueued with a shared `ImportBatchId`.
16. **TC-IR-9.2.6.2** -- Input: progress events from the batch. Expected: editor rolls them up by
    `batch_id` into a per-batch progress bar.
17. **TC-IR-9.2.1.N1** -- Input: fake FS returns `IoError::Permission`. Expected: `outcome=Failed`;
    `FM-1` counter increments.
18. **TC-IR-9.2.1.N2** -- Input: fake importer returns parse error. Expected: `outcome=Failed`;
    asset registry unchanged; `FM-2` counter increments.
19. **TC-IR-9.2.2.N1** -- Covered by TC-IR-9.2.2.3.
20. **TC-IR-9.2.3.N1** -- Input: hot reload barrier doesn't complete in 2s. Expected: barrier
    aborts; swap rolled back; asset registry unchanged; `FM-4` counter increments.
21. **TC-IR-9.2.1.N3** -- Input: burst 32 hot reload reqs against `CH-20` cap=16 without draining.
    Expected: first 16 enqueue; the next 16 each increment `FM-5` while the queue stays at capacity
    until the main thread calls `drain_one`.
22. **TC-IR-9.2.4.N1** -- Input: 500 progress events against `ImportProgressCh` cap=256. Expected:
    244 oldest dropped; `FM-6` counter increments by 244.
23. **TC-IR-9.2.6.N1** -- Input: 100-file batch with one bad file. Expected: batch completes with 99
    `Success` and 1 `Failed`; `FM-7` counter increments.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-9.2.2.B1 | Cascade 100 downstream re-imports | < 50 ms | IR-9.2.2 |
| TC-IR-9.2.3.B1 | Hot reload swap 10 assets | < 1 ms | IR-9.2.3 |
| TC-IR-9.2.4.B1 | Drain 256 progress events | < 0.05 ms | IR-9.2.4 |
| TC-IR-9.2.6.B1 | Batch enqueue 1000 files | < 2 ms | IR-9.2.6 |

All benchmarks run under `cargo bench` in CI; thresholds enforced via the benchmark harness.
