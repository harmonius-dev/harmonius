# User Stories: Documentation and Learning

## F-15.19.1 Auto-Generated API Reference

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.19.1.1 | developer (P-15) | to search an auto-generated API reference covering every public type, trait, function, and method with descriptions, parameter types, examples, and cross-references | I can integrate with the engine API without reading source code |  |  |
| US-15.19.1.2 | designer (P-5) | to click "?" on any inspector property or node type to open the relevant API reference page in the editor's help panel | I can understand each setting without leaving my workspace |  |  |
| US-15.19.1.3 | engine developer (P-26) | the API reference generated as a searchable static website via rustdoc and published alongside each engine release | documentation stays current with the codebase |  |  |
| US-15.19.1.4 | engine tester (P-27) | the CI build to fail if any public API lacks a doc comment | documentation coverage stays at 100% |  |  |

## F-15.19.2 Logic Graph Node Documentation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.19.2.1 | designer (P-5) | to hover over any node in the graph editor and see its description, input/output port descriptions with types and valid ranges, and usage examples | I can understand node behavior without consulting external documentation |  |  |
| US-15.19.2.2 | tech artist (P-13) | a dedicated documentation panel showing full node docs including performance notes and "see also" links when a node is selected | I can evaluate node suitability for performance-sensitive graphs |  |  |
| US-15.19.2.3 | modder (P-24) | node documentation accessible in the mod SDK graph editor | I can build mod logic without access to engine source code or external docs |  |  |
| US-15.19.2.4 | engine developer (P-26) | custom nodes to inherit a documentation template that prompts the author to fill in descriptions for each port | user-authored nodes maintain consistent documentation quality |  |  |

## F-15.19.3 Interactive In-Editor Tutorials

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.19.3.1 | designer (P-5) | an interactive "Getting Started" tutorial that highlights UI elements with spotlight overlays and waits for me to complete each step | I can learn the editor workflow through hands-on practice |  |  |
| US-15.19.3.2 | artist (P-8) | a "Create a Material" tutorial category teaching me to import assets, build material graphs, and light a scene | I can become productive in the art pipeline quickly |  |  |
| US-15.19.3.3 | modder (P-24) | tutorials authored as data assets using the logic graph system | community-created tutorials can be distributed via the asset store |  |  |
| US-15.19.3.4 | engine tester (P-27) | to run each interactive tutorial from start to finish and verify it completes without errors | new users have a reliable onboarding experience on every release |  |  |

## F-15.19.4 Video Tutorial Integration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.19.4.1 | artist (P-8) | an embedded video player docked alongside the viewport that plays tutorial videos without leaving the editor | I can follow along step-by-step |  |  |
| US-15.19.4.2 | designer (P-5) | chapter timestamps in videos that optionally open the relevant editor panel or tool on click | I can jump directly to the section I need |  |  |
| US-15.19.4.3 | creative director (P-2) | a curated video library accessible from the Help menu organized by topic and difficulty | the team has quick access to relevant training content |  |  |
| US-15.19.4.4 | engine tester (P-27) | to verify that previously watched videos are cached locally for offline playback | tutorials are accessible without internet |  |  |

## F-15.19.5 Contextual Help and Tooltip System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.19.5.1 | designer (P-5) | to hover over any inspector property and see a tooltip with its description, type, valid range, default value, and keyboard shortcut | I understand each setting without external documentation |  |  |
| US-15.19.5.2 | artist (P-8) | a "What's This?" mode where clicking any UI element shows its documentation | I can learn the purpose of unfamiliar tools quickly |  |  |
| US-15.19.5.3 | developer (P-15) | tooltips to include links to the full documentation page and relevant tutorial | I can dive deeper when the tooltip alone is insufficient |  |  |
| US-15.19.5.4 | engine tester (P-27) | to verify that help content is stored as localized string assets and displays correctly in all supported languages | non-English users receive accurate contextual help |  |  |

## F-15.19.6 Sample Projects and Template Library

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.19.6.1 | designer (P-5) | to browse genre sample projects (FPS, RPG, RTS, platformer, racing, VR, survival, action/adventure) from the project creation wizard | I can start with pre-built gameplay systems and focus on my unique mechanics |  |  |
| US-15.19.6.2 | artist (P-8) | sample projects to include organized asset structures with explanatory comments | I can learn production asset organization from working examples |  |  |
| US-15.19.6.3 | modder (P-24) | sample projects downloadable from the asset store | I can learn engine capabilities outside the project creation wizard |  |  |
| US-15.19.6.4 | engine tester (P-27) | to verify that every sample project produces a playable build on all target platforms | samples are always functional |  |  |

## F-15.19.7 Inline Code Examples in Documentation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.19.7.1 | developer (P-15) | every API doc page to include runnable Rust code examples guaranteed to compile against the current engine version | I can copy and adapt working code |  |  |
| US-15.19.7.2 | designer (P-5) | visual examples (logic graph screenshots with before/after screenshots) alongside code examples | no-code users also have reference material |  |  |
| US-15.19.7.3 | engine developer (P-26) | examples versioned alongside engine source and updated by the project upgrade system | documentation never references deprecated APIs |  |  |
| US-15.19.7.4 | engine tester (P-27) | all doc-test code examples extracted and compiled automatically in CI | stale examples are caught immediately and never reach users |  |  |
