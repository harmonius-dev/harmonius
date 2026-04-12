# Data Tables User Stories

## Schemas

| ID        | Persona                 |
|-----------|-------------------------|
| US-16.3.1 | game designer (P-5)     |
| US-16.3.2 | QA lead (P-20)          |

1. **US-16.3.1** — **As a** game designer (P-5), **I want** typed column schemas with bool, int,
   float, string, enum, foreign key, and array support plus per-column constraints, **so that** I
   can define gameplay data with compile-time type safety.
2. **US-16.3.2** — **As a** QA lead (P-20), **I want** load-time schema validation reporting the
   offending row and column on failure, **so that** I can catch authoring errors before runtime.

## Rows

| ID        | Persona               |
|-----------|-----------------------|
| US-16.3.3 | game developer (P-15) |
| US-16.3.4 | game designer (P-5)   |

1. **US-16.3.3** — **As a** game developer (P-15), **I want** immutable data table rows accessed by
   RowRef handles for zero-copy reads, **so that** my gameplay code can query data without locking
   or cloning.
2. **US-16.3.4** — **As a** game designer (P-5), **I want** to inherit row values from parent rows
   through prototype chains, **so that** I can author hierarchies like Item > Weapon > Sword >
   FireSword without duplicating values.

## Foreign Keys

| ID        | Persona             |
|-----------|---------------------|
| US-16.3.5 | game designer (P-5) |
| US-16.3.6 | game designer (P-5) |

1. **US-16.3.5** — **As a** game designer (P-5), **I want** foreign key columns referencing rows in
   other tables with load-time integrity checks, **so that** cross-table relations like
   item-to-rarity are validated before runtime.
2. **US-16.3.6** — **As a** game designer (P-5), **I want** to join tables across foreign keys with
   filter predicates, **so that** I can query "all items of rarity epic" without writing custom
   traversal code.

## Indices

| ID        | Persona                 |
|-----------|-------------------------|
| US-16.3.7 | engine developer (P-26) |

1. **US-16.3.7** — **As an** engine developer (P-26), **I want** hash and BTree secondary indices on
   data table columns, **so that** exact and range queries on large tables stay under 1 µs per
   lookup.

## Locale

| ID        | Persona                       |
|-----------|-------------------------------|
| US-16.3.8 | localization specialist (P-18)|

1. **US-16.3.8** — **As a** localization specialist (P-18), **I want** locale-keyed string columns
   with fallback to the default locale on missing translation, **so that** builds never ship with
   empty strings but I can see which keys still need translating.

## ECS Integration

| ID        | Persona               |
|-----------|-----------------------|
| US-16.3.9 | game developer (P-15) |

1. **US-16.3.9** — **As a** game developer (P-15), **I want** to spawn ECS entities from data table
   rows with automatic component population, **so that** I never write manual binding code mapping
   columns to components.

## Formula Columns

| ID         | Persona             |
|------------|---------------------|
| US-16.3.10 | game designer (P-5) |

1. **US-16.3.10** — **As a** game designer (P-5), **I want** computed columns in data tables
   authored via visual logic graphs, **so that** derived stats like damage-per-second update
   automatically when weapon damage or speed changes.

## Performance

| ID         | Persona                 |
|------------|-------------------------|
| US-16.3.11 | game designer (P-5)     |
| US-16.3.12 | engine developer (P-26) |

1. **US-16.3.11** — **As a** game designer (P-5), **I want** hot reload of a 10,000-row table to
   complete in under 500 ms, **so that** tweaking values in the editor feels instantaneous.
2. **US-16.3.12** — **As an** engine developer (P-26), **I want** full table load of up to one
   million rows at startup to complete within 2 seconds, **so that** players reach gameplay quickly
   on content-heavy games.
