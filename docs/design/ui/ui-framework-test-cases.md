# UI Framework Test Cases

Companion test cases for [ui-framework.md](ui-framework.md).

## Unit Tests

### TC-10.1.1.1 Tree diff — insert

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** -- Insert a new widget node into an empty tree.
   - **Expected:** `WidgetTree` contains the node; `DirtyFlags::CHILDREN` set on parent.

### TC-10.1.1.2 Tree diff — remove

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** -- Remove an existing widget node.
   - **Expected:** Node absent from tree; parent `DirtyFlags::CHILDREN` set; entity released to
     pool.

### TC-10.1.1.3 Event hit test

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** -- Fire a pointer event at pixel coordinates that overlap one widget's `ComputedRect`.
   - **Expected:** `EventRouter` returns that widget's entity as the hit target.

### TC-10.1.1.4 Event bubble

| # | Requirement |
|---|-------------|
| 1 | R-10.1.1    |

1. **#1** -- Fire a pointer event on a leaf widget with a parent registered for the same event.
   - **Expected:** Event fires on leaf first, then bubbles to parent; no grandparent receives it if
     parent calls `stop_propagation`.

### TC-10.1.12.1 Tree diff — reorder keyed

| # | Requirement |
|---|-------------|
| 1 | R-10.1.12   |

1. **#1** -- Provide a list of 100 keyed widgets; reorder two adjacent items; diff.
   - **Expected:** Only two `PatchOp::Move` ops emitted; no inserts or removes; O(n) ops.

### TC-10.1.3.1 Pool acquire and release

| # | Requirement |
|---|-------------|
| 1 | R-10.1.3    |

1. **#1** -- Release 10 widget entities to `WidgetPool`; acquire 10.
   - **Expected:** Acquired entities are the previously released ones; no heap allocation.

### TC-10.1.3.2 Pool — zero alloc steady-state scroll

| # | Requirement |
|---|-------------|
| 1 | R-10.1.3    |

1. **#1** -- Scroll a virtualized list of 10 000 items through 200 frames; track heap allocations.
   - **Expected:** Zero heap allocations after the first frame; all from pool free list.

### TC-10.1.4.1 Flex row gap

| # | Requirement |
|---|-------------|
| 1 | R-10.1.4    |

1. **#1** -- Three children in a flex row with `gap: 8px`; container width 100px.
   - **Expected:** Children positions respect 8px gaps; total width within container bounds.

### TC-10.1.4.2 Grid 2x3

| # | Requirement |
|---|-------------|
| 1 | R-10.1.4    |

1. **#1** -- Six widgets in a 2-column, 3-row grid; all equal size.
   - **Expected:** Each cell occupies exactly `(container_width/2) x (container_height/3)`.

### TC-10.1.5.1 Anchor bottom-right

| # | Requirement |
|---|-------------|
| 1 | R-10.1.5    |

1. **#1** -- Widget anchored to bottom-right; container resized.
   - **Expected:** Widget stays in bottom-right corner at all container sizes.

### TC-10.1.5.2 Constraint equal widths

| # | Requirement |
|---|-------------|
| 1 | R-10.1.5    |

1. **#1** -- Two widgets constrained to equal widths inside a container.
   - **Expected:** Both report the same computed width after layout.

### TC-10.1.6.1 Style cascade specificity

| # | Requirement |
|---|-------------|
| 1 | R-10.1.6    |

1. **#1** -- Widget matches two rules: class `.foo` (specificity 0,1,0) and id `#bar` (0,1,0 + id
   boost). Conflicting `background_color`.
   - **Expected:** ID rule wins; widget renders with ID rule's color.

### TC-10.1.6.2 Style theme swap

| # | Requirement |
|---|-------------|
| 1 | R-10.1.6    |

1. **#1** -- Swap active theme via `ThemeRegistry`; check 500 widgets.
   - **Expected:** All widgets resolve new theme colors on the same frame; no frames with mixed
     colors.

### TC-10.1.7.1 Binding one-way

| # | Requirement |
|---|-------------|
| 1 | R-10.1.7    |

1. **#1** -- Bind a `Label` text to a data source entity's `health: f32` component; update health.
   - **Expected:** Label text reflects new value on the same frame as the ECS update.

### TC-10.1.7.2 Binding two-way

| # | Requirement |
|---|-------------|
| 1 | R-10.1.7    |

1. **#1** -- Bind a `Slider.value` two-way to a data entity's `volume: f32`; drag the slider.
   - **Expected:** Data entity's `volume` updated the same frame; slider reflects data entity
     changes in the other direction.

### TC-10.1.7.3 Binding same-frame propagation

| # | Requirement |
|---|-------------|
| 1 | R-10.1.7    |

1. **#1** -- Change 50 bound data entities in a single frame.
   - **Expected:** All 50 dependent widgets reflect the new values within that same frame.

### TC-10.1.8.1 Focus tab order

| # | Requirement |
|---|-------------|
| 1 | R-10.1.8    |

1. **#1** -- Tab through five widgets with explicit `tab_index` values 3, 1, 4, 1, 5.
   - **Expected:** Focus order follows tab index ascending, then DOM order for ties.

### TC-10.1.8.2 Focus trap modal

| # | Requirement |
|---|-------------|
| 1 | R-10.1.8    |

1. **#1** -- Open a modal dialog; press Tab repeatedly.
   - **Expected:** Focus stays within modal; does not reach widgets behind the modal overlay.

### TC-10.1.9.1 Locale RTL mirror

| # | Requirement |
|---|-------------|
| 1 | R-10.1.9    |

1. **#1** -- Switch locale to `ar` (Arabic, RTL); measure a flex-row container.
   - **Expected:** Flex direction reversed; start/end margins swapped; text flows right-to-left.

### TC-10.1.13.1 Animation interrupt

| # | Requirement |
|---|-------------|
| 1 | R-10.1.13   |

1. **#1** -- Start a 0.5s opacity tween; interrupt at 0.2s with a new tween to a different target.
   - **Expected:** New tween starts from current value at interrupt point; no visual pop.

### TC-10.1.14.1 Audio click

| # | Requirement |
|---|-------------|
| 1 | R-10.1.14   |

1. **#1** -- Click a button with `AudioSlots.click` set; verify audio event fired.
   - **Expected:** `AudioFeedback` system emits a play event for the assigned `Handle<AudioAsset>`
     on the UI bus within the same frame.

### TC-10.2.1.1 Rich text inline formatting

| # | Requirement |
|---|-------------|
| 1 | R-10.2.1    |

1. **#1** -- Render a rich text string with bold, italic, and a hyperlink inline.
   - **Expected:** Bold segment uses bold font weight; italic uses slanted glyphs; hyperlink is
     underlined and fires an event on click.

### TC-10.2.2.1 Text input clipboard ops

| # | Requirement |
|---|-------------|
| 1 | R-10.2.2    |

1. **#1** -- Type text into `TextInput`; select all; copy; paste into a second `TextInput`.
   - **Expected:** Second input contains copied text verbatim; undo restores original state.

### TC-10.2.3.1 Slider no jitter

| # | Requirement |
|---|-------------|
| 1 | R-10.2.3    |

1. **#1** -- Drag a slider to a position; release; check value over 10 frames.
   - **Expected:** Value is stable; no frame-to-frame oscillation.

### TC-10.2.4.1 Dropdown filter 500 items

| # | Requirement |
|---|-------------|
| 1 | R-10.2.4    |

1. **#1** -- Open a dropdown with 500 items; type a filter string.
   - **Expected:** Filtered list visible within the same frame; no visible lag.

### TC-10.2.5.1 Virtual list 10k items

| # | Requirement |
|---|-------------|
| 1 | R-10.2.5    |

1. **#1** -- Scroll a virtualized list with 10 000 variable-height items at 60 fps for 300 frames.
   - **Expected:** Frame time stays under 4 ms; zero heap allocations after first frame.

### TC-10.2.6.1 Overlay z-stacking

| # | Requirement |
|---|-------------|
| 1 | R-10.2.6    |

1. **#1** -- Open two context menus and one modal dialog; verify z-order.
   - **Expected:** Modal is always above context menus; later-opened context menu is above earlier.

### TC-10.2.7.1 Drag and drop cross-panel

| # | Requirement |
|---|-------------|
| 1 | R-10.2.7    |

1. **#1** -- Drag an item from panel A; drop on a valid target in panel B.
   - **Expected:** `DragState` transitions through Idle → Pending → Dragging → OverValid → Dropped;
     payload delivered to drop target; ghost preview visible during drag.

### TC-10.2.8.1 Progress bar fill

| # | Requirement |
|---|-------------|
| 1 | R-10.2.8    |

1. **#1** -- Set a linear `ProgressBar` to 75% fill; verify render.
   - **Expected:** Filled region is exactly 75% of bar width; no overflow.

### TC-10.3.1.1 Health bar overlays

| # | Requirement |
|---|-------------|
| 1 | R-10.3.1    |

1. **#1** -- Set `HealthBar` with `predicted_damage` and `absorb_shield` at simultaneous non-zero
   values.
   - **Expected:** Three visually distinct regions rendered: absorb, current, predicted damage.

### TC-10.3.3.1 Cooldown frame accuracy

| # | Requirement |
|---|-------------|
| 1 | R-10.3.3    |

1. **#1** -- Set a 3.0s cooldown; simulate 180 frames at 60 fps; read `cooldown_remaining`.
   - **Expected:** `cooldown_remaining` reaches 0 on exactly frame 180; radial sweep matches.

### TC-10.3.4.1 Nameplate overlap avoidance

| # | Requirement |
|---|-------------|
| 1 | R-10.3.4    |

1. **#1** -- Place 20 entities so their projected screen positions overlap; run overlap avoidance.
   - **Expected:** Rendered nameplates do not overlap; priority order respected; budget cap honored.

### TC-10.3.5.1 Combat text merge

| # | Requirement |
|---|-------------|
| 1 | R-10.3.5    |

1. **#1** -- Spawn five `FloatingCombatText` entities with the same `merge_key` within 100ms.
   - **Expected:** Displayed as a single merged value; trajectory is the average trajectory.

### TC-10.3.6.1 Minimap markers

| # | Requirement |
|---|-------------|
| 1 | R-10.3.6    |

1. **#1** -- Add 10 quest markers and 5 player markers to the minimap; pan the world map.
   - **Expected:** Markers move correctly relative to map pan; correct icons per marker type.

### TC-10.3.10.1 Compass bearing

| # | Requirement |
|---|-------------|
| 1 | R-10.3.10   |

1. **#1** -- Rotate camera 360 degrees; sample compass bar at 45-degree increments.
   - **Expected:** Compass bearing matches camera yaw within 0.5 degrees at all samples.

### TC-10.3.8.1 Chat 200 messages per second

| # | Requirement |
|---|-------------|
| 1 | R-10.3.8    |

1. **#1** -- Inject 200 chat messages in a single frame via the chat system.
   - **Expected:** All messages accepted without data loss; visible in chat within one frame; frame
     time stays within budget.

### TC-10.3.9.1 Inventory sort and filter

| # | Requirement |
|---|-------------|
| 1 | R-10.3.9    |

1. **#1** -- Populate inventory with 200 items; apply a type filter; then sort by value descending.
   - **Expected:** Filtered set shows only matching items; items sorted correctly; no visible
     jitter.

## Unit Tests — Accessibility

### TC-10.6.1.1 Accessible node creation

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** -- Spawn a `Button` widget entity with `AccessibleRole::Button` and a `label`.
   - **Expected:** `AccessibilityTree` contains one `AccessibleNode` with matching role and
     `properties.label`.

### TC-10.6.1.2 Accessibility tree sync — add and remove

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** -- Add then remove a widget; run `AccessibilityTreeSyncSystem` after each.
   - **Expected:** Tree reflects add on first sync; node absent after removal sync; dirty list
     cleared each frame.

### TC-10.6.1.3 Live region announce

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** -- Mark a widget with `live: Some(LiveRegionMode::Polite)`; update its text content.
   - **Expected:** `ScreenReaderPushSystem` emits a push to the platform bridge for the changed node
     within the same frame.

### TC-10.6.1.4 Focus tab order — accessibility

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** -- Simulate Tab key press across a panel with five focusable widgets.
   - **Expected:** Focus moves through widgets in DOM order; screen reader bridge notified of each
     focus change.

### TC-10.6.2.1 Subtitle timing

| # | Requirement |
|---|-------------|
| 1 | R-10.6.2    |

1. **#1** -- Queue a `SubtitleEntry` with `start_time: 1.0` and `end_time: 3.0`; advance time.
   - **Expected:** Subtitle visible during [1.0, 3.0]; not visible before or after.

### TC-10.6.2.2 Caption direction

| # | Requirement |
|---|-------------|
| 1 | R-10.6.2    |

1. **#1** -- Push a `CaptionEntry` with `direction: Some(CaptionDirection::Left)`.
   - **Expected:** Caption displays a left-facing directional indicator icon alongside the text.

### TC-10.6.3.1 Protanopia matrix

| # | Requirement |
|---|-------------|
| 1 | R-10.6.3    |

1. **#1** -- Set `ColorblindFilter` to `Protanopia` with `severity: 1.0`; sample output colors.
   - **Expected:** Output colors match the Machado et al. Protanopia simulation matrix within
     floating-point tolerance.

### TC-10.6.3.2 Deuteranopia matrix

| # | Requirement |
|---|-------------|
| 1 | R-10.6.3    |

1. **#1** -- Set `ColorblindFilter` to `Deuteranopia` with `severity: 1.0`.
   - **Expected:** Output colors match the Deuteranopia matrix within floating-point tolerance.

### TC-10.6.3.3 Tritanopia matrix

| # | Requirement |
|---|-------------|
| 1 | R-10.6.3    |

1. **#1** -- Set `ColorblindFilter` to `Tritanopia` with `severity: 1.0`.
   - **Expected:** Output colors match the Tritanopia matrix within floating-point tolerance.

### TC-10.6.7.1 Contrast ratio AA

| # | Requirement |
|---|-------------|
| 1 | R-10.6.7    |

1. **#1** -- Check `ContrastChecker::check` with foreground `#767676` and background `#FFFFFF`.
   - **Expected:** Returns `ContrastResult::Passes(WcagLevel::AA)`; ratio ≥ 4.5:1.

### TC-10.6.4.1 High contrast borders

| # | Requirement |
|---|-------------|
| 1 | R-10.6.4    |

1. **#1** -- Enable `HighContrastSettings`; render a button.
   - **Expected:** Button uses `border` color from `HighContrastSettings`; border width matches
     `border_width`; decorative elements removed.

### TC-10.6.4.2 UI scale 80 percent

| # | Requirement |
|---|-------------|
| 1 | R-10.6.4    |

1. **#1** -- Set `UiScaleSettings.user_scale` to 0.8; measure a widget's computed size.
   - **Expected:** Computed size is 80% of its base size.

### TC-10.6.4.3 UI scale 250 percent

| # | Requirement |
|---|-------------|
| 1 | R-10.6.4    |

1. **#1** -- Set `UiScaleSettings.user_scale` to 2.5; measure a widget's computed size.
   - **Expected:** Computed size is 250% of its base size; no overflow.

### TC-10.6.5.1 Rebind all actions

| # | Requirement |
|---|-------------|
| 1 | R-10.6.5    |

1. **#1** -- Call `InputRemapper::rebind` for every action with a new `InputSource`.
   - **Expected:** All actions use the new source; old sources no longer trigger actions.

### TC-10.6.5.2 Hold-to-toggle

| # | Requirement |
|---|-------------|
| 1 | R-10.6.5    |

1. **#1** -- Set `HoldToggle` on an action; hold for threshold duration; release.
   - **Expected:** Action remains active after release until pressed again; brief presses do not
     activate toggle.

### TC-10.6.5.3 Scanning navigation

| # | Requirement |
|---|-------------|
| 1 | R-10.6.5    |

1. **#1** -- Enable scanning mode; wait for auto-advance timer to reach a button; activate.
   - **Expected:** Focus advances automatically at scan interval; button activates on single-switch
     input at correct position.

### TC-10.6.6.1 TTS channel filter

| # | Requirement |
|---|-------------|
| 1 | R-10.6.6    |

1. **#1** -- Disable TTS for the "chat" channel; enable for "system"; fire both event types.
   - **Expected:** Chat text not spoken; system text spoken within 200ms.

### TC-10.6.7.2 Reduced motion — no camera shake

| # | Requirement |
|---|-------------|
| 1 | R-10.6.7    |

1. **#1** -- Enable `ReducedMotionSettings.disable_camera_shake`; trigger a camera shake event.
   - **Expected:** Camera shake not applied; other animations unaffected.

### TC-10.6.7.3 Focus indicator visible

| # | Requirement |
|---|-------------|
| 1 | R-10.6.7    |

1. **#1** -- Focus each focusable widget type; capture rendered frame.
   - **Expected:** Each focused widget has a visible focus ring; ring passes WCAG 2.1 non-text
     contrast (3:1 minimum).

## Integration Tests

### TC-10.1.2.1 UI asset round trip

| # | Requirement |
|---|-------------|
| 1 | R-10.1.2    |

1. **#1** -- Compile a `.ui` text file to `.ui.bin` via the content pipeline; load via
   `UIAssetLoader`; verify tree structure matches source.
   - **Expected:** All widget nodes, style sheets, and slot definitions are byte-equivalent to
     source after round-trip; zero deserialization work (mmap only).

### TC-10.1.4.3 Full HUD layout

| # | Requirement |
|---|-------------|
| 1 | R-10.1.4    |

1. **#1** -- Load a representative HUD layout with 500 widgets; run layout system; measure time.
   - **Expected:** Layout completes in under 0.5 ms on the reference platform.

### TC-10.1.10.1 World-space panel input

| # | Requirement |
|---|-------------|
| 1 | R-10.1.10   |

1. **#1** -- Raycast a pointer at a world-space `UiPanel` entity; fire a click event.
   - **Expected:** `EventRouter` receives the pointer event; hit test maps 3D ray to 2D UV coords;
     correct widget receives the event.

### TC-10.1.11.1 VR laser input

| # | Requirement |
|---|-------------|
| 1 | R-10.1.11   |

1. **#1** -- Simulate `XrPointerInput` pointing at a button in world-space; trigger select.
   - **Expected:** Button receives click event; `FocusManager` updates focus to the button.

### TC-10.4.1.1 Quad batching 500 widgets

| # | Requirement |
|---|-------------|
| 1 | R-10.4.1    |

1. **#1** -- Submit 500 widgets to `QuadBatcher`; verify draw call count.
   - **Expected:** Fewer than 50 indirect draw calls; all quads rendered correctly.

### TC-10.4.2.1 MSDF text scales

| # | Requirement |
|---|-------------|
| 1 | R-10.4.2    |

1. **#1** -- Render 5000 unique glyphs at font sizes 8, 16, 32, 64px; verify quality.
   - **Expected:** All glyphs render without artifacts at all sizes; frame time stays under 4 ms.

### TC-10.3.1.2 Raid frame 40 bars

| # | Requirement |
|---|-------------|
| 1 | R-10.3.1    |

1. **#1** -- Spawn 40 `HealthBar` entities with randomized values; simulate 60 frames.
   - **Expected:** All 40 bars render correctly at 60 fps; GPU time under 2 ms.

### TC-10.3.4.2 Nameplate 200 entities

| # | Requirement |
|---|-------------|
| 1 | R-10.3.4    |

1. **#1** -- Place 200 entities with `Nameplate` components; run culling pipeline; measure time.
   - **Expected:** Culling completes in under 0.5 ms; rendered set respects budget cap and priority.

### TC-10.6.1.5 VoiceOver on macOS

| # | Requirement |
|---|-------------|
| 1 | R-10.6.1    |

1. **#1** -- With VoiceOver active, navigate to a `Button` via Tab; read its announcement.
   - **Expected:** NSAccessibility bridge delivers the correct role and label to VoiceOver; no
     crashes.

### TC-10.6.2.3 Subtitle audio sync

| # | Requirement |
|---|-------------|
| 1 | R-10.6.2    |

1. **#1** -- Play a voice-over line with an associated `SubtitleEntry`; measure display offset.
   - **Expected:** Subtitle text visible within ±50ms of audio start.

### TC-10.6.3.4 Colorblind preview

| # | Requirement |
|---|-------------|
| 1 | R-10.6.3    |

1. **#1** -- Render the full HUD with each CVD mode; compare output textures against reference.
   - **Expected:** Each mode's output matches the reference CVD transform within 1 ULP tolerance.

### TC-10.6.5.4 Switch device full UI

| # | Requirement |
|---|-------------|
| 1 | R-10.6.5    |

1. **#1** -- Disable pointer input; navigate entire UI using only a single-switch input device in
   scanning mode.
   - **Expected:** Every interactive element is reachable and operable; no dead ends.

### TC-10.6.7.4 WCAG all screens

| # | Requirement |
|---|-------------|
| 1 | R-10.6.7    |

1. **#1** -- Run `ContrastChecker` on all text/background color pairs across all UI screens in the
   default theme.
   - **Expected:** All text passes WCAG 2.1 AA (4.5:1 normal, 3:1 large text); no failures.

## Benchmarks

### TC-10.1.12.B1 Tree diff 500 widgets

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500-widget tree, 10% dirty each frame | Wall time | < 1 ms | R-10.1.12 |

### TC-10.1.4.B1 Layout 500 widgets

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500-widget mixed flex/grid layout | Wall time | < 0.5 ms | R-10.1.4 |

### TC-10.1.6.B1 Style resolution 500 widgets

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 widgets, 20 style rules each | Wall time | < 0.3 ms | R-10.1.6 |

### TC-10.4.2.B1 Paint full HUD

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full HUD render, representative scene | GPU time | < 2 ms | US-10.4.2 |

### TC-10.4.1.B1 Quad batch 500

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 widgets submitted to QuadBatcher | Draw calls | < 50 | US-10.4.2 |

### TC-10.4.2.B2 MSDF 5000 glyphs

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 5000 unique glyphs rendered in one frame | Wall time | < 4 ms | R-10.4.2 |

### TC-10.2.5.B1 Virtual list 10k items

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Scroll 10 000-item list at 60 fps | Frame time | < 4 ms/frame | R-10.2.5 |

### TC-10.3.4.B1 Nameplate cull 250

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 250 nameplates, full culling pipeline | Wall time | < 0.5 ms | R-10.3.4 |

### TC-10.5.15.B1 2D particles 1024

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1024 active 2D particle effects | Wall time | < 1 ms | F-10.5.15 |

### TC-10.5.14.B1 2D multi-light 32 lights

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 32 2D point lights in a single scene | Wall time | < 2 ms | F-10.5.14 |

### TC-10.6.1.B1 Accessibility tree sync

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500-widget tree, 10% dirty | Wall time | < 0.5 ms | US-10.6.2 |

### TC-10.6.1.B2 Platform bridge push

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50 dirty nodes pushed to OS bridge | Wall time | < 1 ms | US-10.6.2 |

### TC-10.6.7.B1 Colorblind pass 1080p

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | CVD post-process pass at 1920x1080 | GPU time | < 0.3 ms | US-10.6.7 |

### TC-10.6.6.B1 TTS latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `TextToSpeech::speak` to audible output start | Latency | < 200 ms | US-10.6.16 |
