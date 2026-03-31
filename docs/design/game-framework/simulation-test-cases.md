# Simulation Primitives -- Test Cases

Companion test cases for [simulation.md](simulation.md).

## Unit Tests -- Schedule

### TC-13.19.4a.1 Active Entry TimeOfDay

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |
| 2 | R-13.19.4a  |

1. **#1** -- Schedule with 3 TimeOfDay entries (morning, afternoon, evening)
   - Game time = noon
   - **Expected:** Returns afternoon entry
2. **#2** -- Game time = midnight, no matching entry
   - **Expected:** `None`

### TC-13.19.4a.2 Active Entry DayOfWeek

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- Schedule with DayOfWeek entries (Mon, Wed, Fri)
   - Game time = Wednesday
   - **Expected:** Returns Wednesday entry

### TC-13.19.4a.3 Active Entry CalendarDate

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- Schedule with CalendarDate entry at day 100
   - Game time = day 100
   - **Expected:** Returns the matching entry

### TC-13.19.4a.4 Active Entry Interval

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- Schedule with Interval entry every 300 seconds
   - Game time = 600s since start
   - **Expected:** Returns the interval entry

### TC-13.19.4a.5 Active Entry Priority Override

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- Two entries overlap at noon; priority 10 and priority 20
   - Game time = noon
   - **Expected:** Returns entry with priority 20

### TC-13.19.4a.6 Next Entry Lookup

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- 3 entries at 08:00, 12:00, 18:00; current time = 10:00
   - **Expected:** `Some((time_offset, entry_at_12:00))`

### TC-13.19.4a.7 Validate No Gaps

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |
| 2 | R-13.19.4a  |

1. **#1** -- Entries fully cover 24h with no gaps
   - **Expected:** `Ok(())`
2. **#2** -- Gap between 14:00 and 16:00
   - **Expected:** `Err(ScheduleValidationError::Gap { .. })`

### TC-13.19.4a.8 Validate Priority Conflict

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- Two overlapping entries with same priority
   - **Expected:** `Err(ScheduleValidationError::AmbiguousPriority { .. })`

### TC-13.19.4a.9 RepeatMode Daily Wrap

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- `RepeatMode::Daily`, game time past 24h boundary
   - **Expected:** Schedule wraps; `active_entry` returns correct entry for the new day

### TC-13.19.4a.10 RepeatMode Weekly Wrap

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- `RepeatMode::Weekly`, game time past 7-day boundary
   - **Expected:** Schedule wraps; correct DayOfWeek entry returned

### TC-13.19.4a.11 RepeatMode OneShot No Wrap

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- `RepeatMode::OneShot`, all entries have elapsed
   - **Expected:** `active_entry` returns `None`

### TC-13.19.4a.12 RepeatMode Custom Period

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- `RepeatMode::Custom`, `custom_period_secs = 600`
   - Game time = 1200s
   - **Expected:** Schedule has wrapped twice; correct entry active

### TC-13.19.4a.13 Activity Binding SetComponent

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- Binding with `kind = SetComponent`, 2 patches
   - **Expected:** Both component patches applied to entity

### TC-13.19.4a.14 Activity Binding FireEvent

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- Binding with `kind = FireEvent`, event payload set
   - **Expected:** Typed event emitted on the event channel

### TC-13.19.4a.15 Activity Binding ChangeState

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** -- Binding with `kind = ChangeState`, target state set
   - **Expected:** Game mode transition requested

### TC-13.19.4b.1 Schedule State Advance Entry

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |

1. **#1** -- ScheduleState at entry 0, clock crosses entry 1 boundary
   - **Expected:** `current_entry_index == 1`, `time_in_entry == 0.0`

### TC-13.19.4b.2 Schedule State Time In Entry

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |

1. **#1** -- ScheduleState within entry, clock advances 5.0s
   - **Expected:** `time_in_entry` increased by 5.0

### TC-13.19.4b.3 Schedule State Paused

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |

1. **#1** -- `paused = true`, clock advances 10.0s
   - **Expected:** `current_entry_index` and `time_in_entry` unchanged

### TC-13.19.4b.4 Entry Changed Event

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |

1. **#1** -- Clock crosses entry boundary
   - **Expected:** Exactly 1 `ScheduleEntryChanged` event with correct `from_index`, `to_index`, and
     `activity_id`

### TC-13.19.4b.5 Cycle Completed Event Daily

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |

1. **#1** -- `RepeatMode::Daily`, clock crosses 24h
   - **Expected:** Exactly 1 `ScheduleCycleCompleted` event with `cycle_count == 1`

### TC-13.19.4b.6 Cycle Completed OneShot

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |

1. **#1** -- `RepeatMode::OneShot`, last entry elapsed
   - **Expected:** `ScheduleCycleCompleted` with `cycle_count == 1`; no further entry changes

### TC-13.19.4c.1 Schedule Gated Interaction Available

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4c  |

1. **#1** -- NPC schedule active entry = "at_market"
   - **Expected:** Market interaction available to dialogue system

### TC-13.19.4c.2 Schedule Gated Interaction Unavailable

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4c  |

1. **#1** -- NPC schedule active entry = "sleeping"
   - **Expected:** Market interaction not available

## Unit Tests -- Bounded Event Log

### TC-13.19.3a.1 Push Single Entry

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Empty log (capacity 10), push 1 entry
   - **Expected:** `len() == 1`, `is_empty() == false`

### TC-13.19.3a.2 Push Fill To Capacity

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Log with capacity 4, push exactly 4 entries
   - **Expected:** `len() == 4`, all entries present in order

### TC-13.19.3a.3 Push Ring Wrap CapacityBased

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Capacity 4, `CapacityBased` decay, push 6 entries
   - **Expected:** `len() == 4`, oldest 2 entries evicted, entries 3--6 present

### TC-13.19.3a.4 Push Ring Wrap TimeBased

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Capacity 10, `TimeBased` decay (threshold 60s)
   - Push 5 entries at t=0, push 1 entry at t=120
   - **Expected:** 5 old entries evicted; `len() == 1`

### TC-13.19.3a.5 Push Ring Wrap ReliabilityBased

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Capacity 3, `ReliabilityBased` decay (min_reliability=0.5)
   - Push 3 entries: reliability 0.9, 0.3, 0.8
   - Push 4th entry
   - **Expected:** Entry with reliability 0.3 evicted first

### TC-13.19.3a.6 Push Ring Wrap Composite

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- `Composite` decay (TimeBased then ReliabilityBased)
   - Old low-reliability entry and old high-reliability entry
   - **Expected:** Old low-reliability entry evicted first

### TC-13.19.3a.7 Iter Chronological Order

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Push entries at t=10, t=20, t=30
   - **Expected:** `iter()` yields t=10, t=20, t=30

### TC-13.19.3a.8 Iter Rev Reverse Order

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Push entries at t=10, t=20, t=30
   - **Expected:** `iter_rev()` yields t=30, t=20, t=10

### TC-13.19.3a.9 Iter After Wrap

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Capacity 3, push 5 entries (t=10..t=50)
   - **Expected:** `iter()` yields t=30, t=40, t=50

### TC-13.19.3a.10 Query By Event Type

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- 5 entries: 3 type A, 2 type B
   - Query `event_type = Some(A)`
   - **Expected:** 3 results, all type A

### TC-13.19.3a.11 Query By Time Range

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Entries at t=10, 20, 30, 40, 50
   - Query `time_range = Some(TimeRange { start: 20, end: 40 })`
   - **Expected:** 3 results (t=20, t=30, t=40)

### TC-13.19.3a.12 Query By Source Entity

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- 4 entries: 2 from entity A, 2 from entity B
   - Query `source = Some(A)`
   - **Expected:** 2 results, both from entity A

### TC-13.19.3a.13 Query By Min Reliability

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Entries with reliability 0.2, 0.5, 0.8, 1.0
   - Query `min_reliability = Some(0.5)`
   - **Expected:** 3 results (0.5, 0.8, 1.0)

### TC-13.19.3a.14 Query Max Results

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- 10 entries, query `max_results = 3`
   - **Expected:** Exactly 3 results (oldest 3)

### TC-13.19.3a.15 Query Combined Filters

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Mixed entries; query combining event_type + time_range
   - min_reliability
   - **Expected:** Only entries matching all three filters returned

### TC-13.19.3a.16 Query Empty Log

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Empty log, any query
   - **Expected:** Empty result vec

### TC-13.19.3a.17 Retain By Predicate

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- 5 entries; retain only entries with reliability >= 0.5
   - **Expected:** Low-reliability entries removed; `len()` reflects remaining count

### TC-13.19.3a.18 Entry Size Budget

| # | Requirement  |
|---|--------------|
| 1 | NFR-13.19.2  |

1. **#1** -- Construct `LogEntry` with maximum-size payload
   - **Expected:** `size_of::<LogEntry>() <= 256` bytes

### TC-13.19.3a.19 Emotional Weight Preserved

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- Push entry with `emotional_weight = 0.85`
   - **Expected:** Retrieved entry has `emotional_weight == 0.85`

### TC-13.19.3a.20 Decay TimeBased System

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- 10 entries, threshold 60s, game time advances to 120s
   - 5 entries at t=0, 5 entries at t=100
   - **Expected:** 5 old entries decayed; `LogEntryDecayed` events emitted for each

### TC-13.19.3a.21 Decay ReliabilityBased System

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** -- 10 entries; 3 below `min_reliability` threshold
   - **Expected:** 3 entries removed; `LogEntryDecayed` emitted with `reason = ReliabilityBased`

### TC-13.19.3b.1 Propagation Single Hop

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** -- Entity A has entry (hop_count=0, reliability=1.0)
   - Entity B is within propagation radius
   - Config: `reliability_decay=0.7`, `weight_decay=0.8`
   - **Expected:** B receives entry with `reliability == 0.7`, `emotional_weight == original * 0.8`,
     `hop_count == 1`

### TC-13.19.3b.2 Propagation Multi Hop

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** -- Chain: A -> B -> C, `reliability_decay = 0.7`
   - A has entry (reliability=1.0, hop_count=0)
   - **Expected:** B gets reliability=0.7, hop_count=1; C gets reliability=0.49, hop_count=2

### TC-13.19.3b.3 Propagation Max Hops

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** -- `max_hops = 2`, entry at hop_count=2
   - **Expected:** Entry not propagated further

### TC-13.19.3b.4 Propagation Min Reliability Floor

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** -- `min_reliability = 0.3`, entry reliability = 0.25
   - **Expected:** Entry not propagated

### TC-13.19.3b.5 Propagation Deduplication

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** -- A propagates entry to B twice (same event_type + timestamp + source_entity)
   - **Expected:** B has exactly 1 copy of the entry

### TC-13.19.3b.6 Propagation Budget Cap

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** -- `budget_per_frame = 3`, source has 10 entries
   - **Expected:** At most 3 entries propagated in one frame

### TC-13.19.3b.7 Propagation Out Of Range

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** -- Entity B is outside propagation radius
   - **Expected:** No entries propagated to B

### TC-13.19.3b.8 LogEntryAdded Event On Propagation

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** -- Successful propagation from A to B
   - **Expected:** `LogEntryAdded` event emitted for B's log

## Unit Tests -- Grid

### TC-13.20.1.1 Grid New Defaults

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- Create 8x8 grid with schema `[traversable: Bool]`
   - **Expected:** All 64 cells have `traversable == default`

### TC-13.20.1.2 Grid Get Set 2D

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- Set cell (3, 5) property 0 to `CellValue::I32(42)`
   - **Expected:** `get((3,5), 0) == CellValue::I32(42)`

### TC-13.20.1.3 Grid Get Set 3D

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- 3D grid (8x8x8), set (2,3,4) property 0 to `CellValue::Bool(true)`
   - **Expected:** `get((2,3,4), 0) == CellValue::Bool(true)`

### TC-13.20.1.4 Grid Out Of Bounds 2D

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- 8x8 grid, `in_bounds((8, 0)) == false`
   - **Expected:** `false`

### TC-13.20.1.5 Grid Out Of Bounds 3D

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- 8x8x8 grid, `in_bounds((0, 0, 8)) == false`
   - **Expected:** `false`

### TC-13.20.1.6 Grid Cell Properties All

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- Schema with 3 properties; set each for cell (1,1)
   - **Expected:** `cell_properties((1,1))` returns all 3 values

### TC-13.20.1.7 Grid World To Grid

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |
| 2 | R-13.20.1   |

1. **#1** -- `cell_size = 2.0`, world pos (5.0, 3.0, 0.0)
   - **Expected:** `Some(GridCoord { x: 2, y: 1, z: None })`
2. **#2** -- World pos out of grid bounds
   - **Expected:** `None`

### TC-13.20.1.8 Grid To World

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- `cell_size = 2.0`, coord (2, 3)
   - **Expected:** World center = `(5.0, 7.0, 0.0)`

### TC-13.20.2.1 Query Circle Interior

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** -- 16x16 grid, circle at (8,8) radius 3
   - **Expected:** All cells within r=3 returned; count matches expected area

### TC-13.20.2.2 Query Circle Boundary Clamp

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** -- 16x16 grid, circle at (0,0) radius 5
   - **Expected:** Only in-bounds cells returned; no negative coordinates

### TC-13.20.2.3 Query Circle Radius Zero

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** -- Circle at (4,4) radius 0
   - **Expected:** Exactly 1 cell: (4,4)

### TC-13.20.2.4 Query Rectangle

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |

1. **#1** -- 16x16 grid, rect at (8,8) half_width=2 half_height=3
   - **Expected:** 5x7 = 35 cells returned

### TC-13.20.2.5 Query Rectangle Boundary Clamp

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |

1. **#1** -- 16x16 grid, rect at (1,1) half_width=3 half_height=3
   - **Expected:** Clamped to grid bounds; no out-of-range coords

### TC-13.20.2.6 Line Of Sight Clear

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** -- No blocking cells between (0,0) and (7,7)
   - **Expected:** `line_of_sight` returns `true`

### TC-13.20.2.7 Line Of Sight Blocked

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** -- Blocking cell at (4,4) on path from (0,0) to (7,7)
   - **Expected:** `line_of_sight` returns `false`

### TC-13.20.2.8 Line Of Sight Adjacent

| # | Requirement |
|---|-------------|
| 1 | R-13.20.2   |

1. **#1** -- From (3,3) to (3,4), no blockers
   - **Expected:** `line_of_sight` returns `true`

### TC-13.20.3.1 Vision Modifier Volume

| # | Requirement |
|---|-------------|
| 1 | R-13.20.3   |

1. **#1** -- Stealth zone overlay reduces visibility in region
   - **Expected:** Cells inside zone have modified visibility; cells outside unchanged

### TC-13.20.4.1 Fog Memory Last Seen

| # | Requirement |
|---|-------------|
| 1 | R-13.20.4   |

1. **#1** -- Cell was visible at frame 100, now shrouded
   - **Expected:** `last_seen_frame == 100` in fog memory

### TC-13.21.1.1 Find Path Reachable

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |

1. **#1** -- 16x16 grid, clear path from (0,0) to (15,15)
   - **Expected:** `Some(PathfindResult)` with valid path and non-zero `total_cost`

### TC-13.21.1.2 Find Path Unreachable

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |

1. **#1** -- Wall of blocking cells across the grid
   - **Expected:** `None`

### TC-13.21.1.3 Find Path Diagonal

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |

1. **#1** -- Open grid, (0,0) to (5,5)
   - **Expected:** Path contains diagonal steps; `total_cost` reflects diagonal movement

### TC-13.21.1.4 Find Path Cost Weighted

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |

1. **#1** -- Two paths: short through high-cost, long through low-cost
   - **Expected:** A* prefers the lower total cost path

### TC-13.21.1.5 Reachable Cells Budget Exact

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |

1. **#1** -- Origin (8,8), `max_cost = 3`, uniform cost 1
   - **Expected:** All cells within Manhattan distance 3 returned with correct cumulative costs

### TC-13.21.1.6 Reachable Cells Budget Exceeded

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |

1. **#1** -- Origin (8,8), `max_cost = 2`, cell at distance 3
   - **Expected:** Cell at distance 3 not included

### TC-13.21.4.1 Cover Directional Query

| # | Requirement |
|---|-------------|
| 1 | R-13.21.4   |

1. **#1** -- Cell with `cover_north = Full`, `cover_east = Half`
   - **Expected:** Correct cover values returned per direction

### TC-13.21.4.2 Flanking No Cover

| # | Requirement |
|---|-------------|
| 1 | R-13.21.4   |

1. **#1** -- Attack from south on cell with only north cover
   - **Expected:** No cover bonus applies

### TC-13.27.1.1 Block Type Registry Lookup

| # | Requirement |
|---|-------------|
| 1 | R-13.27.1   |

1. **#1** -- Register 100 block types, lookup by ID
   - **Expected:** O(1) lookup returns correct block type

### TC-13.27.1.2 Block Type Registry Unknown ID

| # | Requirement |
|---|-------------|
| 1 | R-13.27.1   |

1. **#1** -- Lookup unregistered block type ID
   - **Expected:** Returns `None` or default air block

### TC-13.27.2.1 Block Placement Via Set

| # | Requirement |
|---|-------------|
| 1 | R-13.27.2   |

1. **#1** -- `grid.set(coord, 0, CellValue::I32(stone_id))`
   - **Expected:** `get(coord, 0) == CellValue::I32(stone_id)`

### TC-13.27.2.2 Block Destruction Via Set

| # | Requirement |
|---|-------------|
| 1 | R-13.27.2   |

1. **#1** -- Set block to air: `grid.set(coord, 0, CellValue::I32(0))`
   - **Expected:** `get(coord, 0) == CellValue::I32(0)`

### TC-13.20.1.9 Overlay Get Set Isolation

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |
| 2 | R-13.20.1   |

1. **#1** -- Faction A overlay sets cell (5,5) to `Visible`
   - **Expected:** Faction A overlay `get((5,5)) == Visible`
2. **#2** -- Faction B overlay reads cell (5,5)
   - **Expected:** Faction B overlay `get((5,5)) == default` (unchanged)

### TC-13.20.1.10 Overlay Multiple Owners

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- 4 overlays on same grid, each sets different cells
   - **Expected:** Each overlay has independent values; no cross contamination

### TC-13.20.1.11 Grid Definition Validate

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |
| 2 | R-13.20.1   |

1. **#1** -- Schema with unique IDs and matching default types
   - **Expected:** `validate()` returns `Ok(())`
2. **#2** -- Schema with duplicate property IDs
   - **Expected:** `Err(GridValidationError::DuplicatePropertyId)`

### TC-13.20.1.12 Grid Definition Cell Count

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- 2D grid 10x20
   - **Expected:** `cell_count() == 200`

### TC-13.20.1.13 Grid Definition Cell Count 3D

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- 3D grid 10x20x5
   - **Expected:** `cell_count() == 1000`

### TC-13.20.1.14 Grid Definition Dimensions

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |
| 2 | R-13.20.1   |

1. **#1** -- Grid with `depth = None`
   - **Expected:** `dimensions() == GridDimensions::Two`
2. **#2** -- Grid with `depth = Some(8)`
   - **Expected:** `dimensions() == GridDimensions::Three`

### TC-13.20.1.15 Grid Definition Bytes Per Cell

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |

1. **#1** -- Schema: `[F32, I32, Bool]`
   - **Expected:** `bytes_per_cell() == 4 + 4 + 1 == 9`

## Integration Tests

### TC-13.19.4b.I1 Schedule Activity Binder End To End

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |
| 2 | R-13.19.4a  |

1. **#1** -- Create entity with ScheduleState; advance clock past entry boundary
   - **Expected:** ActivityBinder applies SetComponent patch; component updated on entity
2. **#2** -- Advance clock to FireEvent entry
   - **Expected:** Typed event emitted and received by listener

### TC-13.19.4b.I2 Schedule With Accelerated Clock

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |

1. **#1** -- Game clock at 10x speed; schedule with 24h Daily cycle
   - **Expected:** All entries fire in correct order at accelerated rate; no skipped entries

### TC-13.19.3b.I1 Multi Hop Gossip Chain

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** -- 5 NPCs in a line (A-B-C-D-E), each within radius of neighbors only
   - Push entry to A; run propagation for 5 frames
   - **Expected:** Entry reaches E with `reliability = 1.0 * 0.7^4`, `hop_count == 4`

### TC-13.19.3b.I2 Gossip Reliability Cutoff

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** -- Chain of 10 NPCs; `min_reliability = 0.3`, `reliability_decay = 0.7`
   - **Expected:** Propagation stops before reaching NPC 10 (reliability drops below 0.3 at hop 4)

### TC-13.20.1.I1 Fog Of War Update Cycle

| # | Requirement |
|---|-------------|
| 1 | R-13.20.1   |
| 2 | R-13.20.2   |

1. **#1** -- Grid with visibility overlay; vision source moves
   - **Expected:** Old cells transition to "shrouded"; new cells become "visible"; fog memory tracks
     last-seen frame
2. **#2** -- Vision source behind blocking wall
   - **Expected:** Cells behind wall remain hidden

### TC-13.21.1.I1 Tactical Grid With Cover

| # | Requirement |
|---|-------------|
| 1 | R-13.21.1   |
| 2 | R-13.21.4   |

1. **#1** -- Tactical grid with elevation, cover, and occupancy
   - Move unit via `find_path`, check movement cost
   - **Expected:** Path avoids blocked cells; cost reflects terrain
2. **#2** -- Query reachable cells with action point budget
   - **Expected:** Only cells within budget highlighted; cover values accessible per cell

### TC-13.27.2.I1 Block Place Destroy Cycle

| # | Requirement |
|---|-------------|
| 1 | R-13.27.2   |

1. **#1** -- Place 10 blocks, destroy 5, place 3 more
   - **Expected:** Grid state consistent; `cell_count` properties reflect all mutations correctly

### TC-SIM.I1 All Primitives Serialize Round Trip

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |
| 2 | R-13.19.3a  |
| 3 | R-13.20.1   |

1. **#1** -- Serialize `ScheduleState` to binary, deserialize
   - **Expected:** All fields identical after round trip
2. **#2** -- Serialize `EventLog` (half full) to binary, deserialize
   - **Expected:** All entries, head, count identical
3. **#3** -- Serialize `Grid` + `GridOverlay` to binary, deserialize
   - **Expected:** All cell values and overlay data identical

### TC-13.23.2.I1 Daily Challenge Reset

| # | Requirement |
|---|-------------|
| 1 | R-13.23.2   |

1. **#1** -- Challenge entity with Daily schedule; advance clock past midnight
   - **Expected:** `ChallengeResetEvent` fired; challenge set rotated

### TC-13.23.4.I1 Login Reward Calendar

| # | Requirement |
|---|-------------|
| 1 | R-13.23.4   |

1. **#1** -- Per-player entity with Daily schedule + EventLog
   - Advance clock 3 days, claim each day
   - **Expected:** 3 `LoginRewardAvailable` events; EventLog records 3 claims; streak count == 3

## Benchmarks

### TC-SIM.NF1.B1 Schedule Evaluation 500 Entities

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 entities with ScheduleState | Wall-clock time | < 1 ms | NFR-SIM.NF1 |

### TC-SIM.NF2.B1 Grid Circle Query 1024x1024

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1024x1024 grid, circle r=32 | Wall-clock time | < 1 ms | NFR-SIM.NF2 |

### TC-SIM.NF3.B1 Event Log Decay 1000 Logs

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 EventLog components, TimeBased decay | Wall-clock time | < 2 ms | NFR-SIM.NF3 |

### TC-SIM.B1 A Star Pathfinding 128x128

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 128x128 grid, worst-case path | Wall-clock time | < 0.5 ms | R-13.21.1 |

### TC-SIM.B2 Propagation 200 NPCs

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200 NPCs with LogPropagator | Wall-clock time | < 2 ms | NFR-13.19.1 |

### TC-SIM.B3 Overlay Merge 4 Factions

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1024x1024 grid, 4 faction overlays | Wall-clock time | < 0.5 ms | R-13.20.1 |

### TC-SIM.B4 Log Entry Size

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | LogEntry with max-size payload | Byte size | <= 256 B | NFR-13.19.2 |
