//! Lag-compensation helpers for hitbox rewind.

use crate::buffers::HitboxBuffer;
use crate::snapshot_types::HitboxSnapshot;

/// Computes rewind ticks and queries historical hitboxes for server-side validation.
#[derive(Clone, Copy, Debug, Default)]
pub struct HistoryRewinder;

impl HistoryRewinder {
    /// Computes the server tick to rewind to from the client's tick and RTT in ticks.
    #[must_use]
    pub fn compute_rewind_tick(client_tick: u64, rtt_ticks: u64) -> u64 {
        client_tick.saturating_sub(rtt_ticks)
    }

    /// Returns every hitbox snapshot stored at `tick`.
    #[must_use]
    pub fn rewind(buffer: &HitboxBuffer, tick: u64) -> Vec<&HitboxSnapshot> {
        buffer.snapshots_at_tick(tick)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;
    use crate::snapshot_types::ColliderShape;
    use glam::{Quat, Vec3};

    #[test]
    fn tc_ir_4_5_5_1_rewind_tick_in_past() {
        assert_eq!(HistoryRewinder::compute_rewind_tick(100, 6), 94);
    }

    #[test]
    fn rewind_collects_all_hitboxes_at_tick() {
        let mut buf = HitboxBuffer::new();
        let e0 = Entity::new(0, 0);
        let e1 = Entity::new(1, 0);
        buf.push(HitboxSnapshot::new(
            10,
            e0,
            Vec3::ZERO,
            Quat::IDENTITY,
            ColliderShape::Sphere { radius: 0.5 },
        ));
        buf.push(HitboxSnapshot::new(
            10,
            e1,
            Vec3::X,
            Quat::IDENTITY,
            ColliderShape::Sphere { radius: 0.5 },
        ));
        let snaps = HistoryRewinder::rewind(&buf, 10);
        assert_eq!(snaps.len(), 2);
    }
}
