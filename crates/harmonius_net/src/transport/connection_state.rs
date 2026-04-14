//! Connection lifecycle state machine (application view after QUIC is up).

/// High-level connection lifecycle (design `ConnectionState` subset used in tests).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    /// No connection.
    Disconnected,
    /// QUIC / transport path coming up.
    Connecting,
    /// Application auth in progress.
    Authenticating,
    /// Ready for gameplay traffic.
    Active,
    /// Draining sends.
    Disconnecting,
}

/// Events emitted exactly once per transition in the happy path tests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConnEvent {
    /// Entered connecting.
    ConnConnecting,
    /// Auth succeeded.
    ConnAuthenticated,
    /// Gameplay active.
    ConnActive,
    /// Fully closed.
    ConnDisconnected,
}

/// Minimal driver for the TC-8.1.2.1 sequence.
#[derive(Clone, Debug)]
pub struct ConnectionStateMachine {
    state: ConnectionState,
    log: Vec<ConnEvent>,
}

impl Default for ConnectionStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionStateMachine {
    /// Starts in [`ConnectionState::Disconnected`].
    pub fn new() -> Self {
        Self {
            state: ConnectionState::Disconnected,
            log: Vec::new(),
        }
    }

    /// Current state.
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    /// Ordered transition log for tests.
    pub fn events(&self) -> &[ConnEvent] {
        &self.log
    }

    /// `connect()` from disconnected.
    pub fn connect(&mut self) {
        assert_eq!(self.state, ConnectionState::Disconnected);
        self.state = ConnectionState::Connecting;
        self.log.push(ConnEvent::ConnConnecting);
    }

    /// QUIC finished; move to authenticating.
    pub fn quic_ready(&mut self) {
        assert_eq!(self.state, ConnectionState::Connecting);
        self.state = ConnectionState::Authenticating;
    }

    /// Auth ok.
    pub fn auth_ok(&mut self) {
        assert_eq!(self.state, ConnectionState::Authenticating);
        self.state = ConnectionState::Active;
        self.log.push(ConnEvent::ConnAuthenticated);
        self.log.push(ConnEvent::ConnActive);
    }

    /// Idle then disconnect.
    pub fn disconnect(&mut self) {
        assert_eq!(self.state, ConnectionState::Active);
        self.state = ConnectionState::Disconnecting;
    }

    /// Drain finished.
    pub fn finish_disconnect(&mut self) {
        assert_eq!(self.state, ConnectionState::Disconnecting);
        self.state = ConnectionState::Disconnected;
        self.log.push(ConnEvent::ConnDisconnected);
    }
}

/// Per-connection keepalive tick: O(1) per connection (single counter bump).
#[derive(Clone, Debug)]
pub struct KeepaliveCursor {
    generation: u64,
}

impl Default for KeepaliveCursor {
    fn default() -> Self {
        Self::new()
    }
}

impl KeepaliveCursor {
    /// New cursor at generation zero.
    pub fn new() -> Self {
        Self { generation: 0 }
    }

    /// One tick of work regardless of how many sibling connections exist in the outer loop.
    pub fn tick(&mut self) {
        self.generation = self.generation.wrapping_add(1);
    }

    /// Observed generation (for tests).
    pub fn generation(&self) -> u64 {
        self.generation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    /// TC-8.1.2.1 — ordered lifecycle events.
    #[test]
    fn test_connection_state_machine() {
        let mut c = ConnectionStateMachine::new();
        c.connect();
        c.quic_ready();
        c.auth_ok();
        c.disconnect();
        c.finish_disconnect();
        assert_eq!(
            c.events(),
            &[
                ConnEvent::ConnConnecting,
                ConnEvent::ConnAuthenticated,
                ConnEvent::ConnActive,
                ConnEvent::ConnDisconnected,
            ]
        );
    }

    /// TC-8.1.2.2 — per-connection keepalive work stays O(1); total tick cost scales ~linearly.
    #[test]
    fn test_keepalive_o1_overhead() {
        fn time_ticks(counts: &[usize]) -> Vec<std::time::Duration> {
            let mut out = Vec::new();
            for &n in counts {
                let mut conns: Vec<KeepaliveCursor> = (0..n).map(|_| KeepaliveCursor::new()).collect();
                let t0 = Instant::now();
                for _ in 0..100 {
                    for c in &mut conns {
                        c.tick();
                    }
                }
                out.push(t0.elapsed());
            }
            out
        }
        let d = time_ticks(&[1_000, 5_000, 10_000]);
        let slope_10k_over_1k = d[2].as_secs_f64() / d[0].as_secs_f64();
        let slope_5k_over_1k = d[1].as_secs_f64() / d[0].as_secs_f64();
        assert!(
            slope_5k_over_1k < 7.0 && slope_10k_over_1k < 12.0,
            "expected roughly linear scaling, got 1k->5k={slope_5k_over_1k}, 1k->10k={slope_10k_over_1k}"
        );
    }
}
