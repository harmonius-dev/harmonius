//! `SurfaceType` tagging and `SurfaceEffectMap` (DashMap-backed).

use std::fmt;

use dashmap::DashMap;

use crate::registry::RowRef;

/// Surface tags from the physics foundation enum (integration IR-2.6.5).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SurfaceType {
    /// Default surface.
    Default,
    /// Metal surface.
    Metal,
    /// Wood surface.
    Wood,
    /// Stone surface.
    Stone,
    /// Dirt surface.
    Dirt,
    /// Grass surface.
    Grass,
    /// Ice surface.
    Ice,
    /// Rubber surface.
    Rubber,
    /// Water surface.
    Water,
    /// Sand surface.
    Sand,
}

/// Maps [`SurfaceType`] to authored table row ids for effect lookup.
#[derive(Debug)]
pub struct SurfaceEffectMap {
    /// Concurrent map used on the collision hot path (design: DashMap).
    pub entries: DashMap<SurfaceType, RowRef>,
}

impl SurfaceEffectMap {
    /// Builds an empty map (populated at load time).
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    /// Inserts or replaces a mapping.
    pub fn insert(&self, surface: SurfaceType, row: RowRef) {
        self.entries.insert(surface, row);
    }

    /// Looks up a row reference for a surface.
    #[must_use]
    pub fn get(&self, surface: SurfaceType) -> Option<RowRef> {
        self.entries.get(&surface).map(|r| *r)
    }
}

impl Default for SurfaceEffectMap {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SurfaceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
