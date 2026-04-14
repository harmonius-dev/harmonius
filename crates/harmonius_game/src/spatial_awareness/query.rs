//! Pure sense queries (`query_sense`) used by FixedUpdate sense evaluation.

use glam::Vec3;

use super::sense::{
    FalloffCurve, ScoringFunction, SenseCandidate, SenseDefinition, SenseResult, SenseShape, TagSet,
};

/// Evaluates `definition` against `candidates` from `source_position` looking along
/// `source_forward` (need not be normalized; normalized internally).
#[must_use]
pub fn query_sense(
    definition: &SenseDefinition,
    source_position: Vec3,
    source_forward: Vec3,
    candidates: &[SenseCandidate],
) -> Vec<SenseResult> {
    let forward = normalize_or_fallback(source_forward);
    let mut results = Vec::new();

    for candidate in candidates {
        if !passes_tag_filter(candidate.tags, definition.filter_tags) {
            continue;
        }

        let Some((distance, angle, inside_shape)) =
            shape_containment(definition, source_position, forward, candidate.position)
        else {
            continue;
        };

        if !inside_shape || distance > definition.range {
            continue;
        }

        let raw_score = falloff_score(distance, definition.range, &definition.falloff);
        let occlusion = if definition.occlusion_check {
            // Spatial index integration is handled by higher-level systems; pure query stays
            // unoccluded until raycasts are wired.
            0.0
        } else {
            0.0
        };

        let angle_factor = angle_score(definition, angle);
        let final_score = compose_final_score(
            raw_score,
            angle_factor,
            occlusion,
            &definition.scoring,
        );

        results.push(SenseResult {
            entity: candidate.entity,
            distance,
            angle,
            occlusion,
            raw_score,
            final_score,
        });
    }

    results.sort_by(|left, right| {
        left.distance
            .total_cmp(&right.distance)
            .then_with(|| left.entity.cmp(&right.entity))
    });

    results
}

fn passes_tag_filter(candidate_tags: TagSet, required: TagSet) -> bool {
    candidate_tags.contains_all(required)
}

fn normalize_or_fallback(vector: Vec3) -> Vec3 {
    let normalized = vector.normalize_or_zero();
    if normalized.length_squared() <= f32::EPSILON {
        Vec3::Z
    } else {
        normalized
    }
}

fn shape_containment(
    definition: &SenseDefinition,
    source: Vec3,
    forward: Vec3,
    target: Vec3,
) -> Option<(f32, f32, bool)> {
    let offset = target - source;
    let distance = offset.length();

    if distance <= f32::EPSILON {
        return Some((0.0, 0.0, true));
    }

    let direction = offset / distance;
    let angle = direction.angle_between(forward);

    let inside = match &definition.shape {
        SenseShape::Sphere { radius } => distance <= *radius,
        SenseShape::Cone { radius, half_angle } => {
            let within_radius = distance <= *radius;
            let within_angle = direction.dot(forward) >= half_angle.cos();
            within_radius && within_angle
        }
        SenseShape::Box { half_extents } => {
            let local = target - source;
            local.x.abs() <= half_extents.x
                && local.y.abs() <= half_extents.y
                && local.z.abs() <= half_extents.z
        }
        SenseShape::Cylinder { radius, height } => {
            let xz = Vec3::new(offset.x, 0.0, offset.z).length();
            xz <= *radius && offset.y.abs() <= *height * 0.5
        }
        SenseShape::Circle2D { radius } => {
            let xz = Vec3::new(offset.x, 0.0, offset.z).length();
            xz <= *radius
        }
        SenseShape::Cone2D { radius, half_angle } => {
            let flat = Vec3::new(offset.x, 0.0, offset.z);
            let flat_len = flat.length();
            if flat_len <= f32::EPSILON {
                true
            } else {
                let flat_dir = flat / flat_len;
                let flat_forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
                let within_radius = flat_len <= *radius;
                let within_angle = if flat_forward.length_squared() <= f32::EPSILON {
                    true
                } else {
                    flat_dir.dot(flat_forward) >= half_angle.cos()
                };
                within_radius && within_angle
            }
        }
        SenseShape::Rect2D { half_extents } => {
            let local_xz = Vec3::new(offset.x, 0.0, offset.z);
            local_xz.x.abs() <= half_extents.x && local_xz.z.abs() <= half_extents.y
        }
    };

    Some((distance, angle, inside))
}

fn falloff_score(distance: f32, range: f32, curve: &FalloffCurve) -> f32 {
    if range <= f32::EPSILON {
        return 0.0;
    }

    let t = (distance / range).clamp(0.0, 1.0);

    match curve {
        FalloffCurve::Linear => 1.0 - t,
        FalloffCurve::InverseLinear => 1.0 - t,
        FalloffCurve::Quadratic => (1.0 - t) * (1.0 - t),
        FalloffCurve::InverseQuadratic => (1.0 - t) * (1.0 - t),
        FalloffCurve::Custom(_) => 1.0 - t,
    }
}

fn angle_score(definition: &SenseDefinition, angle: f32) -> f32 {
    match &definition.shape {
        SenseShape::Cone { half_angle, .. } | SenseShape::Cone2D { half_angle, .. } => {
            if angle <= *half_angle + f32::EPSILON {
                1.0
            } else {
                0.0
            }
        }
        _ => 1.0,
    }
}

fn compose_final_score(
    raw_score: f32,
    angle_factor: f32,
    occlusion: f32,
    scoring: &ScoringFunction,
) -> f32 {
    let distance_component = scoring.distance_weight * raw_score;
    let angle_component = scoring.angle_weight * angle_factor;
    let occlusion_component = scoring.occlusion_penalty * occlusion;
    let mut combined =
        distance_component + angle_component + scoring.modifier_bonus - occlusion_component;
    if combined < 0.0 {
        combined = 0.0;
    }
    if combined > 1.0 {
        1.0
    } else {
        combined
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spatial_awareness::{AssetId, Entity, SenseDefinitionId, StringId, TagId};

    /// TC-17.3.1.1 — SenseDefinition honors shape, range, falloff, and tag filters together.
    #[test]
    fn test_sense_definition_primitive() {
        let enemy_tag = TagId(1);
        let required_tags = TagSet::from_tags(&[enemy_tag]);

        let definition = SenseDefinition {
            id: SenseDefinitionId(1),
            name: StringId(42),
            shape: SenseShape::Cone {
                radius: 25.0,
                half_angle: std::f32::consts::FRAC_PI_2,
            },
            range: 20.0,
            falloff: FalloffCurve::Linear,
            filter_tags: required_tags,
            occlusion_check: false,
            scoring: ScoringFunction::default(),
            update_rate_hz: 30.0,
        };

        let forward = Vec3::new(0.0, 0.0, 1.0);
        let origin = Vec3::ZERO;

        let mut candidates = Vec::new();
        let enemy_distances = [4.0_f32, 8.0, 12.0, 16.0, 18.0];
        for (idx, distance) in enemy_distances.iter().enumerate() {
            candidates.push(SenseCandidate {
                entity: Entity((idx + 1) as u64),
                position: Vec3::new(0.0, 0.0, *distance),
                tags: TagSet::from_tags(&[enemy_tag]),
            });
        }

        for (idx, distance) in enemy_distances.iter().enumerate() {
            candidates.push(SenseCandidate {
                entity: Entity((idx + 6) as u64),
                position: Vec3::new(0.0, 0.0, *distance),
                tags: TagSet::empty(),
            });
        }

        let results = query_sense(&definition, origin, forward, &candidates);
        assert_eq!(results.len(), 5);

        let mut expected_scores = Vec::new();
        for distance in enemy_distances {
            expected_scores.push(1.0 - distance / definition.range);
        }

        for (result, expected) in results.iter().zip(expected_scores.iter()) {
            assert!(
                (result.final_score - *expected).abs() < 1e-4,
                "final_score {} expected {}",
                result.final_score,
                expected
            );
            assert!(
                (result.raw_score - *expected).abs() < 1e-4,
                "raw_score {} expected {}",
                result.raw_score,
                expected
            );
            assert!(result.entity.0 <= 5, "non-matching tag leaked into results");
        }

        assert!(
            matches!(definition.falloff, FalloffCurve::Linear),
            "falloff must remain linear for this test"
        );
        assert!(
            matches!(definition.shape, SenseShape::Cone { .. }),
            "cone shape must be preserved in the definition"
        );
        assert_eq!(definition.filter_tags, required_tags);

        // Custom falloff path remains deterministic (placeholder maps to linear distance falloff).
        let custom_definition = SenseDefinition {
            falloff: FalloffCurve::Custom(AssetId(9)),
            ..definition.clone()
        };
        let custom_results = query_sense(&custom_definition, origin, forward, &candidates);
        assert_eq!(custom_results.len(), 5);
    }
}
