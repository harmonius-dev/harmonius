//! Point light falloff and simple shadow tests.

use glam::Vec2;

/// Quadratic smoothstep falloff: `(1 - d / radius)^2` for `d < radius` (`TC-10.5.14.1`).
#[must_use]
pub fn light_intensity_quadratic(distance: f32, radius: f32) -> f32 {
    if radius <= 0.0 || distance >= radius {
        return 0.0;
    }
    let x = 1.0 - distance / radius;
    x * x
}

/// Returns `true` if the segment `light -> sample` intersects `polygon` edges before reaching the
/// sample (`TC-10.5.14.2`).
#[must_use]
pub fn segment_hits_polygon_first(light: Vec2, sample: Vec2, polygon: &[Vec2]) -> bool {
    if polygon.len() < 2 {
        return false;
    }
    let dir = sample - light;
    let max_t = dir.length();
    if max_t < 1e-8 {
        return false;
    }
    let ndir = dir / max_t;
    let mut best_t = max_t;
    let n = polygon.len();
    for i in 0..n {
        let p0 = polygon[i];
        let p1 = polygon[(i + 1) % n];
        if let Some(t_hit) = ray_segment_intersect(light, ndir, max_t, p0, p1) {
            if t_hit > 1e-4 && t_hit < best_t - 1e-4 {
                best_t = t_hit;
            }
        }
    }
    best_t + 1e-3 < max_t
}

fn ray_segment_intersect(origin: Vec2, dir: Vec2, max_t: f32, a: Vec2, b: Vec2) -> Option<f32> {
    let v1 = origin - a;
    let v2 = b - a;
    let v3 = Vec2::new(-dir.y, dir.x);
    let denom = v2.dot(v3);
    if denom.abs() < 1e-8 {
        return None;
    }
    let t1 = cross_vec2(v2, v1) / denom;
    let t2 = v1.dot(v3) / denom;
    if (0.0..=max_t).contains(&t1) && (0.0..=1.0).contains(&t2) {
        Some(t1)
    } else {
        None
    }
}

fn cross_vec2(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}
