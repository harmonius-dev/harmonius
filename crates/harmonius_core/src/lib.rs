//! Shared core types for the Harmonius engine.

#![deny(clippy::all)]

pub mod error;
pub mod math;

pub use error::EngineError;
pub use math::{Mat4, Vec2, Vec3};

#[cfg(test)]
mod tests {
    use super::{Mat4, Vec2, Vec3};

    #[test]
    fn test_glam_reexport_compiles() {
        let _ = Vec2::new(1.0, 2.0);
        let _ = Vec3::new(1.0, 2.0, 3.0);
        let _ = Mat4::IDENTITY;
    }
}
