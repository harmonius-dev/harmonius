# 13.7 — Gameplay Databases

## Schema and Storage

### F-13.7.1 Typed Table Schema Definition

Define gameplay data tables with typed column schemas — each table declares its columns, types,
constraints, and default values via a declarative API or data file. Schemas support primitives,
enums, entity references, asset references, and nested structs. Schema changes are validated at load
time against the data to catch mismatches before runtime.

- **Requirements:** R-13.7.1
- **Dependencies:** F-1.3.1 (Type Registry), F-1.4.1 (Serialization)
- **Platform notes:** None

### F-13.7.2 Row-Based Data Tables

Store gameplay data as rows in typed tables, each row keyed by a unique ID (string or integer).
Tables are loaded from serialized assets (RON, JSON, CSV, or binary) and stored as ECS resources.
Support for cross-table foreign key references — e.g., a loot table row references item table rows
by ID.

- **Requirements:** R-13.7.2
- **Dependencies:** F-13.7.1, F-1.1.23 (World Resources)
- **Platform notes:** None

### F-13.7.3 Curve and Formula Definitions

Define numeric progression curves (leveling XP, stat scaling, damage falloff) as named data assets
with interpolation modes: linear, step, cubic Bezier, and custom expression. Curves are sampled by
level/distance/time and cached. Formula columns reference other table values and curves, enabling
spreadsheet-like computed fields (e.g., `damage = base_damage * level_curve(level)`).

- **Requirements:** R-13.7.3
- **Dependencies:** F-13.7.1
- **Platform notes:** None

### F-13.7.4 Visual Formula Nodes

The formula system uses visual formula nodes authored in the logic graph editor (F-15.8.4), not a
textual DSL. Formulas are composed by connecting math nodes (add, multiply, min, max, clamp, floor,
ceil, lerp, random range) in a visual graph. Column references and curve lookups are represented as
typed input pins on formula nodes. The graph compiles to the same bytecode as gameplay logic graphs.
Used for damage calculations, XP requirements, shop prices, and crafting yields.

- **Requirements:** R-13.7.4
- **Dependencies:** F-13.7.3, F-13.7.2, F-15.8.4 (Gameplay Logic Graphs)
- **Platform notes:** None

### F-13.7.5 Row Inheritance and Prototype Chains

Table rows can declare a parent row. Child rows inherit all column values from the parent,
overriding only specified columns. Inheritance chains support multi-level depth — e.g.,
`Item > Weapon > Sword > FireSword`. Used for item type hierarchies, NPC templates, ability
variants, and equipment slot filtering (a "Head" slot accepts any row whose chain includes the
"Headgear" prototype). Circular inheritance is detected and rejected at validation time.

- **Requirements:** R-13.7.5
- **Dependencies:** F-13.7.2, F-13.7.10
- **Platform notes:** None

### F-13.7.6 Currency and Resource Definitions

Named currency types (gold, gems, crafting tokens, reputation) defined as table rows with display
name, icon, max cap, and decimal precision. Rows in other tables reference currencies for prices,
rewards, and costs. Support for multi-currency transactions (an item costs 100 gold AND 5 gems) and
conversion rates between currencies.

- **Requirements:** R-13.7.6
- **Dependencies:** F-13.7.2
- **Platform notes:** None

### F-13.7.7 Crafting Recipe Tables

Recipe definitions mapping input items (with quantities) to output items (with quantities and
probability). Recipes support catalysts (consumed vs not consumed), tool requirements, skill level
gates, and discovery conditions. Dismantling recipes define reverse transformations. Recipes
reference item rows by ID, enabling the validation system to catch broken references.

- **Requirements:** R-13.7.7
- **Dependencies:** F-13.7.2, F-13.7.5
- **Platform notes:** None

## Content Types

### F-13.7.8 Loot Tables with Weighted Random

Hierarchical loot table definitions with weighted random selection, guaranteed drops, pity counters,
and nested sub-tables. Each entry specifies an item ID, quantity range, weight, and optional
conditions (level range, quest state, faction). The RNG is seeded deterministically for
server-authoritative drops that clients can verify.

- **Requirements:** R-13.7.8
- **Dependencies:** F-13.7.2
- **Platform notes:** None

### F-13.7.9 Stat and Attribute Tables

Define base stats (health, mana, strength, agility), derived stats (crit chance, armor reduction),
and stat modifiers (flat, percentage, multiplicative) in data tables. Modifier stacking rules
(additive group then multiplicative group) are configurable per stat. Stat tables feed into the
ability system (F-13.1.4) and equipment system for tooltip calculation and combat resolution.

- **Requirements:** R-13.7.9
- **Dependencies:** F-13.7.2, F-13.7.3
- **Platform notes:** None

### F-13.7.10 Asset List Tables

Reference tables that map logical names to asset handles — e.g., a weapon table mapping weapon IDs
to mesh, material, VFX, and sound assets. Asset lists support per-platform overrides
(lower-resolution assets on console) and localization variants (region-specific models). Used by the
content pipeline to build asset dependency graphs for streaming.

- **Requirements:** R-13.7.10
- **Dependencies:** F-13.7.2, F-12.3.2 (Asset Metadata)
- **Platform notes:** None

## Querying and Access

### F-13.7.11 Indexed Lookup and Filtering

Build secondary indices on frequently queried columns for O(1) row lookup by key and O(log n) range
queries on indexed columns. Filter expressions support AND/OR/NOT composition with column predicates
(equals, range, contains, regex). Used by the loot system, shop UI, crafting recipes, and quest
condition evaluation.

- **Requirements:** R-13.7.11
- **Dependencies:** F-13.7.2
- **Platform notes:** None

### F-13.7.12 ECS Component Binding

Automatically populate ECS components from database rows when entities are spawned. A `DatabaseRow`
component references a table and row ID; a binding system copies column values into matching
component fields. Supports partial binding (only specified columns), default overrides, and two-way
sync for editor workflows.

- **Requirements:** R-13.7.12
- **Dependencies:** F-13.7.2, F-1.1.4 (Component Registration), F-1.3.3 (Reflection)
- **Platform notes:** None

## Live Operations

### F-13.7.13 Hot Reload and Versioned Patching

Reload data tables at runtime without restarting the server — designers edit a spreadsheet or data
file, hot-reload pushes the changes to the running server, and all ECS component bindings
(F-13.7.12) are refreshed. Table versions are tracked; rollback restores the previous version if a
patch introduces errors. Used for live balance tuning and seasonal events.

- **Requirements:** R-13.7.13
- **Dependencies:** F-13.7.2, F-12.4.1 (Hot Reload)
- **Platform notes:** None

### F-13.7.14 Data Validation and Constraint Checking

Validate all data tables against their schemas on load and after hot-reload: type checks, foreign
key integrity (referenced rows exist), range constraints (stat values within bounds), and custom
validation rules (loot table weights sum to a valid range). Validation errors are reported with
table name, row ID, and column, enabling rapid iteration for content designers.

- **Requirements:** R-13.7.14
- **Dependencies:** F-13.7.1, F-13.7.2
- **Platform notes:** None
