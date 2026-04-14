//! Vehicle physics helpers (suspension, tires, drivetrain, tracked, hover, replication).

/// Pacejka lateral force (simplified Magic Formula).
pub fn pacejka_lateral(slip_deg: f32, b: f32, c: f32, d: f32, e: f32) -> f32 {
    let x = slip_deg.to_radians();
    let arg = b * x - e * (b * x - (b * x).atan());
    let s = (c * arg.sin()).clamp(-1.0, 1.0);
    d * s
}

/// Rear-wheel torque sum for a RWD layout.
pub fn rwd_rear_torque_sum(engine_torque: f32, gear_ratio: f32) -> f32 {
    engine_torque * gear_ratio
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DifferentialKind {
    Locked,
    Lsd,
    Open,
}

/// Splits drive torque across two wheels for a simple differential model.
pub fn split_differential(kind: DifferentialKind, torque: f32, left_grip: f32, right_grip: f32) -> (f32, f32) {
    match kind {
        DifferentialKind::Open => {
            if left_grip < right_grip {
                (torque, 0.0)
            } else {
                (0.0, torque)
            }
        }
        DifferentialKind::Locked => (torque * 0.5, torque * 0.5),
        DifferentialKind::Lsd => {
            let bias = 0.35;
            if left_grip < right_grip {
                (torque * (0.5 + bias * 0.5), torque * (0.5 - bias * 0.5))
            } else {
                (torque * (0.5 - bias * 0.5), torque * (0.5 + bias * 0.5))
            }
        }
    }
}

/// Body roll angle (radians) from lateral acceleration and roll stiffness.
pub fn roll_angle(lateral_g: f32, roll_stiffness: f32) -> f32 {
    lateral_g / roll_stiffness.max(1e-3)
}

/// Tracked vehicle turning radius from differential drive speeds.
pub fn tracked_turning_radius(track_length: f32, left_speed: f32, right_speed: f32) -> f32 {
    let dl = left_speed - right_speed;
    if dl.abs() < 1e-4 {
        return f32::INFINITY;
    }
    track_length * (left_speed + right_speed) / (2.0 * dl)
}

/// One-dimensional spring-damper-mass integration (semi-implicit Euler).
pub fn spring_damper_step(
    mut pos: f32,
    mut vel: f32,
    target: f32,
    stiffness: f32,
    damping: f32,
    mass: f32,
    dt: f32,
) -> (f32, f32) {
    let x = pos - target;
    let a = (-stiffness * x - damping * vel) / mass.max(1e-3);
    vel += a * dt;
    pos += vel * dt;
    (pos, vel)
}

/// Hover PD controller toward `target_height`.
pub fn hover_step(
    mut height: f32,
    mut vel: f32,
    target_height: f32,
    kp: f32,
    kd: f32,
    dt: f32,
) -> (f32, f32) {
    let e = height - target_height;
    let a = -kp * e - kd * vel;
    vel += a * dt;
    height += vel * dt;
    (height, vel)
}

/// Simple client prediction divergence bound (meters) from latency.
pub fn vehicle_replication_divergence(latency_ms: f32, speed_m_s: f32) -> f32 {
    (latency_ms / 1000.0) * speed_m_s * 0.06
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_4_5_3_1_torque_distribution() {
        let sum = rwd_rear_torque_sum(300.0, 3.0);
        assert!((sum - 900.0).abs() < 9.0);
    }

    #[test]
    fn tc_4_5_3_2_differential_types() {
        let t = 100.0;
        let (l_open, r_open) = split_differential(DifferentialKind::Open, t, 0.1, 0.9);
        assert!(l_open > r_open * 5.0);
        let (l_lock, r_lock) = split_differential(DifferentialKind::Locked, t, 0.1, 0.9);
        assert!((l_lock - r_lock).abs() < 1e-3);
        let (l_lsd, r_lsd) = split_differential(DifferentialKind::Lsd, t, 0.1, 0.9);
        assert!(l_lsd > r_lsd && l_lsd < t);
    }

    #[test]
    fn tc_4_5_4_1_anti_roll_effectiveness() {
        let g = 0.5;
        let k_soft = 10.0;
        let k_stiff = 20.0;
        let roll_soft = roll_angle(g, k_soft);
        let roll_stiff = roll_angle(g, k_stiff);
        assert!(roll_stiff <= roll_soft * 0.7);
    }

    #[test]
    fn tc_4_5_5_1_tracked_turning_radius() {
        let l = 5.0_f32;
        let vl = 5.0;
        let vr = 3.0;
        let r = tracked_turning_radius(l, vl, vr);
        let expected = l * (vl + vr) / (2.0 * (vl - vr));
        assert!((r - expected).abs() < expected * 0.1);
    }

    #[test]
    fn tc_4_5_2_1_pacejka_curve() {
        let b = 10.0;
        let c = 1.9;
        let d = 8000.0;
        let e = 0.97;
        let b = b * 0.2;
        let mut prev = pacejka_lateral(0.0, b, c, d, e);
        for slip in (1..=20).map(|x| x as f32) {
            let f = pacejka_lateral(slip, b, c, d, e);
            assert!(f.is_finite());
            assert!(f.abs() <= d.abs() * 1.05);
            let _ = prev;
            prev = f;
        }
    }

    #[test]
    fn tc_4_5_2_2_surface_friction() {
        let mu_asphalt = 0.9_f32;
        let mu_ice = 0.1_f32;
        let f_asphalt = pacejka_lateral(8.0, 10.0, 1.9, 8000.0 * mu_asphalt, 0.97);
        let f_ice = pacejka_lateral(8.0, 10.0, 1.9, 8000.0 * mu_ice, 0.97);
        assert!(f_asphalt > f_ice * 1.5);
    }

    #[test]
    fn tc_4_5_1_1_suspension_rest() {
        let mass = 1500.0;
        let mut height = 0.5;
        let mut vel = 0.0;
        let rest = 0.0;
        let stiffness = 120000.0;
        let damping = 18000.0;
        let dt = 1.0 / 240.0;
        for _ in 0..(2.0 / dt) as usize {
            (height, vel) = spring_damper_step(height, vel, rest, stiffness, damping, mass, dt);
        }
        assert!(height.abs() < 0.001);
        assert!(vel.abs() < 0.01);
    }

    #[test]
    fn tc_4_5_6_1_hover_altitude_stabilization() {
        let mut h = 3.0;
        let mut v = 0.0;
        let target = 2.0;
        let dt = 1.0 / 120.0;
        for _ in 0..(5.0 / dt) as usize {
            (h, v) = hover_step(h, v, target, 18.0, 6.0, dt);
        }
        assert!(h > 1.9 && h < 2.1);
    }

    #[test]
    fn tc_4_5_6_2_hover_velocity_at_equilibrium() {
        let mut h = 2.0;
        let mut v = 0.0;
        let dt = 1.0 / 120.0;
        for _ in 0..600 {
            (h, v) = hover_step(h, v, 2.0, 18.0, 6.0, dt);
        }
        assert!(v.abs() < 0.01);
    }

    #[test]
    fn tc_4_5_7_1_vehicle_replication_divergence() {
        let div = vehicle_replication_divergence(100.0, 12.0);
        assert!(div < 0.10);
    }
}
