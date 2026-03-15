# R-10.2 — Common Widget Requirements

## Text

### R-10.2.1 Rich Text and Text Shaping

The engine **SHALL** render rich text with inline formatting (bold, italic, color, size, font face),
embedded icons, and hyperlinks, using HarfBuzz-compatible shaping for complex scripts and the
Unicode BiDi algorithm for mixed LTR/RTL content.

- **Derived from:** [F-10.2.1](../../features/ui-2d/common-widgets.md)
- **Rationale:** Chat, quest descriptions, tooltips, and mail require rich text with inline item
  links, player names, emoji, and correct rendering of non-Latin scripts and bidirectional text.
- **Verification:** Unit test: render a string containing bold, italic, color spans, an embedded
  icon, and a hyperlink; verify each inline element produces the correct glyph runs and styling.
  Integration test: render Arabic and Hebrew text mixed with English and verify correct visual
  ordering per the Unicode BiDi algorithm. Verify ligature output for Devanagari and Arabic against
  reference glyph sequences from HarfBuzz.

### R-10.2.2 Text Input and Editing

The engine **SHALL** provide single-line and multi-line text input widgets supporting text
selection, clipboard operations (copy, cut, paste), undo/redo, and platform-native IME integration
for CJK language input without dropping characters under rapid typing.

- **Derived from:** [F-10.2.2](../../features/ui-2d/common-widgets.md)
- **Rationale:** Chat input must handle rapid typing during gameplay, and multi-line input is
  required for mail composition and guild message editing.
- **Verification:** Unit test: programmatically type, select, copy, cut, paste, undo, and redo in
  both single-line and multi-line widgets; verify buffer contents after each operation. Integration
  test: simulate IME composition events for CJK input and verify correct committed text. Stress
  test: enqueue 1,000 key events within a single frame and verify zero dropped characters.

## Buttons and Controls

### R-10.2.3 Buttons, Sliders, and Toggle Controls

The engine **SHALL** provide push buttons (icon + label variants), toggle buttons, checkboxes, radio
buttons, horizontal/vertical sliders (continuous and discrete), and spin boxes, all supporting
keyboard and gamepad interaction, disabled state, and animated state transitions for hover, press,
and release.

- **Derived from:** [F-10.2.3](../../features/ui-2d/common-widgets.md)
- **Rationale:** Standard interactive controls are foundational to every UI panel; animated
  transitions and multi-input-device support ensure a polished, accessible experience.
- **Verification:** Unit test: for each control type, simulate mouse click, keyboard activate, and
  gamepad activate; verify the control fires its value-changed callback with the correct value.
  Verify disabled controls reject all input. Integration test: drag a slider at 120 Hz input rate
  and verify the reported value tracks the pointer position without jitter (value delta per frame
  below a defined threshold).

### R-10.2.4 Combo Boxes and Dropdown Menus

The engine **SHALL** provide a dropdown selection widget that displays options in a popup overlay,
supports search filtering for large option lists, handles dynamic real-time option updates, and
supports grouped/categorized options.

- **Derived from:** [F-10.2.4](../../features/ui-2d/common-widgets.md)
- **Rationale:** Server selection lists, auction house category filters, and similar UI patterns
  require searchable, dynamically updating dropdowns with hundreds of entries.
- **Verification:** Unit test: open a dropdown with 500 options, type a filter string, and verify
  the visible option list contains only matching entries. Integration test: mutate the option list
  while the dropdown is open and verify the displayed list updates without crash or stale entries.
  Verify grouped options render with category headers and correct nesting.

## Scrolling and Lists

### R-10.2.5 Scroll Views and Virtualized List Views

The engine **SHALL** provide scrollable container widgets with inertial scrolling, scroll bars, and
overscroll feedback, and virtualized list views that render only the visible subset of items plus a
configurable buffer, supporting variable-height items, sticky headers, and pull-to-refresh.

- **Derived from:** [F-10.2.5](../../features/ui-2d/common-widgets.md)
- **Rationale:** Lists with thousands of entries (auction house results, guild rosters, chat
  history) must avoid proportional memory and layout cost by virtualizing off-screen items.
- **Verification:** Unit test: populate a virtualized list with 10,000 variable-height items and
  verify that the number of instantiated item widgets never exceeds the visible count plus the
  buffer size. Benchmark: scroll through the full list and verify frame time remains below 4 ms for
  layout. Verify sticky headers remain pinned at the scroll container's top edge during scrolling.

## Overlays and Popups

### R-10.2.6 Tooltips, Context Menus, and Modal Dialogs

The engine **SHALL** provide overlay widgets (tooltips, context menus, modal dialogs) that render
above the normal widget tree with managed z-ordering, dismiss on outside click or escape, and
support stacking multiple overlays simultaneously.

- **Derived from:** [F-10.2.6](../../features/ui-2d/common-widgets.md)
- **Rationale:** Item tooltips, right-click context menus, and confirmation dialogs are pervasive UI
  patterns that must layer correctly and dismiss predictably.
- **Verification:** Unit test: open a modal, then a context menu over it, then a tooltip over the
  context menu; verify z-order is tooltip > context menu > modal. Verify outside click dismisses the
  topmost overlay without affecting lower layers. Verify escape key dismisses overlays in
  top-to-bottom order. Verify modal dialogs block input to widgets beneath them.

### R-10.2.7 Drag and Drop

The engine **SHALL** provide a drag-and-drop system that displays a ghost preview of the dragged
item, highlights valid drop targets, supports split-stack operations via modifier keys, and handles
cross-panel drags between unrelated widget containers.

- **Derived from:** [F-10.2.7](../../features/ui-2d/common-widgets.md)
- **Rationale:** Inventory management, equipment, action bars, trade windows, and mail attachments
  all depend on drag-and-drop with visual feedback and cross-panel transfers.
- **Verification:** Unit test: initiate a drag from container A, hover over valid and invalid drop
  targets, and verify highlight state is correct for each. Drop on a valid target and verify the
  item transfers. Integration test: drag with a modifier key held and verify the split-stack dialog
  appears. Verify cross-panel drag from an inventory widget to a mail attachment widget completes
  successfully.

## Progress and Status

### R-10.2.8 Progress Bars and Loading Indicators

The engine **SHALL** provide determinate progress bars (linear and circular), indeterminate loading
spinners, and segmented progress indicators, supporting animated fill, color gradients, and label
overlays displaying percentage or value/max text.

- **Derived from:** [F-10.2.8](../../features/ui-2d/common-widgets.md)
- **Rationale:** Loading screens, crafting timers, experience bars, and reputation tracks all
  require visually clear progress indication with smooth animation.
- **Verification:** Unit test: set a determinate progress bar to 0%, 50%, and 100% and verify the
  fill region width or arc matches the expected proportion. Verify label overlay text displays the
  correct percentage string. Integration test: animate a progress bar from 0% to 100% over 60 frames
  and verify the fill interpolates smoothly with no visual jumps. Verify indeterminate spinners
  produce continuous rotation animation.
