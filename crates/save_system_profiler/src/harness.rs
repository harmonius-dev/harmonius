//! Deterministic fakes for CI-runnable integration tests.

use crate::channel::SaveProfileChannel;
use crate::types::{
    DebugFlags, IoError, IoWriteEvent, MemorySnapshotEvent, ProfileMessage, SavePhase,
    SaveProfileEvent, SchemaBreakdownEvent, SchemaNodeId,
};

/// In-memory writer that reports simulated latency and byte counts.
#[derive(Debug)]
pub struct FakeFileSystem {
    /// Duration returned for the last write (ms).
    pub last_write_duration_ms: f64,
    /// Bytes reported for the last write.
    pub last_bytes_written: u64,
    /// Error injected for the next write.
    pub next_error: IoError,
}

impl Default for FakeFileSystem {
    fn default() -> Self {
        Self {
            last_write_duration_ms: 0.0,
            last_bytes_written: 0,
            next_error: IoError::None,
        }
    }
}

impl FakeFileSystem {
    /// Simulates a platform write and returns the profiler-facing completion event.
    pub fn complete_write(
        &mut self,
        save_id: u64,
        request_id: u64,
        duration_ms: f64,
        bytes_written: u64,
    ) -> IoWriteEvent {
        self.last_write_duration_ms = duration_ms;
        self.last_bytes_written = bytes_written;
        let error = self.next_error;
        IoWriteEvent {
            save_id,
            request_id,
            duration_ms,
            bytes_written,
            error,
        }
    }
}

/// Emits profile messages for a single save when the HUD allows profiling.
#[derive(Debug)]
pub struct SavePipeline<'a> {
    /// Target channel (CH-24).
    pub channel: &'a mut SaveProfileChannel,
    /// Runtime HUD toggle (FM-4).
    pub flags: &'a DebugFlags,
}

impl<'a> SavePipeline<'a> {
    /// Builds a pipeline bound to `channel` and `flags`.
    pub fn new(channel: &'a mut SaveProfileChannel, flags: &'a DebugFlags) -> Self {
        Self { channel, flags }
    }

    fn emit(&mut self, msg: ProfileMessage) {
        if !self.flags.show_profiler_hud {
            return;
        }
        self.channel.send(msg);
    }

    /// Emits a gather-phase slice (TC-IR-8.1.1.1).
    pub fn gather(&mut self, save_id: u64, start_tick: u64, duration_ms: f64) {
        self.emit(ProfileMessage::SaveProfile(SaveProfileEvent {
            save_id,
            phase: SavePhase::Gather,
            start_tick,
            duration_ms,
            bytes: 0,
        }));
    }

    /// Emits an archive slice with byte attribution (TC-IR-8.1.1.2).
    pub fn archive(&mut self, save_id: u64, start_tick: u64, duration_ms: f64, bytes: u64) {
        self.emit(ProfileMessage::SaveProfile(SaveProfileEvent {
            save_id,
            phase: SavePhase::Archive,
            start_tick,
            duration_ms,
            bytes,
        }));
    }

    /// Emits a compress slice (TC-IR-8.1.1.3).
    pub fn compress(&mut self, save_id: u64, start_tick: u64, duration_ms: f64, bytes: u64) {
        self.emit(ProfileMessage::SaveProfile(SaveProfileEvent {
            save_id,
            phase: SavePhase::Compress,
            start_tick,
            duration_ms,
            bytes,
        }));
    }

    /// Emits write hand-off timing (not always counted against Phase 8).
    pub fn write(&mut self, save_id: u64, start_tick: u64, duration_ms: f64, bytes: u64) {
        self.emit(ProfileMessage::SaveProfile(SaveProfileEvent {
            save_id,
            phase: SavePhase::Write,
            start_tick,
            duration_ms,
            bytes,
        }));
    }

    /// Emits finalize marker (TC-IR-8.1.1.4).
    pub fn finalize(&mut self, save_id: u64, start_tick: u64, duration_ms: f64) {
        self.emit(ProfileMessage::SaveProfile(SaveProfileEvent {
            save_id,
            phase: SavePhase::Finalize,
            start_tick,
            duration_ms,
            bytes: 0,
        }));
    }

    /// Emits main-thread I/O completion (TC-IR-8.1.2.x).
    pub fn io_write(&mut self, ev: IoWriteEvent) {
        self.emit(ProfileMessage::IoWrite(ev));
    }

    /// Emits memory snapshot (TC-IR-8.1.3.x).
    pub fn memory(&mut self, ev: MemorySnapshotEvent) {
        self.emit(ProfileMessage::Memory(ev));
    }

    /// Emits schema breakdown (TC-IR-8.1.6.x).
    pub fn schema(&mut self, ev: SchemaBreakdownEvent) {
        self.emit(ProfileMessage::Schema(ev));
    }
}

/// Drains every queued message into a vector (test helper).
#[must_use]
pub fn drain_messages(channel: &mut SaveProfileChannel) -> Vec<ProfileMessage> {
    let mut out = Vec::new();
    while let Some(m) = channel.try_recv() {
        out.push(m);
    }
    out
}

/// Player subtree key used in schema tests.
pub const NODE_PLAYER: SchemaNodeId = SchemaNodeId(1);
/// World subtree key used in schema tests.
pub const NODE_WORLD: SchemaNodeId = SchemaNodeId(2);
/// AI subtree key used in schema tests.
pub const NODE_AI: SchemaNodeId = SchemaNodeId(3);
