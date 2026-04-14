//! Server-side prediction reconciliation.

use crate::buffers::SnapshotBuffer;
use crate::input::InputFrame;
use crate::snapshot_types::PhysicsSnapshot;

/// Result of attempting a rollback resimulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RollbackOutcome {
    /// No mismatch was detected; physics state unchanged.
    NoRollbackNeeded,
    /// Rollback succeeded and `replay_ticks` fixed-timestep resimulations were run.
    RolledBack {
        /// Number of ticks replayed after restoring the mismatch tick.
        replay_ticks: u32,
    },
    /// Rollback could not be performed safely; caller should request a full state sync.
    FullResync,
}

/// Compares predicted physics against authoritative snapshots and drives rollback replay.
#[derive(Clone, Copy, Debug)]
pub struct ServerReconciler {
    /// Position delta beyond which a mismatch is declared.
    pub mismatch_threshold: f32,
    /// Maximum ticks to replay after rollback before forcing a full resync.
    pub max_rollback_ticks: u32,
}

impl ServerReconciler {
    /// Returns `true` when predicted state differs beyond [`Self::mismatch_threshold`].
    #[must_use]
    pub fn check(&self, predicted: &PhysicsSnapshot, authoritative: &PhysicsSnapshot) -> bool {
        (predicted.position() - authoritative.position()).length() > self.mismatch_threshold
    }

    /// Restores `mismatch_tick` from `buffer` and resimulates up to `current_tick` using `inputs`.
    ///
    /// If the snapshot is missing or the rollback horizon is exceeded, returns
    /// [`RollbackOutcome::FullResync`] without mutating `world`.
    pub fn rollback<W: PhysicsReplayWorld>(
        &self,
        buffer: &SnapshotBuffer,
        mismatch_tick: u64,
        current_tick: u64,
        world: &mut W,
        inputs: &[InputFrame],
    ) -> RollbackOutcome {
        if current_tick <= mismatch_tick {
            return RollbackOutcome::NoRollbackNeeded;
        }

        let delta_ticks = current_tick - mismatch_tick;
        if delta_ticks > u64::from(self.max_rollback_ticks) {
            return RollbackOutcome::FullResync;
        }

        let Some(snapshot) = buffer.get(mismatch_tick) else {
            return RollbackOutcome::FullResync;
        };

        world.restore_from_snapshot(mismatch_tick, snapshot);

        let mut replay_ticks: u32 = 0;
        for tick in (mismatch_tick + 1)..=current_tick {
            let input = inputs
                .iter()
                .find(|i| i.tick == tick)
                .copied()
                .unwrap_or_else(|| InputFrame::zero(tick));
            world.resimulate_tick(tick, &input);
            replay_ticks = replay_ticks.saturating_add(1);
        }

        RollbackOutcome::RolledBack { replay_ticks }
    }
}

/// Hooks used by [`ServerReconciler::rollback`] to apply deterministic replay to a physics world.
pub trait PhysicsReplayWorld {
    /// Restores authoritative state at `tick` from `snapshot`.
    fn restore_from_snapshot(&mut self, tick: u64, snapshot: &PhysicsSnapshot);

    /// Runs one fixed timestep for `tick` using `input`.
    fn resimulate_tick(&mut self, tick: u64, input: &InputFrame);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffers::SnapshotBuffer;
    use crate::snapshot_types::ContactRange;
    use glam::{Quat, Vec3};

    #[derive(Default)]
    struct MockWorld {
        restores: u32,
        resims: Vec<u64>,
    }

    impl PhysicsReplayWorld for MockWorld {
        fn restore_from_snapshot(&mut self, _tick: u64, _snapshot: &PhysicsSnapshot) {
            self.restores += 1;
        }

        fn resimulate_tick(&mut self, tick: u64, _input: &InputFrame) {
            self.resims.push(tick);
        }
    }

    fn snap_at(x: f32) -> PhysicsSnapshot {
        PhysicsSnapshot::new(
            Vec3::new(x, 0.0, 0.0),
            Quat::IDENTITY,
            Vec3::ZERO,
            Vec3::ZERO,
            ContactRange::new(0, 0),
            false,
        )
    }

    #[test]
    fn tc_ir_4_5_3_2_replays_n_ticks() {
        let mut buf = SnapshotBuffer::new();
        buf.push(10, snap_at(0.0));
        let recon = ServerReconciler {
            mismatch_threshold: 0.01,
            max_rollback_ticks: 32,
        };
        let mut world = MockWorld::default();
        let inputs: Vec<InputFrame> = (11..=15)
            .map(|t| InputFrame {
                tick: t,
                move_x: 0.0,
                move_y: 0.0,
                move_z: 0.0,
            })
            .collect();
        let out = recon.rollback(&buf, 10, 15, &mut world, &inputs);
        assert_eq!(out, RollbackOutcome::RolledBack { replay_ticks: 5 });
        assert_eq!(world.restores, 1);
        assert_eq!(world.resims, vec![11, 12, 13, 14, 15]);
    }

    #[test]
    fn tc_ir_4_5_3_3_exceeds_cap_requests_full_resync() {
        let mut buf = SnapshotBuffer::new();
        buf.push(0, snap_at(0.0));
        let recon = ServerReconciler {
            mismatch_threshold: 0.01,
            max_rollback_ticks: 10,
        };
        let mut world = MockWorld::default();
        let out = recon.rollback(&buf, 0, 50, &mut world, &[]);
        assert_eq!(out, RollbackOutcome::FullResync);
        assert_eq!(world.restores, 0);
        assert!(world.resims.is_empty());
    }

    #[test]
    fn tc_ir_4_5_3_n1_missing_snapshot_full_resync() {
        let buf = SnapshotBuffer::new();
        let recon = ServerReconciler {
            mismatch_threshold: 0.01,
            max_rollback_ticks: 32,
        };
        let mut world = MockWorld::default();
        let out = recon.rollback(&buf, 5, 10, &mut world, &[]);
        assert_eq!(out, RollbackOutcome::FullResync);
    }
}
