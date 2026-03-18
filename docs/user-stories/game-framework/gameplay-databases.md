# User Stories — 13.7 Gameplay Databases

## F-13.7.1 Typed Table Schema Definition

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-13.7.1.1 | gameplay director (P-3) | F-13.7.1 | R-13.7.1     |
| US-13.7.1.2 | designer (P-5)          | F-13.7.1 | R-13.7.1     |
| US-13.7.1.3 | engine tester (P-27)    | F-13.7.1 | R-13.7.1     |

1. **US-13.7.1.1** — I want to define gameplay data table schemas with typed columns, constraints,
   and default values via the visual editor so that data tables have well-defined structures without
   writing code
2. **US-13.7.1.2** — I want to use enums, entity references, asset references, and nested structs as
   column types so that table schemas model relationships between game data
3. **US-13.7.1.3** — I want to load data with columns that do not match the schema type and verify a
   clear error is reported so that schema mismatches are detected before runtime

## F-13.7.2 Row-Based Data Tables

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-13.7.2.1 | gameplay director (P-3) | F-13.7.2 | R-13.7.2     |
| US-13.7.2.2 | designer (P-5)          | F-13.7.2 | R-13.7.2     |
| US-13.7.2.3 | engine tester (P-27)    | F-13.7.2 | R-13.7.2     |

1. **US-13.7.2.1** — I want to create row-based data tables keyed by unique IDs and populate them in
   the visual editor so that gameplay data (items, abilities, NPCs) is managed as structured assets
2. **US-13.7.2.2** — I want to reference rows in other tables by ID (e.g., a loot table row
   references item table rows) so that cross-table relationships are explicit and validated
3. **US-13.7.2.3** — I want to create a table with a broken foreign key reference and verify the
   loader reports the specific row and column so that broken references are caught early

## F-13.7.3 Curve and Formula Definitions

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-13.7.3.1 | gameplay director (P-3) | F-13.7.3 | R-13.7.3     |
| US-13.7.3.2 | designer (P-5)          | F-13.7.3 | R-13.7.3     |
| US-13.7.3.3 | engine tester (P-27)    | F-13.7.3 | R-13.7.3     |

1. **US-13.7.3.1** — I want to define numeric curves (leveling XP, stat scaling, damage falloff)
   with interpolation modes (linear, step, cubic Bezier) so that progression tuning is done through
   visual curve editing
2. **US-13.7.3.2** — I want to formula columns to reference curves and other table values so that
   computed fields (e.g., damage = base * level_curve(level)) are derived from authored data
3. **US-13.7.3.3** — I want to sample curves at all interpolation modes and verify output values
   match expected mathematical results so that curve evaluation is numerically correct

## F-13.7.4 Visual Formula Nodes

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-13.7.4.1 | gameplay director (P-3) | F-13.7.4 | R-13.7.4     |
| US-13.7.4.2 | designer (P-5)          | F-13.7.4 | R-13.7.4     |
| US-13.7.4.3 | engine tester (P-27)    | F-13.7.4 | R-13.7.4     |

1. **US-13.7.4.1** — I want to compose formulas by connecting math nodes (add, multiply, min, max,
   clamp, lerp, random range) in the logic graph editor so that damage calculations and XP
   requirements are authored visually
2. **US-13.7.4.2** — I want to column references and curve lookups to appear as typed input pins on
   formula nodes so that formulas integrate with table data without manual wiring
3. **US-13.7.4.3** — I want to compile a visual formula and verify it produces the same bytecode as
   equivalent gameplay logic graphs so that the formula system shares the logic graph runtime

## F-13.7.5 Row Inheritance and Prototype Chains

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-13.7.5.1 | gameplay director (P-3) | F-13.7.5 | R-13.7.5     |
| US-13.7.5.2 | designer (P-5)          | F-13.7.5 | R-13.7.5     |
| US-13.7.5.3 | engine tester (P-27)    | F-13.7.5 | R-13.7.5     |

1. **US-13.7.5.1** — I want to table rows to inherit column values from a parent row, overriding
   only specified columns so that item hierarchies (Item > Weapon > Sword > FireSword) share common
   data
2. **US-13.7.5.2** — I want to equipment slot filtering to accept any row whose chain includes a
   specified prototype (e.g., "Head" slot accepts all "Headgear" descendants) so that slot
   compatibility is driven by data inheritance
3. **US-13.7.5.3** — I want to create a circular inheritance chain and verify it is detected and
   rejected at validation time so that infinite loops in prototype chains are impossible

## F-13.7.6 Currency and Resource Definitions

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-13.7.6.1 | gameplay director (P-3) | F-13.7.6 | R-13.7.6     |
| US-13.7.6.2 | designer (P-5)          | F-13.7.6 | R-13.7.6     |
| US-13.7.6.3 | engine tester (P-27)    | F-13.7.6 | R-13.7.6     |

1. **US-13.7.6.1** — I want to define currency types (gold, gems, crafting tokens, reputation) with
   display name, icon, max cap, and decimal precision so that all currencies are centrally managed
   as data
2. **US-13.7.6.2** — I want to items and services to cost multiple currencies (e.g., 100 gold AND 5
   gems) so that the economy uses layered currency sinks
3. **US-13.7.6.3** — I want to attempt to exceed a currency's max cap and verify the excess is
   rejected or clamped so that currency caps are enforced

## F-13.7.7 Crafting Recipe Tables

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-13.7.7.1 | gameplay director (P-3) | F-13.7.7 | R-13.7.7     |
| US-13.7.7.2 | designer (P-5)          | F-13.7.7 | R-13.7.7     |
| US-13.7.7.3 | engine tester (P-27)    | F-13.7.7 | R-13.7.7     |

1. **US-13.7.7.1** — I want to define recipes mapping input items (with quantities) to output items
   (with quantities and probability) so that the crafting system is entirely data-driven
2. **US-13.7.7.2** — I want to recipes to require minimum skill levels and optional discovery
   conditions before they appear so that recipe availability reflects progression
3. **US-13.7.7.3** — I want to create a recipe referencing a nonexistent item ID and verify the
   validator reports the error so that broken recipes are caught at load time

## F-13.7.8 Loot Tables with Weighted Random

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-13.7.8.1 | gameplay director (P-3) | F-13.7.8 | R-13.7.8     |
| US-13.7.8.2 | designer (P-5)          | F-13.7.8 | R-13.7.8     |
| US-13.7.8.3 | engine tester (P-27)    | F-13.7.8 | R-13.7.8     |

1. **US-13.7.8.1** — I want to create loot tables with weighted random selection, guaranteed drops,
   pity counters, and nested sub-tables so that drop distribution is tunable through data
2. **US-13.7.8.2** — I want to loot entries to have conditions (level range, quest state, faction)
   so that drops are contextually relevant to the player
3. **US-13.7.8.3** — I want to seed the loot RNG with a known value and verify identical drop
   sequences so that server-authoritative drops are deterministic and verifiable

## F-13.7.9 Stat and Attribute Tables

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-13.7.9.1 | gameplay director (P-3) | F-13.7.9 | R-13.7.9     |
| US-13.7.9.2 | designer (P-5)          | F-13.7.9 | R-13.7.9     |
| US-13.7.9.3 | engine tester (P-27)    | F-13.7.9 | R-13.7.9     |

1. **US-13.7.9.1** — I want to define base stats (health, mana, strength) and derived stats (crit
   chance, armor reduction) with configurable modifier stacking rules so that the stat system is
   fully data-driven
2. **US-13.7.9.2** — I want to configure stacking order (additive group then multiplicative group)
   per stat so that modifier interactions follow intended balance rules
3. **US-13.7.9.3** — I want to apply flat, percentage, and multiplicative modifiers to a stat and
   verify the final value matches expected stacking order so that modifier math is correct

## F-13.7.10 Asset List Tables

| ID           | Persona                 | Features  | Requirements |
|--------------|-------------------------|-----------|--------------|
| US-13.7.10.1 | gameplay director (P-3) | F-13.7.10 | R-13.7.10    |
| US-13.7.10.2 | designer (P-5)          | F-13.7.10 | R-13.7.10    |
| US-13.7.10.3 | engine tester (P-27)    | F-13.7.10 | R-13.7.10    |

1. **US-13.7.10.1** — I want to create reference tables mapping item IDs to mesh, material, VFX, and
   sound assets so that the content pipeline builds correct asset dependency graphs
2. **US-13.7.10.2** — I want to asset lists to support per-platform overrides (lower- resolution on
   console) and localization variants so that assets adapt to target hardware
3. **US-13.7.10.3** — I want to load asset list tables on each supported platform and verify all
   asset references resolve so that no platform has missing assets

## F-13.7.11 Indexed Lookup and Filtering

| ID           | Persona                 | Features  | Requirements |
|--------------|-------------------------|-----------|--------------|
| US-13.7.11.1 | gameplay director (P-3) | F-13.7.11 | R-13.7.11    |
| US-13.7.11.2 | designer (P-5)          | F-13.7.11 | R-13.7.11    |
| US-13.7.11.3 | engine tester (P-27)    | F-13.7.11 | R-13.7.11    |

1. **US-13.7.11.1** — I want to build indices on frequently queried columns for O(1) key lookup and
   O(log n) range queries so that loot, shop, and recipe lookups are fast
2. **US-13.7.11.2** — I want to filter rows using AND/OR/NOT composition with column predicates
   (equals, range, contains, regex) so that runtime queries return precise subsets
3. **US-13.7.11.3** — I want to query an indexed table with 10,000 rows and verify O(1) key lookup
   and O(log n) range query performance so that indices provide the expected speedup

## F-13.7.12 ECS Component Binding

| ID           | Persona                 | Features  | Requirements |
|--------------|-------------------------|-----------|--------------|
| US-13.7.12.1 | gameplay director (P-3) | F-13.7.12 | R-13.7.12    |
| US-13.7.12.2 | designer (P-5)          | F-13.7.12 | R-13.7.12    |
| US-13.7.12.3 | engine tester (P-27)    | F-13.7.12 | R-13.7.12    |

1. **US-13.7.12.1** — I want to spawned entities to automatically populate ECS components from
   referenced database rows so that entity templates are driven by data tables without manual
   binding
2. **US-13.7.12.2** — I want to bind only specified columns and override defaults per entity so that
   entities customize their data without full table row duplication
3. **US-13.7.12.3** — I want to modify a component value in the editor and verify it syncs back to
   the database row so that two-way binding works for authoring workflows

## F-13.7.13 Hot Reload and Versioned Patching

| ID           | Persona                 | Features  | Requirements |
|--------------|-------------------------|-----------|--------------|
| US-13.7.13.1 | gameplay director (P-3) | F-13.7.13 | R-13.7.13    |
| US-13.7.13.2 | designer (P-5)          | F-13.7.13 | R-13.7.13    |
| US-13.7.13.3 | engine tester (P-27)    | F-13.7.13 | R-13.7.13    |

1. **US-13.7.13.1** — I want to push data table changes to a running server without restarting so
   that live balance tuning happens with zero downtime
2. **US-13.7.13.2** — I want to roll back a hot-reloaded table to its previous version if the new
   data introduces errors so that bad patches are reversible
3. **US-13.7.13.3** — I want to hot-reload a data table and verify all ECS component bindings
   referencing changed rows are updated so that hot reload propagates consistently

## F-13.7.14 Data Validation and Constraint Checking

| ID           | Persona                 | Features  | Requirements |
|--------------|-------------------------|-----------|--------------|
| US-13.7.14.1 | gameplay director (P-3) | F-13.7.14 | R-13.7.14    |
| US-13.7.14.2 | designer (P-5)          | F-13.7.14 | R-13.7.14    |
| US-13.7.14.3 | engine tester (P-27)    | F-13.7.14 | R-13.7.14    |

1. **US-13.7.14.1** — I want to all data tables to be validated against their schemas on load (type
   checks, foreign key integrity, range constraints) so that data errors are caught before they
   affect gameplay
2. **US-13.7.14.2** — I want to define custom validation rules (e.g., loot table weights sum to a
   valid range) per table so that domain-specific constraints are enforced
3. **US-13.7.14.3** — I want to introduce validation errors in multiple tables and verify each error
   report includes table name, row ID, and column so that designers can locate and fix errors
   quickly
