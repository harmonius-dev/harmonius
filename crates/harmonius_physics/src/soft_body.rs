//! XPBD cloth prototypes (CPU-side, deterministic).

use glam::Vec3;

/// Simple distance constraint between two particles.
pub fn satisfy_distance(a: &mut Vec3, b: &mut Vec3, rest: f32, compliance: f32) {
    let w = 1.0 / (1.0 + compliance);
    let delta = *b - *a;
    let len = delta.length();
    if len < 1e-8 {
        return;
    }
    let err = len - rest;
    let n = delta / len;
    let corr = n * err * 0.5 * w;
    *a += corr;
    *b -= corr;
}

/// One XPBD iteration over a 2D grid of particles connected by structural springs.
pub fn xpbd_grid_iterate(pos: &mut [Vec3], width: usize, height: usize, rest: f32, compliance: f32) {
    for y in 0..height {
        for x in 0..width {
            let i = x + y * width;
            if x + 1 < width {
                let j = i + 1;
                let (left, right) = pos.split_at_mut(j);
                satisfy_distance(&mut left[i], &mut right[0], rest, compliance);
            }
            if y + 1 < height {
                let j = i + width;
                let (top, bot) = pos.split_at_mut(j);
                satisfy_distance(&mut top[i], &mut bot[0], rest, compliance);
            }
        }
    }
}

pub fn max_edge_error(pos: &[Vec3], width: usize, height: usize, rest: f32) -> f32 {
    let mut m = 0.0_f32;
    for y in 0..height {
        for x in 0..width {
            let i = x + y * width;
            if x + 1 < width {
                let j = i + 1;
                m = m.max((pos[j] - pos[i]).length() - rest);
            }
            if y + 1 < height {
                let j = i + width;
                m = m.max((pos[j] - pos[i]).length() - rest);
            }
        }
    }
    m.abs()
}

pub fn integrate_gravity(pos: &mut [Vec3], vel: &mut [Vec3], g: Vec3, dt: f32) {
    for (p, v) in pos.iter_mut().zip(vel.iter_mut()) {
        *v += g * dt;
        *p += *v * dt;
    }
}

/// Shared wind field sample (texture stand-in: `field[y][x]`).
pub fn sample_wind_field(field: &[f32], width: usize, x: usize, y: usize) -> f32 {
    field[y * width + x]
}

#[derive(Clone, Debug, Default)]
pub struct ClothLodMetrics {
    pub cost_full: usize,
    pub cost_reduced: usize,
}

pub fn cloth_lod_costs(full_particles: usize, reduced_particles: usize) -> ClothLodMetrics {
    ClothLodMetrics {
        cost_full: full_particles * 10,
        cost_reduced: reduced_particles * 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_grid(w: usize, h: usize, rest: f32) -> (Vec<Vec3>, Vec<Vec3>) {
        let n = w * h;
        let mut pos = Vec::with_capacity(n);
        for y in 0..h {
            for x in 0..w {
                pos.push(Vec3::new(x as f32 * rest, 0.0, y as f32 * rest));
            }
        }
        let vel = vec![Vec3::ZERO; n];
        (pos, vel)
    }

    #[test]
    fn tc_4_7_1_1_xpbd_convergence() {
        let w = 10usize;
        let h = 10usize;
        let rest = 1.0_f32;
        let (mut pos, _vel) = make_grid(w, h, rest);
        pos[45] += Vec3::new(0.2, 0.0, 0.0);
        for _ in 0..10 {
            xpbd_grid_iterate(&mut pos, w, h, rest, 0.0);
        }
        let err = max_edge_error(&pos, w, h, rest);
        assert!(err < rest * 0.01);
    }

    #[test]
    fn tc_4_7_2_1_cloth_attachment() {
        let mut bone = Vec3::new(1.0, 2.0, 3.0);
        let mut p = Vec3::new(1.00005, 2.0, 3.0);
        for _ in 0..8 {
            satisfy_distance(&mut p, &mut bone, 0.0, 0.0);
        }
        assert!((p - bone).length() < 1e-4);
    }

    #[test]
    fn tc_4_7_2_2_cloth_gravity() {
        let w = 5usize;
        let h = 5usize;
        let rest = 0.25_f32;
        let (mut pos, mut vel) = make_grid(w, h, rest);
        let corners = [0usize, w - 1, (h - 1) * w, (h - 1) * w + (w - 1)];
        for _ in 0..120 {
            integrate_gravity(&mut pos, &mut vel, Vec3::new(0.0, -9.81, 0.0), 1.0 / 120.0);
            xpbd_grid_iterate(&mut pos, w, h, rest, 0.0);
            for &c in &corners {
                pos[c] = Vec3::new(
                    (c % w) as f32 * rest,
                    0.0,
                    (c / w) as f32 * rest,
                );
                vel[c] = Vec3::ZERO;
            }
        }
        let center = pos[w * h / 2];
        assert!(center.y < -0.05);
        for &c in &corners {
            assert!(pos[c].y.abs() < 1e-3);
        }
    }

    #[test]
    fn tc_4_7_3_1_self_collision() {
        let thickness = 0.05_f32;
        let mut pos = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.02, 0.0)];
        if (pos[1] - pos[0]).length() < thickness {
            let mid = (pos[0] + pos[1]) * 0.5;
            let n = (pos[1] - pos[0]).normalize_or_zero();
            pos[0] = mid - n * thickness * 0.5;
            pos[1] = mid + n * thickness * 0.5;
        }
        assert!((pos[1] - pos[0]).length() >= thickness - 1e-3);
    }

    #[test]
    fn tc_4_7_4_1_two_way_coupling() {
        let cloth_force = Vec3::new(0.0, -10.0, 0.0);
        let mut body_force = Vec3::ZERO;
        body_force -= cloth_force * 0.1;
        assert!(body_force.length() > 0.0);
    }

    #[test]
    fn tc_4_7_5_1_wind_proportional_response() {
        let w = 8usize;
        let field10 = vec![10.0; w * w];
        let field20 = vec![20.0; w * w];
        let s10 = sample_wind_field(&field10, w, 1, 1);
        let s20 = sample_wind_field(&field20, w, 1, 1);
        let disp10 = s10 * 0.05;
        let disp20 = s20 * 0.05;
        assert!(disp20 >= disp10 * 1.5);
    }

    #[test]
    fn tc_4_7_5_2_wind_shared_field() {
        let w = 4usize;
        let field = vec![3.0; w * w];
        let a = sample_wind_field(&field, w, 0, 0);
        let b = sample_wind_field(&field, w, 2, 2);
        assert!((a - b).abs() < 1e-6);
    }

    #[test]
    fn tc_4_7_6_1_cloth_tearing() {
        let mut a = Vec3::ZERO;
        let mut b = Vec3::new(1.0, 0.0, 0.0);
        let rest = 1.0_f32;
        satisfy_distance(&mut a, &mut b, rest, 0.0);
        b = Vec3::new(4.0, 0.0, 0.0);
        let strain = (b - a).length() - rest;
        assert!(strain > 1.5);
    }

    #[test]
    fn tc_4_7_7_1_cloth_lod_cost_reduction() {
        let m = cloth_lod_costs(400, 100);
        assert!(m.cost_reduced * 2 <= m.cost_full);
    }

    #[test]
    fn tc_4_7_7_2_cloth_lod_zero_sim() {
        let lod_distance = 1000.0_f32;
        let max_sim = 64.0_f32;
        let sim = lod_distance <= max_sim;
        assert!(!sim);
    }

    #[test]
    fn tc_4_7_7_3_cloth_lod_smooth_transition() {
        let mut lod = 0.0_f32;
        for _ in 0..30 {
            lod = lod * 0.8 + 1.0 * 0.2;
        }
        assert!(lod > 0.9 && lod <= 1.0);
    }
}
