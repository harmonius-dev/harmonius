//! IR-2.5.2 jump-arc prediction via segmented raycasts against a **linear AABB list** (stand-in
//! for the physics-private BVH used at runtime).

use glam::Vec3;

use crate::geometry::AxisAlignedBox;
use crate::metrics::FallbackMetrics;
use crate::types::Entity;

/// Jump arc query issued before committing to an airborne motion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JumpArcQuery {
    pub start: Vec3,
    pub initial_velocity: Vec3,
    pub gravity: Vec3,
    pub segment_count: u32,
    pub segment_dt: f32,
}

/// Result of tracing the jump arc through blocking geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JumpArcResult {
    pub landing: Option<Vec3>,
    /// `0` when the trace completes without a blocker hit (including successful ground landings).
    /// When a segment hits geometry, holds **`segment_index + 1`** where `segment_index` is the
    /// zero-based segment counter in the trace loop (first segment is `1` when blocked on seg 0).
    pub blocked_segment: u32,
    pub blocker: Entity,
}

/// Minimal scene geometry used by the reference jump-arc harness.
#[derive(Clone, Debug, Default)]
pub struct JumpArcScene {
    pub blockers: Vec<(Entity, AxisAlignedBox)>,
    /// Infinite horizontal ground plane at this `y` height (typically `0.0`).
    pub ground_y: Option<f32>,
}

/// Counts physics-private raycasts for negative tests.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JumpArcTraceStats {
    pub segment_raycasts: u64,
}

impl JumpArcScene {
    /// Traces `query` through `self`, discarding per-segment statistics.
    #[must_use]
    pub fn trace(&self, query: JumpArcQuery) -> JumpArcResult {
        let mut stats = JumpArcTraceStats::default();
        let mut metrics = FallbackMetrics::default();
        self.trace_instrumented(query, &mut stats, &mut metrics)
    }

    /// Like [`Self::trace`], but increments `stats.segment_raycasts` once per segment evaluated.
    #[must_use]
    pub fn trace_instrumented(
        &self,
        query: JumpArcQuery,
        stats: &mut JumpArcTraceStats,
        metrics: &mut FallbackMetrics,
    ) -> JumpArcResult {
        if query.segment_count == 0 {
            return JumpArcResult {
                landing: None,
                blocked_segment: 0,
                blocker: Entity::NONE,
            };
        }

        let mut t = 0.0_f32;
        for seg in 0..query.segment_count {
            let t0 = t;
            t += query.segment_dt;
            let p0 = parabola_point(query.start, query.initial_velocity, query.gravity, t0);
            let p1 = parabola_point(query.start, query.initial_velocity, query.gravity, t);
            let delta = p1 - p0;

            if delta.length_squared() <= f32::EPSILON {
                metrics.fm2_physics_private_raycast_failed += 1;
                continue;
            }

            stats.segment_raycasts += 1;

            for (entity, aabb) in &self.blockers {
                if let Some(hit_t) = aabb.ray_segment_hit_t(p0, delta) {
                    if (0.0..=1.0).contains(&hit_t) {
                        return JumpArcResult {
                            landing: None,
                            blocked_segment: seg + 1,
                            blocker: *entity,
                        };
                    }
                }
            }
        }

        let t_end = query.segment_dt * query.segment_count as f32;
        if let Some(gy) = self.ground_y {
            if let Some(t_hit) =
                ground_hit_time(query.start, query.initial_velocity, query.gravity, gy)
            {
                if t_hit > 0.0 && t_hit <= t_end {
                    let landing =
                        parabola_point(query.start, query.initial_velocity, query.gravity, t_hit);
                    return JumpArcResult {
                        landing: Some(landing),
                        blocked_segment: 0,
                        blocker: Entity::NONE,
                    };
                }
            }
        }

        let end = parabola_point(query.start, query.initial_velocity, query.gravity, t_end);

        JumpArcResult {
            landing: Some(end),
            blocked_segment: 0,
            blocker: Entity::NONE,
        }
    }
}

/// Samples the ballistic parabola used by jump-arc prediction.
#[must_use]
pub fn parabola_point(start: Vec3, v0: Vec3, gravity: Vec3, t: f32) -> Vec3 {
    start + v0 * t + 0.5 * gravity * t * t
}

#[must_use]
fn ground_hit_time(start: Vec3, v0: Vec3, g: Vec3, ground_y: f32) -> Option<f32> {
    let a = 0.5 * g.y;
    let b = v0.y;
    let c = start.y - ground_y;
    solve_smallest_positive_quadratic(a, b, c)
}

#[must_use]
fn solve_smallest_positive_quadratic(a: f32, b: f32, c: f32) -> Option<f32> {
    if a.abs() < f32::EPSILON {
        if b.abs() < f32::EPSILON {
            return None;
        }
        let t = -c / b;
        return (t > 0.0).then_some(t);
    }

    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }
    let sqrt_disc = disc.sqrt();
    let t0 = (-b - sqrt_disc) / (2.0 * a);
    let t1 = (-b + sqrt_disc) / (2.0 * a);
    let mut best: Option<f32> = None;
    for t in [t0, t1] {
        if t > 0.0 {
            best = Some(match best {
                None => t,
                Some(cur) => cur.min(t),
            });
        }
    }
    best
}
