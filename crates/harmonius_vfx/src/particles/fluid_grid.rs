//! Tiny Eulerian density grid for CPU validation (`TC-11.1.7.1`).

/// Cubic density storage with periodic boundaries and integer advection shift.
#[derive(Clone, Debug)]
pub struct FluidDensityGrid {
    /// Edge length (same on all axes).
    pub n: usize,
    /// Flat storage row-major `z * n*n + y * n + x`.
    pub data: Vec<f32>,
}

impl FluidDensityGrid {
    /// Allocates zeros with edge `n`.
    #[must_use]
    pub fn new_zeroed(n: usize) -> Self {
        Self {
            n,
            data: vec![0.0; n * n * n],
        }
    }

    fn idx(&self, x: usize, y: usize, z: usize) -> usize {
        z * self.n * self.n + y * self.n + x
    }

    /// Adds `amount` at the cell center.
    pub fn inject_center(&mut self, amount: f32) {
        let c = self.n / 2;
        let i = self.idx(c, c, c);
        self.data[i] += amount;
    }

    /// Total scalar mass (sum of density).
    #[must_use]
    pub fn total_mass(&self) -> f32 {
        self.data.iter().sum()
    }

    /// Advects along +X by `shift` cells using periodic sampling (mass conserving).
    pub fn advect_periodic_x(&mut self, shift: usize) {
        if self.n == 0 {
            return;
        }
        let mut next = vec![0.0; self.data.len()];
        for z in 0..self.n {
            for y in 0..self.n {
                for x in 0..self.n {
                    let src_x = (x + self.n * 1024 - shift) % self.n;
                    let di = self.idx(x, y, z);
                    let si = self.idx(src_x, y, z);
                    next[di] = self.data[si];
                }
            }
        }
        self.data = next;
    }

    /// Integrates density along `dir` through the grid (simple DDA-style sum).
    #[must_use]
    pub fn ray_march_density_sum(&self, origin: (f32, f32, f32), dir: (f32, f32, f32)) -> f32 {
        if self.n == 0 {
            return 0.0;
        }
        let n = self.n as f32;
        let mut t = 0.0_f32;
        let mut acc = 0.0_f32;
        for _ in 0..self.n * 3 {
            let x = origin.0 + dir.0 * t;
            let y = origin.1 + dir.1 * t;
            let z = origin.2 + dir.2 * t;
            if x < 0.0 || y < 0.0 || z < 0.0 || x >= n || y >= n || z >= n {
                break;
            }
            let xi = x.floor() as usize;
            let yi = y.floor() as usize;
            let zi = z.floor() as usize;
            acc += self.data[self.idx(xi.min(self.n - 1), yi.min(self.n - 1), zi.min(self.n - 1))];
            t += 1.0;
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.7.1` — periodic advection conserves mass; ray picks up injected density.
    #[test]
    fn tc_11_1_7_1_eulerian_fluid_grid_step() {
        let mut g = FluidDensityGrid::new_zeroed(8);
        g.inject_center(1.0);
        let m0 = g.total_mass();
        g.advect_periodic_x(1);
        let m1 = g.total_mass();
        assert!((m1 - m0).abs() / m0.max(1e-6) < 0.01);
        let hit = g.ray_march_density_sum((0.5, 4.5, 4.5), (1.0, 0.0, 0.0));
        assert!(hit > 0.0);
    }
}
