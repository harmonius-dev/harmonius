# Gameplay Databases Test Cases

Companion test cases for [databases.md](databases.md).

## Unit Tests

### TC-13.7.1.1 Schema Type Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Schema with I32 column; insert `Value::I32(42)` | Pass | R-13.7.1 |
| 2 | Schema with I32 column; insert `Value::String("abc")` | Validation error naming column | R-13.7.1 |

### TC-13.7.1.2 Schema Constraint Range

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Range [0, 100]; insert 50 | Pass | R-13.7.1 |
| 2 | Range [0, 100]; insert 200 | Error naming column and range | R-13.7.1 |

### TC-13.7.2.1 Row Unique Key

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert two rows with same RowId | Duplicate rejected | R-13.7.2 |

### TC-13.7.2.2 Foreign Key Valid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Table A FK -> Table B row 5; row 5 exists | Resolution returns referenced row | R-13.7.2 |

### TC-13.7.2.3 Foreign Key Broken

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | FK referencing nonexistent row 999 | Validation error with table, row, column | R-13.7.2 |

### TC-13.7.2.4 Load RON Format

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load 100-row table from RON file | All rows present and typed correctly | R-13.7.2 |

### TC-13.7.2.5 Load JSON Format

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load same table from JSON | Identical data to RON load | R-13.7.2 |

### TC-13.7.2.6 Load CSV Format

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load table from CSV | Correct type coercion for all columns | R-13.7.2 |

### TC-13.7.3.1 Curve Linear Interpolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Linear curve: keys at (0,0) and (100,100); sample at 50 | Output = 50.0 | R-13.7.3 |

### TC-13.7.3.2 Curve Step Interpolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Step curve: keys at (0,10) and (50,20); sample at 25 | Output = 10 (previous key) | R-13.7.3 |

### TC-13.7.3.3 Curve Bezier Interpolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cubic Bezier curve; sample at midpoint | Output within 0.01 tolerance of expected value | R-13.7.3 |

### TC-13.7.4.1 Formula Add Multiply

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Formula: `(col_a + col_b) * 2`; col_a=10, col_b=5 | Output = 30.0 | R-13.7.4 |

### TC-13.7.4.2 Formula Curve Lookup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Formula with CurveLookup; input=50 | Output matches direct `curve.sample(50)` | R-13.7.4 |

### TC-13.7.5.1 Inheritance Single Level

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent row (col_a=10, col_b=20); child overrides col_a=99 | Child resolves: col_a=99, col_b=20 | R-13.7.5 |

### TC-13.7.5.2 Inheritance Chain 3 Levels

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Grandparent(a=1,b=2,c=3); parent overrides b=22; child overrides c=33 | Child resolves: a=1, b=22, c=33 | R-13.7.5 |

### TC-13.7.5.3 Inheritance Circular Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Row A parent=B, Row B parent=A | `detect_cycle` returns `Some([A, B])` | R-13.7.5 |

### TC-13.7.6.1 Currency Deduct

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wallet: gold=150; deduct 100 | Balance = 50 | R-13.7.6 |

### TC-13.7.6.2 Currency Multi-Deduct

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wallet: gold=150, gems=10; deduct 100 gold + 5 gems | Gold=50, gems=5 | R-13.7.6 |

### TC-13.7.6.3 Currency Insufficient

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wallet: gold=50; deduct 100 | `InsufficientFunds { required: 100, available: 50 }` | R-13.7.6 |

### TC-13.7.6.4 Currency Cap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wallet: gold=900, cap=1000; credit 200 | Balance clamped to 1000 | R-13.7.6 |

### TC-13.7.6.5 Currency Conversion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Convert 100 gold to gems at rate 10:1 | Gold-=100, gems+=10 | R-13.7.6 |

### TC-13.7.7.1 Recipe Basic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Recipe: 3 iron -> 1 sword; inventory has 5 iron | Iron=2, sword=1 | R-13.7.7 |

### TC-13.7.7.2 Recipe Catalyst

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Recipe with hammer catalyst | Hammer not consumed after crafting | R-13.7.7 |

### TC-13.7.7.3 Recipe Skill Gate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Recipe requires smithing 5; player has 3 | `CraftError::SkillTooLow { need: 5 }` | R-13.7.7 |

### TC-13.7.8.1 Loot Weighted Distribution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Item A weight=3, B weight=1; roll 10,000 times | A appears ~75%, B ~25% (within 5%) | R-13.7.8 |

### TC-13.7.8.2 Loot Guaranteed Entry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Guaranteed entry in table; roll once | Guaranteed item always present | R-13.7.8 |

### TC-13.7.8.3 Loot Pity Counter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pity threshold=10; miss 10 times | Roll 11 returns pity item | R-13.7.8 |

### TC-13.7.8.4 Loot Deterministic Seed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same seed, same table; roll twice | Identical drops both times | R-13.7.8 |

### TC-13.7.9.1 Stat Flat Modifier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base=100, flat +20 | Final = 120 | R-13.7.9 |

### TC-13.7.9.2 Stat Percentage Modifier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base=100, pct +0.5 | Final = 150 | R-13.7.9 |

### TC-13.7.9.3 Stat Standard Stacking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base=100, flat+20, pct+0.5, mult 1.1 | Final = (100+20)*1.5*1.1 = 198 | R-13.7.9 |

### TC-13.7.9.4 Stat Highest Only

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | HighestOnly rule; mods +10, +20 | Final = base + 20 | R-13.7.9 |

### TC-13.7.11.1 Hash Index Lookup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hash index on 10k rows; lookup by key | Correct row returned | R-13.7.11 |

### TC-13.7.11.2 BTree Range Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | BTree index; range [50, 100] on 10k rows | Correct row subset returned | R-13.7.11 |

### TC-13.7.12.1 Binding Spawn

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn entity with `DatabaseRow` ref | Components populated from row values | R-13.7.12 |

### TC-13.7.12.2 Binding Override

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Override column A in `DatabaseRow.overrides` | Override value used instead of DB value | R-13.7.12 |

### TC-13.7.13.1 Hot Reload Valid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify table file; trigger hot-reload | New values visible; `TableReloaded` event emitted | R-13.7.13 |

### TC-13.7.13.2 Hot Reload Invalid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hot-reload with broken data | `ValidationFailed` event; old table unchanged | R-13.7.13 |

### TC-13.7.13.3 Hot Reload Rollback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hot-reload; then rollback | Previous version restored | R-13.7.13 |

### TC-13.7.14.1 Validation Full

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load tables with type errors, broken FKs, range violations | Each error includes table, row, column | R-13.7.14 |

## Integration Tests

### TC-13.7.NF2.I1 Load 50 Tables

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load 50 tables totaling 1M rows | Total load + validate < 2 sec | R-13.7.NF2 |

### TC-13.7.13.I1 Hot Reload Bindings

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hot-reload a table with bound entities | All `DatabaseRow` entities updated within 1 frame | R-13.7.13 |

### TC-13.7.8.I1 Loot Server Authority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Server rolls loot with seed; client verifies | Identical result with same seed | R-13.7.8 |

### TC-13.7.4.I1 Formula Evaluation Throughput

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Evaluate 10k formula rows | > 1M evaluations/sec | R-13.7.4 |

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
