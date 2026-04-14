//! Target groups and framing helpers.

use glam::Vec3;

use crate::ids::Entity;

/// Member of a [`TargetGroup`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TargetGroupMember {
    /// Member entity id.
    pub entity: Entity,
    /// Member position sample.
    pub position: Vec3,
}

/// Aggregates multiple targets into one framing problem.
#[derive(Clone, Debug, PartialEq)]
pub struct TargetGroup {
    /// Group members.
    pub members: Vec<TargetGroupMember>,
}

/// Adjusts FOV to keep members inside a framing fraction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroupFraming {
    /// Target screen-space occupancy `[0, 1]`.
    pub framing_size: f32,
    /// Minimum vertical FOV (degrees).
    pub min_fov: f32,
    /// Maximum vertical FOV (degrees).
    pub max_fov: f32,
}

/// Computes the axis-aligned bounding box center for member positions.
#[must_use]
pub fn group_bounding_box_center(group: &TargetGroup) -> Option<Vec3> {
    if group.members.is_empty() {
        return None;
    }
    let mut min = group.members[0].position;
    let mut max = group.members[0].position;
    for member in &group.members {
        min = min.min(member.position);
        max = max.max(member.position);
    }
    Some((min + max) * 0.5)
}

/// Estimates a vertical FOV (degrees) that frames the group's horizontal spread.
#[must_use]
pub fn evaluate_group_framing_fov(framing: &GroupFraming, spread: f32, distance: f32) -> f32 {
    if distance <= 1e-3 {
        return framing.min_fov;
    }
    let required = 2.0 * (0.5 * spread / distance.max(1e-3)).atan().to_degrees()
        / framing.framing_size.max(1e-3);
    required.clamp(framing.min_fov, framing.max_fov)
}

/// Returns the quality score for a child camera with line-of-sight awareness.
#[must_use]
pub fn evaluate_shot_quality_with_occlusion(
    _child: Entity,
    base_score: f32,
    occluded: bool,
) -> f32 {
    if occluded {
        (base_score * 0.25).max(0.0)
    } else {
        base_score.min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.27.1 — bounding box center across three members.
    #[test]
    fn tc_13_25_27_1_target_group_bbox() {
        let group = TargetGroup {
            members: vec![
                TargetGroupMember {
                    entity: Entity(1),
                    position: Vec3::ZERO,
                },
                TargetGroupMember {
                    entity: Entity(2),
                    position: Vec3::new(10.0, 0.0, 0.0),
                },
                TargetGroupMember {
                    entity: Entity(3),
                    position: Vec3::new(0.0, 10.0, 0.0),
                },
            ],
        };
        let center = group_bounding_box_center(&group).expect("center");
        assert!((center - Vec3::new(5.0, 5.0, 0.0)).length() < 1e-3);
    }

    /// TC-13.25.28.1 — larger spreads request wider FOV within clamps.
    #[test]
    fn tc_13_25_28_1_group_framing_spread() {
        let framing = GroupFraming {
            framing_size: 0.9,
            min_fov: 30.0,
            max_fov: 120.0,
        };
        let narrow = evaluate_group_framing_fov(&framing, 10.0, 25.0);
        let wide = evaluate_group_framing_fov(&framing, 50.0, 25.0);
        assert!(wide > narrow);
    }

    /// TC-13.25.25.1 — occlusion lowers shot quality scores.
    #[test]
    fn tc_13_25_25_1_shot_quality_occlusion() {
        let clear = evaluate_shot_quality_with_occlusion(Entity(1), 1.0, false);
        let blocked = evaluate_shot_quality_with_occlusion(Entity(1), 1.0, true);
        assert!(clear > blocked);
    }
}
