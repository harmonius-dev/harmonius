//! First-person camera and viewmodel spring helpers.

use super::spring::ScalarSpring;

/// Head-bob driver from walk speed and phase.
pub fn head_bob_offset_y(walk_speed_m_s: f32, frame: u32, amplitude_m: f32) -> f32 {
    let speed_scale = (walk_speed_m_s / 1.4).clamp(0.0, 2.5);
    let phase = frame as f32 * 0.4 * (0.35 + speed_scale);
    phase.sin() * amplitude_m
}

/// Landing impulse applied to camera vertical spring.
pub fn apply_landing(spring: &mut ScalarSpring, impulse: f32) {
    spring.vel += impulse;
}

/// Weapon sway from mouse delta (opposite sense).
pub fn weapon_sway_target(mouse_dx: f32, scale: f32) -> f32 {
    -mouse_dx * scale
}

/// Sprint tilt (degrees) around Z for viewmodel.
pub fn sprint_tilt_deg(sprint_active: bool, configured: f32) -> f32 {
    if sprint_active {
        configured
    } else {
        0.0
    }
}

/// Recoil kick with ADS scaling.
pub fn recoil_kick(pattern_sample_deg: f32, ads_scale: f32) -> f32 {
    pattern_sample_deg * ads_scale
}

/// Dual-wield independent springs (no cross-coupling in this model).
#[derive(Clone, Debug)]
pub struct DualHandSprings {
    /// Left pitch spring.
    pub left: ScalarSpring,
    /// Right pitch spring.
    pub right: ScalarSpring,
}

impl DualHandSprings {
    /// Steps both hands independently.
    pub fn step(&mut self, dt: f32, tl: f32, tr: f32) {
        self.left.step(dt, tl);
        self.right.step(dt, tr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_6_1_1_head_bob() {
        let amp = 0.03;
        let mut max_v = 0.0_f32;
        for f in 0..60 {
            let y = head_bob_offset_y(1.5, f, amp);
            max_v = max_v.max(y.abs());
        }
        assert!(max_v <= amp * 1.05);

        let mut s = ScalarSpring::new(0.0, 180.0, 18.0);
        apply_landing(&mut s, -6.0);
        for _ in 0..(0.3_f32 / (1.0 / 60.0)) as u32 {
            s.step(1.0 / 60.0, 0.0);
        }
        assert!(s.value.abs() < amp * 4.0);
    }

    #[test]
    fn tc_9_6_2_1_sway_sprint() {
        let mut sway = ScalarSpring::new(0.0, 30.0, 4.0);
        for _ in 0..120 {
            sway.step(1.0 / 60.0, weapon_sway_target(10.0, 0.01));
        }
        assert!(sway.value < 0.0);
        let tilt = sprint_tilt_deg(true, 15.0);
        assert!((tilt - 15.0).abs() < 1.0);
    }

    #[test]
    fn tc_9_6_3_1_recoil_ads() {
        let pat = [0.0_f32, 1.0, 0.5, 1.1];
        let mut k = ScalarSpring::new(0.0, 200.0, 22.0);
        for i in 0..10 {
            let sample = pat[i % pat.len()];
            k.step(1.0 / 60.0, 0.0);
            k.value += recoil_kick(sample, 1.0);
        }
        assert!(k.value > 0.0);
        let ads = recoil_kick(10.0, 0.4);
        assert!(ads < 10.0);
    }

    #[test]
    fn tc_9_6_4_1_dual_wield() {
        let mut d = DualHandSprings {
            left: ScalarSpring::new(0.0, 40.0, 5.0),
            right: ScalarSpring::new(0.0, 40.0, 5.0),
        };
        for _ in 0..30 {
            d.step(1.0 / 60.0, 1.0, -1.0);
        }
        assert!((d.left.value - d.right.value).abs() > 0.01);
    }
}
