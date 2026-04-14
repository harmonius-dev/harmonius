//! Ray intersection helpers for picking and sub-object tests.

use glam::Vec3;

use crate::types::{EntityRef, SubObjectElement};

/// A normalized ray in world space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray3 {
    /// Ray direction (unit vector).
    pub dir: Vec3,
    /// Ray origin.
    pub origin: Vec3,
}

/// Hit information for sub-object resolution along a ray.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SubObjectRayHit {
    /// Barycentric coordinates of the hit on the triangle.
    pub barycentric: Vec3,
    /// Distance along the ray to the hit.
    pub distance: f32,
    /// Triangle index within the mesh.
    pub triangle_index: u32,
}

/// Moller–Trumbore ray/triangle intersection.
///
/// `triangle_index` identifies the hit within the owning mesh (caller-defined).
pub fn ray_triangle_intersect(
    ray: &Ray3,
    triangle_index: u32,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
) -> Option<SubObjectRayHit> {
    const EPSILON: f32 = 1.0e-6;

    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = ray.dir.cross(edge2);
    let a = edge1.dot(h);

    if a > -EPSILON && a < EPSILON {
        return None;
    }

    let f = 1.0 / a;
    let s = ray.origin - v0;
    let u = f * s.dot(h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = s.cross(edge1);
    let v = f * ray.dir.dot(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * edge2.dot(q);
    if t <= EPSILON {
        return None;
    }

    let w = 1.0 - u - v;
    let barycentric = Vec3::new(w, u, v);
    Some(SubObjectRayHit {
        barycentric,
        distance: t,
        triangle_index,
    })
}

/// Picks the nearest sub-object candidate along `ray`.
pub fn nearest_subobject_along_ray(
    ray: &Ray3,
    triangles: &[(SubObjectElement, [Vec3; 3])],
) -> Option<(SubObjectElement, SubObjectRayHit)> {
    let mut best: Option<(SubObjectElement, SubObjectRayHit)> = None;
    for (triangle_index, &(element, tri)) in triangles.iter().enumerate() {
        let triangle_index = u32::try_from(triangle_index).unwrap_or(u32::MAX);
        if let Some(hit) = ray_triangle_intersect(ray, triangle_index, tri[0], tri[1], tri[2]) {
            let replace = match best {
                None => true,
                Some((_, prev)) => hit.distance < prev.distance,
            };
            if replace {
                best = Some((element, hit));
            }
        }
    }
    best
}

/// Simplified CPU raycast against analytic spheres (stand-in for BVH traversal tests).
pub fn raycast_spheres(ray: &Ray3, spheres: &[(EntityRef, Vec3, f32)]) -> Option<(EntityRef, f32)> {
    let mut best: Option<(EntityRef, f32)> = None;
    for &(entity, center, radius) in spheres {
        let oc = ray.origin - center;
        let b = oc.dot(ray.dir);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - c;
        if discriminant < 0.0 {
            continue;
        }
        let sqrt_d = discriminant.sqrt();
        let t0 = -b - sqrt_d;
        let t1 = -b + sqrt_d;
        let t = if t0 > 1.0e-4 {
            t0
        } else if t1 > 1.0e-4 {
            t1
        } else {
            continue;
        };
        let replace = match best {
            None => true,
            Some((_, best_t)) => t < best_t,
        };
        if replace {
            best = Some((entity, t));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;
    use crate::types::{EntityRef, SubObjectElement, SubObjectKind};

    fn tri_on_xy_plane() -> [Vec3; 3] {
        [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ]
    }

    #[test]
    fn test_nearest_subobject_pre_tagged_vertex_candidate() {
        let ray = Ray3 {
            dir: Vec3::new(0.0, 0.0, -1.0),
            origin: Vec3::new(0.0, 0.0, 1.0),
        };
        let element = SubObjectElement {
            entity: EntityRef(1),
            index: 7,
            kind: SubObjectKind::Vertex,
        };
        let hit = nearest_subobject_along_ray(&ray, &[(element, tri_on_xy_plane())]).expect("hit");
        assert_eq!(hit.0.kind, SubObjectKind::Vertex);
    }

    #[test]
    fn test_nearest_subobject_pre_tagged_edge_candidate() {
        let ray = Ray3 {
            dir: Vec3::new(0.0, 0.0, -1.0),
            origin: Vec3::new(0.5, 0.0, 1.0),
        };
        let element = SubObjectElement {
            entity: EntityRef(2),
            index: 3,
            kind: SubObjectKind::Edge,
        };
        let hit = nearest_subobject_along_ray(&ray, &[(element, tri_on_xy_plane())]).expect("hit");
        assert_eq!(hit.0.kind, SubObjectKind::Edge);
    }

    #[test]
    fn test_nearest_subobject_pre_tagged_face_candidate() {
        let ray = Ray3 {
            dir: Vec3::new(0.0, 0.0, -1.0),
            origin: Vec3::new(0.2, 0.2, 1.0),
        };
        let element = SubObjectElement {
            entity: EntityRef(3),
            index: 9,
            kind: SubObjectKind::Face,
        };
        let hit = nearest_subobject_along_ray(&ray, &[(element, tri_on_xy_plane())]).expect("hit");
        assert_eq!(hit.0.kind, SubObjectKind::Face);
    }

    #[test]
    fn test_subobject_nearest_element() {
        let ray = Ray3 {
            dir: Vec3::new(0.0, 0.0, -1.0),
            origin: Vec3::new(0.1, 0.1, 2.0),
        };
        let near = SubObjectElement {
            entity: EntityRef(1),
            index: 1,
            kind: SubObjectKind::Face,
        };
        let far = SubObjectElement {
            entity: EntityRef(1),
            index: 2,
            kind: SubObjectKind::Face,
        };
        let near_tri = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];
        let far_tri = [
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(1.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, -1.0),
        ];
        let hit =
            nearest_subobject_along_ray(&ray, &[(far, far_tri), (near, near_tri)]).expect("hit");
        assert_eq!(hit.0.index, 1);
        assert_eq!(hit.1.triangle_index, 1);
    }

    #[test]
    fn test_ray_triangle_intersect_reports_triangle_index() {
        let ray = Ray3 {
            dir: Vec3::new(0.0, 0.0, -1.0),
            origin: Vec3::new(0.25, 0.25, 1.0),
        };
        let tri = tri_on_xy_plane();
        let hit = ray_triangle_intersect(&ray, 42, tri[0], tri[1], tri[2]).expect("hit");
        assert_eq!(hit.triangle_index, 42);
    }

    #[test]
    fn test_cpu_raycast_with_scene_bvh() {
        let ray = Ray3 {
            dir: Vec3::new(0.0, 0.0, -1.0),
            origin: Vec3::new(0.0, 0.0, 5.0),
        };
        let spheres = [
            (EntityRef(1), Vec3::new(0.0, 0.0, 2.0), 0.5),
            (EntityRef(2), Vec3::new(0.0, 0.0, -5.0), 1.0),
        ];
        let hit = raycast_spheres(&ray, &spheres).expect("hit");
        assert_eq!(hit.0, EntityRef(1));
    }
}
