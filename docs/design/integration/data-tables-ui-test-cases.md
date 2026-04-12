# Data Tables ↔ UI Framework Integration Test Cases

All test cases are CI-runnable as `cargo test -p harmonius_ui --test data_tables_ui_integration` and
`cargo bench -p harmonius_ui --bench data_tables_ui_integration`.

Data-tables-to-UI binding is inherently 2D: bound widgets render in 2D screen space. No 3D scene
fixtures are needed for these tests.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.10.1.1 | List populates from table | IR-4.10.1 |
| TC-IR-4.10.1.2 | Empty table shows loading indicator | IR-4.10.1 |
| TC-IR-4.10.1.3 | List recycles on rebind | IR-4.10.1 |
| TC-IR-4.10.1.4 | Query uses arena (zero global allocs) | IR-4.10.1 |
| TC-IR-4.10.2.1 | Column binds to label text | IR-4.10.2 |
| TC-IR-4.10.2.2 | Column binds to progress bar | IR-4.10.2 |
| TC-IR-4.10.2.3 | Column binds to icon | IR-4.10.2 |
| TC-IR-4.10.2.4 | Column binds to visibility | IR-4.10.2 |
| TC-IR-4.10.2.5 | ColumnId cast to usize correctly | IR-4.10.2 |
| TC-IR-4.10.3.1 | Filter by equality | IR-4.10.3 |
| TC-IR-4.10.3.2 | Filter by range | IR-4.10.3 |
| TC-IR-4.10.3.3 | Filter by Contains | IR-4.10.3 |
| TC-IR-4.10.3.4 | Filter change re-populates | IR-4.10.3 |
| TC-IR-4.10.4.1 | Hot reload updates list | IR-4.10.4 |
| TC-IR-4.10.4.2 | Hot reload preserves scroll | IR-4.10.4 |
| TC-IR-4.10.5.1 | FK resolves via Column variant | IR-4.10.5 |
| TC-IR-4.10.5.2 | FK resolves via FirstString | IR-4.10.5 |
| TC-IR-4.10.5.3 | FK default is FirstString | IR-4.10.5 |
| TC-IR-4.10.6.1 | Stat panel reads ability row | IR-4.10.6 |
| TC-IR-4.10.6.2 | Stat panel reads class row | IR-4.10.6 |
| TC-IR-4.10.7.1 | Virtual list pages rows | IR-4.10.7 |
| TC-IR-4.10.7.2 | Scroll advances page offset | IR-4.10.7 |
| TC-IR-4.10.7.3 | Page offset clamps at end | IR-4.10.7 |
| TC-IR-4.10.8.1 | Sort ascending | IR-4.10.8 |
| TC-IR-4.10.8.2 | Sort descending | IR-4.10.8 |
| TC-IR-4.10.8.3 | Sort after filter, before paginate | IR-4.10.8 |

Input and expected output per test case:

1. **TC-IR-4.10.1.1** -- Input: 10-row table bound to a `ListView`. Expected: 10 list item widgets
   spawned as children of the list entity.
2. **TC-IR-4.10.1.2** -- Input: 0-row table bound. Expected: zero children, `LoadingIndicator`
   sibling widget becomes visible via `WidgetProperty::Visible`.
3. **TC-IR-4.10.1.3** -- Input: rebind `DataTableBinding.table_id` to a different table. Expected:
   old item entities returned to `WidgetPool`; new items reused from pool.
4. **TC-IR-4.10.1.4** -- Input: bind table, run `DataBindingSystem::run` one frame under a tracking
   allocator. Expected: zero global-heap allocations during the query phase; arena slabs allocated
   once at startup are excluded.
5. **TC-IR-4.10.2.1** -- Input: `RowColumnBinding { target: LabelText }` on a `String` column.
   Expected: `Label.text == row.values[col.0 as usize]` as string.
6. **TC-IR-4.10.2.2** -- Input: `target: ProgressValue` on an `F32` column with value `0.75`.
   Expected: `ProgressBar.value == 0.75`.
7. **TC-IR-4.10.2.3** -- Input: `target: ImageAsset` on an `AssetRef` column. Expected:
   `Image.asset` equals the referenced handle.
8. **TC-IR-4.10.2.4** -- Input: `target: Visible` on a `Bool` column toggled `true`. Expected:
   widget visibility is `true`; toggling `false` hides the widget the same frame.
9. **TC-IR-4.10.2.5** -- Input: bind `ColumnId(3)` on a 5-column row. Expected: the read performs
   `row.values[3]` via `col.0 as usize` conversion.
10. **TC-IR-4.10.3.1** -- Input: `FilterExpr::Column { col: name_col, predicate: Equals("Sword") }`.
    Expected: only rows where name equals "Sword".
11. **TC-IR-4.10.3.2** -- Input: `predicate: GreaterOrEqual(Value::I32(5))` on `level` column.
    Expected: rows with `level >= 5`.
12. **TC-IR-4.10.3.3** -- Input: `predicate: Contains("axe")` on `name`. Expected: rows whose name
    contains the substring `"axe"`.
13. **TC-IR-4.10.3.4** -- Input: replace the binding's filter mid-run. Expected: list updates the
    next frame; no event plumbing required.
14. **TC-IR-4.10.4.1** -- Input: send `TableReloaded(table_id)` over the MPSC event channel.
    Expected: bound widgets reflect new row values after one frame.
15. **TC-IR-4.10.4.2** -- Input: set `scroll_offset = 40`, fire `TableReloaded`, run one frame.
    Expected: `ListView.scroll_offset == 40` (preserved); visible rows reflect reloaded data at that
    offset.
16. **TC-IR-4.10.5.1** -- Input: `fk_display = Some(Column(display_col))`. Expected: label shows the
    value of `display_col` in the referenced row.
17. **TC-IR-4.10.5.2** -- Input: `fk_display = Some(FirstString)` with a schema whose first `String`
    column is `name`. Expected: label shows the `name` value.
18. **TC-IR-4.10.5.3** -- Input: `fk_display = None`. Expected: resolves via `FirstString`
    convention without panicking; label shows the first string column.
19. **TC-IR-4.10.6.1** -- Input: bind an ability row reference. Expected: `damage` and `cooldown`
    columns populate respective progress/label widgets.
20. **TC-IR-4.10.6.2** -- Input: bind a class row reference. Expected: `hp`, `mana`, and `name`
    columns populate their widgets.
21. **TC-IR-4.10.7.1** -- Input: `page_size=20`, 100-row table. Expected: 20 item widgets visible;
    pool holds 80 recycled.
22. **TC-IR-4.10.7.2** -- Input: scroll down 20 items. Expected: `page_offset == 20`; next 20 rows
    visible.
23. **TC-IR-4.10.7.3** -- Input: set `page_offset = 1000` with 50 rows. Expected: offset clamped to
    `max(0, 50 - page_size)`.
24. **TC-IR-4.10.8.1** -- Input: `sort = Some((level_col, Ascending))`. Expected: items appear in
    ascending `level` order.
25. **TC-IR-4.10.8.2** -- Input: `sort = Some((level_col, Descending))`. Expected: items appear in
    descending `level` order.
26. **TC-IR-4.10.8.3** -- Input: both `filter` and `sort` set. Expected: pipeline order is filter
    first, then sort, then paginate; verified by row identity assertions.

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.10.N.1 | Missing table in registry | IR-4.10.1 |
| TC-IR-4.10.N.2 | Column type mismatch | IR-4.10.2 |
| TC-IR-4.10.N.3 | Dangling FK row | IR-4.10.5 |
| TC-IR-4.10.N.4 | Dangling FK column | IR-4.10.5 |
| TC-IR-4.10.N.5 | Null FK value | IR-4.10.5 |
| TC-IR-4.10.N.6 | Filter matches zero rows | IR-4.10.3 |
| TC-IR-4.10.N.7 | Page offset past end | IR-4.10.7 |
| TC-IR-4.10.N.8 | Empty table with offset zero | IR-4.10.7 |
| TC-IR-4.10.N.9 | Invalid ColumnId on narrow row | IR-4.10.2 |
| TC-IR-4.10.N.10 | TableReloaded channel full | IR-4.10.4 |

Input and expected output per negative test case:

1. **TC-IR-4.10.N.1** -- Input: `TableId` unknown to `TableRegistry`. Expected: empty list,
   `LoadingIndicator` visible, one `tracing::warn!` logged.
2. **TC-IR-4.10.N.2** -- Input: bind a `String` column to `WidgetProperty::ProgressValue`. Expected:
   validation at first bind sets an internal disabled flag, one warn logged; widget shows template
   default; subsequent frames do not re-log.
3. **TC-IR-4.10.N.3** -- Input: FK column references a `RowId` that has been removed from the target
   table. Expected: label shows `"<missing>"`.
4. **TC-IR-4.10.N.4** -- Input: `fk_display = Column(bad_col)` where `bad_col` is out of range for
   the referenced schema. Expected: label shows `"<missing>"`.
5. **TC-IR-4.10.N.5** -- Input: FK column holds `Value::Null`. Expected: label shows `"<missing>"`.
6. **TC-IR-4.10.N.6** -- Input: filter whose predicate matches zero rows. Expected: empty list,
   `NoResultsWidget` visible.
7. **TC-IR-4.10.N.7** -- Input: `page_offset = 1000` with 50 rows. Expected: clamped to
   `max(0, row_count - page_size)`; no panic.
8. **TC-IR-4.10.N.8** -- Input: empty table with `page_offset = 0`. Expected: empty list, no panic,
   `NoResultsWidget` visible.
9. **TC-IR-4.10.N.9** -- Input: `ColumnId(255)` on a 4-column table. Expected: warn logged, row
   skipped; list continues with other rows.
10. **TC-IR-4.10.N.10** -- Input: 300 `TableReloaded` events pushed to the bounded MPSC channel
    (`crossbeam_channel::bounded(256)`). Expected: oldest events dropped, one warn logged, latest
    reload state is visible to the consumer.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.10.1.B1 | Bind 1000-row table | < 2 ms p99 | IR-4.10.1 |
| TC-IR-4.10.1.B2 | Steady-state alloc budget | 0 global allocs | IR-4.10.1 |
| TC-IR-4.10.3.B1 | Filter 10k rows | < 0.5 ms p99 | IR-4.10.3 |
| TC-IR-4.10.5.B1 | Resolve 100 foreign keys | < 0.1 ms p99 | IR-4.10.5 |
| TC-IR-4.10.7.B1 | Virtual list scroll at 60 fps | 0 global allocs | IR-4.10.7 |
| TC-IR-4.10.7.B2 | Scroll frame latency | < 0.5 ms p99 | IR-4.10.7 |
| TC-IR-4.10.8.B1 | Sort 10k-row slice | < 1 ms p99 | IR-4.10.8 |

Benchmark detail:

1. **TC-IR-4.10.1.B1** -- Measures the full `DataBindingSystem::run` cost for a 1000-row table bound
   to a `ListView` with five column bindings. Target: p99 latency under 2 ms.
2. **TC-IR-4.10.1.B2** -- Uses a tracking allocator. After a 10-frame warmup, the arena's
   first-frame slab must be large enough that subsequent frames allocate nothing from the global
   allocator. Arena reset between frames reuses slab memory.
3. **TC-IR-4.10.3.B1** -- Runs `DataTable::query(&filter, &arena)` against a 10,000-row table using
   indexed columns. Target: p99 latency under 0.5 ms.
4. **TC-IR-4.10.5.B1** -- Resolves 100 foreign keys via `TableRegistry::get` plus column lookup.
   Target: p99 latency under 0.1 ms.
5. **TC-IR-4.10.7.B1** -- "Zero allocs" applies to the global allocator only; per-frame arena reset
   reuses its pre-allocated slab. This is the agreed interpretation of "zero steady-state allocs"
   for arena-allocated query results.
6. **TC-IR-4.10.7.B2** -- Measures per-frame scroll latency at 60 fps on a 10k-row table. Target:
   p99 latency under 0.5 ms per frame.
7. **TC-IR-4.10.8.B1** -- Sort uses `slice::sort_unstable_by` (pdqsort) on the filtered result
   slice. Target measured on a 10k-row table with an integer column sort.
