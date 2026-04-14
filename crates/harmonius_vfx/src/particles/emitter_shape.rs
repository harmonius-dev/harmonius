//! Deterministic CPU-side spawn positions for [`EmitterShape`].
//!
//! Maps to test cases `TC-11.1.1.1`–`TC-11.1.1.5` in `docs/design/vfx/particles-test-cases.md`.

use glam::Vec3;
use std::f32::consts::PI;

/// Emitter volume or surface used for initial particle placement.
#[derive(Clone, Debug, PartialEq)]
pub enum EmitterShape {
    /// All particles spawn at the emitter origin (`TC-11.1.1.1`).
    Point,
    /// Uniform-ish samples on a sphere shell (`TC-11.1.1.2`).
    SphereSurface {
        /// Distance from the origin to each spawn point.
        radius: f32,
    },
    /// Axis-aligned box volume (`TC-11.1.1.3`).
    BoxVolume {
        /// Half-extents along each world axis.
        half_extents: Vec3,
    },
    /// Directions from the apex within a cone around +Y (`TC-11.1.1.4`).
    Cone {
        /// Full cone opening angle in radians (from one side to the other).
        opening_angle_radians: f32,
        /// Maximum distance along the cone axis from the origin.
        length: f32,
    },
    /// Samples taken from a triangle mesh surface (`TC-11.1.1.5`).
    MeshSurface {
        /// Triangle corners in emitter-local space (three vertices per triangle).
        triangles: Vec<[Vec3; 3]>,
    },
    /// Triangle mesh that changes per animation frame (`TC-11.1.1.6`).
    SkinnedMeshSurface {
        /// One triangle list per pose frame, in emitter-local space.
        frames: Vec<Vec<[Vec3; 3]>>,
        /// Frame index used for sampling.
        active_frame: usize,
    },
}

/// Golden ratio helper for stable low-discrepancy sequences.
const PHI: f32 = 1.618_034;

/// Returns `count` spawn positions in emitter-local space for `shape`.
///
/// The mapping is deterministic for equal inputs so simulations stay replayable.
#[must_use]
pub fn spawn_positions(shape: &EmitterShape, origin: Vec3, count: usize) -> Vec<Vec3> {
    match shape {
        EmitterShape::Point => vec![origin; count],
        EmitterShape::SphereSurface { radius } => (0..count)
            .map(|i| origin + fibonacci_sphere(i, count) * *radius)
            .collect(),
        EmitterShape::BoxVolume { half_extents } => (0..count)
            .map(|i| origin + stratified_box(i, count, *half_extents))
            .collect(),
        EmitterShape::Cone {
            opening_angle_radians,
            length,
        } => {
            let half_angle = (*opening_angle_radians * 0.5).min(PI * 0.99);
            let cos_limit = half_angle.cos();
            (0..count)
                .map(|i| {
                    let dir = uniform_cone_direction(i, count, cos_limit);
                    let t = ((i as f32 * PHI).fract() * 0.999).clamp(0.0, 1.0);
                    origin + dir * (*length * t)
                })
                .collect()
        }
        EmitterShape::MeshSurface { triangles } => spawn_from_triangles(triangles, origin, count),
        EmitterShape::SkinnedMeshSurface {
            frames,
            active_frame,
        } => {
            if frames.is_empty() || count == 0 {
                return Vec::new();
            }
            let fi = active_frame % frames.len();
            let tris = &frames[fi];
            spawn_from_triangles(tris, origin, count)
        }
    }
}

fn spawn_from_triangles(triangles: &[[Vec3; 3]], origin: Vec3, count: usize) -> Vec<Vec3> {
    if triangles.is_empty() || count == 0 {
        return Vec::new();
    }
    (0..count)
        .map(|i| {
            let tri = &triangles[i % triangles.len()];
            let (a, b, c) = (tri[0], tri[1], tri[2]);
            let u = fract01(i as f32 * 0.618_034);
            let v = fract01(i as f32 * 0.379_682);
            if u + v > 1.0 {
                sample_triangle_point(a, b, c, 1.0 - u, 1.0 - v)
            } else {
                sample_triangle_point(a, b, c, u, v)
            }
        })
        .map(|p| origin + p)
        .collect()
}

fn fract01(x: f32) -> f32 {
    x - x.floor()
}

fn fibonacci_sphere(i: usize, n: usize) -> Vec3 {
    if n == 1 {
        return Vec3::Y;
    }
    let k = i as f32 + 0.5;
    let n = n as f32;
    let y = 1.0 - (2.0 * k) / n;
    let r = (1.0 - y * y).max(0.0).sqrt();
    let theta = PHI * PI * 2.0 * k;
    Vec3::new(r * theta.cos(), y, r * theta.sin())
}

fn stratified_box(i: usize, n: usize, h: Vec3) -> Vec3 {
    if n == 0 {
        return Vec3::ZERO;
    }
    let nx = (n as f32).cbrt().ceil() as usize;
    let ny = ((n as f32) / nx as f32).sqrt().ceil() as usize;
    let nz = n.div_ceil(nx * ny);
    let ix = i % nx;
    let iy = (i / nx) % ny;
    let iz = i / (nx * ny);
    let fx = (ix as f32 + 0.5) / nx as f32;
    let fy = (iy as f32 + 0.5) / ny as f32;
    let fz = (iz as f32 + 0.5) / nz.max(1) as f32;
    Vec3::new(
        (fx * 2.0 - 1.0) * h.x,
        (fy * 2.0 - 1.0) * h.y,
        (fz * 2.0 - 1.0) * h.z,
    )
}

fn uniform_cone_direction(i: usize, n: usize, cos_limit: f32) -> Vec3 {
    if n == 0 {
        return Vec3::Y;
    }
    let u = (i as f32 + 0.5) / n as f32;
    let cos_phi = 1.0 - u * (1.0 - cos_limit);
    let phi = cos_phi.clamp(-1.0, 1.0).acos();
    let theta = 2.0 * PI * fract01(i as f32 * PHI);
    let sin_phi = phi.sin();
    Vec3::new(sin_phi * theta.cos(), cos_phi, sin_phi * theta.sin())
}

fn sample_triangle_point(a: Vec3, b: Vec3, c: Vec3, u: f32, v: f32) -> Vec3 {
    let w = 1.0 - u - v;
    a * w + b * u + c * v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_vec3(a: Vec3, b: Vec3, eps: f32) {
        assert!(
            (a - b).length() <= eps,
            "expected {a:?} ~= {b:?} (eps {eps})"
        );
    }

    /// `TC-11.1.1.1` — point emitter, spawn 100 particles at origin.
    #[test]
    fn tc_11_1_1_1_emit_point_shape() {
        let origin = Vec3::new(1.0, -2.0, 3.5);
        let pts = spawn_positions(&EmitterShape::Point, origin, 100);
        assert_eq!(pts.len(), 100);
        for p in pts {
            assert_approx_vec3(p, origin, 1e-5);
        }
    }

    /// `TC-11.1.1.2` — sphere surface emitter, radius=5, spawn 1000.
    #[test]
    fn tc_11_1_1_2_emit_sphere_surface() {
        let origin = Vec3::ZERO;
        let pts = spawn_positions(&EmitterShape::SphereSurface { radius: 5.0 }, origin, 1000);
        assert_eq!(pts.len(), 1000);
        for p in pts {
            let d = (p - origin).length();
            assert!(
                (4.9..=5.1).contains(&d),
                "distance {d} not in [4.9, 5.1] for {p:?}"
            );
        }
    }

    /// `TC-11.1.1.3` — box emitter, half_extents=(2,3,4), spawn 1000.
    #[test]
    fn tc_11_1_1_3_emit_box_volume() {
        let origin = Vec3::ZERO;
        let h = Vec3::new(2.0, 3.0, 4.0);
        let pts = spawn_positions(&EmitterShape::BoxVolume { half_extents: h }, origin, 1000);
        assert_eq!(pts.len(), 1000);
        for p in pts {
            assert!((-2.0..=2.0).contains(&p.x), "x {}", p.x);
            assert!((-3.0..=3.0).contains(&p.y), "y {}", p.y);
            assert!((-4.0..=4.0).contains(&p.z), "z {}", p.z);
        }
    }

    /// `TC-11.1.1.4` — cone emitter, angle=30deg, radius=1.0 treated as length along axis, spawn 500.
    #[test]
    fn tc_11_1_1_4_emit_cone() {
        let origin = Vec3::ZERO;
        let opening = 30.0_f32.to_radians();
        let pts = spawn_positions(
            &EmitterShape::Cone {
                opening_angle_radians: opening,
                length: 1.0,
            },
            origin,
            500,
        );
        assert_eq!(pts.len(), 500);
        let axis = Vec3::Y;
        let half = (opening * 0.5) + 1e-3;
        for p in pts {
            let dir = (p - origin).normalize_or_zero();
            if dir.length_squared() < 1e-6 {
                continue;
            }
            let ang = dir.dot(axis).clamp(-1.0, 1.0).acos();
            assert!(
                ang <= half,
                "direction {dir:?} angle {ang} exceeds half-angle {half}"
            );
        }
    }

    /// `TC-11.1.1.5` — mesh surface emitter with cube mesh (12 triangles).
    #[test]
    fn tc_11_1_1_5_emit_mesh_surface_cube() {
        let unit = 0.5_f32;
        let p000 = Vec3::new(-unit, -unit, -unit);
        let p001 = Vec3::new(-unit, -unit, unit);
        let p010 = Vec3::new(-unit, unit, -unit);
        let p011 = Vec3::new(-unit, unit, unit);
        let p100 = Vec3::new(unit, -unit, -unit);
        let p101 = Vec3::new(unit, -unit, unit);
        let p110 = Vec3::new(unit, unit, -unit);
        let p111 = Vec3::new(unit, unit, unit);
        let tris = vec![
            [p000, p001, p011],
            [p000, p011, p010],
            [p100, p110, p111],
            [p100, p111, p101],
            [p000, p100, p101],
            [p000, p101, p001],
            [p010, p011, p111],
            [p010, p111, p110],
            [p000, p010, p110],
            [p000, p110, p100],
            [p001, p101, p111],
            [p001, p111, p011],
        ];
        let origin = Vec3::ZERO;
        let pts = spawn_positions(&EmitterShape::MeshSurface { triangles: tris }, origin, 600);
        assert_eq!(pts.len(), 600);
        for p in pts {
            let on_face = (p.x.abs() - unit).abs() < 1e-3
                || (p.y.abs() - unit).abs() < 1e-3
                || (p.z.abs() - unit).abs() < 1e-3;
            assert!(on_face, "point {p:?} not on cube shell");
        }
    }

    /// `TC-11.1.1.6` — skinned mesh uses the active pose triangles.
    #[test]
    fn tc_11_1_1_6_emit_skinned_mesh() {
        let tri_z0 = vec![[
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ]];
        let tri_z5 = vec![[
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::new(1.0, 0.0, 5.0),
            Vec3::new(0.0, 1.0, 5.0),
        ]];
        let origin = Vec3::ZERO;
        let pts = spawn_positions(
            &EmitterShape::SkinnedMeshSurface {
                frames: vec![tri_z0, tri_z5],
                active_frame: 1,
            },
            origin,
            50,
        );
        assert_eq!(pts.len(), 50);
        for p in pts {
            assert!((p.z - 5.0).abs() < 0.01, "expected pose z=5, got {p:?}");
        }
    }
}
