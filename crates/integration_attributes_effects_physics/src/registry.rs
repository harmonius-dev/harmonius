//! Table row resolution for effect definitions.

use std::collections::HashMap;

/// Stable row id inside a data table archive.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RowRef(pub u32);

/// Runtime view of an effect row resolved through [`TableRegistry`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectDefinition {
    /// Stable effect id used in tests and telemetry.
    pub id: u32,
}

/// Resolves [`RowRef`] to [`EffectDefinition`] (design: single registry entry point).
#[derive(Debug, Default)]
pub struct TableRegistry {
    effects: HashMap<RowRef, EffectDefinition>,
}

impl TableRegistry {
    /// Empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            effects: HashMap::new(),
        }
    }

    /// Registers an effect row.
    pub fn register_effect(&mut self, row: RowRef, def: EffectDefinition) {
        self.effects.insert(row, def);
    }

    /// Looks up an effect definition.
    #[must_use]
    pub fn effect(&self, row: RowRef) -> Option<&EffectDefinition> {
        self.effects.get(&row)
    }
}
