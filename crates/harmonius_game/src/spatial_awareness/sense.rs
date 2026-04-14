//! Core spatial awareness data types (`SenseDefinition`, shapes, falloff, tags).

use glam::{Vec2, Vec3};

/// Stable identifier for an entity participating in sense queries.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Entity(pub u64);

/// Opaque asset handle used by [`FalloffCurve::Custom`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AssetId(pub u32);

/// Editor string table identifier (data-driven senses).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StringId(pub u32);

/// Unique identifier for a sense definition asset.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SenseDefinitionId(pub u32);

/// Single gameplay tag used for bitmask filters.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TagId(pub u32);

/// Bitmask tag set (`TagId` maps to `1 << bit` for `bit < 64`).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TagSet(u64);

impl TagSet {
    /// Returns an empty tag set (matches every candidate for filtering purposes).
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns whether no tag bits are set.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Builds a set from individual tags (OR-combined).
    pub fn from_tags(tags: &[TagId]) -> Self {
        let mut bits = 0_u64;
        for tag in tags {
            bits |= 1_u64 << (tag.0 as u64);
        }
        Self(bits)
    }

    /// Returns true when every bit in `required` is present on `self`.
    pub const fn contains_all(self, required: Self) -> bool {
        if required.is_empty() {
            return true;
        }
        (self.0 & required.0) == required.0
    }
}

/// Geometric shape of a sense volume.
#[derive(Clone, Debug, PartialEq)]
pub enum SenseShape {
    /// Omnidirectional radius in world units.
    Sphere {
        /// Maximum distance from the sense origin.
        radius: f32,
    },
    /// Directional cone in world space.
    Cone {
        /// Radial reach along the forward axis (also bounded by [`SenseDefinition::range`]).
        radius: f32,
        /// Half-angle in radians measured from the forward vector.
        half_angle: f32,
    },
    /// Axis-aligned box in world space.
    Box {
        /// Half-extents along each axis centered on the sense origin.
        half_extents: Vec3,
    },
    /// Vertical cylinder.
    Cylinder {
        /// Radius on the XZ plane.
        radius: f32,
        /// Total height along Y.
        height: f32,
    },
    /// 2D circle for `Transform2D` entities.
    Circle2D {
        /// Radius in the 2D plane.
        radius: f32,
    },
    /// 2D cone for `Transform2D` entities.
    Cone2D {
        /// Radial reach in the 2D plane.
        radius: f32,
        /// Half-angle in radians in the 2D plane.
        half_angle: f32,
    },
    /// 2D axis-aligned rectangle for `Transform2D` entities.
    Rect2D {
        /// Half-extents in the 2D plane.
        half_extents: Vec2,
    },
}

/// Falloff curve controlling distance attenuation for sense scores.
#[derive(Clone, Debug, PartialEq)]
pub enum FalloffCurve {
    /// Score decreases linearly from 1 at the origin to 0 at max range.
    Linear,
    /// High score near the origin, linear drop to zero at max range.
    InverseLinear,
    /// Quadratic drop with distance.
    Quadratic,
    /// Inverse quadratic drop with distance.
    InverseQuadratic,
    /// Designer curve sampled from an asset.
    Custom(AssetId),
}

/// Weights applied when composing the final sense score.
#[derive(Clone, Debug, PartialEq)]
pub struct ScoringFunction {
    /// Weight for the distance factor in `[0, 1]`.
    pub distance_weight: f32,
    /// Weight for the angular factor in `[0, 1]` (directional senses).
    pub angle_weight: f32,
    /// Occlusion penalty scaled by `occlusion` in `[0, 1]`.
    pub occlusion_penalty: f32,
    /// Additional modifier bonus in `[-1, 1]`.
    pub modifier_bonus: f32,
}

impl Default for ScoringFunction {
    fn default() -> Self {
        Self {
            distance_weight: 1.0,
            angle_weight: 0.0,
            occlusion_penalty: 0.0,
            modifier_bonus: 0.0,
        }
    }
}

/// Immutable sense definition authored from data tables / editor.
#[derive(Clone, Debug, PartialEq)]
pub struct SenseDefinition {
    /// Identifier for this sense asset.
    pub id: SenseDefinitionId,
    /// Human-readable name for tooling.
    pub name: StringId,
    /// Geometric shape of the sense volume.
    pub shape: SenseShape,
    /// Maximum range in world units (canonical cap for falloff and distance tests).
    pub range: f32,
    /// Distance attenuation curve.
    pub falloff: FalloffCurve,
    /// Candidate must contain all bits in this mask when non-empty.
    pub filter_tags: TagSet,
    /// Whether occlusion raycasts should be evaluated (requires spatial index integration).
    pub occlusion_check: bool,
    /// Weighting model for composing [`SenseResult::final_score`].
    pub scoring: ScoringFunction,
    /// Evaluation cadence in hertz (budgeting / scheduling).
    pub update_rate_hz: f32,
}

/// One candidate entity supplied to [`super::query_sense`].
#[derive(Clone, Debug, PartialEq)]
pub struct SenseCandidate {
    /// Entity id for stable ordering and results.
    pub entity: Entity,
    /// World-space position of the candidate.
    pub position: Vec3,
    /// Tag bits carried by the candidate.
    pub tags: TagSet,
}

/// Output row for a single sensed target.
#[derive(Clone, Debug, PartialEq)]
pub struct SenseResult {
    /// Entity that produced this hit.
    pub entity: Entity,
    /// Distance from the sense origin to the candidate.
    pub distance: f32,
    /// Angle between forward and the vector to the candidate (radians).
    pub angle: f32,
    /// Occlusion factor where `0` is fully visible and `1` is fully occluded.
    pub occlusion: f32,
    /// Score after falloff, before composing weights.
    pub raw_score: f32,
    /// Weighted, clamped score in `[0, 1]`.
    pub final_score: f32,
}

/// Awareness levels used by higher-level state machines (exported for future modules).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AwarenessLevel {
    /// No knowledge of the target.
    Unaware,
    /// Weak signal.
    Suspicious,
    /// Confirmed contact.
    Alert,
    /// Maintaining contact.
    Tracking,
    /// Contact lost.
    Lost,
}
