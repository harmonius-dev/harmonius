//! IR-2.5.1 walkability queries against a shared spatial index + material policy.

use glam::Vec3;

use crate::geometry::Plane;
use crate::metrics::FallbackMetrics;
use crate::types::{AgentMask, Entity, MaterialId};

/// Walkability query issued by AI navigation (CH-26 transport in full runtime).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WalkabilityQuery {
    pub request_id: u64,
    pub foot_position: Vec3,
    pub max_slope_deg: f32,
    pub agent_mask: AgentMask,
}

/// Reason a surface is rejected for traversal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnwalkableReason {
    /// Surface is walkable; `walkable == true`.
    None,
    /// No supporting geometry within the query volume / BVH not ready (FM-1).
    NoSurface,
    /// Ground normal exceeds `max_slope_deg` from vertical.
    SlopeTooSteep,
    /// Material disallowed for this `AgentMask`.
    MaterialExcluded,
    /// Reserved for entity filter exclusions (not exercised in the reference harness).
    EntityExcluded,
}

/// Reply payload consumed by AI navigation after physics resolves materials.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WalkabilityResult {
    pub request_id: u64,
    pub walkable: bool,
    pub surface_normal: Vec3,
    pub material: MaterialId,
    pub reason: UnwalkableReason,
}

/// Bounded planar patch used for walkability tests (finite BVH leaf surrogate).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarPatch {
    /// Surface plane equation.
    pub plane: Plane,
    /// Anchor used for finite XZ footprint tests.
    pub center: Vec3,
    /// Half-extent on the XZ plane measured from `center` (Manhattan max norm).
    pub xz_extent: f32,
    /// Resolved material for this patch.
    pub material: MaterialId,
    /// Owning entity for exclusions / telemetry.
    pub entity: Entity,
}

/// Deterministic stand-in for the runtime shared BVH + material resolver.
#[derive(Clone, Debug, Default)]
pub struct WalkabilityScene {
    pub bvh_ready: bool,
    pub surfaces: Vec<PlanarPatch>,
}

impl WalkabilityScene {
    /// Resolves `query` by casting downward along `-Y` from the foot position.
    #[must_use]
    pub fn resolve(
        &self,
        query: WalkabilityQuery,
        metrics: &mut FallbackMetrics,
    ) -> WalkabilityResult {
        if !self.bvh_ready {
            metrics.fm1_bvh_missing += 1;
            return WalkabilityResult {
                request_id: query.request_id,
                walkable: false,
                surface_normal: Vec3::ZERO,
                material: MaterialId(0),
                reason: UnwalkableReason::NoSurface,
            };
        }

        let ray_origin = query.foot_position;
        let ray_dir = Vec3::new(0.0, -1.0, 0.0);
        let mut best_t = f32::MAX;
        let mut best: Option<(Vec3, Vec3, MaterialId)> = None;

        for surface in &self.surfaces {
            if let Some((t, hit, normal, material)) =
                intersect_planar_patch(ray_origin, ray_dir, surface)
            {
                if t < best_t {
                    best_t = t;
                    best = Some((hit, normal, material));
                }
            }
        }

        let Some((_, normal, material)) = best else {
            return WalkabilityResult {
                request_id: query.request_id,
                walkable: false,
                surface_normal: Vec3::ZERO,
                material: MaterialId(0),
                reason: UnwalkableReason::NoSurface,
            };
        };

        if !query.agent_mask.allows(material) {
            return WalkabilityResult {
                request_id: query.request_id,
                walkable: false,
                surface_normal: normal,
                material,
                reason: UnwalkableReason::MaterialExcluded,
            };
        }

        let up = Vec3::Y;
        let n = normal.normalize_or_zero();
        let slope_rad = n.dot(up).clamp(-1.0, 1.0).acos();
        let max_slope_rad = query.max_slope_deg.to_radians();
        if slope_rad > max_slope_rad {
            return WalkabilityResult {
                request_id: query.request_id,
                walkable: false,
                surface_normal: n,
                material,
                reason: UnwalkableReason::SlopeTooSteep,
            };
        }

        WalkabilityResult {
            request_id: query.request_id,
            walkable: true,
            surface_normal: n,
            material,
            reason: UnwalkableReason::None,
        }
    }
}

fn intersect_planar_patch(
    origin: Vec3,
    dir: Vec3,
    patch: &PlanarPatch,
) -> Option<(f32, Vec3, Vec3, MaterialId)> {
    let t = patch.plane.ray_hit_t(origin, dir, f32::MAX)?;
    let hit = origin + dir * t;
    let dx = (hit.x - patch.center.x).abs();
    let dz = (hit.z - patch.center.z).abs();
    if dx > patch.xz_extent || dz > patch.xz_extent {
        return None;
    }
    Some((
        t,
        hit,
        patch.plane.normal.normalize_or_zero(),
        patch.material,
    ))
}
