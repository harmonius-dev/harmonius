# R-10.1 — Widget Framework Requirements

## Widget Tree

1. **R-10.1.1** — The engine **SHALL** compile declarative UI asset files into a retained widget
   tree and apply minimal mutations (insert, remove, update, reorder) on state changes without
   rebuilding the entire tree.
   - **Rationale:** Retained tree with differential updates avoids full rebuild cost while
     preserving declarative authoring simplicity.
   - **Verification:** Modify bound state on a 500-widget tree. Assert only changed subtree nodes
     receive mutation calls; unchanged subtrees are untouched.

2. **R-10.1.2** — The engine **SHALL** provide a binary asset format for UI definitions supporting
   typed properties, data bindings, style references, layout constraints, event handlers, template
   composition, and template slots.
   - **Rationale:** A structured binary format enables fast loading and round-trip editing through
     the visual editor without text files.
   - **Verification:** Round-trip a UI asset containing templates with slots through save/load.
     Assert the deserialized tree matches the original.

3. **R-10.1.3** — The engine **SHALL** pool and recycle widget instances, producing zero heap
   allocations during steady-state list scrolling.
   - **Rationale:** Allocation churn from frequent widget creation causes frame hitches in scrolling
     lists.
   - **Verification:** Scroll a 1000-item virtualized list for 10 seconds. Assert zero widget
     allocations after initial pool fill.

## Layout

4. **R-10.1.4** — The engine **SHALL** implement flexbox and CSS grid layout algorithms supporting
   gap, alignment, justification, wrapping, and min/max size constraints.
   - **Rationale:** Flexbox handles one-dimensional flows; grid handles two-dimensional
     arrangements.
   - **Verification:** Compare computed widget positions against reference values for flex-wrap,
     grid-template, gap, and min/max constraint scenarios.

5. **R-10.1.5** — The engine **SHALL** support anchor-based layout (edges with pixel or percentage
   offsets) and constraint-based layout (relational expressions) that adapt to any screen
   resolution.
   - **Rationale:** HUD elements must maintain fixed positions relative to screen edges across
     resolutions.
   - **Verification:** Anchor a widget to bottom-right at multiple resolutions. Assert pixel offset
     is constant.

## Styling and Binding

6. **R-10.1.6** — The engine **SHALL** apply visual styles through a cascading system with selectors
   matching widget type, ID, class, and state, with runtime theme swapping.
   - **Rationale:** Runtime theme swapping lets designers change visual identity without rebuilding
     assets.
   - **Verification:** Apply theme, swap to alternate at runtime. Assert widgets reflect new values
     within one frame.

7. **R-10.1.7** — The engine **SHALL** provide reactive data binding (one-way, two-way, computed)
   that automatically updates widget properties when game state changes.
   - **Rationale:** Reactive bindings eliminate manual polling for continuously changing state.
   - **Verification:** Bind a widget label to a model value, mutate the model. Assert text updates
     without explicit refresh.

## Navigation and Localization

8. **R-10.1.8** — The engine **SHALL** manage keyboard and gamepad focus traversal with tab order,
   directional navigation, focus groups, and focus trapping for modals.
   - **Rationale:** Complex UIs with multiple panels need structured focus management.
   - **Verification:** Open a modal, send tab and directional keys. Assert focus cycles within the
     modal and does not escape.

9. **R-10.1.9** — The engine **SHALL** provide per-widget localization hooks that switch text,
   images, and layout direction by locale at runtime, including RTL mirroring and pluralization.
   - **Rationale:** Switching locale must automatically re-layout for varying text lengths and RTL
     scripts.
   - **Verification:** Switch locale from English to Arabic. Assert layout direction mirrors and
     text updates.

## VR UI

10. **R-10.1.10** — The engine **SHALL** render a declarative widget tree as a world-space 3D panel
    with configurable resolution and physical size that receives ray-cast input.
    - **Rationale:** Diegetic interfaces and VR menus require the full widget framework in 3D space.
    - **Verification:** Render a world-space panel, ray-cast click a button. Assert click handler
      fires.

11. **R-10.1.11** — The engine **SHALL** support laser pointer, direct touch, gaze-and-dwell, and
    hand tracking pinch input for VR UI panels, with distance-aware text scaling and comfort
    clamping.
    - **Rationale:** VR users interact through different modalities; readability requires
      distance-aware scaling.
    - **Verification:** Activate each VR input mode. Assert a button on a world-space panel can be
      activated through each.

## Diffing and Animation

12. **R-10.1.12** — The engine **SHALL** compute the minimal diff between widget trees using a keyed
    reconciliation algorithm in O(n) for keyed lists, updating only changed nodes in place.
    - **Rationale:** O(n) diffing with in-place patching keeps UI updates within frame budget.
    - **Verification:** Diff a 500-widget keyed list with 10% changes. Assert under 1 ms and only
      changed nodes touched.

13. **R-10.1.13** — The engine **SHALL** animate widget properties (position, size, opacity, color,
    rotation, scale) using keyframed curves with configurable easing, supporting transition,
    looping, staggered, and interruptible modes.
    - **Rationale:** Smooth interruptible animations are essential for responsive UI feedback.
    - **Verification:** Trigger slide-in, interrupt mid-flight with new target. Assert smooth blend
      without discontinuity.

## Audio

14. **R-10.1.14** — The engine **SHALL** play configurable audio feedback for widget interactions
    through a dedicated UI mixer bus with per-widget and per-theme sound overrides.
    - **Rationale:** Audio feedback reinforces interactions and must be configurable per
      accessibility setting.
    - **Verification:** Click a button. Assert expected audio dispatched to UI mixer bus. Disable UI
      sounds. Assert no audio dispatched.

## Spring Animation

15. **R-10.1.13a** — The engine **SHALL** support spring-physics-based animation for widget
    properties with configurable stiffness, damping, and mass.
    - **Rationale:** Spring animations provide natural motion with overshoot and settle behavior
      that keyframed curves cannot easily replicate.
    - **Verification:** Animate a widget position with spring physics. Assert overshoot occurs with
      low damping. Assert critically damped spring settles without overshoot. Assert mass affects
      oscillation frequency.
