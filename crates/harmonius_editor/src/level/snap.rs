//! Grid snapping for entity placement ([TC-15.2.1.1](https://github.com/cjhowe-us/harmonius/blob/main/docs/design/tools/level-world-test-cases.md)).

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
}
