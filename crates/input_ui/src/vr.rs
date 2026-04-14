//! Panel-local **2D** ray vs axis-aligned box tests (projection stub for IR-4.2.6).
//!
//! World-space panel transforms and 3D rays are owned by engine input; this module keeps the
//! deterministic math used by integration tests.

use crate::types::{Rect, Vec2};

/// Axis-aligned panel in the same 2D coordinate space as the cast ray.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelSpec {
    /// Panel bounds in panel-local logical pixels.
    pub bounds: Rect,
}

/// Returns local panel coordinates when the ray hits the panel rectangle.
pub fn ray_panel_hit(origin: Vec2, dir: Vec2, panel: PanelSpec) -> Option<Vec2> {
    if dir.x.abs() < f32::EPSILON && dir.y.abs() < f32::EPSILON {
        return None;
    }

    let mut t_near = f32::NEG_INFINITY;
    let mut t_far = f32::INFINITY;

    // X slab
    if dir.x.abs() < f32::EPSILON {
        if origin.x < panel.bounds.min.x || origin.x > panel.bounds.max.x {
            return None;
        }
    } else {
        let inv = 1.0 / dir.x;
        let mut t0 = (panel.bounds.min.x - origin.x) * inv;
        let mut t1 = (panel.bounds.max.x - origin.x) * inv;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_near = t_near.max(t0);
        t_far = t_far.min(t1);
        if t_near > t_far {
            return None;
        }
    }

    // Y slab
    if dir.y.abs() < f32::EPSILON {
        if origin.y < panel.bounds.min.y || origin.y > panel.bounds.max.y {
            return None;
        }
    } else {
        let inv = 1.0 / dir.y;
        let mut t0 = (panel.bounds.min.y - origin.y) * inv;
        let mut t1 = (panel.bounds.max.y - origin.y) * inv;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_near = t_near.max(t0);
        t_far = t_far.min(t1);
        if t_near > t_far {
            return None;
        }
    }

    let t_hit = if t_near >= 0.0 {
        t_near
    } else if t_far >= 0.0 {
        t_far
    } else {
        return None;
    };

    let hit = Vec2::new(origin.x + dir.x * t_hit, origin.y + dir.y * t_hit);
    if panel.bounds.contains(hit) {
        Some(hit)
    } else {
        None
    }
}
