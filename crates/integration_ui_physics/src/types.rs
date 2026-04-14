//! Shared value types for UI ↔ physics integration.

/// Stable entity identifier used in pick results.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity(pub u64);

/// Camera slot referenced by pick and projection requests.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CameraId(pub u32);

/// Bitmask of collision layers a ray may interact with.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CollisionMask(pub u32);

/// Default world layer bit.
pub const L_DEFAULT: u32 = 1 << 0;
/// Enemy layer bit (used by mask exclusion tests).
pub const L_ENEMY: u32 = 1 << 1;

/// 2D vector (NDC or screen pixels).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}

/// 3D vector in world space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

impl Vec3 {
    /// Zero vector.
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// Dot product.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Euclidean length.
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Unit vector, or zero if length is zero.
    pub fn normalize(self) -> Self {
        let l = self.length();
        if l <= 1e-8 {
            Self::ZERO
        } else {
            Self {
                x: self.x / l,
                y: self.y / l,
                z: self.z / l,
            }
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, s: f32) -> Self::Output {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

/// Localized string handle carried by tooltip templates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalizedStringId(pub u32);

/// Cursor pick request (CH-27 payload).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldPickRequest {
    /// Correlates async results with HUD state.
    pub request_id: u64,
    /// Active camera for NDC → ray.
    pub camera: CameraId,
    /// Cursor in normalized device coordinates.
    pub cursor_ndc: Vec2,
    /// Layer filter for the ray.
    pub ray_mask: CollisionMask,
    /// Maximum ray length in world units.
    pub max_distance: f32,
}

/// First-hit pick result returned to UI.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldPickResult {
    /// Echo of `WorldPickRequest::request_id`.
    pub request_id: u64,
    /// Hit entity, if any.
    pub entity: Option<Entity>,
    /// World-space hit position.
    pub world_position: Vec3,
    /// Distance from ray origin along the ray.
    pub distance: f32,
    /// World-space hit normal (toward the ray origin when possible).
    pub normal: Vec3,
}

/// Bit flags for world projection (`ProjectFlags::NEED_VISIBILITY`, …).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectFlags(pub u32);

impl ProjectFlags {
    /// No optional behavior.
    pub const NONE: Self = Self(0);
    /// Compare projected depth against latched depth readback.
    pub const NEED_VISIBILITY: Self = Self(1);
    /// Clamp resulting screen coordinates to the viewport rectangle.
    pub const CLAMP_TO_SCREEN: Self = Self(2);
    /// Keep anchors that project outside the viewport (`visible` may still be false).
    pub const ALLOW_OFF_SCREEN: Self = Self(4);

    /// Returns true when every set bit in `mask` is set on `self`.
    pub fn has(self, mask: ProjectFlags) -> bool {
        self.0 & mask.0 == mask.0
    }
}

impl std::ops::BitOr for ProjectFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// World anchor projection request.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldProjectRequest {
    /// Camera used for projection.
    pub camera: CameraId,
    /// World-space anchor.
    pub world_position: Vec3,
    /// Projection option bits.
    pub flags: ProjectFlags,
}

/// Screen-space projection result for one anchor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldProjectResult {
    /// Pixel coordinates (origin top-left).
    pub screen_position: Vec2,
    /// Normalized device depth after projection (0 = near, 1 = far for tests).
    pub ndc_depth: f32,
    /// False when culled by occlusion or off-screen rules.
    pub visible: bool,
}

/// HUD reticle snap state derived from pick + aim cone scan.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReticleSnap {
    /// Aimable entity under the reticle, if any.
    pub target: Option<Entity>,
    /// Screen-space offset from reticle center (pixels).
    pub offset: Vec2,
    /// Confidence in \[0, 1\] decreasing with distance.
    pub confidence: f32,
}
