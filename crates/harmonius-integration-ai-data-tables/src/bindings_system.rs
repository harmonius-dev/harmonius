//! Blackboard ↔ data column bindings and reload draining.

use rkyv_derive::{Archive, Deserialize, Serialize};

use crate::blackboard::{Blackboard, BlackboardKey, BlackboardValue};
use crate::cache::{AiTableCache, CachedValue};
use crate::component_store::ComponentStore;
use crate::events::{EntityEventQueue, TableReloaded};
use crate::ids::ColumnId;
use crate::table::{DatabaseRow, TableRegistry, Value};
use crate::trace::AiDataTraceFlag;

/// Integration-only system object (zero-sized).
#[derive(Debug, Default)]
pub struct BlackboardTableBindingSystem;

/// One blackboard key bound to a column.
#[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BlackboardBinding {
    /// Target blackboard key.
    pub key: BlackboardKey,
    /// Source column.
    pub column: ColumnId,
}

/// Sorted binding list (`ColumnId` order, binary search per SC-2).
#[derive(Archive, Serialize, Deserialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct BlackboardBindings {
    /// Bindings sorted by `column` id.
    pub bindings: Vec<BlackboardBinding>,
}

impl BlackboardBindings {
    /// Binary-search index of `column` in `bindings`.
    pub fn find(&self, column: ColumnId) -> Option<usize> {
        self.bindings
            .binary_search_by_key(&column, |b| b.column)
            .ok()
    }
}

impl BlackboardTableBindingSystem {
    /// Drains reload events and invalidates affected `AiTableCache` rows.
    pub fn drain_reload_events(
        events: &mut EntityEventQueue<TableReloaded>,
        caches: &mut ComponentStore<AiTableCache>,
    ) {
        for ev in events.drain() {
            caches.invalidate_table(ev.table, ev.new_version);
        }
    }

    /// Applies all bindings for a spawned entity.
    pub fn bind_entity(
        registry: &TableRegistry,
        db_row: &DatabaseRow,
        bindings: &BlackboardBindings,
        blackboard: &mut Blackboard,
        cache: &mut AiTableCache,
        trace: &AiDataTraceFlag,
    ) {
        cache.bound_table = db_row.table;
        let Some(table) = registry.get(db_row.table) else {
            trace.emit(format!("missing table {:?}", db_row.table));
            for b in &bindings.bindings {
                blackboard.set(b.key, BlackboardValue::None);
            }
            return;
        };

        if table.get(db_row.row).is_none() {
            trace.emit(format!("missing row {:?} {:?}", db_row.table, db_row.row));
            for b in &bindings.bindings {
                blackboard.set(b.key, BlackboardValue::None);
            }
            return;
        }

        for binding in &bindings.bindings {
            apply_binding(
                db_row,
                table,
                binding.column,
                binding.key,
                blackboard,
                cache,
                trace,
            );
        }
        cache.version = table.version();
    }

    /// Rebinds after a reload for entities referencing `event.table`.
    pub fn rebind_on_reload(
        registry: &TableRegistry,
        event: &TableReloaded,
        db_row: &DatabaseRow,
        bindings: &BlackboardBindings,
        blackboard: &mut Blackboard,
        cache: &mut AiTableCache,
        trace: &AiDataTraceFlag,
    ) {
        if db_row.table != event.table {
            return;
        }
        Self::bind_entity(registry, db_row, bindings, blackboard, cache, trace);
    }
}

fn apply_binding(
    db_row: &DatabaseRow,
    table: &crate::table::DataTable,
    column: ColumnId,
    key: BlackboardKey,
    blackboard: &mut Blackboard,
    cache: &mut AiTableCache,
    trace: &AiDataTraceFlag,
) {
    if table.column_index(column).is_none() {
        trace.emit(format!("missing column {:?}", column));
        blackboard.set(key, BlackboardValue::None);
        return;
    }

    let resolved = table.get_resolved(db_row.row, column).or_else(|| {
        table
            .column_index(column)
            .and_then(|ci| table.columns().get(ci))
            .and_then(|c| c.default.clone())
    });
    let resolved = match resolved {
        Some(v) => db_row.apply_overrides(column, Some(v)),
        None => db_row.apply_overrides(column, None),
    };

    let Some(cell) = resolved else {
        trace.emit(format!("missing cell {:?} {:?}", db_row.row, column));
        blackboard.set(key, BlackboardValue::None);
        return;
    };

    let bb = value_to_blackboard(&cell);
    let cv = value_to_cached(&cell);
    blackboard.set(key, bb);
    if let Some(cv) = cv {
        cache.insert_sorted(column, cv);
    }
}

fn value_to_blackboard(value: &Value) -> BlackboardValue {
    match value {
        Value::Float(f) => BlackboardValue::Float(*f),
        Value::Int(i) => BlackboardValue::Int(*i),
        Value::Bool(b) => BlackboardValue::Bool(*b),
        Value::String(s) => BlackboardValue::String(s.clone()),
        Value::Null => BlackboardValue::None,
    }
}

fn value_to_cached(value: &Value) -> Option<CachedValue> {
    match value {
        Value::Float(f) => Some(CachedValue::Float(*f)),
        Value::Int(i) => Some(CachedValue::Int(*i)),
        Value::Bool(b) => Some(CachedValue::Bool(*b)),
        Value::String(s) => Some(CachedValue::String(s.clone())),
        Value::Null => None,
    }
}
