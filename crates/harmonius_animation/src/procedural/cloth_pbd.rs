//! Position-based cloth grid (CPU reference solver for tests).

use crate::math::Vec3;

fn idx(w: usize, i: usize, j: usize) -> usize {
    i * w + j
}

/// Distance constraint between two particle indices.
#[derive(Clone, Copy, Debug)]
struct Dist {
    a: usize,
    b: usize,
    rest: f32,
}

/// Simple cloth grid with structural and bending distance constraints.
#[derive(Clone, Debug)]
pub struct ClothGrid {
    w: usize,
    h: usize,
    spacing: f32,
    pos: Vec<Vec3>,
    inv_mass: Vec<f32>,
    dist: Vec<Dist>,
}

impl ClothGrid {
    /// Builds a `w`×`h` grid with spacing `spacing`, top row pinned.
    pub fn new_grid(w: usize, h: usize, spacing: f32) -> Self {
        let n = w * h;
        let mut pos = Vec::with_capacity(n);
        let mut inv_mass = vec![1.0_f32; n];
        for i in 0..h {
            for j in 0..w {
                let x = j as f32 * spacing;
                let y = -(i as f32) * spacing;
                pos.push(Vec3::new(x, y, 0.0));
                if i == 0 {
                    inv_mass[idx(w, i, j)] = 0.0;
                }
            }
        }
        let mut dist = Vec::new();
        for i in 0..h {
            for j in 0..w {
                if j + 1 < w {
                    dist.push(Dist {
                        a: idx(w, i, j),
                        b: idx(w, i, j + 1),
                        rest: spacing,
                    });
                }
                if i + 1 < h {
                    dist.push(Dist {
                        a: idx(w, i, j),
                        b: idx(w, i + 1, j),
                        rest: spacing,
                    });
                }
                if j + 2 < w {
                    dist.push(Dist {
                        a: idx(w, i, j),
                        b: idx(w, i, j + 2),
                        rest: spacing * 2.0,
                    });
                }
                if i + 2 < h {
                    dist.push(Dist {
                        a: idx(w, i, j),
                        b: idx(w, i + 2, j),
                        rest: spacing * 2.0,
                    });
                }
            }
        }
        Self {
            w,
            h,
            spacing,
            pos,
            inv_mass,
            dist,
        }
    }

    fn reset_pins(&mut self) {
        for j in 0..self.w {
            let x = j as f32 * self.spacing;
            self.pos[idx(self.w, 0, j)] = Vec3::new(x, 0.0, 0.0);
        }
    }

    fn solve_distance(&mut self, iters: u32) {
        for _ in 0..iters {
            for c in &self.dist {
                let pa = self.pos[c.a];
                let pb = self.pos[c.b];
                let w_a = self.inv_mass[c.a];
                let w_b = self.inv_mass[c.b];
                let w_sum = w_a + w_b;
                if w_sum < 1e-12 {
                    continue;
                }
                let delta = pb - pa;
                let len = delta.length();
                if len < 1e-12 {
                    continue;
                }
                let err = len - c.rest;
                let n = delta * (1.0 / len);
                let corr = n * (err / w_sum);
                self.pos[c.a] = pa + corr * w_a;
                self.pos[c.b] = pb - corr * w_b;
            }
            self.reset_pins();
        }
    }

    /// Substep with gravity and constraint iterations.
    pub fn substep(&mut self, dt: f32, gravity_y: f32, iters: u32) {
        for i in 0..self.pos.len() {
            if self.inv_mass[i] > 0.0 {
                self.pos[i].y += gravity_y * dt * dt;
            }
        }
        self.solve_distance(iters);
    }

    /// Row count (height) of the cloth grid.
    pub fn row_count(&self) -> usize {
        self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_5_1_1_cloth_distance() {
        let mut cloth = ClothGrid::new_grid(32, 32, 1.0 / 31.0);
        let g = -9.81_f32;
        let dt = 1.0 / 60.0;
        for _ in 0..60 {
            cloth.substep(dt, g, 4);
        }
        let spacing = 1.0 / 31.0;
        let mut max_err = 0.0_f32;
        for i in 0..cloth.h {
            for j in 0..cloth.w.saturating_sub(1) {
                let a = cloth.pos[idx(cloth.w, i, j)];
                let b = cloth.pos[idx(cloth.w, i, j + 1)];
                let len = (b - a).length();
                max_err = max_err.max((len - spacing).abs() / spacing);
            }
        }
        assert!(max_err < 0.01);
        let bottom = cloth.pos[idx(cloth.w, cloth.row_count() - 1, cloth.w / 2)];
        let top = cloth.pos[idx(cloth.w, 0, cloth.w / 2)];
        assert!(bottom.y < top.y);

        let mut cloth2 = ClothGrid::new_grid(8, 8, 0.125);
        for _ in 0..100 {
            cloth2.substep(1.0 / 120.0, -2.0, 8);
        }
        let mut max_r = 0.0_f32;
        for c in &cloth2.dist {
            let len = (cloth2.pos[c.b] - cloth2.pos[c.a]).length();
            max_r = max_r.max((len - c.rest).abs());
        }
        assert!(max_r < 1e-3);
        assert!(cloth2.pos.iter().all(|p| p.x.is_finite()));
    }
}
