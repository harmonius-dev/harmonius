//! Exponential moving average smoothing for post-rollback error reduction.

use glam::{Quat, Vec3};

use crate::snapshot_types::PhysicsSnapshot;

/// Visual state after applying error correction smoothing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CorrectedState {
    /// Smoothed world position.
    pub position: Vec3,
    /// Smoothed world orientation.
    pub rotation: Quat,
}

/// Smooths residual error after reconciliation using exponential decay.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ErrorCorrector {
    position_error: Vec3,
    rotation_error: Quat,
    decay_rate: f32,
}

impl ErrorCorrector {
    /// Creates a corrector with initial error offsets and decay factor in `(0, 1)`.
    #[must_use]
    pub fn new(position_error: Vec3, rotation_error: Quat, decay_rate: f32) -> Self {
        Self {
            position_error,
            rotation_error,
            decay_rate,
        }
    }

    /// Applies exponential decay to stored error and returns a smoothed pose.
    pub fn apply(&mut self, current: &PhysicsSnapshot, dt: f32) -> CorrectedState {
        let dt = dt.max(0.0);
        let factor = self.decay_rate.powf(dt).clamp(0.0, 1.0);

        let position = current.position() + self.position_error;
        self.position_error *= factor;
        if self.position_error.length() < 0.001 {
            self.position_error = Vec3::ZERO;
        }

        let rotation = (current.rotation() * self.rotation_error).normalize();
        self.rotation_error = self.rotation_error.slerp(Quat::IDENTITY, 1.0 - factor);
        if self.rotation_error.w > 1.0 - 1e-4 && self.rotation_error.xyz().length() < 0.001 {
            self.rotation_error = Quat::IDENTITY;
        }

        CorrectedState { position, rotation }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot_types::ContactRange;

    #[test]
    fn error_corrector_decays_position_error() {
        let mut ec = ErrorCorrector::new(Vec3::new(0.5, 0.0, 0.0), Quat::IDENTITY, 0.5);
        let current = PhysicsSnapshot::new(
            Vec3::ZERO,
            Quat::IDENTITY,
            Vec3::ZERO,
            Vec3::ZERO,
            ContactRange::new(0, 0),
            false,
        );
        let first = ec.apply(&current, 1.0 / 60.0);
        assert!(first.position.x > 0.0);
        let _second = ec.apply(&current, 1.0 / 60.0);
        assert!(ec.position_error.length() < 0.5);
    }
}
