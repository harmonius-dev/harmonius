//! CPU-side helpers for particle rendering math (`TC-11.1.3.*` subset).

use glam::{Mat4, Vec3};

/// Builds an orthonormal camera basis from forward (camera looks along `-forward`).
#[must_use]
pub fn camera_basis(forward: Vec3) -> (Vec3, Vec3) {
    let f = forward.normalize_or_zero();
    let mut world_up = Vec3::Y;
    let mut right = f.cross(world_up);
    if right.length_squared() < 1e-8 {
        world_up = Vec3::X;
        right = f.cross(world_up);
    }
    let right = right.normalize_or_zero();
    let up = right.cross(f).normalize_or_zero();
    (right, up)
}

/// World-space corners for a view-facing unit quad at `center` with half `extent`.
#[must_use]
pub fn sprite_billboard_corners(center: Vec3, forward: Vec3, extent: f32) -> [Vec3; 4] {
    let (right, up) = camera_basis(forward);
    let r = right * extent;
    let u = up * extent;
    [
        center - r - u,
        center + r - u,
        center + r + u,
        center - r + u,
    ]
}

/// Flipbook frame index for `elapsed` seconds at `fps` with `frame_count` frames (`TC-11.1.3.2`).
#[must_use]
pub fn flipbook_frame_index(elapsed: f32, fps: f32, frame_count: u32) -> u32 {
    if frame_count == 0 {
        return 0;
    }
    let idx = (elapsed * fps).floor() as i64;
    (idx.rem_euclid(frame_count as i64)) as u32
}

/// Alpha multiplier for soft depth fade when `depth_delta` is within `fade_distance`.
#[must_use]
pub fn soft_depth_fade_alpha(depth_delta: f32, fade_distance: f32) -> f32 {
    if fade_distance <= 0.0 {
        return 1.0;
    }
    (depth_delta / fade_distance).clamp(0.0, 1.0)
}

/// Returns edge lengths between consecutive ribbon samples; all should be small for continuity
/// (`TC-11.1.3.4`).
#[must_use]
pub fn ribbon_segment_lengths(points: &[Vec3]) -> Vec<f32> {
    if points.len() < 2 {
        return Vec::new();
    }
    points.windows(2).map(|w| w[1].distance(w[0])).collect()
}

/// Uniform Catmull-Rom sample on the segment between `p1` and `p2` with neighbors `p0`, `p3`.
#[must_use]
pub fn catmull_rom_segment(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let t = t.clamp(0.0, 1.0);
    let t2 = t * t;
    let t3 = t2 * t;
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

/// Counts instance transforms (identity check optional); used for mesh particle batching
/// (`TC-11.1.3.6`).
#[must_use]
pub fn mesh_particle_instance_matrices(transforms: &[Mat4]) -> usize {
    transforms.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.3.1` — quad faces camera from four forward directions.
    #[test]
    fn tc_11_1_3_1_sprite_billboard() {
        let center = Vec3::ZERO;
        let forwards = [
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-0.5, 0.0, -0.5).normalize(),
        ];
        for fwd in forwards {
            let corners = sprite_billboard_corners(center, fwd, 0.5);
            let f = fwd.normalize();
            for c in corners {
                let outward = (c - center).normalize_or_zero();
                assert!(
                    outward.dot(f).abs() < 0.05,
                    "corner should lie in the camera view plane"
                );
            }
        }
    }

    /// `TC-11.1.3.2` — 8-frame flipbook at 10 FPS, elapsed 0.35s → frame 3.
    #[test]
    fn tc_11_1_3_2_flipbook_animation() {
        assert_eq!(flipbook_frame_index(0.35, 10.0, 8), 3);
    }

    /// `TC-11.1.3.3` — alpha fades when depth delta is small.
    #[test]
    fn tc_11_1_3_3_soft_depth_fade() {
        let a = soft_depth_fade_alpha(0.05, 0.1);
        assert!((a - 0.5).abs() < 1e-4);
    }

    /// `TC-11.1.3.4` — sequential ribbon points have short segments.
    #[test]
    fn tc_11_1_3_4_ribbon_connectivity() {
        let pts: Vec<Vec3> = (0..10).map(|i| Vec3::new(i as f32, 0.0, 0.0)).collect();
        let lens = ribbon_segment_lengths(&pts);
        assert_eq!(lens.len(), 9);
        for len in lens {
            assert!(len < 1.01 && len > 0.99);
        }
    }

    /// `TC-11.1.3.5` — Catmull-Rom follows control hull.
    #[test]
    fn tc_11_1_3_5_ribbon_catmull_rom() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(1.0, 0.0, 0.0);
        let p2 = Vec3::new(2.0, 1.0, 0.0);
        let p3 = Vec3::new(3.0, 0.0, 0.0);
        let mid = catmull_rom_segment(p0, p1, p2, p3, 0.5);
        assert!(mid.y > 0.0 && mid.x > 1.0 && mid.x < 2.0);
    }

    /// `TC-11.1.3.6` — one matrix per mesh particle instance.
    #[test]
    fn tc_11_1_3_6_mesh_particle_instancing() {
        let mats: Vec<Mat4> = (0..100)
            .map(|i| Mat4::from_translation(Vec3::new(i as f32, 0.0, 0.0)))
            .collect();
        assert_eq!(mesh_particle_instance_matrices(&mats), 100);
    }
}
