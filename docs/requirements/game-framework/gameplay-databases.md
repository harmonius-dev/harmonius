# R-13.7 — Gameplay Database Requirements

## Schema and Storage

| ID       | Derived From                                                    |
|----------|-----------------------------------------------------------------|
| R-13.7.1 | [F-13.7.1](../../features/game-framework/gameplay-databases.md) |
| R-13.7.2 | [F-13.7.2](../../features/game-framework/gameplay-databases.md) |
| R-13.7.3 | [F-13.7.3](../../features/game-framework/gameplay-databases.md) |
| R-13.7.4 | [F-13.7.4](../../features/game-framework/gameplay-databases.md) |
| R-13.7.5 | [F-13.7.5](../../features/game-framework/gameplay-databases.md) |
| R-13.7.6 | [F-13.7.6](../../features/game-framework/gameplay-databases.md) |
| R-13.7.7 | [F-13.7.7](../../features/game-framework/gameplay-databases.md) |

1. **R-13.7.1** — The engine **SHALL** define gameplay data tables with typed column schemas
   (primitives, enums, entity references, asset references, nested structs) including constraints
   and defaults via a declarative API or data file, validating schemas against loaded data at load
   time.
   - **Rationale:** Schema-first table definitions catch data mismatches before runtime and enable
     tooling to generate editors and validators automatically.
   - **Verification:** Unit test: define a schema with each supported column type and load
     conforming data; assert success. Load data with a type mismatch and assert a validation error
     naming the column and expected type.
2. **R-13.7.2** — The engine **SHALL** store gameplay data as uniquely-keyed rows in typed tables
   loaded from serialized assets (RON, JSON, CSV, or binary), stored as ECS resources, with support
   for cross-table foreign key references by row ID.
   - **Rationale:** ECS-resource-based tables enable system-level queries and scheduling, while
     foreign key references maintain relational integrity across content tables.
   - **Verification:** Unit test: load a table from each supported format and verify row retrieval
     by key. Create two tables with a foreign key reference and verify the referenced row is
     resolved correctly. Assert duplicate key insertion is rejected.
3. **R-13.7.3** — The engine **SHALL** define named numeric progression curves with interpolation
   modes (linear, step, cubic Bezier, custom expression) sampled by level, distance, or time, and
   support formula columns that reference other table values and curves for computed fields.
   - **Rationale:** Data-driven curves and formulas enable designers to tune progression and balance
     without code changes, reducing iteration time.
   - **Verification:** Unit test: define curves with each interpolation mode and sample at known
     inputs; assert outputs match expected values within floating-point tolerance. Define a formula
     column referencing a curve and verify the computed value updates when the curve changes.
4. **R-13.7.4** — The engine **SHALL** provide a visual formula node system within the logic graph
   editor for computed database values. Formulas are authored by connecting math nodes (add,
   multiply, min, max, clamp, floor, ceil, lerp, random range) with column reference and curve
   lookup nodes in a visual graph that compiles to the same bytecode as gameplay logic graphs. No
   textual formula syntax **SHALL** be exposed to users.
   - **Rationale:** Visual formula nodes maintain the no-code constraint while providing runtime
     flexibility for balance tuning, compiling to optimized bytecode without per-evaluation parse
     overhead.
   - **Verification:** Unit test: author a formula graph using each math node type and assert
     correct results. Verify that a compiled formula graph produces identical results to direct
     computation. Benchmark: evaluate 10,000 formula rows and verify throughput exceeds 1 million
     evaluations per second.
5. **R-13.7.5** — The engine **SHALL** support row inheritance where child rows inherit all column
   values from a parent row, overriding only specified columns, with multi-level prototype chains
   and circular inheritance detection at validation time.
   - **Rationale:** Prototype chains reduce data duplication for hierarchical content types (items,
     NPCs, abilities) and enable slot filtering by ancestry.
   - **Verification:** Unit test: create a 3-level inheritance chain and verify the leaf row
     inherits unoverridden values from both ancestors. Define a circular chain and assert validation
     rejects it with an error identifying the cycle.
6. **R-13.7.6** — The engine **SHALL** define named currency types as table rows with display name,
   icon, maximum cap, and decimal precision, supporting multi-currency transactions and conversion
   rates between currency types.
   - **Rationale:** Table-driven currency definitions enable designers to add new currencies and set
     economy parameters without code changes.
   - **Verification:** Unit test: define two currencies with caps and precision, execute a
     multi-currency transaction (deducting both), and verify balances. Attempt a transaction
     exceeding the cap and assert it is rejected. Verify currency conversion at a defined rate.
7. **R-13.7.7** — The engine **SHALL** define crafting recipes mapping input items (with quantities)
   to output items (with quantities and probability), supporting catalysts, tool requirements, skill
   level gates, discovery conditions, and reverse dismantling recipes, with item references
   validated against the item table.
   - **Rationale:** Data-driven recipe tables enable content designers to author crafting systems
     entirely through data, with validation catching broken item references at load time.
   - **Verification:** Unit test: define a recipe with consumed inputs, a catalyst, and a tool
     requirement; execute it and verify inputs are consumed, catalyst is retained, and output is
     granted. Define a recipe referencing a nonexistent item ID and assert validation failure.

## Content Types

| ID        | Derived From                                                     |
|-----------|------------------------------------------------------------------|
| R-13.7.8  | [F-13.7.8](../../features/game-framework/gameplay-databases.md)  |
| R-13.7.9  | [F-13.7.9](../../features/game-framework/gameplay-databases.md)  |
| R-13.7.10 | [F-13.7.10](../../features/game-framework/gameplay-databases.md) |

1. **R-13.7.8** — The engine **SHALL** support hierarchical loot table definitions with weighted
   random selection, guaranteed drops, pity counters, nested sub-tables, and per-entry conditions,
   using deterministically seeded RNG for server-authoritative drops.
   - **Rationale:** Deterministic seeded RNG enables server-authoritative loot generation that
     clients can verify, while pity counters ensure bounded worst-case drop rates.
   - **Verification:** Unit test: roll a loot table 10,000 times with a fixed seed and verify drop
     distribution matches expected weights within a 5% tolerance. Verify guaranteed drops appear on
     every roll. Test pity counter triggers after the configured miss streak.
2. **R-13.7.9** — The engine **SHALL** define base stats, derived stats, and stat modifiers (flat,
   percentage, multiplicative) in data tables with configurable stacking rules (additive group then
   multiplicative group) per stat, feeding into the ability and equipment systems.
   - **Rationale:** Data-driven stat definitions with configurable stacking rules enable balance
     designers to tune combat math without code changes.
   - **Verification:** Unit test: apply flat, percentage, and multiplicative modifiers to a base
     stat and verify the final value matches the expected stacking order. Verify tooltip calculation
     matches combat resolution output for the same modifier set.
3. **R-13.7.10** — The engine **SHALL** provide reference tables mapping logical names to asset
   handles with per-platform overrides and localization variants, used by the content pipeline to
   build asset dependency graphs for streaming.
   - **Rationale:** Indirection from logical names to asset handles enables platform-specific and
     locale-specific asset substitution without changing gameplay data.
   - **Verification:** Unit test: resolve a logical name on two platforms and verify distinct asset
     handles are returned. Verify the content pipeline extracts the correct dependency graph from an
     asset list table.

## Querying and Access

| ID        | Derived From                                                     |
|-----------|------------------------------------------------------------------|
| R-13.7.11 | [F-13.7.11](../../features/game-framework/gameplay-databases.md) |
| R-13.7.12 | [F-13.7.12](../../features/game-framework/gameplay-databases.md) |

1. **R-13.7.11** — The engine **SHALL** build secondary indices on designated columns providing O(1)
   key lookup and O(log n) range queries, with filter expressions supporting AND/OR/NOT composition
   over column predicates (equals, range, contains, regex).
   - **Rationale:** Indexed lookup avoids linear scans in hot paths (loot rolls, shop filtering,
     quest condition checks) that would otherwise degrade with table size.
   - **Verification:** Benchmark: index a 100,000-row table and verify key lookup completes in under
     1 microsecond. Unit test: apply a compound AND/OR/NOT filter and verify the result set matches
     a brute-force scan of the same table.
2. **R-13.7.12** — The engine **SHALL** automatically populate ECS components from database rows
   when entities are spawned via a `DatabaseRow` component referencing a table and row ID,
   supporting partial binding, default overrides, and two-way sync for editor workflows.
   - **Rationale:** Automatic binding eliminates boilerplate entity initialization code and ensures
     spawned entities always reflect the latest data table values.
   - **Verification:** Integration test: spawn an entity with a `DatabaseRow` component and verify
     all bound component fields match the referenced row values. Modify a bound column in the editor
     and verify two-way sync updates the component.

## Live Operations

| ID        | Derived From                                                     |
|-----------|------------------------------------------------------------------|
| R-13.7.13 | [F-13.7.13](../../features/game-framework/gameplay-databases.md) |
| R-13.7.14 | [F-13.7.14](../../features/game-framework/gameplay-databases.md) |

1. **R-13.7.13** — The engine **SHALL** reload data tables at runtime without server restart,
   refreshing all ECS component bindings, tracking table versions, and supporting rollback to the
   previous version if a patch introduces validation errors.
   - **Rationale:** Runtime hot-reload enables live balance tuning during development and live
     service without downtime, while version tracking prevents broken patches from persisting.
   - **Verification:** Integration test: modify a data table file, trigger hot-reload, and verify
     bound ECS components reflect the new values within one frame. Introduce a validation error in a
     patch and verify automatic rollback restores the previous table version.
2. **R-13.7.14** — The engine **SHALL** validate all data tables against their schemas on load and
   after hot-reload, checking type correctness, foreign key integrity, range constraints, and custom
   validation rules, reporting errors with table name, row ID, and column.
   - **Rationale:** Comprehensive validation at load time catches content errors before they reach
     runtime, reducing designer iteration time and preventing data-driven bugs.
   - **Verification:** Unit test: load tables with type mismatches, broken foreign keys,
     out-of-range values, and invalid loot table weights; assert each produces a validation error
     identifying the exact table, row, and column. Verify valid tables pass without errors.

## Non-Functional Requirements

| ID         | Derived From        |
|------------|---------------------|
| R-13.7.NF1 | F-13.7.2, F-13.7.11 |
| R-13.7.NF2 | F-13.7.1, F-13.7.14 |
| R-13.7.NF3 | F-13.7.13           |

1. **R-13.7.NF1** — The engine **SHALL** support data tables with at least 100,000 rows without
   degrading indexed lookup performance below 1 microsecond per key lookup or exceeding 500 MB of
   memory for the largest single table.
   - **Rationale:** Large-scale games may contain tens of thousands of items, NPCs, and quests; the
     database must scale without compromising query performance.
   - **Verification:** Load a table with 100,000 rows, build secondary indices, and benchmark key
     lookup. Verify 99th percentile lookup latency is under 1 microsecond. Measure memory footprint
     and verify it stays under 500 MB.
2. **R-13.7.NF2** — The engine **SHALL** load and validate all gameplay data tables within 2 seconds
   during game startup for a dataset of up to 1 million total rows across all tables.
   - **Rationale:** Slow table loading blocks game startup and editor reload cycles, degrading
     designer iteration speed.
   - **Verification:** Create a dataset with 1 million total rows across 50 tables. Measure load and
     validation time from file read to indexed-and-ready state. Verify total time is under 2 seconds
     on target hardware.
3. **R-13.7.NF3** — The engine **SHALL** complete a data table hot-reload (file read, validation,
   ECS binding refresh) within 500 ms for a single table of up to 10,000 rows, enabling rapid
   live-tuning iteration.
   - **Rationale:** Hot reload that takes longer than 500 ms interrupts the designer's flow when
     iterating on balance values during a live play session.
   - **Verification:** Modify a 10,000-row table file, trigger hot-reload, and measure time from
     file change detection to all ECS component bindings reflecting the new values. Verify total
     latency is under 500 ms.
