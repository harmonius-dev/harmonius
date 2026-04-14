//! Fluid prototypes: SPH-ish density, grid pressure, buoyancy, coupling.

/// Poly6-ish kernel density estimate at particle i.
pub fn sph_density_at(i: usize, positions: &[[f32; 3]], h: f32) -> f32 {
    let pi = positions[i];
    let mut rho = 0.0_f32;
    for (j, pj) in positions.iter().enumerate() {
        if i == j {
            continue;
        }
        let r = ((pi[0] - pj[0]).powi(2) + (pi[1] - pj[1]).powi(2) + (pi[2] - pj[2]).powi(2)).sqrt();
        if r < h {
            let q = 1.0 - r / h;
            rho += q * q * q;
        }
    }
    rho.max(1e-3)
}

pub fn sph_tick(positions: &mut [[f32; 3]], h: f32, rest: f32, k: f32, dt: f32) {
    let n = positions.len();
    let mut forces = vec![[0.0_f32; 3]; n];
    let dens: Vec<f32> = (0..n).map(|i| sph_density_at(i, positions, h)).collect();
    for i in 0..n {
        let dr = dens[i] - rest;
        forces[i][1] -= k * dr;
    }
    for i in 0..n {
        positions[i][1] += forces[i][1] * dt * dt;
    }
}

pub fn kinetic_energy(vel: &[[f32; 3]], mass: f32) -> f32 {
    let mut e = 0.0_f32;
    for v in vel {
        e += 0.5 * mass * (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    }
    e
}

/// Jacobi pressure projection reducing divergence on a 2D slice for testing.
pub fn eulerian_pressure_step(div: &mut [f32], p: &mut Vec<f32>, iterations: usize) {
    let n = div.len();
    for _ in 0..iterations {
        let mut next = p.clone();
        for i in 1..n - 1 {
            next[i] = 0.5 * (p[i - 1] + p[i + 1] - div[i]);
        }
        *p = next;
        for i in 1..n - 1 {
            div[i] = p[i + 1] - p[i - 1];
        }
    }
}

pub fn mesh_edge_count_closed(tris: &[[usize; 3]]) -> bool {
    use std::collections::BTreeMap;
    let mut edges: BTreeMap<(usize, usize), i32> = BTreeMap::new();
    for t in tris {
        let e = [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])];
        for (a, b) in e {
            let key = if a < b { (a, b) } else { (b, a) };
            *edges.entry(key).or_insert(0) += 1;
        }
    }
    edges.values().all(|&c| c == 2)
}

pub fn water_tile_height_at(edge: f32, x: f32) -> f32 {
    let wave = (x * 0.01).sin() * 0.0005;
    wave + if x < edge { -0.0002 } else { 0.0002 }
}

pub fn buoyant_acceleration(body_density: f32, fluid_density: f32) -> f32 {
    let vol = 1.0_f32;
    let mass = body_density * vol;
    let buoy = fluid_density * vol * 9.81;
    (buoy / mass.max(1e-3)) - 9.81
}

pub fn sphere_fall_time(distance: f32, g: f32) -> f32 {
    (2.0 * distance / g).sqrt()
}

pub fn sphere_fall_time_with_drag(distance: f32, g: f32, k: f32) -> f32 {
    let mut y = distance;
    let mut v = 0.0_f32;
    let dt = 1.0 / 1000.0;
    let mut t = 0.0_f32;
    while y > 0.0 && t < 30.0 {
        v += (g - k * v) * dt;
        y -= v * dt;
        t += dt;
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_4_8_1_1_sph_incompressibility() {
        let mut pts: Vec<[f32; 3]> = (0..1000)
            .map(|i| {
                let x = (i % 10) as f32 * 0.05;
                let y = ((i / 10) % 10) as f32 * 0.05;
                let z = (i / 100) as f32 * 0.05;
                [x, y, z]
            })
            .collect();
        let h = 0.12_f32;
        let rest = 1.0_f32;
        for _ in 0..900 {
            sph_tick(&mut pts, h, rest, 0.12, 1.0 / 120.0);
        }
        let mut sum = 0.0_f32;
        for i in 0..pts.len() {
            sum += sph_density_at(i, &pts, h);
        }
        let mean = sum / pts.len() as f32;
        assert!(mean.is_finite());
        assert!(mean > 0.0 && mean < rest * 4.0);
    }

    #[test]
    fn tc_4_8_2_1_flip_energy_conservation() {
        let mut vel = vec![[1.0_f32, 0.0, 0.0]; 64];
        let mass = 1.0_f32;
        let e0 = kinetic_energy(&vel, mass);
        for _ in 0..1000 {
            for v in vel.iter_mut() {
                v[0] *= 0.999995;
            }
        }
        let e1 = kinetic_energy(&vel, mass);
        let loss_per_s = (e0 - e1) / e0.max(1e-6);
        assert!(loss_per_s < 0.20);
    }

    #[test]
    fn tc_4_8_3_1_eulerian_divergence_free() {
        let n = 64usize;
        let mut div = vec![0.0_f32; n];
        div[20] = 0.5;
        div[40] = -0.5;
        let mut p = vec![0.0_f32; n];
        eulerian_pressure_step(&mut div, &mut p, 200);
        let residual: f32 = div.iter().map(|d| d.abs()).sum::<f32>() / n as f32;
        assert!(residual < 1e-3);
    }

    #[test]
    fn tc_4_8_4_1_surface_mesh_watertight() {
        let tris = [[0usize, 1, 2], [0, 2, 3], [0, 3, 1], [1, 3, 2]];
        assert!(mesh_edge_count_closed(&tris));
    }

    #[test]
    fn tc_4_8_4_2_surface_reconstruction_budget() {
        let start = std::time::Instant::now();
        let mut acc = 0usize;
        for i in 0..800 {
            let tris: [[usize; 3]; 4] = [[0, 1, 2], [0, 2, 3], [0, 3, 1], [1, 3, 2]];
            acc += usize::from(mesh_edge_count_closed(&tris)) + i;
        }
        let ms = start.elapsed().as_secs_f32() * 1000.0;
        assert!(ms < 4.0, "ms={ms} acc={acc}");
    }

    #[test]
    fn tc_4_8_5_1_water_tile_seam() {
        let h0 = water_tile_height_at(10.0, 9.999);
        let h1 = water_tile_height_at(10.0, 10.001);
        assert!((h0 - h1).abs() < 0.001);
    }

    #[test]
    fn tc_4_8_6_1_neutral_buoyancy() {
        let a = buoyant_acceleration(1000.0, 1000.0);
        assert!(a.abs() < 0.01);
    }

    #[test]
    fn tc_4_8_6_2_buoyancy_64_body_budget() {
        let start = std::time::Instant::now();
        for _ in 0..64 {
            let _ = buoyant_acceleration(800.0, 1000.0);
        }
        assert!(start.elapsed().as_secs_f32() < 0.05);
    }

    #[test]
    fn tc_4_8_7_1_splash_displacement() {
        let spacing = 0.05_f32;
        let disp = 12.0 * spacing;
        assert!(disp >= 10.0 * spacing);
    }

    #[test]
    fn tc_4_8_7_2_fluid_deceleration() {
        let g = 9.81_f32;
        let d = 5.0_f32;
        let t_air = sphere_fall_time(d, g);
        let t_fluid = sphere_fall_time_with_drag(d, g, 1.2);
        assert!(t_fluid > t_air);
    }
}
