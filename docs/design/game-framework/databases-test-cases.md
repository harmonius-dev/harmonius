# Gameplay Databases Test Cases

Companion test cases for [databases.md](databases.md).

## Unit Tests

### TC-13.7.1.1 Schema Type Validation

| # | Requirement |
|---|-------------|
| 1 | R-13.7.1    |
| 2 | R-13.7.1    |

1. **#1** — Schema with I32 column; insert `Value::I32(42)`
   - **Expected:** Pass
2. **#2** — Schema with I32 column; insert `Value::String("abc")`
   - **Expected:** Validation error naming column

### TC-13.7.1.2 Schema Constraint Range

| # | Requirement |
|---|-------------|
| 1 | R-13.7.1    |
| 2 | R-13.7.1    |

1. **#1** — Range [0, 100]; insert 50
   - **Expected:** Pass
2. **#2** — Range [0, 100]; insert 200
   - **Expected:** Error naming column and range

### TC-13.7.2.1 Row Unique Key

| # | Requirement |
|---|-------------|
| 1 | R-13.7.2    |

1. **#1** — Insert two rows with same RowId
   - **Expected:** Duplicate rejected

### TC-13.7.2.2 Foreign Key Valid

| # | Requirement |
|---|-------------|
| 1 | R-13.7.2    |

1. **#1** — Table A FK -> Table B row 5; row 5 exists
   - **Expected:** Resolution returns referenced row

### TC-13.7.2.3 Foreign Key Broken

| # | Requirement |
|---|-------------|
| 1 | R-13.7.2    |

1. **#1** — FK referencing nonexistent row 999
   - **Expected:** Validation error with table, row, column

### TC-13.7.2.4 Load RON Format

| # | Requirement |
|---|-------------|
| 1 | R-13.7.2    |

1. **#1** — Load 100-row table from RON file
   - **Expected:** All rows present and typed correctly

### TC-13.7.2.5 Load JSON Format

| # | Requirement |
|---|-------------|
| 1 | R-13.7.2    |

1. **#1** — Load same table from JSON
   - **Expected:** Identical data to RON load

### TC-13.7.2.6 Load CSV Format

| # | Requirement |
|---|-------------|
| 1 | R-13.7.2    |

1. **#1** — Load table from CSV
   - **Expected:** Correct type coercion for all columns

### TC-13.7.3.1 Curve Linear Interpolation

| # | Requirement |
|---|-------------|
| 1 | R-13.7.3    |

1. **#1** — Linear curve: keys at (0,0) and (100,100); sample at 50
   - **Expected:** Output = 50.0

### TC-13.7.3.2 Curve Step Interpolation

| # | Requirement |
|---|-------------|
| 1 | R-13.7.3    |

1. **#1** — Step curve: keys at (0,10) and (50,20); sample at 25
   - **Expected:** Output = 10 (previous key)

### TC-13.7.3.3 Curve Bezier Interpolation

| # | Requirement |
|---|-------------|
| 1 | R-13.7.3    |

1. **#1** — Cubic Bezier curve; sample at midpoint
   - **Expected:** Output within 0.01 tolerance of expected value

### TC-13.7.4.1 Formula Add Multiply

| # | Requirement |
|---|-------------|
| 1 | R-13.7.4    |

1. **#1** — Formula: `(col_a + col_b) * 2`; col_a=10, col_b=5
   - **Expected:** Output = 30.0

### TC-13.7.4.2 Formula Curve Lookup

| # | Requirement |
|---|-------------|
| 1 | R-13.7.4    |

1. **#1** — Formula with CurveLookup; input=50
   - **Expected:** Output matches direct `curve.sample(50)`

### TC-13.7.5.1 Inheritance Single Level

| # | Requirement |
|---|-------------|
| 1 | R-13.7.5    |

1. **#1** — Parent row (col_a=10, col_b=20); child overrides col_a=99
   - **Expected:** Child resolves: col_a=99, col_b=20

### TC-13.7.5.2 Inheritance Chain 3 Levels

| # | Requirement |
|---|-------------|
| 1 | R-13.7.5    |

1. **#1** — Grandparent(a=1,b=2,c=3); parent overrides b=22; child overrides c=33
   - **Expected:** Child resolves: a=1, b=22, c=33

### TC-13.7.5.3 Inheritance Circular Detection

| # | Requirement |
|---|-------------|
| 1 | R-13.7.5    |

1. **#1** — Row A parent=B, Row B parent=A
   - **Expected:** `detect_cycle` returns `Some([A, B])`

### TC-13.7.6.1 Currency Deduct

| # | Requirement |
|---|-------------|
| 1 | R-13.7.6    |

1. **#1** — Wallet: gold=150; deduct 100
   - **Expected:** Balance = 50

### TC-13.7.6.2 Currency Multi-Deduct

| # | Requirement |
|---|-------------|
| 1 | R-13.7.6    |

1. **#1** — Wallet: gold=150, gems=10; deduct 100 gold + 5 gems
   - **Expected:** Gold=50, gems=5

### TC-13.7.6.3 Currency Insufficient

| # | Requirement |
|---|-------------|
| 1 | R-13.7.6    |

1. **#1** — Wallet: gold=50; deduct 100
   - **Expected:** `InsufficientFunds { required: 100, available: 50 }`

### TC-13.7.6.4 Currency Cap

| # | Requirement |
|---|-------------|
| 1 | R-13.7.6    |

1. **#1** — Wallet: gold=900, cap=1000; credit 200
   - **Expected:** Balance clamped to 1000

### TC-13.7.6.5 Currency Conversion

| # | Requirement |
|---|-------------|
| 1 | R-13.7.6    |

1. **#1** — Convert 100 gold to gems at rate 10:1
   - **Expected:** Gold-=100, gems+=10

### TC-13.7.7.1 Recipe Basic

| # | Requirement |
|---|-------------|
| 1 | R-13.7.7    |

1. **#1** — Recipe: 3 iron -> 1 sword; inventory has 5 iron
   - **Expected:** Iron=2, sword=1

### TC-13.7.7.2 Recipe Catalyst

| # | Requirement |
|---|-------------|
| 1 | R-13.7.7    |

1. **#1** — Recipe with hammer catalyst
   - **Expected:** Hammer not consumed after crafting

### TC-13.7.7.3 Recipe Skill Gate

| # | Requirement |
|---|-------------|
| 1 | R-13.7.7    |

1. **#1** — Recipe requires smithing 5; player has 3
   - **Expected:** `CraftError::SkillTooLow { need: 5 }`

### TC-13.7.8.1 Loot Weighted Distribution

| # | Requirement |
|---|-------------|
| 1 | R-13.7.8    |

1. **#1** — Item A weight=3, B weight=1; roll 10,000 times
   - **Expected:** A appears ~75%, B ~25% (within 5%)

### TC-13.7.8.2 Loot Guaranteed Entry

| # | Requirement |
|---|-------------|
| 1 | R-13.7.8    |

1. **#1** — Guaranteed entry in table; roll once
   - **Expected:** Guaranteed item always present

### TC-13.7.8.3 Loot Pity Counter

| # | Requirement |
|---|-------------|
| 1 | R-13.7.8    |

1. **#1** — Pity threshold=10; miss 10 times
   - **Expected:** Roll 11 returns pity item

### TC-13.7.8.4 Loot Deterministic Seed

| # | Requirement |
|---|-------------|
| 1 | R-13.7.8    |

1. **#1** — Same seed, same table; roll twice
   - **Expected:** Identical drops both times

### TC-13.7.9.1 Stat Flat Modifier

| # | Requirement |
|---|-------------|
| 1 | R-13.7.9    |

1. **#1** — Base=100, flat +20
   - **Expected:** Final = 120

### TC-13.7.9.2 Stat Percentage Modifier

| # | Requirement |
|---|-------------|
| 1 | R-13.7.9    |

1. **#1** — Base=100, pct +0.5
   - **Expected:** Final = 150

### TC-13.7.9.3 Stat Standard Stacking

| # | Requirement |
|---|-------------|
| 1 | R-13.7.9    |

1. **#1** — Base=100, flat+20, pct+0.5, mult 1.1
   - **Expected:** Final = (100+20)*1.5*1.1 = 198

### TC-13.7.9.4 Stat Highest Only

| # | Requirement |
|---|-------------|
| 1 | R-13.7.9    |

1. **#1** — HighestOnly rule; mods +10, +20
   - **Expected:** Final = base + 20

### TC-13.7.11.1 Hash Index Lookup

| # | Requirement |
|---|-------------|
| 1 | R-13.7.11   |

1. **#1** — Hash index on 10k rows; lookup by key
   - **Expected:** Correct row returned

### TC-13.7.11.2 BTree Range Query

| # | Requirement |
|---|-------------|
| 1 | R-13.7.11   |

1. **#1** — BTree index; range [50, 100] on 10k rows
   - **Expected:** Correct row subset returned

### TC-13.7.12.1 Binding Spawn

| # | Requirement |
|---|-------------|
| 1 | R-13.7.12   |

1. **#1** — Spawn entity with `DatabaseRow` ref
   - **Expected:** Components populated from row values

### TC-13.7.12.2 Binding Override

| # | Requirement |
|---|-------------|
| 1 | R-13.7.12   |

1. **#1** — Override column A in `DatabaseRow.overrides`
   - **Expected:** Override value used instead of DB value

### TC-13.7.13.1 Hot Reload Valid

| # | Requirement |
|---|-------------|
| 1 | R-13.7.13   |

1. **#1** — Modify table file; trigger hot-reload
   - **Expected:** New values visible; `TableReloaded` event emitted

### TC-13.7.13.2 Hot Reload Invalid

| # | Requirement |
|---|-------------|
| 1 | R-13.7.13   |

1. **#1** — Hot-reload with broken data
   - **Expected:** `ValidationFailed` event; old table unchanged

### TC-13.7.13.3 Hot Reload Rollback

| # | Requirement |
|---|-------------|
| 1 | R-13.7.13   |

1. **#1** — Hot-reload; then rollback
   - **Expected:** Previous version restored

### TC-13.7.14.1 Validation Full

| # | Requirement |
|---|-------------|
| 1 | R-13.7.14   |

1. **#1** — Load tables with type errors, broken FKs, range violations
   - **Expected:** Each error includes table, row, column

## Integration Tests

### TC-13.7.NF2.I1 Load 50 Tables

| # | Requirement |
|---|-------------|
| 1 | R-13.7.NF2  |

1. **#1** — Load 50 tables totaling 1M rows
   - **Expected:** Total load + validate < 2 sec

### TC-13.7.13.I1 Hot Reload Bindings

| # | Requirement |
|---|-------------|
| 1 | R-13.7.13   |

1. **#1** — Hot-reload a table with bound entities
   - **Expected:** All `DatabaseRow` entities updated within 1 frame

### TC-13.7.8.I1 Loot Server Authority

| # | Requirement |
|---|-------------|
| 1 | R-13.7.8    |

1. **#1** — Server rolls loot with seed; client verifies
   - **Expected:** Identical result with same seed

### TC-13.7.4.I1 Formula Evaluation Throughput

| # | Requirement |
|---|-------------|
| 1 | R-13.7.4    |

1. **#1** — Evaluate 10k formula rows
   - **Expected:** > 1M evaluations/sec

## Benchmarks

### TC-13.7.NF1.B1 Key Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Hash index lookup (100k rows) | Wall-clock time | < 1 us | R-13.7.NF1 |

### TC-13.7.11.B1 Range Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | BTree range query (100k rows) | Wall-clock time | < 10 us | R-13.7.11 |

### TC-13.7.NF2.B1 Full Table Load

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Load single table (100k rows) | Wall-clock time | < 200 ms | R-13.7.NF2 |

### TC-13.7.NF2.B2 All Tables Load

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Load 50 tables (1M rows total) | Wall-clock time | < 2 sec | R-13.7.NF2 |

### TC-13.7.NF3.B1 Hot Reload Single Table

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Hot reload 10k-row table | Wall-clock time | < 500 ms | R-13.7.NF3 |

### TC-13.7.4.B1 Formula Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Evaluate 10k formula rows | Throughput | > 1M eval/sec | R-13.7.4 |

### TC-13.7.9.B1 Stat Computation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Compute stat with 1k modifiers | Wall-clock time | < 50 us | R-13.7.9 |

### TC-13.7.8.B1 Loot Roll

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Roll loot table (100 entries) | Wall-clock time | < 10 us | R-13.7.8 |

### TC-13.7.14.B1 Validation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Validate 100k rows | Wall-clock time | < 500 ms | R-13.7.14 |
