# 10.1 — Widget Framework

## Widget Tree

### F-10.1.1 Declarative Retained Widget Tree

UI is authored as declarative asset files (visual editor, not code) that describe the widget
hierarchy, bindings, styles, and layout constraints. At load time, the asset is compiled into a
retained widget tree. When state changes, the framework diffs the new declared tree against the
current retained tree and applies the minimal set of mutations — insert, remove, update, reorder —
without rebuilding the entire tree. This SwiftUI-inspired approach gives the performance of
retained-mode with the simplicity of declarative authoring. Artists and designers iterate on UI in
the visual editor; the framework handles efficient updates.

- **Dependencies:** None
- **Platform notes:** None

### F-10.1.2 Declarative UI Asset Format

A binary asset format for UI definitions. Each asset declares a widget tree with typed properties,
data binding expressions, style references, layout constraints, and event handlers (wired to logic
graph nodes, F-15.8.4). The format supports template composition — a `HealthBar` template is defined
once and instantiated wherever needed with different bindings. Templates support slots (children
injected by the consumer). The asset is edited exclusively through the visual UI editor; no text
editing required (no-code philosophy).

- **Dependencies:** F-10.1.1, F-12.7.1 (Binary Asset Format)
- **Platform notes:** None

### F-10.1.3 Widget Pooling and Recycling

Pool and recycle widget instances to avoid allocation churn in frequently rebuilt UI elements such
as virtualized list views, inventory grids, and nameplate containers. When a widget scrolls out of
view or is dismissed, it returns to the pool rather than being deallocated, and is rebound to new
data when needed. Critical for MMO scenarios where hundreds of nameplates, chat messages, and
inventory cells are created and destroyed per second.

- **Dependencies:** F-10.1.1
- **Platform notes:** Mobile enforces a lower active widget budget (200 vs 500 on desktop).
  Aggressive pooling and recycling are critical on memory-constrained devices.

## Layout

### F-10.1.4 Flexbox and Grid Layout

Provide CSS-like flexbox and grid layout algorithms for automatic widget positioning. Flexbox
handles one-dimensional flows (toolbars, action bars, chat input rows) while grid layout handles
two-dimensional arrangements (inventory grids, talent trees, auction house result tables). Both
support gap, alignment, justification, wrapping, and min/max size constraints.

- **Dependencies:** F-10.1.1
- **Platform notes:** None

### F-10.1.5 Anchor and Constraint Layout

Support anchor-based and constraint-based layout for HUD elements that must maintain fixed positions
relative to screen edges, other widgets, or world-space attachment points. Anchors define edges
(top, bottom, left, right, center) with pixel or percentage offsets. Constraints express
relationships between widget properties (e.g., "this widget's left edge equals that widget's right
edge plus 8px"), enabling adaptive layouts across resolutions.

- **Dependencies:** F-10.1.1
- **Platform notes:** None

## Styling

### F-10.1.6 CSS-like Styling and Themes

Apply visual properties (colors, fonts, borders, border-radius, padding, margins, opacity, shadows)
through a cascading style system inspired by CSS. Styles can be defined in external theme files and
swapped at runtime to support light/dark modes, faction-specific skins, and seasonal event themes.
Selectors match widgets by type, ID, class, and state (hovered, pressed, focused, disabled).

- **Dependencies:** F-10.1.1
- **Platform notes:** None

## Data Binding

### F-10.1.7 Reactive Data Binding

Bind widget properties to game state through a reactive data binding system so that UI updates
automatically when the underlying data changes. Bindings support one-way (model to view), two-way
(model to view and view to model), and computed/derived values. Essential for MMO UIs where player
stats, inventory contents, quest progress, and buff durations change continuously and must be
reflected in real time without manual polling.

- **Dependencies:** F-10.1.1
- **Platform notes:** None

## Focus and Navigation

### F-10.1.8 Focus and Navigation System

Manage keyboard and gamepad focus traversal across the widget tree with support for tab order,
directional navigation (D-pad / arrow keys), focus groups, and focus trapping for modal dialogs. The
system must handle complex MMO UI navigation scenarios: cycling through action bar slots, navigating
nested menus (settings, guild management), and switching between multiple open panels (inventory,
character sheet, map) without losing context.

- **Dependencies:** F-10.1.1
- **Platform notes:** Gamepad navigation uses platform input APIs; see domain 6 (Input).

## Localization

### F-10.1.9 Localization Hooks

Provide localization hooks at the widget level so that all user-visible text, images, and layout
directions can be switched by locale at runtime. Widgets automatically re-layout when text changes
length across languages. Support right-to-left (RTL) layout mirroring for Arabic and Hebrew.
MMO-scale localization requires handling thousands of string keys, pluralization rules, gendered
text, and number/date/currency formatting per locale.

- **Dependencies:** F-10.1.1, F-10.1.4
- **Platform notes:** None

## VR and 3D In-Game UI

### F-10.1.10 World-Space UI Panels

Render the same declarative widget tree (F-10.1.1) as a world-space 3D panel — a textured quad in
the 3D scene that receives ray-cast input from the player's cursor, gaze, or VR controller.
World-space panels support all layout modes, styling, and data binding. Used for diegetic interfaces
(in-game computer screens, holographic displays, shop kiosks) and VR menus that float in 3D space.
Panel resolution and physical size are configurable per instance.

- **Dependencies:** F-10.1.1, F-10.4.6 (World-Space UI)
- **Platform notes:** VR controller ray input uses the input action system (F-6.2).

### F-10.1.11 VR-Optimized UI Interaction

UI interaction modes for VR: laser pointer from hand controllers, direct touch (finger collides with
UI panel), gaze-and-dwell (look at a button for N seconds to activate), and hand tracking pinch
gestures. The widget tree's focus and navigation system (F-10.1.8) adapts to the active input mode.
UI panels curve slightly toward the viewer for readability. Text size auto-scales based on panel
distance to maintain legibility. Comfort settings clamp panel positions to avoid neck strain.

- **Dependencies:** F-10.1.10, F-10.1.8 (Focus System)
- **Platform notes:** OpenXR hand tracking and controller APIs.

### F-10.1.12 Automatic Minimal Tree Diffing

When bound data changes, the framework computes the minimal diff between the previous and current
declared widget trees using a keyed reconciliation algorithm (similar to React/SwiftUI diffing).
Only changed nodes are updated — property mutations patch in place, insertions and deletions splice
the tree, and reordered children are moved without destroy/recreate. The diffing algorithm runs in
O(n) for common cases (keyed lists) and avoids layout recalculation for subtrees with unchanged
geometry. This ensures 60fps UI updates even with hundreds of bound widgets changing simultaneously.

- **Dependencies:** F-10.1.1, F-10.1.7 (Data Binding)
- **Platform notes:** None

## Animation

### F-10.1.13 Widget Animation System

Animate widget properties (position, size, opacity, color, rotation, scale) over time using
keyframed curves with configurable easing functions (linear, ease-in, ease-out, ease-in-out, cubic
bezier, spring, bounce). Animations are declared as named animation assets authored in the UI visual
editor. Supports transition animations triggered by state changes (panel slide-in on open, fade-out
on close), looping animations (pulsing glow on highlighted items, spinning loading indicator),
staggered list animations (items animate in sequence with configurable delay), and interruptible
animations that smoothly blend to a new target when retriggered mid-flight. The animation system
runs independently of the game's animation system (domain 9), operating directly on widget tree
properties.

- **Dependencies:** F-10.1.1 (Widget Tree), F-10.1.12 (Tree Diffing)
- **Platform notes:** None

## Audio

### F-10.1.14 UI Audio Feedback

Automatic audio feedback for widget interactions: button click, hover enter, hover exit, focus gain,
focus loss, scroll, drag start, drag end, toggle on/off, slider value change, and notification
popup. Each widget type defines default sound slots that designers override per-widget or per-theme.
Sounds reference audio assets played through a dedicated UI mixer bus (F-5.1.3) with low-latency
scheduling. Audio feedback respects accessibility settings — can be disabled globally, individually
per sound type, or replaced with haptic feedback (F-6.4.1) on supported platforms.

- **Dependencies:** F-10.1.1 (Widget Tree), F-5.1.3 (Mixer Bus Graph), F-5.1.7 (Audio Formats)
- **Platform notes:** On mobile, UI sounds use low-latency audio paths (AudioToolbox on iOS,
  SoundPool on Android) to avoid mixer bus scheduling delay.
