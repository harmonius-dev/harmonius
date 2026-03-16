# 10.2 — Common Widgets

## Text

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.2.1 | Rich Text and Text Shaping | Render rich text with inline formatting (bold, italic, color, size, font face), embedded icons, and hyperlinks. Text shaping uses HarfBuzz-compatible shaping for complex scripts (Arabic, Thai, Devanagari) and proper ligature handling. Supports bidirectional text (Unicode BiDi algorithm) for mixed LTR/RTL content. MMO chat, quest descriptions, tooltips, and mail all require rich text with inline item links, player names, and emoji. | R-10.2.1 | F-10.1.1, F-10.4.2 | Text shaping behavior must be consistent across platforms; use a bundled shaping library rather than OS-provided shapers to avoid rendering differences. |
| F-10.2.2 | Text Input and Editing | Provide single-line and multi-line text input widgets with selection, clipboard (copy/cut/paste), undo/redo, and IME (Input Method Editor) support for CJK languages. Chat input must handle rapid typing during gameplay without dropping characters. Multi-line input supports the mail composition and guild message-of-the-day editing use cases. | R-10.2.2 | F-10.2.1, F-10.1.7 | IME integration requires platform-specific APIs (TSM on macOS, IMM32/TSF on Windows, IBus/Fcitx on Linux). |

## Buttons and Controls

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.2.3 | Buttons, Sliders, and Toggle Controls | Standard interactive controls: push buttons (with icon + label variants), toggle buttons, checkboxes, radio buttons, horizontal/vertical sliders (continuous and discrete), and spin boxes. All controls support keyboard and gamepad interaction, disabled state, and animated state transitions (hover, press, release). Slider widgets must handle high-frequency dragging input without jitter. | R-10.2.3 | F-10.1.1, F-10.1.5, F-10.1.7 | None |
| F-10.2.4 | Combo Boxes and Dropdown Menus | Dropdown selection widget displaying a list of options in a popup overlay. Supports search filtering for long option lists (e.g., server selection with hundreds of servers, item category filters in the auction house). Dropdowns must handle dynamic option lists that update in real time and support grouping/categorization of options. | R-10.2.4 | F-10.2.3, F-10.1.1 | None |

## Scrolling and Lists

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.2.5 | Scroll Views and Virtualized List Views | Scrollable container widget with inertial scrolling, scroll bars, and overscroll feedback. Virtualized list views render only the visible subset of items plus a small buffer, enabling lists with thousands of entries (auction house results, guild rosters, chat history) without proportional memory or layout cost. Supports variable-height items, sticky headers, and pull-to-refresh patterns. | R-10.2.5 | F-10.1.1, F-10.1.2, F-10.1.3 | Mobile uses smaller virtualization buffers (fewer off-screen items pre-rendered) to reduce memory usage on constrained devices. |

## Overlays and Popups

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.2.6 | Tooltips, Context Menus, and Modal Dialogs | Overlay widgets that appear above the normal widget tree: tooltips (hover-triggered, with rich content including item stats and comparisons), context menus (right-click with nested submenus), and modal dialogs (confirmation prompts, trade windows, loot rolls). Overlays manage their own z-ordering, dismiss on outside click/escape, and support stacking (tooltip over context menu over modal). | R-10.2.6 | F-10.1.1, F-10.1.7 | None |
| F-10.2.7 | Drag and Drop | Drag-and-drop system for moving items between containers (inventory slots, equipment slots, action bars, trade windows, mail attachments, bank tabs). The drag operation displays a ghost preview of the dragged item, highlights valid drop targets, and supports split-stack operations (drag with modifier key to split a stack). Must handle cross-panel drags (e.g., dragging an item from inventory to the mail composition window). | R-10.2.7 | F-10.1.1, F-10.2.5 | None |

## Progress and Status

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|---|---|---|---|---|---|
| F-10.2.8 | Progress Bars and Loading Indicators | Determinate progress bars (linear and circular), indeterminate loading spinners, and segmented progress indicators. Support animated fill, color gradients, and label overlays showing percentage or value/max text. Used for loading screens, download progress, crafting timers, experience bars, and reputation tracks. | R-10.2.8 | F-10.1.1, F-10.1.5 | None |
