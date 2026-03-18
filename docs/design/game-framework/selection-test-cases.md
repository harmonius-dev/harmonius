# Selection System Test Cases

Companion test cases for [selection.md](selection.md).

## Unit Tests

### TC-13.11.1.1 Pick Nearest 3D

| # | Requirement |
|---|-------------|
| 1 | R-13.11.1   |

1. **#1** — 10,000 entities, raycast from screen center
   - **Expected:** Correct nearest entity returned in < 1 ms

### TC-13.11.1.2 Pick Priority

| # | Requirement |
|---|-------------|
| 1 | R-13.11.1   |

1. **#1** — Interactive object (priority 10) overlapping scenery (priority 1)
   - **Expected:** Interactive object returned

### TC-13.11.1.3 Pick All Sorted

| # | Requirement |
|---|-------------|
| 1 | R-13.11.1   |

1. **#1** — 5 entities along ray at distances [10, 3, 7, 1, 5]
   - **Expected:** Results sorted as [1, 3, 5, 7, 10]

### TC-13.11.1.4 Pick Bone

| # | Requirement |
|---|-------------|
| 1 | R-13.11.1   |

1. **#1** — Raycast hits skeletal mesh arm bone
   - **Expected:** `PickResult.hit_bone == Some(arm_bone_id)`

### TC-13.11.1.5 Pick No Selectable

| # | Requirement |
|---|-------------|
| 1 | R-13.11.1   |

1. **#1** — Entity without `Selectable` component at ray hit
   - **Expected:** `pick_nearest() == None`

### TC-13.11.2.1 Pick 2D Z-Order

| # | Requirement |
|---|-------------|
| 1 | R-13.11.2   |

1. **#1** — 3 overlapping UI widgets, z-orders [1, 3, 2]
   - **Expected:** Topmost (z=3) returned

### TC-13.11.2.2 Pick 2D Circular

| # | Requirement |
|---|-------------|
| 1 | R-13.11.2   |
| 2 | R-13.11.2   |

1. **#1** — Click inside circular hit area
   - **Expected:** Hit detected
2. **#2** — Click outside circular hit area
   - **Expected:** No hit

### TC-13.11.2.3 Pick 2D Alpha

| # | Requirement |
|---|-------------|
| 1 | R-13.11.2   |

1. **#1** — Click on transparent pixel of sprite
   - **Expected:** No hit returned

### TC-13.11.2.4 Pick Touch Slop

| # | Requirement |
|---|-------------|
| 1 | R-13.11.2   |

1. **#1** — Tap 6px away from button edge, slop=8pt
   - **Expected:** Button hit detected

### TC-13.11.3.1 Selection Single

| # | Requirement |
|---|-------------|
| 1 | R-13.11.3   |

1. **#1** — Click A, then click B (Single mode)
   - **Expected:** Only B selected; A deselected

### TC-13.11.3.2 Selection Additive

| # | Requirement |
|---|-------------|
| 1 | R-13.11.3   |

1. **#1** — Select A, Shift+click B
   - **Expected:** Both A and B selected

### TC-13.11.3.3 Selection Subtractive

| # | Requirement |
|---|-------------|
| 1 | R-13.11.3   |

1. **#1** — Select A and B, Ctrl+click A
   - **Expected:** A removed, B remains

### TC-13.11.3.4 Selection Toggle

| # | Requirement |
|---|-------------|
| 1 | R-13.11.3   |
| 2 | R-13.11.3   |

1. **#1** — Toggle mode: click A (not selected)
   - **Expected:** A added
2. **#2** — Toggle mode: click A (already selected)
   - **Expected:** A removed

### TC-13.11.3.5 Selection Exclusive

| # | Requirement |
|---|-------------|
| 1 | R-13.11.3   |

1. **#1** — Select A, then select B (Exclusive mode)
   - **Expected:** Only B selected; `count() == 1`

### TC-13.11.3.6 Selection Component

| # | Requirement |
|---|-------------|
| 1 | R-13.11.3   |
| 2 | R-13.11.3   |

1. **#1** — Add entity to selection
   - **Expected:** `Selected` component added with correct order
2. **#2** — Remove entity from selection
   - **Expected:** `Selected` component removed

### TC-13.11.3.7 Selection Event

| # | Requirement |
|---|-------------|
| 1 | R-13.11.3   |

1. **#1** — Add A, remove B from selection
   - **Expected:** `SelectionChanged { added: [A], removed: [B] }`

### TC-13.11.3.8 Selection Multiplayer

| # | Requirement |
|---|-------------|
| 1 | R-13.11.3   |

1. **#1** — Player selects 3 units, spectator observes
   - **Expected:** Spectator sees same 3 units selected

### TC-13.11.4a.1 RTS Box Select

| # | Requirement |
|---|-------------|
| 1 | R-13.11.4a  |

1. **#1** — Marquee over 15 entities
   - **Expected:** All 15 selected

### TC-13.11.4a.2 RTS Control Group

| # | Requirement |
|---|-------------|
| 1 | R-13.11.4a  |

1. **#1** — Ctrl+1 assign 5 units, press 1
   - **Expected:** 5 units recalled to selection

### TC-13.11.4a.3 RTS Double Click

| # | Requirement |
|---|-------------|
| 1 | R-13.11.4a  |

1. **#1** — Double-click infantry unit
   - **Expected:** All infantry in view selected

### TC-13.11.4b.1 RPG Tab Cycle

| # | Requirement |
|---|-------------|
| 1 | R-13.11.4b  |

1. **#1** — 5 enemies at distances [10, 3, 7, 1, 5], press Tab
   - **Expected:** Cycles through by ascending distance

### TC-13.11.4b.2 RPG Target Of Target

| # | Requirement |
|---|-------------|
| 1 | R-13.11.4b  |

1. **#1** — Target enemy E, E targets player P
   - **Expected:** Target-of-target display shows P

### TC-13.11.4c.1 Action Auto Target

| # | Requirement |
|---|-------------|
| 1 | R-13.11.4c  |

1. **#1** — 3 enemies, nearest at 5m
   - **Expected:** Auto-target selects nearest

### TC-13.11.4c.2 Action Lock On

| # | Requirement |
|---|-------------|
| 1 | R-13.11.4c  |

1. **#1** — Toggle lock-on on enemy E
   - **Expected:** Camera focuses on E

### TC-13.11.4c.3 Action Stick Switch

| # | Requirement |
|---|-------------|
| 1 | R-13.11.4c  |

1. **#1** — Right-stick flick right, enemy to right
   - **Expected:** Target switches to right enemy

### TC-13.11.4d.1 Builder Hierarchy

| # | Requirement |
|---|-------------|
| 1 | R-13.11.4d  |

1. **#1** — Select parent entity with 3 children
   - **Expected:** All 4 entities selected

### TC-13.11.4d.2 Builder Group Ungroup

| # | Requirement |
|---|-------------|
| 1 | R-13.11.4d  |

1. **#1** — Group 3 entities, select group
   - **Expected:** All 3 members selected

### TC-13.11.5.1 Group Assign Recall

| # | Requirement |
|---|-------------|
| 1 | R-13.11.5   |

1. **#1** — Assign 5 entities to group 1, recall
   - **Expected:** Returns exactly those 5 entities

### TC-13.11.5.2 Group Union

| # | Requirement |
|---|-------------|
| 1 | R-13.11.5   |

1. **#1** — Group A={1,2,3}, Group B={2,3,4}
   - **Expected:** Union = {1,2,3,4}

### TC-13.11.5.3 Group Intersection

| # | Requirement |
|---|-------------|
| 1 | R-13.11.5   |

1. **#1** — Group A={1,2,3}, Group B={2,3,4}
   - **Expected:** Intersection = {2,3}

### TC-13.11.5.4 Group Difference

| # | Requirement |
|---|-------------|
| 1 | R-13.11.5   |

1. **#1** — Group A={1,2,3}, Group B={2,3,4}
   - **Expected:** A-B = {1}

### TC-13.11.5.5 Group Persist

| # | Requirement |
|---|-------------|
| 1 | R-13.11.5   |

1. **#1** — Save with group 1 assigned, reload
   - **Expected:** Group 1 entities preserved

### TC-13.11.5.6 Group Local MP

| # | Requirement |
|---|-------------|
| 1 | R-13.11.5   |

1. **#1** — Player A assigns group, check Player B
   - **Expected:** Player B has no access to A's groups

### TC-13.11.6a.1 Command Move

| # | Requirement |
|---|-------------|
| 1 | R-13.11.6a  |

1. **#1** — Move command, selection = 3 mobile + 1 building
   - **Expected:** 3 mobile units receive move; building skipped

### TC-13.11.6a.2 Command Mixed

| # | Requirement |
|---|-------------|
| 1 | R-13.11.6a  |

1. **#1** — Attack command, selection = units + buildings
   - **Expected:** Only units with attack capability execute

### TC-13.11.6b.1 Formation Line

| # | Requirement |
|---|-------------|
| 1 | R-13.11.6b  |

1. **#1** — Line formation, 5 units, spacing=2.0
   - **Expected:** Units positioned at [-4,0], [-2,0], [0,0], [2,0], [4,0] relative to center

### TC-13.11.6b.2 Formation Switch

| # | Requirement |
|---|-------------|
| 1 | R-13.11.6b  |

1. **#1** — Switch from Line to Wedge
   - **Expected:** Slot positions recalculated as wedge

### TC-13.11.6c.1 Split Even

| # | Requirement |
|---|-------------|
| 1 | R-13.11.6c  |

1. **#1** — 10 units, split even into 2 targets
   - **Expected:** 5 units per target

### TC-13.11.6c.2 Split By Tag

| # | Requirement |
|---|-------------|
| 1 | R-13.11.6c  |

1. **#1** — 6 melee + 4 ranged, split by tag
   - **Expected:** Group A = 6 melee, Group B = 4 ranged

### TC-13.11.6d.1 Undo Within Timeout

| # | Requirement |
|---|-------------|
| 1 | R-13.11.6d  |

1. **#1** — Issue move, undo within timeout
   - **Expected:** Entities return to pre-command positions

### TC-13.11.6d.2 Undo Expired

| # | Requirement |
|---|-------------|
| 1 | R-13.11.6d  |

1. **#1** — Issue move, wait past timeout, undo
   - **Expected:** `undo() == false`; positions unchanged

### TC-13.11.7.1 Marquee Intersect

| # | Requirement |
|---|-------------|
| 1 | R-13.11.7   |

1. **#1** — Box over 200 entity screen bounds
   - **Expected:** All 200 selected

### TC-13.11.7.2 Marquee Additive

| # | Requirement |
|---|-------------|
| 1 | R-13.11.7   |

1. **#1** — 5 selected, Shift+drag over 3 new
   - **Expected:** 8 total selected

### TC-13.11.7.3 Marquee Subtractive

| # | Requirement |
|---|-------------|
| 1 | R-13.11.7   |

1. **#1** — 10 selected, Ctrl+drag over 3
   - **Expected:** 7 remaining

### TC-13.11.7.4 Marquee Touch

| # | Requirement |
|---|-------------|
| 1 | R-13.11.7   |

1. **#1** — Long-press+drag on touch device
   - **Expected:** Box selection activates

### TC-13.11.8.1 Outline Selected

| # | Requirement |
|---|-------------|
| 1 | R-13.11.8   |

1. **#1** — Select entity E
   - **Expected:** E rendered with `selected_color` outline at `selected_width`

### TC-13.11.8.2 Outline Hover

| # | Requirement |
|---|-------------|
| 1 | R-13.11.8   |

1. **#1** — Hover over entity E
   - **Expected:** E rendered with `hover_color` outline at `hover_width`

### TC-13.11.8.3 Ground Circle

| # | Requirement |
|---|-------------|
| 1 | R-13.11.8   |

1. **#1** — RTS preset, select unit
   - **Expected:** Team-colored ground circle rendered

### TC-13.11.8.4 Sprite Outline

| # | Requirement |
|---|-------------|
| 1 | R-13.11.8   |

1. **#1** — 2D game, select sprite entity
   - **Expected:** Pixel-perfect outline rendered

### TC-13.11.8.5 Hero Indicator

| # | Requirement |
|---|-------------|
| 1 | R-13.11.8   |

1. **#1** — Select hero entity with `SelectionIndicatorOverride`
   - **Expected:** Distinct indicator style used

### TC-13.11.8.6 Preset Switch

| # | Requirement |
|---|-------------|
| 1 | R-13.11.8   |

1. **#1** — Switch from RTS to RPG preset
   - **Expected:** Visual style updates to RPG config

## Integration Tests

### TC-13.11.NF1.I1 500 Selection Perf

| # | Requirement |
|---|-------------|
| 1 | R-13.11.NF1 |

1. **#1** — Change selection to 500 entities
   - **Expected:** Dispatch completes in < 1 ms

### TC-13.11.NF1.I2 500 Command Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-13.11.NF1 |

1. **#1** — Move command to 500-entity group
   - **Expected:** Dispatch completes in < 1 ms

### TC-13.11.NF2.I1 Marquee 200 60fps

| # | Requirement |
|---|-------------|
| 1 | R-13.11.NF2 |

1. **#1** — Drag marquee over 200 entities
   - **Expected:** No frame exceeds 16.67 ms

### TC-13.11.1.I1 Pick 10K Entities

| # | Requirement |
|---|-------------|
| 1 | R-13.11.1   |

1. **#1** — Raycast pick in 10,000 entity scene
   - **Expected:** Pick completes in < 1 ms

### TC-13.11.0.I1 Full Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-13.11.1   |

1. **#1** — Click -> pick -> select -> outline -> command
   - **Expected:** End-to-end pipeline completes within 1 frame

## Benchmarks

### TC-13.11.1.B1 3D Raycast Pick

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 entities | Pick time | < 1 ms | R-13.11.1 |

### TC-13.11.NF1.B1 Selection Change Dispatch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 entities added to selection | Dispatch time | < 1 ms | R-13.11.NF1 |

### TC-13.11.NF1.B2 Command Dispatch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 entities receive move | Dispatch time | < 1 ms | R-13.11.NF1 |

### TC-13.11.NF2.B1 Marquee Preview Update

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 entities in marquee preview | Frame rate | 60 fps | R-13.11.NF2 |

### TC-13.11.8.B1 Selection Visual Update

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 entities with selection visuals | Update time | < 0.5 ms | R-13.11.8 |

### TC-13.11.5.B1 Group Set Operations

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Union/intersection/difference on 500-entity groups | Operation time | < 0.1 ms | R-13.11.5 |
