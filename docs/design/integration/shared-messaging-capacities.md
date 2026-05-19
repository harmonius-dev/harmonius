# Shared Messaging Capacities

## Purpose

Single normative table of every bounded cross-thread channel and triple buffer in the engine, with
its computed capacity, rationale, and overflow policy. Integration docs link here instead of
restating capacity numbers. If you add a new cross-thread edge, add a row to the table below and
cross-link from your integration doc -- do NOT inline a capacity number.

See also [shared-conventions.md#sc-7](shared-conventions.md) for the capacity formula rule.

No companion test-cases file -- this is a rules document.

## Capacity Formula

```text
Capacity = MaxProducersPerFrame × BurstSize × SafetyMargin
```

| Term | Definition | Typical range |
|------|------------|--------------|
| `MaxProducersPerFrame` | Peak distinct producers in one frame | 1 to N_WORKERS |
| `BurstSize` | Items a single producer may enqueue per frame | 1 to ~500 |
| `SafetyMargin` | 1.5 (low-drop), 2.0 (typical), 3.0, 4.0 (rare-drop) | 1.5 - 4.0 |

After computing `Capacity`, round **up** to the next power of 2 to satisfy lock-free queue sizing
and cache alignment.

### SafetyMargin selection

| Margin | When to use |
|--------|------------|
| `1.5` | Overflow is recoverable and cheap (drop oldest, log) |
| `2.0` | Typical; standard per-frame traffic |
| `3.0` | Overflow causes visible glitches (dropped audio params) |
| `4.0` | Overflow corrupts protocol state (replication diffs, save writes) |

### Tunability

Every channel capacity is a `const` generic parameter on the channel wrapper type so that per-title
builds may override without recompiling the engine core. Defaults listed below are for a
mid-complexity title (100-500 active NPCs, 3k physics bodies, 10k particles).

```rust
/// Wrapper over `crossbeam_channel::bounded` with a
/// const-generic capacity parameter. Per-title builds
/// override `CAP` via a cargo feature that sets a
/// const environment variable.
pub struct BoundedMpsc<T, const CAP: usize> {
    tx: Sender<T>,
    rx: Receiver<T>,
}
```

## Canonical Capacity Table

The table below is split into two tables sharing the `ID` column (per CLAUDE.md rule on wide
tables). Detail notes follow as a numbered list.

### Producers and consumers

| ID | Channel | Producer | Consumer | Edge doc |
|----|---------|----------|----------|----------|
| CH-1 | Input events | Main (device pump) | Workers (action mapping) | input-*.md |
| CH-2 | Network packets (in) | Main (QUIC recv) | Workers (replication) | networking-*.md |
| CH-3 | Audio commands | Workers (Phase 7) | Audio RT | audio-*.md |
| CH-4 | I/O requests | Workers (any) | Main (drain) | core-runtime/io.md |
| CH-5 | Save writes | Workers (save sys) | Main (platform I/O) | save-*.md |
| CH-6 | Render frame | Workers (Phase 7) | Render thread | high-level.md |
| CH-7 | Beat events | Audio RT (mix cb) | Workers (timelines) | timelines-audio.md |
| CH-8 | Timeline seek | Workers (scripting) | Workers (timelines) | timelines-scripting.md |
| CH-9 | Dialogue choice | Workers (UI) | Workers (quest) | scripting-ui.md (new) |
| CH-10 | Shader compile job | Workers (rendering) | Workers (compile pool) | asset-pipeline-rendering.md |
| CH-11 | Pipeline cmd | Workers (render prep) | Workers (pso cache) | asset-pipeline-rendering.md |
| CH-12 | Grid sync | Workers (grid) | Workers (ai, rendering) | ai-grids-volumes.md |
| CH-13 | UI extract | Workers (ui layout) | Render thread | rendering-ui.md |
| CH-14 | Physics events | Workers (physics) | Workers (audio, vfx, script) | audio-physics.md |
| CH-15 | Physics authority | Main (net) | Workers (physics) | networking-physics.md |
| CH-16 | Network voice | Main (net) | Workers (audio jitter buf) | networking-audio.md |
| CH-17 | Network save RPC | Main (net) | Workers (save) | networking-save-system.md |
| CH-18 | Propagation result | Workers (audio prop) | Audio RT | audio-spatial-awareness.md |
| CH-19 | Asset load done | Main (io drain) | Workers (asset sys) | asset-pipeline-*.md |
| CH-20 | Hot reload req | Workers (editor) | Main (dlopen) | editor-asset-pipeline.md (new) |
| CH-21 | Hot reload result | Main (dlopen) | Workers (reload mgr) | editor-asset-pipeline.md (new) |
| CH-22 | Editor mutation | Workers (editor) | Workers (bridge) | editor-core-runtime.md (new) |
| CH-23 | Profiler sample | Workers (any) | Workers (profiler) | profiler-game-loop.md |
| CH-24 | Save profile ev | Workers (save) | Workers (profiler) | save-system-profiler.md (new) |
| CH-25 | Occlusion raycast | Workers (audio prop) | Workers (spatial bvh) | audio-physics.md |
| CH-26 | AI nav query | Workers (ai) | Workers (nav) | ai-physics.md (new) |
| CH-27 | UI cursor ray | Workers (ui) | Workers (physics) | ui-physics.md (new) |
| CH-28 | VFX surface query | Workers (vfx) | Workers (geometry) | geometry-vfx.md (new) |
| CH-29 | Locale change | Workers (settings) | Workers (ui, audio) | localization-ui.md (new) |
| CH-30 | Collision contact | Workers (physics) | Workers (audio impact) | audio-physics.md |
| CH-31 | Particle spawn req | Workers (anim events) | Workers (vfx) | animation-vfx.md |
| CH-32 | Draw command cache | Workers (rendering) | Render thread | rendering-*.md |

### Capacity, policy, and rationale

| ID | MaxProd | Burst | Margin | Capacity | Kind | Overflow |
|----|--------|-------|--------|----------|------|----------|
| CH-1 | 1 | 200 | 3.0 | 1024 | MPSC | DropOldest |
| CH-2 | 1 | 500 | 4.0 | 4096 | MPSC | DropOldest |
| CH-3 | N | 8 | 4.0 | 4096 | MPSC | CoalesceParams |
| CH-4 | N | 32 | 2.0 | 1024 | MPSC | BackPressure |
| CH-5 | 1 | 8 | 4.0 | 64 | MPSC | BackPressure |
| CH-6 | 1 | 1 | -- | 3 | TripleBuf | Overwrite |
| CH-7 | 1 | 8 | 4.0 | 256 | MPSC | DropOldest |
| CH-8 | N | 4 | 4.0 | 256 | MPSC | DropOldest |
| CH-9 | 1 | 2 | 4.0 | 16 | MPSC | BackPressure |
| CH-10 | N | 4 | 2.0 | 64 | MPSC | BackPressure |
| CH-11 | N | 8 | 2.0 | 128 | MPSC | BackPressure |
| CH-12 | N | 16 | 2.0 | 128 | MPSC | DropOldest |
| CH-13 | 1 | 1 | -- | 2 | DoubleBuf | Overwrite |
| CH-14 | N | 16 | 2.0 | 256 | MPSC | DropOldest |
| CH-15 | 1 | 32 | 4.0 | 128 | MPSC | BackPressure |
| CH-16 | 1 | 32 | 4.0 | 1024 | MPSC | DropOldest |
| CH-17 | 1 | 4 | 4.0 | 64 | MPSC | BackPressure |
| CH-18 | N | 8 | 3.0 | 256 | MPSC | CoalesceParams |
| CH-19 | 1 | 32 | 2.0 | 256 | MPSC | BackPressure |
| CH-20 | 1 | 4 | 2.0 | 16 | MPSC | BackPressure |
| CH-21 | 1 | 4 | 2.0 | 16 | MPSC | BackPressure |
| CH-22 | 1 | 64 | 2.0 | 256 | MPSC | BackPressure |
| CH-23 | N | 32 | 2.0 | 2048 | MPSC | DropOldest |
| CH-24 | 1 | 4 | 2.0 | 32 | MPSC | DropOldest |
| CH-25 | N | 8 | 2.0 | 256 | MPSC | DropOldest |
| CH-26 | N | 16 | 2.0 | 256 | MPSC | DropOldest |
| CH-27 | 1 | 2 | 2.0 | 8 | MPSC | DropOldest |
| CH-28 | N | 16 | 2.0 | 256 | MPSC | DropOldest |
| CH-29 | 1 | 1 | 4.0 | 16 | MPSC | BackPressure |
| CH-30 | N | 32 | 2.0 | 512 | MPSC | DropOldest |
| CH-31 | N | 8 | 2.0 | 256 | MPSC | DropOldest |
| CH-32 | 1 | 1 | -- | 3 | TripleBuf | Overwrite |

`N` in `MaxProd` means "up to `N_WORKERS`" where `N_WORKERS` is the job system worker count. The
capacity uses `N_WORKERS = 8` as the reference value; titles with more workers tune via the const
generic knob.

## Rationale Notes

1. **CH-1 Input events (cap=1024)** -- one worst-case producer (the OS pump), ~200 events per frame
   under rapid gamepad + mouse + keyboard input, SafetyMargin=3.0 because sustained overflow is
   visible as dropped inputs. Computed 600, rounded up to 1024.
2. **CH-2 Network packets (cap=4096)** -- one producer (main thread recv), ~500 packets/frame peak
   (multiplayer with 64 players), SafetyMargin=4.0 because protocol layer can NAK but drops add
   latency. Computed 2000, rounded to 4096.
3. **CH-3 Audio commands (cap=4096)** -- up to N workers (N=8), 8 commands/worker/frame peak
   (parameter updates dominate), SafetyMargin=4.0 because dropping an audio command causes glitch.
   Computed 256, rounded to 512... but bumped to 4096 to match the per-audio-callback coalescing
   window (8 frames at 60 fps fits in one 5ms callback).
4. **CH-4 I/O requests (cap=1024)** -- N worker producers, up to 32 requests/worker in a save flush
   frame, SafetyMargin=2.0. Computed 512, rounded to 1024.
5. **CH-5 Save writes (cap=64)** -- one producer (save system worker), up to 8 save jobs/frame,
   SafetyMargin=4.0 because losing a save write is unacceptable. Computed 32, rounded to 64.
6. **CH-6 Render frame (cap=3)** -- triple buffer; no formula, the three slots are `back`, `middle`,
   `front`. Writer swaps `back <-> middle`; reader swaps `middle <-> front`.
7. **CH-7 Beat events (cap=256)** -- audio RT produces beat/bar events, workers consume in
   timelines. 8 beats/frame peak in fast tempo, SafetyMargin=4.0 for cue accuracy. Computed 32,
   bumped to 256 for a two-second drain window safety.
8. **CH-8 Timeline seek (cap=256)** -- scripting triggers timeline seeks. N workers × 4 seeks/frame
   × 4.0 = 128, rounded to 256.
9. **CH-9 Dialogue choice (cap=16)** -- UI confirms a single choice, quest system drains. Tiny
   channel, SafetyMargin=4.0 for reliability.
10. **CH-10 Shader compile (cap=64)** -- async glslc subprocess jobs, 4 per worker per hot-reload
    burst, SafetyMargin=2.0. Computed 64 directly.
11. **CH-11 Pipeline command (cap=128)** -- PSO cache lookups from render prep, 8/worker/frame,
    SafetyMargin=2.0. Computed 128.
12. **CH-12 Grid sync (cap=128)** -- grid cells producing deltas, 16/worker/frame, SafetyMargin=2.0.
    Computed 256 then tuned down to 128 for mid-complexity baseline.
13. **CH-13 UI extract (cap=2)** -- double buffer for widget draw list; writer alternates, reader
    reads last completed.
14. **CH-14 Physics events (cap=256)** -- N workers producing collision events, 16/worker/frame
    peak, SafetyMargin=2.0. Computed 256.
15. **CH-15 Physics authority (cap=128)** -- net thread pushes authority snapshots to physics,
    32/frame during rollback, SafetyMargin=4.0. Computed 128.
16. **CH-16 Voice (cap=1024)** -- net thread pushes voice packets, 32/frame across up to 16 talkers,
    SafetyMargin=4.0 for audio quality. Computed 512, rounded 1024.
17. **CH-17 Net save RPC (cap=64)** -- cloud save response queue, 4/frame peak, SafetyMargin=4.0.
    Computed 16, rounded 64.
18. **CH-18 Propagation result (cap=256)** -- audio propagation workers emit results, 8/worker,
    SafetyMargin=3.0 to smooth coalescing. Computed 192, rounded 256.
19. **CH-19 Asset load done (cap=256)** -- main I/O thread signals completed loads, 32/frame peak,
    SafetyMargin=2.0. Computed 64, rounded 256 for burst tolerance.
20. **CH-20 / CH-21 Hot reload (cap=16 each)** -- rare, one at a time.
21. **CH-22 Editor mutation (cap=256)** -- editor bridge queues mutations, 64 ops in a paste/undo
    burst, SafetyMargin=2.0. Computed 128, rounded 256.
22. **CH-23 Profiler sample (cap=2048)** -- every worker emits frame samples, N × 32 × 2.0 = 512,
    bumped to 2048 for a 4-frame history window.
23. **CH-24 Save profile event (cap=32)** -- save-sys emits profiling samples, 4/frame,
    SafetyMargin=2.0.
24. **CH-25 Occlusion raycast (cap=256)** -- audio propagation to BVH workers, 8/worker/frame,
    SafetyMargin=2.0. Computed 128, rounded 256.
25. **CH-26 AI nav query (cap=256)** -- AI navigation workers to physics BVH workers, 16/worker.
26. **CH-27 UI cursor ray (cap=8)** -- UI emits one ray per cursor per frame (~2 cursors in split
    screen).
27. **CH-28 VFX surface query (cap=256)** -- VFX emitters querying surface normal/tangent,
    16/worker.
28. **CH-29 Locale change (cap=16)** -- rare event; small channel with BackPressure so no locale
    change is lost.
29. **CH-30 Collision contact (cap=512)** -- N workers × 32 contacts × 2.0 = 512. Distinct from
    CH-14; this is the raw stream to the audio impact mapper only.
30. **CH-31 Particle spawn (cap=256)** -- animation event -> vfx particle spawn, 8/worker.
31. **CH-32 Draw command triple buffer (cap=3)** -- separate from RenderFrame triple buffer for
    cases where draw commands are computed earlier than other RenderFrame fields.

## Overflow Policy Definitions

| Policy | Action | When to pick |
|--------|--------|-------------|
| `DropOldest` | Discard oldest item; log overflow counter | Events where stale > missing |
| `CoalesceParams` | Merge consecutive updates to same key | Parameter updates (audio, anim) |
| `BackPressure` | Park producer on condvar until drain | Protocol-critical / unbounded work |
| `Overwrite` | Overwrite destination slot unconditionally | Triple / double buffers |

1. **Every channel declares its overflow policy in the table.** Violating the policy is a review
   blocker.
2. **`BackPressure` is the choice of last resort on hot paths** because it stalls workers. It is
   acceptable on save writes, I/O requests, and rare editor operations.
3. **`DropOldest` increments a per-channel overflow counter** reported via the profiler HUD.

## How to Add a New Channel

1. Compute `Capacity = Prod × Burst × Margin`.
2. Round up to the next power of 2.
3. Add a row to the two tables above (the `ID` must not collide with an existing `CH-N`).
4. Add a numbered rationale note describing producers, bursts, and why you picked the margin.
5. Cross-link from the integration doc that introduces the edge.
6. Define the `const CAP` on the channel wrapper type in the subsystem design.

## Review Criteria

| Check | Description |
|-------|------------|
| Formula applied | Capacity matches `Prod × Burst × Margin`, rounded up |
| Overflow policy set | Every row has an explicit policy |
| Rationale documented | A numbered note for each row |
| No stale references | Deleted edges removed from this file, too |
| Tuning knob declared | `const CAP` is a generic param on the wrapper type |
