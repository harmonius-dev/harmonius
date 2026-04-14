use glam::{Quat, Vec3};
use std::sync::Arc;

use crate::entity::Entity;
use crate::spatial::{
    run_batch, run_rays_individually, BvhScene, ColliderEntry, ColliderShape, ColliderTransform,
    QueryFilter, SpatialQueryDescriptor, SpatialQueryResult,
};

const LAYER_DEBRIS: u64 = 1 << 0;
const LAYER_WORLD: u64 = 1 << 1;
const TAG_ENEMY: u64 = 1 << 10;
const TAG_STATIC: u64 = 1 << 11;

fn scene_sphere(center: Vec3, radius: f32, entity: u32) -> BvhScene {
    let mut s = BvhScene::new();
    s.push(ColliderEntry {
        entity: Entity(entity),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Sphere { radius },
        tags: 0,
        xf: ColliderTransform {
            position: center,
            rotation: Quat::IDENTITY,
        },
    });
    s
}

#[test]
fn tc_4_4_1_1_ray_cast_sphere_hit() {
    let scene = scene_sphere(Vec3::new(5.0, 0.0, 0.0), 1.0, 1);
    let filter = QueryFilter::new();
    let hit = scene
        .ray_cast(Vec3::ZERO, Vec3::X, &filter)
        .expect("hit");
    assert!((hit.position - Vec3::new(4.0, 0.0, 0.0)).length() < 1e-4);
    assert!(hit.normal.dot(Vec3::NEG_X).acos() < 0.001);
}

#[test]
fn tc_4_4_1_2_ray_cast_normal_accuracy() {
    let scene = scene_sphere(Vec3::new(0.0, 1.0, 0.0), 1.0, 1);
    let filter = QueryFilter::new();
    let origin = Vec3::new(2.0, 1.0, 0.0);
    let dir = Vec3::new(-1.0, 0.0, 0.0);
    let hit = scene.ray_cast(origin, dir, &filter).expect("hit");
    let analytical = Vec3::X;
    assert!(hit.normal.dot(analytical).acos() < 0.001);
}

#[test]
fn tc_4_4_1_3_ray_cast_all_shapes() {
    let mut scene = BvhScene::new();
    scene.push(ColliderEntry {
        entity: Entity(1),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Box {
            half_extents: Vec3::ONE,
        },
        tags: 0,
        xf: ColliderTransform::default(),
    });
    scene.push(ColliderEntry {
        entity: Entity(2),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Sphere { radius: 1.0 },
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(5.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
        },
    });
    scene.push(ColliderEntry {
        entity: Entity(3),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Capsule {
            half_height: 1.0,
            radius: 0.5,
        },
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(0.0, 2.0, 5.0),
            rotation: Quat::IDENTITY,
        },
    });
    scene.push(ColliderEntry {
        entity: Entity(4),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::unit_cube_hull(),
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(-4.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
        },
    });
    let tri = ColliderShape::TriangleMesh {
        triangles: vec![[
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(2.0, 0.0, -2.0),
            Vec3::new(0.0, 0.0, 0.0),
        ]],
    };
    scene.push(ColliderEntry {
        entity: Entity(5),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: tri,
        tags: 0,
        xf: ColliderTransform::default(),
    });
    let hf = ColliderShape::Heightfield {
        cell: 1.0,
        depth: 2,
        heights: vec![0.0, 0.0, 0.0, 0.0],
        origin: Vec3::new(-5.0, 0.0, -5.0),
        width: 2,
    };
    scene.push(ColliderEntry {
        entity: Entity(6),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: hf,
        tags: 0,
        xf: ColliderTransform::default(),
    });
    let filter = QueryFilter::new();
    assert!(scene
        .ray_cast(Vec3::new(0.5, 0.5, 3.0), Vec3::NEG_Z, &filter)
        .is_some());
    assert!(scene
        .ray_cast(Vec3::ZERO, Vec3::X, &filter)
        .is_some());
    assert!(scene
        .ray_cast(Vec3::new(0.0, 2.0, 0.0), Vec3::Z, &filter)
        .is_some());
    assert!(scene
        .ray_cast(Vec3::new(-6.0, 0.5, 0.0), Vec3::X, &filter)
        .is_some());
    assert!(scene
        .ray_cast(Vec3::new(0.5, 2.0, -1.0), Vec3::NEG_Y, &filter)
        .is_some());
    assert!(scene
        .ray_cast(Vec3::new(-4.5, 0.5, 0.0), Vec3::X, &filter)
        .is_some());
}

#[test]
fn tc_4_4_1_4_ray_cast_miss() {
    let scene = BvhScene::new();
    let filter = QueryFilter::new();
    assert!(scene
        .ray_cast(Vec3::ZERO, Vec3::Y, &filter)
        .is_none());
}

#[test]
fn tc_4_4_2_1_shape_cast_ground_detection() {
    let mut scene = BvhScene::new();
    scene.push(ColliderEntry {
        entity: Entity(1),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Box {
            half_extents: Vec3::new(50.0, 0.2, 50.0),
        },
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
        },
    });
    let filter = QueryFilter::new().with_max_distance(100.0);
    let hit = scene
        .shape_cast(
            &ColliderShape::Capsule {
                half_height: 1.0,
                radius: 0.5,
            },
            Vec3::new(0.0, 5.0, 0.0),
            Vec3::NEG_Y,
            &filter,
        )
        .expect("ground");
    assert!((hit.contact_point.y - 0.5).abs() < 0.05);
}

#[test]
fn tc_4_4_2_2_shape_cast_no_hit() {
    let scene = BvhScene::new();
    let filter = QueryFilter::new().with_max_distance(100.0);
    assert!(scene
        .shape_cast(
            &ColliderShape::Capsule {
                half_height: 1.0,
                radius: 0.5,
            },
            Vec3::new(0.0, 5.0, 0.0),
            Vec3::X,
            &filter,
        )
        .is_none());
}

#[test]
fn tc_4_4_2_3_shape_cast_contact_accuracy() {
    let mut scene = BvhScene::new();
    scene.push(ColliderEntry {
        entity: Entity(1),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Box {
            half_extents: Vec3::new(1.0, 1.0, 1.0),
        },
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(6.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
        },
    });
    let filter = QueryFilter::new().with_max_distance(50.0);
    let hit = scene
        .shape_cast(
            &ColliderShape::Sphere { radius: 1.0 },
            Vec3::ZERO,
            Vec3::X,
            &filter,
        )
        .expect("hit");
    let expected = Vec3::new(5.0, 0.0, 0.0);
    assert!((hit.contact_point - expected).length() < 1e-3);
}

#[test]
fn tc_4_4_3_1_overlap_sphere_aoe() {
    let mut scene = BvhScene::new();
    let mut expected_inside = 0usize;
    for i in 0..100 {
        let pos = if i < 30 {
            let r = 2.0 + i as f32 * 0.2;
            let ang = i as f32 * 2.39996;
            expected_inside += 1;
            Vec3::new(r * ang.cos(), 0.0, r * ang.sin())
        } else {
            let r = 12.0 + (i - 30) as f32 * 0.05;
            let ang = i as f32 * 1.7;
            Vec3::new(r * ang.cos(), 0.0, r * ang.sin())
        };
        scene.push(ColliderEntry {
            entity: Entity(i + 1),
            health: 1.0,
            layers: LAYER_WORLD,
            shape: ColliderShape::Sphere { radius: 0.2 },
            tags: 0,
            xf: ColliderTransform {
                position: pos,
                rotation: Quat::IDENTITY,
            },
        });
    }
    let mut out = Vec::new();
    let filter = QueryFilter::new();
    scene.overlap(
        &ColliderShape::Sphere { radius: 10.0 },
        Vec3::ZERO,
        Quat::IDENTITY,
        &filter,
        &mut out,
    );
    assert_eq!(out.len(), expected_inside);
    assert_eq!(expected_inside, 30);
}

#[test]
fn tc_4_4_3_2_overlap_all_shapes() {
    let mut scene = BvhScene::new();
    scene.push(ColliderEntry {
        entity: Entity(1),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Sphere { radius: 0.5 },
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(0.5, 0.0, 0.0),
            rotation: Quat::IDENTITY,
        },
    });
    let mut out = Vec::new();
    let filter = QueryFilter::new();
    scene.overlap(
        &ColliderShape::Box {
            half_extents: Vec3::ONE,
        },
        Vec3::ZERO,
        Quat::IDENTITY,
        &filter,
        &mut out,
    );
    assert_eq!(out.len(), 1);
    out.clear();
    scene.overlap(
        &ColliderShape::Capsule {
            half_height: 0.5,
            radius: 0.5,
        },
        Vec3::ZERO,
        Quat::IDENTITY,
        &filter,
        &mut out,
    );
    assert!(!out.is_empty());
    out.clear();
    scene.push(ColliderEntry {
        entity: Entity(2),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::unit_cube_hull(),
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(0.1, 0.0, 0.0),
            rotation: Quat::IDENTITY,
        },
    });
    scene.overlap(
        &ColliderShape::unit_cube_hull(),
        Vec3::new(0.05, 0.0, 0.0),
        Quat::IDENTITY,
        &filter,
        &mut out,
    );
    assert!(!out.is_empty());
}

#[test]
fn tc_4_4_4_1_closest_point_accuracy() {
    let filter = QueryFilter::new();
    let mut sphere_scene = BvhScene::new();
    sphere_scene.push(ColliderEntry {
        entity: Entity(1),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Sphere { radius: 1.0 },
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(5.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
        },
    });
    let c1 = sphere_scene
        .closest_point(Vec3::new(3.0, 0.0, 0.0), &filter)
        .expect("sphere");
    assert!((c1.closest_point - Vec3::new(4.0, 0.0, 0.0)).length() < 1e-3);
    assert!((c1.signed_distance - 1.0).abs() < 1e-3);
    let mut box_scene = BvhScene::new();
    box_scene.push(ColliderEntry {
        entity: Entity(2),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Box {
            half_extents: Vec3::new(1.0, 1.0, 1.0),
        },
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(3.0, 1.0, 1.0),
            rotation: Quat::IDENTITY,
        },
    });
    let c2 = box_scene
        .closest_point(Vec3::new(1.0, 1.0, 0.0), &filter)
        .expect("box");
    assert!((c2.closest_point - Vec3::new(2.0, 1.0, 0.0)).length() < 1e-2);
    let mut cap_scene = BvhScene::new();
    cap_scene.push(ColliderEntry {
        entity: Entity(3),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Capsule {
            half_height: 1.0,
            radius: 0.5,
        },
        tags: 0,
        xf: ColliderTransform::default(),
    });
    assert!(cap_scene
        .closest_point(Vec3::new(0.0, 5.0, 0.0), &filter)
        .is_some());
}

#[test]
fn tc_4_4_4_2_closest_point_signed_distance() {
    let mut scene = BvhScene::new();
    scene.push(ColliderEntry {
        entity: Entity(1),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Sphere { radius: 2.0 },
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(5.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
        },
    });
    let filter = QueryFilter::new();
    let inside = scene
        .closest_point(Vec3::new(5.0, 0.0, 0.0), &filter)
        .expect("inside");
    assert!((inside.signed_distance + 2.0).abs() < 1e-2);
    let outside = scene
        .closest_point(Vec3::new(8.0, 0.0, 0.0), &filter)
        .expect("outside");
    assert!((outside.signed_distance - 1.0).abs() < 1e-2);
}

#[test]
fn tc_4_4_5_1_batch_correctness() {
    let scene = scene_sphere(Vec3::new(5.0, 0.0, 0.0), 1.0, 1);
    let mut rays = Vec::new();
    let mut origins = Vec::new();
    let mut dirs = Vec::new();
    for i in 0..100 {
        let y = (i as f32) * 0.001;
        origins.push(Vec3::new(0.0, y, 0.0));
        dirs.push(Vec3::X);
        rays.push(SpatialQueryDescriptor::Ray {
            origin: origins[i],
            direction: dirs[i],
            filter: QueryFilter::new(),
        });
    }
    let batch = run_batch(&scene, &rays);
    let singles = run_rays_individually(&scene, &origins, &dirs, &QueryFilter::new());
    for (b, s) in batch.into_iter().zip(singles.into_iter()) {
        match b {
            SpatialQueryResult::Ray(rb) => assert_eq!(rb.map(|h| h.distance), s.map(|h| h.distance)),
            _ => panic!("unexpected batch row"),
        }
    }
}

#[test]
fn tc_4_4_6_1_filter_layers() {
    let mut scene = BvhScene::new();
    for i in 0..50 {
        scene.push(ColliderEntry {
            entity: Entity(i + 1),
            health: 1.0,
            layers: LAYER_DEBRIS,
            shape: ColliderShape::Sphere { radius: 0.2 },
            tags: 0,
            xf: ColliderTransform {
                position: Vec3::new(i as f32 * 0.1, 0.0, 0.0),
                rotation: Quat::IDENTITY,
            },
        });
    }
    scene.push(ColliderEntry {
        entity: Entity(1000),
        health: 1.0,
        layers: LAYER_WORLD,
        shape: ColliderShape::Sphere { radius: 2.0 },
        tags: 0,
        xf: ColliderTransform {
            position: Vec3::new(5.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
        },
    });
    let filter = QueryFilter::new().with_layers(LAYER_WORLD);
    let hit = scene
        .ray_cast(Vec3::ZERO, Vec3::X, &filter)
        .expect("world only");
    assert_eq!(hit.layers, LAYER_WORLD);
}

#[test]
fn tc_4_4_6_2_filter_with_without() {
    let mut scene = BvhScene::new();
    let mut expected_non_static = 0usize;
    for i in 0..100 {
        let enemy = i < 20;
        let pos = Vec3::new(i as f32 * 0.05, 0.0, 0.0);
        let static_tag = i % 3 == 0;
        let tags = (if enemy { TAG_ENEMY } else { 0 }) | (if static_tag { TAG_STATIC } else { 0 });
        let query = Vec3::new(0.5, 0.0, 0.0);
        if (pos - query).length() <= 10.2 && !static_tag {
            expected_non_static += 1;
        }
        scene.push(ColliderEntry {
            entity: Entity(i + 1),
            health: 1.0,
            layers: LAYER_WORLD,
            shape: ColliderShape::Sphere { radius: 0.2 },
            tags,
            xf: ColliderTransform {
                position: pos,
                rotation: Quat::IDENTITY,
            },
        });
    }
    let mut out = Vec::new();
    let filter = QueryFilter::new().with_tags(TAG_ENEMY);
    scene.overlap(
        &ColliderShape::Sphere { radius: 10.0 },
        Vec3::new(0.5, 0.0, 0.0),
        Quat::IDENTITY,
        &filter,
        &mut out,
    );
    assert_eq!(out.len(), 20);
    out.clear();
    let filter2 = QueryFilter::new().without_tags(TAG_STATIC);
    scene.overlap(
        &ColliderShape::Sphere { radius: 10.0 },
        Vec3::new(0.5, 0.0, 0.0),
        Quat::IDENTITY,
        &filter2,
        &mut out,
    );
    assert_eq!(out.len(), expected_non_static);
}

#[test]
fn tc_4_4_6_3_filter_predicate() {
    let mut scene = BvhScene::new();
    for i in 0..100 {
        scene.push(ColliderEntry {
            entity: Entity(i + 1),
            health: if i < 60 { 1.0 } else { 0.0 },
            layers: LAYER_WORLD,
            shape: ColliderShape::Sphere { radius: 0.2 },
            tags: 0,
            xf: ColliderTransform {
                position: Vec3::new(i as f32 * 0.05, 0.0, 0.0),
                rotation: Quat::IDENTITY,
            },
        });
    }
    let mut out = Vec::new();
    let filter = QueryFilter::new().with_predicate(Arc::new(|er: &crate::entity::EntityRef| {
        er.health > 0.0
    }));
    scene.overlap(
        &ColliderShape::Sphere { radius: 10.0 },
        Vec3::new(1.0, 0.0, 0.0),
        Quat::IDENTITY,
        &filter,
        &mut out,
    );
    assert_eq!(out.len(), 60);
}

#[test]
fn tc_4_4_6_4_filter_combinations() {
    let mut scene = BvhScene::new();
    for i in 0..40 {
        scene.push(ColliderEntry {
            entity: Entity(i + 1),
            health: 1.0,
            layers: LAYER_WORLD,
            shape: ColliderShape::Sphere { radius: 0.2 },
            tags: TAG_ENEMY,
            xf: ColliderTransform {
                position: Vec3::new(i as f32 * 0.1, 0.0, 0.0),
                rotation: Quat::IDENTITY,
            },
        });
    }
    let mut out = Vec::new();
    let filter = QueryFilter::new()
        .with_layers(LAYER_WORLD)
        .with_tags(TAG_ENEMY)
        .with_predicate(Arc::new(|er: &crate::entity::EntityRef| er.health > 0.0));
    scene.overlap(
        &ColliderShape::Sphere { radius: 5.0 },
        Vec3::new(1.0, 0.0, 0.0),
        Quat::IDENTITY,
        &filter,
        &mut out,
    );
    assert_eq!(out.len(), 40);
}
