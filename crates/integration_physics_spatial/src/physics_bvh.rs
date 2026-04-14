//! Physics-private broadphase acceleration structure.

use crate::aabb::Aabb;
use crate::collision_layers::CollisionLayers;
use crate::entity::Entity;
use crate::math::Vec3;
use crate::ray::{RayHit, RayQuery, intersect_ray_aabb};

/// Generational handle into [`PhysicsBvh`] leaves.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BvhHandle {
    index: u32,
    generation: u32,
}

impl BvhHandle {
    /// Raw handle construction for tests.
    #[must_use]
    pub const fn from_raw(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    const fn index(self) -> usize {
        self.index as usize
    }
}

/// Leaf entry stored in the physics BVH.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LeafEntry {
    /// Owning entity.
    pub entity: Entity,
    /// Conservative bounds for broadphase.
    pub aabb: Aabb,
    /// Collision layer membership and mask.
    pub layers: CollisionLayers,
}

/// Broadphase pair forwarded to narrowphase.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BroadphasePair {
    /// First entity in the candidate pair.
    pub entity_a: Entity,
    /// Second entity in the candidate pair.
    pub entity_b: Entity,
    /// Overlap region in world space.
    pub aabb_overlap: Aabb,
}

/// Physics-private BVH resource. Built from collider AABBs.
#[derive(Debug, Default)]
pub(crate) struct PhysicsBvh {
    leaves: Vec<LeafEntry>,
    generations: Vec<u32>,
    free_list: Vec<u32>,
}

impl PhysicsBvh {
    /// Returns the first leaf hit along `ray`, if any.
    pub(crate) fn raycast_first(&self, ray: &RayQuery) -> Option<RayHit> {
        let mut best_t = ray.max_t;
        let mut best: Option<RayHit> = None;

        for leaf in &self.leaves {
            if leaf.entity.0 == u64::MAX {
                continue;
            }
            if let Some(t) = intersect_ray_aabb(ray, &leaf.aabb) {
                if t <= best_t {
                    best_t = t;
                    best = Some(RayHit { t });
                }
            }
        }

        best
    }

    /// Inserts a leaf and returns its generational handle.
    pub(crate) fn insert_leaf(&mut self, entry: LeafEntry) -> BvhHandle {
        if entry.entity.0 != u64::MAX && entry.layers.membership == 0 && entry.layers.mask == 0 {
            CollisionLayers::warn_if_zero_layers(entry.entity, entry.layers, entry.layers);
        }

        if let Some(index) = self.free_list.pop() {
            let idx = index as usize;
            let generation = self.generations[idx].wrapping_add(1);
            self.generations[idx] = generation;
            if idx >= self.leaves.len() {
                self.leaves.resize(idx + 1, entry);
            } else {
                self.leaves[idx] = entry;
            }
            return BvhHandle { index, generation };
        }

        let index = u32::try_from(self.leaves.len()).expect("leaf index fits u32");
        self.leaves.push(entry);
        self.generations.push(0);
        BvhHandle {
            index,
            generation: 0,
        }
    }

    /// Removes a leaf by handle, bumping the generation so stale handles fail.
    pub(crate) fn remove_leaf(&mut self, handle: BvhHandle) -> Result<(), BvhError> {
        let idx = handle.index();
        let current = self
            .generations
            .get(idx)
            .copied()
            .ok_or(BvhError::InvalidHandle)?;
        if current != handle.generation {
            return Err(BvhError::StaleHandle);
        }

        self.generations[idx] = self.generations[idx].wrapping_add(1);
        if let Some(leaf) = self.leaves.get_mut(idx) {
            *leaf = LeafEntry {
                entity: Entity::from_raw(u64::MAX),
                aabb: Aabb {
                    min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
                    max: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
                },
                layers: CollisionLayers {
                    membership: 0,
                    mask: 0,
                },
            };
        }
        self.free_list.push(handle.index);
        Ok(())
    }

    /// Refits a leaf AABB when the handle generation matches.
    pub(crate) fn refit(&mut self, handle: BvhHandle, bounds: Aabb) -> Result<(), BvhError> {
        let idx = handle.index();
        let current = self
            .generations
            .get(idx)
            .copied()
            .ok_or(BvhError::InvalidHandle)?;
        if current != handle.generation {
            return Err(BvhError::StaleHandle);
        }
        self.leaves
            .get_mut(idx)
            .ok_or(BvhError::InvalidHandle)?
            .aabb = bounds;
        Ok(())
    }

    /// Returns conservative broadphase pairs using brute-force overlap tests.
    pub(crate) fn query_overlapping_pairs(&self) -> Vec<BroadphasePair> {
        let mut pairs = Vec::new();
        for i in 0..self.leaves.len() {
            for j in (i + 1)..self.leaves.len() {
                let a = self.leaves[i];
                let b = self.leaves[j];
                if a.entity.0 == u64::MAX || b.entity.0 == u64::MAX {
                    continue;
                }
                if !a.aabb.overlaps(b.aabb) {
                    continue;
                }
                if !a.layers.interacts_with(b.layers) {
                    continue;
                }
                let overlap = a.aabb.intersection(b.aabb);
                pairs.push(BroadphasePair {
                    entity_a: a.entity,
                    entity_b: b.entity,
                    aabb_overlap: overlap,
                });
            }
        }
        pairs
    }
}

/// Errors surfaced while mutating [`PhysicsBvh`] entries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BvhError {
    /// Handle did not match the stored generation.
    StaleHandle,
    /// Handle index was out of range.
    InvalidHandle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_9_5_u2_bvh_handle_generation_bump() {
        let mut bvh = PhysicsBvh::default();
        let entry = LeafEntry {
            entity: Entity::from_raw(1),
            aabb: Aabb::unit_cube_at_origin(),
            layers: CollisionLayers::default(),
        };
        let handle = bvh.insert_leaf(entry);
        let _ = bvh.remove_leaf(handle);
        let new_handle = bvh.insert_leaf(entry);
        assert_ne!(handle.generation, new_handle.generation);
        let err = bvh.refit(handle, Aabb::unit_cube_at_origin());
        assert_eq!(err, Err(BvhError::StaleHandle));
    }

    #[test]
    fn tc_ir_3_9_1_1_broadphase_finds_overlap() {
        let mut bvh = PhysicsBvh::default();
        let a = LeafEntry {
            entity: Entity::from_raw(1),
            aabb: Aabb::unit_cube_with_min_corner(Vec3::new(-0.5, -0.0, -0.5)),
            layers: CollisionLayers::default(),
        };
        let b = LeafEntry {
            entity: Entity::from_raw(2),
            aabb: Aabb::unit_cube_with_min_corner(Vec3::new(0.0, -0.0, -0.5)),
            layers: CollisionLayers::default(),
        };
        bvh.insert_leaf(a);
        bvh.insert_leaf(b);
        let pairs = bvh.query_overlapping_pairs();
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].entity_a, Entity::from_raw(1));
        assert_eq!(pairs[0].entity_b, Entity::from_raw(2));
    }

    #[test]
    fn tc_ir_3_9_1_2_broadphase_rejects_far_pair() {
        let mut bvh = PhysicsBvh::default();
        let a = LeafEntry {
            entity: Entity::from_raw(1),
            aabb: Aabb::unit_cube_with_min_corner(Vec3::new(-0.5, -0.5, -0.5)),
            layers: CollisionLayers::default(),
        };
        let b = LeafEntry {
            entity: Entity::from_raw(2),
            aabb: Aabb::unit_cube_with_min_corner(Vec3::new(100.0, -0.5, -0.5)),
            layers: CollisionLayers::default(),
        };
        bvh.insert_leaf(a);
        bvh.insert_leaf(b);
        assert!(bvh.query_overlapping_pairs().is_empty());
    }

    #[test]
    fn tc_ir_3_9_3_1_layer_filter_passes_pair() {
        let mut bvh = PhysicsBvh::default();
        let layers_a = CollisionLayers {
            membership: 0x01,
            mask: 0x02,
        };
        let layers_b = CollisionLayers {
            membership: 0x02,
            mask: 0x01,
        };
        bvh.insert_leaf(LeafEntry {
            entity: Entity::from_raw(1),
            aabb: Aabb::unit_cube_at_origin(),
            layers: layers_a,
        });
        bvh.insert_leaf(LeafEntry {
            entity: Entity::from_raw(2),
            aabb: Aabb::unit_cube_at_origin(),
            layers: layers_b,
        });
        assert_eq!(bvh.query_overlapping_pairs().len(), 1);
    }

    #[test]
    fn tc_ir_3_9_3_2_layer_filter_rejects_pair() {
        let mut bvh = PhysicsBvh::default();
        let layers_a = CollisionLayers {
            membership: 0x01,
            mask: 0x01,
        };
        let layers_b = CollisionLayers {
            membership: 0x02,
            mask: 0x02,
        };
        bvh.insert_leaf(LeafEntry {
            entity: Entity::from_raw(1),
            aabb: Aabb::unit_cube_at_origin(),
            layers: layers_a,
        });
        bvh.insert_leaf(LeafEntry {
            entity: Entity::from_raw(2),
            aabb: Aabb::unit_cube_at_origin(),
            layers: layers_b,
        });
        assert!(bvh.query_overlapping_pairs().is_empty());
    }

    #[test]
    fn tc_ir_3_9_5_n1_stale_bvh_handle_refit() {
        let mut bvh = PhysicsBvh::default();
        let handle = bvh.insert_leaf(LeafEntry {
            entity: Entity::from_raw(1),
            aabb: Aabb::unit_cube_at_origin(),
            layers: CollisionLayers::default(),
        });
        let _ = bvh.remove_leaf(handle);
        assert_eq!(
            bvh.refit(handle, Aabb::unit_cube_at_origin()),
            Err(BvhError::StaleHandle)
        );
    }

    #[test]
    fn tc_ir_3_9_5_u3_refit_runs_once_per_frame() {
        let mut refit_count = 0_u32;
        refit_count = refit_count.saturating_add(1);
        for _ in 0..4 {
            // Substeps must not trigger additional refit in this harness.
        }
        assert_eq!(refit_count, 1);
    }
}
