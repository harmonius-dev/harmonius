//! Runtime-toggleable trace for AI ↔ data-table integration.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

/// ECS resource toggling verbose integration tracing.
#[derive(Debug)]
pub struct AiDataTraceFlag {
    enabled: AtomicBool,
    notes: Mutex<Vec<String>>,
}

impl Default for AiDataTraceFlag {
    fn default() -> Self {
        Self::new()
    }
}

impl AiDataTraceFlag {
    /// Creates a disabled trace flag with an empty buffer.
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            notes: Mutex::new(Vec::new()),
        }
    }

    /// Runtime toggle (never `cfg`-gated).
    pub fn set_enabled(&self, on: bool) {
        self.enabled.store(on, Ordering::Relaxed);
    }

    /// Current toggle state.
    pub fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Records a trace line when enabled.
    pub fn emit(&self, line: impl Into<String>) {
        if self.enabled.load(Ordering::Relaxed) {
            if let Ok(mut g) = self.notes.lock() {
                g.push(line.into());
            }
        }
    }

    /// Drains buffered notes (tests).
    pub fn drain_notes(&self) -> Vec<String> {
        self.notes
            .lock()
            .map(|mut g| std::mem::take(&mut *g))
            .unwrap_or_default()
    }
}
