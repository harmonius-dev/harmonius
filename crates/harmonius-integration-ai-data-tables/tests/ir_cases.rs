//! Integration tests mapped from `docs/design/integration/ai-data-tables-test-cases.md`.

use harmonius_integration_ai_data_tables::{
    bake_goap_action_cost, bake_goap_action_from_formula, read_goap_planning_cost, AiDataTraceFlag,
    AiTableCache, BakeError, Blackboard, BlackboardBinding, BlackboardBindings, BlackboardKey,
    BlackboardTableBindingSystem, BlackboardValue, BtTableLookup, CachedValue, ColumnError,
    ColumnId, ColumnSchema, ComponentStore, DataTable, DatabaseRow, EntityEventQueue, EntityId,
    FormulaId, GoapAction, ResponseCurve, Row, RowId, TableColumnConsideration, TableId,
    TableRegistry, TableReloaded, Value, World,
};
use smallvec::smallvec;

fn npc_table() -> DataTable {
    let cols = vec![
        ColumnSchema {
            id: ColumnId(1),
            default: Some(Value::Float(0.0)),
        },
        ColumnSchema {
            id: ColumnId(2),
            default: Some(Value::Float(1.0)),
        },
        ColumnSchema {
            id: ColumnId(3),
            default: None,
        },
    ];
    let rows = vec![Row {
        id: RowId(1),
        parent: None,
        values: vec![
            Value::Float(15.0),
            Value::Int(25),
            Value::String("oops".into()),
        ],
    }];
    DataTable::new(cols, rows, 1)
}

#[test]
fn tc_ir_2_1_5_u1_binding_binary_search() {
    let bindings = BlackboardBindings {
        bindings: vec![
            BlackboardBinding {
                key: BlackboardKey(1),
                column: ColumnId(2),
            },
            BlackboardBinding {
                key: BlackboardKey(2),
                column: ColumnId(5),
            },
            BlackboardBinding {
                key: BlackboardKey(3),
                column: ColumnId(9),
            },
        ],
    };
    assert_eq!(bindings.find(ColumnId(5)), Some(1));
}

#[test]
fn tc_ir_2_1_3_u1_bake_formula() {
    let cost = bake_goap_action_cost(FormulaId(1), &[2.0, 3.0]).expect("bake");
    assert!((cost - 5.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_1_3_u2_unknown_formula() {
    let err = bake_goap_action_cost(FormulaId(99), &[]).unwrap_err();
    assert_eq!(err, BakeError::UnknownFormula(FormulaId(99)));
}

#[test]
fn tc_ir_2_1_1_2_bt_missing_row_uses_schema_default() {
    let cols = vec![ColumnSchema {
        id: ColumnId(1),
        default: Some(Value::Float(42.0)),
    }];
    let rows = vec![Row {
        id: RowId(1),
        parent: None,
        values: vec![Value::Float(1.0)],
    }];
    let table = DataTable::new(cols, rows, 1);
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), table);
    let db_row = DatabaseRow {
        table: TableId(1),
        row: RowId(999),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let leaf = BtTableLookup {
        column: ColumnId(1),
        target_key: BlackboardKey(0),
    };
    let mut cache = AiTableCache::new(TableId(1));
    let v = leaf
        .lookup(&reg, &db_row, &mut cache)
        .expect("default when row missing");
    match v {
        CachedValue::Float(f) => assert!((f - 42.0).abs() < 1e-4),
        _ => panic!("expected float default"),
    }
}

#[test]
fn tc_ir_2_1_1_1_bt_aggro_to_blackboard() {
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), npc_table());
    let db_row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let leaf = BtTableLookup {
        column: ColumnId(1),
        target_key: BlackboardKey(7),
    };
    let mut cache = AiTableCache::new(TableId(1));
    let v = leaf.lookup(&reg, &db_row, &mut cache).expect("lookup");
    match v {
        CachedValue::Float(f) => assert!((f - 15.0).abs() < 1e-4),
        _ => panic!("expected float"),
    }
}

#[test]
fn tc_ir_2_1_1_n1_bt_type_mismatch() {
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), npc_table());
    let db_row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let leaf = BtTableLookup {
        column: ColumnId(3),
        target_key: BlackboardKey(1),
    };
    let mut cache = AiTableCache::new(TableId(1));
    let err = leaf.lookup(&reg, &db_row, &mut cache).unwrap_err();
    assert!(matches!(err, ColumnError::ColumnTypeMismatch { .. }));
}

#[test]
fn tc_ir_2_1_2_1_utility_strength() {
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), npc_table());
    let db_row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let cons = TableColumnConsideration {
        column: ColumnId(2),
        curve: ResponseCurve::Linear,
    };
    let mut cache = AiTableCache::new(TableId(1));
    let score = cons.lookup(&reg, &db_row, &mut cache).expect("utility");
    assert!((score - 25.0).abs() < 1e-4);
}

#[test]
fn tc_ir_2_1_2_2_utility_null_as_zero() {
    let cols = vec![ColumnSchema {
        id: ColumnId(4),
        default: None,
    }];
    let rows = vec![Row {
        id: RowId(1),
        parent: None,
        values: vec![Value::Null],
    }];
    let table = DataTable::new(cols, rows, 1);
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), table);
    let db_row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let cons = TableColumnConsideration {
        column: ColumnId(4),
        curve: ResponseCurve::Linear,
    };
    let mut cache = AiTableCache::new(TableId(1));
    let score = cons.lookup(&reg, &db_row, &mut cache).expect("utility");
    assert!((score - 0.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_1_2_n1_missing_table() {
    let reg = TableRegistry::default();
    let db_row = DatabaseRow {
        table: TableId(9),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let cons = TableColumnConsideration {
        column: ColumnId(1),
        curve: ResponseCurve::Linear,
    };
    let mut cache = AiTableCache::new(TableId(9));
    let err = cons.lookup(&reg, &db_row, &mut cache).unwrap_err();
    assert!(matches!(err, ColumnError::MissingTable(_)));
}

#[test]
fn tc_ir_2_1_2_n2_utility_bool_matches_bt_float_contract() {
    let cols = vec![ColumnSchema {
        id: ColumnId(1),
        default: Some(Value::Bool(true)),
    }];
    let rows = vec![Row {
        id: RowId(1),
        parent: None,
        values: vec![Value::Bool(false)],
    }];
    let table = DataTable::new(cols, rows, 1);
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), table);
    let db_row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let cons = TableColumnConsideration {
        column: ColumnId(1),
        curve: ResponseCurve::Linear,
    };
    let mut cache = AiTableCache::new(TableId(1));
    let err = cons.lookup(&reg, &db_row, &mut cache).unwrap_err();
    assert!(matches!(err, ColumnError::ColumnTypeMismatch { .. }));
}

#[test]
fn tc_ir_2_1_3_1_goap_baked_cost() {
    let action = GoapAction { cost: 5.0 };
    assert!((action.cost - 5.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_1_3_n1_goap_runtime_f32_only() {
    let a = GoapAction { cost: 2.5 };
    let GoapAction { cost } = a;
    let _: f32 = cost;
    assert_eq!(std::mem::size_of::<GoapAction>(), 4);
    assert_eq!(std::any::type_name_of_val(&cost), "f32");
}

#[test]
fn tc_ir_2_1_3_2_goap_formula_baked() {
    let action = bake_goap_action_from_formula(FormulaId(2), &[2.0, 2.5, 2.0]).expect("bake");
    assert!((action.cost - 10.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_1_3_3_goap_bake_pipeline_e2e() {
    let baked = bake_goap_action_from_formula(FormulaId(1), &[1.0, 2.0, 2.0]).expect("bake");
    let read = read_goap_planning_cost(&baked);
    assert!((read - 5.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_1_4_2_ability_fk_chain_two_hops() {
    let cols = vec![
        ColumnSchema {
            id: ColumnId(1),
            default: None,
        },
        ColumnSchema {
            id: ColumnId(2),
            default: None,
        },
    ];
    let rows = vec![
        Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::Float(100.0), Value::Float(1.0)],
        },
        Row {
            id: RowId(2),
            parent: Some(RowId(1)),
            values: vec![Value::Null, Value::Float(2.0)],
        },
        Row {
            id: RowId(3),
            parent: Some(RowId(2)),
            values: vec![Value::Null, Value::Null],
        },
    ];
    let table = DataTable::new(cols, rows, 1);
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), table);
    let cd = reg.get(TableId(1)).expect("table");
    let v1 = cd.get_resolved(RowId(3), ColumnId(1)).expect("merged col1");
    match v1 {
        Value::Float(f) => assert!((f - 100.0).abs() < 1e-4),
        _ => panic!("expected inherited float"),
    }
    let v2 = cd.get_resolved(RowId(3), ColumnId(2)).expect("merged col2");
    match v2 {
        Value::Float(f) => assert!((f - 2.0).abs() < 1e-4),
        _ => panic!("expected float"),
    }
}

#[test]
fn tc_ir_2_1_4_1_ability_fk_resolve() {
    let cols = vec![
        ColumnSchema {
            id: ColumnId(1),
            default: None,
        },
        ColumnSchema {
            id: ColumnId(2),
            default: None,
        },
    ];
    let rows = vec![
        Row {
            id: RowId(10),
            parent: None,
            values: vec![Value::Float(2.0), Value::Float(30.0)],
        },
        Row {
            id: RowId(20),
            parent: Some(RowId(10)),
            values: vec![Value::Null, Value::Float(5.0)],
        },
    ];
    let table = DataTable::new(cols, rows, 1);
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), table);
    let row = reg.resolve_foreign_key(TableId(1), RowId(20)).expect("row");
    let cd = reg.get(TableId(1)).expect("table");
    let cooldown = cd.get_resolved(row.id, ColumnId(2)).expect("merged");
    match cooldown {
        Value::Float(f) => assert!((f - 5.0).abs() < 1e-4),
        _ => panic!("expected float"),
    }
    let range = cd.get_resolved(row.id, ColumnId(1)).expect("inherited");
    match range {
        Value::Float(f) => assert!((f - 2.0).abs() < 1e-4),
        _ => panic!("expected inherited float"),
    }
}

#[test]
fn tc_ir_2_1_5_1_bind_on_spawn() {
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), npc_table());
    let bindings = BlackboardBindings {
        bindings: vec![BlackboardBinding {
            key: BlackboardKey(1),
            column: ColumnId(1),
        }],
    };
    let row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let mut bb = Blackboard::default();
    let mut cache = AiTableCache::new(TableId(1));
    let trace = AiDataTraceFlag::new();
    BlackboardTableBindingSystem::bind_entity(&reg, &row, &bindings, &mut bb, &mut cache, &trace);
    match bb.get(BlackboardKey(1)) {
        BlackboardValue::Float(f) => assert!((f - 15.0).abs() < 1e-4),
        other => panic!("unexpected bb {other:?}"),
    }
}

#[test]
fn tc_ir_2_1_5_2_rebind_on_reload() {
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), npc_table());
    let bindings = BlackboardBindings {
        bindings: vec![BlackboardBinding {
            key: BlackboardKey(1),
            column: ColumnId(1),
        }],
    };
    let row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let mut bb = Blackboard::default();
    let mut cache = AiTableCache::new(TableId(1));
    let trace = AiDataTraceFlag::new();
    BlackboardTableBindingSystem::bind_entity(&reg, &row, &bindings, &mut bb, &mut cache, &trace);
    let new_table = DataTable::new(
        vec![
            ColumnSchema {
                id: ColumnId(1),
                default: Some(Value::Float(0.0)),
            },
            ColumnSchema {
                id: ColumnId(2),
                default: Some(Value::Float(1.0)),
            },
            ColumnSchema {
                id: ColumnId(3),
                default: None,
            },
        ],
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![
                Value::Float(99.0),
                Value::Int(25),
                Value::String("oops".into()),
            ],
        }],
        2,
    );
    reg.swap(TableId(1), new_table);
    let ev = TableReloaded {
        table: TableId(1),
        old_version: 1,
        new_version: 2,
    };
    BlackboardTableBindingSystem::rebind_on_reload(
        &reg, &ev, &row, &bindings, &mut bb, &mut cache, &trace,
    );
    match bb.get(BlackboardKey(1)) {
        BlackboardValue::Float(f) => assert!((f - 99.0).abs() < 1e-4),
        other => panic!("unexpected bb {other:?}"),
    }
}

#[test]
fn tc_ir_2_1_5_n1_missing_column_writes_none() {
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), npc_table());
    let bindings = BlackboardBindings {
        bindings: vec![BlackboardBinding {
            key: BlackboardKey(1),
            column: ColumnId(99),
        }],
    };
    let row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let mut bb = Blackboard::default();
    let mut cache = AiTableCache::new(TableId(1));
    let trace = AiDataTraceFlag::new();
    trace.set_enabled(true);
    BlackboardTableBindingSystem::bind_entity(&reg, &row, &bindings, &mut bb, &mut cache, &trace);
    assert_eq!(bb.get(BlackboardKey(1)), BlackboardValue::None);
    assert!(!trace.drain_notes().is_empty());
}

#[test]
fn tc_ir_2_1_7_snapshot_invalidated_until_rebind() {
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), npc_table());
    let db_row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let leaf = BtTableLookup {
        column: ColumnId(1),
        target_key: BlackboardKey(0),
    };
    let mut cache = AiTableCache::new(TableId(1));
    leaf.lookup(&reg, &db_row, &mut cache).expect("seed cache");
    let table_version = reg.get(TableId(1)).expect("table").version();
    cache.invalidate(table_version);
    let err = leaf.lookup(&reg, &db_row, &mut cache).unwrap_err();
    assert_eq!(err, ColumnError::SnapshotInvalidated(TableId(1)));
    let bindings = BlackboardBindings {
        bindings: vec![BlackboardBinding {
            key: BlackboardKey(1),
            column: ColumnId(1),
        }],
    };
    let mut bb = Blackboard::default();
    let trace = AiDataTraceFlag::new();
    BlackboardTableBindingSystem::bind_entity(
        &reg, &db_row, &bindings, &mut bb, &mut cache, &trace,
    );
    leaf.lookup(&reg, &db_row, &mut cache)
        .expect("after bind_entity repop");
}

#[test]
fn tc_ir_2_1_6_1_hot_reload_clears_cache() {
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), npc_table());
    let entity = EntityId(1);
    let mut caches = ComponentStore::default();
    let mut cache = AiTableCache::new(TableId(1));
    cache.insert_sorted(ColumnId(1), CachedValue::Float(3.0));
    cache.version = 1;
    cache.cleared = false;
    caches.insert(entity, cache);
    let mut events = EntityEventQueue::default();
    events.push(TableReloaded {
        table: TableId(1),
        old_version: 1,
        new_version: 2,
    });
    BlackboardTableBindingSystem::drain_reload_events(&mut events, &mut caches);
    let c = caches.get_mut(entity).expect("cache");
    assert!(c.cleared);
    assert!(c.entries.is_empty());
}

#[test]
fn tc_ir_2_1_6_n1_reload_dangling_row_defaults() {
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), npc_table());
    let bindings = BlackboardBindings {
        bindings: vec![BlackboardBinding {
            key: BlackboardKey(1),
            column: ColumnId(1),
        }],
    };
    let row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let mut bb = Blackboard::default();
    let mut cache = AiTableCache::new(TableId(1));
    let trace = AiDataTraceFlag::new();
    trace.set_enabled(true);
    BlackboardTableBindingSystem::bind_entity(&reg, &row, &bindings, &mut bb, &mut cache, &trace);
    let new_table = DataTable::new(
        vec![ColumnSchema {
            id: ColumnId(1),
            default: Some(Value::Float(3.0)),
        }],
        vec![Row {
            id: RowId(2),
            parent: None,
            values: vec![Value::Float(8.0)],
        }],
        2,
    );
    reg.swap(TableId(1), new_table);
    let ev = TableReloaded {
        table: TableId(1),
        old_version: 1,
        new_version: 2,
    };
    BlackboardTableBindingSystem::rebind_on_reload(
        &reg, &ev, &row, &bindings, &mut bb, &mut cache, &trace,
    );
    assert_eq!(bb.get(BlackboardKey(1)), BlackboardValue::None);
    let notes = trace.drain_notes();
    assert!(
        notes.iter().any(|n| n.contains("missing row")),
        "expected missing row trace, got {notes:?}"
    );
}

#[test]
fn tc_ir_2_1_6_2_reload_queued_until_pre_update() {
    let mut world = World::new();
    world.registry.insert(TableId(1), npc_table());
    let bindings = BlackboardBindings {
        bindings: vec![BlackboardBinding {
            key: BlackboardKey(1),
            column: ColumnId(1),
        }],
    };
    let row = DatabaseRow {
        table: TableId(1),
        row: RowId(1),
        bound_columns: smallvec![],
        overrides: smallvec![],
    };
    let bb = Blackboard::default();
    let entity = world.spawn(Some(row.clone()), Some(bindings), bb);
    world.run_bind_entity(entity);
    world.reload_events.push(TableReloaded {
        table: TableId(1),
        old_version: 1,
        new_version: 2,
    });
    assert_eq!(world.reload_events.len(), 1);
    assert!(!world.cache_mut(entity).expect("cache").cleared);
    world.pre_update();
    assert!(world.reload_events.is_empty());
    assert!(world.cache_mut(entity).expect("cache").cleared);
}

#[test]
fn tc_ir_2_1_6_2_queue_drain_fifo() {
    let mut events = EntityEventQueue::default();
    events.push(TableReloaded {
        table: TableId(1),
        old_version: 1,
        new_version: 2,
    });
    assert_eq!(events.drain().len(), 1);
    assert!(events.drain().is_empty());
}

#[test]
fn tc_dbg_1_trace_toggle() {
    let trace = AiDataTraceFlag::new();
    trace.set_enabled(true);
    trace.emit("ping");
    assert_eq!(trace.drain_notes(), vec!["ping".to_string()]);
}
