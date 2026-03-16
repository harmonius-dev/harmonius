# HUD, Common Widgets & UI Rendering Test Cases

Companion test cases for [hud-widgets.md](hud-widgets.md).

## Unit Tests

### TC-10.2.1.1 Rich Text Inline Formatting

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spans: bold "Hello", italic "World", color red "!" | Correct glyph runs with matching style per span | R-10.2.1 |

### TC-10.2.1.2 BiDi Arabic English Mix

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | String "Hello مرحبا World" | Visual ordering matches Unicode BiDi reference | R-10.2.1 |

### TC-10.2.2.1 Text Input Clipboard Ops

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Type "abc", select all, copy, paste twice | Buffer = "abcabc" | R-10.2.2 |
| 2 | Undo | Buffer = "abc" | R-10.2.2 |

### TC-10.2.2.2 IME Composition CJK

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate CJK IME composition for "中文" | Committed text = "中文" | R-10.2.2 |

### TC-10.2.2.3 Text Input 1000 Keys

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue 1,000 key events in one frame | All 1,000 characters in buffer, zero dropped | R-10.2.2 |

### TC-10.2.3.1 Slider No Jitter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drag slider at 120 Hz input rate | Value delta < threshold each frame, no oscillation | R-10.2.3 |

### TC-10.2.4.1 Dropdown Filter 500

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dropdown with 500 options, type "gold" | Visible list shows only substring matches | R-10.2.4 |

### TC-10.2.4.2 Dropdown Dynamic Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mutate option list while dropdown is open | Display updates, no crash or stale entries | R-10.2.4 |

### TC-10.2.5.1 Virtual List 10K

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 variable-height items in scroll view | Instantiated count <= visible + buffer count | R-10.2.5 |

### TC-10.2.6.1 Overlay Z Stacking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Open modal, then context menu, then tooltip | Z-order: tooltip > menu > modal | R-10.2.6 |

### TC-10.2.6.2 Overlay Dismiss

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Outside click with tooltip + context menu open | Topmost overlay dismissed, lower retained | R-10.2.6 |

### TC-10.2.7.1 Drag Drop Cross Panel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drag item from inventory to mail attachment | Item transfers, ghost preview follows pointer | R-10.2.7 |

### TC-10.2.7.2 Drag Split Stack

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drag with modifier held on stack of 20 | Split dialog shows with count=20 | R-10.2.7 |

### TC-10.2.8.1 Progress Bar Fill

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set progress to 0%, 50%, 100% | Fill width = 0%, 50%, 100% of bar width | R-10.2.8 |

### TC-10.3.1.1 Health Bar Overlays

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | HP=80, predicted_damage=30, absorb=10 | Overlays: fill=80%, damage=30%, absorb=10% of max | R-10.3.1 |

### TC-10.3.2.1 Buff Grid 35 Effects

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply 35 buff/debuff effects | All 35 displayed, grouped by category, correct durations | R-10.3.2 |

### TC-10.3.3.1 Cooldown Frame Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start 5s cooldown at tick T | Sweep reaches zero within 1 frame of T+5s | R-10.3.3 |

### TC-10.3.4.1 Nameplate Overlap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 nameplates at same screen position | No two overlap > 10% area after avoidance | R-10.3.4 |

### TC-10.3.5.1 Combat Text Merge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 60 damage events at same position in one frame | Merged into cumulative totals | R-10.3.5 |

### TC-10.3.6.1 Minimap Markers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20 moving entities on minimap | Icon positions match projected world positions within 1 px | R-10.3.6 |

### TC-10.3.10.1 Compass Bearing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 objectives at known bearings, player rotates 90 deg | Marker positions shift correctly on compass strip | R-10.3.10 |

### TC-10.3.11.1 Offscreen Indicator

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Objective behind player | Edge arrow appears at correct screen edge | R-10.3.11 |
| 2 | Turn toward objective | Smooth transition from edge arrow to on-screen | R-10.3.11 |

### TC-10.3.8.1 Chat 200 Msg Per Sec

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue 250 msg/s for 10 seconds | All appear in correct channels, zero drops, frame < 16.67 ms | R-10.3.8 |

### TC-10.3.9.1 Inventory Sort Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sort by rarity, filter by "sword" | Correct ordering, only "sword" items shown | R-10.3.9 |

## Integration Tests

### TC-10.4.1.I1 Quad Batching 500

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 widgets, 4 atlas pages, 2 blend states | Draw calls = distinct (page, blend) combos | R-10.4.1 |

### TC-10.4.2.I1 MSDF Text Scales

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5,000 glyphs at 100%, 200%, 300% scale | Sharp edges via image comparison | R-10.4.2 |

### TC-10.4.3.I1 Vector Radial Sweep

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cooldown sweep at 100% and 300% | No aliasing, within 1/255 tolerance vs CPU reference | R-10.4.3 |

### TC-10.4.4.I1 Atlas Incremental

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stream 20 textures during gameplay | No frame exceeds 2 ms repack cost | R-10.4.4 |

### TC-10.4.5.I1 3D Preview Dual

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two side-by-side 3D previews | Correct geometry/lighting/animation, GPU < 1 ms combined | R-10.4.5 |

### TC-10.3.1.I1 Raid Frame 40 Bars

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 40 health + 40 mana + cast bars animating | Frame < 16.67 ms for 300 frames | R-10.3.1 |

### TC-10.3.4.I1 Nameplate 200

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 250 entities with nameplates | Frame < 16.67 ms, overlap < 10%, occluded not drawn | R-10.3.4 |

### TC-10.3.5.I1 Combat Text Burst 60

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 60 damage events in one frame | All displayed/merged, no illegible overlap, frame < 16.67 ms | R-10.3.5 |

### TC-10.3.15.I1 Map Marker Consistency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Track quest | Markers on minimap, compass, and world map simultaneously | R-10.3.15 |
| 2 | Untrack quest | All markers removed | R-10.3.15 |

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
