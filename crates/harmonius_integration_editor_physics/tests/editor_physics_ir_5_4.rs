//! Integration + unit coverage for `TC-IR-5.4.*` (editor ↔ physics).

use std::mem::size_of;

use harmonius_integration_editor_physics::{
    apply_box_half_extent_delta_x, apply_capsule_half_height_delta, apply_sphere_radius_delta,
    cap_contact_debug_entries, collider_debug_color_hex, collision_layers_need_warning,
    collision_layers_pair_generate_contacts, contact_point_world_a, contact_point_world_b,
    log_narrowphase_proxy, narrowphase_proxy_for_shape, pack_argb, trigger_event_if_overlapping,
    Collider, ColliderDebugData, ColliderEditCommand, ColliderShape, CollisionLayers, CommandError,
    CompoundChild, CompoundChildEditCommand, CompoundCollider, ContactDebugData, ContactPoint,
    ConvexHull, DebugWireframeCapture, EditorCommand, Entity, MaterialAssignCommand,
    NarrowphaseProxy, PhysicsMaterialHandle, PhysicsPreview, Quat, Transform, TriMesh, UndoStack,
    Vec3, World,
};

fn box_shape(hx: f32, hy: f32, hz: f32) -> ColliderShape {
    ColliderShape::Box {
        half_extents: Vec3 {
            x: hx,
            y: hy,
            z: hz,
        },
    }
}

#[test]
fn tc_ir_5_4_1_1_box_resize_gizmo() {
    let s = box_shape(1.0, 1.0, 1.0);
    let next = apply_box_half_extent_delta_x(&s, 0.5);
    match next {
        ColliderShape::Box { half_extents } => assert!((half_extents.x - 1.5).abs() < 1e-5),
        _ => panic!("expected box"),
    }
}

#[test]
fn tc_ir_5_4_1_2_sphere_radius_gizmo() {
    let s = ColliderShape::Sphere { radius: 1.0 };
    let next = apply_sphere_radius_delta(&s, 0.25);
    match next {
        ColliderShape::Sphere { radius } => assert!((radius - 1.25).abs() < 1e-5),
        _ => panic!("expected sphere"),
    }
}

#[test]
fn tc_ir_5_4_1_3_capsule_height_gizmo() {
    let s = ColliderShape::Capsule {
        half_height: 1.0,
        radius: 0.2,
    };
    let next = apply_capsule_half_height_delta(&s, 0.1);
    match next {
        ColliderShape::Capsule {
            half_height,
            radius,
        } => {
            assert!((half_height - 1.1).abs() < 1e-5);
            assert!((radius - 0.2).abs() < 1e-5);
        }
        _ => panic!("expected capsule"),
    }
}

#[test]
fn tc_ir_5_4_1_4_collider_edit_undo_redo() {
    let e = Entity(1);
    let mut world = World::new();
    let old = box_shape(1.0, 1.0, 1.0);
    world.insert_collider(
        e,
        Collider {
            shape: old.clone(),
            offset: Vec3::zero(),
            is_trigger: false,
        },
    );
    let new_shape = box_shape(1.5, 1.0, 1.0);
    let mut stack = UndoStack::new();
    let cmd = ColliderEditCommand {
        entity: e,
        old_shape: old.clone(),
        new_shape: new_shape.clone(),
    };
    stack.execute(&mut world, Box::new(cmd)).expect("execute");
    match &world.collider(e).expect("collider").shape {
        ColliderShape::Box { half_extents } => assert!((half_extents.x - 1.5).abs() < 1e-5),
        _ => panic!("expected box"),
    }
    stack.undo(&mut world).expect("undo");
    assert_eq!(world.collider(e).expect("collider").shape, old);
    stack.redo(&mut world).expect("redo");
    match &world.collider(e).expect("collider").shape {
        ColliderShape::Box { half_extents } => assert!((half_extents.x - 1.5).abs() < 1e-5),
        _ => panic!("expected box"),
    }
}

#[test]
fn tc_ir_5_4_2_1_wireframe_collider_overlay() {
    let e = Entity(2);
    let shape = box_shape(1.0, 0.5, 0.25);
    let data = ColliderDebugData {
        entity: e,
        shape: shape.clone(),
        is_trigger: false,
    };
    let mut cap = DebugWireframeCapture::default();
    cap.record_collider(&data, Vec3::zero(), false, false, false);
    let prim = cap.primitives.first().expect("prim");
    match prim {
        harmonius_integration_editor_physics::DebugDrawPrimitive::WireframeBox {
            color_rgba,
            ..
        } => assert_eq!(*color_rgba, pack_argb(0xFF, 0x00, 0xFF, 0x00)),
        _ => panic!("expected wireframe box"),
    }
}

#[test]
fn tc_ir_5_4_2_2_sleeping_body_color_change() {
    let e = Entity(3);
    let shape = box_shape(0.5, 0.5, 0.5);
    let _data = ColliderDebugData {
        entity: e,
        shape,
        is_trigger: false,
    };
    let active = collider_debug_color_hex(false, false, false, false);
    let sleeping = collider_debug_color_hex(true, false, false, false);
    assert_ne!(active, sleeping);
    assert_eq!(sleeping, pack_argb(0xFF, 0x80, 0x80, 0x80));
}

#[test]
fn tc_ir_5_4_2_3_compound_collider_display() {
    let e = Entity(4);
    let children = vec![
        (
            box_shape(0.1, 0.1, 0.1),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            false,
        ),
        (
            box_shape(0.2, 0.2, 0.2),
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            false,
        ),
        (
            box_shape(0.15, 0.15, 0.15),
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            false,
        ),
    ];
    let mut cap = DebugWireframeCapture::default();
    cap.record_compound_children(e, &children, Vec3::zero(), false, false);
    assert_eq!(cap.primitives.len(), 3);
}

#[test]
fn tc_ir_5_4_2_4_compound_child_edit_undo_redo() {
    let e = Entity(5);
    let mut world = World::new();
    let c0 = CompoundChild {
        shape: box_shape(0.5, 0.5, 0.5),
        offset: Vec3::zero(),
        rotation: Quat::identity(),
        layers: CollisionLayers::default(),
        material: PhysicsMaterialHandle(0),
    };
    let c1 = CompoundChild {
        shape: box_shape(0.2, 0.2, 0.2),
        offset: Vec3::zero(),
        rotation: Quat::identity(),
        layers: CollisionLayers::default(),
        material: PhysicsMaterialHandle(0),
    };
    let c2 = c1.clone();
    world.insert_compound(
        e,
        CompoundCollider {
            children: vec![c0.clone(), c1.clone(), c2.clone()],
        },
    );
    let new1 = CompoundChild {
        shape: box_shape(0.9, 0.1, 0.1),
        offset: Vec3::zero(),
        rotation: Quat::identity(),
        layers: CollisionLayers::default(),
        material: PhysicsMaterialHandle(0),
    };
    let mut stack = UndoStack::new();
    stack
        .execute(
            &mut world,
            Box::new(CompoundChildEditCommand {
                entity: e,
                child_index: 1,
                old_child: c1.clone(),
                new_child: new1.clone(),
            }),
        )
        .expect("execute");
    assert_eq!(world.compound(e).expect("compound").children[1], new1);
    assert_eq!(world.compound(e).expect("compound").children[0], c0);
    stack.undo(&mut world).expect("undo");
    assert_eq!(world.compound(e).expect("compound").children[1], c1);
}

#[test]
fn tc_ir_5_4_3_1_physics_play_mode() {
    let mut prev = PhysicsPreview::new(Vec3 {
        x: 0.0,
        y: -9.81,
        z: 0.0,
    });
    let e = Entity(10);
    prev.register_body(
        e,
        Vec3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        },
    );
    prev.play();
    prev.tick();
    let y1 = prev.position(e).expect("pos").y;
    prev.tick();
    let y2 = prev.position(e).expect("pos").y;
    assert!(y2 < y1);
}

#[test]
fn tc_ir_5_4_3_2_physics_pause_and_step() {
    let mut prev = PhysicsPreview::new(Vec3 {
        x: 0.0,
        y: -10.0,
        z: 0.0,
    });
    let e = Entity(11);
    prev.register_body(e, Vec3::zero());
    prev.play();
    prev.pause();
    let before = prev.position(e).expect("pos").y;
    prev.step_once();
    let after = prev.position(e).expect("pos").y;
    assert!(after < before);
}

#[test]
fn tc_ir_5_4_3_3_physics_stop_restores_state() {
    let mut prev = PhysicsPreview::new(Vec3 {
        x: 0.0,
        y: -10.0,
        z: 0.0,
    });
    let e = Entity(12);
    prev.register_body(
        e,
        Vec3 {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        },
    );
    prev.play();
    prev.tick();
    prev.tick();
    prev.stop();
    let y = prev.position(e).expect("pos").y;
    assert!((y - 5.0).abs() < 1e-4);
}

#[test]
fn tc_ir_5_4_4_1_contact_point_markers() {
    let mut cap = DebugWireframeCapture::default();
    let contacts = [ContactDebugData {
        point_a: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        point_b: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.01,
        },
        normal: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        depth: 0.001,
    }];
    cap.record_contact_manifold(
        contacts[0].normal,
        &contacts,
        pack_argb(0xFF, 0xFF, 0xFF, 0xFF),
        pack_argb(0xFF, 0x00, 0xFF, 0x00),
    );
    let points = cap
        .primitives
        .iter()
        .filter(|p| {
            matches!(
                p,
                harmonius_integration_editor_physics::DebugDrawPrimitive::Point { .. }
            )
        })
        .count();
    assert!(points >= 2);
}

#[test]
fn tc_ir_5_4_4_2_contact_normal_arrows() {
    let mut cap = DebugWireframeCapture::default();
    let c = ContactDebugData {
        point_a: Vec3::zero(),
        point_b: Vec3::zero(),
        normal: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        depth: 0.0,
    };
    cap.record_contact_manifold(
        c.normal,
        &[c],
        pack_argb(0xFF, 0xFF, 0xFF, 0xFF),
        pack_argb(0xFF, 0x00, 0x00, 0xFF),
    );
    assert!(cap.primitives.iter().any(|p| {
        matches!(
            p,
            harmonius_integration_editor_physics::DebugDrawPrimitive::Arrow { .. }
        )
    }));
}

#[test]
fn tc_ir_5_4_5_1_layer_mask_editing() {
    let mut layers = CollisionLayers {
        membership: 0b111,
        mask: 0b111,
    };
    layers.toggle_membership_bit(3);
    assert_eq!(layers.membership, 0b1111);
}

#[test]
fn tc_ir_5_4_5_2_layer_mask_preview() {
    let a = CollisionLayers {
        membership: 0b001,
        mask: 0b011,
    };
    let on_layer0 = CollisionLayers {
        membership: 0b001,
        mask: u32::MAX,
    };
    let on_layer1 = CollisionLayers {
        membership: 0b010,
        mask: u32::MAX,
    };
    assert!(collision_layers_pair_generate_contacts(&a, &on_layer0));
    assert!(collision_layers_pair_generate_contacts(&a, &on_layer1));
    let on_layer2_only = CollisionLayers {
        membership: 0b100,
        mask: u32::MAX,
    };
    assert!(!collision_layers_pair_generate_contacts(
        &a,
        &on_layer2_only
    ));
}

#[test]
fn tc_ir_5_4_6_1_trigger_volume_wireframe() {
    let e = Entity(20);
    let _data = ColliderDebugData {
        entity: e,
        shape: box_shape(0.5, 0.5, 0.5),
        is_trigger: true,
    };
    let c = collider_debug_color_hex(false, true, false, false);
    assert_eq!(c, pack_argb(0xFF, 0xFF, 0xFF, 0x00));
}

#[test]
fn tc_ir_5_4_6_2_trigger_overlap_event() {
    let ev = trigger_event_if_overlapping(Entity(1), Entity(2), true).expect("event");
    assert_eq!(ev.entity_a, Entity(1));
    assert_eq!(ev.entity_b, Entity(2));
}

#[test]
fn tc_ir_5_4_7_1_physics_material_drag_drop() {
    let mut world = World::new();
    let e = Entity(30);
    world.set_physics_material(e, PhysicsMaterialHandle(1));
    world.register_material_restitution(PhysicsMaterialHandle(1), 0.42);
    world.register_material_restitution(PhysicsMaterialHandle(2), 0.9);
    let mut cmd = MaterialAssignCommand {
        entity: e,
        old_material: PhysicsMaterialHandle(1),
        new_material: PhysicsMaterialHandle(2),
    };
    cmd.execute(&mut world).expect("execute");
    assert!((world.restitution_for_entity(e) - 0.9).abs() < 1e-5);
}

#[test]
fn tc_ir_5_4_7_2_physics_material_assign_undo_redo() {
    let mut world = World::new();
    let e = Entity(31);
    world.set_physics_material(e, PhysicsMaterialHandle(1));
    world.register_material_restitution(PhysicsMaterialHandle(1), 0.1);
    world.register_material_restitution(PhysicsMaterialHandle(2), 0.8);
    let mut stack = UndoStack::new();
    stack
        .execute(
            &mut world,
            Box::new(MaterialAssignCommand {
                entity: e,
                old_material: PhysicsMaterialHandle(1),
                new_material: PhysicsMaterialHandle(2),
            }),
        )
        .expect("execute");
    stack.undo(&mut world).expect("undo");
    assert_eq!(world.physics_material(e), PhysicsMaterialHandle(1));
    stack.redo(&mut world).expect("redo");
    assert_eq!(world.physics_material(e), PhysicsMaterialHandle(2));
}

#[test]
fn tc_ir_5_4_u1_collider_edit_command_execute_writes_new_shape() {
    let e = Entity(40);
    let mut world = World::new();
    let old = box_shape(1.0, 1.0, 1.0);
    world.insert_collider(
        e,
        Collider {
            shape: old.clone(),
            offset: Vec3::zero(),
            is_trigger: false,
        },
    );
    let new_shape = box_shape(2.0, 0.5, 0.5);
    let mut cmd = ColliderEditCommand {
        entity: e,
        old_shape: old,
        new_shape: new_shape.clone(),
    };
    cmd.execute(&mut world).expect("execute");
    assert_eq!(world.collider(e).expect("c").shape, new_shape);
}

#[test]
fn tc_ir_5_4_u2_collider_edit_command_undo_restores_old_shape() {
    let e = Entity(41);
    let mut world = World::new();
    let old = box_shape(1.0, 1.0, 1.0);
    world.insert_collider(
        e,
        Collider {
            shape: old.clone(),
            offset: Vec3::zero(),
            is_trigger: false,
        },
    );
    let new_shape = box_shape(0.25, 2.0, 2.0);
    let mut cmd = ColliderEditCommand {
        entity: e,
        old_shape: old.clone(),
        new_shape,
    };
    cmd.execute(&mut world).expect("execute");
    cmd.undo(&mut world).expect("undo");
    assert_eq!(world.collider(e).expect("c").shape, old);
}

#[test]
fn tc_ir_5_4_u3_collider_edit_command_size_bytes_includes_heap() {
    let verts: Vec<Vec3> = (0..10)
        .map(|i| Vec3 {
            x: i as f32,
            y: 0.0,
            z: 0.0,
        })
        .collect();
    let shape = ColliderShape::TriMesh(TriMesh {
        vertices: verts.clone(),
    });
    let cmd = ColliderEditCommand {
        entity: Entity(0),
        old_shape: shape.clone(),
        new_shape: shape,
    };
    let n = verts.len();
    assert!(cmd.size_bytes() >= n * size_of::<Vec3>());
}

#[test]
fn tc_ir_5_4_u4_compound_child_edit_command_undo_restores_child() {
    tc_ir_5_4_2_4_compound_child_edit_undo_redo();
}

#[test]
fn tc_ir_5_4_u5_material_assign_command_undo_restores_handle() {
    tc_ir_5_4_7_2_physics_material_assign_undo_redo();
}

#[test]
fn tc_ir_5_4_u6_color_scheme_maps_states_to_hex_values() {
    assert_eq!(
        collider_debug_color_hex(false, false, false, false),
        pack_argb(0xFF, 0x00, 0xFF, 0x00)
    );
    assert_eq!(
        collider_debug_color_hex(true, false, false, false),
        pack_argb(0xFF, 0x80, 0x80, 0x80)
    );
    assert_eq!(
        collider_debug_color_hex(false, true, false, false),
        pack_argb(0xFF, 0xFF, 0xFF, 0x00)
    );
    assert_eq!(
        collider_debug_color_hex(false, false, true, false),
        pack_argb(0xFF, 0x00, 0xFF, 0xFF)
    );
    assert_eq!(
        collider_debug_color_hex(false, false, false, true),
        pack_argb(0xFF, 0xFF, 0x00, 0x00)
    );
}

#[test]
fn tc_ir_5_4_u7_local_to_world_transform_of_contact_point() {
    let p = ContactPoint {
        local_a: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        local_b: Vec3::zero(),
        penetration: 0.0,
    };
    let xf = Transform {
        translation: Vec3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        },
        rotation: Quat::identity(),
    };
    let w = contact_point_world_a(&p, xf);
    assert!((w.x - 1.0).abs() < 1e-5 && (w.y - 10.0).abs() < 1e-5);
    let w2 = contact_point_world_b(&p, xf);
    assert!((w2.x - 0.0).abs() < 1e-5 && (w2.y - 10.0).abs() < 1e-5);
}

#[test]
fn tc_ir_5_4_n1_collider_dimension_zero_clamped() {
    let e = Entity(50);
    let mut world = World::new();
    let old = box_shape(1.0, 1.0, 1.0);
    world.insert_collider(
        e,
        Collider {
            shape: old.clone(),
            offset: Vec3::zero(),
            is_trigger: false,
        },
    );
    let bad = box_shape(0.0, 1.0, 1.0);
    let mut cmd = ColliderEditCommand {
        entity: e,
        old_shape: old,
        new_shape: bad,
    };
    cmd.execute(&mut world).expect("execute");
    match &world.collider(e).expect("c").shape {
        ColliderShape::Box { half_extents } => assert!((half_extents.x - 0.01).abs() < 1e-5),
        _ => panic!("box"),
    }
    assert!(world.diagnostics.iter().any(|d| d.contains("clamped")));
}

#[test]
fn tc_ir_5_4_n2_collider_dimension_nan_rejected() {
    let e = Entity(51);
    let mut world = World::new();
    let old = box_shape(1.0, 1.0, 1.0);
    world.insert_collider(
        e,
        Collider {
            shape: old.clone(),
            offset: Vec3::zero(),
            is_trigger: false,
        },
    );
    let bad = ColliderShape::Sphere { radius: f32::NAN };
    let mut cmd = ColliderEditCommand {
        entity: e,
        old_shape: old.clone(),
        new_shape: bad,
    };
    assert_eq!(cmd.execute(&mut world), Err(CommandError::InvalidInput));
    assert_eq!(world.collider(e).expect("c").shape, old);
}

#[test]
fn tc_ir_5_4_n3_degenerate_convex_hull_falls_back_to_aabb() {
    let mut world = World::new();
    let hull = ColliderShape::ConvexHull(ConvexHull {
        vertices: vec![
            Vec3::zero(),
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ],
    });
    assert_eq!(
        narrowphase_proxy_for_shape(&hull),
        NarrowphaseProxy::AabbFallback
    );
    log_narrowphase_proxy(&mut world, &hull);
    assert!(world
        .diagnostics
        .iter()
        .any(|d| d.contains("AABB fallback")));
}

#[test]
fn tc_ir_5_4_n4_layer_mask_all_zero_emits_warning() {
    let layers = CollisionLayers {
        membership: 0,
        mask: 0xFFFF,
    };
    assert!(collision_layers_need_warning(&layers));
}

#[test]
fn tc_ir_5_4_n5_contact_overflow_drops_excess_visualization() {
    let v: Vec<u32> = (0..1500).collect();
    let capped = cap_contact_debug_entries(&v, 1000);
    assert_eq!(capped.len(), 1000);
}

#[test]
fn tc_ir_5_4_n6_undo_empty_stack_is_no_op() {
    let mut world = World::new();
    let mut stack = UndoStack::new();
    assert!(stack.undo(&mut world).is_ok());
}

#[test]
fn tc_ir_5_4_n7_editor_selecting_sleeping_body_force_wakes() {
    let e = Entity(60);
    let mut world = World::new();
    world.insert_sleeping(e);
    world.set_sleep_timer_ticks(e, 99);
    world.editor_select_wake(e);
    assert!(!world.is_sleeping(e));
    assert_eq!(world.sleep_timer_ticks(e), 0);
}

#[test]
fn tc_ir_5_4_n8_compound_child_index_out_of_range_rejected() {
    let e = Entity(61);
    let mut world = World::new();
    let child = CompoundChild {
        shape: box_shape(0.1, 0.1, 0.1),
        offset: Vec3::zero(),
        rotation: Quat::identity(),
        layers: CollisionLayers::default(),
        material: PhysicsMaterialHandle(0),
    };
    world.insert_compound(
        e,
        CompoundCollider {
            children: vec![child.clone(), child.clone(), child.clone()],
        },
    );
    let mut cmd = CompoundChildEditCommand {
        entity: e,
        child_index: 99,
        old_child: CompoundChild {
            shape: box_shape(0.1, 0.1, 0.1),
            offset: Vec3::zero(),
            rotation: Quat::identity(),
            layers: CollisionLayers::default(),
            material: PhysicsMaterialHandle(0),
        },
        new_child: CompoundChild {
            shape: box_shape(0.2, 0.2, 0.2),
            offset: Vec3::zero(),
            rotation: Quat::identity(),
            layers: CollisionLayers::default(),
            material: PhysicsMaterialHandle(0),
        },
    };
    assert_eq!(cmd.execute(&mut world), Err(CommandError::InvalidIndex));
}
