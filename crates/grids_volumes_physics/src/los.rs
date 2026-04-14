//! Line-of-sight bridging between grid cells and physics ray casts.

use glam::Vec3;

use crate::types::Entity;

/// Hit data returned when a physics ray intersects a collider in the private BVH.
#[derive(Clone, Debug, PartialEq)]
pub struct RayHit {
    /// Collider entity that was hit.
    pub entity: Entity,
    /// Distance along the ray from the origin to the hit point.
    pub distance: f32,
    /// World-space contact point.
    pub point: Vec3,
    /// World-space surface normal at the hit.
    pub normal: Vec3,
}

/// Predicate applied to candidate hits (e.g. walls-only gameplay filter).
#[derive(Clone, Copy)]
pub struct CollisionFilterFn(fn(&RayHit) -> bool);

impl CollisionFilterFn {
    /// Filter that accepts hits that should block propagation (stub: structural walls).
    #[must_use]
    pub fn walls_only() -> Self {
        Self(|_hit| true)
    }

    /// Returns whether the hit satisfies this filter.
    #[must_use]
    pub fn matches(self, hit: &RayHit) -> bool {
        (self.0)(hit)
    }
}

/// Narrow-phase queries against the physics-private BVH.
pub trait PhysicsQueries {
    /// Casts a ray; returns the closest blocking hit, if any, after `filter` accepts it.
    fn ray_cast(
        &self,
        origin: Vec3,
        direction_normalized: Vec3,
        max_distance: f32,
        filter: CollisionFilterFn,
    ) -> Option<RayHit>;
}

/// Returns `true` when propagation may spread (no blocking hit along the segment).
#[must_use]
pub fn propagation_los_check(
    from_world: Vec3,
    to_world: Vec3,
    physics: &impl PhysicsQueries,
    filter: CollisionFilterFn,
) -> bool {
    let dir = to_world - from_world;
    let dist = dir.length();
    if dist < f32::EPSILON {
        return true;
    }
    let direction = dir / dist;
    physics
        .ray_cast(from_world, direction, dist, filter)
        .is_none()
}

/// Per-tick memoization of LOS results between cell pairs.
#[derive(Debug, Default)]
pub struct LosCache {
    entries: std::collections::HashMap<(crate::CellCoord, crate::CellCoord), bool>,
}

impl LosCache {
    /// Creates an empty cache.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }

    /// Looks up a cached LOS clearance flag.
    #[must_use]
    pub fn get(&self, from: crate::CellCoord, to: crate::CellCoord) -> Option<bool> {
        self.entries.get(&(from, to)).copied()
    }

    /// Stores whether LOS from `from` to `to` is clear (`true` = no blocking geometry).
    pub fn insert(&mut self, from: crate::CellCoord, to: crate::CellCoord, clear: bool) {
        self.entries.insert((from, to), clear);
    }

    /// Clears all entries (e.g. after collider edits).
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
#[allow(missing_docs)]
mod tests {
    use super::*;

    struct PanicPhysics;

    impl PhysicsQueries for PanicPhysics {
        fn ray_cast(
            &self,
            _origin: Vec3,
            _direction_normalized: Vec3,
            _max_distance: f32,
            _filter: CollisionFilterFn,
        ) -> Option<RayHit> {
            panic!("ray_cast must not run for zero-length LOS segments");
        }
    }

    #[test]
    fn tc_ir_3_10_u5_propagation_los_zero_distance() {
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert!(propagation_los_check(
            p,
            p,
            &PanicPhysics,
            CollisionFilterFn::walls_only(),
        ));
    }

    #[test]
    fn tc_ir_3_10_u6_los_cache_hit() {
        let mut c = LosCache::new();
        let a = crate::CellCoord { x: 0, y: 0 };
        let b = crate::CellCoord { x: 1, y: 0 };
        c.insert(a, b, true);
        assert_eq!(c.get(a, b), Some(true));
    }

    #[test]
    fn tc_ir_3_10_u7_los_cache_miss() {
        let c = LosCache::new();
        let a = crate::CellCoord { x: 0, y: 0 };
        let b = crate::CellCoord { x: 1, y: 0 };
        assert_eq!(c.get(a, b), None);
    }

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
    }

    /// TC-IR-3.10.5.2 — propagation sees clear LOS with no colliders in the physics-private BVH.
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
}
