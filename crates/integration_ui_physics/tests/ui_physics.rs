//! Integration tests for UI ↔ physics CPU harness (`TC-IR-4.3.*`).

use integration_ui_physics::{
    compute_reticle_snap, resolve_pick_through_channel, resolve_world_pick, AimableTarget,
    CameraFixture, CameraId, CameraTable, CollisionMask, DepthReadbackLatch, Entity,
    FallbackMetrics, LocalizedStringId, PhysicsPickScene, PickBody, PickCameraRig, ProjectFlags,
    TooltipComponent, TooltipUiState, UiPickChannel, Vec2, Vec3, WorldPickRequest,
    WorldProjectBatch, WorldProjectRequest, L_DEFAULT, L_ENEMY,
};

const CAM0: CameraId = CameraId(0);

fn box_at(center: Vec3, half: f32, layer: u32, entity: u64) -> PickBody {
    PickBody {
        entity: Entity(entity),
        min: center
            + Vec3 {
                x: -half,
                y: -half,
                z: -half,
            },
        max: center
            + Vec3 {
                x: half,
                y: half,
                z: half,
            },
        collision_layer: layer,
    }
}

fn scene_cube_ir4311() -> PhysicsPickScene {
    PhysicsPickScene {
        bvh_ready: true,
        bodies: vec![box_at(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            0.25,
            L_DEFAULT,
            101,
        )],
    }
}

#[test]
fn tc_ir_431_1_cursor_pick_hits_box() {
    let scene = scene_cube_ir4311();
    let rig = PickCameraRig::standard_origin();
    let cam = CameraFixture::harness_default(CAM0);
    let mut m = FallbackMetrics::default();
    let req = WorldPickRequest {
        request_id: 1,
        camera: CAM0,
        cursor_ndc: Vec2 { x: 0.0, y: 0.0 },
        ray_mask: CollisionMask(L_DEFAULT),
        max_distance: 100.0,
    };
    let r = resolve_world_pick(
        &scene,
        &rig,
        req,
        cam.tan_half_fov_y(),
        cam.viewport_width / cam.viewport_height,
        &mut m,
    );
    assert_eq!(r.entity, Some(Entity(101)));
    assert!((r.world_position.z + 4.75).abs() < 0.6);
    assert!((r.normal.z - 1.0).abs() < 0.2 || (r.normal.z + 1.0).abs() < 0.2);
}

#[test]
fn tc_ir_431_2_cursor_pick_miss() {
    let scene = PhysicsPickScene {
        bvh_ready: true,
        bodies: vec![],
    };
    let rig = PickCameraRig::standard_origin();
    let cam = CameraFixture::harness_default(CAM0);
    let mut m = FallbackMetrics::default();
    let req = WorldPickRequest {
        request_id: 2,
        camera: CAM0,
        cursor_ndc: Vec2 { x: 0.0, y: 0.0 },
        ray_mask: CollisionMask(L_DEFAULT),
        max_distance: 100.0,
    };
    let r = resolve_world_pick(
        &scene,
        &rig,
        req,
        cam.tan_half_fov_y(),
        cam.viewport_width / cam.viewport_height,
        &mut m,
    );
    assert_eq!(r.entity, None);
}

#[test]
fn tc_ir_431_3_ray_mask_excludes_layer() {
    let scene = PhysicsPickScene {
        bvh_ready: true,
        bodies: vec![box_at(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            0.25,
            L_ENEMY,
            7,
        )],
    };
    let rig = PickCameraRig::standard_origin();
    let cam = CameraFixture::harness_default(CAM0);
    let mut m = FallbackMetrics::default();
    let req = WorldPickRequest {
        request_id: 3,
        camera: CAM0,
        cursor_ndc: Vec2 { x: 0.0, y: 0.0 },
        ray_mask: CollisionMask(L_DEFAULT),
        max_distance: 100.0,
    };
    let r = resolve_world_pick(
        &scene,
        &rig,
        req,
        cam.tan_half_fov_y(),
        cam.viewport_width / cam.viewport_height,
        &mut m,
    );
    assert_eq!(r.entity, None);
}

#[test]
fn tc_ir_431_4_nearest_hit_wins() {
    let scene = PhysicsPickScene {
        bvh_ready: true,
        bodies: vec![
            box_at(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                0.2,
                L_DEFAULT,
                50,
            ),
            box_at(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -3.0,
                },
                0.2,
                L_DEFAULT,
                51,
            ),
        ],
    };
    let rig = PickCameraRig::standard_origin();
    let cam = CameraFixture::harness_default(CAM0);
    let mut m = FallbackMetrics::default();
    let req = WorldPickRequest {
        request_id: 4,
        camera: CAM0,
        cursor_ndc: Vec2 { x: 0.0, y: 0.0 },
        ray_mask: CollisionMask(L_DEFAULT),
        max_distance: 100.0,
    };
    let r = resolve_world_pick(
        &scene,
        &rig,
        req,
        cam.tan_half_fov_y(),
        cam.viewport_width / cam.viewport_height,
        &mut m,
    );
    assert_eq!(r.entity, Some(Entity(51)));
}

#[test]
fn tc_ir_432_1_world_position_projects() {
    let cam = CameraFixture::harness_default(CAM0);
    let table = CameraTable::single(cam.clone());
    let depth = DepthReadbackLatch::default();
    let mut m = FallbackMetrics::default();
    let reqs = [WorldProjectRequest {
        camera: CAM0,
        world_position: Vec3 {
            x: 0.0,
            y: 1.0,
            z: -5.0,
        },
        flags: ProjectFlags::NONE,
    }];
    let out = WorldProjectBatch::project_all(&reqs, &table, &depth, &mut m);
    assert_eq!(out.len(), 1);
    let s = out[0].screen_position;
    assert!((s.x - cam.viewport_width * 0.5).abs() < cam.viewport_width * 0.15);
    assert!(s.y < cam.viewport_height * 0.35);
    assert!(out[0].visible);
}

#[test]
fn tc_ir_432_2_clamp_off_screen_right() {
    let cam = CameraFixture::harness_default(CAM0);
    let table = CameraTable::single(cam.clone());
    let depth = DepthReadbackLatch::default();
    let mut m = FallbackMetrics::default();
    let reqs = [WorldProjectRequest {
        camera: CAM0,
        world_position: Vec3 {
            x: 80.0,
            y: 0.0,
            z: -2.0,
        },
        flags: ProjectFlags::CLAMP_TO_SCREEN,
    }];
    let out = WorldProjectBatch::project_all(&reqs, &table, &depth, &mut m);
    assert!((out[0].screen_position.x - cam.viewport_width).abs() < 1.0);
}

#[test]
fn tc_ir_432_3_visible_false_behind_geometry() {
    let cam = CameraFixture::harness_default(CAM0);
    let table = CameraTable::single(cam.clone());
    let mut depth = DepthReadbackLatch::default();
    depth.commit_frame(0, 0.05);
    depth.set_current_frame(0);
    let mut m = FallbackMetrics::default();
    let reqs = [WorldProjectRequest {
        camera: CAM0,
        world_position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -60.0,
        },
        flags: ProjectFlags::NEED_VISIBILITY,
    }];
    let out = WorldProjectBatch::project_all(&reqs, &table, &depth, &mut m);
    assert!(!out[0].visible);
}

#[test]
fn tc_ir_432_4_batch_order_stable() {
    let cam = CameraFixture::harness_default(CAM0);
    let table = CameraTable::single(cam);
    let depth = DepthReadbackLatch::default();
    let mut m = FallbackMetrics::default();
    let mut reqs = Vec::new();
    for i in 0..64 {
        reqs.push(WorldProjectRequest {
            camera: CAM0,
            world_position: Vec3 {
                x: i as f32 * 0.01,
                y: 0.0,
                z: -4.0,
            },
            flags: ProjectFlags::NONE,
        });
    }
    let out = WorldProjectBatch::project_all(&reqs, &table, &depth, &mut m);
    assert_eq!(out.len(), 64);
    for (i, row) in out.iter().enumerate() {
        let back = WorldProjectBatch::project_all(
            std::slice::from_ref(&reqs[i]),
            &table,
            &depth,
            &mut FallbackMetrics::default(),
        );
        assert_eq!(row.screen_position.x, back[0].screen_position.x);
    }
}

#[test]
fn tc_ir_433_1_hover_with_tooltip_shows() {
    let tip = TooltipComponent {
        title: LocalizedStringId(1),
        body: LocalizedStringId(2),
        hover_delay_s: 0.2,
    };
    let mut state = TooltipUiState::new();
    let e = Entity(9);
    let mut acc = 0.0_f32;
    let mut published = None;
    while acc < 0.25 {
        let out = state.update(0.05, Some(e), |id| if id == e { Some(tip) } else { None });
        if out.is_some() {
            published = out;
            break;
        }
        acc += 0.05;
    }
    assert!(published.is_some());
}

#[test]
fn tc_ir_433_2_hover_without_tooltip_hides() {
    let mut state = TooltipUiState::new();
    let e = Entity(9);
    let mut acc = 0.0_f32;
    while acc < 0.5 {
        let out = state.update(0.05, Some(e), |_id| None);
        assert!(out.is_none());
        acc += 0.05;
    }
}

#[test]
fn tc_ir_433_3_hover_delay_respected() {
    let tip = TooltipComponent {
        title: LocalizedStringId(3),
        body: LocalizedStringId(4),
        hover_delay_s: 0.2,
    };
    let mut state = TooltipUiState::new();
    let e = Entity(11);
    let _ = state.update(0.19, Some(e), |_id| Some(tip));
    assert!(state
        .update(0.01, Some(Entity(12)), |_id| Some(tip))
        .is_none());
}

#[test]
fn tc_ir_434_1_diegetic_panel_behind_wall_hidden() {
    let cam = CameraFixture::harness_default(CAM0);
    let table = CameraTable::single(cam.clone());
    let mut depth = DepthReadbackLatch::default();
    depth.commit_frame(0, 0.08);
    depth.set_current_frame(0);
    let mut m = FallbackMetrics::default();
    let reqs = [WorldProjectRequest {
        camera: CAM0,
        world_position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -120.0,
        },
        flags: ProjectFlags::NEED_VISIBILITY,
    }];
    let out = WorldProjectBatch::project_all(&reqs, &table, &depth, &mut m);
    assert!(!out[0].visible);
}

#[test]
fn tc_ir_435_1_reticle_snaps_nearest() {
    let origin = Vec3::ZERO;
    let dir = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let targets = [
        AimableTarget {
            entity: Entity(1),
            world_position: Vec3 {
                x: -0.2,
                y: 0.0,
                z: -3.0,
            },
        },
        AimableTarget {
            entity: Entity(2),
            world_position: Vec3 {
                x: 0.2,
                y: 0.0,
                z: -5.0,
            },
        },
    ];
    let snap = compute_reticle_snap(origin, dir, &targets, 50.0, 0.5);
    assert_eq!(snap.target, Some(Entity(1)));
}

#[test]
fn tc_ir_435_2_confidence_falls_with_distance() {
    let origin = Vec3::ZERO;
    let dir = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let near = compute_reticle_snap(
        origin,
        dir,
        &[AimableTarget {
            entity: Entity(1),
            world_position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
        }],
        20.0,
        0.5,
    );
    let far = compute_reticle_snap(
        origin,
        dir,
        &[AimableTarget {
            entity: Entity(1),
            world_position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -10.0,
            },
        }],
        20.0,
        0.5,
    );
    assert!(near.confidence > far.confidence);
}

#[test]
fn tc_ir_435_3_no_target_in_cone() {
    let origin = Vec3::ZERO;
    let dir = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let targets = [AimableTarget {
        entity: Entity(1),
        world_position: Vec3 {
            x: 20.0,
            y: 0.0,
            z: -2.0,
        },
    }];
    let snap = compute_reticle_snap(origin, dir, &targets, 50.0, 0.999);
    assert_eq!(snap.target, None);
}

#[test]
fn tc_ir_431_n1_cursor_outside_window() {
    let scene = scene_cube_ir4311();
    let rig = PickCameraRig::standard_origin();
    let cam = CameraFixture::harness_default(CAM0);
    let mut m = FallbackMetrics::default();
    let req = WorldPickRequest {
        request_id: 10,
        camera: CAM0,
        cursor_ndc: Vec2 { x: 1.5, y: 0.0 },
        ray_mask: CollisionMask(L_DEFAULT),
        max_distance: 100.0,
    };
    let r = resolve_world_pick(
        &scene,
        &rig,
        req,
        cam.tan_half_fov_y(),
        cam.viewport_width / cam.viewport_height,
        &mut m,
    );
    assert_eq!(r.entity, None);
    assert_eq!(m.fm1_cursor_outside, 1);
}

#[test]
fn tc_ir_431_n2_bvh_not_built() {
    let mut scene = scene_cube_ir4311();
    scene.bvh_ready = false;
    let rig = PickCameraRig::standard_origin();
    let cam = CameraFixture::harness_default(CAM0);
    let mut m = FallbackMetrics::default();
    let req = WorldPickRequest {
        request_id: 11,
        camera: CAM0,
        cursor_ndc: Vec2 { x: 0.0, y: 0.0 },
        ray_mask: CollisionMask(L_DEFAULT),
        max_distance: 100.0,
    };
    let r = resolve_world_pick(
        &scene,
        &rig,
        req,
        cam.tan_half_fov_y(),
        cam.viewport_width / cam.viewport_height,
        &mut m,
    );
    assert_eq!(r.entity, None);
    assert_eq!(m.fm2_bvh_not_built, 1);
}

#[test]
fn tc_ir_432_n1_unknown_camera() {
    let cam = CameraFixture::harness_default(CAM0);
    let table = CameraTable::single(cam);
    let depth = DepthReadbackLatch::default();
    let mut m = FallbackMetrics::default();
    let reqs = [WorldProjectRequest {
        camera: CameraId(99),
        world_position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -5.0,
        },
        flags: ProjectFlags::NONE,
    }];
    let out = WorldProjectBatch::project_all(&reqs, &table, &depth, &mut m);
    assert_eq!(out[0].screen_position, Vec2 { x: 0.0, y: 0.0 });
    assert!(!out[0].visible);
    assert_eq!(m.fm6_unknown_camera, 1);
}

#[test]
fn tc_ir_432_n2_depth_readback_stale() {
    let cam = CameraFixture::harness_default(CAM0);
    let table = CameraTable::single(cam);
    let mut depth = DepthReadbackLatch::default();
    depth.commit_frame(0, 0.5);
    depth.set_current_frame(3);
    let mut m = FallbackMetrics::default();
    let reqs = [WorldProjectRequest {
        camera: CAM0,
        world_position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -5.0,
        },
        flags: ProjectFlags::NONE,
    }];
    let _ = WorldProjectBatch::project_all(&reqs, &table, &depth, &mut m);
    assert_eq!(m.fm3_depth_stale, 1);
}

#[test]
fn tc_ir_431_n3_ch27_overflow() {
    let scene = scene_cube_ir4311();
    let rig = PickCameraRig::standard_origin();
    let cam = CameraFixture::harness_default(CAM0);
    let mut ch = UiPickChannel::new();
    let mut m = FallbackMetrics::default();
    for i in 0..32 {
        ch.push(
            WorldPickRequest {
                request_id: i,
                camera: CAM0,
                cursor_ndc: Vec2 { x: 0.0, y: 0.0 },
                ray_mask: CollisionMask(L_DEFAULT),
                max_distance: 100.0,
            },
            &mut m,
        );
    }
    assert_eq!(m.fm4_ch27_drop, 24);
    let results = resolve_pick_through_channel(
        &mut ch,
        &scene,
        &rig,
        cam.tan_half_fov_y(),
        cam.viewport_width / cam.viewport_height,
        &mut FallbackMetrics::default(),
    );
    assert_eq!(results.len(), 8);
}
