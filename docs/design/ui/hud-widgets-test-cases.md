# HUD, Common Widgets & UI Rendering Test Cases

Companion test cases for [hud-widgets.md](hud-widgets.md).

## Unit Tests

### TC-10.2.1.1 Rich Text Inline Formatting

| # | Requirement |
|---|-------------|
| 1 | R-10.2.1    |

1. **#1** — Spans: bold "Hello", italic "World", color red "!"
   - **Expected:** Correct glyph runs with matching style per span

### TC-10.2.1.2 BiDi Arabic English Mix

| # | Requirement |
|---|-------------|
| 1 | R-10.2.1    |

1. **#1** — String "Hello مرحبا World"
   - **Expected:** Visual ordering matches Unicode BiDi reference

### TC-10.2.2.1 Text Input Clipboard Ops

| # | Requirement |
|---|-------------|
| 1 | R-10.2.2    |
| 2 | R-10.2.2    |

1. **#1** — Type "abc", select all, copy, paste twice
   - **Expected:** Buffer = "abcabc"
2. **#2** — Undo
   - **Expected:** Buffer = "abc"

### TC-10.2.2.2 IME Composition CJK

| # | Requirement |
|---|-------------|
| 1 | R-10.2.2    |

1. **#1** — Simulate CJK IME composition for "中文"
   - **Expected:** Committed text = "中文"

### TC-10.2.2.3 Text Input 1000 Keys

| # | Requirement |
|---|-------------|
| 1 | R-10.2.2    |

1. **#1** — Enqueue 1,000 key events in one frame
   - **Expected:** All 1,000 characters in buffer, zero dropped

### TC-10.2.3.1 Slider No Jitter

| # | Requirement |
|---|-------------|
| 1 | R-10.2.3    |

1. **#1** — Drag slider at 120 Hz input rate
   - **Expected:** Value delta < threshold each frame, no oscillation

### TC-10.2.4.1 Dropdown Filter 500

| # | Requirement |
|---|-------------|
| 1 | R-10.2.4    |

1. **#1** — Dropdown with 500 options, type "gold"
   - **Expected:** Visible list shows only substring matches

### TC-10.2.4.2 Dropdown Dynamic Update

| # | Requirement |
|---|-------------|
| 1 | R-10.2.4    |

1. **#1** — Mutate option list while dropdown is open
   - **Expected:** Display updates, no crash or stale entries

### TC-10.2.5.1 Virtual List 10K

| # | Requirement |
|---|-------------|
| 1 | R-10.2.5    |

1. **#1** — 10,000 variable-height items in scroll view
   - **Expected:** Instantiated count <= visible + buffer count

### TC-10.2.6.1 Overlay Z Stacking

| # | Requirement |
|---|-------------|
| 1 | R-10.2.6    |

1. **#1** — Open modal, then context menu, then tooltip
   - **Expected:** Z-order: tooltip > menu > modal

### TC-10.2.6.2 Overlay Dismiss

| # | Requirement |
|---|-------------|
| 1 | R-10.2.6    |

1. **#1** — Outside click with tooltip + context menu open
   - **Expected:** Topmost overlay dismissed, lower retained

### TC-10.2.7.1 Drag Drop Cross Panel

| # | Requirement |
|---|-------------|
| 1 | R-10.2.7    |

1. **#1** — Drag item from inventory to mail attachment
   - **Expected:** Item transfers, ghost preview follows pointer

### TC-10.2.7.2 Drag Split Stack

| # | Requirement |
|---|-------------|
| 1 | R-10.2.7    |

1. **#1** — Drag with modifier held on stack of 20
   - **Expected:** Split dialog shows with count=20

### TC-10.2.8.1 Progress Bar Fill

| # | Requirement |
|---|-------------|
| 1 | R-10.2.8    |

1. **#1** — Set progress to 0%, 50%, 100%
   - **Expected:** Fill width = 0%, 50%, 100% of bar width

### TC-10.3.1.1 Health Bar Overlays

| # | Requirement |
|---|-------------|
| 1 | R-10.3.1    |

1. **#1** — HP=80, predicted_damage=30, absorb=10
   - **Expected:** Overlays: fill=80%, damage=30%, absorb=10% of max

### TC-10.3.2.1 Buff Grid 35 Effects

| # | Requirement |
|---|-------------|
| 1 | R-10.3.2    |

1. **#1** — Apply 35 buff/debuff effects
   - **Expected:** All 35 displayed, grouped by category, correct durations

### TC-10.3.3.1 Cooldown Frame Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-10.3.3    |

1. **#1** — Start 5s cooldown at tick T
   - **Expected:** Sweep reaches zero within 1 frame of T+5s

### TC-10.3.4.1 Nameplate Overlap

| # | Requirement |
|---|-------------|
| 1 | R-10.3.4    |

1. **#1** — 10 nameplates at same screen position
   - **Expected:** No two overlap > 10% area after avoidance

### TC-10.3.5.1 Combat Text Merge

| # | Requirement |
|---|-------------|
| 1 | R-10.3.5    |

1. **#1** — 60 damage events at same position in one frame
   - **Expected:** Merged into cumulative totals

### TC-10.3.6.1 Minimap Markers

| # | Requirement |
|---|-------------|
| 1 | R-10.3.6    |

1. **#1** — 20 moving entities on minimap
   - **Expected:** Icon positions match projected world positions within 1 px

### TC-10.3.10.1 Compass Bearing

| # | Requirement |
|---|-------------|
| 1 | R-10.3.10   |

1. **#1** — 5 objectives at known bearings, player rotates 90 deg
   - **Expected:** Marker positions shift correctly on compass strip

### TC-10.3.11.1 Offscreen Indicator

| # | Requirement |
|---|-------------|
| 1 | R-10.3.11   |
| 2 | R-10.3.11   |

1. **#1** — Objective behind player
   - **Expected:** Edge arrow appears at correct screen edge
2. **#2** — Turn toward objective
   - **Expected:** Smooth transition from edge arrow to on-screen

### TC-10.3.8.1 Chat 200 Msg Per Sec

| # | Requirement |
|---|-------------|
| 1 | R-10.3.8    |

1. **#1** — Enqueue 250 msg/s for 10 seconds
   - **Expected:** All appear in correct channels, zero drops, frame < 16.67 ms

### TC-10.3.9.1 Inventory Sort Filter

| # | Requirement |
|---|-------------|
| 1 | R-10.3.9    |

1. **#1** — Sort by rarity, filter by "sword"
   - **Expected:** Correct ordering, only "sword" items shown

## Integration Tests

### TC-10.4.1.I1 Quad Batching 500

| # | Requirement |
|---|-------------|
| 1 | R-10.4.1    |

1. **#1** — 500 widgets, 4 atlas pages, 2 blend states
   - **Expected:** Draw calls = distinct (page, blend) combos

### TC-10.4.2.I1 MSDF Text Scales

| # | Requirement |
|---|-------------|
| 1 | R-10.4.2    |

1. **#1** — 5,000 glyphs at 100%, 200%, 300% scale
   - **Expected:** Sharp edges via image comparison

### TC-10.4.3.I1 Vector Radial Sweep

| # | Requirement |
|---|-------------|
| 1 | R-10.4.3    |

1. **#1** — Cooldown sweep at 100% and 300%
   - **Expected:** No aliasing, within 1/255 tolerance vs CPU reference

### TC-10.4.4.I1 Atlas Incremental

| # | Requirement |
|---|-------------|
| 1 | R-10.4.4    |

1. **#1** — Stream 20 textures during gameplay
   - **Expected:** No frame exceeds 2 ms repack cost

### TC-10.4.5.I1 3D Preview Dual

| # | Requirement |
|---|-------------|
| 1 | R-10.4.5    |

1. **#1** — Two side-by-side 3D previews
   - **Expected:** Correct geometry/lighting/animation, GPU < 1 ms combined

### TC-10.3.1.I1 Raid Frame 40 Bars

| # | Requirement |
|---|-------------|
| 1 | R-10.3.1    |

1. **#1** — 40 health + 40 mana + cast bars animating
   - **Expected:** Frame < 16.67 ms for 300 frames

### TC-10.3.4.I1 Nameplate 200

| # | Requirement |
|---|-------------|
| 1 | R-10.3.4    |

1. **#1** — 250 entities with nameplates
   - **Expected:** Frame < 16.67 ms, overlap < 10%, occluded not drawn

### TC-10.3.5.I1 Combat Text Burst 60

| # | Requirement |
|---|-------------|
| 1 | R-10.3.5    |

1. **#1** — 60 damage events in one frame
   - **Expected:** All displayed/merged, no illegible overlap, frame < 16.67 ms

### TC-10.3.15.I1 Map Marker Consistency

| # | Requirement |
|---|-------------|
| 1 | R-10.3.15   |
| 2 | R-10.3.15   |

1. **#1** — Track quest
   - **Expected:** Markers on minimap, compass, and world map simultaneously
2. **#2** — Untrack quest
   - **Expected:** All markers removed

## Benchmarks

### TC-10.4.1.B1 Quad Batch 500 Widgets

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 widgets across multiple atlas pages | Draw call count | < 50 | R-10.4.1 |

### TC-10.4.2.B1 Full HUD GPU Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Complete HUD (health, mana, minimap, action bars) | GPU time | < 2 ms | R-10.4.2 |

### TC-10.4.2.B2 MSDF 5000 Glyphs

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Render 5,000 glyphs | Text pass time | < 4 ms | R-10.4.2 |

### TC-10.2.5.B1 Virtual List 10K Scroll

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Scroll through 10K items at max speed | Per-frame time | < 4 ms | R-10.2.5 |

### TC-10.4.4.B1 Atlas Incremental Repack

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Incremental repack with 20 new textures | Per-frame repack cost | < 2 ms | R-10.4.4 |

### TC-10.3.4.B1 Nameplate Cull 250 Entities

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 250 nameplate entities, frustum cull + sort | Cull time | < 0.5 ms | R-10.3.4 |

### TC-10.3.5.B1 Combat Text 60 Events

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Process 60 combat text events in one frame | Processing time | < 0.3 ms | R-10.3.5 |

### TC-10.3.8.B1 Chat 250 Msg Per Sec Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 250 messages per second sustained | Frame time | < 16.67 ms | R-10.3.8 |

### TC-10.1.4.B1 Layout Solve 500 Nodes

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Flexbox + grid layout for 500 widget nodes | Layout time | < 1 ms | R-10.1.4 |
