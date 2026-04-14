//! Stable identifiers for tables, rows, and columns.

use smol_str::SmolStr;

/// Dense table identifier used by `TableRegistry`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TableId(pub u32);

/// Primary key for a row within a table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RowId(pub u64);

/// Column index within a schema (0-based).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColumnId(pub u16);

/// Reference to a row in another (or same) table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RowRef {
    /// Target table for the foreign key.
    pub table: TableId,
    /// Referenced row primary key.
    pub row: RowId,
}

/// Opaque enum schema handle from asset pipeline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnumSchemaId(pub u32);

/// Stable asset handle resolved by the asset system.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AssetHandle(pub u64);

/// ECS entity identifier (engine-owned).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Entity(pub u64);

/// ECS entity tag for binding tests.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EntityTag(pub u64);

/// Locale tag (BCP-47 style) for localized strings.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocaleTag(pub SmolStr);
