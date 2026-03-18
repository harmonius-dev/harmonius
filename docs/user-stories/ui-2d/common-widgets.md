# User Stories — 10.2 Common Widgets

## Text

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-10.2.1 | Player (P-23)           | F-10.2.1 | R-10.2.1     |
| US-10.2.2 | Engine developer (P-26) | F-10.2.1 | R-10.2.1     |
| US-10.2.3 | QA engineer (P-19)      | F-10.2.1 | R-10.2.1     |
| US-10.2.4 | Player (P-23)           | F-10.2.2 | R-10.2.2     |
| US-10.2.5 | Engine developer (P-26) | F-10.2.2 | R-10.2.2     |

1. **US-10.2.1** — I want rich text with inline formatting, embedded icons, hyperlinks, and proper
   shaping for complex scripts like Arabic and Thai, so that chat messages, quest descriptions, and
   tooltips render correctly in any language.
   - **Acceptance:** Inline bold, italic, color, size, font face formatting; embedded icons and
     hyperlinks in text flow; HarfBuzz-compatible shaping for complex scripts; bidirectional text
     for mixed LTR/RTL content
2. **US-10.2.2** — I want to implement a rich text renderer using a bundled shaping library for
   consistent cross-platform behavior, so that text renders identically on Windows, macOS, and Linux
   without OS-specific shaper differences.
   - **Acceptance:** Bundled shaping library, not OS-provided shapers; Unicode BiDi algorithm for
     bidirectional text; ligature handling for all supported scripts
3. **US-10.2.3** — I want to verify that rich text with complex scripts, bidirectional content, and
   inline formatting renders identically on all platforms, so that players see the same text
   regardless of operating system.
   - **Acceptance:** Pixel-identical text rendering across Windows, macOS, Linux; complex script
     shaping matches reference renders; BiDi algorithm produces correct visual ordering
4. **US-10.2.4** — I want text input with IME support for CJK languages, clipboard operations, and
   undo/redo, so that I can compose chat messages in any language during gameplay without dropping
   characters.
   - **Acceptance:** Single-line and multi-line text input; IME composition for CJK languages; copy,
     cut, paste, undo, redo operations; no dropped characters during rapid typing
5. **US-10.2.5** — I want to implement IME integration using platform-specific APIs (TSM on macOS,
   IMM32/TSF on Windows, IBus/Fcitx on Linux), so that CJK text input works natively on each
   platform.
   - **Acceptance:** macOS IME via Text Services Manager; Windows IME via IMM32/TSF APIs; Linux IME
     via IBus/Fcitx

## Buttons and Controls

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-10.2.6  | Player (P-23)        | F-10.2.3 | R-10.2.3     |
| US-10.2.7  | Artist (P-8)         | F-10.2.3 | R-10.2.3     |
| US-10.2.8  | Engine tester (P-27) | F-10.2.3 | R-10.2.3     |
| US-10.2.9  | Player (P-23)        | F-10.2.4 | R-10.2.4     |
| US-10.2.10 | Developer (P-15)     | F-10.2.4 | R-10.2.4     |

1. **US-10.2.6** — I want responsive buttons, sliders, checkboxes, radio buttons, and toggle
   controls with animated state transitions, so that I can adjust settings and interact with UI
   controls smoothly with any input device.
   - **Acceptance:** Push buttons, toggle buttons, checkboxes, radio buttons; horizontal and
     vertical sliders (continuous and discrete); keyboard and gamepad interaction for all controls;
     animated hover, press, release state transitions
2. **US-10.2.7** — I want to style buttons, sliders, and toggles with custom graphics for each
   interaction state (normal, hovered, pressed, disabled), so that UI controls match the game's
   visual identity.
   - **Acceptance:** Custom graphics per interaction state; smooth animated transitions between
     states; slider handle and track fully styleable
3. **US-10.2.8** — I want to verify that sliders handle high-frequency dragging input without jitter
   or value skipping, so that audio volume, camera sensitivity, and other slider-driven settings
   feel responsive and precise.
   - **Acceptance:** No value jitter during rapid mouse/stick dragging; discrete sliders snap to
     valid values without overshoot; slider responds identically to mouse, keyboard, and gamepad
4. **US-10.2.9** — I want searchable dropdown menus that filter options as I type, so that I can
   quickly find the right server, item category, or setting from lists with hundreds of entries.
   - **Acceptance:** Search filtering narrows options as user types; dynamic option lists update in
     real time; options support grouping and categorization
5. **US-10.2.10** — I want to create dropdown widgets with dynamic option lists that update in real
   time and support search filtering, so that server selection, auction house filters, and category
   pickers handle large datasets efficiently.
   - **Acceptance:** Popup overlay displays option list; search box filters options by substring
     match; options update dynamically from bound data source

## Scrolling and Lists

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-10.2.11 | Player (P-23)           | F-10.2.5 | R-10.2.5     |
| US-10.2.12 | Engine developer (P-26) | F-10.2.5 | R-10.2.5     |

1. **US-10.2.11** — I want virtualized list views with inertial scrolling, scroll bars, and
   overscroll feedback that handle thousands of entries without lag, so that browsing auction house
   results, guild rosters, and chat history feels responsive.
   - **Acceptance:** Only visible items plus buffer rendered; inertial scrolling with overscroll
     feedback; variable-height items and sticky headers supported; 10,000+ entry list scrolls at
     60fps
2. **US-10.2.12** — I want to implement scroll views that virtualize rendering to only the visible
   subset plus a buffer, so that lists with thousands of entries have constant memory and layout
   cost regardless of total item count.
   - **Acceptance:** Rendering virtualized to visible subset plus buffer; variable-height items
     measure correctly; pull-to-refresh pattern supported; mobile uses smaller virtualization
     buffers

## Overlays and Popups

| ID         | Persona          | Features | Requirements |
|------------|------------------|----------|--------------|
| US-10.2.13 | Player (P-23)    | F-10.2.6 | R-10.2.6     |
| US-10.2.14 | Designer (P-5)   | F-10.2.6 | R-10.2.6     |
| US-10.2.15 | Player (P-23)    | F-10.2.7 | R-10.2.7     |
| US-10.2.16 | Developer (P-15) | F-10.2.7 | R-10.2.7     |

1. **US-10.2.13** — I want hover-triggered tooltips showing item stats and comparisons, right-click
   context menus with nested submenus, and modal confirmation dialogs, so that I can inspect,
   compare, and manage items efficiently.
   - **Acceptance:** Tooltips show rich content including stat comparisons; context menus support
     nested submenus; modal dialogs block interaction behind them; overlays stack correctly (tooltip
     over context menu over modal)
2. **US-10.2.14** — I want to design tooltip layouts that display item stats, set bonuses, and
   equipped-vs-inspected comparisons, so that players can make informed equipment decisions at a
   glance.
   - **Acceptance:** Tooltip layout supports structured stat display; side-by-side comparison of two
     items; tooltips dismiss on outside click or Escape
3. **US-10.2.15** — I want to drag and drop items between inventory slots, equipment slots, action
   bars, trade windows, and mail attachments with ghost preview and valid drop target highlighting,
   so that I can manage my items intuitively across all container types.
   - **Acceptance:** Ghost preview follows cursor during drag; valid drop targets highlight during
     drag; cross-panel drags supported (inventory to mail); split-stack via modifier key during drag
4. **US-10.2.16** — I want a drag-and-drop system that works across different container panel types
   with split-stack operations, so that items move correctly between inventory, bank, vendor, and
   trade windows.
   - **Acceptance:** Drag source and drop target communicate across panels; stack splitting divides
     item stacks by quantity; invalid drops rejected with visual feedback

## Progress and Status

| ID         | Persona       | Features | Requirements |
|------------|---------------|----------|--------------|
| US-10.2.17 | Player (P-23) | F-10.2.8 | R-10.2.8     |
| US-10.2.18 | Artist (P-8)  | F-10.2.8 | R-10.2.8     |

1. **US-10.2.17** — I want determinate progress bars, circular indicators, and loading spinners that
   show accurate progress for loading screens, crafting timers, and experience tracking, so that I
   know how long each operation will take.
   - **Acceptance:** Linear and circular determinate progress bars; indeterminate loading spinners
     for unknown durations; animated fill with color gradients; label overlays showing percentage or
     value/max
2. **US-10.2.18** — I want to customize progress bar fill colors, gradients, segment styles, and
   label fonts, so that experience bars, reputation tracks, and crafting timers match the game's
   visual design language.
   - **Acceptance:** Custom fill colors and gradients per bar type; segmented progress indicator
     support; animated fill transitions
