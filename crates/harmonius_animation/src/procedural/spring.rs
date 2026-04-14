//! Critically damped spring-damper for scalar channels used by first-person rigs.
//!
//! **Stand-in:** The procedural design references `SpringDamper` from `harmonius_core::primitives`.
//! This workspace member is standalone, so [`ScalarSpring`] is a **temporary** scalar integrator.
//! Replace with shared core types before shipping engine wiring.

/// Second-order spring toward a moving target.
#[derive(Clone, Debug)]
pub struct ScalarSpring {
    /// Current value.
    pub value: f32,
    /// Current velocity.
    pub vel: f32,
    /// Stiffness `k`.
    pub stiffness: f32,
    /// Damping `d`.
    pub damping: f32,
}

impl ScalarSpring {
    /// Creates a spring at rest `value`.
    pub fn new(value: f32, stiffness: f32, damping: f32) -> Self {
        Self {
            value,
            vel: 0.0,
            stiffness,
            damping,
        }
    }

    /// Integrates toward `target` with semi-implicit Euler.
    pub fn step(&mut self, dt: f32, target: f32) {
        let acc = self.stiffness * (target - self.value) - self.damping * self.vel;
        self.vel += acc * dt;
        self.value += self.vel * dt;
    }
}
