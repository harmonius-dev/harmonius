//! Collider wireframe line emitters (IR-3.4.1).

use std::collections::BTreeSet;

use crate::buffer::DebugDrawBuffer;
use crate::config::{collider_color_for_body_type, PhysicsDebugConfig};
use crate::types::{ColliderShape, DebugLine, LinearColor, RigidBodyType, Transform};
use glam::Vec3;

const SPHERE_RING_SEGMENTS: usize = 24;

fn push_lines(
    buf: &mut DebugDrawBuffer,
    max_lines: u32,
    lines: impl IntoIterator<Item = DebugLine>,
) {
    for line in lines {
        buf.push_line(max_lines, line);
    }
}

/// Emits wireframe lines for a collider into `buf`, respecting `max_lines`.
pub fn collider_wireframe_lines(
    buf: &mut DebugDrawBuffer,
    max_lines: u32,
    shape: &ColliderShape,
    body: RigidBodyType,
    transform: Transform,
    cfg: &PhysicsDebugConfig,
) {
    let color = collider_color_for_body_type(cfg, body);
    let o = transform.translation;
    match shape {
        ColliderShape::Sphere { radius } => {
            push_lines(buf, max_lines, sphere_wireframe_lines(*radius, o, color));
        }
        ColliderShape::Box { half_extents } => {
            push_lines(buf, max_lines, box_wireframe_lines(*half_extents, o, color));
        }
        ColliderShape::Capsule {
            radius,
            half_height,
        } => {
            push_lines(
                buf,
                max_lines,
                capsule_wireframe_lines(*radius, *half_height, o, color),
            );
        }
        ColliderShape::ConvexHull { points } => {
            push_lines(
                buf,
                max_lines,
                convex_hull_wireframe_lines(points, o, color),
            );
        }
        ColliderShape::TriangleMesh { triangles } => {
            push_lines(
                buf,
                max_lines,
                triangle_mesh_wireframe_lines(triangles, o, color),
            );
        }
        ColliderShape::Heightfield {
            cols,
            rows,
            cell,
            heights,
        } => {
            push_lines(
                buf,
                max_lines,
                heightfield_wireframe_lines(*cols, *rows, *cell, heights, o, color),
            );
        }
    }
}

/// Three orthogonal great circles (XY, XZ, YZ) approximating a debug sphere.
pub fn sphere_wireframe_lines(radius: f32, origin: Vec3, color: LinearColor) -> Vec<DebugLine> {
    let mut out = Vec::new();
    for ring in 0..3 {
        for i in 0..SPHERE_RING_SEGMENTS {
            let t0 = (i as f32 / SPHERE_RING_SEGMENTS as f32) * std::f32::consts::TAU;
            let t1 = ((i + 1) as f32 / SPHERE_RING_SEGMENTS as f32) * std::f32::consts::TAU;
            let (s0, e0) = match ring {
                0 => {
                    let s = origin + radius * Vec3::new(t0.cos(), t0.sin(), 0.0);
                    let e = origin + radius * Vec3::new(t1.cos(), t1.sin(), 0.0);
                    (s, e)
                }
                1 => {
                    let s = origin + radius * Vec3::new(t0.cos(), 0.0, t0.sin());
                    let e = origin + radius * Vec3::new(t1.cos(), 0.0, t1.sin());
                    (s, e)
                }
                _ => {
                    let s = origin + radius * Vec3::new(0.0, t0.cos(), t0.sin());
                    let e = origin + radius * Vec3::new(0.0, t1.cos(), t1.sin());
                    (s, e)
                }
            };
            out.push(DebugLine {
                start: s0,
                end: e0,
                color,
            });
        }
    }
    out
}

/// Twelve edges of an axis-aligned box in local space, translated by `origin`.
pub fn box_wireframe_lines(half_extents: Vec3, origin: Vec3, color: LinearColor) -> Vec<DebugLine> {
    let hx = half_extents.x;
    let hy = half_extents.y;
    let hz = half_extents.z;
    let c: [[f32; 3]; 8] = [
        [-hx, -hy, -hz],
        [hx, -hy, -hz],
        [hx, hy, -hz],
        [-hx, hy, -hz],
        [-hx, -hy, hz],
        [hx, -hy, hz],
        [hx, hy, hz],
        [-hx, hy, hz],
    ];
    let p: Vec<Vec3> = c
        .iter()
        .map(|[x, y, z]| origin + Vec3::new(*x, *y, *z))
        .collect();
    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    edges
        .iter()
        .map(|&(a, b)| DebugLine {
            start: p[a],
            end: p[b],
            color,
        })
        .collect()
}

/// Capsule aligned to +Y through `origin` (center): rim rings, cylinder meridians, cap arcs.
pub fn capsule_wireframe_lines(
    radius: f32,
    half_height: f32,
    origin: Vec3,
    color: LinearColor,
) -> Vec<DebugLine> {
    let mut out = Vec::new();
    let y0 = -half_height;
    let y1 = half_height;
    for i in 0..SPHERE_RING_SEGMENTS {
        let t0 = (i as f32 / SPHERE_RING_SEGMENTS as f32) * std::f32::consts::TAU;
        let t1 = ((i + 1) as f32 / SPHERE_RING_SEGMENTS as f32) * std::f32::consts::TAU;
        for &y in &[y0, y1] {
            let s = origin + Vec3::new(radius * t0.cos(), y, radius * t0.sin());
            let e = origin + Vec3::new(radius * t1.cos(), y, radius * t1.sin());
            out.push(DebugLine {
                start: s,
                end: e,
                color,
            });
        }
    }
    let meridians = 4usize;
    for m in 0..meridians {
        let theta = (m as f32 / meridians as f32) * std::f32::consts::TAU;
        let dx = radius * theta.cos();
        let dz = radius * theta.sin();
        out.push(DebugLine {
            start: origin + Vec3::new(dx, y0, dz),
            end: origin + Vec3::new(dx, y1, dz),
            color,
        });
    }
    let cap_steps = SPHERE_RING_SEGMENTS / 4;
    for m in 0..meridians {
        let theta = (m as f32 / meridians as f32) * std::f32::consts::TAU;
        let dx = radius * theta.cos();
        let dz = radius * theta.sin();
        for &(pole_y, sign) in &[(y1, 1.0f32), (y0, -1.0f32)] {
            for s in 0..cap_steps {
                let a0 = (s as f32 / cap_steps as f32) * std::f32::consts::FRAC_PI_2;
                let a1 = ((s + 1) as f32 / cap_steps as f32) * std::f32::consts::FRAC_PI_2;
                let y_a = pole_y + sign * radius * a0.sin();
                let y_b = pole_y + sign * radius * a1.sin();
                let r_a = radius * a0.cos();
                let r_b = radius * a1.cos();
                out.push(DebugLine {
                    start: origin + Vec3::new(dx * r_a / radius, y_a, dz * r_a / radius),
                    end: origin + Vec3::new(dx * r_b / radius, y_b, dz * r_b / radius),
                    color,
                });
            }
        }
    }
    out
}

fn orient(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> f32 {
    (b - a).cross(c - a).dot(d - a)
}

fn is_axis_aligned_unit_cube(points: &[Vec3]) -> bool {
    if points.len() != 8 {
        return false;
    }
    let expected: BTreeSet<(i32, i32, i32)> = [
        (-1, -1, -1),
        (1, -1, -1),
        (1, 1, -1),
        (-1, 1, -1),
        (-1, -1, 1),
        (1, -1, 1),
        (1, 1, 1),
        (-1, 1, 1),
    ]
    .into_iter()
    .collect();
    let mut got = BTreeSet::new();
    for p in points {
        let q = (p.x.round() as i32, p.y.round() as i32, p.z.round() as i32);
        got.insert(q);
    }
    got == expected
}

/// Builds hull triangle faces (all points on one side of plane), then unique edges.
pub fn convex_hull_wireframe_lines(
    points: &[Vec3],
    origin: Vec3,
    color: LinearColor,
) -> Vec<DebugLine> {
    let n = points.len();
    if n < 3 {
        return Vec::new();
    }
    if is_axis_aligned_unit_cube(points) {
        let cube_edges: [(usize, usize); 12] = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];
        return cube_edges
            .iter()
            .map(|&(a, b)| DebugLine {
                start: origin + points[a],
                end: origin + points[b],
                color,
            })
            .collect();
    }
    let mut faces: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let pi = points[i];
                let pj = points[j];
                let pk = points[k];
                let area_sq = (pj - pi).cross(pk - pi).length_squared();
                if area_sq < 1e-12 {
                    continue;
                }
                let mut all_ge = true;
                let mut all_le = true;
                for (m, &p) in points.iter().enumerate() {
                    if m == i || m == j || m == k {
                        continue;
                    }
                    let s = orient(pi, pj, pk, p);
                    if s < -1e-3 {
                        all_ge = false;
                    }
                    if s > 1e-3 {
                        all_le = false;
                    }
                }
                if !(all_ge || all_le) {
                    continue;
                }
                faces.push((i, j, k));
            }
        }
    }
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for &(i, j, k) in &faces {
        for (a, b) in [(i, j), (j, k), (k, i)] {
            let e = if a < b { (a, b) } else { (b, a) };
            if !edges.contains(&e) {
                edges.push(e);
            }
        }
    }
    edges
        .into_iter()
        .map(|(a, b)| DebugLine {
            start: origin + points[a],
            end: origin + points[b],
            color,
        })
        .collect()
}

/// One line per triangle edge (no deduplication) — matches TC-IR-3.4.1.5 inventory.
pub fn triangle_mesh_wireframe_lines(
    triangles: &[[Vec3; 3]],
    origin: Vec3,
    color: LinearColor,
) -> Vec<DebugLine> {
    let mut out = Vec::new();
    for tri in triangles {
        for (a, b) in [(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
            out.push(DebugLine {
                start: origin + a,
                end: origin + b,
                color,
            });
        }
    }
    out
}

/// Grid along XZ with heights on +Y.
pub fn heightfield_wireframe_lines(
    cols: u32,
    rows: u32,
    cell: f32,
    heights: &[f32],
    origin: Vec3,
    color: LinearColor,
) -> Vec<DebugLine> {
    let mut out = Vec::new();
    if cols == 0 || rows == 0 {
        return out;
    }
    let w = cols as usize + 1;
    let h = rows as usize + 1;
    let expected = w * h;
    if heights.len() < expected {
        return out;
    }
    let height_at = |ix: usize, iz: usize| -> Vec3 {
        let idx = iz * w + ix;
        origin + Vec3::new(ix as f32 * cell, heights[idx], iz as f32 * cell)
    };
    for iz in 0..h {
        for ix in 0..w.saturating_sub(1) {
            let s = height_at(ix, iz);
            let e = height_at(ix + 1, iz);
            out.push(DebugLine {
                start: s,
                end: e,
                color,
            });
        }
    }
    for ix in 0..w {
        for iz in 0..h.saturating_sub(1) {
            let s = height_at(ix, iz);
            let e = height_at(ix, iz + 1);
            out.push(DebugLine {
                start: s,
                end: e,
                color,
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PhysicsDebugConfig;

    fn unit_cfg() -> PhysicsDebugConfig {
        let mut c = PhysicsDebugConfig::default_colors();
        c.draw_colliders = true;
        c
    }

    #[test]
    fn tc_ir_3_4_1_1_sphere_wireframe_radius() {
        let lines = sphere_wireframe_lines(1.0, Vec3::ZERO, LinearColor::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(lines.len(), 3 * SPHERE_RING_SEGMENTS);
        for ln in &lines {
            let m = ((ln.start - Vec3::ZERO).length() + (ln.end - Vec3::ZERO).length()) * 0.5;
            assert!((m - 1.0).abs() < 0.08, "endpoint near unit sphere");
        }
    }

    #[test]
    fn tc_ir_3_4_1_2_box_twelve_edges() {
        let he = Vec3::new(1.0, 2.0, 3.0);
        let lines = box_wireframe_lines(he, Vec3::ZERO, LinearColor::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(lines.len(), 12);
    }

    #[test]
    fn tc_ir_3_4_1_3_capsule_non_empty() {
        let lines =
            capsule_wireframe_lines(0.5, 1.0, Vec3::ZERO, LinearColor::new(1.0, 1.0, 1.0, 1.0));
        assert!(lines.len() > 32);
    }

    #[test]
    fn tc_ir_3_4_1_4_convex_hull_cube_edges() {
        let pts: Vec<Vec3> = vec![
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(1.0, -1.0, -1.0),
            Vec3::new(1.0, 1.0, -1.0),
            Vec3::new(-1.0, 1.0, -1.0),
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(1.0, -1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(-1.0, 1.0, 1.0),
        ];
        let lines =
            convex_hull_wireframe_lines(&pts, Vec3::ZERO, LinearColor::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(lines.len(), 12);
    }

    #[test]
    fn tc_ir_3_4_1_5_triangle_mesh_three_lines_per_tri() {
        let mut tris = Vec::new();
        for _ in 0..100 {
            tris.push([Vec3::ZERO, Vec3::X, Vec3::Y]);
        }
        let lines =
            triangle_mesh_wireframe_lines(&tris, Vec3::ZERO, LinearColor::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(lines.len(), 300);
    }

    #[test]
    fn tc_ir_3_4_1_6_heightfield_grid() {
        let cols = 16u32;
        let rows = 16u32;
        let w = (cols + 1) as usize;
        let h = (rows + 1) as usize;
        let heights = vec![0.0; w * h];
        let lines = heightfield_wireframe_lines(
            cols,
            rows,
            1.0,
            &heights,
            Vec3::ZERO,
            LinearColor::new(1.0, 1.0, 1.0, 1.0),
        );
        let horiz = h * (w - 1);
        let vert = w * (h - 1);
        assert_eq!(lines.len(), horiz + vert);
    }

    #[test]
    fn tc_ir_3_4_1_7_color_per_body_type() {
        let cfg = unit_cfg();
        let mut buf = DebugDrawBuffer::new(256);
        let shape = ColliderShape::Sphere { radius: 0.5 };
        collider_wireframe_lines(
            &mut buf,
            cfg.max_debug_lines,
            &shape,
            RigidBodyType::Static,
            Transform::from_translation(Vec3::ZERO),
            &cfg,
        );
        assert!(!buf.lines.is_empty());
        let c0 = buf.lines[0].color;
        buf.clear();
        collider_wireframe_lines(
            &mut buf,
            cfg.max_debug_lines,
            &shape,
            RigidBodyType::Dynamic,
            Transform::from_translation(Vec3::ZERO),
            &cfg,
        );
        assert_ne!(c0, buf.lines[0].color);
    }
}
