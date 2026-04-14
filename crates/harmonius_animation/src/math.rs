//! Minimal 3D vector helpers (no external math crate).

/// Three-component vector used by procedural solvers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, o: Self) -> Self {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, o: Self) -> Self {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Vec3 {
    /// Origin.
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// Constructs a vector from components.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Dot product.
    pub fn dot(self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    /// Cross product.
    pub fn cross(self, o: Self) -> Self {
        Self {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }

    /// Euclidean length.
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Normalizes in place, returning zero vector if length is tiny.
    pub fn normalized(self) -> Self {
        let l = self.length();
        if l < 1e-8 {
            Self::ZERO
        } else {
            self * (1.0 / l)
        }
    }

    /// Linear interpolation.
    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        a * (1.0 - t) + b * t
    }
}

/// Returns true if `a` and `b` are within `eps` in each component.
pub fn approx_eq_vec3(a: Vec3, b: Vec3, eps: f32) -> bool {
    (a.x - b.x).abs() < eps && (a.y - b.y).abs() < eps && (a.z - b.z).abs() < eps
}

/// Returns true if `a` is within `eps` of `b`.
pub fn approx_eq_f32(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}

/// Clamps `x` into `[lo, hi]`.
pub fn clamp_f32(x: f32, lo: f32, hi: f32) -> f32 {
    x.max(lo).min(hi)
}
