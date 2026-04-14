/// Two-component vector used for 2D flow fields.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}

impl Vec2 {
    /// Constructs a vector.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns a normalized copy when length is non-zero.
    #[must_use]
    pub fn normalized(self) -> Self {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        if len > f32::EPSILON {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self::new(0.0, 0.0)
        }
    }
}

/// Three-component vector used for 3D flow fields.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

impl Vec3 {
    /// Constructs a vector.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns a normalized copy when length is non-zero.
    #[must_use]
    pub fn normalized(self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > f32::EPSILON {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Self::new(0.0, 0.0, 0.0)
        }
    }
}
