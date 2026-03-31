# R-13.7 — Gameplay Database Requirements

## Schema and Storage

1. **R-13.7.1** — The engine **SHALL** support typed table schemas with primitive, enum, entity
   reference, asset reference, and nested struct column types, validated at load time against data.
   - **Rationale:** Schema validation catches type mismatches before runtime, preventing silent data
     errors.
   - **Verification:** Define a schema with each type. Load valid data and verify success. Load data
     with a type mismatch and verify a clear error is reported.

2. **R-13.7.2** — The engine **SHALL** store gameplay data as rows keyed by unique ID in typed
   tables with cross-table foreign key references, loaded from serialized assets and stored as ECS
   resources.
   - **Rationale:** ECS-accessible tables enable gameplay systems to query data through standard ECS
     patterns.
   - **Verification:** Create two tables with a foreign key from table A to table B. Load both and
     verify the reference resolves. Delete the referenced row and verify validation reports a broken
     foreign key.

3. **R-13.7.3** — The engine **SHALL** support progression curves with linear, step, cubic Bezier,
   and expression interpolation, and visual formula nodes for computed fields composed in the logic
   graph editor.
   - **Rationale:** Visual formulas enable designers to define damage and price calculations without
     code.
   - **Verification:** Define a level curve and verify correct interpolation. Compose a formula node
     graph computing damage and verify output matches expected values.

4. **R-13.7.4** — The engine **SHALL** support row inheritance with multi-level prototype chains and
   circular inheritance detection at validation time.
   - **Rationale:** Prototype chains reduce data duplication across item hierarchies.
   - **Verification:** Define Item > Weapon > Sword chain. Verify child rows inherit parent values
     and override specified columns. Create a circular reference and verify rejection.

5. **R-13.7.5** — The engine **SHALL** support named currency types with max cap, display
   properties, multi-currency transactions, and crafting recipe tables mapping inputs to outputs
   with probability, skill gates, catalysts, and discovery conditions.
   - **Rationale:** Data-driven currencies and recipes enable economy design without code.
   - **Verification:** Define a currency with max cap and verify clamping. Define a recipe with
     skill gate and verify it rejects insufficient skill. Execute a craft with probability and
     verify output varies.

## Content Types

6. **R-13.7.6** — The engine **SHALL** support hierarchical loot tables with weighted random
   selection, guaranteed drops, pity counters, nested sub-tables, and deterministic seeding for
   server-authoritative drops.
   - **Rationale:** Deterministic seeding allows server-authoritative loot that clients can verify.
   - **Verification:** Define a loot table with weights and a pity counter. Roll 1000 times and
     verify distribution matches weights. Verify the pity counter triggers a guaranteed drop after
     the configured threshold.

7. **R-13.7.7** — The engine **SHALL** support stat and attribute tables with flat, percentage, and
   multiplicative modifiers, configurable stacking rules per stat, and asset list tables with
   per-platform overrides.
   - **Rationale:** Data-driven stat tables feed into combat and equipment without hardcoded
     formulas.
   - **Verification:** Define base stats with flat and percentage modifiers. Verify the final value
     matches expected calculation. Verify asset list per-platform override selects the correct
     asset.

## Querying and Live Operations

8. **R-13.7.8** — The engine **SHALL** support indexed lookups on data tables with O(1) key access
   and O(log n) range queries, filter expressions with AND/OR/NOT, and automatic ECS component
   binding from database rows at entity spawn.
   - **Rationale:** Fast lookups are critical for runtime systems querying tables every frame.
   - **Verification:** Create a 10,000-row table with an index. Verify key lookup completes in under
     1 microsecond. Verify range query returns correct results. Spawn an entity with a DatabaseRow
     component and verify component fields are populated.

9. **R-13.7.9** — The engine **SHALL** support hot reload of data tables at runtime without server
   restart, with version tracking, rollback on error, and full validation on reload.
   - **Rationale:** Hot reload enables live balance tuning without downtime.
   - **Verification:** Modify a table value and hot reload. Verify the change is visible to running
     systems. Introduce a validation error and verify rollback to the previous version.
