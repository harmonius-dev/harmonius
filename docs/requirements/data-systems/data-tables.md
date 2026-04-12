# R-16.3 — Data Tables Requirements

## Schemas

1. **R-16.3.1** — The engine **SHALL** provide typed data table schemas supporting columns of type
   bool, i32, i64, f32, f64, string, enum, foreign key, asset ref, entity ref, and array, with
   per-column constraints (min, max, required, unique).
   - **Rationale:** Data tables are the universal primitive for authored gameplay data; strong
     typing eliminates a class of runtime errors that dynamic tables suffer.
   - **Verification:** Unit test: author a schema with all column types; load a conforming table;
     assert correct type recovery per column.
2. **R-16.3.2** — The engine **SHALL** validate data table schemas at load time with type checks,
   foreign key integrity verification, range constraint enforcement, and custom validation rules
   sourced from the codegen'd validator registry.
   - **Rationale:** Load-time validation catches authoring errors before runtime; custom rules let
     designers express domain constraints.
   - **Verification:** Unit test: load a table with a dangling FK; assert load fails with FK error
     naming the offending row. Load a table with out-of-range value; assert range error.

## Rows

1. **R-16.3.3** — The engine **SHALL** store data table rows as immutable assets loaded as ECS
   resources, with row lookup by primary key returning a `RowRef` handle for zero-copy access.
   - **Rationale:** Immutability enables zero-copy access, deterministic simulation, and safe
     parallel reads without locking.
   - **Verification:** Unit test: load a table; fetch a row by key; mutate attempt fails at compile
     time. Benchmark: 10,000 lookups return consistent RowRefs.
2. **R-16.3.4** — The engine **SHALL** support row inheritance via prototype chains where a row
   inherits column values from a parent row and overrides specific columns, with cycle detection.
   - **Rationale:** Hierarchical data (Weapon > Sword > FireSword) avoids authoring boilerplate and
     keeps related rows in sync when base values change.
   - **Verification:** Unit test: define a parent row with values; define a child overriding one
     column; read child; assert inherited values plus override. Introduce a cycle; assert load
     fails.

## Foreign Keys and Joins

1. **R-16.3.5** — The engine **SHALL** support foreign key columns referencing rows in other data
   tables with load-time integrity validation and runtime `RowRef` resolution.
   - **Rationale:** Cross-table references express relationships (item references rarity, character
     references class) without duplicating data.
   - **Verification:** Unit test: load two tables with an FK from one to the other; resolve FK;
     assert returned RowRef points to the correct row.
2. **R-16.3.6** — The engine **SHALL** support cross-table join queries via foreign key
   relationships with filter predicates, returning composed row results.
   - **Rationale:** Designers and systems need to query across related tables (all items of rarity
     epic, all quests unlocked by class X) without custom per-query plumbing.
   - **Verification:** Unit test: join items to rarities by FK; filter where rarity.tier equals
     "epic"; assert returned set matches expected rows.

## Indices

1. **R-16.3.7** — The engine **SHALL** support hash indices with O(1) exact lookup and BTree indices
   with O(log n) range query as secondary indices on data table columns.
   - **Rationale:** Different queries demand different index structures; hash for exact lookup,
     BTree for range scans (level >= 5 and level <= 10).
   - **Verification:** Benchmark: hash index lookup on a 100,000-row table; assert under 1 µs mean.
     BTree range query returns correct sorted range.

## Locale

1. **R-16.3.8** — The engine **SHALL** support locale-keyed string columns with fallback to the
   default locale when a translation is missing in the requested locale.
   - **Rationale:** Localization must be first-class in data tables; fallback avoids shipping
     untranslated placeholder strings.
   - **Verification:** Unit test: load a table with English and German strings; request French;
     assert fallback to English with a warning log.

## ECS Integration

1. **R-16.3.9** — The engine **SHALL** spawn ECS entities from data table rows with automatic
   component population via codegen'd binding functions that map columns to component fields.
   - **Rationale:** Row-to-entity conversion is boilerplate-heavy; codegen eliminates it and keeps
     entity construction consistent with the authoring schema.
   - **Verification:** Integration test: author a row with transform, mesh, and health columns;
     spawn an entity from the row; assert all three components exist with values from the row.

## Formula Columns

1. **R-16.3.10** — The engine **SHALL** support formula columns in data tables defined via visual
   logic graphs that compile to native Rust functions through the codegen pipeline, evaluating on
   row access with memoization.
   - **Rationale:** Derived stats (damage per second from weapon damage and speed) should update
     automatically when source columns change, without per-row script execution overhead.
   - **Verification:** Unit test: author a formula column returning col_a * col_b; load rows; read
     formula column; assert memoized result equals product.

## Performance

1. **R-16.3.11** — The engine **SHALL** hot-reload a data table of 10,000 rows within 500 ms
   end-to-end including schema validation, index rebuild, and ECS resource swap.
   - **Rationale:** Fast iteration requires near-instant hot reload when designers tweak values in
     the editor.
   - **Verification:** Benchmark: mutate one row in a 10,000-row table; trigger hot reload; assert
     total wall time under 500 ms.
2. **R-16.3.12** — The engine **SHALL** fully load and validate all data tables totalling up to one
   million rows within 2 seconds on desktop hardware at startup.
   - **Rationale:** Startup time is user-visible; large-table games must not impose multi-second
     loads before reaching gameplay.
   - **Verification:** Benchmark: load tables summing to 1,000,000 rows; assert total time under 2 s
     including validation and index build.
