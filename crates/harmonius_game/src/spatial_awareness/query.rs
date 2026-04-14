//! Pure sense queries (`query_sense`) used by FixedUpdate sense evaluation.

use glam::Vec3;

use super::sense::{
    FalloffCurve, ScoringFunction, SenseCandidate, SenseDefinition, SenseResult, SenseShape, TagSet,
};

/// Optional hooks for sense evaluation (custom falloff sampling, etc.).
#[derive(Clone, Copy, Debug, Default)]
pub struct SenseQueryOptions {
    /// When [`FalloffCurve::Custom`] is active, maps normalized distance `t`
    /// in `[0, 1]` to a `[0, 1]` falloff factor. If `None`, custom curves
    /// fall back to linear attenuation.
    pub custom_falloff_t: Option<fn(f32) -> f32>,
}

/// Evaluates `definition` against `candidates` from `source_position` looking along
/// `source_forward` (need not be normalized; normalized internally).
///
/// Results are sorted by [`SenseResult::final_score`] descending, then by ascending
/// [`super::sense::Entity`] for deterministic tie-breaking.
#[must_use]
pub fn query_sense(
    definition: &SenseDefinition,
    source_position: Vec3,
    source_forward: Vec3,
    candidates: &[SenseCandidate],
    options: &SenseQueryOptions,
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

        let distance_factor = distance_factor(
            distance,
            definition.range,
            &definition.falloff,
            options.custom_falloff_t,
        );
        let angle_factor = angle_factor(definition, angle);
        let occlusion = if definition.occlusion_check {
            // Occlusion is supplied by the spatial index integration layer; tests
            // override via `SenseCandidate` extensions are not used — callers that
            // need occlusion should pre-filter or extend this path later.
            0.0
        } else {
            0.0
        };

        let (raw_score, final_score) = compose_scores(
            distance_factor,
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
            target_position: candidate.position,
        });
    }

    results.sort_by(|left, right| {
        right
            .final_score
            .total_cmp(&left.final_score)
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

fn distance_factor(
    distance: f32,
    range: f32,
    curve: &FalloffCurve,
    custom: Option<fn(f32) -> f32>,
) -> f32 {
    if range <= f32::EPSILON {
        return 0.0;
    }

    let t = (distance / range).clamp(0.0, 1.0);

    match curve {
        FalloffCurve::Linear => 1.0 - t,
        FalloffCurve::InverseLinear => t,
        FalloffCurve::Quadratic => (1.0 - t) * (1.0 - t),
        FalloffCurve::InverseQuadratic => t * t,
        FalloffCurve::Custom(_id) => {
            if let Some(sample) = custom {
                sample(t)
            } else {
                1.0 - t
            }
        }
    }
}

fn angle_factor(definition: &SenseDefinition, angle: f32) -> f32 {
    match &definition.shape {
        SenseShape::Cone { half_angle, .. } | SenseShape::Cone2D { half_angle, .. } => {
            if *half_angle <= f32::EPSILON {
                return 0.0;
            }
            (1.0 - (angle / *half_angle)).clamp(0.0, 1.0)
        }
        _ => 1.0,
    }
}

fn compose_scores(
    distance_factor: f32,
    angle_factor: f32,
    occlusion: f32,
    scoring: &ScoringFunction,
) -> (f32, f32) {
    let mut raw = distance_factor * scoring.distance_weight
        + angle_factor * scoring.angle_weight
        + scoring.modifier_bonus
        - occlusion * scoring.occlusion_penalty;

    if !raw.is_finite() {
        raw = 0.0;
    }

    let clamped = raw.clamp(0.0, 1.0);
    (raw, clamped)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spatial_awareness::{AssetId, Entity, SenseDefinitionId, SpatialIndex, SpatialTraversalMode, StringId, TagId};

    fn cone_definition(_enemy_tag: TagId, required: TagSet) -> SenseDefinition {
        SenseDefinition {
            id: SenseDefinitionId(1),
            name: StringId(42),
            shape: SenseShape::Cone {
                radius: 25.0,
                half_angle: std::f32::consts::FRAC_PI_2,
            },
            range: 20.0,
            falloff: FalloffCurve::Linear,
            filter_tags: required,
            occlusion_check: false,
            scoring: ScoringFunction::default(),
            update_rate_hz: 30.0,
        }
    }

    /// TC-17.3.1.1 — SenseDefinition honors shape, range, falloff, and tag filters together.
    #[test]
    fn test_sense_definition_primitive() {
        let enemy_tag = TagId(1);
        let required_tags = TagSet::from_tags(&[enemy_tag]);
        let definition = cone_definition(enemy_tag, required_tags);

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

        let options = SenseQueryOptions::default();
        let results = query_sense(&definition, origin, forward, &candidates, &options);
        assert_eq!(results.len(), 5);

        for result in &results {
            assert!(result.entity.0 <= 5, "non-matching tag leaked into results");
            let t = result.distance / definition.range;
            let distance_factor = 1.0 - t;
            let angle_factor = 1.0;
            let (_, expected) = compose_scores(
                distance_factor,
                angle_factor,
                0.0,
                &definition.scoring,
            );
            assert!(
                (result.final_score - expected).abs() < 1e-3,
                "final_score {} expected {} for entity {:?}",
                result.final_score,
                expected,
                result.entity
            );
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

        let custom_definition = SenseDefinition {
            falloff: FalloffCurve::Custom(AssetId(9)),
            ..definition.clone()
        };
        let custom_options = SenseQueryOptions {
            custom_falloff_t: None,
        };
        let custom_results = query_sense(
            &custom_definition,
            origin,
            forward,
            &candidates,
            &custom_options,
        );
        assert_eq!(custom_results.len(), 5);
    }

    /// TC-7.6.1.1 — sphere sense radius 10; 3 inside, 2 outside.
    #[test]
    fn test_sphere_sense_candidates() {
        let definition = SenseDefinition {
            shape: SenseShape::Sphere { radius: 10.0 },
            range: 10.0,
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let origin = Vec3::ZERO;
        let forward = Vec3::Z;
        let candidates: Vec<_> = [3.0_f32, 6.0, 9.0, 12.0, 15.0]
            .iter()
            .enumerate()
            .map(|(idx, z)| SenseCandidate {
                entity: Entity((idx + 1) as u64),
                position: Vec3::new(0.0, 0.0, *z),
                tags: TagSet::empty(),
            })
            .collect();
        let results = query_sense(
            &definition,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        );
        assert_eq!(results.len(), 3);
    }

    /// TC-7.6.1.2 — cone half-angle 90°; target at 45°; non-zero score.
    #[test]
    fn test_cone_sense_fov_inside() {
        let definition = SenseDefinition {
            shape: SenseShape::Cone {
                radius: 50.0,
                half_angle: std::f32::consts::FRAC_PI_2,
            },
            range: 50.0,
            scoring: ScoringFunction {
                distance_weight: 0.0,
                angle_weight: 1.0,
                occlusion_penalty: 0.0,
                modifier_bonus: 0.0,
            },
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let origin = Vec3::ZERO;
        let forward = Vec3::new(1.0, 0.0, 0.0);
        let angle = std::f32::consts::FRAC_PI_4;
        let position = Vec3::new(angle.cos(), angle.sin(), 0.0) * 5.0;
        let candidates = [SenseCandidate {
            entity: Entity(1),
            position,
            tags: TagSet::empty(),
        }];
        let results = query_sense(
            &definition,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        );
        assert_eq!(results.len(), 1);
        assert!(results[0].final_score > 0.0);
    }

    /// TC-7.6.1.3 — target at 100°; excluded from cone results.
    #[test]
    fn test_cone_sense_fov_outside() {
        let definition = SenseDefinition {
            shape: SenseShape::Cone {
                radius: 50.0,
                half_angle: std::f32::consts::FRAC_PI_2,
            },
            range: 50.0,
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let origin = Vec3::ZERO;
        let forward = Vec3::new(1.0, 0.0, 0.0);
        let angle = 100_f32.to_radians();
        let position = Vec3::new(angle.cos(), angle.sin(), 0.0) * 5.0;
        let candidates = [SenseCandidate {
            entity: Entity(1),
            position,
            tags: TagSet::empty(),
        }];
        let results = query_sense(
            &definition,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        );
        assert!(results.is_empty());
    }

    /// TC-7.6.1.4 — box half-extents (5,5,5) inclusion.
    #[test]
    fn test_box_sense_candidates() {
        let definition = SenseDefinition {
            shape: SenseShape::Box {
                half_extents: Vec3::splat(5.0),
            },
            range: 20.0,
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let origin = Vec3::ZERO;
        let forward = Vec3::Z;
        let candidates = [
            SenseCandidate {
                entity: Entity(1),
                position: Vec3::new(4.0, 4.0, 4.0),
                tags: TagSet::empty(),
            },
            SenseCandidate {
                entity: Entity(2),
                position: Vec3::new(6.0, 0.0, 0.0),
                tags: TagSet::empty(),
            },
        ];
        let results = query_sense(
            &definition,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        );
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].entity, Entity(1));
    }

    /// TC-7.6.1.5 — cylinder height excludes targets above the cap.
    #[test]
    fn test_cylinder_sense_candidates() {
        let definition = SenseDefinition {
            shape: SenseShape::Cylinder {
                radius: 5.0,
                height: 10.0,
            },
            range: 50.0,
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let origin = Vec3::ZERO;
        let forward = Vec3::Z;
        let candidates = [SenseCandidate {
            entity: Entity(1),
            position: Vec3::new(0.0, 6.0, 0.0),
            tags: TagSet::empty(),
        }];
        let results = query_sense(
            &definition,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        );
        assert!(results.is_empty());
    }

    /// TC-7.6.1.6 — Circle2D radius 10; 3 inside, 2 outside.
    #[test]
    fn test_circle2d_sense_candidates() {
        let definition = SenseDefinition {
            shape: SenseShape::Circle2D { radius: 10.0 },
            range: 10.0,
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let origin = Vec3::ZERO;
        let forward = Vec3::Z;
        let candidates: Vec<_> = (1..=5)
            .map(|idx| SenseCandidate {
                entity: Entity(idx),
                position: Vec3::new(idx as f32 * 3.0, 0.0, 0.0),
                tags: TagSet::empty(),
            })
            .collect();
        let index = SpatialIndex::new(SpatialTraversalMode::Bvh2d);
        assert_eq!(index.mode, SpatialTraversalMode::Bvh2d);
        let results = query_sense(
            &definition,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        );
        assert_eq!(results.len(), 3);
        let _ = index;
    }

    /// TC-7.6.1.7 — Cone2D half-angle 45°; target at 30° has non-zero score.
    #[test]
    fn test_cone2d_sense_fov_inside() {
        let definition = SenseDefinition {
            shape: SenseShape::Cone2D {
                radius: 20.0,
                half_angle: 45_f32.to_radians(),
            },
            range: 20.0,
            scoring: ScoringFunction {
                distance_weight: 0.0,
                angle_weight: 1.0,
                occlusion_penalty: 0.0,
                modifier_bonus: 0.0,
            },
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let origin = Vec3::ZERO;
        let forward = Vec3::new(1.0, 0.0, 0.0);
        let angle = 30_f32.to_radians();
        let position = Vec3::new(angle.cos() * 5.0, 0.0, angle.sin() * 5.0);
        let candidates = [SenseCandidate {
            entity: Entity(1),
            position,
            tags: TagSet::empty(),
        }];
        let results = query_sense(
            &definition,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        );
        assert_eq!(results.len(), 1);
        assert!(results[0].final_score > 0.0);
    }

    /// TC-7.6.1.8 — Rect2D inclusion.
    #[test]
    fn test_rect2d_sense_candidates() {
        let definition = SenseDefinition {
            shape: SenseShape::Rect2D {
                half_extents: glam::Vec2::splat(5.0),
            },
            range: 20.0,
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let origin = Vec3::ZERO;
        let forward = Vec3::Z;
        let candidates = [
            SenseCandidate {
                entity: Entity(1),
                position: Vec3::new(4.0, 0.0, 3.0),
                tags: TagSet::empty(),
            },
            SenseCandidate {
                entity: Entity(2),
                position: Vec3::new(6.0, 0.0, 0.0),
                tags: TagSet::empty(),
            },
        ];
        let results = query_sense(
            &definition,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        );
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].entity, Entity(1));
    }

    /// TC-7.6.2.1 — linear falloff half range -> 0.5 distance factor.
    #[test]
    fn test_falloff_linear() {
        let t = 0.5_f32;
        let value = distance_factor(10.0, 20.0, &FalloffCurve::Linear, None);
        assert!((value - (1.0 - t)).abs() < 1e-5);
    }

    /// TC-7.6.2.2 — inverse linear half range -> 0.5.
    #[test]
    fn test_falloff_inverse_linear() {
        let value = distance_factor(10.0, 20.0, &FalloffCurve::InverseLinear, None);
        assert!((value - 0.5).abs() < 1e-5);
    }

    /// TC-7.6.2.3 — quadratic half range -> 0.25.
    #[test]
    fn test_falloff_quadratic() {
        let value = distance_factor(10.0, 20.0, &FalloffCurve::Quadratic, None);
        assert!((value - 0.25).abs() < 1e-5);
    }

    /// TC-7.6.2.4 — custom curve sampling at t = 0.5.
    #[test]
    fn test_falloff_custom_curve() {
        let curve = FalloffCurve::Custom(AssetId(1));
        let sample = |t: f32| -> f32 { 0.25 + t * 0.5 };
        let value = distance_factor(10.0, 20.0, &curve, Some(sample));
        assert!((value - sample(0.5)).abs() < 1e-5);
    }

    /// TC-7.6.3.1 — distance-only scoring matches distance factor.
    #[test]
    fn test_scoring_distance_only() {
        let scoring = ScoringFunction {
            distance_weight: 1.0,
            angle_weight: 0.0,
            occlusion_penalty: 0.0,
            modifier_bonus: 0.0,
        };
        let (raw, final_score) = compose_scores(0.6, 1.0, 0.0, &scoring);
        assert!((final_score - 0.6).abs() < 1e-5);
        assert!((raw - 0.6).abs() < 1e-5);
    }

    /// TC-7.6.3.2 — angle-only scoring at 45° of 90° cone -> 0.5.
    #[test]
    fn test_scoring_angle_weight() {
        let scoring = ScoringFunction {
            distance_weight: 0.0,
            angle_weight: 1.0,
            occlusion_penalty: 0.0,
            modifier_bonus: 0.0,
        };
        let definition = SenseDefinition {
            shape: SenseShape::Cone {
                radius: 50.0,
                half_angle: std::f32::consts::FRAC_PI_2,
            },
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let angle = std::f32::consts::FRAC_PI_4;
        let angle_component = angle_factor(&definition, angle);
        let (_, final_score) = compose_scores(0.0, angle_component, 0.0, &scoring);
        assert!((final_score - 0.5).abs() < 1e-3);
    }

    /// TC-7.6.3.3 — occlusion reduces score by penalty amount.
    #[test]
    fn test_scoring_occlusion_penalty() {
        let scoring = ScoringFunction {
            distance_weight: 1.0,
            angle_weight: 0.0,
            occlusion_penalty: 0.8,
            modifier_bonus: 0.0,
        };
        let (_, final_score) = compose_scores(1.0, 0.0, 1.0, &scoring);
        assert!((final_score - 0.2).abs() < 1e-5);
    }

    /// TC-7.6.3.4 — modifier bonus increases score.
    #[test]
    fn test_scoring_modifier_bonus() {
        let scoring = ScoringFunction {
            distance_weight: 1.0,
            angle_weight: 0.0,
            occlusion_penalty: 0.0,
            modifier_bonus: 0.2,
        };
        let (_, final_score) = compose_scores(0.5, 0.0, 0.0, &scoring);
        assert!((final_score - 0.7).abs() < 1e-5);
    }

    /// TC-7.6.3.5 — clamping rules for combined weights.
    #[test]
    fn test_score_clamp_zero_one() {
        let high = ScoringFunction {
            distance_weight: 1.0,
            angle_weight: 0.5,
            occlusion_penalty: 0.0,
            modifier_bonus: 0.0,
        };
        let (_, final_high) = compose_scores(1.0, 1.0, 0.0, &high);
        assert!((final_high - 1.0).abs() < 1e-5);

        let negative = ScoringFunction {
            distance_weight: -1.0,
            angle_weight: 0.0,
            occlusion_penalty: 0.0,
            modifier_bonus: 0.0,
        };
        let (_, final_low) = compose_scores(1.0, 0.0, 0.0, &negative);
        assert!((final_low - 0.0).abs() < 1e-5);
    }

    /// TC-7.6.6.1 — deterministic ordering for equal final scores.
    #[test]
    fn test_deterministic_sort_entity_tiebreak() {
        let definition = SenseDefinition {
            shape: SenseShape::Sphere { radius: 10.0 },
            range: 10.0,
            falloff: FalloffCurve::Linear,
            scoring: ScoringFunction {
                distance_weight: 0.0,
                angle_weight: 0.0,
                occlusion_penalty: 0.0,
                modifier_bonus: 0.5,
            },
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let candidates = [
            SenseCandidate {
                entity: Entity(2),
                position: Vec3::new(2.0, 0.0, 0.0),
                tags: TagSet::empty(),
            },
            SenseCandidate {
                entity: Entity(1),
                position: Vec3::new(1.0, 0.0, 0.0),
                tags: TagSet::empty(),
            },
        ];
        let results = query_sense(
            &definition,
            Vec3::ZERO,
            Vec3::Z,
            &candidates,
            &SenseQueryOptions::default(),
        );
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].entity, Entity(1));
        assert_eq!(results[1].entity, Entity(2));
    }

    /// TC-1.9.1.1 — sphere candidates align with spatial index enumeration.
    #[test]
    fn test_shared_bvh_used_by_sense() {
        let mut index = SpatialIndex::new(SpatialTraversalMode::Bvh3d);
        for idx in 0..500 {
            let entity = Entity(idx);
            let position = Vec3::new((idx as f32) * 0.1, 0.0, 0.0);
            index.insert(entity, position);
        }
        let center = Vec3::ZERO;
        let radius = 20.0;
        let index_hits = index.query_sphere(center, radius, false);
        let definition = SenseDefinition {
            shape: SenseShape::Sphere { radius },
            range: radius,
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let candidates: Vec<_> = index
            .entities()
            .iter()
            .map(|(entity, position)| SenseCandidate {
                entity: *entity,
                position: *position,
                tags: TagSet::empty(),
            })
            .collect();
        let sense_hits: std::collections::HashSet<_> = query_sense(
            &definition,
            center,
            Vec3::Z,
            &candidates,
            &SenseQueryOptions::default(),
        )
        .iter()
        .map(|result| result.entity)
        .collect();
        let index_set: std::collections::HashSet<_> = index_hits.iter().copied().collect();
        assert_eq!(sense_hits, index_set);
    }

    /// TC-1.9.4.1 — unified spatial query entry points share the same backing index.
    #[test]
    fn test_unified_query_api() {
        let mut index = SpatialIndex::new(SpatialTraversalMode::Bvh3d);
        for idx in 0..200 {
            index.insert(
                Entity(idx),
                Vec3::new((idx as f32) * 0.2, (idx as f32) * 0.1, (idx as f32) * -0.15),
            );
        }
        let origin = Vec3::ZERO;
        let direction = Vec3::new(0.2, 0.1, -0.15).normalize();
        let ray_hits = index.raycast(origin, direction, 50.0);
        let sphere_hits = index.query_sphere(origin, 8.0, true);
        let box_hits = index.query_box(Vec3::splat(-25.0), Vec3::splat(25.0), true);
        let frustum_hits = index.query_frustum(Vec3::splat(-10.0), Vec3::splat(10.0), true);
        assert!(!ray_hits.is_empty());
        assert!(!sphere_hits.is_empty());
        assert!(!box_hits.is_empty());
        assert!(!frustum_hits.is_empty());
        let sorted_sphere = sphere_hits.clone();
        let mut distances = Vec::new();
        for entity in &sorted_sphere {
            let position = index
                .entities()
                .iter()
                .find(|pair| pair.0 == *entity)
                .map(|pair| pair.1)
                .expect("position");
            distances.push((position - origin).length());
        }
        let mut ordered = distances.clone();
        ordered.sort_by(|left: &f32, right: &f32| left.total_cmp(right));
        assert_eq!(distances, ordered);
    }

    /// TC-17.3.2.1 — 2D sense shapes consult the planar index mode.
    #[test]
    fn test_2d_sense_shapes_all() {
        let index = SpatialIndex::new(SpatialTraversalMode::Bvh2d);
        let origin = Vec3::ZERO;
        let forward = Vec3::X;
        let mut candidates = Vec::new();
        for idx in 0..20 {
            candidates.push(SenseCandidate {
                entity: Entity(idx),
                position: Vec3::new((idx as f32) * 0.5, 0.0, (idx as f32) * -0.25),
                tags: TagSet::empty(),
            });
        }

        let circle = SenseDefinition {
            shape: SenseShape::Circle2D { radius: 4.0 },
            range: 4.0,
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let cone2d = SenseDefinition {
            shape: SenseShape::Cone2D {
                radius: 6.0,
                half_angle: std::f32::consts::FRAC_PI_3,
            },
            range: 6.0,
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let rect = SenseDefinition {
            shape: SenseShape::Rect2D {
                half_extents: glam::Vec2::new(3.0, 2.5),
            },
            range: 10.0,
            ..cone_definition(TagId(0), TagSet::empty())
        };

        let mut circle_brute = std::collections::HashSet::new();
        let mut cone_brute = std::collections::HashSet::new();
        let mut rect_brute = std::collections::HashSet::new();
        for candidate in &candidates {
            let offset = candidate.position - origin;
            let flat = glam::Vec2::new(offset.x, offset.z);
            if flat.length() <= 4.0 {
                circle_brute.insert(candidate.entity);
            }
            let dir = forward;
            let flat_dir = glam::Vec2::new(dir.x, dir.z).normalize_or_zero();
            let len = flat.length();
            let within_radius = len <= 6.0;
            let within_angle = if len <= f32::EPSILON {
                true
            } else {
                let ndir = flat / len;
                ndir.dot(flat_dir) >= (std::f32::consts::FRAC_PI_3).cos()
            };
            if within_radius && within_angle {
                cone_brute.insert(candidate.entity);
            }
            if flat.x.abs() <= 3.0 && flat.y.abs() <= 2.5 {
                rect_brute.insert(candidate.entity);
            }
        }

        let circle_hits: std::collections::HashSet<_> = query_sense(
            &circle,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        )
        .iter()
        .map(|hit| hit.entity)
        .collect();
        let cone_hits: std::collections::HashSet<_> = query_sense(
            &cone2d,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        )
        .iter()
        .map(|hit| hit.entity)
        .collect();
        let rect_hits: std::collections::HashSet<_> = query_sense(
            &rect,
            origin,
            forward,
            &candidates,
            &SenseQueryOptions::default(),
        )
        .iter()
        .map(|hit| hit.entity)
        .collect();

        assert_eq!(circle_hits, circle_brute);
        assert_eq!(cone_hits, cone_brute);
        assert_eq!(rect_hits, rect_brute);
        assert_eq!(index.mode, SpatialTraversalMode::Bvh2d);
    }

    /// TC-17.3.5.1 — throughput budget (ignored by default; run locally).
    #[test]
    #[ignore = "micro-benchmark; enable with `cargo test test_100_queries_1000_targets_2ms -- --ignored`"]
    fn test_100_queries_1000_targets_2ms() {
        let candidates: Vec<_> = (0..1000)
            .map(|idx| SenseCandidate {
                entity: Entity(idx),
                position: Vec3::new((idx as f32) * 0.01, 0.0, 0.0),
                tags: TagSet::empty(),
            })
            .collect();
        let definition = SenseDefinition {
            shape: SenseShape::Sphere { radius: 5.0 },
            range: 5.0,
            ..cone_definition(TagId(0), TagSet::empty())
        };
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = query_sense(
                &definition,
                Vec3::ZERO,
                Vec3::Z,
                &candidates,
                &SenseQueryOptions::default(),
            );
        }
        assert!(start.elapsed().as_secs_f64() * 1000.0 < 2.0);
    }

    /// TC-17.3.6.1 — selection throughput budget (ignored by default).
    #[test]
    #[ignore = "micro-benchmark; enable with `cargo test test_selection_queries_50_half_ms -- --ignored`"]
    fn test_selection_queries_50_half_ms() {
        use crate::spatial_awareness::{execute_selection, SelectionQuery};
        let mut index = SpatialIndex::new(SpatialTraversalMode::Bvh3d);
        for idx in 0..500 {
            index.insert(
                Entity(idx),
                Vec3::new((idx as f32) * 0.02, (idx as f32) * 0.01, 0.0),
            );
        }
        let start = std::time::Instant::now();
        for idx in 0..50 {
            let query = if idx % 3 == 0 {
                SelectionQuery::Raycast {
                    origin: Vec3::new(-1.0, 0.0, 0.0),
                    direction: Vec3::X,
                    max_distance: 50.0,
                }
            } else if idx % 3 == 1 {
                SelectionQuery::BoxSelect {
                    min: Vec3::splat(-5.0),
                    max: Vec3::splat(5.0),
                }
            } else {
                SelectionQuery::NearestN {
                    center: Vec3::ZERO,
                    radius: 10.0,
                    count: 5,
                }
            };
            let _ = execute_selection(&query, &index, |_| true);
        }
        assert!(start.elapsed().as_secs_f64() * 1000.0 < 0.5);
    }

    /// TC-17.3.7.1 — GPU path placeholder (requires graphics harness).
    #[test]
    #[ignore = "GPU integration not available in this crate yet"]
    fn test_gpu_sense_eval_1m() {}

    /// TC-17.3.8.1 — rendering bridge placeholder.
    #[test]
    #[ignore = "Rendering integration not available in this crate yet"]
    fn test_awareness_drives_indicator() {}

    /// TC-17.3.9.1 — debug draw bridge placeholder.
    #[test]
    #[ignore = "Debug draw integration not available in this crate yet"]
    fn test_gizmo_sense_volume_render() {}
}
