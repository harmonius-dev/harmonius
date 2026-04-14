//! Acoustic materials and table lookup.

use std::collections::HashMap;

use crate::entity::Entity;

/// Per-surface acoustic properties for three frequency bands (low, mid, high).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AcousticMaterial {
    /// Unitless absorption per band.
    pub absorption: [f32; 3],
    /// Decibel transmission loss per band.
    pub transmission_loss_db: [f32; 3],
    /// Scattering coefficient.
    pub scattering: f32,
}

impl AcousticMaterial {
    /// Default stone substitution from the integration failure-modes table.
    pub const DEFAULT_STONE: Self = Self {
        absorption: [0.02, 0.03, 0.04],
        transmission_loss_db: [40.0, 45.0, 50.0],
        scattering: 0.1,
    };

    /// Dense carpet — high high-frequency absorption.
    pub const CARPET: Self = Self {
        absorption: [0.25, 0.45, 0.85],
        transmission_loss_db: [15.0, 28.0, 55.0],
        scattering: 0.35,
    };

    /// Glass-like surface — moderate loss, lower absorption.
    pub const GLASS: Self = Self {
        absorption: [0.02, 0.02, 0.03],
        transmission_loss_db: [18.0, 22.0, 26.0],
        scattering: 0.05,
    };

    /// Low-absorption stone used in positive tests.
    pub const STONE: Self = Self::DEFAULT_STONE;
}

/// Maps hit entities to acoustic materials.
#[derive(Debug, Default, Clone)]
pub struct AcousticMaterialTable {
    entries: HashMap<Entity, AcousticMaterial>,
}

impl AcousticMaterialTable {
    /// Creates an empty table.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Inserts or replaces a material for `entity`.
    pub fn insert(&mut self, entity: Entity, material: AcousticMaterial) {
        self.entries.insert(entity, material);
    }

    /// Looks up a material, returning `None` when missing.
    #[must_use]
    pub fn get(&self, entity: Entity) -> Option<AcousticMaterial> {
        self.entries.get(&entity).copied()
    }

    /// Resolves a material or falls back to [`AcousticMaterial::DEFAULT_STONE`].
    #[must_use]
    pub fn resolve_or_default_stone(&self, entity: Entity) -> AcousticMaterial {
        self.get(entity).unwrap_or(AcousticMaterial::DEFAULT_STONE)
    }
}
