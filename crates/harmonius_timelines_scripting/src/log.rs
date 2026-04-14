//! Test-visible warning hooks (FM-1, FM-6, FM-7).

use std::sync::{Mutex, OnceLock};

use crate::ids::GraphId;

static WARNINGS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn buffer() -> &'static Mutex<Vec<String>> {
    WARNINGS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Clears captured warnings (call at test start).
pub fn clear_warnings() {
    if let Ok(mut g) = buffer().lock() {
        g.clear();
    }
}

/// Records a warning string for assertions.
pub fn warn(message: impl Into<String>) {
    if let Ok(mut g) = buffer().lock() {
        g.push(message.into());
    }
}

/// FM-1: missing graph entry for timeline event.
pub fn warn_missing_timeline_entry(graph_id: GraphId) {
    warn(format!("missing on_timeline_event for graph_id={}", graph_id.0));
}

/// FM-6: MPSC channel full.
pub fn warn_channel_full(channel: &'static str) {
    warn(format!("mpsc full: {channel}"));
}

/// FM-7: slot dropped after hot reload.
pub fn warn_dropped_slot_after_reload(slot: crate::ids::SlotId) {
    warn(format!("dropped slot after reload: {}", slot.0));
}

/// Returns captured warnings (newest last).
pub fn take_warnings() -> Vec<String> {
    buffer()
        .lock()
        .map_or_else(|_| Vec::new(), |mut g| std::mem::take(&mut *g))
}
