//! Event and debug bridges (minimal stubs).

use std::sync::Mutex;

/// User-facing event placeholder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event(pub String);

/// Deferred event sink (thread-safe for parallel graph workers).
#[derive(Debug, Default)]
pub struct EventWriter {
    inner: Mutex<Vec<Event>>,
}

impl EventWriter {
    /// New empty writer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Queue an event.
    pub fn send(&self, event: Event) {
        self.inner.lock().expect("poison").push(event);
    }

    /// Drain queued events (tests).
    pub fn drain(&self) -> Vec<Event> {
        self.inner.lock().expect("poison").drain(..).collect()
    }
}

/// Debug message placeholder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugMsg(pub String);

/// Optional editor / tooling debug channel.
#[derive(Debug, Default)]
pub struct DebugBridge {
    inner: Mutex<Vec<DebugMsg>>,
}

impl DebugBridge {
    /// New bridge.
    pub fn new() -> Self {
        Self::default()
    }

    /// Send debug text.
    pub fn send(&self, msg: DebugMsg) {
        self.inner.lock().expect("poison").push(msg);
    }
}
