//! Analytical intersection helpers (pure, deterministic).

use glam::{Quat, Vec3};

use super::shape::{ColliderShape, ColliderTransform};

const EPS: f32 = 1e-5;

fn closest_point_on_segment(a: Vec3, b: Vec3, p: Vec3) -> Vec3 {
    let ab = b - a;
    let t = if ab.length_squared() < EPS * EPS {
        0.0
    } else {
        ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0)
    };
    a + ab * t
}

/// Ray vs unit sphere at origin, returns t of entry if hit forward ray.
pub fn ray_sphere(origin: Vec3, dir: Vec3, radius: f32) -> Option<(f32, Vec3, Vec3)> {
    let b = origin.dot(dir);
    let c = origin.length_squared() - radius * radius;
    let disc = b * b - c;
    if disc < 0.0 {
        return None;
    }
    let sqrt_d = disc.sqrt();
    let mut t0 = -b - sqrt_d;
    let mut t1 = -b + sqrt_d;
    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }
    let t = if t0 >= 0.0 {
        t0
    } else if t1 >= 0.0 {
        t1
    } else {
        return None;
    };
    let p = origin + dir * t;
    let n = (p / radius).normalize_or_zero();
    Some((t, p, n))
}

/// Ray vs axis-aligned box centered at origin with half extents `he`.
pub fn ray_box(origin: Vec3, dir: Vec3, he: Vec3) -> Option<(f32, Vec3, Vec3)> {
    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;
    let mut n_min = Vec3::X;
    for i in 0..3 {
        let o = origin[i];
        let d = dir[i];
        let h = he[i];
        if d.abs() < EPS {
            if o.abs() > h {
                return None;
            }
            continue;
        }
        let inv = 1.0 / d;
        let mut t0 = (-h - o) * inv;
        let mut t1 = (h - o) * inv;
        let mut n0 = Vec3::ZERO;
        n0[i] = -1.0;
        let mut n1 = Vec3::ZERO;
        n1[i] = 1.0;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
            std::mem::swap(&mut n0, &mut n1);
        }
        if t0 > t_min {
            t_min = t0;
            n_min = n0;
        }
        t_max = t_max.min(t1);
        if t_min > t_max {
            return None;
        }
    }
    if t_max < 0.0 {
        return None;
    }
    let t = if t_min >= 0.0 {
        t_min
    } else {
        t_max
    };
    if t < 0.0 {
        return None;
    }
    let p = origin + dir * t;
    Some((t, p, n_min.normalize_or_zero()))
}

/// Ray vs triangle (Möller–Trumbore).
pub fn ray_triangle(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, Vec3, Vec3)> {
    let e1 = b - a;
    let e2 = c - a;
    let p = dir.cross(e2);
    let det = e1.dot(p);
    if det.abs() < EPS {
        return None;
    }
    let inv_det = 1.0 / det;
    let tvec = origin - a;
    let u = tvec.dot(p) * inv_det;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = tvec.cross(e1);
    let v = dir.dot(q) * inv_det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = e2.dot(q) * inv_det;
    if t < 0.0 {
        return None;
    }
    let hit = origin + dir * t;
    let n = e1.cross(e2).normalize_or_zero();
    Some((t, hit, n))
}

/// Ray vs capsule (Y-up segment) in local space; transform world ray to local.
pub fn ray_capsule_local(
    origin: Vec3,
    dir: Vec3,
    half_height: f32,
    radius: f32,
) -> Option<(f32, Vec3, Vec3)> {
    let a = Vec3::new(0.0, -half_height, 0.0);
    let b = Vec3::new(0.0, half_height, 0.0);
    let m = origin - a;
    let d = b - a;
    let dd = d.length_squared();
    let nd = dir.dot(d);
    let nm = m.dot(d);
    let a_coef = dir.length_squared() * dd - nd * nd;
    let b_coef = 2.0 * (dir.dot(m) * dd - nd * nm);
    let c_coef = m.length_squared() * dd - nm * nm - radius * radius * dd;
    if a_coef.abs() < EPS {
        return None;
    }
    let disc = b_coef * b_coef - 4.0 * a_coef * c_coef;
    if disc < 0.0 {
        return None;
    }
    let sqrt_d = disc.sqrt();
    let mut t0 = (-b_coef - sqrt_d) / (2.0 * a_coef);
    let mut t1 = (-b_coef + sqrt_d) / (2.0 * a_coef);
    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }
    let pick = |t: f32| -> Option<(f32, Vec3, Vec3)> {
        if t < 0.0 {
            return None;
        }
        let p = origin + dir * t;
        let c = closest_point_on_segment(a, b, p);
        let n = (p - c).normalize_or_zero();
        Some((t, p, n))
    };
    pick(t0).or_else(|| pick(t1))
}

/// World-space ray vs collider.
pub fn ray_collider(
    origin: Vec3,
    dir: Vec3,
    shape: &ColliderShape,
    xf: ColliderTransform,
) -> Option<(f32, Vec3, Vec3)> {
    let lo = xf.world_to_local(origin);
    let ld = xf.world_to_local_dir(dir).normalize_or_zero();
    let local_hit = match shape {
        ColliderShape::Sphere { radius } => ray_sphere(lo, ld, *radius),
        ColliderShape::Box { half_extents } => ray_box(lo, ld, *half_extents),
        ColliderShape::Capsule {
            half_height,
            radius,
        } => ray_capsule_local(lo, ld, *half_height, *radius),
        ColliderShape::ConvexHull { points } => {
            if points.is_empty() {
                None
            } else {
                let mut mn = points[0];
                let mut mx = points[0];
                for p in points.iter().skip(1) {
                    mn = mn.min(*p);
                    mx = mx.max(*p);
                }
                let he = (mx - mn) * 0.5;
                let center = (mx + mn) * 0.5;
                ray_box(lo - center, ld, he)
            }
        }
        ColliderShape::TriangleMesh { triangles } => {
            let mut best: Option<(f32, Vec3, Vec3)> = None;
            for tri in triangles {
                if let Some((t, hit, n)) = ray_triangle(lo, ld, tri[0], tri[1], tri[2]) {
                    if best.is_none_or(|(bt, _, _)| t < bt) {
                        best = Some((t, hit, n));
                    }
                }
            }
            best
        }
        ColliderShape::Heightfield {
            cell,
            width,
            depth,
            heights,
            origin,
        } => ray_heightfield(lo, ld, *cell, *width, *depth, heights, *origin),
    };
    let (t, hit_local, n_local) = local_hit?;
    let hit_world = xf.to_world(hit_local);
    let n_world = (xf.rotation * n_local).normalize_or_zero();
    Some((t, hit_world, n_world))
}

fn ray_heightfield(
    origin: Vec3,
    dir: Vec3,
    cell: f32,
    width: usize,
    depth: usize,
    heights: &[f32],
    grid_origin: Vec3,
) -> Option<(f32, Vec3, Vec3)> {
    if width == 0 || depth == 0 || heights.len() != width * depth {
        return None;
    }
    let mut best: Option<(f32, Vec3, Vec3)> = None;
    for ix in 0..width.saturating_sub(1) {
        for iz in 0..depth.saturating_sub(1) {
            let idx = |x: usize, z: usize| heights[x + z * width];
            let x0 = grid_origin.x + ix as f32 * cell;
            let z0 = grid_origin.z + iz as f32 * cell;
            let x1 = x0 + cell;
            let z1 = z0 + cell;
            let h00 = idx(ix, iz);
            let h10 = idx(ix + 1, iz);
            let h01 = idx(ix, iz + 1);
            let h11 = idx(ix + 1, iz + 1);
            let y00 = grid_origin.y + h00;
            let y10 = grid_origin.y + h10;
            let y01 = grid_origin.y + h01;
            let y11 = grid_origin.y + h11;
            let v00 = Vec3::new(x0, y00, z0);
            let v10 = Vec3::new(x1, y10, z0);
            let v01 = Vec3::new(x0, y01, z1);
            let v11 = Vec3::new(x1, y11, z1);
            for tri in [(v00, v10, v01), (v10, v11, v01)] {
                if let Some((t, hit, n)) = ray_triangle(origin, dir, tri.0, tri.1, tri.2) {
                    if best.is_none_or(|(bt, _, _)| t < bt) {
                        best = Some((t, hit, n));
                    }
                }
            }
        }
    }
    best
}

/// Signed distance from point to sphere surface (negative inside).
pub fn signed_distance_sphere(point: Vec3, center: Vec3, radius: f32) -> f32 {
    point.distance(center) - radius
}

/// Closest point on OBB (box) to a point in world space.
pub fn closest_point_box(point: Vec3, he: Vec3, xf: ColliderTransform) -> (Vec3, Vec3, f32) {
    let local = xf.world_to_local(point);
    let clamped = local.clamp(-he, he);
    let d = local - clamped;
    let dist = d.length();
    if dist < EPS {
        let pen = he - local.abs();
        let axis = if pen.x <= pen.y && pen.x <= pen.z {
            Vec3::X * local.x.signum()
        } else if pen.y <= pen.z {
            Vec3::Y * local.y.signum()
        } else {
            Vec3::Z * local.z.signum()
        };
        let closest = clamped - axis * pen.min_element().abs();
        let n = (xf.rotation * axis).normalize_or_zero();
        let sd = -pen.min_element();
        return (xf.to_world(closest), n, sd);
    }
    let n_local = d / dist;
    let closest_world = xf.to_world(clamped);
    let n_world = (xf.rotation * n_local).normalize();
    (closest_world, n_world, dist)
}

/// Closest point on capsule (Y aligned) to local point.
pub fn closest_point_capsule_local(point: Vec3, half_height: f32, radius: f32) -> (Vec3, Vec3, f32) {
    let a = Vec3::new(0.0, -half_height, 0.0);
    let b = Vec3::new(0.0, half_height, 0.0);
    let c = closest_point_on_segment(a, b, point);
    let v = point - c;
    let dist_axis = v.length();
    if dist_axis < EPS {
        let n = Vec3::X;
        return (c + n * radius, n, radius);
    }
    let n = v / dist_axis;
    let closest = c + n * radius;
    let sd = dist_axis - radius;
    (closest, n, sd)
}

/// Point inside OBB test in world space.
#[allow(dead_code)]
pub fn point_in_box_world(point: Vec3, he: Vec3, xf: ColliderTransform) -> bool {
    let local = xf.world_to_local(point);
    local.abs().cmple(he).all()
}

/// Sphere vs OBB overlap (Minkowski expand box by radius).
pub fn sphere_box_overlap(center: Vec3, radius: f32, he: Vec3, xf: ColliderTransform) -> bool {
    let local = xf.world_to_local(center);
    let q = local.abs() - he;
    let u = q.max(Vec3::ZERO).length();
    let v = q.max_element().min(0.0);
    u + v <= radius
}

/// Conservative AABB overlap for convex hull point sets in local space of `a`.
pub fn convex_hulls_overlap_conservative(
    a: &[Vec3],
    b: &[Vec3],
    rel_rot: Quat,
    rel_pos: Vec3,
) -> bool {
    let mut a_min = Vec3::splat(f32::INFINITY);
    let mut a_max = Vec3::splat(f32::NEG_INFINITY);
    for p in a {
        a_min = a_min.min(*p);
        a_max = a_max.max(*p);
    }
    let mut b_min = Vec3::splat(f32::INFINITY);
    let mut b_max = Vec3::splat(f32::NEG_INFINITY);
    for p in b {
        let w = rel_rot * *p + rel_pos;
        b_min = b_min.min(w);
        b_max = b_max.max(w);
    }
    a_min.cmple(b_max).all() && b_min.cmple(a_max).all()
}
