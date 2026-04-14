//! Minimal 2D rigid-body stepping for deterministic tests.

use glam::Vec2;

use crate::types::{BodyType2d, RigidBody2d};

/// Integrate gravity for a lone dynamic body with Euler steps (`TC-10.5.10.1`).
pub fn step_dynamic_gravity(body: &mut RigidBody2d, world_gravity: Vec2, dt: f32, steps: u32) {
    if body.body_type != BodyType2d::Dynamic {
        return;
    }
    let g = world_gravity * body.gravity_scale;
    for _ in 0..steps {
        body.velocity += g * dt;
    }
}

/// Vertical motion against a one-way platform at `y = platform_y` (`TC-10.5.10.2`).
pub fn step_one_way_platform(
    y: &mut f32,
    vy: &mut f32,
    dt: f32,
    platform_y: f32,
    one_way: bool,
    drop_through: bool,
) {
    if !one_way {
        return;
    }
    let next_y = *y + *vy * dt;
    if drop_through {
        *y = next_y;
        return;
    }
    // Land only when falling onto the platform from above.
    if *vy < 0.0 && *y > platform_y && next_y <= platform_y {
        *y = platform_y;
        *vy = 0.0;
    } else {
        *y = next_y;
    }
}

/// Parameters for [`swept_circle_hits_vertical_wall`].
#[derive(Clone, Copy, Debug)]
pub struct SweptCircleWall {
    /// Circle center X at the start of the step.
    pub x: f32,
    /// Linear velocity X in world units per second.
    pub vx: f32,
    /// Circle radius in world units.
    pub radius: f32,
    /// Vertical slab center X.
    pub wall_x: f32,
    /// Half thickness of the slab along X.
    pub wall_half_width: f32,
    /// Timestep in seconds.
    pub dt: f32,
}

/// Swept circle vs infinite vertical slab; returns hit fraction in `[0,1]` (`TC-10.5.10.3`).
#[must_use]
pub fn swept_circle_hits_vertical_wall(s: SweptCircleWall) -> Option<f32> {
    if s.vx <= 0.0 {
        return None;
    }
    let wall_left = s.wall_x - s.wall_half_width;
    let travel = s.vx * s.dt;
    let hit_x = wall_left - s.radius;
    if s.x >= hit_x || s.x + travel < hit_x {
        return None;
    }
    let t = ((hit_x - s.x) / travel).clamp(0.0, 1.0);
    Some(t)
}
