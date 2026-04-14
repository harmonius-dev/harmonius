//! Snapshot interpolation and extrapolation for remote bodies.

use glam::{Quat, Vec3};

use crate::snapshot_types::PhysicsSnapshot;

/// Interpolated kinematic state for rendering remote entities.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InterpolatedState {
    /// Blended world position.
    pub position: Vec3,
    /// Blended world orientation.
    pub rotation: Quat,
    /// Blended linear velocity.
    pub linear_velocity: Vec3,
    /// Blended angular velocity.
    pub angular_velocity: Vec3,
}

/// Interpolates between two authoritative snapshots for a remote body.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnapshotInterpolator {
    /// Older server snapshot.
    pub from_snapshot: PhysicsSnapshot,
    /// Newer server snapshot.
    pub to_snapshot: PhysicsSnapshot,
    /// Blend factor in `[0, 1]`.
    pub alpha: f32,
}

impl SnapshotInterpolator {
    /// Linearly interpolates translation and velocity; slerps rotation.
    #[must_use]
    pub fn interpolate(&self) -> InterpolatedState {
        let a = self.alpha.clamp(0.0, 1.0);
        if self.from_snapshot == self.to_snapshot {
            return InterpolatedState {
                position: self.from_snapshot.position(),
                rotation: self.from_snapshot.rotation(),
                linear_velocity: self.from_snapshot.linear_velocity(),
                angular_velocity: self.from_snapshot.angular_velocity(),
            };
        }

        InterpolatedState {
            position: self
                .from_snapshot
                .position()
                .lerp(self.to_snapshot.position(), a),
            rotation: self
                .from_snapshot
                .rotation()
                .slerp(self.to_snapshot.rotation(), a),
            linear_velocity: self
                .from_snapshot
                .linear_velocity()
                .lerp(self.to_snapshot.linear_velocity(), a),
            angular_velocity: self
                .from_snapshot
                .angular_velocity()
                .lerp(self.to_snapshot.angular_velocity(), a),
        }
    }
}

/// Extrapolated kinematic state when snapshots arrive late.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtrapolatedState {
    /// Predicted world position after `dt`.
    pub position: Vec3,
    /// Orientation held constant from the last snapshot.
    pub rotation: Quat,
    /// Linear velocity carried forward from the snapshot.
    pub linear_velocity: Vec3,
    /// Angular velocity carried forward from the snapshot.
    pub angular_velocity: Vec3,
}

/// Constant-velocity extrapolation with a hard time clamp.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Extrapolator {
    /// Most recent authoritative snapshot.
    pub last_snapshot: PhysicsSnapshot,
    /// Maximum extrapolation horizon in seconds.
    pub max_extrapolation_dt: f32,
}

impl Extrapolator {
    /// Projects the snapshot forward in time, clamping `dt` to [`Self::max_extrapolation_dt`].
    #[must_use]
    pub fn extrapolate(&self, dt: f32) -> ExtrapolatedState {
        let dt = dt.max(0.0).min(self.max_extrapolation_dt);
        ExtrapolatedState {
            position: self.last_snapshot.position() + self.last_snapshot.linear_velocity() * dt,
            rotation: self.last_snapshot.rotation(),
            linear_velocity: self.last_snapshot.linear_velocity(),
            angular_velocity: self.last_snapshot.angular_velocity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot_types::ContactRange;

    fn snap(pos: Vec3) -> PhysicsSnapshot {
        PhysicsSnapshot::new(
            pos,
            Quat::IDENTITY,
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::ZERO,
            ContactRange::new(0, 0),
            false,
        )
    }

    #[test]
    fn tc_ir_4_5_6_1_interpolation_between_two_snaps() {
        let a = snap(Vec3::ZERO);
        let b = snap(Vec3::new(1.0, 0.0, 0.0));
        let interp = SnapshotInterpolator {
            from_snapshot: a,
            to_snapshot: b,
            alpha: 0.5,
        };
        let out = interp.interpolate();
        assert!((out.position.x - 0.5).abs() < 1e-5);
    }

    #[test]
    fn tc_ir_4_5_6_n1_identical_snapshots_return_from() {
        let s = snap(Vec3::ONE);
        let interp = SnapshotInterpolator {
            from_snapshot: s,
            to_snapshot: s,
            alpha: 0.25,
        };
        let out = interp.interpolate();
        assert_eq!(out.position, s.position());
    }

    #[test]
    fn tc_ir_4_5_6_2_extrapolation_when_dt_small() {
        let ex = Extrapolator {
            last_snapshot: snap(Vec3::ZERO),
            max_extrapolation_dt: 0.05,
        };
        let out = ex.extrapolate(0.1);
        assert!((out.position.y - 0.05).abs() < 1e-5);
    }

    #[test]
    fn tc_ir_4_5_6_n2_extrapolation_clamps_dt() {
        let ex = Extrapolator {
            last_snapshot: snap(Vec3::ZERO),
            max_extrapolation_dt: 0.05,
        };
        let out_large = ex.extrapolate(1.0);
        let out_small = ex.extrapolate(0.05);
        assert_eq!(out_large.position, out_small.position);
    }
}
