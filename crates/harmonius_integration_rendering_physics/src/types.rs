use glam::Vec3;

/// Linear-space RGBA color used by debug lines.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl LinearColor {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

/// Minimal translation-only transform for interpolation tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub translation: Vec3,
}

impl Transform {
    pub const fn from_translation(translation: Vec3) -> Self {
        Self { translation }
    }
}

/// Per-entity previous and current transforms for visual smoothing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InterpolatedTransform {
    pub previous: Transform,
    pub current: Transform,
}

/// Frame-global interpolation alpha shared by every interpolated entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InterpAlpha {
    pub value: f32,
}

/// One debug line segment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugLine {
    pub start: Vec3,
    pub end: Vec3,
    pub color: LinearColor,
}

/// Rigid body classification for collider coloring.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RigidBodyType {
    Static,
    Dynamic,
    Kinematic,
}

/// Collider shape variants that map to wireframe emitters.
#[derive(Clone, Debug, PartialEq)]
pub enum ColliderShape {
    Sphere {
        radius: f32,
    },
    Box {
        half_extents: Vec3,
    },
    Capsule {
        radius: f32,
        half_height: f32,
    },
    ConvexHull {
        points: Vec<Vec3>,
    },
    TriangleMesh {
        triangles: Vec<[Vec3; 3]>,
    },
    Heightfield {
        cols: u32,
        rows: u32,
        cell: f32,
        heights: Vec<f32>,
    },
}
