//! Public façade around the crate-private [`crate::physics_bvh::PhysicsBvh`].

use crate::physics_bvh::{BroadphasePair, BvhHandle, LeafEntry, PhysicsBvh};
use crate::ray::{RayHit, RayQuery};

/// Opaque broadphase world owned by the physics subsystem.
#[derive(Debug, Default)]
pub struct PhysicsBroadphase {
    inner: PhysicsBvh,
}

impl PhysicsBroadphase {
    /// Creates an empty broadphase world.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: PhysicsBvh::default(),
        }
    }

    /// Inserts a leaf entry and returns its handle.
    pub fn insert_leaf(&mut self, entry: LeafEntry) -> BvhHandle {
        self.inner.insert_leaf(entry)
    }

    /// Removes a leaf, invalidating prior handles.
    pub fn remove_leaf(&mut self, handle: BvhHandle) -> Result<(), BroadphaseError> {
        self.inner.remove_leaf(handle).map_err(|err| match err {
            crate::physics_bvh::BvhError::StaleHandle => BroadphaseError::StaleHandle,
            crate::physics_bvh::BvhError::InvalidHandle => BroadphaseError::InvalidHandle,
        })
    }

    /// Refits a leaf when the handle is still valid.
    pub fn refit_leaf(
        &mut self,
        handle: BvhHandle,
        bounds: crate::aabb::Aabb,
    ) -> Result<(), BroadphaseError> {
        self.inner.refit(handle, bounds).map_err(|err| match err {
            crate::physics_bvh::BvhError::StaleHandle => BroadphaseError::StaleHandle,
            crate::physics_bvh::BvhError::InvalidHandle => BroadphaseError::InvalidHandle,
        })
    }

    /// Returns overlapping broadphase pairs filtered by collision layers.
    #[must_use]
    pub fn overlapping_pairs(&self) -> Vec<BroadphasePair> {
        self.inner.query_overlapping_pairs()
    }

    /// Performs a single raycast against all active leaves.
    #[must_use]
    pub fn raycast_first(&self, ray: &RayQuery) -> Option<RayHit> {
        self.inner.raycast_first(ray)
    }

    pub(crate) fn inner(&self) -> &PhysicsBvh {
        &self.inner
    }
}

/// Errors returned from [`PhysicsBroadphase`] mutation helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BroadphaseError {
    /// Handle no longer matches the stored generation.
    StaleHandle,
    /// Index was out of range for the current leaf table.
    InvalidHandle,
}
