# R-15.1 -- Editor Framework Requirements

## Requirements

1. **R-15.1.1** — The engine **SHALL** provide a dockable panel system with drag, drop, split, tab,
   and float operations and persistent named layout profiles.
   - **Rationale:** Different roles need different workspace arrangements; profiles enable instant
     switching.
   - **Verification:** Create two layout profiles, switch between them, and verify all panel
     positions restore correctly.

2. **R-15.1.2** — The engine **SHALL** support multiple simultaneous 3D viewports, each with an
   independent camera, render settings, and debug overlays.
   - **Rationale:** Parallel views enable top-down and perspective editing simultaneously.
   - **Verification:** Open three viewports with different cameras and overlays; verify each renders
     independently.

3. **R-15.1.3** — The engine **SHALL** implement a command-pattern undo/redo stack where every
   editor action is a serializable, reversible command with transaction grouping support.
   - **Rationale:** Safe experimentation requires reliable undo; transaction grouping keeps
     multi-step actions atomic.
   - **Verification:** Perform a grouped multi-entity move, undo once, and verify all entities
     revert together.

4. **R-15.1.4** — The engine **SHALL** provide a unified selection model supporting click, marquee,
   lasso, and programmatic modes with additive and subtractive modifiers and named selection sets.
   - **Rationale:** Diverse selection modes speed up workflows on large scenes with thousands of
     entities.
   - **Verification:** Create a named selection set, deselect all, restore the set, and verify the
     correct entities are selected.

5. **R-15.1.5** — The engine **SHALL** render interactive translate, rotate, and scale gizmos with
   local, world, and custom reference frames, axis and plane constraints, and configurable snap
   increments.
   - **Rationale:** Precision placement is fundamental to level design and gameplay tuning.
   - **Verification:** Enable 1-meter grid snap, move an entity, and verify the resulting position
     is grid-aligned.

6. **R-15.1.6** — The engine **SHALL** display bounding boxes, distance rulers, angle protractors,
   and area measurement gizmos in the viewport.
   - **Rationale:** Designers must verify gameplay metrics such as corridor widths and jump
     distances visually.
   - **Verification:** Place two objects 5 m apart, activate the ruler gizmo, and verify the
     displayed distance is 5 m.

7. **R-15.1.7** — The engine **SHALL** provide a centralized preference system with versioned JSON
   storage, team-wide defaults via version control, and per-user local overrides.
   - **Rationale:** Shared defaults ensure consistency; local overrides respect individual
     preferences.
   - **Verification:** Set a team default grid size, override it locally, and verify the override
     takes precedence.

8. **R-15.1.8** — The engine **SHALL** expose a stable plugin API for custom panels, gizmos,
   importers, context menus, and toolbar buttons with hot-reload support during development.
   - **Rationale:** Extensibility enables studio-specific tooling without forking the editor.
   - **Verification:** Load a plugin adding a custom panel, modify its source, trigger hot-reload,
     and verify the panel updates.

9. **R-15.1.9** — The engine **SHALL** provide a VR editing mode with stereoscopic rendering, head
   tracking, and 6-DoF motion controller input that shares world state with the desktop editor in
   real time.
   - **Rationale:** VR editing lets designers experience levels at true player scale.
   - **Verification:** Enter VR mode, place an entity with motion controllers, exit VR, and verify
     the entity appears on the desktop viewport.
