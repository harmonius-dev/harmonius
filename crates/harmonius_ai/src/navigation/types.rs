//! Core navigation identifiers and path results.

use glam::Vec3;

/// Stable identifier for an animation or gameplay ability used in link metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityId(pub u32);

/// Opaque tag carried on off-mesh links for animation selection.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AnimTag(pub u32);

/// Navigation mesh classification for traversal cost and filtering.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AreaType {
    /// Default walkable ground.
    Ground,
    /// Road surface.
    Road,
    /// Grass.
    Grass,
    /// Mud.
    Mud,
    /// Shallow water.
    Water,
    /// Swamp.
    Swamp,
    /// Lethal or impassable hazard.
    Lava,
    /// User-defined surface type.
    Custom(u16),
}

/// 2D tile coordinate in the NavMesh streaming grid (XZ plane).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TileCoord {
    /// Column index (world X / tile_size, floored).
    pub x: i32,
    /// Row index (world Z / tile_size, floored).
    pub z: i32,
}

/// Identifies a NavMesh layer (agent size class).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LayerId(pub u8);

/// Tile coordinate plus layer.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TileKey {
    /// Tile column and row.
    pub coord: TileCoord,
    /// Layer id.
    pub layer: LayerId,
}

/// Reference to one convex polygon inside a tile.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PolyRef {
    /// Owning tile key.
    pub tile: TileKey,
    /// Index into `NavMeshTile::polygons`.
    pub poly_index: u16,
}

/// Bit flags on navigation polygons.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PolyFlags(pub u16);

impl PolyFlags {
    /// Default walkable surface.
    pub const WALKABLE: Self = Self(0x01);
    /// Swimming surface.
    pub const SWIM: Self = Self(0x02);
    /// Door / dynamic portal.
    pub const DOOR: Self = Self(0x04);
    /// Jump-only connection hint.
    pub const JUMP: Self = Self(0x08);
    /// Carved or editor-disabled; not traversable.
    pub const DISABLED: Self = Self(0x10);

    /// Returns true if every set bit in `flag` is set on `self`.
    #[must_use]
    pub fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0
    }

    /// Inserts `flag` into the bit set.
    pub fn insert(&mut self, flag: Self) {
        self.0 |= flag.0;
    }

    /// Removes `flag` from the bit set.
    pub fn remove(&mut self, flag: Self) {
        self.0 &= !flag.0;
    }
}

/// Precondition for traversing an off-mesh link.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LinkPrecondition {
    /// Agent must be able to climb (ladders, etc.).
    RequiresClimb,
    /// Design-time ability gate.
    HasAbility(AbilityId),
}

/// Handle to an off-mesh link inside a tile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OffMeshLinkRef {
    /// Tile containing the link array slot.
    pub tile: TileKey,
    /// Index into `NavMeshTile::links`.
    pub link_index: u16,
}

/// One waypoint along a computed path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Waypoint {
    /// World-space position on the mesh.
    pub position: Vec3,
    /// Polygon containing this waypoint.
    pub poly_ref: PolyRef,
    /// Optional off-mesh link used to reach or leave this waypoint.
    pub link: Option<OffMeshLinkRef>,
}

/// Outcome classification for a completed query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathStatus {
    /// Full path from start to goal.
    Complete,
    /// Partial path (closest point or budget stop).
    Partial,
    /// No path exists.
    NotFound,
}

/// Pathfinding output consumed by steering and animation.
#[derive(Clone, Debug, PartialEq)]
pub struct PathResult {
    /// High-level status.
    pub status: PathStatus,
    /// Ordered waypoints for steering.
    pub waypoints: Vec<Waypoint>,
    /// Polygon corridor before funneling (optional diagnostic).
    pub corridor: Vec<PolyRef>,
    /// Total integrated traversal cost.
    pub total_cost: f32,
    /// Index of the waypoint the agent is moving toward.
    current_index: u32,
}

impl PathResult {
    /// Successful path returned by the pathfinder.
    #[must_use]
    pub fn complete(waypoints: Vec<Waypoint>, corridor: Vec<PolyRef>, total_cost: f32) -> Self {
        Self {
            status: PathStatus::Complete,
            waypoints,
            corridor,
            total_cost,
            current_index: 0,
        }
    }

    /// Builds an empty failed result.
    #[must_use]
    pub fn not_found() -> Self {
        Self {
            status: PathStatus::NotFound,
            waypoints: Vec::new(),
            corridor: Vec::new(),
            total_cost: 0.0,
            current_index: 0,
        }
    }

    /// Returns the active waypoint if any remain.
    #[must_use]
    pub fn current_waypoint(&self) -> Option<&Waypoint> {
        self.waypoints.get(self.current_index as usize)
    }

    /// Advances the follower to the next waypoint.
    pub fn advance(&mut self) {
        if (self.current_index as usize) < self.waypoints.len() {
            self.current_index = self.current_index.saturating_add(1);
        }
    }

    /// True when all waypoints have been consumed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.current_index as usize >= self.waypoints.len()
    }

    /// Remaining slice of waypoints including the current one.
    #[must_use]
    pub fn remaining_waypoints(&self) -> &[Waypoint] {
        self.waypoints
            .get(self.current_index as usize..)
            .unwrap_or(&[])
    }
}
