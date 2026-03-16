# Widget Framework Test Cases

Companion test cases for [widget-framework.md](widget-framework.md).

## Unit Tests

### TC-10.1.1.1 Tree Diff Insert

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add widget to declared tree | Insert patch emitted with correct widget data | R-10.1.1 |

### TC-10.1.1.2 Tree Diff Remove

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove widget from declared tree | Remove patch emitted for the widget | R-10.1.1 |

### TC-10.1.1.3 Tree Diff Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change one property on existing widget | UpdateProperties patch with only changed fields | R-10.1.1 |

### TC-10.1.12.1 Tree Diff Reorder Keyed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reorder 5 keyed list items | Reorder patches emitted, no Remove/Insert | R-10.1.12 |

### TC-10.1.12.2 Tree Diff Unchanged Skip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500-widget tree with 0 changes | Zero patches emitted | R-10.1.12 |

### TC-10.1.3.1 Pool Acquire Release

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Acquire widget, release it, acquire again | Same entity reused | R-10.1.3 |

### TC-10.1.3.2 Pool Budget Enforced

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Exhaust widget pool budget | `acquire()` returns None | R-10.1.3 |

### TC-10.1.3.3 Pool Zero Alloc Scroll

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scroll 1000-item list for 10 seconds | Zero heap allocations after initial fill | R-10.1.3 |

### TC-10.1.4.1 Flex Row Gap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Flex row, 3 children width=50, gap=10 | Positions: 0, 60, 120 | R-10.1.4 |

### TC-10.1.4.2 Flex Wrap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Flex row width=200, 4 children width=80 | Children 3-4 wrap to next line | R-10.1.4 |

### TC-10.1.4.3 Flex Justify Space Between

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Flex row width=300, 3 items width=50, justify=SpaceBetween | Spacing = (300 - 150) / 2 = 75 between items | R-10.1.4 |

### TC-10.1.4.4 Grid 2x3

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2-column 3-row grid, cell size 100x50 | All 6 cells at correct positions and sizes | R-10.1.4 |

### TC-10.1.4.5 Grid MinMax Track

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Grid column with MinMax(50, 200), container=300 | Column width clamped between 50 and 200 | R-10.1.4 |

### TC-10.1.5.1 Anchor Bottom Right

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Anchor to bottom-right, offset=10, at 3 resolutions | Constant 10px offset from bottom-right at all resolutions | R-10.1.5 |

### TC-10.1.5.2 Anchor Percentage

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Anchor offset=50%, parent width=200 | Widget at position 100 | R-10.1.5 |

### TC-10.1.5.3 Constraint Equal Widths

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two widgets with equal-width constraint | Both have identical computed width | R-10.1.5 |

### TC-10.1.5.4 Constraint Circular Detect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | A depends on B, B depends on A | Returns `Err(WidgetError::CircularConstraint)` | R-10.1.5 |

### TC-10.1.6.1 Style Cascade Specificity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Widget with type, class, ID, state, inline rules | Cascade resolves in correct specificity order | R-10.1.6 |

### TC-10.1.6.2 Style Theme Swap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Swap active theme from "dark" to "light" | All widgets reflect new theme within one frame | R-10.1.6 |

### TC-10.1.6.3 Style State Selector

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hover over button | Hovered style applied (e.g., background color changes) | R-10.1.6 |

### TC-10.1.6.4 Style Cache Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two buttons with identical classes | Style cache returns same computed result | R-10.1.6 |

### TC-10.1.7.1 Binding One Way

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change source Health component from 100 to 50 | Bound widget property updates to 50 | R-10.1.7 |

### TC-10.1.7.2 Binding Two Way

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drag slider to value=0.7 | Source component updated to 0.7 | R-10.1.7 |

### TC-10.1.7.3 Binding Computed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Source A=10, Source B=5, computed=A+B | Widget shows 15 | R-10.1.7 |

### TC-10.1.7.4 Binding Same Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change source in game system | Widget updated in same frame | R-10.1.7 |

### TC-10.1.8.1 Focus Tab Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 widgets with tab_index [1,2,3,4,5], press Tab 5 times | Focus visits in tab_index order | R-10.1.8 |

### TC-10.1.8.2 Focus Directional

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3x3 grid of widgets, D-pad right from (0,0) | Focus moves to (1,0) | R-10.1.8 |

### TC-10.1.8.3 Focus Trap Modal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Open modal with 3 buttons, Tab repeatedly | Focus cycles within modal, never escapes | R-10.1.8 |

### TC-10.1.8.4 Focus Stack Push Pop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push modal focus group, pop it | Focus restores to pre-modal widget | R-10.1.8 |

### TC-10.1.9.1 Locale Switch Text

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch locale from "en" to "ja" | Text updates from string table for "ja" | R-10.1.9 |

### TC-10.1.9.2 Locale RTL Mirror

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch to RTL locale (e.g., "ar") | Layout direction mirrors, flex rows reversed | R-10.1.9 |

### TC-10.1.13.1 Animation Linear

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Linear animation 0.0 to 1.0 over 1s, sample at t=0.5 | Value = 0.5 | R-10.1.13 |

### TC-10.1.13.2 Animation Ease In Out

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | EaseInOut curve 0.0 to 1.0, sample at t=0.5 | Value = 0.5 (midpoint of symmetric curve) | R-10.1.13 |

### TC-10.1.13.3 Animation Interrupt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Interrupt animation at t=0.3 with new target | Smooth blend from current value (0.3) to new target | R-10.1.13 |

### TC-10.1.13.4 Animation Loop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Loop mode animation, advance past duration | Animation repeats from start | R-10.1.13 |

### TC-10.1.13.5 Animation Staggered

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5-item stagger, delay_per_item=0.1s | Item 3 starts at t=0.3s | R-10.1.13 |

### TC-10.1.14.1 Audio Click

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Click button with click AudioSlot set | Audio dispatched to UI mixer bus | R-10.1.14 |

### TC-10.1.14.2 Audio Disabled

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Disable UI sounds globally, click button | No audio dispatched | R-10.1.14 |

### TC-10.1.14.3 Audio Per Slot Disable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Disable hover sounds only, hover widget | No audio | R-10.1.14 |
| 2 | Click same widget | Audio dispatched | R-10.1.14 |

### TC-10.1.1.4 Event Hit Test

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Nested widgets, pointer at child position | Hit test returns child entity | R-10.1.1 |

### TC-10.1.1.5 Event Bubble

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Click child widget | Parent receives bubbled event | R-10.1.1 |

### TC-10.1.1.6 Event Capture Stop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent stops propagation in capture phase | Child does not receive event | R-10.1.1 |

## Integration Tests

### TC-10.1.2.I1 Asset Round Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Save UIAsset to binary, load it back | Tree structure matches original | R-10.1.2 |

### TC-10.1.2.I2 Template Slot Injection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Instantiate template with slot content | Children appear in correct slot positions | R-10.1.2 |

### TC-10.1.4.I1 Full HUD Layout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load full HUD at 1080p | All elements positioned correctly | R-10.1.4 |
| 2 | Load full HUD at 4K | All elements scale correctly | R-10.1.4 |

### TC-10.1.10.I1 World Space Panel Input

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render world-space panel, ray-cast click button | Button handler fires | R-10.1.10 |

### TC-10.1.11.I1 VR Laser Input

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate VR laser pointer on button | Button activates | R-10.1.11 |

### TC-10.1.11.I2 VR Gaze Dwell

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gaze on button for dwell_seconds | Button activates after dwell | R-10.1.11 |

### TC-10.1.1.I1 Full Pipeline 500 Widgets

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500-widget tree, change 10% of bindings | Full pipeline completes under 2 ms | R-10.1.1 |

### TC-10.1.6.I1 Multi Theme Concurrent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two canvases with different themes | Independent style resolution, correct colors | R-10.1.6 |

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
