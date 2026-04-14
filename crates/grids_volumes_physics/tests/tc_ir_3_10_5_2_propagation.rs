//! TC-IR-3.10.5.2 — propagation LOS in unobstructed space (crate integration test target).

use glam::Vec3;
use grids_volumes_physics::{
    propagation_los_check, CollisionFilterFn, Entity, OverlapEntities, PhysicsQueries, RayHit,
    WorldAabb,
};

struct OpenSpacePhysics;

impl PhysicsQueries for OpenSpacePhysics {
    fn ray_cast(
        &self,
        _origin: Vec3,
        _direction_normalized: Vec3,
        _max_distance: f32,
        _filter: CollisionFilterFn,
    ) -> Option<RayHit> {
        None
    }

    fn aabb_overlap(
        &self,
        _bounds: WorldAabb,
        _filter: &mut dyn FnMut(&Entity) -> bool,
    ) -> OverlapEntities {
        OverlapEntities::new()
    }
}

#[test]
fn tc_ir_3_10_5_2_propagation_passes_open_space() {
    let physics = OpenSpacePhysics;
    assert!(propagation_los_check(
        Vec3::ZERO,
        Vec3::new(10.0, 0.0, 0.0),
        &physics,
        CollisionFilterFn::walls_only(),
    ));
}
