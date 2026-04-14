//! Server-side replication coordinator (minimal synchronous stub).

use crossbeam_channel::Receiver;

use crate::delta_tracker::DeltaTracker;
use crate::wire::AckMessage;

/// Minimal replication coordinator: drains ACKs and mirrors debug toggles.
#[derive(Debug)]
pub struct ReplicationSystem {
    ack_rx: Receiver<AckMessage>,
    /// Effective overlay after the last [`Self::tick`].
    debug_overlay_enabled: bool,
    /// Queued overlay flip from [`Self::set_debug_overlay`], applied on the next tick.
    pending_debug_overlay: Option<bool>,
    debug_log_deltas: bool,
    debug_force_resync: bool,
}

impl ReplicationSystem {
    /// Creates a system bound to `ack_rx`.
    #[must_use]
    pub fn new(ack_rx: Receiver<AckMessage>) -> Self {
        Self {
            ack_rx,
            debug_overlay_enabled: false,
            pending_debug_overlay: None,
            debug_log_deltas: false,
            debug_force_resync: false,
        }
    }

    /// Queues the replication debug overlay; effective after the next [`Self::tick`].
    pub fn set_debug_overlay(&mut self, enabled: bool) {
        self.pending_debug_overlay = Some(enabled);
    }

    /// Current overlay toggle (after pending changes are applied in [`Self::tick`]).
    #[must_use]
    pub fn debug_overlay_enabled(&self) -> bool {
        self.debug_overlay_enabled
    }

    /// Sets `debug.replication.log_deltas` (design debug toggles).
    pub fn set_debug_log_deltas(&mut self, enabled: bool) {
        self.debug_log_deltas = enabled;
    }

    /// Returns whether delta logging is enabled.
    #[must_use]
    pub fn debug_log_deltas(&self) -> bool {
        self.debug_log_deltas
    }

    /// Sets `debug.replication.force_resync` (design debug toggles).
    pub fn set_debug_force_resync(&mut self, enabled: bool) {
        self.debug_force_resync = enabled;
    }

    /// Returns whether forced resync is enabled.
    #[must_use]
    pub fn debug_force_resync(&self) -> bool {
        self.debug_force_resync
    }

    /// One replication tick: applies queued debug toggles, then drains ACKs into `tracker`.
    #[must_use]
    pub fn tick(&mut self, tracker: &mut DeltaTracker) -> u32 {
        if let Some(enabled) = self.pending_debug_overlay.take() {
            self.debug_overlay_enabled = enabled;
        }
        self.drain_acks(tracker)
    }

    /// Drains pending ACKs and advances baselines in `tracker`.
    pub fn drain_acks(&mut self, tracker: &mut DeltaTracker) -> u32 {
        let mut count = 0u32;
        while let Ok(msg) = self.ack_rx.try_recv() {
            tracker.advance_baseline(msg.client, msg.tick);
            count = count.saturating_add(1);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::ConnectionId;
    use crate::ids::Entity;
    use crate::ids::SequenceTick;
    use crossbeam_channel::bounded;

    #[test]
    fn tc_ir_4_4_8_n2_ack_overflow_is_try_send_bounded() {
        let (tx, rx) = bounded::<AckMessage>(256);
        let mut sent = 0u32;
        let mut dropped = 0u32;
        for i in 0..512 {
            match tx.try_send(AckMessage {
                client: ConnectionId(1),
                tick: SequenceTick(i),
            }) {
                Ok(()) => sent = sent.saturating_add(1),
                Err(_) => dropped = dropped.saturating_add(1),
            }
        }
        assert!(dropped > 0, "expected bounded channel drops under flood");
        let mut system = ReplicationSystem::new(rx);
        let mut tracker = DeltaTracker::new();
        let drained = system.drain_acks(&mut tracker);
        assert_eq!(drained, sent);
    }

    #[test]
    fn tc_ir_4_4_8_5_debug_toggle_applies_on_next_tick() {
        let (_tx, rx) = bounded::<AckMessage>(1);
        let mut system = ReplicationSystem::new(rx);
        let mut tracker = DeltaTracker::new();
        assert!(!system.debug_overlay_enabled());
        system.set_debug_overlay(true);
        assert!(
            !system.debug_overlay_enabled(),
            "overlay flips apply on tick per TC-IR-4.4.8.5"
        );
        assert_eq!(system.tick(&mut tracker), 0);
        assert!(system.debug_overlay_enabled());
    }

    #[test]
    fn drain_acks_advances_baseline() {
        let (tx, rx) = bounded(8);
        let mut system = ReplicationSystem::new(rx);
        let mut tracker = DeltaTracker::new();
        let client = ConnectionId(3);
        let live = vec![7u8, 8, 9];
        tracker
            .compute_delta(client, Entity(1), SequenceTick(1), &live)
            .expect("delta");
        tx.send(AckMessage {
            client,
            tick: SequenceTick(200),
        })
        .expect("send");
        assert_eq!(system.tick(&mut tracker), 1);
        assert_eq!(tracker.baseline_tick(client), Some(SequenceTick(200)));
    }
}
