//! AI behavior ↔ data tables integration (IR-2.1.x).
//!
//! Implements the contracts from `docs/design/integration/ai-data-tables.md`:
//! table-backed BT lookups, utility considerations, GOAP baked costs, blackboard bindings,
//! `TableReloaded` cache invalidation, and trace toggles.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod bindings_system;
mod blackboard;
mod bt;
mod cache;
mod component_store;
mod events;
mod goap;
mod ids;
mod table;
mod trace;
mod utility;
mod world;

pub use bindings_system::{BlackboardBinding, BlackboardBindings, BlackboardTableBindingSystem};
pub use blackboard::{Blackboard, BlackboardKey, BlackboardValue};
pub use bt::BtTableLookup;
pub use cache::{AiTableCache, CachedValue, CachedValueKind, ColumnError};
pub use component_store::ComponentStore;
pub use events::{EntityEventQueue, TableReloaded};
pub use goap::{
    bake_goap_action_cost, bake_goap_action_from_formula, read_goap_planning_cost, BakeError,
    FormulaId, GoapAction,
};
pub use ids::{ColumnId, RowId, TableId};
pub use table::{ColumnSchema, DataTable, DatabaseRow, Row, TableRegistry, Value};
pub use trace::AiDataTraceFlag;
pub use utility::{ResponseCurve, TableColumnConsideration};
pub use world::{EntityId, World};
