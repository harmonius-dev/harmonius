# Widget Framework Test Cases

Companion test cases for [widget-framework.md](widget-framework.md).

## Unit Tests

### TC-10.1.1.1 Tree Diff Insert

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** — Add widget to declared tree
   - **Expected:** Insert patch emitted with correct widget data

### TC-10.1.1.2 Tree Diff Remove

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** — Remove widget from declared tree
   - **Expected:** Remove patch emitted for the widget

### TC-10.1.1.3 Tree Diff Update

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** — Change one property on existing widget
   - **Expected:** UpdateProperties patch with only changed fields

### TC-10.1.12.1 Tree Diff Reorder Keyed

| # | Requirement |
|---|-------------|
| 1 | R-10.1.12   |

1. **#1** — Reorder 5 keyed list items
   - **Expected:** Reorder patches emitted, no Remove/Insert

### TC-10.1.12.2 Tree Diff Unchanged Skip

| # | Requirement |
|---|-------------|
| 1 | R-10.1.12   |

1. **#1** — 500-widget tree with 0 changes
   - **Expected:** Zero patches emitted

### TC-10.1.3.1 Pool Acquire Release

| # | Requirement |
|---|-------------|
| 1 | R-10.1.3    |

1. **#1** — Acquire widget, release it, acquire again
   - **Expected:** Same entity reused

### TC-10.1.3.2 Pool Budget Enforced

| # | Requirement |
|---|-------------|
| 1 | R-10.1.3    |

1. **#1** — Exhaust widget pool budget
   - **Expected:** `acquire()` returns None

### TC-10.1.3.3 Pool Zero Alloc Scroll

| # | Requirement |
|---|-------------|
| 1 | R-10.1.3    |

1. **#1** — Scroll 1000-item list for 10 seconds
   - **Expected:** Zero heap allocations after initial fill

### TC-10.1.4.1 Flex Row Gap

| # | Requirement |
|---|-------------|
| 1 | R-10.1.4    |

1. **#1** — Flex row, 3 children width=50, gap=10
   - **Expected:** Positions: 0, 60, 120

### TC-10.1.4.2 Flex Wrap

| # | Requirement |
|---|-------------|
| 1 | R-10.1.4    |

1. **#1** — Flex row width=200, 4 children width=80
   - **Expected:** Children 3-4 wrap to next line

### TC-10.1.4.3 Flex Justify Space Between

| # | Requirement |
|---|-------------|
| 1 | R-10.1.4    |

1. **#1** — Flex row width=300, 3 items width=50, justify=SpaceBetween
   - **Expected:** Spacing = (300 - 150) / 2 = 75 between items

### TC-10.1.4.4 Grid 2x3

| # | Requirement |
|---|-------------|
| 1 | R-10.1.4    |

1. **#1** — 2-column 3-row grid, cell size 100x50
   - **Expected:** All 6 cells at correct positions and sizes

### TC-10.1.4.5 Grid MinMax Track

| # | Requirement |
|---|-------------|
| 1 | R-10.1.4    |

1. **#1** — Grid column with MinMax(50, 200), container=300
   - **Expected:** Column width clamped between 50 and 200

### TC-10.1.5.1 Anchor Bottom Right

| # | Requirement |
|---|-------------|
| 1 | R-10.1.5    |

1. **#1** — Anchor to bottom-right, offset=10, at 3 resolutions
   - **Expected:** Constant 10px offset from bottom-right at all resolutions

### TC-10.1.5.2 Anchor Percentage

| # | Requirement |
|---|-------------|
| 1 | R-10.1.5    |

1. **#1** — Anchor offset=50%, parent width=200
   - **Expected:** Widget at position 100

### TC-10.1.5.3 Constraint Equal Widths

| # | Requirement |
|---|-------------|
| 1 | R-10.1.5    |

1. **#1** — Two widgets with equal-width constraint
   - **Expected:** Both have identical computed width

### TC-10.1.5.4 Constraint Circular Detect

| # | Requirement |
|---|-------------|
| 1 | R-10.1.5    |

1. **#1** — A depends on B, B depends on A
   - **Expected:** Returns `Err(WidgetError::CircularConstraint)`

### TC-10.1.6.1 Style Cascade Specificity

| # | Requirement |
|---|-------------|
| 1 | R-10.1.6    |

1. **#1** — Widget with type, class, ID, state, inline rules
   - **Expected:** Cascade resolves in correct specificity order

### TC-10.1.6.2 Style Theme Swap

| # | Requirement |
|---|-------------|
| 1 | R-10.1.6    |

1. **#1** — Swap active theme from "dark" to "light"
   - **Expected:** All widgets reflect new theme within one frame

### TC-10.1.6.3 Style State Selector

| # | Requirement |
|---|-------------|
| 1 | R-10.1.6    |

1. **#1** — Hover over button
   - **Expected:** Hovered style applied (e.g., background color changes)

### TC-10.1.6.4 Style Cache Hit

| # | Requirement |
|---|-------------|
| 1 | R-10.1.6    |

1. **#1** — Two buttons with identical classes
   - **Expected:** Style cache returns same computed result

### TC-10.1.7.1 Binding One Way

| # | Requirement |
|---|-------------|
| 1 | R-10.1.7    |

1. **#1** — Change source Health component from 100 to 50
   - **Expected:** Bound widget property updates to 50

### TC-10.1.7.2 Binding Two Way

| # | Requirement |
|---|-------------|
| 1 | R-10.1.7    |

1. **#1** — Drag slider to value=0.7
   - **Expected:** Source component updated to 0.7

### TC-10.1.7.3 Binding Computed

| # | Requirement |
|---|-------------|
| 1 | R-10.1.7    |

1. **#1** — Source A=10, Source B=5, computed=A+B
   - **Expected:** Widget shows 15

### TC-10.1.7.4 Binding Same Frame

| # | Requirement |
|---|-------------|
| 1 | R-10.1.7    |

1. **#1** — Change source in game system
   - **Expected:** Widget updated in same frame

### TC-10.1.8.1 Focus Tab Order

| # | Requirement |
|---|-------------|
| 1 | R-10.1.8    |

1. **#1** — 5 widgets with tab_index [1,2,3,4,5], press Tab 5 times
   - **Expected:** Focus visits in tab_index order

### TC-10.1.8.2 Focus Directional

| # | Requirement |
|---|-------------|
| 1 | R-10.1.8    |

1. **#1** — 3x3 grid of widgets, D-pad right from (0,0)
   - **Expected:** Focus moves to (1,0)

### TC-10.1.8.3 Focus Trap Modal

| # | Requirement |
|---|-------------|
| 1 | R-10.1.8    |

1. **#1** — Open modal with 3 buttons, Tab repeatedly
   - **Expected:** Focus cycles within modal, never escapes

### TC-10.1.8.4 Focus Stack Push Pop

| # | Requirement |
|---|-------------|
| 1 | R-10.1.8    |

1. **#1** — Push modal focus group, pop it
   - **Expected:** Focus restores to pre-modal widget

### TC-10.1.9.1 Locale Switch Text

| # | Requirement |
|---|-------------|
| 1 | R-10.1.9    |

1. **#1** — Switch locale from "en" to "ja"
   - **Expected:** Text updates from string table for "ja"

### TC-10.1.9.2 Locale RTL Mirror

| # | Requirement |
|---|-------------|
| 1 | R-10.1.9    |

1. **#1** — Switch to RTL locale (e.g., "ar")
   - **Expected:** Layout direction mirrors, flex rows reversed

### TC-10.1.13.1 Animation Linear

| # | Requirement |
|---|-------------|
| 1 | R-10.1.13   |

1. **#1** — Linear animation 0.0 to 1.0 over 1s, sample at t=0.5
   - **Expected:** Value = 0.5

### TC-10.1.13.2 Animation Ease In Out

| # | Requirement |
|---|-------------|
| 1 | R-10.1.13   |

1. **#1** — EaseInOut curve 0.0 to 1.0, sample at t=0.5
   - **Expected:** Value = 0.5 (midpoint of symmetric curve)

### TC-10.1.13.3 Animation Interrupt

| # | Requirement |
|---|-------------|
| 1 | R-10.1.13   |

1. **#1** — Interrupt animation at t=0.3 with new target
   - **Expected:** Smooth blend from current value (0.3) to new target

### TC-10.1.13.4 Animation Loop

| # | Requirement |
|---|-------------|
| 1 | R-10.1.13   |

1. **#1** — Loop mode animation, advance past duration
   - **Expected:** Animation repeats from start

### TC-10.1.13.5 Animation Staggered

| # | Requirement |
|---|-------------|
| 1 | R-10.1.13   |

1. **#1** — 5-item stagger, delay_per_item=0.1s
   - **Expected:** Item 3 starts at t=0.3s

### TC-10.1.14.1 Audio Click

| # | Requirement |
|---|-------------|
| 1 | R-10.1.14   |

1. **#1** — Click button with click AudioSlot set
   - **Expected:** Audio dispatched to UI mixer bus

### TC-10.1.14.2 Audio Disabled

| # | Requirement |
|---|-------------|
| 1 | R-10.1.14   |

1. **#1** — Disable UI sounds globally, click button
   - **Expected:** No audio dispatched

### TC-10.1.14.3 Audio Per Slot Disable

| # | Requirement |
|---|-------------|
| 1 | R-10.1.14   |
| 2 | R-10.1.14   |

1. **#1** — Disable hover sounds only, hover widget
   - **Expected:** No audio
2. **#2** — Click same widget
   - **Expected:** Audio dispatched

### TC-10.1.1.4 Event Hit Test

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** — Nested widgets, pointer at child position
   - **Expected:** Hit test returns child entity

### TC-10.1.1.5 Event Bubble

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** — Click child widget
   - **Expected:** Parent receives bubbled event

### TC-10.1.1.6 Event Capture Stop

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** — Parent stops propagation in capture phase
   - **Expected:** Child does not receive event

## Integration Tests

### TC-10.1.2.I1 Asset Round Trip

| # | Requirement |
|---|-------------|
| 1 | R-10.1.2    |

1. **#1** — Save UIAsset to binary, load it back
   - **Expected:** Tree structure matches original

### TC-10.1.2.I2 Template Slot Injection

| # | Requirement |
|---|-------------|
| 1 | R-10.1.2    |

1. **#1** — Instantiate template with slot content
   - **Expected:** Children appear in correct slot positions

### TC-10.1.4.I1 Full HUD Layout

| # | Requirement |
|---|-------------|
| 1 | R-10.1.4    |
| 2 | R-10.1.4    |

1. **#1** — Load full HUD at 1080p
   - **Expected:** All elements positioned correctly
2. **#2** — Load full HUD at 4K
   - **Expected:** All elements scale correctly

### TC-10.1.10.I1 World Space Panel Input

| # | Requirement |
|---|-------------|
| 1 | R-10.1.10   |

1. **#1** — Render world-space panel, ray-cast click button
   - **Expected:** Button handler fires

### TC-10.1.11.I1 VR Laser Input

| # | Requirement |
|---|-------------|
| 1 | R-10.1.11   |

1. **#1** — Simulate VR laser pointer on button
   - **Expected:** Button activates

### TC-10.1.11.I2 VR Gaze Dwell

| # | Requirement |
|---|-------------|
| 1 | R-10.1.11   |

1. **#1** — Gaze on button for dwell_seconds
   - **Expected:** Button activates after dwell

### TC-10.1.1.I1 Full Pipeline 500 Widgets

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** — 500-widget tree, change 10% of bindings
   - **Expected:** Full pipeline completes under 2 ms

### TC-10.1.6.I1 Multi Theme Concurrent

| # | Requirement |
|---|-------------|
| 1 | R-10.1.6    |

1. **#1** — Two canvases with different themes
   - **Expected:** Independent style resolution, correct colors

## Benchmarks

### TC-10.1.12.B1 Tree Diff

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 widgets, 10% changed | Diff time | < 1 ms | R-10.1.12 |

### TC-10.1.4.B1 Layout Pass

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 widgets (flex + grid) | Layout time | < 0.5 ms | R-10.1.4 |

### TC-10.1.6.B1 Style Resolution

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 widgets | Resolution time | < 0.3 ms | R-10.1.6 |

### TC-10.1.1.B1 Paint Full HUD

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full HUD paint pass | GPU time | < 2 ms | R-10.1.1 |
| 2 | Full HUD paint pass | Draw calls | < 50 | R-10.1.1 |

### TC-10.1.7.B1 Binding Propagation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 bindings with changed sources | Propagation time | < 0.1 ms | R-10.1.7 |

### TC-10.1.3.B1 Pool Acquire Release Cycle

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Acquire + release cycle | Cycle time | < 100 ns | R-10.1.3 |

### TC-10.1.1.B2 Hit Test

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 widgets, 10 deep hierarchy | Hit test time | < 50 us | R-10.1.1 |

### TC-10.1.13.B1 Animation Tick

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50 active animations | Tick time | < 0.1 ms | R-10.1.13 |

### TC-10.1.8.B1 Focus Directional Search

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Directional focus search in 500-widget tree | Search time | < 50 us | R-10.1.8 |

### TC-10.1.2.B1 Asset Load

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Load complex UI asset from binary | Load time | < 5 ms | R-10.1.2 |
