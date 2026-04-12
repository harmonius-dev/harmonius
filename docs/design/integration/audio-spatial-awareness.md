# Audio ↔ Spatial Awareness Integration Design

## Systems Involved

| System | Design | Domain |
|--------|--------|--------|
| Audio | [audio.md](../audio/audio.md) | Audio |
| Spatial Awareness | [spatial-awareness.md](../simulation/spatial-awareness.md) | Simulation |

## Integration Requirements

| ID | Requirement | Systems |
|----|-------------|---------|
| IR-1.9.1 | Shared BVH provides occlusion rays | SA, Audio |
| IR-1.9.2 | Acoustic materials on BVH surfaces | SA, Audio |
| IR-1.9.3 | Propagation results feed spatial audio | SA, Audio |
| IR-1.9.4 | Obstruction detection via ray count | SA, Audio |
| IR-1.9.5 | Change detection amortizes ray tracing | SA, Audio |

1. **IR-1.9.1** -- The audio propagation solver casts occlusion rays through the shared BVH spatial
   index (F-1.9.1). Each ray tests line-of-sight between source and listener, accumulating surface
   hits for absorption calculations.
2. **IR-1.9.2** -- BVH surface hits return the hit entity. The propagation system queries
   `AcousticMaterial` from that entity's components (absorption, transmission loss, scattering per
   frequency band) to compute per-band attenuation.
3. **IR-1.9.3** -- `PropagationResult` per source (occlusion factor, early reflections, reverb
   contribution) is written to per-entity slots in `PropagationResultStore`, then sent to the audio
   thread via bounded MPSC crossbeam-channel. The audio thread drains the channel at each buffer
   callback to update its `PropagationSnapshot`.
4. **IR-1.9.4** -- Obstruction is quantified by the fraction of `SpatialAudio.occlusion_rays` that
   are blocked. Full occlusion applies maximum low-pass filtering; partial occlusion interpolates.
5. **IR-1.9.5** -- Only sources or listeners with `Changed<Transform>` are re-traced. Static sources
   cache their `PropagationResult`. Amortized tracing rotates 1/N sources per frame (N=4 at 60 fps
   yields 15 Hz update per source). Newly spawned sources default to line-of-sight (occlusion=1.0,
   zero band loss) until their first trace completes. Worst-case latency for a new source is N
   frames (4 frames / 67 ms at 60 fps) before the first propagation update.

## Data Contracts

| Type | Defined in | Consumed by | Purpose |
|------|-----------|-------------|---------|
| `SharedSpatialIndex` | Core Runtime | Audio | BVH queries |
| `AcousticMaterial` | Audio | SA (surface) | Absorption |
| `AcousticMaterialTable` | Audio | Propagation | Lookup |
| `SpatialAudio` | Audio | SA (ray count) | Config |
| `PropagationResult` | Audio | Audio thread | Filtering |
| `PropagationResultStore` | Audio | Bridge | Per-entity |
| `PropagationSnapshot` | Audio | Audio thread | Snapshot |
| `AudioPropagationSender` | Audio | Workers | MPSC tx |
| `AudioPropagationReceiver` | Audio | Audio thread | MPSC rx |

The shared BVH (for AI, audio, and gameplay queries) is distinct from the physics-private BVH (for
broadphase collision and raycasts). Surface hits from the shared BVH return the entity that owns the
surface. The propagation system then queries `AcousticMaterial` from that entity's components.
`PhysicsMaterialHandle` is not used here -- it belongs to the physics-private BVH.

```rust
/// Result of propagation tracing for one source.
/// Written by worker threads into per-entity
/// slots, then drained to the audio thread via
/// a bounded MPSC channel.
pub struct PropagationResult {
    /// Source entity this result belongs to.
    pub source: Entity,
    /// 0.0 = fully occluded, 1.0 = line of sight.
    pub occlusion: f32,
    /// Per-band transmission loss (low, mid, high).
    pub band_loss: [f32; 3],
    /// Early reflection taps (delay + gain pairs).
    /// Fixed-size array avoids heap allocation on
    /// the audio thread read path. Excess taps are
    /// dropped by energy (lowest gain first).
    pub reflections: [ReflectionTap; 8],
    /// Number of valid entries in `reflections`.
    pub reflection_count: u8,
    /// Reverb send level derived from geometry.
    pub reverb_send: f32,
    /// Frame number when last updated.
    pub last_updated_frame: u64,
}

pub struct ReflectionTap {
    pub delay_ms: f32,
    pub gain: f32,
    pub direction: Vec3,
}

/// Per-entity partitioned storage for propagation
/// results. Each entity slot is independent, so
/// `par_for_each` writes to disjoint slots without
/// contention. Interior mutability via `UnsafeCell`
/// is safe because the job system guarantees each
/// entity is processed by exactly one worker.
pub struct PropagationResultStore {
    /// Indexed by entity index. Each slot is
    /// written by exactly one worker thread.
    slots: Vec<UnsafeCell<PropagationResult>>,
}

/// MPSC channel pair for bridging ECS workers to
/// the audio thread. Bounded to 256 entries to
/// prevent unbounded growth. If the channel is
/// full, the oldest result for that source is
/// stale but acceptable at 15 Hz update rate.
///
/// Arc is used here for the receiver endpoint
/// only because it is shared immutable data (the
/// channel itself is lock-free; Arc just shares
/// the allocation). This is the sole permitted
/// Arc usage in this integration.
pub type AudioPropagationSender =
    crossbeam_channel::Sender<PropagationResult>;
pub type AudioPropagationReceiver =
    crossbeam_channel::Receiver<PropagationResult>;

/// Snapshot of all propagation results, owned by
/// the audio thread. Updated by draining the MPSC
/// channel at each buffer callback.
pub struct PropagationSnapshot {
    /// Indexed by source voice ID. Updated by
    /// draining the MPSC receiver each callback.
    results: Vec<PropagationResult>,
}

/// System that traces propagation rays through
/// the shared BVH (not the physics-private BVH).
/// Runs par_for_each on worker threads during
/// Phase 3. Writes to per-entity slots in
/// PropagationResultStore, then sends updated
/// results to the audio thread via bounded MPSC.
pub fn audio_propagation_system(
    sources: Query<(
        Entity,
        &AudioSource,
        &SpatialAudio,
        &GlobalTransform,
    ), Changed<GlobalTransform>>,
    listeners: Query<(
        &AudioListener,
        &GlobalTransform,
    )>,
    spatial_index: Res<SharedSpatialIndex>,
    materials: Res<AcousticMaterialTable>,
    result_store: Res<PropagationResultStore>,
    sender: Res<AudioPropagationSender>,
    frame: Res<FrameCount>,
);
```

## Data Flow

```mermaid
sequenceDiagram
    participant Src as AudioSource
    participant Sys as PropagationSystem (Phase 3)
    participant BVH as SharedSpatialIndex (shared)
    participant AM as AcousticMaterial
    participant PRS as PropagationResultStore
    participant MPSC as MPSC Channel (bounded 256)
    participant AT as Audio Thread
    participant Snap as PropagationSnapshot
    participant Voice as Voice DSP

    Src->>Sys: Changed<Transform> triggers trace
    Sys->>BVH: cast occlusion rays
    BVH-->>Sys: surface hits + entity refs
    Sys->>AM: lookup absorption per band
    Sys->>PRS: write per-entity slot
    Sys->>MPSC: send PropagationResult
    Note over AT: Next buffer callback
    AT->>MPSC: drain all pending results
    AT->>Snap: update PropagationSnapshot
    AT->>Voice: apply occlusion filter
    AT->>Voice: apply reflection taps
    AT->>Voice: set reverb send level
```

## Timing and Ordering

| System | Phase | Timestep | Order |
|--------|-------|----------|-------|
| Propagation trace | 3-Simulation | Variable | Workers |
| Result write | 3-Simulation | Variable | After trace |
| MPSC send | 3-Simulation | Variable | After write |
| Audio thread read | Dedicated | Real-time | MPSC drain |

Propagation tracing runs on worker threads during Phase 3 using `par_for_each`. Each source's trace
is independent, enabling parallel execution across all workers. Results are written to per-entity
slots in `PropagationResultStore`, then sent to the audio thread via bounded MPSC crossbeam-channel.

The audio thread drains the MPSC channel at each buffer callback, updating its local
`PropagationSnapshot`. No mutex or lock contention between worker and audio threads. The
crossbeam-channel is lock-free internally.

Amortized schedule: with N=4 rotation and 100 sources at 60 fps, each source updates at 15 Hz.
Propagation changes are slow (geometry is static or moves slowly), so 15 Hz is imperceptible. Newly
spawned sources use line-of-sight defaults until their first trace (worst case 4 frames / 67 ms).

## Failure Modes

| Failure | Impact | Fallback |
|---------|--------|----------|
| BVH not ready | No occlusion data | LOS default (1) |
| Material missing | Wrong absorption | Default stone (2) |
| Stale result | Old filtering | Accept at 15 Hz (3) |
| All rays blocked | Full occlusion | Max LP filter (4) |
| MPSC full | Result dropped | Keep stale (5) |
| New source | No trace yet | LOS default (6) |
| Entity despawned | Orphan result | Discard on drain (7) |

1. **BVH not ready** -- shared BVH is unavailable during first frame or rebuild. Propagation system
   skips tracing; audio thread uses line-of-sight defaults (occlusion=1.0, zero band loss, zero
   reverb send). Sound plays at full volume without spatial filtering.
2. **Material missing** -- entity hit by ray has no `AcousticMaterial` component. Propagation system
   substitutes a default stone material (absorption=[0.02, 0.03, 0.04], transmission
   loss=[40, 45, 50] dB, scattering=0.1).
3. **Stale result** -- audio thread has not received an update for a source within the last N
   frames. Acceptable because propagation changes are slow (geometry is static or moves slowly). The
   `last_updated_frame` field lets the audio thread detect staleness for debugging.
4. **All rays blocked** -- every occlusion ray is blocked. Propagation system sets occlusion=0.0 and
   applies maximum low-pass filter cutoff.
5. **MPSC full** -- bounded channel (256 entries) is full. The `try_send` fails and the result is
   dropped. The audio thread keeps the previous result for that source. Acceptable at 15 Hz.
6. **New source** -- newly spawned source has no propagation result yet. Audio thread uses
   line-of-sight defaults (same as BVH-not-ready). First trace arrives within N frames (worst case 4
   frames / 67 ms at 60 fps).
7. **Entity despawned** -- a result arrives for a source that no longer exists. Audio thread
   discards the result during MPSC drain when the voice ID is not found in the active voice table.

## Platform Considerations

None -- identical across all platforms. BVH queries and acoustic material lookups are pure CPU
operations. The MPSC crossbeam-channel is portable across all targets. Audio thread platform
differences are behind `AudioBackend`.

## Test Plan

See companion [audio-spatial-awareness-test-cases.md](audio-spatial-awareness-test-cases.md).

## Review Feedback

1. **[CONFIDENT]** The constraints mandate that ECS-to-audio-thread communication uses a lock-free
   SPSC command queue (`constraints.md` Architecture / ECS-primary), but the design uses "atomic
   pointer swap" on a double buffer with no mention of SPSC. Clarify whether the double-buffer swap
   IS the SPSC channel or replace with the canonical SPSC pattern.

2. **[CONFIDENT]** The `audio_propagation_system` signature uses `ResMut<PropagationResultStore>`,
   which is an exclusive mutable borrow. If worker threads write to this via `par_for_each`,
   interior mutability or per-entity partitioned storage is needed. `ResMut` conflicts with parallel
   writes unless it wraps an internally-partitioned structure -- document the partitioning strategy
   explicitly.

3. **[CONFIDENT]** The constraints require 2D/2.5D/3D support (constraints.md "First-class 2.5D"),
   yet the design only references `Vec3`, `GlobalTransform`, and 3D BVH. Add coverage for 2D spatial
   audio (2D BVH, `Transform2D`, distance attenuation in 2D).

4. **[CONFIDENT]** The Timing table labels the audio thread read as "Async swap", but the
   constraints forbid async/await in engine runtime. The word "async" is misleading here. Rename to
   "Lock-free swap" or "Atomic swap" to avoid confusion.

5. **[CONFIDENT]** `SmallVec<[ReflectionTap; 8]>` is heap-backed when it exceeds 8 entries. Since
   `PropagationResult` is shared across thread boundaries via atomic swap, document the allocation
   strategy (arena per frame, fixed cap, etc.) to avoid heap allocation on the audio thread's read
   path.

6. **[CONFIDENT]** The constraints specify that all inter-thread communication uses
   crossbeam-channel and prohibit shared mutable state and mutexes. The double-buffer atomic pointer
   swap pattern is neither a crossbeam-channel nor an SPSC queue. Reconcile with the constraint or
   justify the exception.

7. **[UNCERTAIN]** `PhysicsMaterialHandle` is listed as the return type from BVH surface hits, but
   constraints say the physics BVH is private and not shared. The shared BVH is for AI, audio, and
   gameplay. Verify that shared BVH surfaces carry `PhysicsMaterialHandle` or define a separate
   `AcousticSurfaceHandle`.

8. **[CONFIDENT]** The test cases cover all five IRs (IR-1.9.1 through IR-1.9.5) with 14 functional
   tests and 3 benchmarks. Coverage is adequate.

9. **[CONFIDENT]** No test case covers the 2D/2.5D audio propagation path. Add test cases for 2D BVH
   ray tracing and 2D distance attenuation to match the 2.5D constraint.

10. **[UNCERTAIN]** The amortized rotation (N=4, 25 sources per frame at 100 total) means a newly
    spawned source may wait up to 4 frames before its first trace. Consider adding a test case for
    first-frame behavior of new sources (e.g., default to full occlusion or line-of-sight until
    traced).

11. **[CONFIDENT]** The Data Contracts table lists `PropagationResult` consumed by "Audio thread",
    but the Rust pseudocode stores results in `ResMut<PropagationResultStore>` which is an ECS
    resource. ECS resources are not directly accessible from the dedicated audio thread. Document
    the bridging mechanism (SPSC queue, mapped memory, etc.) between the ECS world and the audio
    thread.

12. **[CONFIDENT]** The design has all required sections (Systems Involved, Integration
    Requirements, Data Contracts, Data Flow, Timing and Ordering, Failure Modes, Platform
    Considerations, Test Plan) and includes Rust pseudocode and a Mermaid sequence diagram.
    Structurally complete.
