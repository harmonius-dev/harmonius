# 15.19 — Documentation and Learning

## API Reference

| ID | Feature |
| ----------- | -------------------------------- |
| F-15.19.1 | Auto-Generated API Reference |
| F-15.19.2 | Logic Graph Node Documentation |

1. **F-15.19.1** — Automatically generate comprehensive API reference documentation from Rust doc
   comments across all public engine crates. The reference covers: every public type, trait,
   function, and method with descriptions, parameter types, return types, examples, and
   cross-references. Generated as a searchable static website (rustdoc) published alongside each
   engine release. The reference is also accessible from within the editor via a help panel that
   opens the relevant page when clicking "?" on any inspector property or node type.
   - **Deps:** F-1.3.1 (Type Registry), F-15.1.1 (Editor Framework)
   - **Platform:** Desktop only for in-editor help. Static website accessible from any browser.
2. **F-15.19.2** — Every built-in logic graph node (F-15.8.10) includes inline documentation:
   description, input/output port descriptions with types and valid ranges, usage examples as
   embedded mini-graphs, performance notes, and "see also" links to related nodes. Documentation is
   displayed in a tooltip when hovering over a node in the graph editor and in a dedicated
   documentation panel when a node is selected. Custom nodes (user-authored) inherit a documentation
   template that prompts the author to fill in descriptions for each port.
   - **Deps:** F-15.8.10 (Graph Node Library), F-15.8.4 (Gameplay Logic Graphs)
   - **Platform:** Desktop only (editor feature).

## In-Editor Tutorials

| ID | Feature |
| ----------- | ------------------------------------ |
| F-15.19.3 | Interactive In-Editor Tutorials |
| F-15.19.4 | Video Tutorial Integration |
| F-15.19.5 | Contextual Help and Tooltip System |

1. **F-15.19.3** — Step-by-step guided tutorials that run inside the editor, teaching users how to
   use engine features through hands-on exercises. Each tutorial: highlights UI elements with
   spotlight overlays, provides instruction text in a docked panel, waits for the user to complete
   each step before advancing, and can be paused/resumed/restarted. Tutorial categories: Getting
   Started (create a project, place an entity, play), Gameplay (build a simple game mechanic with
   logic graphs), Art (import an asset, create a material, light a scene), Audio (set up spatial
   audio, create adaptive music), Networking (enable multiplayer, test with two clients), and
   Advanced (custom shaders, procedural generation, optimization). Tutorials are authored as data
   assets using the logic graph system, enabling community-created tutorials distributed via the
   asset store. System)
   - **Deps:** F-15.1.1 (Editor Framework), F-15.8.4 (Logic Graphs), F-13.24.7a (Tutorial Step
   - **Platform:** Desktop only (editor feature).
2. **F-15.19.4** — Embedded video player within the editor that plays tutorial videos without
   leaving the editor context. Videos are streamed from the engine's documentation server or cached
   locally. The video panel can be docked alongside the viewport, allowing users to follow along
   step-by-step. Chapters within videos link to specific editor actions — clicking a chapter
   timestamp can optionally open the relevant editor panel or tool. A curated video library is
   accessible from the editor's Help menu, organized by topic and difficulty level. Community
   tutorial videos can be linked from the asset store.
   - **Deps:** F-15.1.1 (Editor Framework)
   - **Platform:** Desktop only. Video streaming requires internet; local cache for offline use.
3. **F-15.19.5** — Every editor panel, property inspector field, toolbar button, and menu item
   includes contextual help accessible via hover tooltip, "?" button, or F1 key. Tooltips show:
   short description (1 line), detailed description (expandable), keyboard shortcut if applicable,
   link to full documentation, and link to relevant tutorial. Property inspector fields show: value
   type, valid range, default value, and what the property affects. Help content is stored as
   localized string assets and updated independently of engine releases. A "What's This?" mode
   (similar to Qt) allows clicking any UI element to see its documentation.
   - **Deps:** F-15.1.1 (Editor Framework), F-10.1.9 (Localization Hooks)
   - **Platform:** Desktop only (editor feature).

## Sample Projects

| ID | Feature |
| ----------- | --------------------------------------- |
| F-15.19.6 | Sample Projects and Template Library |
| F-15.19.7 | Inline Code Examples in Documentation |

1. **F-15.19.6** — A library of complete sample projects demonstrating engine capabilities across
   genres: 3D action/adventure (combat, AI, level streaming), 2D platformer (sprites, tilemaps, 2D
   physics), FPS (weapons, networking, headless server), RTS (selection, fog of war, building), RPG
   (inventory, dialogue, quests, character customization), racing (vehicle physics, checkpoints,
   AI), VR experience (hand tracking, spatial UI), and survival (building, crafting, voxel terrain).
   Each sample includes: playable build, annotated logic graphs with explanatory comments, organized
   asset structure, and a companion walkthrough document explaining key design decisions. Samples
   are downloadable from the launcher's project creation wizard (F-15.15.3) or the asset store.
   - **Deps:** F-15.15.3 (Project Creation Wizard), F-15.17.1 (Asset Store)
2. **F-15.19.7** — All engine API documentation includes runnable code examples (Rust snippets for
   plugin developers) and visual examples (logic graph screenshots for no-code users). Code examples
   are tested as part of the CI pipeline (F-15.18.6) — if an example fails to compile against the
   current engine version, the build fails, ensuring documentation never goes stale. Visual examples
   include before/after screenshots showing the effect of each feature. Examples are versioned
   alongside the engine source and automatically updated by the project upgrade system (F-15.15.2).
   - **Deps:** F-15.18.6 (CI/CD Pipeline), F-15.15.2 (Project Upgrades)
