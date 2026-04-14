//! Two-dimensional transforms and world matrices.

use glam::{Mat3, Vec2, Vec3Swizzles};

/// Local-space 2D transform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform2D {
    /// Translation relative to the parent.
    pub position: Vec2,
    /// Rotation in radians relative to the parent.
    pub rotation: f32,
    /// Non-uniform scale relative to the parent.
    pub scale: Vec2,
}

impl Transform2D {
    /// Identity transform.
    pub const IDENTITY: Self = Self {
        position: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
    };

    /// Translation-only transform.
    #[must_use]
    pub const fn from_position(position: Vec2) -> Self {
        Self {
            position,
            ..Self::IDENTITY
        }
    }

    /// Rotation-only transform.
    #[must_use]
    pub const fn from_rotation(rotation: f32) -> Self {
        Self {
            rotation,
            ..Self::IDENTITY
        }
    }

    /// Computes the local `T * R * S` matrix.
    #[must_use]
    pub fn local_matrix(&self) -> Mat3 {
        Mat3::from_scale_angle_translation(self.scale, self.rotation, self.position)
    }

    /// Composes this local transform against a parent's world matrix.
    #[must_use]
    pub fn compose(&self, parent_global: &GlobalTransform2D) -> GlobalTransform2D {
        GlobalTransform2D {
            matrix: parent_global.matrix * self.local_matrix(),
            last_write_epoch: 0,
        }
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// World-space 2D transform computed by propagation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlobalTransform2D {
    /// Column-major 2D affine matrix.
    pub matrix: Mat3,
    pub(crate) last_write_epoch: u64,
}

impl GlobalTransform2D {
    /// Identity world transform.
    pub const IDENTITY: Self = Self {
        matrix: Mat3::IDENTITY,
        last_write_epoch: 0,
    };

    /// Wraps an existing matrix.
    #[must_use]
    pub const fn from_matrix(matrix: Mat3) -> Self {
        Self {
            matrix,
            last_write_epoch: 0,
        }
    }

    /// World-space translation (Mat3 affine column).
    #[must_use]
    pub fn translation(&self) -> Vec2 {
        self.matrix.z_axis.xy()
    }

    /// World-space rotation in radians.
    #[must_use]
    pub fn rotation(&self) -> f32 {
        self.matrix.x_axis.y.atan2(self.matrix.x_axis.x)
    }

    /// World-space scale (axis lengths).
    #[must_use]
    pub fn scale(&self) -> Vec2 {
        Vec2::new(self.matrix.x_axis.length(), self.matrix.y_axis.length())
    }

    /// Transforms a point from local to world space.
    #[must_use]
    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        (self.matrix * point.extend(1.0)).xy()
    }

    /// Computes the inverse world transform.
    #[must_use]
    pub fn inverse(&self) -> Self {
        Self {
            matrix: self.matrix.inverse(),
            last_write_epoch: self.last_write_epoch,
        }
    }

    /// Propagation epoch of the last write (dirty tests).
    #[must_use]
    pub const fn last_write_epoch(&self) -> u64 {
        self.last_write_epoch
    }
}

impl Default for GlobalTransform2D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Previous-frame 2D world transform for interpolation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreviousGlobalTransform2D {
    /// Stored matrix from the prior propagation pass.
    pub matrix: Mat3,
}

impl PreviousGlobalTransform2D {
    /// Identity placeholder.
    pub const IDENTITY: Self = Self {
        matrix: Mat3::IDENTITY,
    };
}

impl Default for PreviousGlobalTransform2D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_transform2d_identity() {
        assert_eq!(Transform2D::IDENTITY.local_matrix(), Mat3::IDENTITY);
    }

    #[test]
    fn tc_transform2d_compose() {
        let local = Transform2D {
            position: Vec2::new(1.0, 2.0),
            rotation: 0.25,
            scale: Vec2::new(2.0, 3.0),
        };
        let parent = GlobalTransform2D::IDENTITY;
        let world = local.compose(&parent);
        let diff = world.matrix - local.local_matrix();
        assert!(diff.to_cols_array().iter().all(|value| value.abs() < 1e-5));
    }

    #[test]
    fn tc_global_transform2d_decompose() {
        let local = Transform2D {
            position: Vec2::new(3.0, -1.0),
            rotation: 1.1,
            scale: Vec2::new(2.0, 2.0),
        };
        let world = GlobalTransform2D::from_matrix(local.local_matrix());
        assert!((world.translation() - local.position).length() < 1e-4);
        assert!((world.rotation() - local.rotation).abs() < 1e-4);
        assert!((world.scale() - local.scale).length() < 1e-4);
    }
}
