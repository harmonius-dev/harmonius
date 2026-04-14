//! Default physics material constants used when assets are not yet resolved (FM-7).

/// Default Coulomb friction when a [`crate::PhysicsMaterialHandle`] is unresolved.
pub const DEFAULT_PHYSICS_MATERIAL_FRICTION: f32 = 0.5;

/// Default restitution when a [`crate::PhysicsMaterialHandle`] is unresolved.
pub const DEFAULT_PHYSICS_MATERIAL_RESTITUTION: f32 = 0.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_8_e7_default_material_constants() {
        assert!((DEFAULT_PHYSICS_MATERIAL_FRICTION - 0.5).abs() < f32::EPSILON);
        assert!((DEFAULT_PHYSICS_MATERIAL_RESTITUTION - 0.0).abs() < f32::EPSILON);
    }
}
