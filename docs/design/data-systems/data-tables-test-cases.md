# Data Tables — Test Cases

Companion to [data-tables.md](data-tables.md).

Test case IDs use `TC-13.7.Z.N` format. Every row links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                          | Req       |
|---------------|-------------------------------|-----------|
| TC-13.7.1.1   | `test_schema_type_validation` | R-13.7.1  |
| TC-13.7.1.2   | `test_schema_constraint_range` | R-13.7.1  |
| TC-13.7.2.1   | `test_row_unique_key`         | R-13.7.2  |
| TC-13.7.3.1   | `test_foreign_key_valid`      | R-13.7.3  |
| TC-13.7.3.2   | `test_foreign_key_broken`     | R-13.7.3  |
| TC-13.7.4.1   | `test_hot_reload_valid`       | R-13.7.4  |
| TC-13.7.4.2   | `test_hot_reload_invalid`     | R-13.7.4  |
| TC-13.7.4.3   | `test_hot_reload_rollback`    | R-13.7.4  |
| TC-13.7.5.1   | `test_inheritance_single`     | R-13.7.5  |
| TC-13.7.5.2   | `test_inheritance_chain_3`    | R-13.7.5  |
| TC-13.7.5.3   | `test_inheritance_circular`   | R-13.7.5  |
| TC-13.7.10.1  | `test_locale_resolves`        | R-13.7.10 |
| TC-13.7.10.2  | `test_locale_fallback`        | R-13.7.10 |
| TC-13.7.10.3  | `test_locale_overlay`         | R-13.7.10 |
| TC-13.7.11.1  | `test_index_hash_lookup`      | R-13.7.11 |
| TC-13.7.11.2  | `test_index_btree_range`      | R-13.7.11 |
| TC-13.7.11.3  | `test_filter_and_or_not`      | R-13.7.11 |
| TC-13.7.12.1  | `test_binding_spawn`          | R-13.7.12 |
| TC-13.7.12.2  | `test_binding_override`       | R-13.7.12 |
| TC-13.7.12.3  | `test_binding_2d_entity`      | R-13.7.12 |
| TC-13.7.14.1  | `test_validation_full`        | R-13.7.14 |
| TC-13.7.14.2  | `test_asset_ref_broken`       | R-13.7.14 |

1. **TC-13.7.1.1** `test_schema_type_validation` — Schema with I32 column; insert matching value
   (pass), then mismatched String value (fail). Assert error names the column.
   - Input: `ColumnDef { col_type: ColumnType::I32 }`, values `Value::I32(42)` then
     `Value::String("x".into())`
   - Expected: first `Ok(())`, second `Err([ValidationError { column: col_id }])`

2. **TC-13.7.1.2** `test_schema_constraint_range` — Range `[0.0, 100.0]`; insert 50 (pass), insert
   200 (fail). Assert error includes column and bound.
   - Input: `ColumnConstraint::Range { min: 0.0, max: 100.0 }`, values 50 and 200
   - Expected: first `Ok(())`, second `Err([ValidationError])`

3. **TC-13.7.2.1** `test_row_unique_key` — Insert two rows with the same `RowId`. Assert duplicate
   is rejected and table remains at one row.
   - Input: two `Row { id: RowId(1), .. }`
   - Expected: second insert returns `Err`, `table.row_count() == 1`

4. **TC-13.7.3.1** `test_foreign_key_valid` — Table A has FK column pointing to Table B row 5; row 5
   exists. Assert `resolve_foreign_key` returns the row.
   - Input: `RowRef { table: B, row: RowId(5) }`, row 5 present in B
   - Expected: `Some(&row_5)`

5. **TC-13.7.3.2** `test_foreign_key_broken` — FK to nonexistent row 999. Assert validation error
   with correct table, row, and column IDs.
   - Input: `RowRef { table: B, row: RowId(999) }`, row absent
   - Expected: `ValidationError { table: A, row: fk_row, column: fk_col, .. }`

6. **TC-13.7.4.1** `test_hot_reload_valid` — Modify table file; trigger reload. Assert new values
   visible and `TableReloaded` event emitted with `old_version + 1`.
   - Input: table at version 1, file changed to add a new string value
   - Expected: `table.version() == 2`, new value present, one `TableReloaded` event

7. **TC-13.7.4.2** `test_hot_reload_invalid` — Reload with schema-mismatched data. Assert
   `ValidationFailed` event, original table unchanged.
   - Input: file with wrong type for a column
   - Expected: `ValidationFailed` emitted, `table.version()` unchanged

8. **TC-13.7.4.3** `test_hot_reload_rollback` — Reload succeeds, then `rollback()` called. Assert
   previous version restored.
   - Input: table at version 1, reload to version 2, call `rollback()`
   - Expected: `table.version() == 1`, original values present

9. **TC-13.7.5.1** `test_inheritance_single` — Parent row `(a=10, b=20)`; child overrides `a=99`.
   Resolved values: `a=99`, `b=20`.
   - Input: parent row id=1 `(a=10, b=20)`, child id=2 `parent=1, a=99, b=null`
   - Expected: `get_resolved(2, col_a) == Value::I32(99)`,
     `get_resolved(2, col_b) == Value::I32(20)`

10. **TC-13.7.5.2** `test_inheritance_chain_3` — Three-level chain A→B→C. Assert value resolution
    picks the nearest non-null ancestor at each level.
    - Input: A `(x=1)`, B `parent=A, x=null, y=2`, C `parent=B, x=null, y=null, z=3`
    - Expected: C resolves `x=1` (from A), `y=2` (from B), `z=3` (own)

11. **TC-13.7.5.3** `test_inheritance_circular` — Row A `parent=B`, row B `parent=A`. Assert
    `detect_cycle` returns path `[A, B]` (or `[B, A]`).
    - Input: two rows with mutual parent references
    - Expected: `detect_cycle` returns `Some(vec![RowId(A), RowId(B)])`

12. **TC-13.7.10.1** `test_locale_resolves` — Localized string column; active locale is `"en-US"`;
    locale value present. Assert resolved value is the locale string.
    - Input: base value `"sword"`, locale `"en-US"` value `"Sword"`,
      `LocalizationManager.active = "en-US"`
    - Expected: `get_resolved` returns `Value::String("Sword")`

13. **TC-13.7.10.2** `test_locale_fallback` — Active locale `"fr-CA"` missing; base locale `"fr"`
    present. Assert fallback returns `"fr"` value.
    - Input: no `"fr-CA"` overlay, `"fr"` overlay present with `"Épée"`
    - Expected: `get_resolved` returns `Value::String("Épée")`

14. **TC-13.7.10.3** `test_locale_overlay` — Overlay table for `"ja"` overrides string column.
    Assert overlay value takes precedence over base value.
    - Input: base `"sword"`, `"ja"` overlay `"剣"`
    - Expected: with locale `"ja"`, `get_resolved` returns `Value::String("剣")`

15. **TC-13.7.11.1** `test_index_hash_lookup` — Table with 10k rows and a Hash index on column C.
    Lookup by exact value returns correct row set.
    - Input: 10k rows with random values on C; lookup C == 42
    - Expected: returned rows all have C == 42; no false positives

16. **TC-13.7.11.2** `test_index_btree_range` — BTree index on column C (integer). Range query
    `[50, 100]` over 10k rows returns exactly the matching subset.
    - Input: 10k rows with C in `[0, 200]`; query `range(col_c, 50, 100)`
    - Expected: result matches brute-force scan of same range

17. **TC-13.7.11.3** `test_filter_and_or_not` — Compound `And(Or(A, B), Not(C))` filter. Assert
    result matches brute-force evaluation of the same expression.
    - Input: 1k rows with three boolean-testable columns
    - Expected: `query(filter)` result set == brute-force scan result set

18. **TC-13.7.12.1** `test_binding_spawn` — Spawn entity with `DatabaseRow` referencing row 1 of a
    table. Assert all bound columns are written to ECS components.
    - Input: `DatabaseRow { table: T, row: RowId(1), bound_columns: [], overrides: [] }`
    - Expected: ECS components on entity match row 1 column values

19. **TC-13.7.12.2** `test_binding_override` — Override column C in `DatabaseRow.overrides`. Assert
    override value is used instead of database value.
    - Input: row has C = 10; `overrides = [(col_c, Value::I32(99))]`
    - Expected: ECS component field for C == 99

20. **TC-13.7.12.3** `test_binding_2d_entity` — Spawn a 2D entity (has `Transform2D`, no
    `Transform`) with a `DatabaseRow`. Assert binding writes to `Transform2D` fields correctly.
    - Input: row with position column; entity has `Transform2D` only
    - Expected: `Transform2D` position updated; no panic about missing `Transform`

21. **TC-13.7.14.1** `test_validation_full` — Table with type errors, broken FKs, and range
    violations. Assert each violation produces a `ValidationError` with correct table, row, column.
    - Input: table with three intentional errors
    - Expected: `validate_table` returns three errors, each with correct IDs and `severity: Error`

22. **TC-13.7.14.2** `test_asset_ref_broken` — Row with `AssetRef` pointing to a deleted asset.
    Assert validation produces a `ValidationError` with `severity: Warning`.
    - Input: `Value::AssetRef(handle_to_missing_asset)`
    - Expected: `ValidationError { severity: Warning, .. }`

## Integration Tests

| ID           | Name                       | Req        |
|--------------|----------------------------|------------|
| TC-13.7.I.1  | `test_load_50_tables`      | R-13.7.NF2 |
| TC-13.7.I.2  | `test_hot_reload_bindings` | R-13.7.4   |
| TC-13.7.I.3  | `test_fk_cross_table`      | R-13.7.3   |
| TC-13.7.I.4  | `test_reverse_lookup`      | R-13.7.3   |
| TC-13.7.I.5  | `test_join_query`          | R-13.7.3   |
| TC-13.7.I.6  | `test_formula_bake_time`   | F-13.7.1   |
| TC-13.7.I.7  | `test_formula_runtime`     | F-13.7.1   |
| TC-13.7.I.8  | `test_asset_ref_streaming` | R-13.7.12  |

1. **TC-13.7.I.1** `test_load_50_tables` — Load 50 tables totalling 1M rows. Assert total load +
   validate time < 2 s on reference hardware.
   - Input: 50 RON files totalling 1M rows
   - Expected: wall time < 2 s, all tables valid, registry has 50 entries

2. **TC-13.7.I.2** `test_hot_reload_bindings` — Hot-reload a table with 100 bound entities. Assert
   all `DatabaseRow` entities are rebound within 1 frame (PreUpdate + AssetReload phase).
   - Input: 100 entities with `DatabaseRow` referencing table T; modify T's file
   - Expected: after next frame, all entities have updated component values; 0 stale bindings

3. **TC-13.7.I.3** `test_fk_cross_table` — Three tables A→B→C with chained FK references. Assert
   full resolution chain returns correct leaf row from C.
   - Input: row in A has FK to B, row in B has FK to C
   - Expected: two-hop `resolve_foreign_key` chain returns C's row

4. **TC-13.7.I.4** `test_reverse_lookup` — Table B has 50 rows that reference row 1 of table A.
   Assert `reverse_lookup(target: A/1, source: B, fk_col: c)` returns all 50 row IDs.
   - Input: 50 rows in B all with FK column pointing to A row 1
   - Expected: result length == 50, all IDs correct

5. **TC-13.7.I.5** `test_join_query` — Join tables Items and Recipes on `item_id`. Assert join
   result contains one `JoinRow` per (item, recipe) pair.
   - Input: 10 items, 15 recipes (some items have multiple recipes)
   - Expected: join result row count == 15; each `JoinRow.left` matches `JoinRow.right.item_id`

6. **TC-13.7.I.6** `test_formula_bake_time` — Formula column `dps = damage * attack_speed` baked at
   asset-build time. Assert loaded binary asset contains pre-computed DPS values.
   - Input: rows with damage and attack_speed; formula marked bake-time
   - Expected: binary asset has static DPS value per row; no runtime evaluation

7. **TC-13.7.I.7** `test_formula_runtime` — Runtime formula column that reads a cross-table FK
   value. Assert value updates when the referenced table is hot-reloaded.
   - Input: formula reads column from FK-referenced table; hot-reload that table
   - Expected: formula result changes after reload; cached value invalidated

8. **TC-13.7.I.8** `test_asset_ref_streaming` — Spawn entity from `DatabaseRow` with an `AssetRef`
   column pointing to a not-yet-loaded asset. Assert entity gets `PendingAsset` marker. Assert
   marker removed and component bound once asset finishes loading.
   - Input: entity spawned before asset is ready
   - Expected: `has_component::<PendingAsset>()` initially true; false after asset ready

## Benchmarks

| ID           | Benchmark                      | Target    | Req        |
|--------------|--------------------------------|-----------|------------|
| TC-13.7.B.1  | Sorted-Vec lookup (100k rows)  | < 1 µs    | R-13.7.NF1 |
| TC-13.7.B.2  | BTree range query (100k rows)  | < 10 µs   | R-13.7.11  |
| TC-13.7.B.3  | Full table load (100k rows)    | < 200 ms  | R-13.7.NF2 |
| TC-13.7.B.4  | All tables load (1M rows)      | < 2 s     | R-13.7.NF2 |
| TC-13.7.B.5  | Hot reload (10k rows)          | < 500 ms  | R-13.7.NF3 |
| TC-13.7.B.6  | Validation (100k rows)         | < 500 ms  | R-13.7.14  |
| TC-13.7.B.7  | rkyv mmap load (100k rows)     | < 5 ms    | R-13.7.NF2 |
| TC-13.7.B.8  | Join query (10k × 10k)         | < 50 ms   | R-13.7.3   |

1. **TC-13.7.B.1** — Binary-search primary-key lookup on a 100k-row sorted `Vec<Row>`. Single lookup
   wall time. Measured with `criterion`.

2. **TC-13.7.B.2** — BTree secondary index range query returning ~1% of 100k rows. Wall time for
   range construction and result collection.

3. **TC-13.7.B.3** — Load and validate one RON table with 100k rows via platform-native I/O.
   End-to-end wall time from `import_table` call to registry insertion.

4. **TC-13.7.B.4** — Load 50 tables totalling 1M rows in parallel. Wall time to full registry
   readiness.

5. **TC-13.7.B.5** — Hot-reload a single 10k-row table: file-watch event → I/O → validate → swap →
   rebind. Wall time end-to-end.

6. **TC-13.7.B.6** — `validate_all` over one table with 100k rows (type checks + FK integrity +
   range constraints). Wall time.

7. **TC-13.7.B.7** — `load_binary_table` on a 100k-row rkyv binary file via mmap. Wall time from
   `mmap` call to first `get()` call succeeding. Must not allocate (zero deserialization).

8. **TC-13.7.B.8** — `join_query` over two tables each with 10k rows, one FK column. Arena
   allocated. Wall time for full join result construction.
