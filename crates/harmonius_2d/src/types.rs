//! Core 2D types shared across sprite, physics, and tilemap helpers.

use glam::Vec2;

/// GPU blend mode affecting batch breaks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BlendMode {
    /// Standard alpha compositing.
    AlphaBlend,
    /// Additive blending.
    Additive,
}

/// Logical grid layout for tilemaps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridType {
    /// Axis-aligned square tiles.
    Orthogonal,
    /// Diamond isometric layout.
    IsometricDiamond,
    /// Staggered isometric rows.
    IsometricStaggered,
    /// Flat-top hex layout.
    HexFlatTop,
    /// Pointy-top hex layout.
    HexPointyTop,
}

/// Opaque tile identifier in a chunk.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TileId(pub u32);

/// Per-tile bitmask flags (collision, flip, autotile hints, etc.).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TileFlags(pub u32);

/// Sort keys for sprite draw ordering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayerZ {
    /// Primary bucket; lower values draw beneath higher values.
    pub sort_layer: i16,
    /// Fine ordering within a `sort_layer`.
    pub z_order: f32,
}

/// Lightweight rigid body state for deterministic stepping tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidBody2d {
    /// Simulation classification.
    pub body_type: BodyType2d,
    /// Scales world gravity before integration.
    pub gravity_scale: f32,
    /// Linear velocity in world units per second.
    pub velocity: Vec2,
    /// Enables continuous collision detection along the motion segment.
    pub ccd_enabled: bool,
}

/// Rigid body motion classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyType2d {
    /// Fully simulated.
    Dynamic,
    /// Scripted motion.
    Kinematic,
    /// Immovable collision object.
    Static,
}

/// Convex collision primitives used in narrow-phase tests.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape2d {
    /// Axis-aligned box from center with half extents.
    Box {
        /// Half width and height.
        half_extents: Vec2,
    },
    /// Convex polygon in CCW order.
    ConvexPolygon {
        /// Vertices in local space.
        vertices: smallvec::SmallVec<[Vec2; 8]>,
    },
}
