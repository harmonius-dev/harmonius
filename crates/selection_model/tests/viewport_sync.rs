use glam::{Affine3A, IVec2, Vec3};
use selection_model::{
    aggregate_affine_for_selection, marquee_select, nearest_subobject_along_ray, raycast_spheres,
    EditorWorldId, EntityRef, IntersectMode, Ray3, ScreenRect, SelectionState, SubObjectElement,
    SubObjectKind,
};

#[test]
fn test_two_viewport_sync_on_pick() {
    let mut shared = SelectionState::default();
    let world = EditorWorldId(1);
    assert!(shared.add_notify(world, EntityRef(10)).is_some());

    let viewport_a = shared.snapshot();
    let viewport_b = shared.snapshot();
    assert_eq!(viewport_a, viewport_b);
}

#[test]
fn test_four_viewport_sync_on_marquee() {
    let marquee = ScreenRect {
        min: IVec2::new(0, 0),
        max: IVec2::new(50, 50),
    };
    let hits = marquee_select(
        marquee,
        IntersectMode::Intersect,
        [
            (
                EntityRef(1),
                ScreenRect {
                    min: IVec2::new(5, 5),
                    max: IVec2::new(15, 15),
                },
            ),
            (
                EntityRef(2),
                ScreenRect {
                    min: IVec2::new(60, 60),
                    max: IVec2::new(70, 70),
                },
            ),
        ]
        .into_iter(),
    );

    let mut shared = SelectionState::default();
    for entity in hits {
        assert!(shared.add(entity));
    }

    let v1 = shared.snapshot();
    let v2 = shared.snapshot();
    let v3 = shared.snapshot();
    let v4 = shared.snapshot();
    assert_eq!(v1, v2);
    assert_eq!(v2, v3);
    assert_eq!(v3, v4);
}

#[test]
fn test_ray_picking_api_smoke() {
    let ray = Ray3 {
        dir: Vec3::new(0.0, 0.0, -1.0),
        origin: Vec3::new(0.0, 0.0, 3.0),
    };
    let hit = raycast_spheres(&ray, &[(EntityRef(1), Vec3::new(0.0, 0.0, 0.0), 0.25)])
        .expect("sphere hit");
    assert_eq!(hit.0, EntityRef(1));

    let element = SubObjectElement {
        entity: EntityRef(1),
        index: 0,
        kind: SubObjectKind::Face,
    };
    let tri = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];
    assert!(nearest_subobject_along_ray(&ray, &[(element, tri)]).is_some());
}

#[test]
fn test_gizmo_position_from_aggregate() {
    let mut sel = SelectionState::default();
    assert!(sel.add(EntityRef(1)));
    assert!(sel.add(EntityRef(2)));
    let gizmo = aggregate_affine_for_selection(&sel, |entity| {
        if entity == EntityRef(1) {
            Some(Affine3A::from_translation(Vec3::new(0.0, 0.0, 0.0)))
        } else if entity == EntityRef(2) {
            Some(Affine3A::from_translation(Vec3::new(4.0, 0.0, 0.0)))
        } else {
            None
        }
    })
    .expect("aggregate");
    assert_eq!(Vec3::from(gizmo.translation), Vec3::new(2.0, 0.0, 0.0));
}
