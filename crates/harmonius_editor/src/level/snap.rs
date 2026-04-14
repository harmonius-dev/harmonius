//! Placement snapping (grid, surface alignment) for `PLAN-tools-level-world` / `level-world.md`.
//!
//! Covered test IDs: `TC-15.2.1.1`, `TC-15.2.1.2`, `TC-15.2.1.3` in
//! `docs/design/tools/level-world-test-cases.md`.

use std::ops::{Add, Sub};

/// World-space vector with `f32` components (authoritative layout in `level-world.md`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    /// X coordinate in world units.
    pub x: f32,
    /// Y coordinate in world units.
    pub y: f32,
    /// Z coordinate in world units.
    pub z: f32,
}

impl Vec3 {
    /// Builds a vector from individual components.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Dot product of two vectors.
    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product `self × other`.
    #[must_use]
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Euclidean length `√(x² + y² + z²)`.
    #[must_use]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Same direction with length `1`, or [`None`] if length is not finite or near zero.
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let len = self.length();
        if !len.is_finite() || len < 1e-20 {
            return None;
        }
        Some(Self::new(self.x / len, self.y / len, self.z / len))
    }

    /// Multiplies each component by `s`.
    #[must_use]
    pub fn mul_scalar(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Unit-norm rotation as `(x, y, z, w)` with real part `w` (`xi + yj + zk + w`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    /// Imaginary `i` coefficient.
    pub x: f32,
    /// Imaginary `j` coefficient.
    pub y: f32,
    /// Imaginary `k` coefficient.
    pub z: f32,
    /// Real / scalar part.
    pub w: f32,
}

impl Quat {
    /// Identity quaternion (`0`, `0`, `0`, `1`): no rotation.
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    /// Constructs a quaternion from `(x, y, z, w)` imaginary-then-real parts.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Scales to unit length, or [`None`] if the norm is not finite or near zero.
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if !len.is_finite() || len < 1e-20 {
            return None;
        }
        Some(Self::new(
            self.x / len,
            self.y / len,
            self.z / len,
            self.w / len,
        ))
    }
}

/// Rotates `v` by unit quaternion `q` (Hamilton convention, active rotation).
#[must_use]
pub fn rotate_vector_by_unit_quat(q: Quat, v: Vec3) -> Vec3 {
    let u = Vec3::new(q.x, q.y, q.z);
    let t = u.cross(v).mul_scalar(2.0);
    v + t.mul_scalar(q.w) + u.cross(t)
}

/// World-space +Y after surface snap should match the hit **unit** normal (`level-world.md`).
///
/// Returns a unit quaternion that maps world +Y onto `surface_normal` when `surface_normal` is a
/// valid unit vector. This is the deterministic rotation used once a terrain raycast supplies the
/// hit normal (TC-15.2.1.2).
#[must_use]
pub fn rotation_align_positive_y_to_surface_normal(surface_normal: Vec3) -> Option<Quat> {
    let n = surface_normal.try_normalize()?;
    let from = Vec3::new(0.0, 1.0, 0.0);
    let d = from.dot(n).clamp(-1.0, 1.0);

    if d > 0.999_999 {
        return Some(Quat::IDENTITY);
    }

    if d < -0.999_999 {
        // 180° — rotate +Y to -Y around X.
        return Quat::new(1.0, 0.0, 0.0, 0.0).try_normalize();
    }

    let c = from.cross(n);
    let raw = Quat::new(c.x, c.y, c.z, 1.0 + d);
    raw.try_normalize()
}

/// Snaps `position` to the nearest candidate mesh vertex (world space).
///
/// Returns [`None`] when `candidates` is empty. Used once vertex candidates are collected from the
/// spatial index (TC-15.2.1.3).
#[must_use]
pub fn snap_position_to_nearest_vertex(position: Vec3, candidates: &[Vec3]) -> Option<Vec3> {
    let first = *candidates.first()?;
    let mut best = first;
    let mut best_d2 = (position - best).dot(position - best);
    for &v in candidates.iter().skip(1) {
        let d2 = (position - v).dot(position - v);
        if d2 < best_d2 {
            best_d2 = d2;
            best = v;
        }
    }
    Some(best)
}

/// Snapping mode for entity placement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SnapMode {
    /// Align translations to a uniform grid (`cell_size` world units per cell).
    Grid {
        /// Cell edge length; must be strictly positive.
        cell_size: f32,
    },
    /// Project onto the nearest surface (requires spatial queries; not implemented here).
    Surface,
    /// Snap to mesh vertices (requires spatial queries; not implemented here).
    Vertex,
    /// Disable snapping; returns the input position unchanged.
    None,
}

/// Snaps `position` according to `mode`.
///
/// Returns [`None`] when the mode needs scene data that is not supplied (surface / vertex).
#[must_use]
pub fn snap_position(position: Vec3, mode: SnapMode) -> Option<Vec3> {
    match mode {
        SnapMode::Grid { cell_size } => Some(snap_grid(position, cell_size)),
        SnapMode::None => Some(position),
        SnapMode::Surface | SnapMode::Vertex => None,
    }
}

/// Snaps each axis independently to the nearest multiple of `cell_size`.
///
/// # Panics
///
/// Debug builds: panics when `cell_size` is not finite or `<= 0`.
#[must_use]
pub fn snap_grid(position: Vec3, cell_size: f32) -> Vec3 {
    debug_assert!(cell_size.is_finite() && cell_size > 0.0);
    let c = cell_size;
    Vec3::new(
        (position.x / c).round() * c,
        (position.y / c).round() * c,
        (position.z / c).round() * c,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_vec3_close(a: Vec3, b: Vec3, eps: f32) {
        assert!(
            (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps && (a.z - b.z).abs() <= eps,
            "expected {:?} ~= {:?} (eps={eps})",
            a,
            b
        );
    }

    /// TC-15.2.1.1 — Grid Snap Alignment (`docs/design/tools/level-world-test-cases.md`).
    #[test]
    fn tc_15_2_1_1_grid_snap_alignment() {
        assert_vec3_close(
            snap_grid(Vec3::new(1.3, 0.0, 2.7), 1.0),
            Vec3::new(1.0, 0.0, 3.0),
            1e-4,
        );
        assert_vec3_close(
            snap_grid(Vec3::new(2.5, 0.0, 4.5), 0.5),
            Vec3::new(2.5, 0.0, 4.5),
            1e-4,
        );
        assert_vec3_close(
            snap_grid(Vec3::new(0.1, 0.0, 0.1), 1.0),
            Vec3::new(0.0, 0.0, 0.0),
            1e-4,
        );
    }

    #[test]
    fn tc_15_2_1_1_snap_mode_grid_path() {
        let got = snap_position(Vec3::new(1.3, 0.0, 2.7), SnapMode::Grid { cell_size: 1.0 })
            .expect("grid snap must succeed without scene");
        assert_vec3_close(got, Vec3::new(1.0, 0.0, 3.0), 1e-4);
    }

    #[test]
    fn snap_mode_none_is_identity() {
        let p = Vec3::new(9.25, -3.5, 1.0);
        assert_eq!(snap_position(p, SnapMode::None), Some(p));
    }

    /// TC-15.2.1.2 — Surface snap: entity up aligns to terrain normal (`level-world-test-cases.md`).
    #[test]
    fn tc_15_2_1_2_surface_snap_terrain_aligns_up_to_normal() {
        let slope_from_vertical_rad = std::f32::consts::FRAC_PI_6;
        // Unit normal 30° from world +Y toward +X (raycast hit on 30° terrain slope).
        let surface_normal = Vec3::new(
            slope_from_vertical_rad.sin(),
            slope_from_vertical_rad.cos(),
            0.0,
        );
        let q = rotation_align_positive_y_to_surface_normal(surface_normal)
            .expect("rotation must exist for valid terrain normal");
        let entity_up = rotate_vector_by_unit_quat(q, Vec3::new(0.0, 1.0, 0.0));
        assert_vec3_close(entity_up, surface_normal, 1e-4);

        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let cos_angle = entity_up.dot(world_up).clamp(-1.0, 1.0);
        let angle_from_vertical = cos_angle.acos();
        assert!(
            (angle_from_vertical - slope_from_vertical_rad).abs() < 1e-3,
            "entity up should be ~30° from vertical, got {} rad",
            angle_from_vertical
        );
    }

    /// TC-15.2.1.3 — Vertex snap matches target vertex (`level-world-test-cases.md`).
    #[test]
    fn tc_15_2_1_3_vertex_snap_precision() {
        let vertex = Vec3::new(5.0, 3.0, 7.0);
        let candidates = [vertex];
        let snapped = snap_position_to_nearest_vertex(Vec3::new(4.9, 3.02, 7.01), &candidates)
            .expect("non-empty candidate list");
        assert_vec3_close(snapped, vertex, 1e-5);
    }
}
