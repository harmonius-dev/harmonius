//! Immutable snapshot value types shared between networking and physics.

use glam::{Quat, Vec3};
use rkyv_derive::{Archive, Deserialize, Serialize};

use crate::entity::Entity;

/// Handle to a convex mesh asset used by [`ColliderShape::ConvexHull`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct ConvexMeshHandle(pub u32);

/// Handle to a triangle mesh asset used by [`ColliderShape::TriMesh`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct TriMeshHandle(pub u32);

/// Handle to a heightfield asset used by [`ColliderShape::Heightfield`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct HeightfieldHandle(pub u32);

/// Compact reference into a per-tick contact pool owned by the physics worker.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct ContactRange {
    start: u32,
    len: u16,
}

impl ContactRange {
    /// Constructs a contact range.
    #[must_use]
    pub const fn new(start: u32, len: u16) -> Self {
        Self { start, len }
    }

    /// Start index into the contact pool.
    #[must_use]
    pub const fn start(self) -> u32 {
        self.start
    }

    /// Number of manifold entries referenced by this range.
    #[must_use]
    pub const fn len(self) -> u16 {
        self.len
    }

    /// Returns `true` when this range references no contacts.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.len == 0
    }
}

/// Collider shape variants for hitbox rewind and replication.
#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub enum ColliderShape {
    /// Sphere collider.
    Sphere {
        /// Radius in world units.
        radius: f32,
    },
    /// Axis-aligned box collider.
    Box {
        /// Half-extents along each axis.
        half_extents: Vec3,
    },
    /// Capsule collider.
    Capsule {
        /// Half height of the capsule cylinder section.
        half_height: f32,
        /// Radius of the capsule hemispheres and cylinder.
        radius: f32,
    },
    /// Convex hull collider referencing mesh data.
    ConvexHull {
        /// Asset handle for convex mesh vertices.
        vertex_handle: ConvexMeshHandle,
    },
    /// Triangle mesh collider.
    TriMesh {
        /// Asset handle for triangle indices and vertices.
        mesh_handle: TriMeshHandle,
    },
    /// Heightfield collider.
    Heightfield {
        /// Asset handle for sampled height data.
        field_handle: HeightfieldHandle,
    },
}

/// Physics state captured per entity per tick for rollback support.
#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct PhysicsSnapshot {
    position: Vec3,
    rotation: Quat,
    linear_velocity: Vec3,
    angular_velocity: Vec3,
    contacts: ContactRange,
    sleeping: bool,
}

impl PhysicsSnapshot {
    /// Captures a new immutable physics snapshot.
    #[must_use]
    pub fn new(
        position: Vec3,
        rotation: Quat,
        linear_velocity: Vec3,
        angular_velocity: Vec3,
        contacts: ContactRange,
        sleeping: bool,
    ) -> Self {
        Self {
            position,
            rotation,
            linear_velocity,
            angular_velocity,
            contacts,
            sleeping,
        }
    }

    /// World-space position.
    #[must_use]
    pub fn position(self) -> Vec3 {
        self.position
    }

    /// World-space orientation.
    #[must_use]
    pub fn rotation(self) -> Quat {
        self.rotation
    }

    /// Linear velocity in world space.
    #[must_use]
    pub fn linear_velocity(self) -> Vec3 {
        self.linear_velocity
    }

    /// Angular velocity in world space.
    #[must_use]
    pub fn angular_velocity(self) -> Vec3 {
        self.angular_velocity
    }

    /// Compact contact manifold reference for this tick.
    #[must_use]
    pub fn contacts(self) -> ContactRange {
        self.contacts
    }

    /// Whether the body is sleeping in the solver.
    #[must_use]
    pub fn sleeping(self) -> bool {
        self.sleeping
    }
}

/// Hitbox snapshot for lag compensation rewind.
#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct HitboxSnapshot {
    tick: u64,
    entity: Entity,
    position: Vec3,
    rotation: Quat,
    collider_shape: ColliderShape,
}

impl HitboxSnapshot {
    /// Constructs a hitbox snapshot for a single entity at a tick.
    #[must_use]
    pub fn new(
        tick: u64,
        entity: Entity,
        position: Vec3,
        rotation: Quat,
        collider_shape: ColliderShape,
    ) -> Self {
        Self {
            tick,
            entity,
            position,
            rotation,
            collider_shape,
        }
    }

    /// Tick this snapshot was recorded on.
    #[must_use]
    pub fn tick(self) -> u64 {
        self.tick
    }

    /// Owning entity.
    #[must_use]
    pub fn entity(self) -> Entity {
        self.entity
    }

    /// World-space collider position.
    #[must_use]
    pub fn position(self) -> Vec3 {
        self.position
    }

    /// World-space collider orientation.
    #[must_use]
    pub fn rotation(self) -> Quat {
        self.rotation
    }

    /// Collider shape at this tick.
    #[must_use]
    pub fn collider_shape(self) -> ColliderShape {
        self.collider_shape
    }
}
