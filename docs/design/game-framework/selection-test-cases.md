# Selection System Test Cases

Companion test cases for [selection.md](selection.md).

## Unit Tests

### TC-13.11.1.1 Pick Nearest 3D

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 entities, raycast from screen center | Correct nearest entity returned in < 1 ms | R-13.11.1 |

### TC-13.11.1.2 Pick Priority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Interactive object (priority 10) overlapping scenery (priority 1) | Interactive object returned | R-13.11.1 |

### TC-13.11.1.3 Pick All Sorted

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 entities along ray at distances [10, 3, 7, 1, 5] | Results sorted as [1, 3, 5, 7, 10] | R-13.11.1 |

### TC-13.11.1.4 Pick Bone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Raycast hits skeletal mesh arm bone | `PickResult.hit_bone == Some(arm_bone_id)` | R-13.11.1 |

### TC-13.11.1.5 Pick No Selectable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity without `Selectable` component at ray hit | `pick_nearest() == None` | R-13.11.1 |

### TC-13.11.2.1 Pick 2D Z-Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 overlapping UI widgets, z-orders [1, 3, 2] | Topmost (z=3) returned | R-13.11.2 |

### TC-13.11.2.2 Pick 2D Circular

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Click inside circular hit area | Hit detected | R-13.11.2 |
| 2 | Click outside circular hit area | No hit | R-13.11.2 |

### TC-13.11.2.3 Pick 2D Alpha

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Click on transparent pixel of sprite | No hit returned | R-13.11.2 |

### TC-13.11.2.4 Pick Touch Slop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tap 6px away from button edge, slop=8pt | Button hit detected | R-13.11.2 |

### TC-13.11.3.1 Selection Single

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Click A, then click B (Single mode) | Only B selected; A deselected | R-13.11.3 |

### TC-13.11.3.2 Selection Additive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select A, Shift+click B | Both A and B selected | R-13.11.3 |

### TC-13.11.3.3 Selection Subtractive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select A and B, Ctrl+click A | A removed, B remains | R-13.11.3 |

### TC-13.11.3.4 Selection Toggle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Toggle mode: click A (not selected) | A added | R-13.11.3 |
| 2 | Toggle mode: click A (already selected) | A removed | R-13.11.3 |

### TC-13.11.3.5 Selection Exclusive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select A, then select B (Exclusive mode) | Only B selected; `count() == 1` | R-13.11.3 |

### TC-13.11.3.6 Selection Component

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add entity to selection | `Selected` component added with correct order | R-13.11.3 |
| 2 | Remove entity from selection | `Selected` component removed | R-13.11.3 |

### TC-13.11.3.7 Selection Event

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add A, remove B from selection | `SelectionChanged { added: [A], removed: [B] }` | R-13.11.3 |

### TC-13.11.3.8 Selection Multiplayer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player selects 3 units, spectator observes | Spectator sees same 3 units selected | R-13.11.3 |

### TC-13.11.4a.1 RTS Box Select

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Marquee over 15 entities | All 15 selected | R-13.11.4a |

### TC-13.11.4a.2 RTS Control Group

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ctrl+1 assign 5 units, press 1 | 5 units recalled to selection | R-13.11.4a |

### TC-13.11.4a.3 RTS Double Click

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Double-click infantry unit | All infantry in view selected | R-13.11.4a |

### TC-13.11.4b.1 RPG Tab Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 enemies at distances [10, 3, 7, 1, 5], press Tab | Cycles through by ascending distance | R-13.11.4b |

### TC-13.11.4b.2 RPG Target Of Target

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target enemy E, E targets player P | Target-of-target display shows P | R-13.11.4b |

### TC-13.11.4c.1 Action Auto Target

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 enemies, nearest at 5m | Auto-target selects nearest | R-13.11.4c |

### TC-13.11.4c.2 Action Lock On

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Toggle lock-on on enemy E | Camera focuses on E | R-13.11.4c |

### TC-13.11.4c.3 Action Stick Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Right-stick flick right, enemy to right | Target switches to right enemy | R-13.11.4c |

### TC-13.11.4d.1 Builder Hierarchy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select parent entity with 3 children | All 4 entities selected | R-13.11.4d |

### TC-13.11.4d.2 Builder Group Ungroup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Group 3 entities, select group | All 3 members selected | R-13.11.4d |

### TC-13.11.5.1 Group Assign Recall

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Assign 5 entities to group 1, recall | Returns exactly those 5 entities | R-13.11.5 |

### TC-13.11.5.2 Group Union

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Group A={1,2,3}, Group B={2,3,4} | Union = {1,2,3,4} | R-13.11.5 |

### TC-13.11.5.3 Group Intersection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Group A={1,2,3}, Group B={2,3,4} | Intersection = {2,3} | R-13.11.5 |

### TC-13.11.5.4 Group Difference

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Group A={1,2,3}, Group B={2,3,4} | A-B = {1} | R-13.11.5 |

### TC-13.11.5.5 Group Persist

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Save with group 1 assigned, reload | Group 1 entities preserved | R-13.11.5 |

### TC-13.11.5.6 Group Local MP

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player A assigns group, check Player B | Player B has no access to A's groups | R-13.11.5 |

### TC-13.11.6a.1 Command Move

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move command, selection = 3 mobile + 1 building | 3 mobile units receive move; building skipped | R-13.11.6a |

### TC-13.11.6a.2 Command Mixed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attack command, selection = units + buildings | Only units with attack capability execute | R-13.11.6a |

### TC-13.11.6b.1 Formation Line

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Line formation, 5 units, spacing=2.0 | Units positioned at [-4,0], [-2,0], [0,0], [2,0], [4,0] relative to center | R-13.11.6b |

### TC-13.11.6b.2 Formation Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch from Line to Wedge | Slot positions recalculated as wedge | R-13.11.6b |

### TC-13.11.6c.1 Split Even

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 units, split even into 2 targets | 5 units per target | R-13.11.6c |

### TC-13.11.6c.2 Split By Tag

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 6 melee + 4 ranged, split by tag | Group A = 6 melee, Group B = 4 ranged | R-13.11.6c |

### TC-13.11.6d.1 Undo Within Timeout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Issue move, undo within timeout | Entities return to pre-command positions | R-13.11.6d |

### TC-13.11.6d.2 Undo Expired

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Issue move, wait past timeout, undo | `undo() == false`; positions unchanged | R-13.11.6d |

### TC-13.11.7.1 Marquee Intersect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Box over 200 entity screen bounds | All 200 selected | R-13.11.7 |

### TC-13.11.7.2 Marquee Additive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 selected, Shift+drag over 3 new | 8 total selected | R-13.11.7 |

### TC-13.11.7.3 Marquee Subtractive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 selected, Ctrl+drag over 3 | 7 remaining | R-13.11.7 |

### TC-13.11.7.4 Marquee Touch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Long-press+drag on touch device | Box selection activates | R-13.11.7 |

### TC-13.11.8.1 Outline Selected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select entity E | E rendered with `selected_color` outline at `selected_width` | R-13.11.8 |

### TC-13.11.8.2 Outline Hover

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hover over entity E | E rendered with `hover_color` outline at `hover_width` | R-13.11.8 |

### TC-13.11.8.3 Ground Circle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | RTS preset, select unit | Team-colored ground circle rendered | R-13.11.8 |

### TC-13.11.8.4 Sprite Outline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2D game, select sprite entity | Pixel-perfect outline rendered | R-13.11.8 |

### TC-13.11.8.5 Hero Indicator

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select hero entity with `SelectionIndicatorOverride` | Distinct indicator style used | R-13.11.8 |

### TC-13.11.8.6 Preset Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch from RTS to RPG preset | Visual style updates to RPG config | R-13.11.8 |

## Integration Tests

### TC-13.11.NF1.I1 500 Selection Perf

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change selection to 500 entities | Dispatch completes in < 1 ms | R-13.11.NF1 |

### TC-13.11.NF1.I2 500 Command Dispatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move command to 500-entity group | Dispatch completes in < 1 ms | R-13.11.NF1 |

### TC-13.11.NF2.I1 Marquee 200 60fps

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drag marquee over 200 entities | No frame exceeds 16.67 ms | R-13.11.NF2 |

### TC-13.11.1.I1 Pick 10K Entities

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Raycast pick in 10,000 entity scene | Pick completes in < 1 ms | R-13.11.1 |

### TC-13.11.0.I1 Full Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Click -> pick -> select -> outline -> command | End-to-end pipeline completes within 1 frame | R-13.11.1 |

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
