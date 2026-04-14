//! Batch spatial queries (sequential deterministic execution).

use glam::{Quat, Vec3};

use super::filter::QueryFilter;
use super::scene::{BvhScene, ClosestPointResult, OverlapResult, RayHit, ShapeHit};

/// Descriptor for one query in a batch.
#[derive(Clone)]
pub enum SpatialQueryDescriptor {
    Overlap {
        filter: QueryFilter,
        position: Vec3,
        rotation: Quat,
        shape: super::shape::ColliderShape,
    },
    Ray {
        direction: Vec3,
        filter: QueryFilter,
        origin: Vec3,
    },
    Shape {
        direction: Vec3,
        filter: QueryFilter,
        origin: Vec3,
        shape: super::shape::ColliderShape,
    },
}

/// Result row for a batch entry.
#[derive(Clone, Debug, PartialEq)]
pub enum SpatialQueryResult {
    Closest(Option<ClosestPointResult>),
    Overlap(Vec<OverlapResult>),
    Ray(Option<RayHit>),
    Shape(Option<ShapeHit>),
}

/// Runs batch queries in declaration order (deterministic).
pub fn run_batch(scene: &BvhScene, descriptors: &[SpatialQueryDescriptor]) -> Vec<SpatialQueryResult> {
    let mut out = Vec::with_capacity(descriptors.len());
    let mut buf = Vec::new();
    for d in descriptors {
        match d {
            SpatialQueryDescriptor::Ray {
                origin,
                direction,
                filter,
            } => out.push(SpatialQueryResult::Ray(scene.ray_cast(*origin, *direction, filter))),
            SpatialQueryDescriptor::Shape {
                shape,
                origin,
                direction,
                filter,
            } => out.push(SpatialQueryResult::Shape(scene.shape_cast(
                shape,
                *origin,
                *direction,
                filter,
            ))),
            SpatialQueryDescriptor::Overlap {
                shape,
                position,
                rotation,
                filter,
            } => {
                scene.overlap(shape, *position, *rotation, filter, &mut buf);
                out.push(SpatialQueryResult::Overlap(buf.clone()));
            }
        }
    }
    out
}

/// Runs the same rays individually (for equivalence checks).
pub fn run_rays_individually(
    scene: &BvhScene,
    origins: &[Vec3],
    dirs: &[Vec3],
    filter: &QueryFilter,
) -> Vec<Option<RayHit>> {
    origins
        .iter()
        .zip(dirs.iter())
        .map(|(o, d)| scene.ray_cast(*o, *d, filter))
        .collect()
}
