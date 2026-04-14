//! Simplified revolute joint angle limits for tests.

/// Parameters for [`simulate_revolute_with_angle_limits`].
#[derive(Clone, Copy, Debug)]
pub struct RevoluteLimitSim {
    /// Initial joint angle in radians.
    pub angle: f32,
    /// Initial angular velocity in radians per second.
    pub omega: f32,
    /// Applied torque in consistent units.
    pub applied_torque: f32,
    /// Rotational inertia (resistance to angular acceleration).
    pub inertia: f32,
    /// Integration timestep in seconds.
    pub dt: f32,
    /// Lower angle limit in radians.
    pub lower: f32,
    /// Upper angle limit in radians.
    pub upper: f32,
    /// Number of fixed substeps.
    pub steps: u32,
}

/// Integrate angular velocity from torque and clamp the joint angle (`TC-10.5.12.1`).
#[must_use]
pub fn simulate_revolute_with_angle_limits(sim: RevoluteLimitSim) -> (f32, f32) {
    let RevoluteLimitSim {
        mut angle,
        mut omega,
        applied_torque,
        inertia,
        dt,
        lower,
        upper,
        steps,
    } = sim;
    for _ in 0..steps {
        if inertia > 1e-8 {
            omega += (applied_torque / inertia) * dt;
        }
        angle += omega * dt;
        if angle < lower {
            angle = lower;
            omega = 0.0;
        } else if angle > upper {
            angle = upper;
            omega = 0.0;
        }
    }
    (angle, omega)
}
