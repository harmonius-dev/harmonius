# R-10.1 — Widget Framework Requirements

## R-10.1.1–R-10.1.14 Widget Framework

| ID        | Derived From                                          |
|-----------|-------------------------------------------------------|
| R-10.1.1  | [F-10.1.1](../../features/ui-2d/widget-framework.md)  |
| R-10.1.2  | [F-10.1.2](../../features/ui-2d/widget-framework.md)  |
| R-10.1.3  | [F-10.1.3](../../features/ui-2d/widget-framework.md)  |
| R-10.1.4  | [F-10.1.4](../../features/ui-2d/widget-framework.md)  |
| R-10.1.5  | [F-10.1.5](../../features/ui-2d/widget-framework.md)  |
| R-10.1.6  | [F-10.1.6](../../features/ui-2d/widget-framework.md)  |
| R-10.1.7  | [F-10.1.7](../../features/ui-2d/widget-framework.md)  |
| R-10.1.8  | [F-10.1.8](../../features/ui-2d/widget-framework.md)  |
| R-10.1.9  | [F-10.1.9](../../features/ui-2d/widget-framework.md)  |
| R-10.1.10 | [F-10.1.10](../../features/ui-2d/widget-framework.md) |
| R-10.1.11 | [F-10.1.11](../../features/ui-2d/widget-framework.md) |
| R-10.1.12 | [F-10.1.12](../../features/ui-2d/widget-framework.md) |
| R-10.1.13 | [F-10.1.13](../../features/ui-2d/widget-framework.md) |
| R-10.1.14 | [F-10.1.14](../../features/ui-2d/widget-framework.md) |

1. **R-10.1.1** — The engine **SHALL** compile declarative UI asset files into a retained widget
   tree and apply minimal mutations (insert, remove, update, reorder) on state changes without
   rebuilding the entire tree.
   - **Rationale:** A retained tree with differential updates avoids the cost of full rebuilds while
     preserving the simplicity of declarative authoring for artists and designers.
   - **Verification:** Unit test modifies bound state on a 500-widget tree and asserts that only the
     changed subtree nodes receive mutation calls; unchanged subtrees remain untouched.
2. **R-10.1.2** — The engine **SHALL** provide a binary asset format for UI definitions that
   supports typed properties, data bindings, style references, layout constraints, event handler
   wiring, template composition, and template slots.
   - **Rationale:** A structured binary format enables fast loading and round-trip editing through
     the visual editor without requiring users to write or modify text files.
   - **Verification:** Integration test round-trips a UI asset containing templates with slots
     through save/load and confirms the deserialized tree matches the original.
3. **R-10.1.3** — The engine **SHALL** pool and recycle widget instances so that widgets removed
   from the tree are returned to a pool and rebound to new data on reuse, producing zero heap
   allocations during steady-state list scrolling.
   - **Rationale:** Allocation churn from frequent widget creation and destruction causes frame
     hitches in scenarios such as scrolling lists and nameplate updates.
   - **Verification:** Performance test scrolls a 1000-item virtualized list for 10 seconds and
     asserts zero widget allocations after the initial pool fill.
4. **R-10.1.4** — The engine **SHALL** implement flexbox and CSS grid layout algorithms supporting
   gap, alignment, justification, wrapping, and min/max size constraints for automatic widget
   positioning.
   - **Rationale:** Flexbox handles one-dimensional flows and grid handles two-dimensional
     arrangements, covering the full range of UI layout patterns.
   - **Verification:** Layout test compares computed widget positions and sizes against reference
     values for flex-wrap, grid-template, gap, and min/max constraint scenarios.
5. **R-10.1.5** — The engine **SHALL** support anchor-based layout (edges with pixel or percentage
   offsets) and constraint-based layout (relational expressions between widget properties) that
   adapt to any screen resolution.
   - **Rationale:** HUD elements must maintain fixed positions relative to screen edges or other
     widgets across all supported resolutions and aspect ratios.
   - **Verification:** Layout test anchors a widget to the bottom-right corner at multiple
     resolutions and asserts the pixel offset from the screen edge is constant.
6. **R-10.1.6** — The engine **SHALL** apply visual styles (colors, fonts, borders, padding,
   margins, opacity, shadows) through a cascading system with selectors matching widget type, ID,
   class, and state, and support runtime theme swapping.
   - **Rationale:** Cascading styles with runtime theme swapping let designers change the entire
     visual identity of the UI without rebuilding assets.
   - **Verification:** Test applies a theme, swaps to an alternate theme at runtime, and asserts all
     affected widgets reflect the new theme's property values within one frame.
7. **R-10.1.7** — The engine **SHALL** provide a reactive data binding system supporting one-way,
   two-way, and computed bindings that automatically updates widget properties when the underlying
   game state changes.
   - **Rationale:** Reactive bindings eliminate manual polling and ensure the UI stays in sync with
     continuously changing game state such as player stats and quest progress.
   - **Verification:** Unit test binds a widget label to a model value, mutates the model, and
     asserts the widget text updates without explicit refresh calls.
8. **R-10.1.8** — The engine **SHALL** manage keyboard and gamepad focus traversal with tab order,
   directional navigation, focus groups, and focus trapping for modal dialogs across arbitrarily
   nested widget hierarchies.
   - **Rationale:** Complex UIs with multiple open panels require structured focus management to
     prevent focus from escaping modals or losing context when switching panels.
   - **Verification:** Test opens a modal dialog, sends tab and directional key events, and asserts
     focus cycles within the modal and does not escape to background panels.
9. **R-10.1.9** — The engine **SHALL** provide per-widget localization hooks that switch text,
   images, and layout direction by locale at runtime, including RTL mirroring, pluralization,
   gendered text, and locale-specific number/date/currency formatting.
   - **Rationale:** Switching locale must automatically re-layout widgets to accommodate varying
     text lengths and RTL scripts without manual intervention.
   - **Verification:** Test switches locale from English (LTR) to Arabic (RTL) and asserts that
     layout direction mirrors and text content updates to the Arabic string table.
10. **R-10.1.10** — The engine **SHALL** render a declarative widget tree as a world-space 3D panel
    with configurable resolution and physical size that receives ray-cast input and supports all
    layout modes, styling, and data binding.
    - **Rationale:** Diegetic interfaces and VR menus require the full widget framework to operate
      on textured quads positioned in 3D space.
    - **Verification:** Integration test renders a world-space panel, performs a ray-cast click on a
      button widget, and asserts the button's click handler fires.
11. **R-10.1.11** — The engine **SHALL** support laser pointer, direct touch, gaze-and-dwell, and
    hand tracking pinch gesture input modes for VR UI panels, with automatic text scaling based on
    panel distance and comfort clamping of panel positions.
    - **Rationale:** VR users interact with UI through different modalities than desktop users, and
      readability requires distance-aware scaling and ergonomic panel placement.
    - **Verification:** Test activates each VR input mode in sequence and asserts that a button on a
      world-space panel can be activated through each mode.
12. **R-10.1.12** — The engine **SHALL** compute the minimal diff between previous and current
    widget trees using a keyed reconciliation algorithm that runs in O(n) for keyed lists and
    updates only changed nodes in place.
    - **Rationale:** O(n) diffing with in-place patching avoids unnecessary allocations and layout
      recalculations, keeping UI updates within frame budget.
    - **Verification:** Benchmark test diffs a 500-widget keyed list with 10% changes and asserts
      the operation completes within 1ms and touches only the changed nodes.
13. **R-10.1.13** — The engine **SHALL** animate widget properties (position, size, opacity, color,
    rotation, scale) using keyframed curves with configurable easing, supporting transition,
    looping, staggered, and interruptible animation modes.
    - **Rationale:** Smooth, interruptible animations with multiple easing modes are essential for
      responsive UI feedback and polished visual transitions.
    - **Verification:** Test triggers a slide-in animation, interrupts it mid-flight with a new
      target, and asserts the widget smoothly blends to the new target without discontinuity.
14. **R-10.1.14** — The engine **SHALL** play configurable audio feedback for widget interactions
    (click, hover, focus, scroll, drag, toggle, slider change, notification) through a dedicated UI
    mixer bus with per-widget and per-theme sound overrides.
    - **Rationale:** Audio feedback reinforces user interactions and must be configurable per
      widget, per theme, and per accessibility setting.
    - **Verification:** Test clicks a button widget and asserts the expected audio asset is
      dispatched to the UI mixer bus; repeat after disabling UI sounds globally and assert no audio
      is dispatched.
