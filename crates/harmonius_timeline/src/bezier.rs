//! CSS cubic-bezier easing helpers.

use crate::vectors::Vec2;

fn cubic_component(p0: f64, p1: f64, p2: f64, p3: f64, u: f64) -> f64 {
    let inv = 1.0 - u;
    inv * inv * inv * p0 + 3.0 * inv * inv * u * p1 + 3.0 * inv * u * u * p2 + u * u * u * p3
}

fn sample_x(c1: Vec2, c2: Vec2, u: f64) -> f64 {
    cubic_component(0.0, f64::from(c1.x), f64::from(c2.x), 1.0, u)
}

fn sample_y(c1: Vec2, c2: Vec2, u: f64) -> f64 {
    cubic_component(0.0, f64::from(c1.y), f64::from(c2.y), 1.0, u)
}

/// Maps a normalized segment parameter `t` in `[0, 1]` through a CSS cubic-bezier curve.
pub fn ease_t(t: f64, c1: Vec2, c2: Vec2) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    let mut lo = 0.0;
    let mut hi = 1.0;
    for _ in 0..32 {
        let mid = (lo + hi) * 0.5;
        let x = sample_x(c1, c2, mid);
        if x < t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let u = (lo + hi) * 0.5;
    sample_y(c1, c2, u)
}
