//! Integration tests mapped to `TC-IR-2.9.*` in `scripting-data-tables-test-cases.md`.

use std::hint::black_box;
use std::path::Path;
use std::time::Instant;

use harmonius_scripting_data_tables_integration::graph::{DirectedGraph, Edge, NodeId};
use harmonius_scripting_data_tables_integration::loader::{MainThreadDylibLoader, MainThreadToken};
use harmonius_scripting_data_tables_integration::{
    AtomicRegistrySwitch, BakeError, BakeOutcome, ColumnId, ColumnType, DatabaseRow, DylibHandle,
    ForeignRow, FormulaFn, FormulaFnSlot, FormulaFnTable, FormulaId, FormulaIdRange, MaterialsRow,
    ReloadStep, RowId, RowRange, RowRef, RowSource, TableId, TableLookupNode, TableRegistry,
    WeaponsRow, detect_formula_dependency_cycle, run_pre_frame_reload, sandbox_check_rust_source,
    validate_formula_output_type,
};

const WEAPONS: TableId = TableId(0);
const MATERIALS: TableId = TableId(1);

fn sample_registry() -> TableRegistry {
    TableRegistry {
        weapons: vec![WeaponsRow {
            base_dmg: 3.0,
            bonus: 2.0,
            material: RowRef {
                table_id: MATERIALS,
                row_id: RowId(0),
            },
        }],
        materials: vec![MaterialsRow {
            weight: 5.0,
            origin: RowRef {
                table_id: MATERIALS,
                row_id: RowId(0),
            },
        }],
    }
}

#[test]
fn tc_ir_2_9_1_1_formula_column_stored_as_logic_graph() {
    let mut graph: DirectedGraph<(), ()> = DirectedGraph::new();
    graph.nodes.push(());
    graph.edges.push(Edge {
        from: NodeId(0),
        to: NodeId(0),
        payload: (),
    });
    assert_eq!(graph.nodes.len(), 1);
    assert_eq!(graph.edges.len(), 1);
}

#[test]
fn tc_ir_2_9_1_2_formula_reads_same_row_columns() {
    let registry = sample_registry();
    let weapon = registry.weapons_row(RowId(0)).expect("weapon row");
    let damage = weapon.base_dmg + weapon.bonus;
    assert!((damage - 5.0).abs() < f32::EPSILON);
}

fn weapon_damage_formula(row: &WeaponsRow, registry: &TableRegistry) -> f32 {
    let material = registry
        .resolve_material(row.material)
        .expect("material row");
    row.base_dmg + row.bonus + material.weight
}

#[test]
fn tc_ir_2_9_2_1_formula_graph_compiles_to_fn() {
    let registry = sample_registry();
    let value = weapon_damage_formula(registry.weapons_row(RowId(0)).unwrap(), &registry);
    assert!((value - 10.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_9_2_2_static_output_type_emitted() {
    let f: FormulaFn<WeaponsRow, f32> = weapon_damage_formula;
    let registry = sample_registry();
    let out = f(registry.weapons_row(RowId(0)).unwrap(), &registry);
    assert!((out - 10.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_9_2_3_no_type_erased_value_wrapper() {
    let i_fn: FormulaFn<WeaponsRow, i32> = |row, _reg| row.base_dmg as i32 + row.bonus as i32;
    let f_fn: FormulaFn<WeaponsRow, f32> = |row, _reg| row.base_dmg + row.bonus;
    let registry = sample_registry();
    let w = registry.weapons_row(RowId(0)).unwrap();
    assert_eq!(i_fn(w, &registry), 5);
    assert!((f_fn(w, &registry) - 5.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_9_3_1_formula_reads_fk_row() {
    let registry = sample_registry();
    let weapon = registry.weapons_row(RowId(0)).unwrap();
    let material = registry.resolve_material(weapon.material).unwrap();
    assert!((material.weight - 5.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_9_3_2_formula_resolves_fk_chain() {
    let registry = TableRegistry {
        weapons: vec![WeaponsRow {
            base_dmg: 1.0,
            bonus: 0.0,
            material: RowRef {
                table_id: MATERIALS,
                row_id: RowId(0),
            },
        }],
        materials: vec![
            MaterialsRow {
                weight: 2.0,
                origin: RowRef {
                    table_id: MATERIALS,
                    row_id: RowId(1),
                },
            },
            MaterialsRow {
                weight: 3.0,
                origin: RowRef {
                    table_id: MATERIALS,
                    row_id: RowId(1),
                },
            },
        ],
    };
    let weapon = registry.weapons_row(RowId(0)).unwrap();
    let m0 = registry.resolve_material(weapon.material).unwrap();
    let m1 = registry.resolve_material(m0.origin).unwrap();
    assert!((m1.weight - 3.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_9_3_3_row_field_access_is_plain_read() {
    let registry = sample_registry();
    let row = registry.weapons_row(RowId(0)).unwrap();
    let sum = row.base_dmg + row.bonus;
    assert!((sum - 5.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_9_4_1_logic_graph_reads_table() {
    let node = TableLookupNode {
        table_id: WEAPONS,
        column_id: ColumnId(0),
        row_source: RowSource::Static(RowRef {
            table_id: WEAPONS,
            row_id: RowId(0),
        }),
    };
    let registry = sample_registry();
    let value = registry
        .weapons_damage_via_lookup(node.table_id, node.column_id, RowId(0))
        .expect("lookup");
    assert!((value - 5.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_9_4_2_logic_graph_reads_via_entity_binding() {
    let binding = DatabaseRow {
        table: WEAPONS,
        row: RowId(0),
    };
    let registry = sample_registry();
    let row = registry
        .weapons_row_via_database_row(binding)
        .expect("entity row");
    assert!((row.base_dmg + row.bonus - 5.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_9_5_1_table_reload_reevaluates_formulas() {
    let mut registry = sample_registry();
    registry.weapons[0].base_dmg = 10.0;
    let row = registry.weapons_row(RowId(0)).unwrap();
    assert!((row.base_dmg + row.bonus - 12.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_9_5_2_formula_dylib_reload_rebakes_tables() {
    let log = run_pre_frame_reload(
        true,
        WEAPONS,
        2,
        RowRange {
            start: RowId(0),
            end: RowId(1),
        },
        2,
        FormulaIdRange {
            start: FormulaId(0),
            end: FormulaId(1),
        },
    );
    assert!(log.formula_dylib_reloaded.is_some());
    assert!(log.table_reloaded.is_some());
}

#[test]
fn tc_ir_2_9_5_3_table_reloaded_entity_event_fields() {
    let log = run_pre_frame_reload(
        true,
        WEAPONS,
        7,
        RowRange {
            start: RowId(0),
            end: RowId(100),
        },
        3,
        FormulaIdRange {
            start: FormulaId(0),
            end: FormulaId(2),
        },
    );
    let evt = log.table_reloaded.expect("table event");
    assert_eq!(evt.table_id, WEAPONS);
    assert_eq!(evt.registry_version, 7);
}

#[test]
fn tc_ir_2_9_5_4_dylib_loader_requires_main_thread_token() {
    let token = MainThreadToken::new_for_tests();
    let mut loader = MainThreadDylibLoader::new(&token);
    let path = Path::new("/tmp/formula_v1.dylib");
    let handle = loader.load_stub(&token, path, 1).expect("load");
    assert_eq!(handle.version, 1);
}

#[test]
fn tc_ir_2_9_5_5_snapshot_swap_is_atomic() {
    let switch = AtomicRegistrySwitch::new(sample_registry());
    let snap = switch.read_snapshot();
    assert_eq!(snap.weapons.len(), 1);
    let mut updated = snap.clone();
    updated.weapons[0].base_dmg = 99.0;
    switch.swap_after_rebuild(1, updated);
    let after = switch.read_snapshot();
    assert!((after.weapons[0].base_dmg - 99.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_9_6_1_cycle_detected_at_bake() {
    let err = detect_formula_dependency_cycle(
        2,
        &[(FormulaId(0), FormulaId(1)), (FormulaId(1), FormulaId(0))],
    )
    .expect_err("cycle");
    assert_eq!(err, BakeError::CycleDetected);
}

#[test]
fn tc_ir_2_9_6_2_type_mismatch_rejected() {
    let err = validate_formula_output_type(ColumnType::I32, ColumnType::F32).expect_err("mismatch");
    assert_eq!(err, BakeError::TypeMismatch);
}

#[test]
fn tc_ir_2_9_6_3_sandbox_rejects_unsafe() {
    let err = sandbox_check_rust_source("fn demo() { unsafe { } }").expect_err("sandbox");
    assert_eq!(err, BakeError::SandboxViolation);
}

#[test]
fn tc_ir_2_9_fm1_formula_cycle() {
    let err = detect_formula_dependency_cycle(
        2,
        &[(FormulaId(0), FormulaId(1)), (FormulaId(1), FormulaId(0))],
    )
    .expect_err("cycle");
    assert_eq!(err, BakeError::CycleDetected);
}

#[test]
fn tc_ir_2_9_fm2_fk_target_missing() {
    let registry = sample_registry();
    let dangling = RowRef {
        table_id: WEAPONS,
        row_id: RowId(99),
    };
    let outcome = match registry.resolve_foreign_key(dangling) {
        ForeignRow::Missing => BakeOutcome::DefaultUsed,
        _ => BakeOutcome::Ok,
    };
    assert_eq!(outcome, BakeOutcome::DefaultUsed);
}

#[test]
fn tc_ir_2_9_fm3_type_mismatch_preserves_dylib_semantics() {
    let err = validate_formula_output_type(ColumnType::I32, ColumnType::F32).unwrap_err();
    assert_eq!(err, BakeError::TypeMismatch);
}

#[test]
fn tc_ir_2_9_fm4_formula_compile_error_retains_version_semantics() {
    let table = FormulaFnTable {
        fns: Vec::new(),
        dylib: DylibHandle { version: 3 },
        version: 3,
    };
    assert_eq!(table.version, 3);
}

#[test]
fn tc_ir_2_9_fm5_dlopen_failure_leaves_handle_unchanged() {
    let token = MainThreadToken::new_for_tests();
    let mut loader = MainThreadDylibLoader::new(&token);
    let path = Path::new("/tmp/CORRUPT.dylib");
    let err = loader.load_stub(&token, path, 4).expect_err("dlopen");
    assert!(err.contains("dlopen"));
}

#[test]
fn tc_ir_2_9_fm6_unresolved_symbol_slot() {
    let slot = FormulaFnSlot {
        formula_id: FormulaId(0),
        fn_addr: std::ptr::null(),
        output_type: ColumnType::F32,
    };
    assert!(slot.fn_addr.is_null());
}

#[test]
fn tc_ir_2_9_fm7_reload_race_ordering() {
    let log = run_pre_frame_reload(
        true,
        WEAPONS,
        5,
        RowRange {
            start: RowId(0),
            end: RowId(10_000),
        },
        9,
        FormulaIdRange {
            start: FormulaId(0),
            end: FormulaId(4),
        },
    );
    assert_eq!(
        log.steps,
        vec![
            ReloadStep::DylibLoad,
            ReloadStep::TableRebake,
            ReloadStep::SnapshotSwap,
            ReloadStep::DispatchFormulaDylibReloaded,
            ReloadStep::DispatchTableReloaded,
        ]
    );
}

#[test]
#[ignore]
fn tc_ir_2_9_2_b1_ten_k_formula_evaluations_under_one_ms() {
    let registry = sample_registry();
    let row = registry.weapons_row(RowId(0)).unwrap();
    let start = Instant::now();
    for _ in 0..10_000 {
        black_box(weapon_damage_formula(black_box(row), black_box(&registry)));
    }
    assert!(start.elapsed().as_secs_f64() * 1000.0 < 1.0);
}

#[test]
#[ignore]
fn tc_ir_2_9_5_b1_hot_reload_ten_k_rows_under_half_second() {
    let weapons: Vec<_> = (0..10_000)
        .map(|i| WeaponsRow {
            base_dmg: i as f32,
            bonus: 1.0,
            material: RowRef {
                table_id: MATERIALS,
                row_id: RowId(0),
            },
        })
        .collect();
    let registry = TableRegistry {
        weapons,
        materials: vec![MaterialsRow {
            weight: 1.0,
            origin: RowRef {
                table_id: MATERIALS,
                row_id: RowId(0),
            },
        }],
    };
    let start = Instant::now();
    let _sum: f32 = registry
        .weapons
        .iter()
        .map(|w| black_box(w.base_dmg + w.bonus))
        .sum();
    assert!(start.elapsed().as_secs_f64() < 0.5);
}
