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

10. **R-15.1.10** — The engine **SHALL** implement a non-linear undo tree where branching edits
    preserve all history branches, with LCA-based navigation to reach any historical state.
    - **Rationale:** A linear undo stack permanently loses branches; a tree preserves all approaches
      for recovery.
    - **Verification:** Make edits A-B-C, undo to A, make edits D-E, then navigate back to state C
      and verify data integrity.

11. **R-15.1.11** — The engine **SHALL** isolate editor state in a separate ECS world synchronized
    with the game world via an EventBridge, preventing editor state from leaking into game builds.
    - **Rationale:** Editor-only state (selection, gizmos, undo) must never appear in shipped game
      data.
    - **Verification:** Run a game build and verify no editor-only components or entities are
      present in the ECS world.

12. **R-15.1.12** — The engine **SHALL** support editor extension via logic graphs for custom
    panels, gizmos, inspectors, importers, validation rules, and automation scripts.
    - **Rationale:** Logic graph-based extension uses the same no-code system as gameplay authoring.
    - **Verification:** Author a logic graph that adds a custom inspector widget and verify it
      appears when the target component is selected.

13. **R-15.1.13** — The engine **SHALL** skip UI render passes when no widget is dirty and no input
    occurred, and shall repaint only dirty widget regions for partial redraw.
    - **Rationale:** Dirty-based rendering minimizes CPU and GPU usage when the editor is idle.
    - **Verification:** Leave the editor idle for 5 seconds and verify zero UI render passes occur.
      Modify one widget and verify only its region repaints.

14. **R-15.1.14** — The engine **SHALL** generate a screen-space occlusion mask from opaque UI
    panels and skip 3D rendering of fully covered viewport pixels.
    - **Rationale:** Rendering behind opaque panels wastes GPU cycles.
    - **Verification:** Cover 50% of the viewport with opaque panels and verify GPU tile work drops
      proportionally.

15. **R-15.1.15** — The engine **SHALL** support frosted-glass transparent panels with configurable
    Gaussian or Kawase background blur at reduced resolution.
    - **Rationale:** Transparent blur panels provide visual context while maintaining readability.
    - **Verification:** Enable a frosted-glass panel and verify the background blurs at the
      configured resolution without artifacts.

16. **R-15.1.16** — The engine **SHALL** support multi-monitor layouts with per-monitor DPI scaling,
    floating panel snap-to-edge, and independent dock trees when panels span monitors.
    - **Rationale:** Multi-monitor setups are standard for professional content creation workflows.
    - **Verification:** Float a panel onto a second monitor with different DPI and verify correct
      scaling and snap behavior.

17. **R-15.1.17** — The engine **SHALL** provide predefined workflow layout profiles for common
    editing tasks (animation, level design, VFX, logic graph, 2D, debug, review) and support custom
    user-defined layouts.
    - **Rationale:** Predefined layouts reduce setup time for common workflows.
    - **Verification:** Switch to the animation layout profile and verify all animation-related
      panels are arranged correctly.

18. **R-15.1.18** — The engine **SHALL** reduce editor rendering to event-driven mode when the
    editor is unfocused or idle, consuming minimal CPU and GPU power.
    - **Rationale:** Idle power reduction extends laptop battery life and reduces fan noise during
      breaks.
    - **Verification:** Unfocus the editor, measure CPU and GPU usage, and verify both drop to
      near-zero within 2 seconds.
