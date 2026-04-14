//! Guide strand mass-spring reference for hair tests.

use crate::math::Vec3;

/// One strand as polyline points.
#[derive(Clone, Debug)]
pub struct StrandGuide {
    /// Point positions.
    pub pts: Vec<Vec3>,
    /// Rest segment lengths.
    pub rest: Vec<f32>,
}

impl StrandGuide {
    /// Straight vertical strand of `n` points spaced `h`.
    pub fn straight_vertical(n: usize, h: f32) -> Self {
        let mut pts = Vec::new();
        let mut rest = Vec::new();
        for i in 0..n {
            pts.push(Vec3::new(0.0, -(i as f32) * h, 0.0));
            if i > 0 {
                rest.push(h);
            }
        }
        Self { pts, rest }
    }

    /// Relaxation step: gravity, stretch, optional wind.
    pub fn step(&mut self, dt: f32, gravity_y: f32, wind: Vec3, stretch_iters: u32) {
        for p in &mut self.pts {
            p.y += gravity_y * dt * dt;
            *p = *p + wind * (dt * dt);
        }
        for _ in 0..stretch_iters {
            for i in 0..self.pts.len() - 1 {
                let a = self.pts[i];
                let b = self.pts[i + 1];
                let dir = b - a;
                let len = dir.length();
                if len < 1e-8 {
                    continue;
                }
                let n = dir * (1.0 / len);
                let err = len - self.rest[i];
                let corr = n * (err * 0.5);
                self.pts[i] = a + corr;
                self.pts[i + 1] = b - corr;
            }
            self.pts[0] = Vec3::ZERO;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_5_2_1_strand() {
        let mut s = StrandGuide::straight_vertical(16, 0.05);
        for _ in 0..60 {
            s.step(1.0 / 60.0, -3.0, Vec3::ZERO, 16);
        }
        let mut max_err = 0.0_f32;
        for i in 0..s.pts.len() - 1 {
            let len = (s.pts[i + 1] - s.pts[i]).length();
            max_err = max_err.max((len - s.rest[i]).abs() / s.rest[i]);
        }
        assert!(max_err < 0.03);
        let tip = *s.pts.last().unwrap();
        assert!(tip.y < -0.1);

        let mut s0 = StrandGuide::straight_vertical(16, 0.05);
        for _ in 0..60 {
            s0.step(1.0 / 60.0, -1.0, Vec3::ZERO, 6);
        }
        let tip0 = *s0.pts.last().unwrap();

        let mut s2 = StrandGuide::straight_vertical(16, 0.05);
        for _ in 0..60 {
            s2.step(1.0 / 60.0, -1.0, Vec3::new(5.0, 0.0, 0.0), 6);
        }
        let tip2 = *s2.pts.last().unwrap();
        assert!(tip2.x > tip0.x);
    }
}
