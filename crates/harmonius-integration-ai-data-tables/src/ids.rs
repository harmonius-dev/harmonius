//! Dense identifier newtypes used across the integration surface.

use rkyv_derive::{Archive, Deserialize, Serialize};

/// Dense table index (`TableRegistry` slot).
#[derive(
    Archive,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,
)]
#[repr(transparent)]
pub struct TableId(pub u32);

/// Primary key within a `DataTable`.
#[derive(
    Archive, Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord,
)]
#[repr(transparent)]
pub struct RowId(pub u32);

/// Column handle within a table schema.
#[derive(
    Archive, Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord,
)]
#[repr(transparent)]
pub struct ColumnId(pub u16);
