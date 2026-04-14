//! Rich presence throttling (`R-14.5.3`).

use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Presence payload fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresenceState {
    /// Human-readable activity string.
    pub activity: String,
    /// Optional zone or map id.
    pub zone: Option<String>,
    /// Optional party size.
    pub party_size: Option<u32>,
}

/// Throttle / transport failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresenceError {
    /// Update ignored due to throttle window.
    Throttled,
}

/// Throttles presence updates to one network publish per interval.
#[derive(Debug)]
pub struct RichPresenceService {
    inner: Mutex<Inner>,
}

#[derive(Debug)]
struct Inner {
    latest: Option<PresenceState>,
    last_emit: Option<Instant>,
    interval: Duration,
}

impl RichPresenceService {
    /// Creates a service with a 15 s throttle window.
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Inner {
                latest: None,
                last_emit: None,
                interval: Duration::from_secs(15),
            }),
        }
    }

    /// Publishes `state`, applying throttle rules.
    pub fn update(&self, state: PresenceState) -> Result<(), PresenceError> {
        let mut g = self.inner.lock().expect("presence mutex poisoned");
        let now = Instant::now();
        if let Some(t) = g.last_emit {
            if now.duration_since(t) < g.interval {
                g.latest = Some(state);
                return Err(PresenceError::Throttled);
            }
        }
        g.latest = Some(state);
        g.last_emit = Some(now);
        Ok(())
    }

    /// Clears presence state.
    pub fn clear(&self) -> Result<(), PresenceError> {
        let mut g = self.inner.lock().expect("presence mutex poisoned");
        g.latest = None;
        g.last_emit = None;
        Ok(())
    }

    /// Returns the latest logical presence (includes throttled-but-queued updates).
    pub fn current(&self) -> Option<PresenceState> {
        self.inner
            .lock()
            .expect("presence mutex poisoned")
            .latest
            .clone()
    }
}

impl Default for RichPresenceService {
    fn default() -> Self {
        Self::new()
    }
}
