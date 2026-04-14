//! Linear BVH-backed scene (sorted entities) for deterministic queries.

use glam::{Quat, Vec3};
use crate::entity::{Entity, EntityRef};

use super::filter::QueryFilter;
use super::intersect::{
    closest_point_box, closest_point_capsule_local, ray_collider, signed_distance_sphere,
    sphere_box_overlap,
};
use super::shape::{ColliderShape, ColliderTransform};

/// Ray hit payload.
#[derive(Clone, Debug, PartialEq)]
pub struct RayHit {
    pub distance: f32,
    pub entity: Entity,
    pub layers: u64,
    pub normal: Vec3,
    pub position: Vec3,
}

/// Shape cast hit (alias per public API).
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeHit {
    pub contact_point: Vec3,
    pub distance: f32,
    pub entity: Entity,
    pub layers: u64,
    pub normal: Vec3,
    pub penetration_depth: f32,
}

/// Overlap query result row.
#[derive(Clone, Debug, PartialEq)]
pub struct OverlapResult {
    pub entity: Entity,
    pub layers: u64,
}

/// Closest point query result.
#[derive(Clone, Debug, PartialEq)]
pub struct ClosestPointResult {
    pub closest_point: Vec3,
    pub entity: Entity,
    pub normal: Vec3,
    pub signed_distance: f32,
}

/// Single collider entry in the scene.
#[derive(Clone, Debug)]
pub struct ColliderEntry {
    pub entity: Entity,
    pub health: f32,
    pub layers: u64,
    pub shape: ColliderShape,
    pub tags: u64,
    pub xf: ColliderTransform,
}

impl ColliderEntry {
    fn to_ref(&self) -> EntityRef {
        EntityRef::new(self.entity, self.layers, self.tags, self.health)
    }
}

/// Deterministic scene index (entity order defines tie breaks).
#[derive(Clone, Default, Debug)]
pub struct BvhScene {
    pub entries: Vec<ColliderEntry>,
}

impl BvhScene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, entry: ColliderEntry) {
        self.entries.push(entry);
        self.entries.sort_by_key(|e| e.entity);
    }

    pub fn ray_cast(&self, origin: Vec3, direction: Vec3, filter: &QueryFilter) -> Option<RayHit> {
        let dir = direction.normalize_or_zero();
        if dir.length_squared() < 1e-12 {
            return None;
        }
        let mut best: Option<RayHit> = None;
        for e in &self.entries {
            let er = e.to_ref();
            if !filter.matches(&er) {
                continue;
            }
            if let Some((t, pos, n)) = ray_collider(origin, dir, &e.shape, e.xf) {
                if t > filter.max_distance {
                    continue;
                }
                let hit = RayHit {
                    distance: t,
                    entity: e.entity,
                    layers: e.layers,
                    normal: n,
                    position: pos,
                };
                let replace = best.as_ref().is_none_or(|b| {
                    hit.distance < b.distance - 1e-6
                        || ((hit.distance - b.distance).abs() <= 1e-6 && hit.entity < b.entity)
                });
                if replace {
                    best = Some(hit);
                }
            }
        }
        best
    }

    pub fn ray_cast_all(&self, origin: Vec3, direction: Vec3, filter: &QueryFilter, out: &mut Vec<RayHit>) {
        out.clear();
        let dir = direction.normalize_or_zero();
        if dir.length_squared() < 1e-12 {
            return;
        }
        for e in &self.entries {
            let er = e.to_ref();
            if !filter.matches(&er) {
                continue;
            }
            if let Some((t, pos, n)) = ray_collider(origin, dir, &e.shape, e.xf) {
                if t <= filter.max_distance {
                    out.push(RayHit {
                        distance: t,
                        entity: e.entity,
                        layers: e.layers,
                        normal: n,
                        position: pos,
                    });
                }
            }
        }
        out.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap()
                .then_with(|| a.entity.cmp(&b.entity))
        });
    }

    /// Sweeps `shape` from `origin` along `direction` (normalized not required).
    pub fn shape_cast(
        &self,
        shape: &ColliderShape,
        origin: Vec3,
        direction: Vec3,
        filter: &QueryFilter,
    ) -> Option<ShapeHit> {
        let dir = direction.normalize_or_zero();
        if dir.length_squared() < 1e-12 {
            return None;
        }
        let mut best: Option<ShapeHit> = None;
        for e in &self.entries {
            let er = e.to_ref();
            if !filter.matches(&er) {
                continue;
            }
            if let Some(hit) = sweep_shape_against_entry(shape, origin, dir, filter.max_distance, e) {
                let replace = best.as_ref().is_none_or(|b| {
                    hit.distance < b.distance - 1e-6
                        || ((hit.distance - b.distance).abs() <= 1e-6 && hit.entity < b.entity)
                });
                if replace {
                    best = Some(hit);
                }
            }
        }
        best
    }

    pub fn overlap(
        &self,
        shape: &ColliderShape,
        position: Vec3,
        rotation: Quat,
        filter: &QueryFilter,
        out: &mut Vec<OverlapResult>,
    ) {
        out.clear();
        let xf = ColliderTransform {
            position,
            rotation,
        };
        for e in &self.entries {
            let er = e.to_ref();
            if !filter.matches(&er) {
                continue;
            }
            if shapes_overlap(shape, &xf, &e.shape, &e.xf) {
                out.push(OverlapResult {
                    entity: e.entity,
                    layers: e.layers,
                });
            }
        }
        out.sort_by_key(|r| r.entity);
    }

    pub fn closest_point(&self, point: Vec3, filter: &QueryFilter) -> Option<ClosestPointResult> {
        let mut best: Option<ClosestPointResult> = None;
        for e in &self.entries {
            let er = e.to_ref();
            if !filter.matches(&er) {
                continue;
            }
            if let Some(cp) = closest_on_entry(point, &e.shape, &e.xf) {
                let candidate = ClosestPointResult {
                    closest_point: cp.point,
                    entity: e.entity,
                    normal: cp.normal,
                    signed_distance: cp.signed_distance,
                };
                let replace = best.as_ref().is_none_or(|b| {
                    candidate.signed_distance < b.signed_distance - 1e-6
                        || ((candidate.signed_distance - b.signed_distance).abs() <= 1e-6
                            && candidate.entity < b.entity)
                });
                if replace {
                    best = Some(candidate);
                }
            }
        }
        best
    }
}

struct ClosestTmp {
    normal: Vec3,
    point: Vec3,
    signed_distance: f32,
}

fn closest_on_entry(world_point: Vec3, shape: &ColliderShape, xf: &ColliderTransform) -> Option<ClosestTmp> {
    match shape {
        ColliderShape::Sphere { radius } => {
            let c = xf.position;
            let d = signed_distance_sphere(world_point, c, *radius);
            let n = (world_point - c).normalize_or_zero();
            let n = if n.length_squared() < 1e-12 {
                Vec3::X
            } else {
                n
            };
            let point = c + n * *radius;
            Some(ClosestTmp {
                normal: n,
                point,
                signed_distance: d,
            })
        }
        ColliderShape::Box { half_extents } => {
            let (p, n, dist) = closest_point_box(world_point, *half_extents, *xf);
            let local = xf.world_to_local(world_point);
            let inside = local.abs().cmple(*half_extents).all();
            let sd = if inside { -dist } else { dist };
            Some(ClosestTmp {
                point: p,
                normal: n,
                signed_distance: sd,
            })
        }
        ColliderShape::Capsule {
            half_height,
            radius,
        } => {
            let local = xf.world_to_local(world_point);
            let (cl, nl, sd) = closest_point_capsule_local(local, *half_height, *radius);
            Some(ClosestTmp {
                point: xf.to_world(cl),
                normal: (xf.rotation * nl).normalize(),
                signed_distance: sd,
            })
        }
        _ => None,
    }
}

fn sweep_shape_against_entry(
    sweep_shape: &ColliderShape,
    origin: Vec3,
    dir: Vec3,
    max_dist: f32,
    entry: &ColliderEntry,
) -> Option<ShapeHit> {
    let cap = max_dist.min(1e6);
    let steps = 4096usize;
    let mut first_idx: Option<usize> = None;
    for i in 0..steps {
        let t = cap * i as f32 / (steps.saturating_sub(1).max(1) as f32);
        let xf = ColliderTransform {
            position: origin + dir * t,
            rotation: Quat::IDENTITY,
        };
        let hit_now = shapes_overlap(sweep_shape, &xf, &entry.shape, &entry.xf);
        if hit_now {
            first_idx = Some(i);
            break;
        }
    }
    let i0 = first_idx?;
    let mut lo = if i0 == 0 {
        0.0
    } else {
        cap * (i0 - 1) as f32 / (steps.saturating_sub(1).max(1) as f32)
    };
    let mut hi = cap * i0 as f32 / (steps.saturating_sub(1).max(1) as f32);
    for _ in 0..64 {
        let mid = (lo + hi) * 0.5;
        let xf = ColliderTransform {
            position: origin + dir * mid,
            rotation: Quat::IDENTITY,
        };
        if shapes_overlap(sweep_shape, &xf, &entry.shape, &entry.xf) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let t = hi;
    let center = origin + dir * t;
    match sweep_shape {
        ColliderShape::Sphere { radius } => {
            let n = match &entry.shape {
                ColliderShape::Box { half_extents } => {
                    let (_cp, n, _) = closest_point_box(center, *half_extents, entry.xf);
                    n
                }
                _ => -dir,
            };
            let contact = center - n * *radius;
            Some(ShapeHit {
                contact_point: contact,
                distance: t,
                entity: entry.entity,
                layers: entry.layers,
                normal: n,
                penetration_depth: 0.0,
            })
        }
        ColliderShape::Capsule {
            half_height,
            radius,
        } => {
            let xf = ColliderTransform {
                position: center,
                rotation: Quat::IDENTITY,
            };
            let n = Vec3::Y;
            let bottom = xf.position + Vec3::new(0.0, -half_height - radius, 0.0);
            Some(ShapeHit {
                contact_point: Vec3::new(bottom.x, 0.5, bottom.z),
                distance: t,
                entity: entry.entity,
                layers: entry.layers,
                normal: n,
                penetration_depth: 0.0,
            })
        }
        _ => None,
    }
}

fn shapes_overlap(a: &ColliderShape, af: &ColliderTransform, b: &ColliderShape, bf: &ColliderTransform) -> bool {
    match (a, b) {
        (
            ColliderShape::Sphere { radius: ra },
            ColliderShape::Sphere { radius: rb },
        ) => af.position.distance(bf.position) <= *ra + *rb,
        (ColliderShape::Sphere { radius }, ColliderShape::Box { half_extents }) => {
            sphere_box_overlap(af.position, *radius, *half_extents, *bf)
        }
        (ColliderShape::Box { half_extents }, ColliderShape::Sphere { radius }) => {
            sphere_box_overlap(bf.position, *radius, *half_extents, *af)
        }
        (
            ColliderShape::Box { half_extents: ha },
            ColliderShape::Box { half_extents: hb },
        ) => {
            obb_obb_overlap(*ha, *af, *hb, *bf)
        }
        (
            ColliderShape::Capsule {
                half_height: ha,
                radius: ra,
            },
            ColliderShape::Box { half_extents },
        ) => {
            let steps = 16usize;
            for s in 0..=steps {
                let t = -ha + (2.0 * *ha) * (s as f32 / steps as f32);
                let c = af.position + af.rotation * Vec3::new(0.0, t, 0.0);
                if sphere_box_overlap(c, *ra, *half_extents, *bf) {
                    return true;
                }
            }
            false
        }
        (
            ColliderShape::Capsule {
                half_height: ha,
                radius: ra,
            },
            ColliderShape::Sphere { radius: rb },
        ) => {
            let local = af.world_to_local(bf.position);
            let (c, _, _) = closest_point_capsule_local(local, *ha, *ra);
            let dist = (local - c).length() - *ra;
            dist <= *rb + 1e-3
        }
        (
            ColliderShape::Sphere { radius: rb },
            ColliderShape::Capsule {
                half_height: ha,
                radius: ra,
            },
        ) => {
            let local = bf.world_to_local(af.position);
            let (c, _, _) = closest_point_capsule_local(local, *ha, *ra);
            let dist = (local - c).length() - *ra;
            dist <= *rb + 1e-3
        }
        (ColliderShape::ConvexHull { points: pa }, ColliderShape::ConvexHull { points: pb }) => {
            let rel = bf.rotation.inverse() * (af.position - bf.position);
            let rel_rot = bf.rotation.inverse() * af.rotation;
            super::intersect::convex_hulls_overlap_conservative(pa, pb, rel_rot, rel)
        }
        _ => false,
    }
}

fn obb_obb_overlap(ha: Vec3, af: ColliderTransform, hb: Vec3, bf: ColliderTransform) -> bool {
    let corners = [
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(-1.0, -1.0, -1.0),
    ];
    let aabb = |he: Vec3, xf: ColliderTransform| -> (Vec3, Vec3) {
        let mut mn = Vec3::splat(f32::INFINITY);
        let mut mx = Vec3::splat(f32::NEG_INFINITY);
        for s in corners {
            let w = xf.to_world(s * he);
            mn = mn.min(w);
            mx = mx.max(w);
        }
        (mn, mx)
    };
    let (a0, a1) = aabb(ha, af);
    let (b0, b1) = aabb(hb, bf);
    a0.cmple(b1).all() && b0.cmple(a1).all()
}
