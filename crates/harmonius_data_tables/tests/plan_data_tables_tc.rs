//! Traceable coverage for `PLAN-data-systems-data-tables` / `data-tables-test-cases.md`.

use bumpalo::Bump;
use harmonius_data_tables::{
    binding_snapshot, detect_cycle, formula_item_dps, join_query, populate_transform2d,
    validate_all, validate_table, BindingSnapshot, ColumnConstraint, ColumnDef, ColumnId,
    ColumnPredicate, ColumnType, CustomRule, DataTable, DatabaseRow, FilterExpr, HotReloadEvent,
    HotReloadStack, IndexType, Row, RowId, RowRef, TableId, TableRegistry, TableSchema,
    Transform2D, Value,
};
use smol_str::SmolStr;
use std::collections::BTreeMap;
use std::ptr;
use std::time::Instant;

fn col(id: u16, name: &str, col_type: ColumnType, nullable: bool) -> ColumnDef {
    ColumnDef {
        id: ColumnId(id),
        name: SmolStr::new(name),
        col_type,
        nullable,
        indexed: false,
        index_kind: None,
        constraints: smallvec::SmallVec::new(),
        custom_rule: None,
        fk_target_table: None,
    }
}

/// TC-13.7.1.1 — schema type validation (I32 column rejects String).
#[test]
fn tc_13_7_1_1_schema_type_validation() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "c", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let ok = Row {
        id: RowId(1),
        parent: None,
        values: vec![Value::I32(42)],
    };
    assert!(schema.validate_row(&ok).is_ok());
    let bad = Row {
        id: RowId(2),
        parent: None,
        values: vec![Value::String(SmolStr::new_inline("x"))],
    };
    let err = schema.validate_row(&bad).unwrap_err();
    assert_eq!(err[0].column, ColumnId(0));
}

/// TC-13.7.1.2 — range constraint on scalar column.
#[test]
fn tc_13_7_1_2_schema_constraint_range() {
    let mut c = col(0, "c", ColumnType::F32, false);
    c.constraints.push(ColumnConstraint::Range {
        min: 0.0,
        max: 100.0,
    });
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![c],
        primary_key: ColumnId(0),
    };
    let ok = Row {
        id: RowId(1),
        parent: None,
        values: vec![Value::F32(50.0)],
    };
    assert!(schema.validate_row(&ok).is_ok());
    let bad = Row {
        id: RowId(2),
        parent: None,
        values: vec![Value::F32(200.0)],
    };
    assert!(schema.validate_row(&bad).is_err());
}

/// TC-13.7.2.1 — duplicate primary key rejected at table build.
#[test]
fn tc_13_7_2_1_row_unique_key() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "c", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let rows = vec![
        Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(1)],
        },
        Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(2)],
        },
    ];
    assert!(DataTable::try_new(schema, rows, 1).is_err());
}

/// TC-13.7.3.1 / TC-16.3.5.1 — FK resolves across registry.
#[test]
fn tc_13_7_3_1_foreign_key_valid_and_tc_16_3_5_1() {
    let mut fk = col(1, "fk", ColumnType::ForeignKey, false);
    fk.fk_target_table = Some(TableId(1));
    let schema_a = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("a"),
        columns: vec![col(0, "id", ColumnType::I32, false), fk],
        primary_key: ColumnId(0),
    };
    let schema_b = TableSchema {
        table_id: TableId(1),
        name: SmolStr::new_inline("b"),
        columns: vec![col(0, "id", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let b = DataTable::try_new(
        schema_b,
        vec![Row {
            id: RowId(5),
            parent: None,
            values: vec![Value::I32(5)],
        }],
        1,
    )
    .unwrap();
    let a = DataTable::try_new(
        schema_a,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![
                Value::I32(1),
                Value::ForeignKey(RowRef {
                    table: TableId(1),
                    row: RowId(5),
                }),
            ],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), b);
    reg.insert(TableId(0), a);
    let rr = RowRef {
        table: TableId(1),
        row: RowId(5),
    };
    let row5 = reg.resolve_foreign_key(&rr).expect("row");
    assert_eq!(row5.id, RowId(5));
}

/// TC-13.7.3.2 — broken FK yields validation error with ids.
#[test]
fn tc_13_7_3_2_foreign_key_broken() {
    let mut fk = col(1, "fk", ColumnType::ForeignKey, false);
    fk.fk_target_table = Some(TableId(1));
    let schema_a = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("a"),
        columns: vec![col(0, "id", ColumnType::I32, false), fk],
        primary_key: ColumnId(0),
    };
    let schema_b = TableSchema {
        table_id: TableId(1),
        name: SmolStr::new_inline("b"),
        columns: vec![col(0, "id", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let b = DataTable::try_new(
        schema_b,
        vec![Row {
            id: RowId(5),
            parent: None,
            values: vec![Value::I32(5)],
        }],
        1,
    )
    .unwrap();
    let a = DataTable::try_new(
        schema_a,
        vec![Row {
            id: RowId(10),
            parent: None,
            values: vec![
                Value::I32(10),
                Value::ForeignKey(RowRef {
                    table: TableId(1),
                    row: RowId(999),
                }),
            ],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), b);
    reg.insert(TableId(0), a);
    let errs = validate_table(reg.get(TableId(0)).unwrap(), &reg);
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0].table, TableId(0));
    assert_eq!(errs[0].row, RowId(10));
    assert_eq!(errs[0].column, ColumnId(1));
}

/// TC-13.7.4.1 — successful reload bumps version and journals reload.
#[test]
fn tc_13_7_4_1_hot_reload_valid() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "v", ColumnType::String, false)],
        primary_key: ColumnId(0),
    };
    let v1 = DataTable::try_new(
        schema.clone(),
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::String(SmolStr::new_inline("a"))],
        }],
        1,
    )
    .unwrap();
    let v2 = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::String(SmolStr::new_inline("b"))],
        }],
        2,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), v1);
    let mut journal = Vec::new();
    let mut stack = HotReloadStack::default();
    stack
        .try_reload(&mut reg, TableId(0), v2, &mut journal)
        .unwrap();
    assert_eq!(reg.get(TableId(0)).unwrap().version(), 2);
    assert!(matches!(
        journal[0],
        HotReloadEvent::Reloaded(ref e) if e.new_version == 2
    ));
}

/// TC-13.7.4.2 — invalid reload leaves version unchanged and journals failure.
#[test]
fn tc_13_7_4_2_hot_reload_invalid() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "v", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let good = DataTable::try_new(
        schema.clone(),
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(1)],
        }],
        1,
    )
    .unwrap();
    let bad = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::String(SmolStr::new_inline("oops"))],
        }],
        2,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), good);
    let mut journal = Vec::new();
    let mut stack = HotReloadStack::default();
    assert!(stack
        .try_reload(&mut reg, TableId(0), bad, &mut journal)
        .is_err());
    assert_eq!(reg.get(TableId(0)).unwrap().version(), 1);
    assert!(matches!(journal[0], HotReloadEvent::Failed(_)));
}

/// TC-13.7.4.3 — rollback restores previous version.
#[test]
fn tc_13_7_4_3_hot_reload_rollback() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "v", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let t1 = DataTable::try_new(
        schema.clone(),
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(1)],
        }],
        1,
    )
    .unwrap();
    let t2 = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(2)],
        }],
        2,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), t1);
    let mut journal = Vec::new();
    let mut stack = HotReloadStack::default();
    stack
        .try_reload(&mut reg, TableId(0), t2, &mut journal)
        .unwrap();
    stack.rollback(&mut reg, TableId(0)).unwrap();
    assert_eq!(reg.get(TableId(0)).unwrap().version(), 1);
    assert_eq!(
        reg.get(TableId(0)).unwrap().get(RowId(1)).unwrap().values[0],
        Value::I32(1)
    );
}

/// TC-13.7.5.1 — single-level inheritance merge.
#[test]
fn tc_13_7_5_1_inheritance_single() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![
            col(0, "a", ColumnType::I32, true),
            col(1, "b", ColumnType::I32, true),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![
            Row {
                id: RowId(1),
                parent: None,
                values: vec![Value::I32(1), Value::I32(10)],
            },
            Row {
                id: RowId(2),
                parent: Some(RowId(1)),
                values: vec![Value::I32(99), Value::Null],
            },
        ],
        1,
    )
    .unwrap();
    assert_eq!(
        table.get_resolved(RowId(2), ColumnId(0), None),
        Some(Value::I32(99))
    );
    assert_eq!(
        table.get_resolved(RowId(2), ColumnId(1), None),
        Some(Value::I32(10))
    );
}

/// TC-13.7.5.2 — three-level inheritance chain.
#[test]
fn tc_13_7_5_2_inheritance_chain_3() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![
            col(0, "x", ColumnType::I32, true),
            col(1, "y", ColumnType::I32, true),
            col(2, "z", ColumnType::I32, true),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![
            Row {
                id: RowId(1),
                parent: None,
                values: vec![Value::I32(1), Value::Null, Value::Null],
            },
            Row {
                id: RowId(2),
                parent: Some(RowId(1)),
                values: vec![Value::Null, Value::I32(2), Value::Null],
            },
            Row {
                id: RowId(3),
                parent: Some(RowId(2)),
                values: vec![Value::Null, Value::Null, Value::I32(3)],
            },
        ],
        1,
    )
    .unwrap();
    assert_eq!(
        table.get_resolved(RowId(3), ColumnId(0), None),
        Some(Value::I32(1))
    );
    assert_eq!(
        table.get_resolved(RowId(3), ColumnId(1), None),
        Some(Value::I32(2))
    );
    assert_eq!(
        table.get_resolved(RowId(3), ColumnId(2), None),
        Some(Value::I32(3))
    );
}

/// TC-13.7.5.3 — circular inheritance detected.
#[test]
fn tc_13_7_5_3_inheritance_circular() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "k", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![
            Row {
                id: RowId(1),
                parent: Some(RowId(2)),
                values: vec![Value::I32(1)],
            },
            Row {
                id: RowId(2),
                parent: Some(RowId(1)),
                values: vec![Value::I32(2)],
            },
        ],
        1,
    )
    .unwrap();
    let cyc = detect_cycle(&table, RowId(1)).expect("cycle");
    assert!(cyc.contains(&RowId(1)) && cyc.contains(&RowId(2)));
}

/// TC-13.7.10.1 — locale resolves exact active tag.
#[test]
fn tc_13_7_10_1_locale_resolves() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "s", ColumnType::LocalizedString, false)],
        primary_key: ColumnId(0),
    };
    let mut ov = BTreeMap::new();
    ov.insert(SmolStr::new("en-US"), SmolStr::new("Sword"));
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::Localized(harmonius_data_tables::LocalizedPack {
                base: SmolStr::new_inline("sword"),
                overlays: ov,
            })],
        }],
        1,
    )
    .unwrap();
    assert_eq!(
        table.get_resolved(RowId(1), ColumnId(0), Some("en-US")),
        Some(Value::String(SmolStr::new("Sword")))
    );
}

/// TC-13.7.10.2 — language fallback from region tag.
#[test]
fn tc_13_7_10_2_locale_fallback() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "s", ColumnType::LocalizedString, false)],
        primary_key: ColumnId(0),
    };
    let mut ov = BTreeMap::new();
    ov.insert(SmolStr::new("fr"), SmolStr::new("Épée"));
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::Localized(harmonius_data_tables::LocalizedPack {
                base: SmolStr::new_inline("sword"),
                overlays: ov,
            })],
        }],
        1,
    )
    .unwrap();
    assert_eq!(
        table.get_resolved(RowId(1), ColumnId(0), Some("fr-CA")),
        Some(Value::String(SmolStr::new("Épée")))
    );
}

/// TC-16.3.8.1 — same fallback path as TC-13.7.10.2 with lowercase sample.
#[test]
fn tc_16_3_8_1_locale_string_fallback() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "s", ColumnType::LocalizedString, false)],
        primary_key: ColumnId(0),
    };
    let mut ov = BTreeMap::new();
    ov.insert(SmolStr::new("fr"), SmolStr::new("épée"));
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::Localized(harmonius_data_tables::LocalizedPack {
                base: SmolStr::new_inline("sword"),
                overlays: ov,
            })],
        }],
        1,
    )
    .unwrap();
    assert_eq!(
        table.get_resolved(RowId(1), ColumnId(0), Some("fr-CA")),
        Some(Value::String(SmolStr::new("épée")))
    );
}

/// TC-13.7.10.3 — exact overlay wins over base.
#[test]
fn tc_13_7_10_3_locale_overlay() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "s", ColumnType::LocalizedString, false)],
        primary_key: ColumnId(0),
    };
    let mut ov = BTreeMap::new();
    ov.insert(SmolStr::new("ja"), SmolStr::new("剣"));
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::Localized(harmonius_data_tables::LocalizedPack {
                base: SmolStr::new_inline("sword"),
                overlays: ov,
            })],
        }],
        1,
    )
    .unwrap();
    assert_eq!(
        table.get_resolved(RowId(1), ColumnId(0), Some("ja")),
        Some(Value::String(SmolStr::new("剣")))
    );
}

/// TC-13.7.11.1 — hash index exact lookup on 10k rows.
#[test]
fn tc_13_7_11_1_index_hash_lookup() {
    let mut c = col(1, "c", ColumnType::I32, false);
    c.indexed = true;
    c.index_kind = Some(IndexType::Hash);
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "id", ColumnType::I32, false), c],
        primary_key: ColumnId(0),
    };
    let mut rows = Vec::new();
    for i in 0..10_000 {
        rows.push(Row {
            id: RowId(i),
            parent: None,
            values: vec![Value::I32(i as i32), Value::I32((i % 100) as i32)],
        });
    }
    let table = DataTable::try_new(schema, rows, 1).unwrap();
    let hits = table
        .index_lookup(ColumnId(1), &Value::I32(42))
        .expect("hits");
    for id in &hits {
        let r = table.get(*id).unwrap();
        assert_eq!(r.values[1], Value::I32(42));
    }
}

/// TC-13.7.11.2 — btree range matches brute force.
#[test]
fn tc_13_7_11_2_index_btree_range() {
    let mut c = col(1, "c", ColumnType::I32, false);
    c.indexed = true;
    c.index_kind = Some(IndexType::BTree);
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "id", ColumnType::I32, false), c],
        primary_key: ColumnId(0),
    };
    let mut rows = Vec::new();
    for i in 0..10_000 {
        rows.push(Row {
            id: RowId(i),
            parent: None,
            values: vec![Value::I32(i as i32), Value::I32((i % 201) as i32)],
        });
    }
    let table = DataTable::try_new(schema, rows, 1).unwrap();
    let bump = Bump::new();
    let got = table.range(ColumnId(1), &Value::I32(50), &Value::I32(100), &bump);
    let brute: Vec<RowId> = table
        .rows()
        .iter()
        .filter(|r| {
            if let Value::I32(v) = r.values[1] {
                (50..=100).contains(&v)
            } else {
                false
            }
        })
        .map(|r| r.id)
        .collect();
    let mut got_ids: Vec<RowId> = got.iter().map(|r| r.id).collect();
    got_ids.sort_unstable();
    let mut brute_sorted = brute;
    brute_sorted.sort_unstable();
    assert_eq!(got_ids, brute_sorted);
}

/// TC-13.7.11.3 — compound filter matches brute force.
#[test]
fn tc_13_7_11_3_filter_and_or_not() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "a", ColumnType::Bool, false),
            col(2, "b", ColumnType::Bool, false),
            col(3, "c", ColumnType::Bool, false),
        ],
        primary_key: ColumnId(0),
    };
    let mut rows = Vec::new();
    for i in 0..1000 {
        rows.push(Row {
            id: RowId(i),
            parent: None,
            values: vec![
                Value::I32(i as i32),
                Value::Bool(i % 3 == 0),
                Value::Bool(i % 5 == 0),
                Value::Bool(i % 7 == 0),
            ],
        });
    }
    let table = DataTable::try_new(schema, rows, 1).unwrap();
    let expr = FilterExpr::And(vec![
        FilterExpr::Or(vec![
            FilterExpr::Column {
                col: ColumnId(1),
                predicate: ColumnPredicate::Equals(Value::Bool(true)),
            },
            FilterExpr::Column {
                col: ColumnId(2),
                predicate: ColumnPredicate::Equals(Value::Bool(true)),
            },
        ]),
        FilterExpr::Not(Box::new(FilterExpr::Column {
            col: ColumnId(3),
            predicate: ColumnPredicate::Equals(Value::Bool(true)),
        })),
    ]);
    let bump = Bump::new();
    let got = table.query(&expr, &bump);
    let mut got_ids: Vec<RowId> = got.iter().map(|r| r.id).collect();
    got_ids.sort_unstable();
    let mut brute: Vec<RowId> = table
        .rows()
        .iter()
        .filter(|r| {
            let a = r.values[1] == Value::Bool(true);
            let b = r.values[2] == Value::Bool(true);
            let c = r.values[3] == Value::Bool(true);
            (a || b) && !c
        })
        .map(|r| r.id)
        .collect();
    brute.sort_unstable();
    assert_eq!(got_ids, brute);
}

/// TC-13.7.12.1 — binding snapshot contains all columns.
#[test]
fn tc_13_7_12_1_binding_spawn() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "name", ColumnType::String, false),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(1), Value::String(SmolStr::new_inline("x"))],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), table);
    let db = DatabaseRow {
        table: TableId(0),
        row: RowId(1),
        bound_columns: smallvec::SmallVec::new(),
        overrides: smallvec::SmallVec::new(),
    };
    let snap = binding_snapshot(&reg, &db).expect("snap");
    assert_eq!(
        snap.get("name"),
        Some(&Value::String(SmolStr::new_inline("x")))
    );
}

/// TC-13.7.12.2 — overrides win over inherited / stored values.
#[test]
fn tc_13_7_12_2_binding_override() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "c", ColumnType::I32, false),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(1), Value::I32(10)],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), table);
    let db = DatabaseRow {
        table: TableId(0),
        row: RowId(1),
        bound_columns: smallvec::SmallVec::new(),
        overrides: smallvec::smallvec![(ColumnId(1), Value::I32(99))],
    };
    let snap = binding_snapshot(&reg, &db).expect("snap");
    assert_eq!(snap.get("c"), Some(&Value::I32(99)));
}

/// TC-13.7.12.3 — 2D transform fields from row.
#[test]
fn tc_13_7_12_3_binding_2d_entity() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "x", ColumnType::F32, false),
            col(2, "y", ColumnType::F32, false),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(1), Value::F32(3.0), Value::F32(4.0)],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), table);
    let db = DatabaseRow {
        table: TableId(0),
        row: RowId(1),
        bound_columns: smallvec::SmallVec::new(),
        overrides: smallvec::SmallVec::new(),
    };
    let snap = binding_snapshot(&reg, &db).expect("snap");
    let mut t2 = Transform2D::default();
    populate_transform2d(&snap, &mut t2);
    assert!((t2.x - 3.0).abs() < f32::EPSILON);
    assert!((t2.y - 4.0).abs() < f32::EPSILON);
}

/// TC-13.7.14.1 — multiple validation errors collected.
#[test]
fn tc_13_7_14_1_validation_full() {
    let mut fk = col(2, "fk", ColumnType::ForeignKey, false);
    fk.fk_target_table = Some(TableId(1));
    let mut rcol = col(1, "r", ColumnType::F32, false);
    rcol.constraints.push(ColumnConstraint::Range {
        min: 0.0,
        max: 10.0,
    });
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "id", ColumnType::I32, false), rcol, fk],
        primary_key: ColumnId(0),
    };
    let b = DataTable::try_new(
        TableSchema {
            table_id: TableId(1),
            name: SmolStr::new_inline("b"),
            columns: vec![col(0, "id", ColumnType::I32, false)],
            primary_key: ColumnId(0),
        },
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(1)],
        }],
        1,
    )
    .unwrap();
    let bad = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![
                Value::String(SmolStr::new_inline("bad")),
                Value::F32(99.0),
                Value::ForeignKey(RowRef {
                    table: TableId(1),
                    row: RowId(99),
                }),
            ],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), b);
    reg.insert(TableId(0), bad);
    let errs = validate_table(reg.get(TableId(0)).unwrap(), &reg);
    assert_eq!(errs.len(), 3);
    assert!(errs
        .iter()
        .all(|e| e.severity == harmonius_data_tables::ValidationSeverity::Error));
}

/// TC-13.7.14.2 — missing asset is warning severity.
#[test]
fn tc_13_7_14_2_asset_ref_broken() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "a", ColumnType::AssetRef, false),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![
                Value::I32(1),
                Value::AssetRef(harmonius_data_tables::AssetHandle(999)),
            ],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), table);
    let errs = validate_table(reg.get(TableId(0)).unwrap(), &reg);
    assert_eq!(errs.len(), 1);
    assert_eq!(
        errs[0].severity,
        harmonius_data_tables::ValidationSeverity::Warning
    );
}

/// TC-13.10.1.1 — ability-shaped row snapshot.
#[test]
fn tc_13_10_1_1_ability_row_load() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("Ability"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "name", ColumnType::String, false),
            col(2, "damage", ColumnType::I32, false),
            col(3, "cooldown", ColumnType::F32, false),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![
                Value::I32(1),
                Value::String(SmolStr::new("Fireball")),
                Value::I32(50),
                Value::F32(3.0),
            ],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), table);
    let snap = binding_snapshot(
        &reg,
        &DatabaseRow {
            table: TableId(0),
            row: RowId(1),
            bound_columns: smallvec::SmallVec::new(),
            overrides: smallvec::SmallVec::new(),
        },
    )
    .unwrap();
    assert_eq!(
        snap.get("name"),
        Some(&Value::String(SmolStr::new("Fireball")))
    );
    assert_eq!(snap.get("damage"), Some(&Value::I32(50)));
    assert_eq!(snap.get("cooldown"), Some(&Value::F32(3.0)));
}

/// TC-13.12.1a.1 — race row numeric modifiers in snapshot.
#[test]
fn tc_13_12_1a_1_race_row_load() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("Race"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "str_mod", ColumnType::I32, false),
            col(2, "dex_mod", ColumnType::I32, false),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(1), Value::I32(2), Value::I32(0)],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), table);
    let snap = binding_snapshot(
        &reg,
        &DatabaseRow {
            table: TableId(0),
            row: RowId(1),
            bound_columns: smallvec::SmallVec::new(),
            overrides: smallvec::SmallVec::new(),
        },
    )
    .unwrap();
    assert_eq!(snap.get("str_mod"), Some(&Value::I32(2)));
    assert_eq!(snap.get("dex_mod"), Some(&Value::I32(0)));
}

/// TC-13.12.1b.1 — class row HP fields.
#[test]
fn tc_13_12_1b_1_class_row_load() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("Class"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "name", ColumnType::String, false),
            col(2, "base_hp", ColumnType::I32, false),
            col(3, "hp_per_level", ColumnType::I32, false),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![
                Value::I32(1),
                Value::String(SmolStr::new("Fighter")),
                Value::I32(10),
                Value::I32(6),
            ],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), table);
    let snap = binding_snapshot(
        &reg,
        &DatabaseRow {
            table: TableId(0),
            row: RowId(1),
            bound_columns: smallvec::SmallVec::new(),
            overrides: smallvec::SmallVec::new(),
        },
    )
    .unwrap();
    assert_eq!(snap.get("base_hp"), Some(&Value::I32(10)));
    assert_eq!(snap.get("hp_per_level"), Some(&Value::I32(6)));
}

/// TC-13.12.1c.1 — two FK refs resolve.
#[test]
fn tc_13_12_1c_1_multiclass_row_load() {
    let mut fk0 = col(1, "c0", ColumnType::ForeignKey, false);
    fk0.fk_target_table = Some(TableId(1));
    let mut fk1 = col(2, "c1", ColumnType::ForeignKey, false);
    fk1.fk_target_table = Some(TableId(1));
    let schema_mc = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("MultiClass"),
        columns: vec![col(0, "id", ColumnType::I32, false), fk0, fk1],
        primary_key: ColumnId(0),
    };
    let schema_cls = TableSchema {
        table_id: TableId(1),
        name: SmolStr::new_inline("Class"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "name", ColumnType::String, false),
        ],
        primary_key: ColumnId(0),
    };
    let classes = DataTable::try_new(
        schema_cls,
        vec![
            Row {
                id: RowId(1),
                parent: None,
                values: vec![Value::I32(1), Value::String(SmolStr::new_inline("F"))],
            },
            Row {
                id: RowId(2),
                parent: None,
                values: vec![Value::I32(2), Value::String(SmolStr::new_inline("W"))],
            },
        ],
        1,
    )
    .unwrap();
    let mc = DataTable::try_new(
        schema_mc,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![
                Value::I32(1),
                Value::ForeignKey(RowRef {
                    table: TableId(1),
                    row: RowId(1),
                }),
                Value::ForeignKey(RowRef {
                    table: TableId(1),
                    row: RowId(2),
                }),
            ],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), classes);
    reg.insert(TableId(0), mc);
    let r0 = RowRef {
        table: TableId(1),
        row: RowId(1),
    };
    let r1 = RowRef {
        table: TableId(1),
        row: RowId(2),
    };
    assert_eq!(
        reg.resolve_foreign_key(&r0).unwrap().values[1],
        Value::String(SmolStr::new_inline("F"))
    );
    assert_eq!(
        reg.resolve_foreign_key(&r1).unwrap().values[1],
        Value::String(SmolStr::new_inline("W"))
    );
}

/// TC-13.12.1d.1 — prestige row exact read.
#[test]
fn tc_13_12_1d_1_prestige_row_load() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("Prestige"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "unlock_level", ColumnType::I32, false),
            col(2, "bonus_xp_mult", ColumnType::F32, false),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(1), Value::I32(10), Value::F32(2.0)],
        }],
        1,
    )
    .unwrap();
    let r = table.get(RowId(1)).unwrap();
    assert_eq!(r.values[1], Value::I32(10));
    assert_eq!(r.values[2], Value::F32(2.0));
}

/// TC-16.3.1.1 — one column per [`ColumnType`] variant.
#[test]
fn tc_16_3_1_1_column_types_all() {
    let mut fk = col(7, "fk", ColumnType::ForeignKey, false);
    fk.fk_target_table = Some(TableId(0));
    let cols = vec![
        col(0, "bool", ColumnType::Bool, false),
        col(1, "i32", ColumnType::I32, false),
        col(2, "i64", ColumnType::I64, false),
        col(3, "f32", ColumnType::F32, false),
        col(4, "f64", ColumnType::F64, false),
        col(5, "string", ColumnType::String, false),
        col(6, "enum", ColumnType::Enum, false),
        fk,
        col(8, "asset", ColumnType::AssetRef, false),
        col(9, "entity", ColumnType::EntityRef, false),
        col(10, "array", ColumnType::Array, false),
    ];
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("wide"),
        columns: cols,
        primary_key: ColumnId(1),
    };
    let row = Row {
        id: RowId(1),
        parent: None,
        values: vec![
            Value::Bool(true),
            Value::I32(1),
            Value::I64(2),
            Value::F32(3.0),
            Value::F64(4.0),
            Value::String(SmolStr::new_inline("s")),
            Value::Enum {
                schema_id: harmonius_data_tables::EnumSchemaId(1),
                variant: 2,
            },
            Value::ForeignKey(RowRef {
                table: TableId(0),
                row: RowId(1),
            }),
            Value::AssetRef(harmonius_data_tables::AssetHandle(9)),
            Value::EntityRef(harmonius_data_tables::Entity(8)),
            Value::Array(vec![Value::I32(1), Value::I32(2)]),
        ],
    };
    assert_eq!(row.values.len(), 11);
    assert!(schema.validate_row(&row).is_ok());
}

/// TC-16.3.2.1 — load-time validation finds three violations (constructed in Rust).
#[test]
fn tc_16_3_2_1_load_time_validation() {
    let mut fk = col(2, "fk", ColumnType::ForeignKey, false);
    fk.fk_target_table = Some(TableId(1));
    let mut rcol = col(1, "r", ColumnType::I32, false);
    rcol.constraints.push(ColumnConstraint::Range {
        min: 0.0,
        max: 10.0,
    });
    rcol.custom_rule = Some(CustomRule::MustBeEven);
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "id", ColumnType::I32, false), rcol, fk],
        primary_key: ColumnId(0),
    };
    let b = DataTable::try_new(
        TableSchema {
            table_id: TableId(1),
            name: SmolStr::new_inline("b"),
            columns: vec![col(0, "id", ColumnType::I32, false)],
            primary_key: ColumnId(0),
        },
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(1)],
        }],
        1,
    )
    .unwrap();
    let bad = DataTable::try_new(
        schema,
        vec![
            Row {
                id: RowId(1),
                parent: None,
                values: vec![
                    Value::I32(1),
                    Value::I32(3),
                    Value::ForeignKey(RowRef {
                        table: TableId(1),
                        row: RowId(1),
                    }),
                ],
            },
            Row {
                id: RowId(2),
                parent: None,
                values: vec![
                    Value::I32(2),
                    Value::I32(100),
                    Value::ForeignKey(RowRef {
                        table: TableId(1),
                        row: RowId(1),
                    }),
                ],
            },
            Row {
                id: RowId(3),
                parent: None,
                values: vec![
                    Value::I32(3),
                    Value::I32(4),
                    Value::ForeignKey(RowRef {
                        table: TableId(1),
                        row: RowId(99),
                    }),
                ],
            },
        ],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(1), b);
    reg.insert(TableId(0), bad);
    let errs = validate_table(reg.get(TableId(0)).unwrap(), &reg);
    assert_eq!(errs.len(), 3);
}

/// TC-16.3.3.1 — resolved row pointer aliases table storage.
#[test]
fn tc_16_3_3_1_row_ref_zero_copy() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "id", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![Value::I32(42)],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), table);
    let rr = RowRef {
        table: TableId(0),
        row: RowId(1),
    };
    let p0 = reg.resolve_foreign_key(&rr).unwrap() as *const Row;
    let p1 = reg.get(TableId(0)).unwrap().get(RowId(1)).unwrap() as *const Row;
    assert!(ptr::eq(p0, p1));
}

/// TC-16.3.4.1 — three-cycle detected.
#[test]
fn tc_16_3_4_1_proto_chain_cycle_det() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "k", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![
            Row {
                id: RowId(1),
                parent: Some(RowId(3)),
                values: vec![Value::I32(1)],
            },
            Row {
                id: RowId(2),
                parent: Some(RowId(1)),
                values: vec![Value::I32(2)],
            },
            Row {
                id: RowId(3),
                parent: Some(RowId(2)),
                values: vec![Value::I32(3)],
            },
        ],
        1,
    )
    .unwrap();
    let cyc = detect_cycle(&table, RowId(1)).expect("cycle");
    assert!(cyc.contains(&RowId(1)));
    assert!(cyc.contains(&RowId(2)));
    assert!(cyc.contains(&RowId(3)));
}

/// TC-16.3.6.1 — join query pairs items to materials.
#[test]
fn tc_16_3_6_1_join_query_cross_table() {
    let schema_m = TableSchema {
        table_id: TableId(1),
        name: SmolStr::new_inline("Materials"),
        columns: vec![col(0, "id", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let mut fk = col(1, "material_id", ColumnType::ForeignKey, false);
    fk.fk_target_table = Some(TableId(1));
    let schema_i = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("Items"),
        columns: vec![col(0, "id", ColumnType::I32, false), fk],
        primary_key: ColumnId(0),
    };
    let mats = DataTable::try_new(
        schema_m,
        vec![
            Row {
                id: RowId(10),
                parent: None,
                values: vec![Value::I32(10)],
            },
            Row {
                id: RowId(20),
                parent: None,
                values: vec![Value::I32(20)],
            },
            Row {
                id: RowId(30),
                parent: None,
                values: vec![Value::I32(30)],
            },
        ],
        1,
    )
    .unwrap();
    let items = DataTable::try_new(
        schema_i,
        vec![
            Row {
                id: RowId(1),
                parent: None,
                values: vec![
                    Value::I32(1),
                    Value::ForeignKey(RowRef {
                        table: TableId(1),
                        row: RowId(10),
                    }),
                ],
            },
            Row {
                id: RowId(2),
                parent: None,
                values: vec![
                    Value::I32(2),
                    Value::ForeignKey(RowRef {
                        table: TableId(1),
                        row: RowId(20),
                    }),
                ],
            },
            Row {
                id: RowId(3),
                parent: None,
                values: vec![
                    Value::I32(3),
                    Value::ForeignKey(RowRef {
                        table: TableId(1),
                        row: RowId(30),
                    }),
                ],
            },
            Row {
                id: RowId(4),
                parent: None,
                values: vec![
                    Value::I32(4),
                    Value::ForeignKey(RowRef {
                        table: TableId(1),
                        row: RowId(10),
                    }),
                ],
            },
            Row {
                id: RowId(5),
                parent: None,
                values: vec![
                    Value::I32(5),
                    Value::ForeignKey(RowRef {
                        table: TableId(1),
                        row: RowId(20),
                    }),
                ],
            },
        ],
        1,
    )
    .unwrap();
    let bump = Bump::new();
    let j = join_query(&items, ColumnId(1), &mats, None, &bump);
    assert_eq!(j.len(), 5);
    for pair in j {
        let Value::ForeignKey(rr) = &pair.left.values[1] else {
            panic!();
        };
        assert_eq!(pair.right.id, rr.row);
    }
}

/// TC-16.3.7.1 — hash string + btree i32 indices.
#[test]
fn tc_16_3_7_1_hash_btree_indices() {
    let mut name = col(1, "name", ColumnType::String, false);
    name.indexed = true;
    name.index_kind = Some(IndexType::Hash);
    let mut score = col(2, "score", ColumnType::I32, false);
    score.indexed = true;
    score.index_kind = Some(IndexType::BTree);
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![col(0, "id", ColumnType::I32, false), name, score],
        primary_key: ColumnId(0),
    };
    let mut rows = Vec::new();
    for i in 0..1000 {
        let namev = if i % 50 == 0 { "alpha" } else { "other" };
        rows.push(Row {
            id: RowId(i),
            parent: None,
            values: vec![
                Value::I32(i as i32),
                Value::String(SmolStr::new(namev)),
                Value::I32((i % 30) as i32),
            ],
        });
    }
    let table = DataTable::try_new(schema, rows, 1).unwrap();
    let h = table
        .index_lookup(ColumnId(1), &Value::String(SmolStr::new("alpha")))
        .unwrap();
    for id in &h {
        assert_eq!(
            table.get(*id).unwrap().values[1],
            Value::String(SmolStr::new("alpha"))
        );
    }
    let bump = Bump::new();
    let r = table.range(ColumnId(2), &Value::I32(10), &Value::I32(20), &bump);
    let mut brute: Vec<RowId> = table
        .rows()
        .iter()
        .filter(|row| {
            if let Value::I32(s) = row.values[2] {
                (10..=20).contains(&s)
            } else {
                false
            }
        })
        .map(|row| row.id)
        .collect();
    brute.sort_unstable();
    let mut got: Vec<RowId> = r.iter().map(|row| row.id).collect();
    got.sort_unstable();
    assert_eq!(got, brute);
}

/// TC-16.3.9.1 — binding snapshot matches row (codegen stand-in).
#[test]
fn tc_16_3_9_1_ecs_bind_from_row() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("Item"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "name", ColumnType::String, false),
            col(2, "damage", ColumnType::I32, false),
        ],
        primary_key: ColumnId(0),
    };
    let table = DataTable::try_new(
        schema,
        vec![Row {
            id: RowId(1),
            parent: None,
            values: vec![
                Value::I32(1),
                Value::String(SmolStr::new_inline("Sword")),
                Value::I32(7),
            ],
        }],
        1,
    )
    .unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), table);
    let snap = binding_snapshot(
        &reg,
        &DatabaseRow {
            table: TableId(0),
            row: RowId(1),
            bound_columns: smallvec::SmallVec::new(),
            overrides: smallvec::SmallVec::new(),
        },
    )
    .unwrap();
    assert_eq!(
        snap,
        BindingSnapshot::from([
            (SmolStr::new_inline("id"), Value::I32(1)),
            (
                SmolStr::new_inline("name"),
                Value::String(SmolStr::new_inline("Sword")),
            ),
            (SmolStr::new_inline("damage"), Value::I32(7)),
        ])
    );
}

/// TC-16.3.10.1 — native formula symbol.
#[test]
fn tc_16_3_10_1_formula_column_native() {
    assert!((formula_item_dps(10.0, 2.5) - 25.0).abs() < f32::EPSILON);
}

/// TC-16.3.11.1 — in-memory hot reload under 500ms for 10k rows.
#[test]
fn tc_16_3_11_1_hot_reload_10k_time() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("t"),
        columns: vec![
            col(0, "id", ColumnType::I32, false),
            col(1, "v", ColumnType::I32, false),
        ],
        primary_key: ColumnId(0),
    };
    let mut rows_a = Vec::new();
    for i in 0..10_000 {
        rows_a.push(Row {
            id: RowId(i),
            parent: None,
            values: vec![Value::I32(i as i32), Value::I32(0)],
        });
    }
    let t1 = DataTable::try_new(schema.clone(), rows_a, 1).unwrap();
    let mut rows_b = Vec::new();
    for i in 0..10_000 {
        rows_b.push(Row {
            id: RowId(i),
            parent: None,
            values: vec![Value::I32(i as i32), Value::I32(1)],
        });
    }
    let t2 = DataTable::try_new(schema, rows_b, 2).unwrap();
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), t1);
    let mut journal = Vec::new();
    let mut stack = HotReloadStack::default();
    let start = Instant::now();
    stack
        .try_reload(&mut reg, TableId(0), t2, &mut journal)
        .unwrap();
    assert!(start.elapsed().as_millis() < 500);
    assert!(matches!(journal[0], HotReloadEvent::Reloaded(_)));
}

/// TC-16.3.12.1 — load + validate 1M rows under 2s (release); debug uses smaller budget.
#[test]
fn tc_16_3_12_1_full_load_1m_time() {
    let schema = TableSchema {
        table_id: TableId(0),
        name: SmolStr::new_inline("big"),
        columns: vec![col(0, "id", ColumnType::I32, false)],
        primary_key: ColumnId(0),
    };
    let (n, budget_ms) = if cfg!(debug_assertions) {
        (250_000u64, 10_000u128)
    } else {
        (1_000_000u64, 2_000u128)
    };
    let mut rows = Vec::new();
    for i in 0..n {
        rows.push(Row {
            id: RowId(i),
            parent: None,
            values: vec![Value::I32(i as i32)],
        });
    }
    let start = Instant::now();
    let table = DataTable::try_new(schema.clone(), rows, 1).expect("table");
    let mut reg = TableRegistry::default();
    reg.insert(TableId(0), table);
    let errs = validate_all(&reg);
    let elapsed = start.elapsed();
    assert!(errs.is_empty(), "{errs:?}");
    assert!(
        elapsed.as_millis() < budget_ms,
        "load+validate took {:?}",
        elapsed
    );
}
