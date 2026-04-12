# User Stories -- 15.20 Editor Plugin Architecture

## Stories

| ID           | Persona                    |
|--------------|----------------------------|
| US-15.20.1.1 | extension developer (P-25) |
| US-15.20.1.2 | engine developer (P-26)    |
| US-15.20.2.1 | extension developer (P-25) |
| US-15.20.2.2 | game designer (P-5)        |
| US-15.20.3.1 | engine developer (P-26)    |
| US-15.20.3.2 | technical artist (P-13)    |
| US-15.20.4.1 | extension developer (P-25) |
| US-15.20.4.2 | engine developer (P-26)    |
| US-15.20.5.1 | extension developer (P-25) |
| US-15.20.5.2 | build engineer (P-16)      |
| US-15.20.6.1 | extension developer (P-25) |
| US-15.20.6.2 | game designer (P-5)        |
| US-15.20.7.1 | game designer (P-5)        |
| US-15.20.7.2 | extension developer (P-25) |
| US-15.20.8.1 | game designer (P-5)        |
| US-15.20.8.2 | extension developer (P-25) |
| US-15.20.9.1 | game designer (P-5)        |
| US-15.20.9.2 | technical artist (P-13)    |
| US-15.20.10.1 | extension developer (P-25) |
| US-15.20.10.2 | engine developer (P-26)    |

1. **US-15.20.1.1** — **As a** extension developer (P-25), **I want** a stable ABI plugin system for
   custom panels, inspectors, gizmos, and importers, **so that** my plugins work across engine
   versions.

2. **US-15.20.1.2** — **As a** engine developer (P-26), **I want** capability-based API access
   limiting plugins to requested subsystems, **so that** plugins cannot access internals.

3. **US-15.20.2.1** — **As a** extension developer (P-25), **I want** to register custom editor
   widgets for my component types, **so that** the inspector renders plugin-provided editors.

4. **US-15.20.2.2** — **As a** game designer (P-5), **I want** custom component editors to integrate
   with undo/redo automatically, **so that** plugin-driven edits are reversible.

5. **US-15.20.3.1** — **As a** engine developer (P-26), **I want** engine subsystems to register
   their editors through the same plugin mechanism as third-party plugins, **so that** built-in and
   external editors share one extension point.

6. **US-15.20.3.2** — **As a** technical artist (P-13), **I want** animation, physics, audio, AI,
   and VFX subsystems to provide specialized editors via the plugin API, **so that** all subsystem
   editors are consistent.

7. **US-15.20.4.1** — **As a** extension developer (P-25), **I want** hot-reload that preserves open
   panels, inspector state, and undo history, **so that** I iterate without restarting.

8. **US-15.20.4.2** — **As a** engine developer (P-26), **I want** hot-reload to serialize plugin
   state via the reflection system, **so that** reload is automatic and safe.

9. **US-15.20.5.1** — **As a** extension developer (P-25), **I want** plugin manifests declaring
   dependencies with semantic version ranges, **so that** the host resolves and loads plugins in
   correct order.

10. **US-15.20.5.2** — **As a** build engineer (P-16), **I want** missing dependency errors with
    installation guidance, **so that** broken plugin installs are diagnosed quickly.

11. **US-15.20.6.1** — **As a** extension developer (P-25), **I want** to publish plugins to the
    asset marketplace with one-click install for users, **so that** distribution is seamless.

12. **US-15.20.6.2** — **As a** game designer (P-5), **I want** automatic update notifications for
    installed plugins, **so that** I stay on compatible versions.

13. **US-15.20.7.1** — **As a** game designer (P-5), **I want** to create editor plugins entirely
    using logic graphs and the UI editor without writing code, **so that** I can build custom tools
    myself.

14. **US-15.20.7.2** — **As a** extension developer (P-25), **I want** no-code plugins packaged as
    standard editor plugins, **so that** they participate in hot-reload and marketplace
    distribution.

15. **US-15.20.8.1** — **As a** game designer (P-5), **I want** editor-specific logic graph nodes
    for panels, menus, gizmos, and undo commands, **so that** plugin behavior is fully visual.

16. **US-15.20.8.2** — **As a** extension developer (P-25), **I want** graph execution driven by
    editor lifecycle events, **so that** plugins respond to selection changes and ticks.

17. **US-15.20.9.1** — **As a** game designer (P-5), **I want** plugin UI defined in the visual UI
    editor with property bindings to logic graph variables, **so that** layout is drag-and-drop.

18. **US-15.20.9.2** — **As a** technical artist (P-13), **I want** plugin UI to inherit the editor
    theme for visual consistency, **so that** custom tools blend with built-in panels.

19. **US-15.20.10.1** — **As a** extension developer (P-25), **I want** hot module reload completing
    within 2 seconds for both no-code and native plugins, **so that** iteration is fast.

20. **US-15.20.10.2** — **As a** engine developer (P-26), **I want** reload to work in both debug
    and release builds, **so that** hot-reload is available in production workflows.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.20.1 | extension developer (P-25) |
| US-15.20.10 | extension developer (P-25) |
| US-15.20.2 | extension developer (P-25) |
| US-15.20.3 | engine developer (P-26) |
| US-15.20.4 | extension developer (P-25) |
| US-15.20.5 | extension developer (P-25) |
| US-15.20.6 | extension developer (P-25) |
| US-15.20.7 | game designer (P-5) |
| US-15.20.8 | game designer (P-5) |
| US-15.20.9 | game designer (P-5) |

1. **US-15.20.1** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.20.1.1 through US-15.20.1.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-15.20.10** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.20.10.1 through US-15.20.10.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.20.2** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.20.2.1 through US-15.20.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-15.20.3** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-15.20.3.1 through US-15.20.3.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-15.20.4** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.20.4.1 through US-15.20.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-15.20.5** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.20.5.1 through US-15.20.5.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-15.20.6** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.20.6.1 through US-15.20.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-15.20.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.20.7.1 through US-15.20.7.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

9. **US-15.20.8** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.20.8.1 through US-15.20.8.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

10. **US-15.20.9** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-15.20.9.1 through US-15.20.9.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.
