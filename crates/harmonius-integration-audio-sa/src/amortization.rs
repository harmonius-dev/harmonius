//! Amortization and change detection helpers for propagation scheduling.

use glam::Vec3;

/// Tracks whether a source moved enough to warrant a new trace.
#[derive(Debug, Clone, Copy)]
pub struct SourceTraceState {
    last_position: Vec3,
}

impl SourceTraceState {
    /// Initializes state at `spawn_position`.
    #[must_use]
    pub fn new(spawn_position: Vec3) -> Self {
        Self {
            last_position: spawn_position,
        }
    }

    /// Returns `true` when the source moved more than one centimeter.
    #[must_use]
    pub fn position_changed(&self, new_position: Vec3) -> bool {
        (self.last_position - new_position).length_squared() > 1e-4
    }

    /// Records the last traced position.
    pub fn commit_position(&mut self, new_position: Vec3) {
        self.last_position = new_position;
    }
}

/// Returns `true` when a propagation trace should run for this source.
///
/// First trace always runs (`last_result_frame` is `None`). Subsequent traces run only when the
/// source moves.
#[must_use]
pub fn should_retrace_source(
    last_result_frame: Option<u64>,
    state: &mut SourceTraceState,
    new_position: Vec3,
) -> bool {
    let moved = state.position_changed(new_position);
    if moved {
        state.commit_position(new_position);
        return true;
    }
    if last_result_frame.is_none() {
        state.commit_position(new_position);
        return true;
    }
    false
}

/// Returns `true` when a propagation trace should run for this source/listener pair.
///
/// Combines [`should_retrace_source`] for the emitter with listener movement (same 1 cm epsilon).
/// Initialize `listener_state` at the current listener position before the first call.
#[must_use]
pub fn should_retrace_audio_propagation(
    last_result_frame: Option<u64>,
    source_state: &mut SourceTraceState,
    source_pos: Vec3,
    listener_state: &mut SourceTraceState,
    listener_pos: Vec3,
) -> bool {
    let source_retrace = should_retrace_source(last_result_frame, source_state, source_pos);
    let listener_moved = listener_state.position_changed(listener_pos);
    if listener_moved {
        listener_state.commit_position(listener_pos);
    }
    source_retrace || listener_moved
}

/// Returns how many sources should be traced this frame under `1/N` rotation scheduling.
#[must_use]
pub fn amortized_trace_count(source_count: usize, frame: u64, n: u32) -> usize {
    let n = u64::from(n.max(1));
    (0..source_count)
        .filter(|&idx| (frame + idx as u64).is_multiple_of(n))
        .count()
}
