//! Single-tick physics debug pass (IR-3.4.5 early-out + IR-3.4.1–4 emitters).

use crate::buffer::DebugDrawBuffer;
use crate::bvh_debug::{bvh_debug_lines, BvhNode};
use crate::config::PhysicsDebugConfig;
use crate::contacts::{contact_debug_lines, ContactManifold};
use crate::ray_debug::{ray_debug_lines, shape_cast_debug_lines, RayCastResult, ShapeCastResult};
use crate::types::{ColliderShape, LinearColor, RigidBodyType, Transform};
use crate::wireframe::collider_wireframe_lines;

/// Minimal world snapshot for the debug pass (flat ECS mirror).
#[derive(Clone, Debug, Default)]
pub struct PhysicsDebugWorldSnapshot {
    pub colliders: Vec<(ColliderShape, RigidBodyType, Transform)>,
    pub manifolds: Vec<ContactManifold>,
    pub bvh_nodes: Vec<BvhNode>,
    pub raycasts: Vec<RayCastResult>,
    pub shapecasts: Vec<ShapeCastResult>,
}

/// Fills `buf` from `snapshot` when any draw flag is set; otherwise returns immediately.
pub fn run_physics_debug_system(
    cfg: &PhysicsDebugConfig,
    buf: &mut DebugDrawBuffer,
    snapshot: &PhysicsDebugWorldSnapshot,
) {
    if !cfg.any_draw_enabled() {
        return;
    }
    let max = cfg.max_debug_lines;
    if cfg.draw_colliders {
        for (shape, body, xf) in &snapshot.colliders {
            collider_wireframe_lines(buf, max, shape, *body, *xf, cfg);
        }
    }
    if cfg.draw_contacts {
        contact_debug_lines(
            buf,
            max,
            &snapshot.manifolds,
            cfg.contact_normal_scale,
            cfg.contact_point_dot_radius,
            LinearColor::new(1.0, 0.5, 0.0, 1.0),
            LinearColor::new(0.0, 0.8, 1.0, 1.0),
        );
    }
    if cfg.draw_bvh {
        bvh_debug_lines(
            buf,
            max,
            &snapshot.bvh_nodes,
            cfg.bvh_max_depth,
            cfg.bvh_color_leaf,
            cfg.bvh_color_internal,
        );
    }
    if cfg.draw_raycasts {
        ray_debug_lines(
            buf,
            max,
            &snapshot.raycasts,
            LinearColor::new(0.0, 1.0, 0.0, 1.0),
            LinearColor::new(1.0, 0.0, 0.0, 1.0),
        );
    }
    if cfg.draw_shapecasts {
        shape_cast_debug_lines(buf, max, &snapshot.shapecasts, 16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ColliderShape, Transform};
    use glam::Vec3;

    #[test]
    fn tc_ir_3_4_5_1_runtime_toggle_off_zero_lines() {
        let mut cfg = PhysicsDebugConfig::default_colors();
        cfg.draw_colliders = false;
        let mut buf = DebugDrawBuffer::new(256);
        let snap = PhysicsDebugWorldSnapshot {
            colliders: vec![(
                ColliderShape::Sphere { radius: 1.0 },
                RigidBodyType::Dynamic,
                Transform::from_translation(Vec3::ZERO),
            )],
            ..Default::default()
        };
        run_physics_debug_system(&cfg, &mut buf, &snap);
        assert!(buf.lines.is_empty());
    }

    #[test]
    fn tc_ir_3_4_5_2_runtime_toggle_on_emits_lines() {
        let mut cfg = PhysicsDebugConfig::default_colors();
        cfg.draw_colliders = true;
        let mut buf = DebugDrawBuffer::new(4096);
        let snap = PhysicsDebugWorldSnapshot {
            colliders: vec![(
                ColliderShape::Sphere { radius: 1.0 },
                RigidBodyType::Dynamic,
                Transform::from_translation(Vec3::ZERO),
            )],
            ..Default::default()
        };
        run_physics_debug_system(&cfg, &mut buf, &snap);
        assert!(!buf.lines.is_empty());
    }

    #[test]
    fn tc_ir_3_4_5_3_shipping_build_has_config() {
        let _ = PhysicsDebugConfig::default_colors();
    }

    #[test]
    fn tc_ir_3_4_n2_stale_contacts_cleared_by_caller() {
        let mut cfg = PhysicsDebugConfig::default_colors();
        cfg.draw_contacts = true;
        let mut buf = DebugDrawBuffer::new(256);
        let m = ContactManifold {
            normal: Vec3::Y,
            penetration: 0.1,
            points: vec![crate::contacts::ContactPoint {
                position: Vec3::ZERO,
            }],
        };
        let snap = PhysicsDebugWorldSnapshot {
            manifolds: vec![m],
            ..Default::default()
        };
        run_physics_debug_system(&cfg, &mut buf, &snap);
        assert!(!buf.lines.is_empty());
        buf.clear();
        run_physics_debug_system(&cfg, &mut buf, &PhysicsDebugWorldSnapshot::default());
        assert!(buf.lines.is_empty());
    }

    #[test]
    fn tc_ir_3_4_n3_bvh_depth_clamp() {
        let mut cfg = PhysicsDebugConfig::default_colors();
        cfg.draw_bvh = true;
        cfg.bvh_max_depth = 3;
        let mut buf = DebugDrawBuffer::new(4096);
        let nodes: Vec<BvhNode> = (0..10)
            .map(|d| BvhNode {
                min: Vec3::splat(d as f32),
                max: Vec3::splat(d as f32 + 1.0),
                depth: d,
                is_leaf: d == 9,
            })
            .collect();
        let snap = PhysicsDebugWorldSnapshot {
            bvh_nodes: nodes,
            ..Default::default()
        };
        run_physics_debug_system(&cfg, &mut buf, &snap);
        assert!(buf.lines.len() < 10 * 12);
    }
}
