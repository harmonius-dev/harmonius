# R-10.2 — Common Widget Requirements

## Text

1. **R-10.2.1** — The engine **SHALL** render rich text with inline formatting (bold, italic, color,
   size, font face), embedded icons, and hyperlinks, using HarfBuzz-compatible shaping for complex
   scripts and the Unicode BiDi algorithm for mixed LTR/RTL content.
   - **Rationale:** Chat, tooltips, and quest descriptions require rich text with correct non-Latin
     script rendering and bidirectional text.
   - **Verification:** Render a string with bold, italic, icon, and hyperlink. Verify correct glyph
     runs. Render Arabic mixed with English. Verify BiDi ordering.

2. **R-10.2.2** — The engine **SHALL** provide single-line and multi-line text input widgets with
   selection, clipboard, undo/redo, and platform-native IME for CJK input without dropping
   characters under rapid typing.
   - **Rationale:** Chat must handle rapid typing during gameplay; multi-line is needed for mail
     composition.
   - **Verification:** Type, select, copy, paste, undo, redo. Verify buffer contents. Simulate CJK
     IME and verify committed text. Enqueue 1000 keys in one frame. Assert zero dropped characters.

## Buttons and Controls

3. **R-10.2.3** — The engine **SHALL** provide buttons, toggle buttons, checkboxes, radio buttons,
   sliders (continuous and discrete), and spin boxes, all supporting keyboard and gamepad
   interaction, disabled state, and animated state transitions.
   - **Rationale:** Standard controls are foundational; animated transitions and multi-device
     support ensure a polished accessible experience.
   - **Verification:** For each control, simulate mouse, keyboard, and gamepad activation. Verify
     correct value-changed callbacks. Drag a slider at 120 Hz. Assert no value jitter.

4. **R-10.2.4** — The engine **SHALL** provide a dropdown widget with popup overlay, search
   filtering, dynamic real-time option updates, and grouped/categorized options.
   - **Rationale:** Server selection and auction filters require searchable dropdowns with hundreds
     of entries.
   - **Verification:** Open dropdown with 500 options. Type filter. Assert correct visible subset.
     Mutate options while open. Assert no crash or stale entries.

## Scrolling and Lists

5. **R-10.2.5** — The engine **SHALL** provide scrollable containers with inertial scrolling, scroll
   bars, overscroll feedback, and virtualized list views rendering only the visible subset plus a
   configurable buffer.
   - **Rationale:** Lists with thousands of entries must avoid proportional memory and layout cost.
   - **Verification:** Populate a list with 10000 variable-height items. Assert instantiated widgets
     never exceed visible count plus buffer. Assert frame time under 4 ms for layout.

## Overlays and Popups

6. **R-10.2.6** — The engine **SHALL** provide overlay widgets (tooltips, context menus, modals)
   with managed z-ordering, dismiss on outside click or escape, and stacking support.
   - **Rationale:** Item tooltips, context menus, and confirmations must layer correctly and dismiss
     predictably.
   - **Verification:** Open modal, context menu, tooltip stacked. Assert z-order is tooltip > menu >
     modal. Click outside. Assert topmost dismissed.

7. **R-10.2.7** — The engine **SHALL** provide a drag-and-drop system with ghost preview, valid drop
   target highlighting, split-stack via modifier keys, and cross-panel drags.
   - **Rationale:** Inventory, equipment, trade, and mail all depend on drag-and-drop with
     cross-panel transfers.
   - **Verification:** Drag from container A. Verify highlight state. Drop on valid target. Assert
     transfer. Drag with modifier. Assert split-stack.

## Progress and Status

8. **R-10.2.8** — The engine **SHALL** provide determinate progress bars (linear and circular),
   indeterminate spinners, and segmented indicators with animated fill, gradients, and label
   overlays.
   - **Rationale:** Loading, crafting, experience, and reputation all require clear progress
     indication.
   - **Verification:** Set bar to 0%, 50%, 100%. Assert fill matches proportion. Animate 0% to 100%
     over 60 frames. Assert smooth interpolation.

## Tab Bar and Tree View

9. **R-10.2.9** — The engine **SHALL** provide tab bar and tree view widgets with keyboard
   navigation, drag-to-reorder tabs, and collapsible tree node expand/collapse.
   - **Rationale:** Tab bars organize multi-panel UIs (inventory, skills, quests); tree views
     display hierarchical data (scene graphs, file browsers).
   - **Verification:** Create a tab bar with 5 tabs. Navigate with keyboard arrows. Drag to reorder.
     Assert order persists. Create a tree view with 3 levels. Expand and collapse nodes with
     keyboard. Assert correct visibility.

## Color Picker

10. **R-10.2.10** — The engine **SHALL** provide a color picker widget with hue/saturation wheel,
    brightness slider, hex/RGB input, and eyedropper.
    - **Rationale:** Character customization, material editing, and theme authoring require a
      full-featured color selection interface.
    - **Verification:** Open the color picker. Select a color via the wheel. Assert hex and RGB
      inputs update. Enter a hex value. Assert wheel position updates. Use the eyedropper on a known
      pixel. Assert correct color sampled.

## Drag-and-Drop State Machine

11. **R-10.2.7a** — The engine **SHALL** manage drag-and-drop via a state machine tracking pending,
    dragging, valid target, invalid target, drop, and cancel states with configurable threshold
    distance.
    - **Rationale:** A state machine ensures correct drag-and-drop behavior across complex
      multi-panel UI with clear state transitions.
    - **Verification:** Begin a drag below threshold distance. Assert state remains pending. Exceed
      threshold. Assert state transitions to dragging. Hover over valid target. Assert valid-target
      state. Drop. Assert drop state fires. Press escape during drag. Assert cancel state fires.
