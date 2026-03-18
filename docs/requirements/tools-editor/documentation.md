# R-15.19 -- Documentation and Learning Requirements

## API Reference

| ID        | Derived From                                              |
|-----------|-----------------------------------------------------------|
| R-15.19.1 | [F-15.19.1](../../features/tools-editor/documentation.md) |
| R-15.19.2 | [F-15.19.2](../../features/tools-editor/documentation.md) |

1. **R-15.19.1** — The engine **SHALL** auto-generate API documentation from Rust doc comments as a
   searchable static website per release, with in-editor "?" buttons opening the relevant page, and
   CI enforcing `![deny(missing_docs)]` on all public items.
   - **Rationale:** Complete, up-to-date API docs are essential for developer productivity.
   - **Verification:** Benchmark: verify the in-editor help panel opens within 500 ms.
2. **R-15.19.2** — Every built-in logic graph node **SHALL** include a description tooltip,
   input/output port documentation with types and valid ranges, usage examples as embedded
   mini-graphs, and a dedicated doc panel on selection. Custom nodes **SHALL** inherit a
   documentation template.
   - **Rationale:** No-code users need inline documentation to understand node behavior without
     external references.
   - **Verification:** Unit test: verify all built-in nodes have description, port docs, and
     examples.

## Tutorials

| ID        | Derived From                                              |
|-----------|-----------------------------------------------------------|
| R-15.19.3 | [F-15.19.3](../../features/tools-editor/documentation.md) |
| R-15.19.4 | [F-15.19.4](../../features/tools-editor/documentation.md) |
| R-15.19.5 | [F-15.19.5](../../features/tools-editor/documentation.md) |

1. **R-15.19.3** — The editor **SHALL** provide step-by-step guided tutorials with spotlight
   overlays highlighting UI elements, waiting for user completion per step, pause/resume/restart
   controls, and community-created tutorials via the asset store.
   - **Rationale:** Hands-on learning inside the editor is more effective than external
     documentation.
   - **Verification:** Benchmark: verify each tutorial category completes under 30 minutes.
2. **R-15.19.4** — The editor **SHALL** embed a dockable video player for tutorials with chapter
   timestamps linking to editor panels, and local caching for offline playback.
   - **Rationale:** Video tutorials within the editor enable step-by-step following without context
     switching.
   - **Verification:** Unit test: cache a video and verify playback works without network
     connectivity.
3. **R-15.19.5** — The editor **SHALL** provide tooltips on every inspector property showing
   description, type, range, and default, with "?" button and F1 access to contextual help, a
   "What's This?" mode for clicking any UI element, and help content as localized string assets
   updated independently of engine releases.
   - **Rationale:** Contextual help at the point of use reduces documentation lookup time.
   - **Verification:** Benchmark: verify tooltips appear within 200 ms on hover for all inspector
     properties.

## Sample Projects

| ID        | Derived From                                              |
|-----------|-----------------------------------------------------------|
| R-15.19.6 | [F-15.19.6](../../features/tools-editor/documentation.md) |
| R-15.19.7 | [F-15.19.7](../../features/tools-editor/documentation.md) |

1. **R-15.19.6** — The engine **SHALL** provide sample projects for at least 8 game genres (action,
   platformer, FPS, RTS, RPG, racing, VR, survival) with annotated logic graphs, playable builds,
   and companion walkthrough documents.
   - **Rationale:** Working reference projects accelerate learning across all game types.
   - **Verification:** Integration test: verify each sample project loads and runs without errors.
2. **R-15.19.7** — API documentation **SHALL** include runnable code examples, visual examples
   (logic graph screenshots) for no-code users, CI-compiled doc-tests, and visual examples updated
   by the project upgrade system.
   - **Rationale:** Stale examples break trust in documentation; CI testing prevents this.
   - **Verification:** Unit test: compile and run all doc-test examples and verify they pass.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/documentation.md](../../user-stories/tools-editor/documentation.md).
Requirements in this document are derived from those user stories.
