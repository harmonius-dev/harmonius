//! Nine-slice UV solver (IR-3.6.2 / design `NineSliceSolver`).

use glam::Vec2;

/// UV axis-aligned rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UvRect {
    /// Minimum corner.
    pub min: Vec2,
    /// Maximum corner.
    pub max: Vec2,
}

/// Atlas UV region for a sprite.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AtlasRegion {
    /// Minimum UV.
    pub min: Vec2,
    /// Maximum UV.
    pub max: Vec2,
}

/// Border insets in logical pixels (design `Edges`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edges {
    /// Left inset.
    pub left: f32,
    /// Top inset.
    pub top: f32,
    /// Right inset.
    pub right: f32,
    /// Bottom inset.
    pub bottom: f32,
}

/// Nine-slice UV output for a single widget.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NineSliceResult {
    /// Nine UV patches (3×3 ordering: rows left→right, top→bottom).
    pub patches: [UvRect; 9],
    /// Patch sizes in layout space (logical pixels).
    pub patch_sizes: [Vec2; 9],
}

/// Solves nine-slice sprite UVs from border insets (design `NineSliceSolver`).
pub struct NineSliceSolver;

impl NineSliceSolver {
    /// Computes nine-slice UVs; clamps border insets when they exceed half the sprite dimension
    /// (design fallback).
    #[must_use]
    pub fn solve(region: &AtlasRegion, borders: Edges, target_size: Vec2) -> NineSliceResult {
        let half_w = (target_size.x * 0.5).max(1e-6);
        let half_h = (target_size.y * 0.5).max(1e-6);
        let mut b = borders;
        b.left = b.left.clamp(0.0, half_w);
        b.right = b.right.clamp(0.0, half_w);
        b.top = b.top.clamp(0.0, half_h);
        b.bottom = b.bottom.clamp(0.0, half_h);
        let du = region.max - region.min;
        let uv_w = du.x.max(1e-6);
        let uv_h = du.y.max(1e-6);
        let u_left = region.min.x + (b.left / target_size.x) * uv_w;
        let u_right = region.max.x - (b.right / target_size.x) * uv_w;
        let v_top = region.min.y + (b.top / target_size.y) * uv_h;
        let v_bottom = region.max.y - (b.bottom / target_size.y) * uv_h;
        let mid_u1 = u_right;
        let mid_v1 = v_bottom;
        let patches = [
            UvRect {
                min: Vec2::new(region.min.x, region.min.y),
                max: Vec2::new(u_left, v_top),
            },
            UvRect {
                min: Vec2::new(u_left, region.min.y),
                max: Vec2::new(mid_u1, v_top),
            },
            UvRect {
                min: Vec2::new(mid_u1, region.min.y),
                max: Vec2::new(region.max.x, v_top),
            },
            UvRect {
                min: Vec2::new(region.min.x, v_top),
                max: Vec2::new(u_left, mid_v1),
            },
            UvRect {
                min: Vec2::new(u_left, v_top),
                max: Vec2::new(mid_u1, mid_v1),
            },
            UvRect {
                min: Vec2::new(mid_u1, v_top),
                max: Vec2::new(region.max.x, mid_v1),
            },
            UvRect {
                min: Vec2::new(region.min.x, mid_v1),
                max: Vec2::new(u_left, region.max.y),
            },
            UvRect {
                min: Vec2::new(u_left, mid_v1),
                max: Vec2::new(mid_u1, region.max.y),
            },
            UvRect {
                min: Vec2::new(mid_u1, mid_v1),
                max: Vec2::new(region.max.x, region.max.y),
            },
        ];
        let sx = target_size.x.max(1e-6);
        let sy = target_size.y.max(1e-6);
        let patch_sizes = [
            Vec2::new(b.left, b.top),
            Vec2::new((sx - b.left - b.right).max(0.0), b.top),
            Vec2::new(b.right, b.top),
            Vec2::new(b.left, (sy - b.top - b.bottom).max(0.0)),
            Vec2::new(
                (sx - b.left - b.right).max(0.0),
                (sy - b.top - b.bottom).max(0.0),
            ),
            Vec2::new(b.right, (sy - b.top - b.bottom).max(0.0)),
            Vec2::new(b.left, b.bottom),
            Vec2::new((sx - b.left - b.right).max(0.0), b.bottom),
            Vec2::new(b.right, b.bottom),
        ];
        NineSliceResult {
            patches,
            patch_sizes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AtlasRegion, Edges, NineSliceSolver};
    use glam::Vec2;

    #[test]
    fn oversized_borders_clamped_to_half_dimensions() {
        let region = AtlasRegion {
            min: Vec2::ZERO,
            max: Vec2::ONE,
        };
        let borders = Edges {
            left: 80.0,
            top: 80.0,
            right: 80.0,
            bottom: 80.0,
        };
        let target = Vec2::new(100.0, 100.0);
        let out = NineSliceSolver::solve(&region, borders, target);
        assert!(out.patch_sizes[0].x <= 50.0);
        assert!(out.patch_sizes[0].y <= 50.0);
    }

    #[test]
    fn zero_borders_full_center_patch_uv_span() {
        let region = AtlasRegion {
            min: Vec2::ZERO,
            max: Vec2::ONE,
        };
        let borders = Edges {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        };
        let target = Vec2::new(120.0, 60.0);
        let out = NineSliceSolver::solve(&region, borders, target);
        let mid = out.patches[4];
        assert!((mid.max - mid.min).x > 0.9);
        assert!((mid.max - mid.min).y > 0.9);
    }
}
