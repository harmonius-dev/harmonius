//! Particle-driven light buffer bookkeeping (`TC-11.1.6.*`).

/// Returns how many lights would be written for `alive_particles` capped by `max_lights`.
#[must_use]
pub fn particle_light_write_count(alive_particles: u32, max_lights: u32) -> u32 {
    alive_particles.min(max_lights)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.6.1` — alive particles map into clustered slots up to the cap.
    #[test]
    fn tc_11_1_6_1_particle_light_injection() {
        assert_eq!(particle_light_write_count(10, 64), 10);
    }

    /// `TC-11.1.6.2` — never exceed per-emitter light cap.
    #[test]
    fn tc_11_1_6_2_particle_light_cap() {
        assert_eq!(particle_light_write_count(100, 8), 8);
    }
}
