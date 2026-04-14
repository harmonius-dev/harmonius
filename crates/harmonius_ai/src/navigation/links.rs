//! Off-mesh links between navigation polygons.

use glam::Vec3;

use super::types::{AbilityId, AnimTag, AreaType, LinkPrecondition, PolyRef};

/// Traversal direction policy for a link.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LinkDirection {
    /// Traversable in both directions.
    Bidirectional,
    /// Traversable only from start polygon to end polygon.
    OneWay,
}

/// Connection between two mesh locations, possibly across a gap.
#[derive(Clone, Debug, PartialEq)]
pub struct OffMeshLink {
    /// World-space start.
    pub start_pos: Vec3,
    /// World-space end.
    pub end_pos: Vec3,
    /// Polygon at the start.
    pub start_poly: PolyRef,
    /// Polygon at the end.
    pub end_poly: PolyRef,
    /// Agent stand radius for snapping.
    pub radius: f32,
    /// Extra traversal cost added to the path integral.
    pub cost: f32,
    /// Direction policy.
    pub direction: LinkDirection,
    /// Area classification (for filters and costs).
    pub area_type: AreaType,
    /// Animation hint.
    pub animation_tag: AnimTag,
    /// Optional gating predicate.
    pub precondition: Option<LinkPrecondition>,
    /// Optional ability id when using `LinkPrecondition::HasAbility`.
    pub ability_id: Option<AbilityId>,
}

impl OffMeshLink {
    /// Returns true if an agent with `can_climb` may use this link.
    #[must_use]
    pub fn satisfied_by(&self, can_climb: bool) -> bool {
        match self.precondition {
            None | Some(LinkPrecondition::HasAbility(_)) => true,
            Some(LinkPrecondition::RequiresClimb) => can_climb,
        }
    }
}
