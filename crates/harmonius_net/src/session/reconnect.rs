//! Reconnect grace handling (TC-8.5.5.*).

use std::collections::HashMap;
use std::time::Duration;

use super::types::{PlayerId, PlayerSnapshot, SessionError, SessionRestored};

#[derive(Clone, Debug)]
pub struct ReconnectHandler {
    grace: Duration,
    snapshots: HashMap<PlayerId, PlayerSnapshot>,
    disconnect_at: HashMap<PlayerId, u64>,
    now_fn: fn() -> u64,
}

impl ReconnectHandler {
    pub fn new(grace: Duration, now_fn: fn() -> u64) -> Self {
        Self {
            grace,
            snapshots: HashMap::new(),
            disconnect_at: HashMap::new(),
            now_fn,
        }
    }

    pub fn start_grace(&mut self, player: PlayerId, snap: PlayerSnapshot) {
        let t = (self.now_fn)();
        self.disconnect_at.insert(player, t);
        self.snapshots.insert(player, snap);
    }

    pub fn reconnect(
        &self,
        player: PlayerId,
    ) -> Result<SessionRestored, SessionError> {
        let t0 = self.disconnect_at.get(&player).copied().ok_or(SessionError::UnknownPlayer)?;
        let now = (self.now_fn)();
        let elapsed = Duration::from_secs(now.saturating_sub(t0));
        if elapsed > self.grace {
            return Err(SessionError::GraceExpired);
        }
        self.snapshots.get(&player).ok_or(SessionError::UnknownPlayer)?;
        Ok(SessionRestored { player_id: player })
    }

    pub fn snapshot(&self, player: PlayerId) -> Option<&PlayerSnapshot> {
        self.snapshots.get(&player)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::super::types::{Health, Transform};
    use super::*;

    thread_local! {
        static NOW: Cell<u64> = Cell::new(0);
    }

    fn now() -> u64 {
        NOW.with(|c| c.get())
    }

    fn set(t: u64) {
        NOW.with(|c| c.set(t));
    }

    fn sample_snap() -> PlayerSnapshot {
        PlayerSnapshot {
            transform: Transform {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            health: Health { current: 80, max: 100 },
            buffs: vec!["shield".into()],
        }
    }

    /// TC-8.5.5.1
    #[test]
    fn test_reconnect_within_grace() {
        set(0);
        let mut rc = ReconnectHandler::new(Duration::from_secs(120), now);
        let pid = PlayerId(42);
        let snap = sample_snap();
        rc.start_grace(pid, snap.clone());
        set(10);
        let r = rc.reconnect(pid).expect("restored");
        assert_eq!(r.player_id, pid);
        assert_eq!(rc.snapshot(pid), Some(&snap));
    }

    /// TC-8.5.5.2
    #[test]
    fn test_reconnect_after_grace() {
        set(0);
        let mut rc = ReconnectHandler::new(Duration::from_secs(120), now);
        let pid = PlayerId(7);
        rc.start_grace(pid, sample_snap());
        set(200);
        let e = rc.reconnect(pid).unwrap_err();
        assert_eq!(e, SessionError::GraceExpired);
    }
}
