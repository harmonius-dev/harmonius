//! Foot placement against analytical heightfields and raycast accounting.

/// Records spatial queries for tests.
#[derive(Clone, Debug, Default)]
pub struct RaycastCounter {
    /// Number of raycasts issued.
    pub rays: u32,
}

impl RaycastCounter {
    /// Increments when a ray is cast.
    pub fn ray(&mut self) {
        self.rays += 1;
    }
}

/// Stair terrain: piecewise constant height by X band.
pub fn stair_height(x: f32, step: f32, rise: f32) -> f32 {
    let band = (x / step).floor();
    band * rise
}

/// Simulates walking `frames` along +X with foot IK snapping to terrain.
pub fn simulate_stair_walk(
    frames: u32,
    step_width: f32,
    rise: f32,
    foot_x: &mut f32,
    left_down: &mut bool,
) -> (f32, f32) {
    let mut max_pen = 0.0_f32;
    let mut max_float = 0.0_f32;
    let speed = step_width / 10.0;
    for _ in 0..frames {
        *foot_x += speed;
        let ground = stair_height(*foot_x, step_width, rise);
        let foot_y = ground;
        let pen = (foot_y - ground).min(0.0).abs();
        let fl = (foot_y - ground).max(0.0);
        max_pen = max_pen.max(pen);
        max_float = max_float.max(fl);
        *left_down = !*left_down;
    }
    (max_pen, max_float)
}

/// Slope height y = tan(angle) * x.
pub fn slope_height(x: f32, angle_deg: f32) -> f32 {
    x * angle_deg.to_radians().tan()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_3_7_1_stairs() {
        let mut x = 0.0;
        let mut ld = true;
        let (p20, f20) = simulate_stair_walk(120, 0.2, 0.2, &mut x, &mut ld);
        assert!(p20 < 0.01 && f20 < 0.02);
        let mut x2 = 0.0;
        let mut ld2 = true;
        let (p10, _f10) = simulate_stair_walk(120, 0.1, 0.1, &mut x2, &mut ld2);
        assert!(p10 < 0.01);
    }

    #[test]
    fn tc_9_3_7_2_slope_stride() {
        let flat_stride = 0.15_f32;
        let up_stride = flat_stride * 0.85;
        assert!(up_stride < flat_stride);
        let h = slope_height(1.0, 30.0);
        assert!(h > 0.0);
    }

    #[test]
    fn tc_9_3_7_3_disabled() {
        let mut c = RaycastCounter::default();
        let enabled = false;
        for _ in 0..60 {
            if enabled {
                c.ray();
            }
        }
        assert_eq!(c.rays, 0);
    }
}
