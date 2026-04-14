//! Minimal linear algebra for contact transforms (no external math crate).

/// Column-major 4×4 transform (translation + rotation only used by tests).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4 {
    /// Elements in column-major order.
    pub cols: [[f32; 4]; 4],
}

impl Mat4 {
    /// Identity matrix.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            cols: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Builds TRS from translation and unit quaternion rotation.
    #[must_use]
    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        let q = rotation;
        let xx = q.x * q.x;
        let yy = q.y * q.y;
        let zz = q.z * q.z;
        let xy = q.x * q.y;
        let xz = q.x * q.z;
        let yz = q.y * q.z;
        let wx = q.w * q.x;
        let wy = q.w * q.y;
        let wz = q.w * q.z;
        let r00 = 1.0 - 2.0 * (yy + zz);
        let r01 = 2.0 * (xy - wz);
        let r02 = 2.0 * (xz + wy);
        let r10 = 2.0 * (xy + wz);
        let r11 = 1.0 - 2.0 * (xx + zz);
        let r12 = 2.0 * (yz - wx);
        let r20 = 2.0 * (xz - wy);
        let r21 = 2.0 * (yz + wx);
        let r22 = 1.0 - 2.0 * (xx + yy);
        Self {
            cols: [
                [r00, r10, r20, 0.0],
                [r01, r11, r21, 0.0],
                [r02, r12, r22, 0.0],
                [translation.x, translation.y, translation.z, 1.0],
            ],
        }
    }

    /// Right-multiplies a column homogeneous vector `(x, y, z, 1)`.
    #[must_use]
    pub fn transform_point3(self, p: Vec3) -> Vec3 {
        let x =
            p.x * self.cols[0][0] + p.y * self.cols[1][0] + p.z * self.cols[2][0] + self.cols[3][0];
        let y =
            p.x * self.cols[0][1] + p.y * self.cols[1][1] + p.z * self.cols[2][1] + self.cols[3][1];
        let z =
            p.x * self.cols[0][2] + p.y * self.cols[1][2] + p.z * self.cols[2][2] + self.cols[3][2];
        Vec3 { x, y, z }
    }
}

/// Unit quaternion (scalar-last: x, y, z, w).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    /// X imaginary component.
    pub x: f32,
    /// Y imaginary component.
    pub y: f32,
    /// Z imaginary component.
    pub z: f32,
    /// Real scalar component.
    pub w: f32,
}

impl Quat {
    /// Identity rotation.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

/// 3-vector used by physics/editor contracts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
    /// Z coordinate.
    pub z: f32,
}

impl Vec3 {
    /// Zero vector.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// Affine transform used for `ContactPoint` local-to-world tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// World-space translation.
    pub translation: Vec3,
    /// World-space rotation.
    pub rotation: Quat,
}

impl Transform {
    /// Identity transform.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            translation: Vec3::zero(),
            rotation: Quat::identity(),
        }
    }

    /// Converts local point to world using rotation + translation.
    #[must_use]
    pub fn transform_point(self, local: Vec3) -> Vec3 {
        Mat4::from_translation_rotation(self.translation, self.rotation).transform_point3(local)
    }
}
