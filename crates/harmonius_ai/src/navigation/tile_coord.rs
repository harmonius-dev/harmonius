//! Mapping world positions to streaming tile coordinates.

use glam::Vec3;

use super::types::TileCoord;

/// Converts a world-space position to integer tile indices using `tile_size` meters per edge.
///
/// Uses `floor` on each axis so negative coordinates map to negative tile indices
/// (TC-7.1.2.1).
#[must_use]
pub fn world_position_to_tile_coord(position: Vec3, tile_size: f32) -> TileCoord {
    debug_assert!(tile_size > 0.0);
    TileCoord {
        x: (position.x / tile_size).floor() as i32,
        z: (position.z / tile_size).floor() as i32,
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
