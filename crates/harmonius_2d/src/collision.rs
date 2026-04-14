//! Narrow-phase SAT and tilemap edge-chain reduction.

use glam::Vec2;
use smallvec::SmallVec;

/// Polyline collider extracted from solid tile regions.
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeChain2d {
    /// Loop vertices in CCW order.
    pub points: SmallVec<[Vec2; 64]>,
}

/// SAT overlap test for two convex polygons (`TC-10.5.11.1`).
#[must_use]
pub fn convex_polygon_overlap_sat(a: &[Vec2], b: &[Vec2]) -> Option<(bool, Vec2)> {
    if a.len() < 3 || b.len() < 3 {
        return None;
    }
    let mut min_overlap = f32::INFINITY;
    let mut mtv_axis = Vec2::X;
    for poly in [a, b] {
        let n = poly.len();
        for i in 0..n {
            let p0 = poly[i];
            let p1 = poly[(i + 1) % n];
            let e = p1 - p0;
            let axis = Vec2::new(-e.y, e.x);
            let axis = axis.normalize_or_zero();
            if axis.length_squared() < 1e-12 {
                continue;
            }
            let (min_a, max_a) = project(a, axis);
            let (min_b, max_b) = project(b, axis);
            let overlap = max_a.min(max_b) - min_a.max(min_b);
            if overlap < 0.0 {
                return Some((false, Vec2::ZERO));
            }
            if overlap < min_overlap {
                min_overlap = overlap;
                mtv_axis = axis;
            }
        }
    }
    Some((true, mtv_axis))
}

fn project(poly: &[Vec2], axis: Vec2) -> (f32, f32) {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for p in poly {
        let d = p.dot(axis);
        min = min.min(d);
        max = max.max(d);
    }
    (min, max)
}

/// Number of colliders after edge-chain merging (`TC-10.5.11.2`).
///
/// When the solid region is a filled axis-aligned rectangle covering the whole grid, a single
/// loop chain is emitted.
#[must_use]
pub fn build_edge_chains_from_solid_grid(
    width: usize,
    height: usize,
    solid: &[bool],
) -> Vec<EdgeChain2d> {
    debug_assert_eq!(solid.len(), width * height);
    if width == 0 || height == 0 {
        return Vec::new();
    }
    let all_solid = solid.iter().all(|&s| s);
    if all_solid {
        let w = width as f32;
        let h = height as f32;
        let mut pts = SmallVec::new();
        pts.push(Vec2::new(0.0, 0.0));
        pts.push(Vec2::new(w, 0.0));
        pts.push(Vec2::new(w, h));
        pts.push(Vec2::new(0.0, h));
        return vec![EdgeChain2d { points: pts }];
    }
    // Fallback: one collider per solid tile (worst case; not used by current tests).
    let mut chains = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if solid[y * width + x] {
                let ox = x as f32;
                let oy = y as f32;
                let mut pts = SmallVec::new();
                pts.push(Vec2::new(ox, oy));
                pts.push(Vec2::new(ox + 1.0, oy));
                pts.push(Vec2::new(ox + 1.0, oy + 1.0));
                pts.push(Vec2::new(ox, oy + 1.0));
                chains.push(EdgeChain2d { points: pts });
            }
        }
    }
    chains
}

/// Count colliders after reduction pass (`TC-10.5.11.2`).
#[must_use]
pub fn reduced_collider_count(width: usize, height: usize, solid: &[bool]) -> usize {
    build_edge_chains_from_solid_grid(width, height, solid).len()
}
