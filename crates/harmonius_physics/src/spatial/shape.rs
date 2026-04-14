//! Collider shapes for deterministic spatial queries.

use glam::{Quat, Vec3};

/// Supported primitive and mesh collider shapes.
#[derive(Clone, Debug, PartialEq)]
pub enum ColliderShape {
    /// Axis-aligned box in local space; half extents along each axis.
    Box {
        half_extents: Vec3,
    },
    /// Convex hull defined by points in local space.
    ConvexHull {
        points: Vec<Vec3>,
    },
    /// Heightfield samples on an XZ grid; heights are world Y offsets at grid points.
    Heightfield {
        cell: f32,
        depth: usize,
        heights: Vec<f32>,
        origin: Vec3,
        width: usize,
    },
    /// Triangle soup in local space.
    TriangleMesh {
        triangles: Vec<[Vec3; 3]>,
    },
    /// Capsule aligned with local +Y: segment from -Y to +Y scaled by `half_height`, expanded
    /// by `radius`.
    Capsule {
        half_height: f32,
        radius: f32,
    },
    /// Sphere in local space.
    Sphere {
        radius: f32,
    },
}

impl ColliderShape {
    /// Unit cube convex hull (8 vertices) centered at origin, half size 1.
    pub fn unit_cube_hull() -> Self {
        let p = 1.0_f32;
        Self::ConvexHull {
            points: vec![
                Vec3::new(-p, -p, -p),
                Vec3::new(p, -p, -p),
                Vec3::new(-p, p, -p),
                Vec3::new(p, p, -p),
                Vec3::new(-p, -p, p),
                Vec3::new(p, -p, p),
                Vec3::new(-p, p, p),
                Vec3::new(p, p, p),
            ],
        }
    }
}

/// World transform for a collider (position + rotation).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColliderTransform {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Default for ColliderTransform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        }
    }
}

impl ColliderTransform {
    /// Local point to world space.
    pub fn to_world(&self, local: Vec3) -> Vec3 {
        self.rotation * local + self.position
    }

    /// World vector to local space (no translation).
    pub fn world_to_local_dir(&self, world: Vec3) -> Vec3 {
        self.rotation.inverse() * world
    }

    /// World point to local space.
    pub fn world_to_local(&self, world: Vec3) -> Vec3 {
        self.rotation.inverse() * (world - self.position)
    }
}
