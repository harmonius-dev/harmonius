# Data Tables

Rows of typed data: items, NPCs, abilities, recipes, and any tabular content.

## What it covers

- Typed column schemas (integers, floats, booleans, enums, strings, asset references, entity
  references, arrays).
- Load-time validation: type checking, foreign-key integrity, range constraints, and custom
  rules.
- Immutable rows accessed via zero-copy handles; fast primary-key lookup.
- Row inheritance: prototypes with override columns and cycle detection.
- Foreign keys and cross-table joins with runtime integrity.
- Secondary indices: hash (O(1) lookup) and BTree (O(log n) range).
- Locale-keyed strings with default fallback.
- Formula columns: visual logic graphs compiled to native functions.
- Fast hot reload (10k rows in milliseconds) and efficient cold load (up to 1M rows).

## Concepts

### Schema and Validation

Column types are declared upfront: bool, int, float, enum, string, foreign key, asset ref, entity
ref, or array. Code generation produces native types, validators, and indices. Load-time validation
rejects data that violates types, ranges, or foreign-key constraints.

### Rows and Inheritance

Rows are immutable resources. Prototype-based inheritance chains (e.g., Item > Weapon > Sword >
FireSword) reduce duplication; override columns apply local customizations. Cycle detection prevents
circular prototypes.

### Formulas and Binding

Visual logic graphs in formula columns compile to native code at load time. Results are memoized;
changes to source columns invalidate memoized values. Rows auto-bind to ECS components via codegen'd
functions.

## How it fits

- See [directed-graphs](./directed-graphs.md) for row references embedded in graph nodes.
- See [attributes-and-effects](./attributes-and-effects.md) for item and effect definitions
  stored as table rows.
- See [containers-and-slots](./containers-and-slots.md) for item type lookups.
- Integrates with [game-framework](../game-framework/gameplay-features.md) for all tabular
  content (items, NPCs, recipes, loot tables).
