//! Deterministic reference harness for the AI ↔ physics integration design.
//!
//! This crate encodes the public contracts (`WalkabilityQuery`, `JumpArcQuery`, …) and ships
//! CPU-only scene surrogates used by the companion test-case document.

#![deny(clippy::all)]
#![deny(unsafe_code)]
// Public structs intentionally mirror `docs/design/integration/ai-physics.md` field-for-field.
#![allow(missing_docs)]

pub mod avoidance;
pub mod channel;
pub mod climb;
pub mod geometry;
pub mod grounded;
pub mod jump_arc;
pub mod los;
pub mod metrics;
pub mod types;
pub mod walkability;

pub use avoidance::{
    enumerate_avoidance, AvoidanceQuery, AvoidanceResult, NeighborState, PhysicsBody, RigidBodyKind,
};
pub use channel::{drain_jump_arcs, drain_walkability, AiNavQueryChannel, NavRequest};
pub use climb::{ClimbScene, ClimbSweepQuery, ClimbSweepResult, CylinderClimbFixture};
pub use geometry::{AxisAlignedBox, Plane};
pub use grounded::{airborne_state, read_ai_grounded_state, AiGroundedState, FootContact};
pub use jump_arc::{parabola_point, JumpArcQuery, JumpArcResult, JumpArcScene, JumpArcTraceStats};
pub use los::{LineOfSightQuery, LineOfSightResult, LineOfSightScene};
pub use metrics::FallbackMetrics;
pub use types::{AgentMask, Entity, MaterialId};
pub use walkability::{
    PlanarPatch, UnwalkableReason, WalkabilityQuery, WalkabilityResult, WalkabilityScene,
};

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;

    fn flat_ground_scene() -> WalkabilityScene {
        WalkabilityScene {
            bvh_ready: true,
            surfaces: vec![PlanarPatch {
                plane: Plane::from_point_normal(Vec3::ZERO, Vec3::Y),
                center: Vec3::ZERO,
                xz_extent: 50.0,
                material: MaterialId(0),
                entity: Entity(1),
            }],
        }
    }

    #[test]
    fn tc_ir_2_5_1_1_walkable_flat_ground() {
        let scene = flat_ground_scene();
        let mut fm = FallbackMetrics::default();
        let res = scene.resolve(
            WalkabilityQuery {
                request_id: 1,
                foot_position: Vec3::new(0.0, 0.01, 0.0),
                max_slope_deg: 45.0,
                agent_mask: AgentMask::default().with_material(MaterialId(0)),
            },
            &mut fm,
        );
        assert!(res.walkable);
        assert_eq!(res.reason, UnwalkableReason::None);
        assert!((res.surface_normal - Vec3::Y).length() < 1e-3);
    }

    #[test]
    fn tc_ir_2_5_1_2_slope_too_steep() {
        let normal = Vec3::new(
            0.0,
            70.0_f32.to_radians().cos(),
            70.0_f32.to_radians().sin(),
        );
        let plane = Plane::from_point_normal(Vec3::ZERO, normal);
        let scene = WalkabilityScene {
            bvh_ready: true,
            surfaces: vec![PlanarPatch {
                plane,
                center: Vec3::ZERO,
                xz_extent: 10.0,
                material: MaterialId(0),
                entity: Entity(2),
            }],
        };
        let mut fm = FallbackMetrics::default();
        let foot = plane.normal * 0.01;
        let res = scene.resolve(
            WalkabilityQuery {
                request_id: 2,
                foot_position: foot,
                max_slope_deg: 45.0,
                agent_mask: AgentMask::default().with_material(MaterialId(0)),
            },
            &mut fm,
        );
        assert!(!res.walkable);
        assert_eq!(res.reason, UnwalkableReason::SlopeTooSteep);
    }

    #[test]
    fn tc_ir_2_5_1_3_no_surface() {
        let scene = flat_ground_scene();
        let mut fm = FallbackMetrics::default();
        let res = scene.resolve(
            WalkabilityQuery {
                request_id: 3,
                foot_position: Vec3::new(100.0, 50.0, 100.0),
                max_slope_deg: 45.0,
                agent_mask: AgentMask::default().with_material(MaterialId(0)),
            },
            &mut fm,
        );
        assert!(!res.walkable);
        assert_eq!(res.reason, UnwalkableReason::NoSurface);
    }

    #[test]
    fn tc_ir_2_5_1_4_material_excluded() {
        let scene = WalkabilityScene {
            bvh_ready: true,
            surfaces: vec![PlanarPatch {
                plane: Plane::from_point_normal(Vec3::ZERO, Vec3::Y),
                center: Vec3::ZERO,
                xz_extent: 50.0,
                material: MaterialId(1),
                entity: Entity(3),
            }],
        };
        let mut fm = FallbackMetrics::default();
        let res = scene.resolve(
            WalkabilityQuery {
                request_id: 4,
                foot_position: Vec3::new(0.0, 0.01, 0.0),
                max_slope_deg: 45.0,
                agent_mask: AgentMask::default().with_material(MaterialId(0)),
            },
            &mut fm,
        );
        assert!(!res.walkable);
        assert_eq!(res.reason, UnwalkableReason::MaterialExcluded);
    }

    #[test]
    fn tc_ir_2_5_1_5_request_id_round_trip() {
        let scene = flat_ground_scene();
        let mut channel = AiNavQueryChannel::new();
        let mut fm = FallbackMetrics::default();
        for id in 0..64 {
            channel.send(
                NavRequest::Walkability(WalkabilityQuery {
                    request_id: id,
                    foot_position: Vec3::new(0.0, 0.01, 0.0),
                    max_slope_deg: 45.0,
                    agent_mask: AgentMask::default().with_material(MaterialId(0)),
                }),
                &mut fm,
            );
        }
        let replies = drain_walkability(&mut channel, &scene, &mut fm);
        assert_eq!(replies.len(), 64);
        for (idx, reply) in replies.iter().enumerate() {
            assert_eq!(reply.request_id, idx as u64);
        }
    }

    #[test]
    fn tc_ir_2_5_1_n1_bvh_missing_fm1() {
        let scene = WalkabilityScene {
            bvh_ready: false,
            surfaces: vec![],
        };
        let mut fm = FallbackMetrics::default();
        let res = scene.resolve(
            WalkabilityQuery {
                request_id: 9,
                foot_position: Vec3::new(0.0, 0.01, 0.0),
                max_slope_deg: 45.0,
                agent_mask: AgentMask::default().with_material(MaterialId(0)),
            },
            &mut fm,
        );
        assert_eq!(res.reason, UnwalkableReason::NoSurface);
        assert_eq!(fm.fm1_bvh_missing, 1);
    }

    #[test]
    fn tc_ir_2_5_1_n2_channel_drop_oldest_fm3() {
        let scene = flat_ground_scene();
        let mut channel = AiNavQueryChannel::new();
        let mut fm = FallbackMetrics::default();
        for id in 0..400 {
            channel.send(
                NavRequest::Walkability(WalkabilityQuery {
                    request_id: id,
                    foot_position: Vec3::new(0.0, 0.01, 0.0),
                    max_slope_deg: 45.0,
                    agent_mask: AgentMask::default().with_material(MaterialId(0)),
                }),
                &mut fm,
            );
        }
        assert_eq!(channel.len(), 256);
        assert_eq!(fm.fm3_channel_drop_oldest, 144);
        let replies = drain_walkability(&mut channel, &scene, &mut fm);
        assert_eq!(replies.len(), 256);
    }

    #[test]
    fn tc_ir_2_5_2_1_unobstructed_jump_landing() {
        let scene = JumpArcScene {
            blockers: vec![],
            ground_y: Some(0.0),
        };
        let query = JumpArcQuery {
            start: Vec3::ZERO,
            initial_velocity: Vec3::new(5.0, 5.0, 0.0),
            gravity: Vec3::new(0.0, -9.81, 0.0),
            segment_count: 16,
            segment_dt: 0.08,
        };
        let res = scene.trace(query);
        assert_eq!(res.blocked_segment, 0);
        assert!(res.landing.is_some());
        let landing = res.landing.expect("landing");
        assert!(landing.y.abs() < 1e-2, "landing.y={}", landing.y);
    }

    #[test]
    fn tc_ir_2_5_2_2_wall_blocks_jump() {
        let wall = AxisAlignedBox::new(Vec3::new(2.9, -10.0, -10.0), Vec3::new(3.1, 10.0, 10.0));
        let scene = JumpArcScene {
            blockers: vec![(Entity(42), wall)],
            ground_y: Some(0.0),
        };
        let query = JumpArcQuery {
            start: Vec3::new(0.0, 1.0, 0.0),
            initial_velocity: Vec3::new(8.0, 0.0, 0.0),
            gravity: Vec3::new(0.0, -9.81, 0.0),
            segment_count: 64,
            segment_dt: 0.02,
        };
        let res = scene.trace(query);
        assert!(res.landing.is_none());
        assert!(res.blocked_segment > 0);
        assert_eq!(res.blocker, Entity(42));
    }

    #[test]
    fn tc_ir_2_5_2_3_ceiling_blocks_jump() {
        let ceiling = AxisAlignedBox::new(Vec3::new(-2.0, 1.2, -2.0), Vec3::new(8.0, 1.4, 2.0));
        let scene = JumpArcScene {
            blockers: vec![(Entity(7), ceiling)],
            ground_y: Some(0.0),
        };
        let query = JumpArcQuery {
            start: Vec3::ZERO,
            initial_velocity: Vec3::new(2.0, 6.0, 0.0),
            gravity: Vec3::new(0.0, -9.81, 0.0),
            segment_count: 64,
            segment_dt: 0.02,
        };
        let res = scene.trace(query);
        assert!(res.landing.is_none());
        assert_eq!(res.blocker, Entity(7));
    }

    #[test]
    fn tc_ir_2_5_2_4_arc_respects_gravity_vector() {
        let g = Vec3::new(0.0, -9.81, 0.0);
        let v0 = Vec3::new(0.0, 5.0, 0.0);
        let start = Vec3::ZERO;
        for i in 1..=16 {
            let t = 0.01 * i as f32;
            let y = parabola_point(start, v0, g, t).y;
            let expected = v0.y * t + 0.5 * g.y * t * t;
            assert!(
                (y - expected).abs() < 1e-3,
                "t={t} y={y} expected={expected}"
            );
        }
    }

    #[test]
    fn tc_ir_2_5_2_n1_zero_segments_no_raycasts() {
        let scene = JumpArcScene::default();
        let mut stats = JumpArcTraceStats::default();
        let res = scene.trace_instrumented(
            JumpArcQuery {
                start: Vec3::ZERO,
                initial_velocity: Vec3::ONE,
                gravity: Vec3::new(0.0, -1.0, 0.0),
                segment_count: 0,
                segment_dt: 0.01,
            },
            &mut stats,
        );
        assert!(res.landing.is_none());
        assert_eq!(stats.segment_raycasts, 0);
    }

    #[test]
    fn tc_ir_2_5_3_1_ledge_within_height() {
        let ledge = AxisAlignedBox::new(Vec3::new(0.0, 1.2, -1.0), Vec3::new(1.5, 1.25, 1.0));
        let scene = ClimbScene { wall_x: 0.0, ledge };
        let res = scene.sweep(ClimbSweepQuery {
            origin: Vec3::new(0.29, 0.0, 0.0),
            direction: Vec3::Y,
            capsule_radius: 0.3,
            max_height: 1.5,
        });
        let top = res.ledge_top.expect("ledge");
        assert!((top.y - 1.25).abs() < 1e-2);
        assert!((res.climb_height - 1.25).abs() < 1e-2);
    }

    #[test]
    fn tc_ir_2_5_3_2_ledge_above_max_height() {
        let ledge = AxisAlignedBox::new(Vec3::new(0.0, 2.0, -1.0), Vec3::new(1.5, 2.05, 1.0));
        let scene = ClimbScene { wall_x: 0.0, ledge };
        let res = scene.sweep(ClimbSweepQuery {
            origin: Vec3::new(0.29, 0.0, 0.0),
            direction: Vec3::Y,
            capsule_radius: 0.3,
            max_height: 1.5,
        });
        assert!(res.ledge_top.is_none());
    }

    #[test]
    fn tc_ir_2_5_3_n1_zero_direction_is_panic_free() {
        let ledge = AxisAlignedBox::new(Vec3::new(0.0, 1.2, -1.0), Vec3::new(1.5, 1.25, 1.0));
        let scene = ClimbScene { wall_x: 0.0, ledge };
        let res = scene.sweep(ClimbSweepQuery {
            origin: Vec3::new(0.29, 0.0, 0.0),
            direction: Vec3::ZERO,
            capsule_radius: 0.3,
            max_height: 1.5,
        });
        assert!(res.ledge_top.is_none());
    }

    #[test]
    fn tc_ir_2_5_3_3_curved_wall_tangent() {
        let fixture = CylinderClimbFixture {
            radius: 1.0,
            ledge_height: 1.1,
        };
        let agent_xz = Vec3::new(2.0, 0.0, 0.5);
        let top = fixture.tangent_ledge_top(agent_xz);
        let expected_len = (agent_xz.x * agent_xz.x + agent_xz.z * agent_xz.z).sqrt();
        let scale = fixture.radius / expected_len;
        let expected = Vec3::new(agent_xz.x * scale, fixture.ledge_height, agent_xz.z * scale);
        assert!((top - expected).length() < 1e-4);
    }

    #[test]
    fn tc_ir_2_5_4_1_neighbors_within_radius() {
        let mut bodies = Vec::new();
        for i in 0..10 {
            bodies.push(PhysicsBody {
                entity: Entity(i + 1),
                position: Vec3::new(i as f32 * 0.2, 0.0, 0.0),
                radius: 0.5,
                body: RigidBodyKind::Dynamic { linvel: Vec3::ZERO },
            });
        }
        for i in 0..5 {
            bodies.push(PhysicsBody {
                entity: Entity(100 + i),
                position: Vec3::new(50.0 + i as f32, 0.0, 0.0),
                radius: 0.5,
                body: RigidBodyKind::Dynamic { linvel: Vec3::ZERO },
            });
        }
        let mut fm = FallbackMetrics::default();
        let res = enumerate_avoidance(
            &bodies,
            AvoidanceQuery {
                center: Vec3::ZERO,
                radius: 5.0,
                self_entity: Entity(999),
            },
            &mut fm,
        );
        assert_eq!(res.neighbors.len(), 10);
    }

    #[test]
    fn tc_ir_2_5_4_2_self_excluded() {
        let bodies = [PhysicsBody {
            entity: Entity(1),
            position: Vec3::ZERO,
            radius: 0.5,
            body: RigidBodyKind::Dynamic { linvel: Vec3::ZERO },
        }];
        let mut fm = FallbackMetrics::default();
        let res = enumerate_avoidance(
            &bodies,
            AvoidanceQuery {
                center: Vec3::ZERO,
                radius: 5.0,
                self_entity: Entity(1),
            },
            &mut fm,
        );
        assert!(res.neighbors.is_empty());
    }

    #[test]
    fn tc_ir_2_5_4_3_truncates_to_sixteen_fm6() {
        let mut bodies = Vec::new();
        for i in 0..32 {
            bodies.push(PhysicsBody {
                entity: Entity(i + 1),
                position: Vec3::new(i as f32 * 0.05, 0.0, 0.0),
                radius: 0.2,
                body: RigidBodyKind::Dynamic { linvel: Vec3::ZERO },
            });
        }
        let mut fm = FallbackMetrics::default();
        let res = enumerate_avoidance(
            &bodies,
            AvoidanceQuery {
                center: Vec3::ZERO,
                radius: 10.0,
                self_entity: Entity(999),
            },
            &mut fm,
        );
        assert_eq!(res.neighbors.len(), 16);
        assert_eq!(fm.fm6_avoidance_truncation, 16);
    }

    #[test]
    fn tc_ir_2_5_4_4_neighbor_velocity_matches() {
        let bodies = [PhysicsBody {
            entity: Entity(2),
            position: Vec3::new(1.0, 0.0, 0.0),
            radius: 0.5,
            body: RigidBodyKind::Dynamic {
                linvel: Vec3::new(3.0, 0.0, 0.0),
            },
        }];
        let mut fm = FallbackMetrics::default();
        let res = enumerate_avoidance(
            &bodies,
            AvoidanceQuery {
                center: Vec3::ZERO,
                radius: 5.0,
                self_entity: Entity(999),
            },
            &mut fm,
        );
        assert_eq!(res.neighbors[0].velocity, Vec3::new(3.0, 0.0, 0.0));
    }

    #[test]
    fn tc_ir_2_5_4_n1_empty_scene() {
        let mut fm = FallbackMetrics::default();
        let res = enumerate_avoidance(
            &[],
            AvoidanceQuery {
                center: Vec3::ZERO,
                radius: 5.0,
                self_entity: Entity(1),
            },
            &mut fm,
        );
        assert!(res.neighbors.is_empty());
    }

    #[test]
    fn tc_ir_2_5_5_1_grounded_with_contact() {
        let contacts = [FootContact {
            ground_normal: Vec3::Y,
            ground_entity: Entity(5),
        }];
        let mut fm = FallbackMetrics::default();
        let state = read_ai_grounded_state(Some(&contacts), airborne_state(), &mut fm);
        assert!(state.grounded);
        assert_eq!(state.ground_entity, Entity(5));
    }

    #[test]
    fn tc_ir_2_5_5_2_airborne_empty_contacts() {
        let mut fm = FallbackMetrics::default();
        let state = read_ai_grounded_state(
            Some(&[]),
            AiGroundedState {
                grounded: true,
                ground_normal: Vec3::Y,
                ground_entity: Entity(1),
            },
            &mut fm,
        );
        assert!(!state.grounded);
        assert_eq!(fm.fm4_reuse_grounded, 0);
    }

    #[test]
    fn tc_ir_2_5_5_3_ground_normal_matches_slope() {
        let n = Vec3::new(0.0, 0.7, 0.7).normalize();
        let contacts = [FootContact {
            ground_normal: n,
            ground_entity: Entity(2),
        }];
        let mut fm = FallbackMetrics::default();
        let state = read_ai_grounded_state(Some(&contacts), airborne_state(), &mut fm);
        assert!((state.ground_normal - n).length() < 1e-4);
    }

    #[test]
    fn tc_ir_2_5_5_n1_missing_contact_list_reuses_prev_fm4() {
        let prev = AiGroundedState {
            grounded: true,
            ground_normal: Vec3::Y,
            ground_entity: Entity(3),
        };
        let mut fm = FallbackMetrics::default();
        let state = read_ai_grounded_state(None, prev, &mut fm);
        assert_eq!(state, prev);
        assert_eq!(fm.fm4_reuse_grounded, 1);
    }

    #[test]
    fn tc_ir_2_5_6_1_los_visible() {
        let scene = LineOfSightScene::default();
        let res = scene.query(LineOfSightQuery {
            observer: Vec3::ZERO,
            target: Vec3::new(5.0, 0.0, 0.0),
        });
        assert_eq!(res, LineOfSightResult::Visible);
    }

    #[test]
    fn tc_ir_2_5_6_2_los_occluded() {
        let wall = AxisAlignedBox::new(Vec3::new(2.0, -2.0, -2.0), Vec3::new(2.5, 2.0, 2.0));
        let scene = LineOfSightScene {
            occluders: vec![(Entity(8), wall)],
        };
        let res = scene.query(LineOfSightQuery {
            observer: Vec3::ZERO,
            target: Vec3::new(5.0, 0.0, 0.0),
        });
        assert_eq!(res, LineOfSightResult::Occluded { blocker: Entity(8) });
    }
}
