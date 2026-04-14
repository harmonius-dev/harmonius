//! Mapping world positions to streaming tile coordinates.

use glam::Vec3;

use super::types::TileCoord;

/// Converts a world-space position to integer tile indices using `tile_size` meters per edge.
///
/// Uses `floor` on each axis so negative coordinates map to negative tile indices
/// (TC-7.1.2.1). Callers must supply a finite, positive `tile_size`; non-finite or non-positive
/// values clamp to `1.0` so release builds never divide by zero.
#[must_use]
pub fn world_position_to_tile_coord(position: Vec3, tile_size: f32) -> TileCoord {
    let ts = if tile_size.is_finite() && tile_size > 0.0 {
        tile_size
    } else {
        1.0
    };
    TileCoord {
        x: (position.x / ts).floor() as i32,
        z: (position.z / ts).floor() as i32,
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;

    /// TC-7.1.2.1 #1
    #[test]
    fn tc_7_1_2_1_positive_octant() {
        let c = world_position_to_tile_coord(Vec3::new(150.0, 0.0, 250.0), 64.0);
        assert_eq!(c.x, 2);
        assert_eq!(c.z, 3);
    }

    /// TC-7.1.2.1 #2
    #[test]
    fn tc_7_1_2_1_negative_quadrant() {
        let c = world_position_to_tile_coord(Vec3::new(-10.0, 0.0, -10.0), 64.0);
        assert_eq!(c.x, -1);
        assert_eq!(c.z, -1);
    }
}
