# User Stories — 10.2 Common Widgets

## US-10.2.1 Read Rich Text With Inline Formatting and Complex Scripts

**As a** player (P-23), **I want** rich text with inline formatting, embedded icons,
hyperlinks, and proper shaping for complex scripts like Arabic and Thai, **so that** chat
messages, quest descriptions, and tooltips render correctly in any language.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Inline bold, italic, color, size, font face formatting | F-10.2.1 | R-10.2.1 |
| Embedded icons and hyperlinks in text flow | F-10.2.1 | R-10.2.1 |
| HarfBuzz-compatible shaping for complex scripts | F-10.2.1 | R-10.2.1 |
| Bidirectional text for mixed LTR/RTL content | F-10.2.1 | R-10.2.1 |

## US-10.2.2 Implement Rich Text Rendering With Cross-Platform Shaping

**As an** engine developer (P-26), **I want** to implement a rich text renderer using a
bundled shaping library for consistent cross-platform behavior, **so that** text renders
identically on Windows, macOS, and Linux without OS-specific shaper differences.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Bundled shaping library, not OS-provided shapers | F-10.2.1 | R-10.2.1 |
| Unicode BiDi algorithm for bidirectional text | F-10.2.1 | R-10.2.1 |
| Ligature handling for all supported scripts | F-10.2.1 | R-10.2.1 |

## US-10.2.3 Verify Rich Text Renders Identically Across Platforms

**As a** QA engineer (P-19), **I want** to verify that rich text with complex scripts,
bidirectional content, and inline formatting renders identically on all platforms, **so that**
players see the same text regardless of operating system.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Pixel-identical text rendering across Windows, macOS, Linux | F-10.2.1 | R-10.2.1 |
| Complex script shaping matches reference renders | F-10.2.1 | R-10.2.1 |
| BiDi algorithm produces correct visual ordering | F-10.2.1 | R-10.2.1 |

## US-10.2.4 Type Chat Messages With IME and Clipboard Support

**As a** player (P-23), **I want** text input with IME support for CJK languages, clipboard
operations, and undo/redo, **so that** I can compose chat messages in any language during
gameplay without dropping characters.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Single-line and multi-line text input | F-10.2.2 | R-10.2.2 |
| IME composition for CJK languages | F-10.2.2 | R-10.2.2 |
| Copy, cut, paste, undo, redo operations | F-10.2.2 | R-10.2.2 |
| No dropped characters during rapid typing | F-10.2.2 | R-10.2.2 |

## US-10.2.5 Implement Platform-Specific IME Integration

**As an** engine developer (P-26), **I want** to implement IME integration using
platform-specific APIs (TSM on macOS, IMM32/TSF on Windows, IBus/Fcitx on Linux), **so that**
CJK text input works natively on each platform.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| macOS IME via Text Services Manager | F-10.2.2 | R-10.2.2 |
| Windows IME via IMM32/TSF APIs | F-10.2.2 | R-10.2.2 |
| Linux IME via IBus/Fcitx | F-10.2.2 | R-10.2.2 |

## US-10.2.6 Interact With Buttons, Sliders, and Toggles

**As a** player (P-23), **I want** responsive buttons, sliders, checkboxes, radio buttons,
and toggle controls with animated state transitions, **so that** I can adjust settings and
interact with UI controls smoothly with any input device.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Push buttons, toggle buttons, checkboxes, radio buttons | F-10.2.3 | R-10.2.3 |
| Horizontal and vertical sliders (continuous and discrete) | F-10.2.3 | R-10.2.3 |
| Keyboard and gamepad interaction for all controls | F-10.2.3 | R-10.2.3 |
| Animated hover, press, release state transitions | F-10.2.3 | R-10.2.3 |

## US-10.2.7 Design Custom Button and Slider Styles

**As an** artist (P-8), **I want** to style buttons, sliders, and toggles with custom
graphics for each interaction state (normal, hovered, pressed, disabled), **so that** UI
controls match the game's visual identity.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Custom graphics per interaction state | F-10.2.3 | R-10.2.3 |
| Smooth animated transitions between states | F-10.2.3 | R-10.2.3 |
| Slider handle and track fully styleable | F-10.2.3 | R-10.2.3 |

## US-10.2.8 Verify Slider Input Handles High-Frequency Dragging

**As an** engine tester (P-27), **I want** to verify that sliders handle high-frequency
dragging input without jitter or value skipping, **so that** audio volume, camera
sensitivity, and other slider-driven settings feel responsive and precise.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| No value jitter during rapid mouse/stick dragging | F-10.2.3 | R-10.2.3 |
| Discrete sliders snap to valid values without overshoot | F-10.2.3 | R-10.2.3 |
| Slider responds identically to mouse, keyboard, and gamepad | F-10.2.3 | R-10.2.3 |

## US-10.2.9 Filter Long Option Lists in Dropdown Menus

**As a** player (P-23), **I want** searchable dropdown menus that filter options as I type,
**so that** I can quickly find the right server, item category, or setting from lists with
hundreds of entries.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Search filtering narrows options as user types | F-10.2.4 | R-10.2.4 |
| Dynamic option lists update in real time | F-10.2.4 | R-10.2.4 |
| Options support grouping and categorization | F-10.2.4 | R-10.2.4 |

## US-10.2.10 Implement Searchable Dropdowns With Dynamic Options

**As a** developer (P-15), **I want** to create dropdown widgets with dynamic option lists
that update in real time and support search filtering, **so that** server selection,
auction house filters, and category pickers handle large datasets efficiently.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Popup overlay displays option list | F-10.2.4 | R-10.2.4 |
| Search box filters options by substring match | F-10.2.4 | R-10.2.4 |
| Options update dynamically from bound data source | F-10.2.4 | R-10.2.4 |

## US-10.2.11 Scroll Through Thousands of List Items Smoothly

**As a** player (P-23), **I want** virtualized list views with inertial scrolling, scroll
bars, and overscroll feedback that handle thousands of entries without lag, **so that**
browsing auction house results, guild rosters, and chat history feels responsive.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Only visible items plus buffer rendered | F-10.2.5 | R-10.2.5 |
| Inertial scrolling with overscroll feedback | F-10.2.5 | R-10.2.5 |
| Variable-height items and sticky headers supported | F-10.2.5 | R-10.2.5 |
| 10,000+ entry list scrolls at 60fps | F-10.2.5 | R-10.2.5 |

## US-10.2.12 Implement Virtualized List Views With Variable-Height Items

**As an** engine developer (P-26), **I want** to implement scroll views that virtualize
rendering to only the visible subset plus a buffer, **so that** lists with thousands of
entries have constant memory and layout cost regardless of total item count.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Rendering virtualized to visible subset plus buffer | F-10.2.5 | R-10.2.5 |
| Variable-height items measure correctly | F-10.2.5 | R-10.2.5 |
| Pull-to-refresh pattern supported | F-10.2.5 | R-10.2.5 |
| Mobile uses smaller virtualization buffers | F-10.2.5 | R-10.2.5 |

## US-10.2.13 Inspect Items Via Tooltips and Right-Click Context Menus

**As a** player (P-23), **I want** hover-triggered tooltips showing item stats and
comparisons, right-click context menus with nested submenus, and modal confirmation
dialogs, **so that** I can inspect, compare, and manage items efficiently.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Tooltips show rich content including stat comparisons | F-10.2.6 | R-10.2.6 |
| Context menus support nested submenus | F-10.2.6 | R-10.2.6 |
| Modal dialogs block interaction behind them | F-10.2.6 | R-10.2.6 |
| Overlays stack correctly (tooltip over context menu over modal) | F-10.2.6 | R-10.2.6 |

## US-10.2.14 Design Tooltip Layouts for Item Stat Comparisons

**As a** designer (P-5), **I want** to design tooltip layouts that display item stats,
set bonuses, and equipped-vs-inspected comparisons, **so that** players can make informed
equipment decisions at a glance.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Tooltip layout supports structured stat display | F-10.2.6 | R-10.2.6 |
| Side-by-side comparison of two items | F-10.2.6 | R-10.2.6 |
| Tooltips dismiss on outside click or Escape | F-10.2.6 | R-10.2.6 |

## US-10.2.15 Drag Items Between Inventory, Equipment, and Trade Windows

**As a** player (P-23), **I want** to drag and drop items between inventory slots,
equipment slots, action bars, trade windows, and mail attachments with ghost preview and
valid drop target highlighting, **so that** I can manage my items intuitively across all
container types.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Ghost preview follows cursor during drag | F-10.2.7 | R-10.2.7 |
| Valid drop targets highlight during drag | F-10.2.7 | R-10.2.7 |
| Cross-panel drags supported (inventory to mail) | F-10.2.7 | R-10.2.7 |
| Split-stack via modifier key during drag | F-10.2.7 | R-10.2.7 |

## US-10.2.16 Implement Cross-Panel Drag and Drop With Stack Splitting

**As a** developer (P-15), **I want** a drag-and-drop system that works across different
container panel types with split-stack operations, **so that** items move correctly between
inventory, bank, vendor, and trade windows.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Drag source and drop target communicate across panels | F-10.2.7 | R-10.2.7 |
| Stack splitting divides item stacks by quantity | F-10.2.7 | R-10.2.7 |
| Invalid drops rejected with visual feedback | F-10.2.7 | R-10.2.7 |

## US-10.2.17 See Accurate Progress for Loading, Crafting, and Experience

**As a** player (P-23), **I want** determinate progress bars, circular indicators, and
loading spinners that show accurate progress for loading screens, crafting timers, and
experience tracking, **so that** I know how long each operation will take.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Linear and circular determinate progress bars | F-10.2.8 | R-10.2.8 |
| Indeterminate loading spinners for unknown durations | F-10.2.8 | R-10.2.8 |
| Animated fill with color gradients | F-10.2.8 | R-10.2.8 |
| Label overlays showing percentage or value/max | F-10.2.8 | R-10.2.8 |

## US-10.2.18 Style Progress Bars to Match Game Visual Identity

**As an** artist (P-8), **I want** to customize progress bar fill colors, gradients,
segment styles, and label fonts, **so that** experience bars, reputation tracks, and
crafting timers match the game's visual design language.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Custom fill colors and gradients per bar type | F-10.2.8 | R-10.2.8 |
| Segmented progress indicator support | F-10.2.8 | R-10.2.8 |
| Animated fill transitions | F-10.2.8 | R-10.2.8 |
