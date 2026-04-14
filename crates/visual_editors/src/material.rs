//! Material instances and library helpers.

use crate::material_graph::{AssetId, MaterialGraph};

/// Typed parameter values for instances.
#[derive(Debug, Clone, PartialEq)]
pub enum ParamValue {
    Float(f32),
}

#[derive(Debug, Clone)]
pub struct MaterialInstance {
    pub id: AssetId,
    pub parent_material: AssetId,
    overrides: Vec<(String, ParamValue)>,
}

impl MaterialInstance {
    /// Creates an empty instance pointing at a parent material.
    pub fn new(id: AssetId, parent_material: AssetId) -> Self {
        Self {
            id,
            parent_material,
            overrides: Vec::new(),
        }
    }

    /// Sets or replaces an override (sorted insertion for determinism).
    pub fn set_override(&mut self, name: String, value: ParamValue) {
        if let Some(i) = self.overrides.iter().position(|(k, _)| k == &name) {
            self.overrides[i].1 = value;
        } else {
            self.overrides.push((name, value));
            self.overrides.sort_by(|a, b| a.0.cmp(&b.0));
        }
    }

    /// Resolves a parameter with instance precedence over parent defaults.
    pub fn effective_value(&self, name: &str, parent: &MaterialGraph) -> Option<ParamValue> {
        if let Some((_, v)) = self.overrides.iter().find(|(k, _)| k == name) {
            return Some(v.clone());
        }
        parent
            .parameter_values
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, f)| ParamValue::Float(*f))
    }
}

/// Deterministic substring search across material names.
pub fn material_library_search<'a>(names: &'a [&str], query: &str) -> Vec<&'a str> {
    names
        .iter()
        .copied()
        .filter(|n| n.contains(query))
        .collect()
}
