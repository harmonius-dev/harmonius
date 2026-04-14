//! Reynolds-style flocking forces from local neighbor samples.

use glam::Vec3;

/// Combine separation, alignment, and cohesion from `neighbors` `(position, velocity)` pairs.
pub fn compute_flocking(
    position: Vec3,
    velocity: Vec3,
    neighbors: &[(Vec3, Vec3)],
    separation_weight: f32,
    alignment_weight: f32,
    cohesion_weight: f32,
) -> Vec3 {
    if neighbors.is_empty() {
        return Vec3::ZERO;
    }
    let count = neighbors.len() as f32;
    let mut separation = Vec3::ZERO;
    let mut alignment = Vec3::ZERO;
    let mut center = Vec3::ZERO;

    for &(n_pos, n_vel) in neighbors {
        let diff = position - n_pos;
        let dist = diff.length();
        if dist > f32::EPSILON {
            separation += diff / (dist * dist);
        }
        alignment += n_vel;
        center += n_pos;
    }

    alignment = alignment / count - velocity;
    center = center / count - position;

    separation * separation_weight + alignment * alignment_weight + center * cohesion_weight
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_neighbors(
        positions: &[Vec3],
        velocities: &[Vec3],
        i: usize,
        radius: f32,
    ) -> Vec<(Vec3, Vec3)> {
        let mut out = Vec::new();
        for j in 0..positions.len() {
            if i == j {
                continue;
            }
            if positions[i].distance(positions[j]) <= radius {
                out.push((positions[j], velocities[j]));
            }
        }
        out
    }

    #[test]
    fn tc_7_7_1_1_flocking_separation() {
        let n = 50_usize;
        let radius = 12.0_f32;
        let sep_d = 0.5_f32;
        let mut pos: Vec<Vec3> = Vec::new();
        let mut vel = vec![Vec3::ZERO; n];
        for i in 0..n {
            let a = i as f32 * 0.35;
            pos.push(Vec3::new(a.cos(), 0.0, a.sin()) * 3.0);
        }
        for _ in 0..300 {
            let snapshot = pos.clone();
            for i in 0..n {
                let nbs = brute_neighbors(&snapshot, &vel, i, radius);
                let f = compute_flocking(snapshot[i], vel[i], &nbs, 2.5, 0.4, 0.2);
                vel[i] = (vel[i] + f * 0.04).clamp_length_max(2.0);
            }
            for i in 0..n {
                pos[i] += vel[i] * 0.04;
            }
        }
        let mut min_pair = f32::MAX;
        for i in 0..n {
            for j in i + 1..n {
                min_pair = min_pair.min(pos[i].distance(pos[j]));
            }
        }
        assert!(min_pair > sep_d);
    }

    #[test]
    fn tc_7_7_1_2_flocking_cohesion() {
        let n = 50_usize;
        let cohesion_radius = 20.0_f32;
        let neighbor_radius = 25.0_f32;
        let mut pos: Vec<Vec3> = Vec::new();
        let mut vel = vec![Vec3::ZERO; n];
        for i in 0..n {
            let a = i as f32 * 0.9;
            pos.push(Vec3::new(a.cos(), 0.0, a.sin()) * 8.0);
        }
        for _ in 0..300 {
            let snapshot = pos.clone();
            for i in 0..n {
                let nbs = brute_neighbors(&snapshot, &vel, i, neighbor_radius);
                let f = compute_flocking(snapshot[i], vel[i], &nbs, 1.2, 0.3, 0.8);
                vel[i] = (vel[i] + f * 0.05).clamp_length_max(3.0);
            }
            for i in 0..n {
                pos[i] += vel[i] * 0.05;
            }
        }
        let mut c = Vec3::ZERO;
        for p in &pos {
            c += *p;
        }
        c /= n as f32;
        for p in &pos {
            assert!(p.distance(c) < cohesion_radius);
        }
    }
}
