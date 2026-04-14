//! Event and metric types for save-system ↔ profiler integration (IR-8.1.x).

/// Canonical save pipeline phase identifiers for profile samples.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SavePhase {
    /// World gather / snapshot work.
    Gather,
    /// Archive serialization.
    Archive,
    /// Compression stage.
    Compress,
    /// Serialized write hand-off (not counted against Phase 8 budget when present).
    Write,
    /// Save correlation closed for this `save_id`.
    Finalize,
}

/// One timed slice of save work on the save worker.
#[derive(Clone, Debug, PartialEq)]
pub struct SaveProfileEvent {
    /// Correlates all events for one save operation.
    pub save_id: u64,
    /// Which slice of the pipeline completed.
    pub phase: SavePhase,
    /// Engine tick when the slice started (monotonic, test harness may use small integers).
    pub start_tick: u64,
    /// Wall time spent in this slice, milliseconds.
    pub duration_ms: f64,
    /// Bytes attributed to this slice (often zero except archive/compress).
    pub bytes: u64,
}

/// High-level I/O outcome for a completed platform write.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoError {
    /// Write completed successfully.
    None,
    /// Syscall interrupted; retried by platform layer.
    Interrupted,
    /// No space left on device.
    DiskFull,
    /// Permission denied.
    Permission,
    /// Unclassified error.
    Unknown,
}

/// Main-thread file write completion correlated back to a save.
#[derive(Clone, Debug, PartialEq)]
pub struct IoWriteEvent {
    /// Matches [`SaveProfileEvent::save_id`].
    pub save_id: u64,
    /// Correlates with `IoRequest` bookkeeping in the I/O layer.
    pub request_id: u64,
    /// Measured write duration in milliseconds.
    pub duration_ms: f64,
    /// Bytes reported by the platform layer.
    pub bytes_written: u64,
    /// Error classification for HUD banners.
    pub error: IoError,
}

/// Resident-set style memory snapshot around archive construction.
#[derive(Clone, Debug, PartialEq)]
pub struct MemorySnapshotEvent {
    /// Matches [`SaveProfileEvent::save_id`].
    pub save_id: u64,
    /// Working-set estimate before archive work.
    pub rss_before: u64,
    /// Working-set estimate after archive work.
    pub rss_after: u64,
    /// Peak observed while archive ran.
    pub peak_during: u64,
}

/// Stable identifier for a top-level schema subtree in the save archive.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SchemaNodeId(pub u32);

/// One subtree contribution to archive size for HUD sunburst views.
#[derive(Clone, Debug, PartialEq)]
pub struct SchemaBreakdownEvent {
    /// Matches [`SaveProfileEvent::save_id`].
    pub save_id: u64,
    /// Subtree key (player, world, ai, …).
    pub node: SchemaNodeId,
    /// Serialized subtree size in bytes.
    pub bytes: u64,
    /// Entities counted under this subtree.
    pub entity_count: u32,
}

/// Rolling counters surfaced on the profiler HUD.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SaveMetrics {
    /// Successful save completions.
    pub total_saves: u64,
    /// Saves that ended in failure (I/O or pipeline).
    pub total_failures: u64,
    /// Mean of completed-save durations in the rolling window.
    pub avg_duration_ms: f64,
    /// Approximate 99th percentile in the rolling window.
    pub p99_duration_ms: f64,
    /// Last platform or pipeline error code for quick HUD readout.
    pub last_error_code: u64,
}

/// Runtime HUD toggles referenced by integration tests (FM-4).
#[derive(Clone, Debug)]
pub struct DebugFlags {
    /// When false, the save pipeline must not emit profiler events.
    pub show_profiler_hud: bool,
}

impl Default for DebugFlags {
    fn default() -> Self {
        Self {
            show_profiler_hud: true,
        }
    }
}

/// Unified envelope on CH-24 (cap 32, DropOldest).
#[derive(Clone, Debug, PartialEq)]
pub enum ProfileMessage {
    /// Save worker timing slice.
    SaveProfile(SaveProfileEvent),
    /// Main-thread write completion.
    IoWrite(IoWriteEvent),
    /// Memory snapshot around archive.
    Memory(MemorySnapshotEvent),
    /// Per-subtree archive size.
    Schema(SchemaBreakdownEvent),
}
