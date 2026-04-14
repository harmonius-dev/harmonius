//! CPU-side regression coverage for rendering ↔ camera integration contracts.

#![allow(clippy::too_many_arguments)]

use std::f32::consts::FRAC_PI_4;

use glam::{Mat4, Quat, Vec3, Vec4};

use crate::{
    aabb_contains, build_render_view_from_camera, clamp_drs_scale, drs_pi_step,
    effective_render_layers, extract_sorted_render_views, filter_visible_entities,
    projection_matrix, render_view_from_camera_message, reset_warning_gates_for_tests,
    resolve_post_process_stack, scale_viewport, send_render_views_to_channel,
    view_uniform_from_render_view, CameraOutput, DrsFeedbackQueue, DynamicResolutionState, Entity,
    PostProcessBlend, PostProcessParams, PostProcessVolume, Projection, Viewport,
};

fn mrc(matrix: &Mat4, row: usize, col: usize) -> f32 {
    matrix.to_cols_array_2d()[col][row]
}

fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

fn approx_mat(a: Mat4, b: Mat4, eps: f32) -> bool {
    a.to_cols_array_2d()
        .iter()
        .zip(b.to_cols_array_2d().iter())
        .all(|(ca, cb)| {
            ca.iter()
                .zip(cb.iter())
                .all(|(xa, xb)| (*xa - *xb).abs() <= eps)
        })
}

fn cam(
    brain: u32,
    stable: u32,
    active: bool,
    pos: Vec3,
    rot: Quat,
    projection: Projection,
    layers: u32,
    order: i32,
    viewport: Viewport,
) -> CameraOutput {
    CameraOutput {
        brain: Entity(brain),
        stable_index: stable,
        active,
        position: pos,
        rotation: rot,
        projection,
        render_layers: layers,
        render_order: order,
        viewport,
    }
}

#[test]
fn tc_ir_3_1_1_1_perspective_snapshot_matrix_entries() {
    reset_warning_gates_for_tests();
    let projection = Projection::Perspective {
        fov_y_radians: 1.047,
        aspect: 16.0 / 9.0,
        near: 0.1,
        far: 1000.0,
    };
    let output = cam(
        1,
        0,
        true,
        Vec3::new(0.0, 5.0, -10.0),
        Quat::IDENTITY,
        projection,
        1,
        0,
        Viewport {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        },
    );
    let snap = build_render_view_from_camera(&output, 1.0, 0.5).expect("snapshot");
    let p = snap.projection;
    assert!(approx_eq(mrc(&p, 2, 2), 0.0, 1e-3));
    assert!(approx_eq(mrc(&p, 2, 3), 0.1, 1e-3));
    let t = snap.view_matrix.w_axis.truncate();
    assert!(approx_eq(t.x, 0.0, 1e-4));
    assert!(approx_eq(t.y, -5.0, 1e-4));
    assert!(approx_eq(t.z, 10.0, 1e-4));
}

#[test]
fn tc_ir_3_1_1_2_orthographic_maps_corner_to_ndc_unit_xy() {
    reset_warning_gates_for_tests();
    let projection = Projection::Orthographic {
        half_height: 10.0,
        aspect: 2.0,
        near: -1.0,
        far: 1.0,
    };
    let matrix = projection_matrix(&projection);
    assert!(approx_eq(mrc(&matrix, 0, 0), 0.05, 1e-5));
    assert!(approx_eq(mrc(&matrix, 1, 1), 0.1, 1e-5));
    let clip = matrix * Vec4::new(20.0, 10.0, 0.0, 1.0);
    assert!(approx_eq(clip.x, 1.0, 1e-5));
    assert!(approx_eq(clip.y, 1.0, 1e-5));
}

#[test]
fn tc_ir_3_1_2_1_render_layers_single_bit() {
    reset_warning_gates_for_tests();
    let mask = 0x01u32;
    let entities = vec![
        (Entity(1), 0x01u32),
        (Entity(2), 0x02u32),
    ];
    let visible = filter_visible_entities(mask, &entities);
    assert_eq!(visible, vec![Entity(1)]);
}

#[test]
fn tc_ir_3_1_2_2_render_layers_ff_sees_all() {
    reset_warning_gates_for_tests();
    let mask = 0xFFu32;
    let entities = vec![
        (Entity(1), 0x01),
        (Entity(2), 0x02),
        (Entity(3), 0x04),
        (Entity(4), 0x08),
    ];
    let visible = filter_visible_entities(mask, &entities);
    assert_eq!(visible.len(), 4);
}

#[test]
fn tc_ir_3_1_3_1_single_volume_exposure_override() {
    reset_warning_gates_for_tests();
    let vol = PostProcessVolume {
        entity: Entity(10),
        min: Vec3::new(-1.0, -1.0, -1.0),
        max: Vec3::new(1.0, 1.0, 1.0),
        priority: 0,
        params: PostProcessParams {
            exposure: 2.0,
            ..PostProcessParams::default()
        },
    };
    let blends = vec![PostProcessBlend {
        volume: Entity(10),
        weight: 1.0,
        priority: 0,
    }];
    let inside = resolve_post_process_stack(Vec3::ZERO, &blends, &[vol], |_| true);
    assert!(approx_eq(inside.resolved.exposure, 2.0, 1e-5));
    let outside = resolve_post_process_stack(Vec3::new(5.0, 0.0, 0.0), &blends, &[vol], |_| true);
    assert!(approx_eq(outside.resolved.exposure, 1.0, 1e-5));
}

#[test]
fn tc_ir_3_1_3_2_overlapping_weighted_average() {
    reset_warning_gates_for_tests();
    let v1 = PostProcessVolume {
        entity: Entity(1),
        min: Vec3::splat(-2.0),
        max: Vec3::splat(2.0),
        priority: 0,
        params: PostProcessParams {
            exposure: 1.0,
            ..PostProcessParams::default()
        },
    };
    let v2 = PostProcessVolume {
        entity: Entity(2),
        min: Vec3::splat(-2.0),
        max: Vec3::splat(2.0),
        priority: 0,
        params: PostProcessParams {
            exposure: 3.0,
            ..PostProcessParams::default()
        },
    };
    let blends = vec![
        PostProcessBlend {
            volume: Entity(1),
            weight: 0.6,
            priority: 0,
        },
        PostProcessBlend {
            volume: Entity(2),
            weight: 0.4,
            priority: 0,
        },
    ];
    let stack = resolve_post_process_stack(Vec3::ZERO, &blends, &[v1, v2], |_| true);
    assert!(approx_eq(stack.resolved.exposure, 1.8, 1e-5));
}

#[test]
fn tc_ir_3_1_3_3_nested_priority_wins() {
    reset_warning_gates_for_tests();
    let outer = PostProcessVolume {
        entity: Entity(1),
        min: Vec3::splat(-5.0),
        max: Vec3::splat(5.0),
        priority: 0,
        params: PostProcessParams {
            exposure: 0.5,
            contrast: 0.5,
            ..PostProcessParams::default()
        },
    };
    let inner = PostProcessVolume {
        entity: Entity(2),
        min: Vec3::splat(-1.0),
        max: Vec3::splat(1.0),
        priority: 1,
        params: PostProcessParams {
            exposure: 3.0,
            contrast: 2.0,
            ..PostProcessParams::default()
        },
    };
    let blends = vec![
        PostProcessBlend {
            volume: Entity(1),
            weight: 1.0,
            priority: 0,
        },
        PostProcessBlend {
            volume: Entity(2),
            weight: 1.0,
            priority: 0,
        },
    ];
    let stack = resolve_post_process_stack(Vec3::ZERO, &blends, &[outer, inner], |_| true);
    assert!(approx_eq(stack.resolved.exposure, 3.0, 1e-5));
    assert!(approx_eq(stack.resolved.contrast, 2.0, 1e-5));
}

#[test]
fn tc_ir_3_1_3_4_boundary_rule_is_deterministic() {
    reset_warning_gates_for_tests();
    let min = Vec3::new(0.0, 0.0, 0.0);
    let max = Vec3::new(1.0, 1.0, 1.0);
    assert!(aabb_contains(min, max, Vec3::new(0.0, 0.5, 0.5)));
    assert!(!aabb_contains(min, max, Vec3::new(1.0, 0.5, 0.5)));
}

#[test]
fn tc_ir_3_1_3_5_no_volumes_default_params() {
    reset_warning_gates_for_tests();
    let blends: Vec<PostProcessBlend> = Vec::new();
    let stack = resolve_post_process_stack(Vec3::ZERO, &blends, &[], |_| true);
    assert_eq!(stack.resolved, PostProcessParams::default());
}

#[test]
fn tc_ir_3_1_4_1_multi_camera_two_views() {
    reset_warning_gates_for_tests();
    let projection = Projection::Perspective {
        fov_y_radians: 1.0,
        aspect: 1.0,
        near: 0.1,
        far: 100.0,
    };
    let outputs = vec![
        cam(1, 0, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 0, Viewport::default()),
        cam(
            2,
            1,
            true,
            Vec3::new(1.0, 0.0, 0.0),
            Quat::IDENTITY,
            projection,
            1,
            0,
            Viewport::default(),
        ),
    ];
    let views = extract_sorted_render_views(&outputs, 1.0, 0.5);
    assert_eq!(views.len(), 2);
}

#[test]
fn tc_ir_3_1_4_2_positive_render_order_sorts() {
    reset_warning_gates_for_tests();
    let projection = Projection::Perspective {
        fov_y_radians: 1.0,
        aspect: 1.0,
        near: 0.1,
        far: 100.0,
    };
    let outputs = vec![
        cam(1, 0, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 1, Viewport::default()),
        cam(
            2,
            1,
            true,
            Vec3::ZERO,
            Quat::IDENTITY,
            projection,
            1,
            0,
            Viewport::default(),
        ),
    ];
    let views = extract_sorted_render_views(&outputs, 1.0, 0.5);
    assert_eq!(views[0].render_order, 0);
    assert_eq!(views[1].render_order, 1);
}

#[test]
fn tc_ir_3_1_4_3_negative_render_order_before_zero() {
    reset_warning_gates_for_tests();
    let projection = Projection::Perspective {
        fov_y_radians: 1.0,
        aspect: 1.0,
        near: 0.1,
        far: 100.0,
    };
    let outputs = vec![
        cam(1, 0, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 0, Viewport::default()),
        cam(
            2,
            1,
            true,
            Vec3::ZERO,
            Quat::IDENTITY,
            projection,
            1,
            -5,
            Viewport::default(),
        ),
    ];
    let views = extract_sorted_render_views(&outputs, 1.0, 0.5);
    assert_eq!(views[0].render_order, -5);
    assert_eq!(views[1].render_order, 0);
}

#[test]
fn tc_ir_3_1_4_4_equal_render_order_preserves_stable_index() {
    reset_warning_gates_for_tests();
    let projection = Projection::Perspective {
        fov_y_radians: 1.0,
        aspect: 1.0,
        near: 0.1,
        far: 100.0,
    };
    let outputs = vec![
        cam(3, 2, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 0, Viewport::default()),
        cam(1, 0, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 0, Viewport::default()),
        cam(2, 1, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 0, Viewport::default()),
    ];
    let views = extract_sorted_render_views(&outputs, 1.0, 0.5);
    assert_eq!(views[0].stable_index, 0);
    assert_eq!(views[1].stable_index, 1);
    assert_eq!(views[2].stable_index, 2);
}

#[test]
fn tc_ir_3_1_5_1_view_uniform_view_matches_inverse_world() {
    reset_warning_gates_for_tests();
    let projection = Projection::Perspective {
        fov_y_radians: 1.0,
        aspect: 1.0,
        near: 0.1,
        far: 100.0,
    };
    let output = cam(
        1,
        0,
        true,
        Vec3::new(1.0, 2.0, 3.0),
        Quat::from_rotation_y(FRAC_PI_4),
        projection,
        1,
        0,
        Viewport::default(),
    );
    let world = Mat4::from_rotation_translation(output.rotation, output.position);
    let snap = build_render_view_from_camera(&output, 1.0, 0.5).expect("snapshot");
    let rv = render_view_from_camera_message(&snap);
    let uniform = view_uniform_from_render_view(&rv);
    let expected_view = world.inverse();
    assert!(approx_mat(uniform.view, expected_view, 1e-4));
}

#[test]
fn tc_ir_3_1_5_2_reverse_z_entries() {
    reset_warning_gates_for_tests();
    let projection = Projection::Perspective {
        fov_y_radians: 1.0,
        aspect: 1.0,
        near: 0.1,
        far: 1000.0,
    };
    let matrix = projection_matrix(&projection);
    assert!(mrc(&matrix, 2, 2).abs() <= 1e-4);
    assert!(approx_eq(mrc(&matrix, 2, 3), 0.1, 1e-4));
}

#[test]
fn tc_ir_3_1_6_1_drs_scales_viewport() {
    reset_warning_gates_for_tests();
    let base = Viewport {
        x: 0,
        y: 0,
        width: 1920,
        height: 1080,
    };
    let scaled = scale_viewport(base, 0.75, 0.5);
    assert_eq!(scaled.width, 1440);
    assert_eq!(scaled.height, 810);
}

#[test]
fn tc_ir_3_1_6_2_drs_zero_clamps_to_min_scale() {
    reset_warning_gates_for_tests();
    let base = Viewport {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };
    let scaled = scale_viewport(base, 0.0, 0.5);
    assert_eq!(scaled.width, 400);
    assert_eq!(scaled.height, 300);
    assert!(scaled.width > 0 && scaled.height > 0);
}

#[test]
fn tc_ir_3_1_n1_no_active_camera_emits_fallback() {
    reset_warning_gates_for_tests();
    let outputs: Vec<CameraOutput> = Vec::new();
    let views = extract_sorted_render_views(&outputs, 1.0, 0.5);
    assert_eq!(views.len(), 1);
    assert_eq!(views[0].camera_position, Vec3::ZERO);
}

#[test]
fn tc_ir_3_1_n2_zero_fov_clamps_without_nan() {
    reset_warning_gates_for_tests();
    let projection = Projection::Perspective {
        fov_y_radians: 0.0,
        aspect: 1.0,
        near: 0.1,
        far: 100.0,
    };
    let output = cam(
        9,
        0,
        true,
        Vec3::ZERO,
        Quat::IDENTITY,
        projection,
        1,
        0,
        Viewport::default(),
    );
    let matrix = projection_matrix(&output.projection);
    assert!(!matrix.is_nan());
    let snap = build_render_view_from_camera(&output, 1.0, 0.5).expect("snapshot");
    assert!(!snap.projection.is_nan());
}

#[test]
fn tc_ir_3_1_n3_missing_volume_skipped() {
    reset_warning_gates_for_tests();
    let blends = vec![PostProcessBlend {
        volume: Entity(999),
        weight: 1.0,
        priority: 0,
    }];
    let stack = resolve_post_process_stack(Vec3::ZERO, &blends, &[], |_| false);
    assert_eq!(stack.resolved, PostProcessParams::default());
}

#[test]
fn tc_ir_3_1_n4_zero_mask_substitutes_all_bits() {
    reset_warning_gates_for_tests();
    let mask = effective_render_layers(0, Entity(1));
    assert_eq!(mask, 0xFFFF_FFFF);
    let entities = vec![(Entity(1), 0x01u32), (Entity(2), 0x02u32)];
    let visible = filter_visible_entities(mask, &entities);
    assert_eq!(visible.len(), 2);
}

#[test]
fn tc_ir_3_1_n5_duplicate_render_order_is_deterministic() {
    reset_warning_gates_for_tests();
    let projection = Projection::Perspective {
        fov_y_radians: 1.0,
        aspect: 1.0,
        near: 0.1,
        far: 100.0,
    };
    let outputs = vec![
        cam(1, 0, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 0, Viewport::default()),
        cam(2, 1, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 0, Viewport::default()),
        cam(3, 2, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 0, Viewport::default()),
        cam(4, 3, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 0, Viewport::default()),
    ];
    for _ in 0..1000 {
        let views = extract_sorted_render_views(&outputs, 1.0, 0.5);
        assert_eq!(
            views
                .iter()
                .map(|v| v.stable_index)
                .collect::<Vec<_>>(),
            vec![0, 1, 2, 3]
        );
    }
}

#[test]
fn tc_ir_3_1_n6_drs_queue_drops_oldest() {
    reset_warning_gates_for_tests();
    let queue = DrsFeedbackQueue::new();
    for idx in 0..10 {
        queue.push(DynamicResolutionState {
            scale: idx as f32,
            min_scale: 0.5,
            frame_ms: 16.6,
        });
    }
    let drained = queue.drain();
    assert_eq!(drained.len(), 4);
    assert!(approx_eq(drained[0].scale, 6.0, 1e-5));
    assert!(approx_eq(drained[3].scale, 9.0, 1e-5));
}

#[test]
fn channel_send_receives_two_views() {
    reset_warning_gates_for_tests();
    let (sender, receiver) = crossbeam_channel::unbounded();
    let projection = Projection::Perspective {
        fov_y_radians: 1.0,
        aspect: 1.0,
        near: 0.1,
        far: 100.0,
    };
    let outputs = vec![
        cam(1, 0, true, Vec3::ZERO, Quat::IDENTITY, projection, 1, 0, Viewport::default()),
        cam(
            2,
            1,
            true,
            Vec3::new(2.0, 0.0, 0.0),
            Quat::IDENTITY,
            projection,
            1,
            0,
            Viewport::default(),
        ),
    ];
    let views = extract_sorted_render_views(&outputs, 1.0, 0.5);
    send_render_views_to_channel(&views, &sender);
    let first = receiver.recv().expect("first");
    let second = receiver.recv().expect("second");
    assert_ne!(first.camera_position, second.camera_position);
}

#[test]
fn drs_pi_step_is_finite() {
    reset_warning_gates_for_tests();
    let next = drs_pi_step(1.0, 20.0, 16.6, 0.5);
    assert!(next.is_finite());
    assert!((0.5..=1.0).contains(&next));
}

#[test]
fn clamp_drs_respects_bounds() {
    reset_warning_gates_for_tests();
    assert!(approx_eq(clamp_drs_scale(0.0, 0.5), 0.5, 1e-6));
    assert!(approx_eq(clamp_drs_scale(1.2, 0.5), 1.0, 1e-6));
}
