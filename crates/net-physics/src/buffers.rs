//! Fixed-capacity ring buffers for physics and hitbox history.

use crate::entity::Entity;
use crate::snapshot_types::{HitboxSnapshot, PhysicsSnapshot};

/// Maximum ticks of physics history retained for rollback (IR-4.5.7).
pub const MAX_ROLLBACK_TICKS: usize = 64;

/// Maximum hitbox history depth for lag compensation (IR-4.5.5).
pub const MAX_REWIND_TICKS: usize = 64;

/// Ring buffer of [`PhysicsSnapshot`] values keyed by tick.
#[derive(Debug)]
pub struct SnapshotBuffer {
    slots: [Option<(u64, PhysicsSnapshot)>; MAX_ROLLBACK_TICKS],
    head: usize,
    len: usize,
}

impl Default for SnapshotBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl SnapshotBuffer {
    /// Constructs an empty buffer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
            head: 0,
            len: 0,
        }
    }

    /// Appends a snapshot for `tick`, evicting the oldest entry when full.
    pub fn push(&mut self, tick: u64, snapshot: PhysicsSnapshot) {
        if self.len < MAX_ROLLBACK_TICKS {
            let idx = (self.head + self.len) % MAX_ROLLBACK_TICKS;
            self.slots[idx] = Some((tick, snapshot));
            self.len += 1;
        } else {
            self.slots[self.head] = Some((tick, snapshot));
            self.head = (self.head + 1) % MAX_ROLLBACK_TICKS;
        }
    }

    /// Returns the snapshot stored for `tick`, if still retained.
    #[must_use]
    pub fn get(&self, tick: u64) -> Option<&PhysicsSnapshot> {
        for i in 0..self.len {
            let idx = (self.head + i) % MAX_ROLLBACK_TICKS;
            if let Some((t, snap)) = self.slots[idx].as_ref()
                && *t == tick
            {
                return Some(snap);
            }
        }
        None
    }
}

/// Ring buffer of [`HitboxSnapshot`] entries for lag compensation.
#[derive(Debug)]
pub struct HitboxBuffer {
    slots: [Option<HitboxSnapshot>; MAX_REWIND_TICKS],
    head: usize,
    len: usize,
}

impl Default for HitboxBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl HitboxBuffer {
    /// Constructs an empty buffer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
            head: 0,
            len: 0,
        }
    }

    /// Records a hitbox snapshot, evicting the oldest entry when full.
    pub fn push(&mut self, snapshot: HitboxSnapshot) {
        if self.len < MAX_REWIND_TICKS {
            let idx = (self.head + self.len) % MAX_REWIND_TICKS;
            self.slots[idx] = Some(snapshot);
            self.len += 1;
        } else {
            self.slots[self.head] = Some(snapshot);
            self.head = (self.head + 1) % MAX_REWIND_TICKS;
        }
    }

    /// Returns the snapshot for `entity` at `tick`, if present.
    #[must_use]
    pub fn get(&self, entity: Entity, tick: u64) -> Option<&HitboxSnapshot> {
        for i in 0..self.len {
            let idx = (self.head + i) % MAX_REWIND_TICKS;
            if let Some(snap) = self.slots[idx].as_ref()
                && snap.tick() == tick
                && snap.entity() == entity
            {
                return Some(snap);
            }
        }
        None
    }

    /// Returns references to every hitbox snapshot recorded at `tick`.
    #[must_use]
    pub fn snapshots_at_tick(&self, tick: u64) -> Vec<&HitboxSnapshot> {
        let mut out = Vec::new();
        for i in 0..self.len {
            let idx = (self.head + i) % MAX_REWIND_TICKS;
            if let Some(snap) = self.slots[idx].as_ref() && snap.tick() == tick {
                out.push(snap);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot_types::ContactRange;
    use glam::{Quat, Vec3};

    fn sample_snapshot(tick: u64) -> PhysicsSnapshot {
        let f = tick as f32;
        PhysicsSnapshot::new(
            Vec3::new(f, 0.0, 0.0),
            Quat::IDENTITY,
            Vec3::ZERO,
            Vec3::ZERO,
            ContactRange::new(0, 0),
            false,
        )
    }

    #[test]
    fn tc_ir_4_5_7_n1_buffer_overflow_evicts_oldest() {
        let mut buf = SnapshotBuffer::new();
        for t in 0..(MAX_ROLLBACK_TICKS as u64 + 10) {
            buf.push(t, sample_snapshot(t));
        }
        assert!(buf.get(0).is_none());
        assert!(buf.get(10).is_some());
    }

    #[test]
    fn tc_ir_4_5_7_1_snapshot_capture_fields() {
        let mut buf = SnapshotBuffer::new();
        let snap = PhysicsSnapshot::new(
            Vec3::ONE,
            Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0),
            ContactRange::new(7, 3),
            true,
        );
        buf.push(42, snap);
        let got = buf.get(42).expect("snapshot");
        assert_eq!(got.position(), Vec3::ONE);
        assert_eq!(got.rotation(), Quat::IDENTITY);
        assert_eq!(got.linear_velocity(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(got.angular_velocity(), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(got.contacts().start(), 7);
        assert_eq!(got.contacts().len(), 3);
        assert!(got.sleeping());
    }

    #[test]
    fn tc_ir_4_5_3_n1_get_returns_none_when_tick_evicted() {
        let mut buf = SnapshotBuffer::new();
        for t in 0..MAX_ROLLBACK_TICKS as u64 {
            buf.push(t, sample_snapshot(t));
        }
        buf.push(MAX_ROLLBACK_TICKS as u64, sample_snapshot(MAX_ROLLBACK_TICKS as u64));
        assert!(buf.get(0).is_none());
    }

    #[test]
    fn tc_ir_4_5_5_n2_unknown_entity_returns_none() {
        let mut buf = HitboxBuffer::new();
        let e0 = Entity::new(0, 0);
        let e1 = Entity::new(99, 0);
        buf.push(HitboxSnapshot::new(
            1,
            e0,
            Vec3::ZERO,
            Quat::IDENTITY,
            crate::snapshot_types::ColliderShape::Sphere { radius: 1.0 },
        ));
        assert!(buf.get(e1, 1).is_none());
    }
}
