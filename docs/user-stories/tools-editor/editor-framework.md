# User Stories -- 15.1 Editor Framework

## Stories

| ID          | Persona                    |
|-------------|----------------------------|
| US-15.1.1.1 | game designer (P-5)        |
| US-15.1.1.2 | level designer (P-6)       |
| US-15.1.2.1 | level designer (P-6)       |
| US-15.1.2.2 | technical artist (P-13)    |
| US-15.1.3.1 | game designer (P-5)        |
| US-15.1.3.2 | engine developer (P-26)    |
| US-15.1.4.1 | level designer (P-6)       |
| US-15.1.4.2 | game designer (P-5)        |
| US-15.1.5.1 | level designer (P-6)       |
| US-15.1.5.2 | game designer (P-5)        |
| US-15.1.6.1 | level designer (P-6)       |
| US-15.1.6.2 | game designer (P-5)        |
| US-15.1.7.1 | game designer (P-5)        |
| US-15.1.7.2 | technical artist (P-13)    |
| US-15.1.8.1 | extension developer (P-25) |
| US-15.1.8.2 | engine developer (P-26)    |
| US-15.1.9.1 | level designer (P-6)       |
| US-15.1.9.2 | game designer (P-5)        |
| US-15.1.10.1 | game designer (P-5)       |
| US-15.1.11.1 | environment artist (P-8)  |

1. **US-15.1.1.1** — **As a** game designer (P-5), **I want** to drag, dock, split, and tab panels
   into custom layouts, **so that** my workspace matches my current task.

2. **US-15.1.1.2** — **As a** level designer (P-6), **I want** to save and switch between named
   layout profiles, **so that** I can move between level design and lighting review instantly.

3. **US-15.1.2.1** — **As a** level designer (P-6), **I want** multiple simultaneous 3D viewports
   with independent cameras, **so that** I can see top-down and perspective views together.

4. **US-15.1.2.2** — **As a** technical artist (P-13), **I want** each viewport to have independent
   render settings and debug overlays, **so that** I can compare wireframe and lit views.

5. **US-15.1.3.1** — **As a** game designer (P-5), **I want** every editor action captured as a
   reversible command in an undo/redo stack, **so that** I can safely experiment with changes.

6. **US-15.1.3.2** — **As a** engine developer (P-26), **I want** transaction grouping to batch
   related operations into a single undo step, **so that** multi-entity moves revert atomically.

7. **US-15.1.4.1** — **As a** level designer (P-6), **I want** click, marquee, and lasso selection
   with additive and subtractive modifiers, **so that** I can select complex object groups quickly.

8. **US-15.1.4.2** — **As a** game designer (P-5), **I want** to save named selection sets,
   **so that** I can reselect frequently used groups without re-picking.

9. **US-15.1.5.1** — **As a** level designer (P-6), **I want** translate, rotate, and scale gizmos
   with axis and plane constraints, **so that** I can precisely position entities.

10. **US-15.1.5.2** — **As a** game designer (P-5), **I want** configurable snap increments per
    axis, **so that** objects align to my gameplay grid.

11. **US-15.1.6.1** — **As a** level designer (P-6), **I want** bounding box, ruler, and protractor
    overlays in the viewport, **so that** I can verify corridor widths and jump distances.

12. **US-15.1.6.2** — **As a** game designer (P-5), **I want** area measurement gizmos, **so that**
    I can confirm line-of-sight clearances for gameplay tuning.

13. **US-15.1.7.1** — **As a** game designer (P-5), **I want** centralized preferences for bindings,
    themes, and grid settings stored as versioned JSON, **so that** team defaults propagate through
    version control.

14. **US-15.1.7.2** — **As a** technical artist (P-13), **I want** per-user preference overrides
    that stay local, **so that** my personal tweaks do not affect teammates.

15. **US-15.1.8.1** — **As a** extension developer (P-25), **I want** a stable plugin API for custom
    panels, gizmos, and importers, **so that** I can extend the editor without modifying its core.

16. **US-15.1.8.2** — **As a** engine developer (P-26), **I want** plugins hot-reloaded during
    development, **so that** I can iterate on editor tools without restarting.

17. **US-15.1.9.1** — **As a** level designer (P-6), **I want** to edit scenes inside a VR headset
    with motion controllers, **so that** I can place and scale objects at true player scale.

18. **US-15.1.9.2** — **As a** game designer (P-5), **I want** VR mode changes visible on the
    desktop monitor in real time, **so that** team members can follow along without a headset.

19. **US-15.1.10.1** — **As a** game designer (P-5), **I want** to navigate back to a previous undo
    branch that I abandoned earlier in a non-linear undo tree, **so that** I can recover an approach
    I previously discarded without losing my current branch.

20. **US-15.1.11.1** — **As a** environment artist (P-8), **I want** to float editor panels onto a
    second monitor with per-monitor DPI scaling and independent dock trees, **so that** I can
    maximize my viewport on the primary display while keeping inspectors on the secondary one.
