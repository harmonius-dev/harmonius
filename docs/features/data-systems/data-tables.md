# 16.3 — Data Tables

## Schemas

| ID       | Feature                        |
|----------|--------------------------------|
| F-16.3.1 | Typed Column Schemas          |
| F-16.3.2 | Load-Time Schema Validation   |

1. **F-16.3.1** — Data table schemas declare typed columns (bool, i32, i64, f32, f64, string, enum,
   foreign key, asset ref, entity ref, array) with per-column constraints (min, max, required,
   unique). Schemas are authored in the editor and codegen'd to Rust structs in the middleman
   `.dylib`.
   - **Deps:** F-1.3 (Codegen), F-1.4.1 (rkyv Serialization)
2. **F-16.3.2** — Schema validation runs at load time with type checks, foreign key integrity
   verification, range constraint enforcement, and custom validation rules sourced from the
   codegen'd validator registry. Validation failures report offending row and column.
   - **Deps:** F-16.3.1

## Rows

| ID       | Feature                        |
|----------|--------------------------------|
| F-16.3.3 | Immutable Rows with RowRef    |
| F-16.3.4 | Row Inheritance Chains        |

1. **F-16.3.3** — Data table rows are immutable assets loaded as ECS resources. Row lookup by
   primary key returns a `RowRef` handle for zero-copy access. Mutable queries return owned copies
   rather than borrowed references.
   - **Deps:** F-1.6 (Asset Loading), F-1.1.1
2. **F-16.3.4** — Rows inherit column values from a parent row via prototype chains, with child rows
   overriding specific columns. Cycle detection runs at load time. Used for hierarchical data like
   `Item > Weapon > Sword > FireSword`.
   - **Deps:** F-16.3.3

## Foreign Keys and Joins

| ID       | Feature                        |
|----------|--------------------------------|
| F-16.3.5 | Cross-Table Foreign Keys      |
| F-16.3.6 | Cross-Table Join Queries      |

1. **F-16.3.5** — Foreign key columns reference rows in other data tables. Load-time validation
   verifies all referenced rows exist. Runtime access resolves FK to a `RowRef`.
   - **Deps:** F-16.3.3
2. **F-16.3.6** — Cross-table join queries combine rows via foreign key relationships with filter
   predicates, returning composed row results. Used for "all items of rarity epic" or "all quests
   unlocked by class X" queries.
   - **Deps:** F-16.3.5

## Indices

| ID       | Feature                        |
|----------|--------------------------------|
| F-16.3.7 | Hash and BTree Indices        |

1. **F-16.3.7** — Data tables support hash indices (O(1) exact lookup) and BTree indices (O(log n)
   range query) as secondary indices on columns. Indices rebuild on table load and hot reload.
   - **Deps:** F-16.3.3

## Locale

| ID       | Feature                        |
|----------|--------------------------------|
| F-16.3.8 | Locale-Keyed String Columns   |

1. **F-16.3.8** — String columns can be locale-keyed with fallback to the default locale when a
   translation is missing. Fallbacks emit a warning log without blocking load. Used for all
   displayed text in localized builds.
   - **Deps:** F-16.3.1, F-13.29 (Localization)

## ECS Integration

| ID       | Feature                        |
|----------|--------------------------------|
| F-16.3.9 | Row-to-Entity Binding          |

1. **F-16.3.9** — Rows spawn ECS entities with automatic component population via codegen'd binding
   functions that map columns to component fields. Binding eliminates per-row boilerplate and keeps
   entity construction consistent with the schema.
   - **Deps:** F-16.3.3, F-1.1.1, F-1.3

## Formula Columns

| ID        | Feature                       |
|-----------|-------------------------------|
| F-16.3.10 | Logic-Graph Formula Columns  |

1. **F-16.3.10** — Formula columns are defined via visual logic graphs that compile to native Rust
   functions through the codegen pipeline. Evaluation is memoized per row read and invalidated when
   source columns change.
   - **Deps:** F-16.3.1, F-15.1 (Logic Graph Editor), F-1.3

## Performance

| ID        | Feature                       |
|-----------|-------------------------------|
| F-16.3.11 | Fast Hot Reload              |
| F-16.3.12 | Fast Startup Load            |

1. **F-16.3.11** — Hot reload of a 10,000-row data table completes in under 500 ms end-to-end,
   including schema validation, index rebuild, and ECS resource swap. Editor iteration feels
   instantaneous.
   - **Deps:** F-16.3.1, F-16.3.3, F-16.3.7
2. **F-16.3.12** — Full load and validation of all data tables up to one million rows total
   completes in under 2 seconds on desktop hardware at startup.
   - **Deps:** F-16.3.1, F-16.3.3, F-16.3.7
