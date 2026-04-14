use glam::{Vec2, Vec3, Vec4};

/// Axis-aligned bounding box in 3D.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    /// Minimum corner.
    pub min: Vec3,
    /// Maximum corner.
    pub max: Vec3,
}

/// Classification of an AABB against a frustum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrustumTest {
    /// Fully outside the frustum.
    Outside,
    /// Straddles the frustum boundary.
    Intersecting,
    /// Fully inside the frustum.
    Inside,
}

/// Sphere used for overlap tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    /// Center in world space.
    pub center: Vec3,
    /// Radius in world units.
    pub radius: f32,
}

impl Aabb {
    /// Creates an AABB from explicit bounds.
    #[must_use]
    pub const fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Builds an AABB from a center and half-extents.
    #[must_use]
    pub fn from_center_extents(center: Vec3, half_extents: Vec3) -> Self {
        Self {
            min: center - half_extents,
            max: center + half_extents,
        }
    }

    /// Returns the center point.
    #[must_use]
    pub fn center(self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// Returns full extents along each axis.
    #[must_use]
    pub fn extents(self) -> Vec3 {
        self.max - self.min
    }

    /// Returns positive half-extents.
    #[must_use]
    pub fn half_extents(self) -> Vec3 {
        self.extents() * 0.5
    }

    /// Surface area of the box (zero degenerate faces still contribute positive area for non-flat
    /// boxes).
    #[must_use]
    pub fn surface_area(self) -> f32 {
        let e = self.extents();
        2.0 * (e.x * e.y + e.x * e.z + e.y * e.z)
    }

    /// Axis-aligned volume.
    #[must_use]
    pub fn volume(self) -> f32 {
        let e = self.extents();
        e.x * e.y * e.z
    }

    /// Merges two AABBs into the smallest enclosing AABB.
    #[must_use]
    pub fn merged(self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Point-in-box test with inclusive bounds.
    #[must_use]
    pub fn contains_point(self, p: Vec3) -> bool {
        p.cmpge(self.min).all() && p.cmple(self.max).all()
    }

    /// Overlap test between two AABBs.
    #[must_use]
    pub fn intersects(self, other: &Self) -> bool {
        self.min.cmple(other.max).all() && self.max.cmpge(other.min).all()
    }

    /// Ray intersection using pre-inverted direction components.
    #[must_use]
    pub fn intersects_ray(self, origin: Vec3, inv_dir: Vec3, t_max: f32) -> Option<f32> {
        let mut t_min = 0.0_f32;
        let mut t_max_v = t_max;

        for axis in 0..3usize {
            let o = origin[axis];
            let inv = inv_dir[axis];
            let min_b = self.min[axis];
            let max_b = self.max[axis];

            if inv.is_finite() && inv.abs() > f32::EPSILON {
                let t0 = (min_b - o) * inv;
                let t1 = (max_b - o) * inv;
                let (t_near, t_far) = if t0 <= t1 { (t0, t1) } else { (t1, t0) };
                t_min = t_min.max(t_near);
                t_max_v = t_max_v.min(t_far);
                if t_min > t_max_v {
                    return None;
                }
            } else if o < min_b || o > max_b {
                return None;
            }
        }

        if t_min.is_finite() {
            Some(t_min.max(0.0))
        } else {
            None
        }
    }

    /// Conservative sphere overlap against this AABB.
    #[must_use]
    pub fn intersects_sphere(self, center: Vec3, radius: f32) -> bool {
        let closest = self.min.max(center.min(self.max));
        let d = center - closest;
        d.length_squared() <= radius * radius
    }

    /// Classifies this AABB against six inward-facing frustum planes where `distance <= 0`
    /// means inside along that plane's half-space.
    #[must_use]
    pub fn intersects_frustum(self, planes: &[Vec4; 6]) -> FrustumTest {
        let corners = self.corners();
        let mut all_corners_inside_all_planes = true;

        for plane in planes {
            let mut all_outside_this_plane = true;
            for c in corners {
                let distance = plane.dot(c.extend(1.0));
                if distance <= 0.0 {
                    all_outside_this_plane = false;
                } else {
                    all_corners_inside_all_planes = false;
                }
            }
            if all_outside_this_plane {
                return FrustumTest::Outside;
            }
        }

        if all_corners_inside_all_planes {
            FrustumTest::Inside
        } else {
            FrustumTest::Intersecting
        }
    }

    /// Expands the AABB uniformly by `margin` along every axis.
    #[must_use]
    pub fn expanded(self, margin: f32) -> Self {
        let m = Vec3::splat(margin);
        Self {
            min: self.min - m,
            max: self.max + m,
        }
    }

    /// Returns true when `inner` is fully contained.
    #[must_use]
    pub fn contains_aabb(self, inner: &Self) -> bool {
        inner.min.cmpge(self.min).all() && inner.max.cmple(self.max).all()
    }

    fn corners(self) -> [Vec3; 8] {
        let mn = self.min;
        let mx = self.max;
        [
            Vec3::new(mn.x, mn.y, mn.z),
            Vec3::new(mx.x, mn.y, mn.z),
            Vec3::new(mn.x, mx.y, mn.z),
            Vec3::new(mx.x, mx.y, mn.z),
            Vec3::new(mn.x, mn.y, mx.z),
            Vec3::new(mx.x, mn.y, mx.z),
            Vec3::new(mn.x, mx.y, mx.z),
            Vec3::new(mx.x, mx.y, mx.z),
        ]
    }
}

/// Axis-aligned bounding box in 2D.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb2D {
    /// Minimum corner.
    pub min: Vec2,
    /// Maximum corner.
    pub max: Vec2,
}

impl Aabb2D {
    /// Creates a 2D AABB from explicit bounds.
    #[must_use]
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Builds a 2D AABB from a center and half-extents.
    #[must_use]
    pub fn from_center_extents(center: Vec2, half_extents: Vec2) -> Self {
        Self {
            min: center - half_extents,
            max: center + half_extents,
        }
    }

    /// Returns the center point.
    #[must_use]
    pub fn center(self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    /// Returns full extents along each axis.
    #[must_use]
    pub fn extents(self) -> Vec2 {
        self.max - self.min
    }

    /// Positive half-extents.
    #[must_use]
    pub fn half_extents(self) -> Vec2 {
        self.extents() * 0.5
    }

    /// Area of the rectangle.
    #[must_use]
    pub fn area(self) -> f32 {
        let e = self.extents();
        e.x * e.y
    }

    /// Merges two 2D AABBs.
    #[must_use]
    pub fn merged(self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Point containment with inclusive bounds.
    #[must_use]
    pub fn contains_point(self, p: Vec2) -> bool {
        p.cmpge(self.min).all() && p.cmple(self.max).all()
    }

    /// Overlap test between two 2D AABBs.
    #[must_use]
    pub fn intersects(self, other: &Self) -> bool {
        self.min.cmple(other.max).all() && self.max.cmpge(other.min).all()
    }

    /// Ray intersection in the XY plane using inverted direction components.
    #[must_use]
    pub fn intersects_ray_2d(self, origin: Vec2, inv_dir: Vec2, t_max: f32) -> Option<f32> {
        let mut t_min = 0.0_f32;
        let mut t_max_v = t_max;

        for axis in 0..2usize {
            let o = origin[axis];
            let inv = inv_dir[axis];
            let min_b = self.min[axis];
            let max_b = self.max[axis];

            if inv.is_finite() && inv.abs() > f32::EPSILON {
                let t0 = (min_b - o) * inv;
                let t1 = (max_b - o) * inv;
                let (t_near, t_far) = if t0 <= t1 { (t0, t1) } else { (t1, t0) };
                t_min = t_min.max(t_near);
                t_max_v = t_max_v.min(t_far);
                if t_min > t_max_v {
                    return None;
                }
            } else if o < min_b || o > max_b {
                return None;
            }
        }

        if t_min.is_finite() {
            Some(t_min.max(0.0))
        } else {
            None
        }
    }

    /// Circle vs AABB overlap.
    #[must_use]
    pub fn intersects_circle(self, center: Vec2, radius: f32) -> bool {
        let closest = self.min.max(center.min(self.max));
        let d = center - closest;
        d.length_squared() <= radius * radius
    }

    /// Uniform expansion by `margin`.
    #[must_use]
    pub fn expanded(self, margin: f32) -> Self {
        let m = Vec2::splat(margin);
        Self {
            min: self.min - m,
            max: self.max + m,
        }
    }
}
