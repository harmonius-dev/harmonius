//! Point sets, height filtering, boolean ops, and mesh instance transforms.

use glam::{Mat4, Quat, Vec3};

/// Position + orientation + non-uniform scale for mesh spawning.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointInstance {
    /// World translation.
    pub position: Vec3,
    /// Orientation.
    pub rotation: Quat,
    /// Non-uniform scale.
    pub scale: Vec3,
}

/// Filter points by world-space height in `[ymin, ymax]`.
pub fn filter_height_range(points: &[PointInstance], ymin: f32, ymax: f32) -> Vec<PointInstance> {
    points
        .iter()
        .copied()
        .filter(|p| {
            let y = p.position.y;
            y >= ymin && y <= ymax
        })
        .collect()
}

/// Union of two point streams (concatenation for disjoint sets).
pub fn union_points(a: &[PointInstance], b: &[PointInstance]) -> Vec<PointInstance> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    out.extend_from_slice(a);
    out.extend_from_slice(b);
    out
}

/// Returns points in `a` farther than `radius` from every point in `b` (Euclidean, 3D).
pub fn difference_points(
    a: &[PointInstance],
    b: &[PointInstance],
    radius: f32,
) -> Vec<PointInstance> {
    let r2 = radius * radius;
    a.iter()
        .copied()
        .filter(|pa| {
            !b.iter()
                .any(|pb| pa.position.distance_squared(pb.position) <= r2)
        })
        .collect()
}

/// Builds mesh instance transforms from authored point instances.
pub fn spawn_mesh_instances(points: &[PointInstance], asset: &str) -> Vec<(String, Mat4)> {
    points
        .iter()
        .map(|p| {
            let m = Mat4::from_scale_rotation_translation(p.scale, p.rotation, p.position);
            (asset.to_string(), m)
        })
        .collect()
}

/// Extracts scale vectors from instance transforms for validation.
pub fn instance_scales(instances: &[(String, Mat4)]) -> Vec<Vec3> {
    instances
        .iter()
        .map(|(_, m)| {
            let sx = m.x_axis.length();
            let sy = m.y_axis.length();
            let sz = m.z_axis.length();
            Vec3::new(sx, sy, sz)
        })
        .collect()
}
