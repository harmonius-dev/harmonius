//! Line-of-sight bridging between grid cells and physics ray casts.

use glam::Vec3;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

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

/// World-space axis-aligned bounds (`Aabb` in design diagrams).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldAabb {
    /// Minimum corner (inclusive).
    pub min: Vec3,
    /// Maximum corner (inclusive).
    pub max: Vec3,
}

impl WorldAabb {
    /// Builds bounds from inclusive corners.
    #[must_use]
    pub const fn from_min_max(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
}

/// Entities whose colliders overlap a [`WorldAabb`] query (inline capacity before spill).
pub type OverlapEntities = SmallVec<[Entity; 8]>;

/// Predicate applied to candidate hits (e.g. walls-only gameplay filter).
#[derive(Clone, Copy)]
pub struct CollisionFilterFn(fn(&RayHit) -> bool);

impl CollisionFilterFn {
    /// Filter that accepts hits that should block propagation.
    ///
    /// Stub: accepts every hit until collider metadata can identify structural walls.
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

    /// Returns colliders overlapping `bounds`, retaining entities accepted by `filter`.
    fn aabb_overlap(
        &self,
        bounds: WorldAabb,
        filter: &mut dyn FnMut(&Entity) -> bool,
    ) -> OverlapEntities;
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
    entries: FxHashMap<(crate::CellCoord, crate::CellCoord), bool>,
}

impl LosCache {
    /// Creates an empty cache.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: FxHashMap::default(),
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
    use std::sync::atomic::{AtomicBool, Ordering};

    use super::*;

    struct RefuseRayPhysics {
        ray_called: AtomicBool,
    }

    impl PhysicsQueries for RefuseRayPhysics {
        fn ray_cast(
            &self,
            _origin: Vec3,
            _direction_normalized: Vec3,
            _max_distance: f32,
            _filter: CollisionFilterFn,
        ) -> Option<RayHit> {
            self.ray_called.store(true, Ordering::Relaxed);
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
    fn tc_ir_3_10_u5_propagation_los_zero_distance() {
        let physics = RefuseRayPhysics {
            ray_called: AtomicBool::new(false),
        };
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert!(propagation_los_check(
            p,
            p,
            &physics,
            CollisionFilterFn::walls_only(),
        ));
        assert!(
            !physics.ray_called.load(Ordering::Relaxed),
            "zero-length segment must not invoke ray_cast",
        );
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
}
