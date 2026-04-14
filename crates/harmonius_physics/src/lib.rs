//! Advanced physics building blocks for Harmonius (PLAN-physics-advanced).
//!
//! Provides deterministic, test-first implementations of spatial queries,
//! vehicle helpers, destruction, cloth/XPBD, and fluid prototypes.

#![forbid(unsafe_code)]

pub mod destruction;
pub mod entity;
pub mod fluid;
pub mod soft_body;
pub mod spatial;
pub mod vehicle;

pub use entity::{Entity, EntityRef};
pub use spatial::{
    BvhScene, ClosestPointResult, OverlapResult, QueryFilter, RayHit, ShapeCastHit,
};
