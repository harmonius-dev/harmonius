# R-15.19 -- Documentation and Learning Requirements

## API Reference

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.19.1 | The engine **SHALL** auto-generate API documentation from Rust doc comments as a searchable static website per release, with in-editor "?" buttons opening the relevant page, and CI enforcing `![deny(missing_docs)]` on all public items. | [F-15.19.1](../../features/tools-editor/documentation.md) | Complete, up-to-date API docs are essential for developer productivity. | Benchmark: verify the in-editor help panel opens within 500 ms. |
| R-15.19.2 | Every built-in logic graph node **SHALL** include a description tooltip, input/output port documentation with types and valid ranges, usage examples as embedded mini-graphs, and a dedicated doc panel on selection. Custom nodes **SHALL** inherit a documentation template. | [F-15.19.2](../../features/tools-editor/documentation.md) | No-code users need inline documentation to understand node behavior without external references. | Unit test: verify all built-in nodes have description, port docs, and examples. |

## Tutorials

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.19.3 | The editor **SHALL** provide step-by-step guided tutorials with spotlight overlays highlighting UI elements, waiting for user completion per step, pause/resume/restart controls, and community-created tutorials via the asset store. | [F-15.19.3](../../features/tools-editor/documentation.md) | Hands-on learning inside the editor is more effective than external documentation. | Benchmark: verify each tutorial category completes under 30 minutes. |
| R-15.19.4 | The editor **SHALL** embed a dockable video player for tutorials with chapter timestamps linking to editor panels, and local caching for offline playback. | [F-15.19.4](../../features/tools-editor/documentation.md) | Video tutorials within the editor enable step-by-step following without context switching. | Unit test: cache a video and verify playback works without network connectivity. |
| R-15.19.5 | The editor **SHALL** provide tooltips on every inspector property showing description, type, range, and default, with "?" button and F1 access to contextual help, a "What's This?" mode for clicking any UI element, and help content as localized string assets updated independently of engine releases. | [F-15.19.5](../../features/tools-editor/documentation.md) | Contextual help at the point of use reduces documentation lookup time. | Benchmark: verify tooltips appear within 200 ms on hover for all inspector properties. |

## Sample Projects

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.19.6 | The engine **SHALL** provide sample projects for at least 8 game genres (action, platformer, FPS, RTS, RPG, racing, VR, survival) with annotated logic graphs, playable builds, and companion walkthrough documents. | [F-15.19.6](../../features/tools-editor/documentation.md) | Working reference projects accelerate learning across all game types. | Integration test: verify each sample project loads and runs without errors. |
| R-15.19.7 | API documentation **SHALL** include runnable code examples, visual examples (logic graph screenshots) for no-code users, CI-compiled doc-tests, and visual examples updated by the project upgrade system. | [F-15.19.7](../../features/tools-editor/documentation.md) | Stale examples break trust in documentation; CI testing prevents this. | Unit test: compile and run all doc-test examples and verify they pass. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/documentation.md](../../user-stories/tools-editor/documentation.md).
Requirements in this document are derived from those user stories.
