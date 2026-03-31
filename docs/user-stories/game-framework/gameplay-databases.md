# User Stories — Gameplay Databases (13.7)

## Typed Table Schema

| ID          | Persona                |
|-------------|------------------------|
| US-13.7.1.1 | game designer (P-5)    |
| US-13.7.1.2 | game developer (P-15)  |
| US-13.7.1.3 | engine developer (P-26)|

1. **US-13.7.1.1** — **As a** game designer (P-5), **I want** to define typed table schemas with
   columns, types, and defaults via the visual editor, **so that** data tables are structured
   without code.
2. **US-13.7.1.2** — **As a** game developer (P-15), **I want** schemas to support entity references
   and asset references, **so that** tables link to other game objects.
3. **US-13.7.1.3** — **As a** engine developer (P-26), **I want** schema changes validated at load
   time against data, **so that** mismatches are caught before runtime.

## Row-Based Data Tables

| ID          | Persona              |
|-------------|----------------------|
| US-13.7.2.1 | game designer (P-5)  |
| US-13.7.2.2 | game designer (P-5)  |
| US-13.7.2.3 | game developer (P-15)|

1. **US-13.7.2.1** — **As a** game designer (P-5), **I want** to store gameplay data as rows keyed
   by unique ID in typed tables, **so that** data is organized and queryable.
2. **US-13.7.2.2** — **As a** game designer (P-5), **I want** cross-table foreign key references,
   **so that** loot tables reference item tables by ID.
3. **US-13.7.2.3** — **As a** game developer (P-15), **I want** tables loaded from serialized assets
   and stored as ECS resources, **so that** gameplay systems query data through the ECS.

## Curves and Visual Formula Nodes

| ID          | Persona              |
|-------------|----------------------|
| US-13.7.3.1 | game designer (P-5)  |
| US-13.7.4.1 | game designer (P-5)  |
| US-13.7.4.2 | game designer (P-5)  |

1. **US-13.7.3.1** — **As a** game designer (P-5), **I want** to define progression curves with
   interpolation modes, **so that** leveling XP and stat scaling are tunable.
2. **US-13.7.4.1** — **As a** game designer (P-5), **I want** visual formula nodes in the logic
   graph editor for damage and price calculations, **so that** formulas are composed visually
   without code.
3. **US-13.7.4.2** — **As a** game designer (P-5), **I want** column references and curve lookups as
   typed input pins on formula nodes, **so that** formulas read table data.

## Row Inheritance and Prototype Chains

| ID          | Persona              |
|-------------|----------------------|
| US-13.7.5.1 | game designer (P-5)  |
| US-13.7.5.2 | game developer (P-15)|

1. **US-13.7.5.1** — **As a** game designer (P-5), **I want** table rows to inherit from parent rows
   overriding only specified columns, **so that** item hierarchies like Item > Weapon > Sword are
   concise.
2. **US-13.7.5.2** — **As a** game developer (P-15), **I want** circular inheritance detected and
   rejected at validation time, **so that** data integrity is maintained.

## Currency and Crafting

| ID          | Persona              |
|-------------|----------------------|
| US-13.7.6.1 | game designer (P-5)  |
| US-13.7.7.1 | game designer (P-5)  |
| US-13.7.7.2 | player (P-23)        |

1. **US-13.7.6.1** — **As a** game designer (P-5), **I want** named currency types with max cap,
   display name, and icon, **so that** the game economy is data-driven.
2. **US-13.7.7.1** — **As a** game designer (P-5), **I want** crafting recipe tables mapping inputs
   to outputs with probability and skill gates, **so that** crafting rules are data-driven.
3. **US-13.7.7.2** — [game-specific] **As a** player (P-23), **I want** dismantling recipes that
   reverse crafting, **so that** I recover materials from unwanted items.

## Content Types

| ID           | Persona              |
|--------------|----------------------|
| US-13.7.8.1  | game designer (P-5)  |
| US-13.7.9.1  | game designer (P-5)  |
| US-13.7.10.1 | game developer (P-15)|

1. **US-13.7.8.1** — **As a** game designer (P-5), **I want** hierarchical loot tables with weighted
   random selection and pity counters, **so that** drop rates are tunable.
2. **US-13.7.9.1** — **As a** game designer (P-5), **I want** stat and attribute tables with
   configurable modifier stacking rules, **so that** balance is data-driven.
3. **US-13.7.10.1** — **As a** game developer (P-15), **I want** asset list tables mapping logical
   names to handles with per-platform overrides, **so that** the content pipeline builds dependency
   graphs.

## Querying and Access

| ID           | Persona              |
|--------------|----------------------|
| US-13.7.11.1 | game designer (P-5)  |
| US-13.7.12.1 | game developer (P-15)|
| US-13.7.12.2 | game designer (P-5)  |

1. **US-13.7.11.1** — **As a** game designer (P-5), **I want** indexed lookups and filter
   expressions on data tables, **so that** querying by key or range is fast.
2. **US-13.7.12.1** — **As a** game developer (P-15), **I want** automatic ECS component binding
   from database rows at entity spawn, **so that** column values populate components without manual
   code.
3. **US-13.7.12.2** — **As a** game designer (P-5), **I want** two-way sync between database rows
   and components in the editor, **so that** I preview data changes in real time.

## Live Operations

| ID           | Persona                    |
|--------------|----------------------------|
| US-13.7.13.1 | game designer (P-5)        |
| US-13.7.13.2 | server administrator (P-22)|
| US-13.7.14.1 | game designer (P-5)        |

1. **US-13.7.13.1** — **As a** game designer (P-5), **I want** hot reload of data tables at runtime
   without server restart, **so that** I iterate on balance live.
2. **US-13.7.13.2** — **As a** server administrator (P-22), **I want** table version tracking with
   rollback on error, **so that** bad patches are reverted safely.
3. **US-13.7.14.1** — **As a** game designer (P-5), **I want** data validation on load and after hot
   reload with clear error reports, **so that** broken references are caught.
