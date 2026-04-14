//! Position behaviors: follow, hard lock, composer, spline dolly.

use glam::{EulerRot, Quat, Vec2, Vec3};

use crate::types::{Follow, FollowBindingMode, ScreenRect};

/// Hard lock copies the tracking target position.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HardLockToTarget;

/// Adaptive framing with dead, soft, and hard zones in normalized screen space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionComposer {
    /// Screen-space dead zone (no movement).
    pub dead_zone: ScreenRect,
    /// Screen-space soft zone (damped movement).
    pub soft_zone: ScreenRect,
    /// Desired screen-space anchor.
    pub screen_position: Vec2,
    /// Lookahead time (seconds).
    pub lookahead_time: f32,
    /// Lookahead smoothing (seconds).
    pub lookahead_smoothing: f32,
    /// Position damping time constants (seconds).
    pub damping: Vec3,
}

/// Sample point on a piecewise-linear spline path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SplinePoint {
    /// World-space position.
    pub position: Vec3,
}

/// Runtime sample for spline dolly evaluation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SplineDollySample {
    /// Control polyline.
    pub points: [SplinePoint; 4],
    /// Number of active points (>=2).
    pub len: usize,
}

/// Computes the desired world-space camera position for [`HardLockToTarget`].
#[must_use]
pub fn evaluate_hard_lock_position(target_position: Vec3) -> Vec3 {
    target_position
}

/// Computes follow offset in world space for a binding mode after the target rotates.
#[must_use]
pub fn evaluate_follow_offset_world(
    binding: FollowBindingMode,
    target_position: Vec3,
    target_rotation: Quat,
    offset: Vec3,
) -> Vec3 {
    match binding {
        FollowBindingMode::WorldSpace => target_position + offset,
        FollowBindingMode::LockToTarget | FollowBindingMode::LazyFollow => {
            target_position + target_rotation * offset
        }
        FollowBindingMode::LockToTargetNoRoll => {
            let (yaw, _, _) = target_rotation.to_euler(EulerRot::YXZ);
            let yaw_only = Quat::from_rotation_y(yaw);
            target_position + yaw_only * offset
        }
        FollowBindingMode::LockWithWorldUp => {
            let (yaw, _, _) = target_rotation.to_euler(EulerRot::YXZ);
            let yaw_only = Quat::from_rotation_y(yaw);
            target_position + yaw_only * offset
        }
        FollowBindingMode::LockOnAssign => target_position + target_rotation * offset,
    }
}

/// Applies exponential damping toward `target` with per-axis time constants.
#[must_use]
pub fn evaluate_follow_damped_step(current: Vec3, target: Vec3, damping: Vec3, dt: f32) -> Vec3 {
    let mut out = current;
    for axis in 0..3 {
        let tau = damping[axis].max(1e-4);
        let alpha = 1.0 - (-dt / tau).exp();
        out[axis] = current[axis] + (target[axis] - current[axis]) * alpha;
    }
    out
}

/// Evaluates a single follow step including damping toward the binding-derived target.
#[must_use]
pub fn evaluate_follow_step(
    follow: &Follow,
    current: Vec3,
    target_position: Vec3,
    target_rotation: Quat,
    dt: f32,
) -> Vec3 {
    let desired = evaluate_follow_offset_world(
        follow.binding_mode,
        target_position,
        target_rotation,
        follow.offset,
    );
    evaluate_follow_damped_step(current, desired, follow.position_damping, dt)
}

/// Returns a corrective delta when the subject drifts outside composer zones.
#[must_use]
pub fn evaluate_position_composer_correction(
    composer: &PositionComposer,
    subject_screen: Vec2,
) -> Vec3 {
    let anchor = composer.screen_position;
    let delta = subject_screen - anchor;
    let dead = &composer.dead_zone;
    if subject_screen.x >= dead.min.x
        && subject_screen.x <= dead.max.x
        && subject_screen.y >= dead.min.y
        && subject_screen.y <= dead.max.y
    {
        return Vec3::ZERO;
    }

    let soft = &composer.soft_zone;
    let clamped = subject_screen.clamp(soft.min, soft.max);
    if clamped == subject_screen {
        // Outside dead but still inside soft — gentle push.
        return Vec3::new(delta.x, delta.y, 0.0) * 0.25;
    }

    // Hard limit — snap correction toward boundary.
    Vec3::new(
        (clamped - subject_screen).x,
        (clamped - subject_screen).y,
        0.0,
    )
}

/// Advances normalized spline parameter at fixed world speed.
#[must_use]
pub fn evaluate_spline_dolly_speed(
    sample: &SplineDollySample,
    mut t: f32,
    speed: f32,
    dt: f32,
) -> f32 {
    let length = spline_length(sample);
    if length <= 1e-4 {
        return t;
    }
    t += (speed * dt) / length;
    t.clamp(0.0, 1.0)
}

/// Finds nearest polyline parameter `t` in `[0,1]` to `target`.
#[must_use]
pub fn evaluate_spline_dolly_nearest(sample: &SplineDollySample, target: Vec3) -> f32 {
    let len = sample.len.max(2);
    let mut best_t = 0.0;
    let mut best_d2 = f32::MAX;
    let total = (len - 1) as f32;
    for i in 0..len - 1 {
        let a = sample.points[i].position;
        let b = sample.points[i + 1].position;
        let (d2, seg_t) = closest_point_on_segment_squared(a, b, target);
        if d2 < best_d2 {
            best_d2 = d2;
            let global_t = (i as f32 + seg_t) / total;
            best_t = global_t.clamp(0.0, 1.0);
        }
    }
    best_t
}

fn spline_length(sample: &SplineDollySample) -> f32 {
    let len = sample.len.max(2);
    let mut sum = 0.0;
    for i in 0..len - 1 {
        sum += sample.points[i]
            .position
            .distance(sample.points[i + 1].position);
    }
    sum.max(1e-4)
}

fn closest_point_on_segment_squared(a: Vec3, b: Vec3, p: Vec3) -> (f32, f32) {
    let ab = b - a;
    let ap = p - a;
    let denom = ab.length_squared().max(1e-6);
    let t = (ap.dot(ab) / denom).clamp(0.0, 1.0);
    let closest = a + ab * t;
    ((p - closest).length_squared(), t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    /// TC-13.25.3.1 — binding modes respond predictably to a 90° yaw.
    #[test]
    fn tc_13_25_3_1_follow_binding_modes() {
        let target_pos = Vec3::ZERO;
        let yaw_90 = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        let offset = Vec3::new(0.0, 0.0, 5.0);
        let world =
            evaluate_follow_offset_world(FollowBindingMode::WorldSpace, target_pos, yaw_90, offset);
        assert!((world - offset).length() < 1e-4);

        let locked = evaluate_follow_offset_world(
            FollowBindingMode::LockToTarget,
            target_pos,
            yaw_90,
            offset,
        );
        assert!((locked - Vec3::new(5.0, 0.0, 0.0)).length() < 1e-3);
    }

    /// TC-13.25.3.2 — damping eases toward the target over time.
    #[test]
    fn tc_13_25_3_2_follow_damping_smooths() {
        let follow = Follow {
            offset: Vec3::ZERO,
            binding_mode: FollowBindingMode::WorldSpace,
            position_damping: Vec3::splat(0.2),
            angular_damping: 0.2,
        };
        let mut current = Vec3::ZERO;
        for _ in 0..10 {
            current = evaluate_follow_step(
                &follow,
                current,
                Vec3::new(10.0, 0.0, 0.0),
                Quat::IDENTITY,
                1.0 / 60.0,
            );
        }
        assert!(current.x > 1.0 && current.x < 10.0);
    }

    /// TC-13.25.6.1 — hard lock matches the target position exactly.
    #[test]
    fn tc_13_25_6_1_hard_lock_zero_offset() {
        let target = Vec3::new(10.0, 5.0, 3.0);
        assert_eq!(evaluate_hard_lock_position(target), target);
    }

    /// TC-13.25.7.1 — dead zone suppresses movement.
    #[test]
    fn tc_13_25_7_1_position_composer_dead_zone() {
        let composer = PositionComposer {
            dead_zone: ScreenRect {
                min: Vec2::splat(0.4),
                max: Vec2::splat(0.6),
            },
            soft_zone: ScreenRect {
                min: Vec2::splat(0.2),
                max: Vec2::splat(0.8),
            },
            screen_position: Vec2::splat(0.5),
            lookahead_time: 0.0,
            lookahead_smoothing: 0.0,
            damping: Vec3::ONE,
        };
        let correction = evaluate_position_composer_correction(&composer, Vec2::splat(0.5));
        assert_eq!(correction, Vec3::ZERO);
    }

    /// TC-13.25.7.2 — hard limit snaps outside the soft region.
    #[test]
    fn tc_13_25_7_2_position_composer_hard_limit() {
        let composer = PositionComposer {
            dead_zone: ScreenRect {
                min: Vec2::splat(0.48),
                max: Vec2::splat(0.52),
            },
            soft_zone: ScreenRect {
                min: Vec2::splat(0.1),
                max: Vec2::splat(0.9),
            },
            screen_position: Vec2::splat(0.5),
            lookahead_time: 0.0,
            lookahead_smoothing: 0.0,
            damping: Vec3::ONE,
        };
        let correction = evaluate_position_composer_correction(&composer, Vec2::new(0.05, 0.5));
        assert_ne!(correction, Vec3::ZERO);
    }

    /// TC-13.25.8.1 — fixed speed advances normalized distance linearly.
    #[test]
    fn tc_13_25_8_1_spline_dolly_fixed_speed() {
        let sample = SplineDollySample {
            points: [
                SplinePoint {
                    position: Vec3::ZERO,
                },
                SplinePoint {
                    position: Vec3::new(10.0, 0.0, 0.0),
                },
                SplinePoint {
                    position: Vec3::ZERO,
                },
                SplinePoint {
                    position: Vec3::ZERO,
                },
            ],
            len: 2,
        };
        let t0 = 0.0;
        let t1 = evaluate_spline_dolly_speed(&sample, t0, 2.0, 0.5);
        assert!((t1 - 0.1).abs() < 1e-3, "t1={t1}");
    }

    /// TC-13.25.8.2 — nearest mode tracks a moving target along the polyline.
    #[test]
    fn tc_13_25_8_2_spline_dolly_nearest() {
        let sample = SplineDollySample {
            points: [
                SplinePoint {
                    position: Vec3::ZERO,
                },
                SplinePoint {
                    position: Vec3::new(10.0, 0.0, 0.0),
                },
                SplinePoint {
                    position: Vec3::new(10.0, 10.0, 0.0),
                },
                SplinePoint {
                    position: Vec3::ZERO,
                },
            ],
            len: 3,
        };
        let t = evaluate_spline_dolly_nearest(&sample, Vec3::new(5.0, 0.0, 0.0));
        assert!((t - 0.25).abs() < 1e-3);
        let t2 = evaluate_spline_dolly_nearest(&sample, Vec3::new(10.0, 5.0, 0.0));
        assert!((t2 - 0.75).abs() < 1e-2);
    }
}
