//! ECS-facing data contracts from `docs/design/integration/editor-physics.md`.

use std::mem::size_of;

use crate::math::{Quat, Vec3};

/// Stable entity identifier for the headless harness.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Entity(pub u64);

/// Handle to a registered physics material asset.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PhysicsMaterialHandle(pub u32);

/// Membership and collision mask bitfields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CollisionLayers {
    /// Layer membership bits.
    pub membership: u32,
    /// Layer mask bits.
    pub mask: u32,
}

impl Default for CollisionLayers {
    fn default() -> Self {
        Self {
            membership: 1,
            mask: u32::MAX,
        }
    }
}

impl CollisionLayers {
    /// XOR-toggles one membership bit (TC-IR-5.4.5.1).
    pub fn toggle_membership_bit(&mut self, bit: u32) {
        self.membership ^= 1 << bit;
    }
}

/// Marker component: entity participates in rigid-body simulation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RigidBody;

/// Marker: rigid body is sleeping for broadphase/debug coloring.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Sleeping;

/// Accumulated sleep time for wake tests.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SleepTimer {
    /// Ticks spent sleeping.
    pub ticks: u32,
}

/// Convex hull vertex set (world or shape space depending on caller).
#[derive(Clone, Debug, PartialEq)]
pub struct ConvexHull {
    /// Hull vertices.
    pub vertices: Vec<Vec3>,
}

/// Triangle mesh vertex set.
#[derive(Clone, Debug, PartialEq)]
pub struct TriMesh {
    /// Mesh vertices.
    pub vertices: Vec<Vec3>,
}

/// Collider shape variants edited by gizmos.
#[derive(Clone, Debug, PartialEq)]
pub enum ColliderShape {
    /// Axis-aligned box half-extents.
    Box {
        /// Half-extents along each axis.
        half_extents: Vec3,
    },
    /// Sphere radius.
    Sphere {
        /// Radius.
        radius: f32,
    },
    /// Y-axis capsule: cylinder half-height + sphere radius.
    Capsule {
        /// Half-height of cylinder section (excluding caps).
        half_height: f32,
        /// Radius of capsule caps and cylinder.
        radius: f32,
    },
    /// Convex hull collider.
    ConvexHull(ConvexHull),
    /// Triangle mesh collider.
    TriMesh(TriMesh),
    /// Placeholder heightfield variant (not exercised by current tests).
    Heightfield,
}

impl ColliderShape {
    /// Returns heap-backed byte estimate for `size_bytes` accounting.
    #[must_use]
    pub fn heap_size(&self) -> usize {
        match self {
            ColliderShape::ConvexHull(h) => h.vertices.len() * size_of::<Vec3>(),
            ColliderShape::TriMesh(m) => m.vertices.len() * size_of::<Vec3>(),
            ColliderShape::Box { .. }
            | ColliderShape::Sphere { .. }
            | ColliderShape::Capsule { .. }
            | ColliderShape::Heightfield => 0,
        }
    }
}

/// Single primitive child inside a compound collider.
#[derive(Clone, Debug, PartialEq)]
pub struct CompoundChild {
    /// Local child shape.
    pub shape: ColliderShape,
    /// Local translation from compound origin.
    pub offset: Vec3,
    /// Local rotation.
    pub rotation: Quat,
    /// Per-child collision layers.
    pub layers: CollisionLayers,
    /// Per-child material.
    pub material: PhysicsMaterialHandle,
}

/// Compound collider aggregate.
#[derive(Clone, Debug, PartialEq)]
pub struct CompoundCollider {
    /// Ordered child primitives.
    pub children: Vec<CompoundChild>,
}

/// Single collider component (non-compound path).
#[derive(Clone, Debug, PartialEq)]
pub struct Collider {
    /// Primitive shape.
    pub shape: ColliderShape,
    /// Local offset from entity origin.
    pub offset: Vec3,
    /// Trigger volumes participate in overlap queries only.
    pub is_trigger: bool,
}

/// Single contact sample on a manifold.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContactPoint {
    /// Contact on entity A in A's local space.
    pub local_a: Vec3,
    /// Contact on entity B in B's local space.
    pub local_b: Vec3,
    /// Penetration depth along the manifold normal.
    pub penetration: f32,
}
