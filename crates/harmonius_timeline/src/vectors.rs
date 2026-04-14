//! Minimal vector types used by timeline sampling without pulling conflicting deps.

/// Two-component vector for easing control points and planar tracks.
#[derive(Clone, Copy, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Vec2 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}

impl Vec2 {
    /// Constructs a vector from components.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Component-wise linear interpolation.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

/// Three-component vector for spatial tracks.
#[derive(Clone, Copy, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

impl Vec3 {
    /// Constructs a vector from components.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Component-wise linear interpolation.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }
}

/// Four-component vector for HDR colors.
#[derive(Clone, Copy, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Vec4 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
    /// W component.
    pub w: f32,
}

impl Vec4 {
    /// Constructs a vector from components.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Component-wise linear interpolation.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
            w: self.w + (other.w - self.w) * t,
        }
    }
}

/// Normalized quaternion for rotation tracks.
#[derive(Clone, Copy, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Quat {
    /// X imaginary component.
    pub x: f32,
    /// Y imaginary component.
    pub y: f32,
    /// Z imaginary component.
    pub z: f32,
    /// Scalar component.
    pub w: f32,
}

impl Quat {
    /// Identity rotation.
    pub const fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    /// Spherical interpolation between unit quaternions.
    pub fn slerp(self, other: Self, t: f32) -> Self {
        let mut dot = self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z;
        let mut b = other;
        if dot < 0.0 {
            dot = -dot;
            b = Self {
                x: -other.x,
                y: -other.y,
                z: -other.z,
                w: -other.w,
            };
        }

        if dot > 0.9995 {
            return Self {
                x: self.x + (b.x - self.x) * t,
                y: self.y + (b.y - self.y) * t,
                z: self.z + (b.z - self.z) * t,
                w: self.w + (b.w - self.w) * t,
            }
            .normalize();
        }

        let theta_0 = dot.clamp(-1.0, 1.0).acos();
        let theta = theta_0 * t;
        let sin_theta = theta.sin();
        let sin_theta_0 = theta_0.sin();

        let s0 = (theta_0 - theta).sin() / sin_theta_0;
        let s1 = sin_theta / sin_theta_0;

        Self {
            x: self.x * s0 + b.x * s1,
            y: self.y * s0 + b.y * s1,
            z: self.z * s0 + b.z * s1,
            w: self.w * s0 + b.w * s1,
        }
        .normalize()
    }

    fn normalize(self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if len <= f32::EPSILON {
            return Self::identity();
        }

        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len,
        }
    }
}
