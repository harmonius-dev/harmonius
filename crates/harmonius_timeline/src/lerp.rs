//! Linear interpolation helpers for typed track values.

use crate::vectors::{Quat, Vec2, Vec3};
use crate::Entity;

/// Linear blend used by `Track::sample` for hot-path interpolation.
pub trait Lerp {
    /// Returns `self` blended toward `other` using `t` in the range `[0.0, 1.0]`.
    fn lerp(&self, other: &Self, t: f64) -> Self;
}

impl Lerp for f32 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (*self as f64 + (*other as f64 - *self as f64) * t) as f32
    }
}

impl Lerp for f64 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        *self + (*other - *self) * t
    }
}

impl Lerp for Vec2 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        Vec2::lerp(*self, *other, t as f32)
    }
}

impl Lerp for Vec3 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        Vec3::lerp(*self, *other, t as f32)
    }
}

impl Lerp for Quat {
    fn lerp(&self, other: &Quat, t: f64) -> Self {
        self.slerp(*other, t as f32)
    }
}

impl Lerp for bool {
    fn lerp(&self, other: &bool, t: f64) -> Self {
        if t < 0.5 {
            *self
        } else {
            *other
        }
    }
}

impl Lerp for Entity {
    fn lerp(&self, other: &Entity, t: f64) -> Self {
        if t < 0.5 {
            *self
        } else {
            *other
        }
    }
}
