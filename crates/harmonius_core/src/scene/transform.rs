//! Three-dimensional transforms and world matrices.

use glam::{Mat4, Quat, Vec3};
use serde::{Deserialize, Serialize};

/// Local-space transform relative to a parent (or the world origin when no parent exists).
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Transform {
    /// Position relative to the parent.
    pub translation: Vec3,
    /// Orientation relative to the parent.
    pub rotation: Quat,
    /// Non-uniform scale relative to the parent.
    pub scale: Vec3,
}

impl Transform {
    /// Identity transform.
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    /// Builds a translation-only transform.
    #[must_use]
    pub const fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            ..Self::IDENTITY
        }
    }

    /// Builds a rotation-only transform.
    #[must_use]
    pub const fn from_rotation(rotation: Quat) -> Self {
        Self {
            rotation,
            ..Self::IDENTITY
        }
    }

    /// Builds a scale-only transform.
    #[must_use]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Self::IDENTITY
        }
    }

    /// Replaces translation.
    #[must_use]
    pub const fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    /// Replaces rotation.
    #[must_use]
    pub const fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    /// Replaces scale.
    #[must_use]
    pub const fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    /// Computes the local TRS matrix (`T * R * S`).
    #[must_use]
    pub fn local_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// Local forward axis (`-Z`) in a right-handed coordinate system.
    #[must_use]
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    /// Local right axis (`+X`).
    #[must_use]
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    /// Local up axis (`+Y`).
    #[must_use]
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    /// Composes this local transform against a parent's world matrix.
    #[must_use]
    pub fn compose(&self, parent_global: &GlobalTransform) -> GlobalTransform {
        GlobalTransform {
            matrix: parent_global.matrix * self.local_matrix(),
            last_write_epoch: 0,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// World-space transform computed by propagation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlobalTransform {
    /// World-space affine transform.
    pub matrix: Mat4,
    pub(crate) last_write_epoch: u64,
}

impl GlobalTransform {
    /// Identity world transform.
    pub const IDENTITY: Self = Self {
        matrix: Mat4::IDENTITY,
        last_write_epoch: 0,
    };

    /// Wraps an existing matrix.
    #[must_use]
    pub const fn from_matrix(matrix: Mat4) -> Self {
        Self {
            matrix,
            last_write_epoch: 0,
        }
    }

    /// World-space translation column.
    #[must_use]
    pub fn translation(&self) -> Vec3 {
        self.matrix.w_axis.truncate()
    }

    /// World-space rotation (best-effort decomposition).
    #[must_use]
    pub fn rotation(&self) -> Quat {
        let (_, rotation, _) = self.matrix.to_scale_rotation_translation();
        rotation
    }

    /// World-space scale (best-effort decomposition).
    #[must_use]
    pub fn scale(&self) -> Vec3 {
        let (scale, _, _) = self.matrix.to_scale_rotation_translation();
        scale
    }

    /// Transforms a point from local space to world space.
    #[must_use]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.matrix.transform_point3(point)
    }

    /// Transforms a direction vector, ignoring translation.
    #[must_use]
    pub fn transform_direction(&self, direction: Vec3) -> Vec3 {
        self.matrix.transform_vector3(direction)
    }

    /// Computes the inverse world transform.
    #[must_use]
    pub fn inverse(&self) -> Self {
        Self {
            matrix: self.matrix.inverse(),
            last_write_epoch: self.last_write_epoch,
        }
    }

    /// World forward axis (`-Z`).
    #[must_use]
    pub fn forward(&self) -> Vec3 {
        self.rotation() * Vec3::NEG_Z
    }

    /// Epoch of the last propagation write (used for dirty tests).
    #[must_use]
    pub const fn last_write_epoch(&self) -> u64 {
        self.last_write_epoch
    }
}

impl Default for GlobalTransform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Previous-frame world transform used for render interpolation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreviousGlobalTransform {
    /// Stored matrix from the prior propagation pass.
    pub matrix: Mat4,
}

impl PreviousGlobalTransform {
    /// Identity placeholder.
    pub const IDENTITY: Self = Self {
        matrix: Mat4::IDENTITY,
    };
}

impl Default for PreviousGlobalTransform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Interpolated world transform consumed by rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InterpolatedTransform(pub Mat4);

impl InterpolatedTransform {
    /// Identity interpolation target.
    pub const IDENTITY: Self = Self(Mat4::IDENTITY);

    /// Linearly interpolates matrix elements (not SLERP); matches design examples.
    #[must_use]
    pub fn from_lerp(previous: Mat4, current: Mat4, alpha: f32) -> Self {
        Self(Mat4::from_cols(
            previous.x_axis.lerp(current.x_axis, alpha),
            previous.y_axis.lerp(current.y_axis, alpha),
            previous.z_axis.lerp(current.z_axis, alpha),
            previous.w_axis.lerp(current.w_axis, alpha),
        ))
    }

    /// Borrows the underlying matrix.
    #[must_use]
    pub const fn as_mat4(&self) -> &Mat4 {
        &self.0
    }
}

impl Default for InterpolatedTransform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_1_2_4_1_transform_identity() {
        assert_eq!(Transform::IDENTITY.local_matrix(), Mat4::IDENTITY);
    }

    #[test]
    fn tc_1_2_4_2_transform_compose_trs() {
        let translation = Vec3::new(1.0, 2.0, 3.0);
        let rotation = Quat::from_rotation_y(f32::to_radians(45.0));
        let scale = Vec3::splat(2.0);
        let transform = Transform {
            translation,
            rotation,
            scale,
        };
        let expected = Mat4::from_translation(translation)
            * Mat4::from_quat(rotation)
            * Mat4::from_scale(scale);
        let diff = transform.local_matrix() - expected;
        assert!(diff.to_cols_array().iter().all(|value| value.abs() < 1e-5));
    }

    #[test]
    fn interpolated_transform_lerp_smoke() {
        let blended = InterpolatedTransform::from_lerp(Mat4::IDENTITY, Mat4::IDENTITY, 0.5);
        let diff = *blended.as_mat4() - Mat4::IDENTITY;
        assert!(diff
            .to_cols_array()
            .iter()
            .all(|value: &f32| value.abs() < 1e-6));
    }

    #[test]
    fn tc_1_2_4_3_global_transform_decompose() {
        let translation = Vec3::new(5.0, 0.0, 0.0);
        let rotation = Quat::from_rotation_z(f32::to_radians(90.0));
        let scale = Vec3::ONE;
        let local = Transform {
            translation,
            rotation,
            scale,
        };
        let global = GlobalTransform::from_matrix(local.local_matrix());
        assert!((global.translation() - translation).length() < 1e-4);
        let decomposed_rotation = global.rotation();
        let angle = decomposed_rotation.angle_between(rotation);
        assert!(angle < 1e-3 || (std::f32::consts::PI * 2.0 - angle).abs() < 1e-3);
        assert!((global.scale() - scale).length() < 1e-3);
    }
}
