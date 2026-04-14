//! Schema-driven typed data tables for Harmonius gameplay data.
//!
//! This crate implements the in-memory core of [`DataTable`] storage, validation,
//! secondary indices, cross-table joins, and hot-reload bookkeeping described in
//! `docs/design/data-systems/data-tables.md`.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(missing_docs)]

pub mod binding;
pub mod diagnostics;
pub mod filter;
pub mod formula;
pub mod hot_reload;
pub mod ids;
pub mod import;
pub mod index;
pub mod join;
pub mod registry;
pub mod row;
pub mod schema;
pub mod table;
pub mod validation;
pub mod value;

pub use binding::{
    binding_snapshot, populate_transform2d, BindingSnapshot, DatabaseRow, Transform2D,
};
pub use diagnostics::{ValidationError, ValidationSeverity};
pub use filter::{ColumnPredicate, FilterExpr};
pub use formula::formula_item_dps;
pub use hot_reload::{
    HotReloadError, HotReloadEvent, HotReloadStack, TableReloaded, ValidationFailed,
};
pub use ids::{AssetHandle, ColumnId, Entity, EnumSchemaId, LocaleTag, RowId, RowRef, TableId};
pub use import::{import_table_from_ron, ImportError};
pub use index::IndexType;
pub use join::{join_query, JoinRow};
pub use registry::TableRegistry;
pub use row::Row;
pub use schema::{ColumnConstraint, ColumnDef, ColumnType, CustomRule, TableSchema};
pub use table::{detect_cycle, resolve_inherited, DataTable, DataTableError};
pub use validation::{validate_all, validate_table};
pub use value::{LocalizedPack, Value};
