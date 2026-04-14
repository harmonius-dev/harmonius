//! Axis-aligned bounding boxes (`Aabb`).

use crate::math::Vec3;

/// Axis-aligned bounding box in world space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    /// Minimum corner (inclusive).
    pub min: Vec3,
    /// Maximum corner (inclusive).
    pub max: Vec3,
}

impl Aabb {
    /// Unit cube centered around the origin with half-extent `0.5`.
    #[must_use]
    pub fn unit_cube_at_origin() -> Self {
        Self {
            min: Vec3::new(-0.5, -0.5, -0.5),
            max: Vec3::new(0.5, 0.5, 0.5),
        }
    }

    /// Unit cube translated so its minimum corner sits at `min_corner`.
    #[must_use]
    pub fn unit_cube_with_min_corner(min_corner: Vec3) -> Self {
        Self {
            min: min_corner,
            max: Vec3::new(min_corner.x + 1.0, min_corner.y + 1.0, min_corner.z + 1.0),
        }
    }

    /// Returns `true` when the two boxes overlap with positive volume (touching edges count).
    #[must_use]
    pub fn overlaps(self, other: Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    /// Returns the intersection AABB when overlapping; otherwise returns a degenerate box.
    #[must_use]
    pub fn intersection(self, other: Self) -> Self {
        Self {
            min: Vec3::new(
                self.min.x.max(other.min.x),
                self.min.y.max(other.min.y),
                self.min.z.max(other.min.z),
            ),
            max: Vec3::new(
                self.max.x.min(other.max.x),
                self.max.y.min(other.max.y),
                self.max.z.min(other.max.z),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_9_1_u1_overlap_two_unit_cubes() {
        let a = Aabb::unit_cube_with_min_corner(Vec3::new(-0.5, -0.5, -0.5));
        let b = Aabb::unit_cube_with_min_corner(Vec3::new(0.0, -0.5, -0.5));
        assert!(a.overlaps(b));
    }

    #[test]
    fn tc_ir_3_9_1_u2_overlap_disjoint_cubes() {
        let a = Aabb::unit_cube_with_min_corner(Vec3::new(-0.5, -0.5, -0.5));
        let b = Aabb::unit_cube_with_min_corner(Vec3::new(100.0, -0.5, -0.5));
        assert!(!a.overlaps(b));
    }
}
