//! Dynamically typed cell values used on the import and tooling path.

use crate::ids::{AssetHandle, Entity, EnumSchemaId, RowRef};
use smol_str::SmolStr;

/// Localized string cell: base value plus optional per-locale overlays.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalizedPack {
    /// Default string when no overlay matches.
    pub base: SmolStr,
    /// Overlay map: locale tag → localized string.
    pub overlays: std::collections::BTreeMap<SmolStr, SmolStr>,
}

/// A dynamically typed value stored in a cell.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// SQL-style null for nullable columns.
    Null,
    Bool(bool),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(SmolStr),
    Enum {
        /// Enum schema id from the middleman registry.
        schema_id: EnumSchemaId,
        /// Variant index within the enum schema.
        variant: u32,
    },
    ForeignKey(RowRef),
    AssetRef(AssetHandle),
    EntityRef(Entity),
    Array(Vec<Value>),
    /// Localized gameplay string with overlays.
    Localized(LocalizedPack),
}

impl Value {
    /// Returns `true` when this value is [`Value::Null`].
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}
