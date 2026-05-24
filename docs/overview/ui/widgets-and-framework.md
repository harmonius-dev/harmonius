# Widgets and Framework

UI widget library, layout systems, and input handling.

## What it covers

- Widget hierarchy: tree of UI elements (panels, buttons, text, images).
- Layout: anchors, alignment, margins, padding.
- Constraints: size and position relationships between widgets.
- Widget types: buttons, text fields, dropdowns, lists, sliders.
- Text rendering: font rasterization, multi-line text, rich text (colors, sizes).
- Input routing: dispatching keyboard and mouse events to focused widget.
- Focus management: keyboard tab order and current focus.
- State: enabled/disabled, hover, pressed widget states.
- Themes and styles: skinning widgets with colors, fonts, sizes.
- Data binding: synchronizing widget state with underlying data.

## Concepts

### Widget Hierarchy and Layout

UI organizes as a tree: root canvas contains panels; panels contain buttons and text. Each widget has
local position and size; layout systems compute final screen-space bounds. Anchors tie widgets to
screen edges (button anchored top-right stays at top-right when resizing). Constraints establish
relationships: "text width = parent width – 20 pixels".

### Widget Types and Interactions

Button widgets respond to clicks; text fields capture typed input; sliders track drag movements.
Dropdowns expand to show options; lists scroll through items. Each widget type has standard
interactions: button = click; text field = type; slider = drag. Custom widgets extend base widget
class with custom rendering and input handling.

### Text Rendering and Localization

Text renders using rasterized fonts (pre-rendered glyphs) or signed-distance fields (scalable
rendering). Multi-line text wraps at container width. Rich text supports inline formatting (bold,
colored spans). Localization loads text from translation tables; different languages render with
different fonts and line breaks.

### Input and Focus

Input events (key press, mouse move) route to the focused widget. Keyboard tab moves focus between
widgets in defined order. Mouse clicks set focus to clicked widget. Widgets consume events they
handle; unhandled events bubble to parent. This enables efficient event routing without global input
handling.

### Theming and Data Binding

Themes define colors, fonts, sizes applied to widget types. Changing theme instantly reskins all
widgets. Data binding synchronizes widgets with data: text field content syncs with model value;
slider position syncs with numeric parameter. Changes propagate bidirectionally: user modifies UI,
data updates; code updates data, UI refreshes.

## How it fits

- See [hud-and-2d.md](./hud-and-2d.md) for in-game HUD layout and gameplay-specific widgets.
- See [accessibility.md](./accessibility.md) for accessible widget design and interaction.
- See [../input/devices-and-actions.md](../input/devices-and-actions.md) for input event
  generation.
