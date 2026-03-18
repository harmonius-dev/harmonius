# User Stories: Documentation and Learning

## F-15.19.1 Auto-Generated API Reference

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.19.1.1 | developer (P-15)        |          |              |
| US-15.19.1.2 | designer (P-5)          |          |              |
| US-15.19.1.3 | engine developer (P-26) |          |              |
| US-15.19.1.4 | engine tester (P-27)    |          |              |

1. **US-15.19.1.1** — to search an auto-generated API reference covering every public type, trait,
   function, and method with descriptions, parameter types, examples, and cross-references
   - **Acceptance:** I can integrate with the engine API without reading source code
2. **US-15.19.1.2** — to click "?" on any inspector property or node type to open the relevant API
   reference page in the editor's help panel
   - **Acceptance:** I can understand each setting without leaving my workspace
3. **US-15.19.1.3** — the API reference generated as a searchable static website via rustdoc and
   published alongside each engine release
   - **Acceptance:** documentation stays current with the codebase
4. **US-15.19.1.4** — the CI build to fail if any public API lacks a doc comment
   - **Acceptance:** documentation coverage stays at 100%

## F-15.19.2 Logic Graph Node Documentation

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.19.2.1 | designer (P-5)          |          |              |
| US-15.19.2.2 | tech artist (P-13)      |          |              |
| US-15.19.2.3 | modder (P-24)           |          |              |
| US-15.19.2.4 | engine developer (P-26) |          |              |

1. **US-15.19.2.1** — to hover over any node in the graph editor and see its description,
   input/output port descriptions with types and valid ranges, and usage examples
   - **Acceptance:** I can understand node behavior without consulting external documentation
2. **US-15.19.2.2** — a dedicated documentation panel showing full node docs including performance
   notes and "see also" links when a node is selected
   - **Acceptance:** I can evaluate node suitability for performance-sensitive graphs
3. **US-15.19.2.3** — node documentation accessible in the mod SDK graph editor
   - **Acceptance:** I can build mod logic without access to engine source code or external docs
4. **US-15.19.2.4** — custom nodes to inherit a documentation template that prompts the author to
   fill in descriptions for each port
   - **Acceptance:** user-authored nodes maintain consistent documentation quality

## F-15.19.3 Interactive In-Editor Tutorials

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.19.3.1 | designer (P-5)       |          |              |
| US-15.19.3.2 | artist (P-8)         |          |              |
| US-15.19.3.3 | modder (P-24)        |          |              |
| US-15.19.3.4 | engine tester (P-27) |          |              |

1. **US-15.19.3.1** — an interactive "Getting Started" tutorial that highlights UI elements with
   spotlight overlays and waits for me to complete each step
   - **Acceptance:** I can learn the editor workflow through hands-on practice
2. **US-15.19.3.2** — a "Create a Material" tutorial category teaching me to import assets, build
   material graphs, and light a scene
   - **Acceptance:** I can become productive in the art pipeline quickly
3. **US-15.19.3.3** — tutorials authored as data assets using the logic graph system
   - **Acceptance:** community-created tutorials can be distributed via the asset store
4. **US-15.19.3.4** — to run each interactive tutorial from start to finish and verify it completes
   without errors
   - **Acceptance:** new users have a reliable onboarding experience on every release

## F-15.19.4 Video Tutorial Integration

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.19.4.1 | artist (P-8)            |          |              |
| US-15.19.4.2 | designer (P-5)          |          |              |
| US-15.19.4.3 | creative director (P-2) |          |              |
| US-15.19.4.4 | engine tester (P-27)    |          |              |

1. **US-15.19.4.1** — an embedded video player docked alongside the viewport that plays tutorial
   videos without leaving the editor
   - **Acceptance:** I can follow along step-by-step
2. **US-15.19.4.2** — chapter timestamps in videos that optionally open the relevant editor panel or
   tool on click
   - **Acceptance:** I can jump directly to the section I need
3. **US-15.19.4.3** — a curated video library accessible from the Help menu organized by topic and
   difficulty
   - **Acceptance:** the team has quick access to relevant training content
4. **US-15.19.4.4** — to verify that previously watched videos are cached locally for offline
   playback
   - **Acceptance:** tutorials are accessible without internet

## F-15.19.5 Contextual Help and Tooltip System

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.19.5.1 | designer (P-5)       |          |              |
| US-15.19.5.2 | artist (P-8)         |          |              |
| US-15.19.5.3 | developer (P-15)     |          |              |
| US-15.19.5.4 | engine tester (P-27) |          |              |

1. **US-15.19.5.1** — to hover over any inspector property and see a tooltip with its description,
   type, valid range, default value, and keyboard shortcut
   - **Acceptance:** I understand each setting without external documentation
2. **US-15.19.5.2** — a "What's This?" mode where clicking any UI element shows its documentation
   - **Acceptance:** I can learn the purpose of unfamiliar tools quickly
3. **US-15.19.5.3** — tooltips to include links to the full documentation page and relevant tutorial
   - **Acceptance:** I can dive deeper when the tooltip alone is insufficient
4. **US-15.19.5.4** — to verify that help content is stored as localized string assets and displays
   correctly in all supported languages
   - **Acceptance:** non-English users receive accurate contextual help

## F-15.19.6 Sample Projects and Template Library

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.19.6.1 | designer (P-5)       |          |              |
| US-15.19.6.2 | artist (P-8)         |          |              |
| US-15.19.6.3 | modder (P-24)        |          |              |
| US-15.19.6.4 | engine tester (P-27) |          |              |

1. **US-15.19.6.1** — to browse genre sample projects (FPS, RPG, RTS, platformer, racing, VR,
   survival, action/adventure) from the project creation wizard
   - **Acceptance:** I can start with pre-built gameplay systems and focus on my unique mechanics
2. **US-15.19.6.2** — sample projects to include organized asset structures with explanatory
   comments
   - **Acceptance:** I can learn production asset organization from working examples
3. **US-15.19.6.3** — sample projects downloadable from the asset store
   - **Acceptance:** I can learn engine capabilities outside the project creation wizard
4. **US-15.19.6.4** — to verify that every sample project produces a playable build on all target
   platforms
   - **Acceptance:** samples are always functional

## F-15.19.7 Inline Code Examples in Documentation

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.19.7.1 | developer (P-15)        |          |              |
| US-15.19.7.2 | designer (P-5)          |          |              |
| US-15.19.7.3 | engine developer (P-26) |          |              |
| US-15.19.7.4 | engine tester (P-27)    |          |              |

1. **US-15.19.7.1** — every API doc page to include runnable Rust code examples guaranteed to
   compile against the current engine version
   - **Acceptance:** I can copy and adapt working code
2. **US-15.19.7.2** — visual examples (logic graph screenshots with before/after screenshots)
   alongside code examples
   - **Acceptance:** no-code users also have reference material
3. **US-15.19.7.3** — examples versioned alongside engine source and updated by the project upgrade
   system
   - **Acceptance:** documentation never references deprecated APIs
4. **US-15.19.7.4** — all doc-test code examples extracted and compiled automatically in CI
   - **Acceptance:** stale examples are caught immediately and never reach users
