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

### TC-10.3.2.1 Buff/debuff radial sweep and priority filter

| # | Requirement |
|---|-------------|
| 1 | R-10.3.2    |
| 2 | R-10.3.2    |

1. **#1** -- Apply 35 buff/debuff effects with 7 priority tiers and randomized remaining durations
   to a single target; advance one frame.
   - **Expected:** All 35 icons grouped by priority tier; radial sweep angle equals
     `remaining / total * 360` within 0.5 degree.
2. **#2** -- Enable priority filter to hide all but the top 10 effects.
   - **Expected:** Exactly 10 icons visible; hidden icons removed from quad submission.

### TC-10.3.7.1 Quest tracker progress and waypoint

| # | Requirement |
|---|-------------|
| 1 | R-10.3.7    |
| 2 | R-10.3.7    |

1. **#1** -- Add a 3-step quest chain with step 2 active; 5/10 sub-objectives complete; waypoint at
   world position (100, 0, 200); player at (0, 0, 0).
   - **Expected:** Tracker shows step 2 label, "5/10" progress text, compass bearing toward (100, 0,
     200), and distance 223.6 m within 0.5 m.
2. **#2** -- Advance the sub-objective count to 10/10.
   - **Expected:** Tracker transitions to step 3; step 2 shown as complete; compass waypoint updated
     to new step target.

### TC-10.3.11.1 Off-screen objective edge arrow

| # | Requirement |
|---|-------------|
| 1 | R-10.3.11   |
| 2 | R-10.3.11   |

1. **#1** -- Track an objective behind the camera (behind frustum); run HUD update.
   - **Expected:** Edge arrow appears on screen edge pointing toward the objective; in-world marker
     absent.
2. **#2** -- Rotate camera until the objective enters the frustum.
   - **Expected:** Edge arrow fades out; in-world marker fades in over same transition frames; no
     visual pop.

### TC-10.3.12.1 Procedural minimap chunk stream

| # | Requirement |
|---|-------------|
| 1 | R-10.3.12   |
| 2 | R-10.3.12   |

1. **#1** -- Load a procedural world tile with terrain, biome, building, and road data; run minimap
   generator.
   - **Expected:** Minimap texture contains terrain shading, biome tint, building footprints, and
     road polylines for the loaded tile.
2. **#2** -- Stream in a new adjacent chunk.
   - **Expected:** Minimap texture incrementally updates for the new chunk region; no full redraw of
     previously generated tiles.

### TC-10.3.13.1 World map tiled image pyramid zoom

| # | Requirement |
|---|-------------|
| 1 | R-10.3.13   |
| 2 | R-10.3.13   |

1. **#1** -- Open world map with 6-level tile pyramid; pan and zoom from world level to local.
   - **Expected:** Correct LOD tiles loaded at each zoom step; map continuously panned without
     missing tiles.
2. **#2** -- Place a waypoint at a map-space coordinate; initiate fast travel.
   - **Expected:** Waypoint entity created at the world-space coordinate; fast-travel event fired
     with destination.

### TC-10.3.14.1 Map overlay post-process styles

| # | Requirement |
|---|-------------|
| 1 | R-10.3.14   |
| 2 | R-10.3.14   |

1. **#1** -- Apply each of 5 post-processing styles (parchment, painted, monochrome, ink, neon) to a
   generated map; read back the output texture.
   - **Expected:** Each style produces distinct output pixels versus the base generated map.
2. **#2** -- Substitute a static hand-painted map with active dynamic markers.
   - **Expected:** Hand-painted texture rendered as base; dynamic marker overlay preserved and
     rendered on top.

### TC-10.3.15.1 Data-driven marker clustering

| # | Requirement |
|---|-------------|
| 1 | R-10.3.15   |
| 2 | R-10.3.15   |

1. **#1** -- Register markers across 8 categories (quest, POI, merchant, enemy, ally, resource,
   event, waypoint) with quest-driven visibility flags.
   - **Expected:** Markers render on minimap and compass simultaneously with identical icons per
     category; hidden markers absent.
2. **#2** -- Cluster 50 markers within a single minimap pixel bucket.
   - **Expected:** Cluster icon replaces individual markers; cluster count badge shows 50.

### TC-10.4.3.1 Vector path Bezier stroke and gradient

| # | Requirement |
|---|-------------|
| 1 | R-10.4.3    |
| 2 | R-10.4.3    |

1. **#1** -- Render a filled cubic Bezier curve and a stroked arc with a conical gradient at 100%
   and 300% UI scale.
   - **Expected:** Tessellated or SDF output matches reference pixel data within 1 ULP tolerance at
     both scales; no jaggies at 300%.
2. **#2** -- Render a filled path with 500 curve segments.
   - **Expected:** Vector path renderer emits a single draw; GPU compute tessellation completes
     within the frame budget.

### TC-10.4.4.1 Nine-slice atlas incremental repack

| # | Requirement |
|---|-------------|
| 1 | R-10.4.4    |
| 2 | R-10.4.4    |

1. **#1** -- Pack a nine-slice texture into the UI atlas and render a 200x80 button using the
   nine-slice solver.
   - **Expected:** Corner, edge, and center regions map to correct atlas UVs; stretched edges are
     tiled; corners are 1:1.
2. **#2** -- Stream 20 new textures into the atlas.
   - **Expected:** Incremental repack completes in under 2 ms; existing UV handles remain valid.

### TC-10.4.5.1 Render-to-texture 3D preview

| # | Requirement |
|---|-------------|
| 1 | R-10.4.5    |
| 2 | R-10.4.5    |

1. **#1** -- Spawn 4 RTT preview targets at half resolution; submit orbit-camera scenes to each.
   - **Expected:** All 4 preview textures rendered; each texture reflects its scene's camera pose.
2. **#2** -- Animate the preview camera by 10 degrees per frame.
   - **Expected:** Preview texture updates each frame without stalls.

### TC-10.4.6.1 World-space UI with depth test

| # | Requirement |
|---|-------------|
| 1 | R-10.4.6    |
| 2 | R-10.4.6    |

1. **#1** -- Place a world-space UI panel at world position (0, 2, 5); render with depth test
   enabled.
   - **Expected:** Panel occluded by a closer 3D object; panel perspective matches scene camera
     matrix.
2. **#2** -- Disable depth test on the panel.
   - **Expected:** Panel draws through the occluder.

### TC-10.4.7.1 SDF edge AA and stencil clip

| # | Requirement |
|---|-------------|
| 1 | R-10.4.7    |
| 2 | R-10.4.7    |

1. **#1** -- Render a rounded rectangle at 100%, 200%, and 300% UI scale with SDF anti-aliasing.
   - **Expected:** Edge transition is 1.5 pixels wide at all scales; no staircasing.
2. **#2** -- Place a child widget outside the parent's `clip_rect`; render with stencil clipping.
   - **Expected:** Child geometry clipped at parent bounds; no pixels drawn outside parent rect.

### TC-6.1.1.1 Keyboard press/release/repeat events reach UI

| # | Requirement |
|---|-------------|
| 1 | R-6.1.1     |

1. **#1** -- Dispatch platform keyboard press, release, and auto-repeat events through the input
   system to the UI `EventRouter` for a focused `TextInput` widget.
   - **Expected:** `TextInput` receives `KeyDown`, `KeyUp`, and `KeyRepeat` events in order with
     matching scancodes; no events dropped.

### TC-8.9.10.1 Chat profanity filter integration

| # | Requirement |
|---|-------------|
| 1 | R-8.9.10    |

1. **#1** -- Post a chat message containing a filtered word via the `ChatSystem`; profanity filter
   evaluates against the full word list.
   - **Expected:** Filtered word replaced with mask characters in the rendered chat entry; original
     message preserved for moderation logs.

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

### TC-10.1.1.I1 Declarative screen composition with template slots

| # | Requirement |
|---|-------------|
| 1 | US-10.1.1   |

1. **#1** -- Load a `.ui.bin` authored in the visual editor that composes a parent template with a
   child `UiTemplate` instantiated into a named `SlotDef`; instantiate via `WidgetPool`.
   - **Expected:** Parent and child widget trees are joined at the slot entity; `TemplateBinding`
     maps parent properties into child bindings; no code was written to assemble the tree.

### TC-10.1.3.I1 Widget pooling for virtualized lists

| # | Requirement |
|---|-------------|
| 1 | US-10.1.2   |

1. **#1** -- Scroll a `VirtualList` of 5000 items and an inventory grid of 200 slots concurrently
   for 500 frames; track heap allocations via test allocator.
   - **Expected:** `WidgetPool::acquire` reuses recycled entities; zero heap allocations after the
     first frame for both the list and the grid.

### TC-10.1.7.I1 Same-frame data binding update

| # | Requirement |
|---|-------------|
| 1 | US-10.1.3   |

1. **#1** -- Bind player HUD widgets (health, mana, quest progress) via `DataBindingComponent` to a
   player entity; mutate all three components in one frame.
   - **Expected:** All three HUD widgets render the new values within the same frame; no one-frame
     lag between state change and display.

### TC-10.1.4.I1 Flex and grid compose toolbar and inventory

| # | Requirement |
|---|-------------|
| 1 | US-10.1.4   |

1. **#1** -- Author a toolbar using flex row and an inventory grid using 8x6 grid; resize the
   container.
   - **Expected:** Toolbar items re-flow automatically; grid cells stay at equal width and height
     with no manual per-item positioning.

### TC-10.1.5.I1 Anchor/constraint HUD across resolutions

| # | Requirement |
|---|-------------|
| 1 | US-10.1.5   |

1. **#1** -- Author a HUD with bottom-right anchored action bar and top-center constrained health
   bar; render at 1280x720, 1920x1080, and 3840x2160.
   - **Expected:** Action bar stays bottom-right; health bar stays top-centered at all three
     resolutions; no widgets cropped.

### TC-10.1.6.I1 Runtime theme swap for light/dark and faction skins

| # | Requirement |
|---|-------------|
| 1 | US-10.1.6   |

1. **#1** -- Register three themes (light, dark, faction-red) sharing one widget tree; swap active
   theme between all three on consecutive frames.
   - **Expected:** The same widget tree renders with each theme's colors and fonts; no widget tree
     rebuild; style cascade resolves within one frame of swap.

### TC-10.1.7.I2 Reactive one-way, two-way, and computed bindings

| # | Requirement |
|---|-------------|
| 1 | US-10.1.7   |

1. **#1** -- Configure a `Label` with one-way binding to health, a `Slider` with two-way binding to
   volume, and a `ProgressBar` with a computed binding `current/max * 100`; mutate all sources.
   - **Expected:** Label reflects source; slider round-trips back to source on drag; progress bar
     reflects computed ratio; no polling code required in test harness.

### TC-10.1.8.I1 Keyboard tab, D-pad, and modal focus trap

| # | Requirement |
|---|-------------|
| 1 | US-10.1.8   |

1. **#1** -- Present a screen with 6 focusable widgets and a modal dialog with 3 focusable widgets;
   drive focus with keyboard Tab and gamepad D-pad.
   - **Expected:** Tab cycles through the 6 widgets in tab-index order; D-pad moves focus spatially;
     after modal opens, focus is trapped to the 3 modal widgets.

### TC-10.1.9.I1 Runtime language switch with RTL mirror

| # | Requirement |
|---|-------------|
| 1 | US-10.1.9   |

1. **#1** -- Boot with `en-US`; switch to `ar-SA` at runtime via `LocalizationManager`; verify a
   screen containing text, an image with locale-specific variant, and a flex row.
   - **Expected:** All `LocalizedStringId` values resolve to Arabic; locale-specific image swapped
     automatically via `Handle<T>`; flex row direction reversed; no restart required.

### TC-10.1.11.I1 VR laser, touch, gaze, and hand-track pinch

| # | Requirement |
|---|-------------|
| 1 | US-10.1.10  |

1. **#1** -- Attach an OpenXR session with laser, hand tracking, and eye tracking inputs; point at a
   world-space UI panel using each method in turn.
   - **Expected:** Laser ray hits correct button and fires click; direct-touch intersection fires
     click on collision; gaze-and-dwell fires click after dwell threshold; hand pinch fires click.

### TC-10.1.10.I1 Diegetic world-space panel with ray-cast input

| # | Requirement |
|---|-------------|
| 1 | US-10.1.11  |

1. **#1** -- Instantiate the same `UiTemplate` once as a screen overlay and once as a world-space
   panel on a game object; ray-cast pointer input at the world panel.
   - **Expected:** Both instances share the same widget tree source; screen overlay uses
     screen-space quad batch; world panel receives `EventRouter` events via 3D ray hit test.

### TC-10.1.14.I1 Disable UI audio globally and per-type

| # | Requirement |
|---|-------------|
| 1 | US-10.1.12  |

1. **#1** -- Set `AudioSlots` master mute; then unmute and mute only `click`; then route disabled
   audio events through the haptic output adapter.
   - **Expected:** When master muted, no audio events fire; when click-only muted, hover and focus
     still fire; haptic events fire in place of silenced audio events.

### TC-10.1.13.I1 Keyframe animations with easing

| # | Requirement |
|---|-------------|
| 1 | US-10.1.13  |

1. **#1** -- Author a widget property animation with 4 keyframes and `ease_in_out_cubic` easing;
   play and sample every 10 frames.
   - **Expected:** Sampled values match the easing curve within 0.5% tolerance; transition ends at
     exact final keyframe.

### TC-10.1.14.I2 Automatic audio feedback on click/hover/scroll/notify

| # | Requirement |
|---|-------------|
| 1 | US-10.1.14  |

1. **#1** -- Click a `Button`, hover over a second button, scroll a `ScrollView`, and fire a
   `Notification` toast.
   - **Expected:** Four distinct audio events fire on the UI bus: `click`, `hover_enter`,
     `scroll_tick`, and the toast `panel_open` sound.

<!-- THIN: design section lacks detail -->
### TC-10.1.9.I2 Localization hook fallback to base locale

| # | Requirement |
|---|-------------|
| 1 | US-10.1.15  |

1. **#1** -- Switch to `ja-JP`; request a `LocalizedStringId` absent from the Japanese bundle but
   present in the `en-US` base.
   - **Expected:** Resolver returns the `en-US` string and logs a dev-mode warning; raw string ID is
     never displayed.

<!-- THIN: design section lacks detail -->
### TC-10.1.9.I3 Locale-aware number/date formatting

| # | Requirement |
|---|-------------|
| 1 | US-10.1.16  |

1. **#1** -- Format the number `1234567.89` and the timestamp `2026-04-12T18:30:00Z` under the
   `de-DE` locale.
   - **Expected:** Number formatted as `1.234.567,89`; date formatted as `12.04.2026 18:30` per CLDR
     patterns.

<!-- THIN: design section lacks detail -->
### TC-10.1.10.I2 World-space panel depth sorting

| # | Requirement |
|---|-------------|
| 1 | US-10.1.17  |

1. **#1** -- Place two world-space panels at different depths; render with depth-test enabled and a
   3D occluder between them.
   - **Expected:** Closer panel draws on top; the occluder hides the far panel's overlapping region;
     no sorting artifacts.

<!-- THIN: design section lacks detail -->
### TC-10.1.11.I2 VR gaze-and-dwell selection

| # | Requirement |
|---|-------------|
| 1 | US-10.1.18  |

1. **#1** -- Fix gaze on a button for 800 ms; dwell threshold is 750 ms.
   - **Expected:** Button `click` event fires exactly once at 750 ms; `EventRouter` does not
     double-fire.

<!-- THIN: design section lacks detail -->
### TC-10.1.11.I3 VR hand-tracked pinch input

| # | Requirement |
|---|-------------|
| 1 | US-10.1.19  |

1. **#1** -- Simulate an `XrHandTrackingEXT` pinch gesture while pointing at a slider.
   - **Expected:** Slider enters drag state on pinch-start; tracks pinch position until pinch-end;
     releases cleanly.

<!-- THIN: design section lacks detail -->
### TC-10.1.13.I2 Interrupt mid-animation transition

| # | Requirement |
|---|-------------|
| 1 | US-10.1.20  |

1. **#1** -- Start a 1.0 s scale tween; at 0.3 s trigger a new tween to a different target.
   - **Expected:** New tween begins from the current interpolated value; no visual pop; old tween
     marked cancelled.

<!-- THIN: design section lacks detail -->
### TC-10.1.13.I3 Widget state transition animation

| # | Requirement |
|---|-------------|
| 1 | US-10.1.21  |

1. **#1** -- Drive a button's `InteractionState` through `Idle -> Hover -> Pressed -> Idle`; each
   transition has a registered tween.
   - **Expected:** Each state change plays its tween; final state is stable with interaction
     matching last input.

<!-- THIN: design section lacks detail -->
### TC-10.1.14.I3 Focus audio feedback

| # | Requirement |
|---|-------------|
| 1 | US-10.1.22  |

1. **#1** -- Tab focus through 5 widgets, each with a `focus_gain` audio slot assigned.
   - **Expected:** 5 distinct `focus_gain` audio events fire on the UI bus in focus order; each
     plays on the same frame the focus change occurs.

<!-- THIN: design section lacks detail -->
### TC-10.1.14.I4 Spatial audio for world-space panels

| # | Requirement |
|---|-------------|
| 1 | US-10.1.23  |

1. **#1** -- Assign `spatial_falloff` to a world-space panel's `AudioSlots`; trigger a button click
   event on the panel at (5, 0, 0) while listener is at (0, 0, 0).
   - **Expected:** Click audio routed through the spatial audio bus with position (5, 0, 0); volume
     attenuated per the falloff curve.

### TC-10.6.1.I1 Screen reader tree exposure

| # | Requirement |
|---|-------------|
| 1 | US-10.6.1   |

1. **#1** -- With VoiceOver active on macOS, build a screen with a button, text field, and toggle;
   navigate via screen reader.
   - **Expected:** Each widget's role, label, and state are announced; tree structure matches
     visible widget hierarchy; no crashes.

### TC-10.6.3.I1 Colorblind mode with non-color cues

| # | Requirement |
|---|-------------|
| 1 | US-10.6.3   |

1. **#1** -- Enable `Protanopia` colorblind mode on a HUD with red/green health status; render.
   - **Expected:** CVD post-process applied; status widgets display icon/pattern cues in addition to
     color so information is readable without relying on hue.

### TC-10.6.4.I1 Non-color cues for all color-coded elements

| # | Requirement |
|---|-------------|
| 1 | US-10.6.4   |

1. **#1** -- Inspect all color-coded gameplay elements (health, buff tier, quest status) in the
   default theme; verify each has an alternative cue.
   - **Expected:** Every color-coded element pairs with a shape, icon, or pattern; no element
     conveys state by hue alone.

### TC-10.6.4.I2 UI scale with high contrast

| # | Requirement |
|---|-------------|
| 1 | US-10.6.5   |

1. **#1** -- Set `UiScaleSettings.user_scale` to 2.0 and enable `HighContrastSettings`; render a
   full menu screen.
   - **Expected:** All widgets double in size proportionally; high-contrast borders and colors
     applied; no clipping or overlap.

### TC-10.6.5.I1 Full input remap and switch-device scanning

| # | Requirement |
|---|-------------|
| 1 | US-10.6.6   |

1. **#1** -- Remap every `ActionId` to alternative `InputSource` values via `InputRemapper::rebind`
   including a one-handed layout; enable scanning mode.
   - **Expected:** All actions fire from new sources; old sources inactive; scanning mode auto-
     advances focus and activates on single-switch input.

<!-- THIN: design section lacks detail -->
### TC-10.6.4.I3 Accessible default styles meet WCAG

| # | Requirement |
|---|-------------|
| 1 | US-10.6.8   |

1. **#1** -- Query every widget type's default style in the base theme; compute `ContrastChecker`
   for its default text/background pair.
   - **Expected:** Every default passes WCAG 2.1 AA (>= 4.5:1 normal, >= 3:1 large); failures
     reported per widget type.

<!-- THIN: design section lacks detail -->
### TC-10.6.5.I2 Hold-to-toggle conversion

| # | Requirement |
|---|-------------|
| 1 | US-10.6.9   |

1. **#1** -- Register `HoldToggle` on a sprint action; briefly tap and then press-release.
   - **Expected:** Brief tap does not toggle; press-release toggles sprint on; another tap toggles
     off.

<!-- THIN: design section lacks detail -->
### TC-10.6.7.I1 Full WCAG audit across menu UI

| # | Requirement |
|---|-------------|
| 1 | US-10.6.10  |

1. **#1** -- Run automated WCAG 2.1 audit across all menu screens; report contrast failures and
   missing focus indicators.
   - **Expected:** All menu UI passes AA (AAA for critical gameplay info); report enumerates 0
     failures on default theme.

<!-- THIN: design section lacks detail -->
### TC-10.6.4.I4 High contrast border thickness

| # | Requirement |
|---|-------------|
| 1 | US-10.6.12  |

1. **#1** -- Enable `HighContrastSettings` with `border_width: 3.0`; render a button.
   - **Expected:** Button border is 3 pixels thick with the configured foreground color; visible at
     all UI scales.

<!-- THIN: design section lacks detail -->
### TC-10.6.5.I3 Input rebinding profile persistence

| # | Requirement |
|---|-------------|
| 1 | US-10.6.13  |

1. **#1** -- Create `InputProfile` A, rebind several actions, save; create profile B with different
   bindings; switch profiles via `InputRemapper::load_profile`.
   - **Expected:** Switching applies the target profile's bindings exactly; switching back restores
     profile A; no cross-contamination between profiles.

<!-- THIN: design section lacks detail -->
### TC-10.6.5.I4 Per-character input profile

| # | Requirement |
|---|-------------|
| 1 | US-10.6.15  |

1. **#1** -- Create two `InputProfile` entries with distinct `character_id` values; load the first,
   play; load the second on character swap.
   - **Expected:** Each character uses its own bindings; swap applies new bindings on the next frame
     with no input drops.

<!-- THIN: design section lacks detail -->
### TC-10.6.6.I1 TTS per-channel enable/disable

| # | Requirement |
|---|-------------|
| 1 | US-10.6.18  |

1. **#1** -- Enable TTS for `system` channel, disable for `chat`; emit one message per channel.
   - **Expected:** System message spoken via `TextToSpeech::speak`; chat message not spoken.

<!-- THIN: design section lacks detail -->
### TC-10.6.6.I2 TTS chat conversion latency

| # | Requirement |
|---|-------------|
| 1 | US-10.6.19  |

1. **#1** -- Emit a chat message with TTS enabled; measure time from dispatch to audible output
   start via the platform TTS backend.
   - **Expected:** Latency under 200 ms on all supported platforms.

<!-- THIN: design section lacks detail -->
### TC-10.6.7.I2 Reduced motion suppresses parallax

| # | Requirement |
|---|-------------|
| 1 | US-10.6.21  |

1. **#1** -- Enable `ReducedMotionSettings.disable_parallax`; animate a parallax background.
   - **Expected:** Background does not parallax-scroll; other animations unaffected.

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
