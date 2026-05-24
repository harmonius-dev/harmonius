# UI

Widgets, layouts, and accessibility for user interfaces.

## Topics

- [widgets-and-framework](./widgets-and-framework.md) — buttons, text, lists, and composable
  UI elements.
- [hud-and-2d](./hud-and-2d.md) — HUD layout, 2D game UI, and screen-space rendering.
- [accessibility](./accessibility.md) — screen readers, colorblind modes, and input remapping.

## Key takeaways

- Widget hierarchy (tree of UI elements) with anchors and constraints enables responsive layout
  scaling to any resolution without manual repositioning.
- Data binding (two-way synchronization between widgets and data) reduces boilerplate and keeps UI
  state coherent with underlying model.
- Theme system (centralized colors, fonts, sizes) enables instant reskinning and maintains visual
  consistency across all widgets.
- Focus management (tab order, keyboard navigation) enables full UI control via keyboard for
  accessibility.
- Colorblind modes (simulating deuteranopia, protanopia, tritanopia) and alternative iconography
  ensure information is conveyed without color alone.

## Integration risks

- Widget anchor and constraint combinations can produce unsolvable layouts (conflicting constraints);
  layout solver must detect and report errors clearly. See [widgets-and-framework.md](./widgets-and-framework.md)
  for constraint validation.
- Data binding bidirectional updates can cause feedback loops (data changes UI, UI changes data in
  same frame); update guards necessary. See [widgets-and-framework.md](./widgets-and-framework.md)
  for circular dependency prevention.
- Theme changes mid-game require all widgets to re-render; not caching reskins causes frame drops.
  See [widgets-and-framework.md](./widgets-and-framework.md) for theme change scheduling.
- Colorblind mode must be toggled without restarting game; runtime palette swapping necessary. See
  [accessibility.md](./accessibility.md) for palette architecture.
- Screen reader text must be non-empty and concise; overly verbose labels cause player fatigue. See
  [accessibility.md](./accessibility.md) for label best practices.
