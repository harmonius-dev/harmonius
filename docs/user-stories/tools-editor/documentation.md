# User Stories: Documentation and Learning

## F-15.19.1 Auto-Generated API Reference

## US-15.19.1.1 Developer Searches API Reference

**As a** developer (P-15), **I want** to search an auto-generated API reference covering every
public type, trait, function, and method with descriptions, parameter types, examples, and
cross-references, **so that** I can integrate with the engine API without reading source code.

## US-15.19.1.2 Designer Clicks Help in Inspector

**As a** designer (P-5), **I want** to click "?" on any inspector property or node type to open the
relevant API reference page in the editor's help panel, **so that** I can understand each setting
without leaving my workspace.

## US-15.19.1.3 Engine Dev Publishes with Releases

**As an** engine developer (P-26), **I want** the API reference generated as a searchable static
website via rustdoc and published alongside each engine release, **so that** documentation stays
current with the codebase.

## US-15.19.1.4 Engine Tester Validates Coverage

**As an** engine tester (P-27), **I want** the CI build to fail if any public API lacks a doc
comment, **so that** documentation coverage stays at 100%.

## F-15.19.2 Logic Graph Node Documentation

## US-15.19.2.1 Designer Hovers for Node Docs

**As a** designer (P-5), **I want** to hover over any node in the graph editor and see its
description, input/output port descriptions with types and valid ranges, and usage examples,
**so that** I can understand node behavior without consulting external documentation.

## US-15.19.2.2 Tech Artist Views Dedicated Doc Panel

**As a** tech artist (P-13), **I want** a dedicated documentation panel showing full node docs
including performance notes and "see also" links when a node is selected, **so that** I can evaluate
node suitability for performance-sensitive graphs.

## US-15.19.2.3 Modder Reads Node Docs in Mod SDK

**As a** modder (P-24), **I want** node documentation accessible in the mod SDK graph editor,
**so that** I can build mod logic without access to engine source code or external docs.

## US-15.19.2.4 Engine Dev Fills Documentation Template

**As an** engine developer (P-26), **I want** custom nodes to inherit a documentation template that
prompts the author to fill in descriptions for each port, **so that** user-authored nodes maintain
consistent documentation quality.

## F-15.19.3 Interactive In-Editor Tutorials

## US-15.19.3.1 Designer Follows Getting Started Tutorial

**As a** designer (P-5), **I want** an interactive "Getting Started" tutorial that highlights UI
elements with spotlight overlays and waits for me to complete each step, **so that** I can learn the
editor workflow through hands-on practice.

## US-15.19.3.2 Artist Learns Material Authoring

**As an** artist (P-8), **I want** a "Create a Material" tutorial category teaching me to import
assets, build material graphs, and light a scene, **so that** I can become productive in the art
pipeline quickly.

## US-15.19.3.3 Modder Creates Tutorial Content

**As a** modder (P-24), **I want** tutorials authored as data assets using the logic graph system,
**so that** community-created tutorials can be distributed via the asset store.

## US-15.19.3.4 Engine Tester Validates Tutorial Completion

**As an** engine tester (P-27), **I want** to run each interactive tutorial from start to finish and
verify it completes without errors, **so that** new users have a reliable onboarding experience on
every release.

## F-15.19.4 Video Tutorial Integration

## US-15.19.4.1 Artist Watches Docked Video

**As an** artist (P-8), **I want** an embedded video player docked alongside the viewport that plays
tutorial videos without leaving the editor, **so that** I can follow along step-by-step.

## US-15.19.4.2 Designer Navigates by Chapter

**As a** designer (P-5), **I want** chapter timestamps in videos that optionally open the relevant
editor panel or tool on click, **so that** I can jump directly to the section I need.

## US-15.19.4.3 Creative Director Curates Video Library

**As a** creative director (P-2), **I want** a curated video library accessible from the Help menu
organized by topic and difficulty, **so that** the team has quick access to relevant training
content.

## US-15.19.4.4 Engine Tester Validates Offline Cache

**As an** engine tester (P-27), **I want** to verify that previously watched videos are cached
locally for offline playback, **so that** tutorials are accessible without internet.

## F-15.19.5 Contextual Help and Tooltip System

## US-15.19.5.1 Designer Hovers for Property Help

**As a** designer (P-5), **I want** to hover over any inspector property and see a tooltip with its
description, type, valid range, default value, and keyboard shortcut, **so that** I understand each
setting without external documentation.

## US-15.19.5.2 Artist Uses What's This Mode

**As an** artist (P-8), **I want** a "What's This?" mode where clicking any UI element shows its
documentation, **so that** I can learn the purpose of unfamiliar tools quickly.

## US-15.19.5.3 Developer Links to Full Docs

**As a** developer (P-15), **I want** tooltips to include links to the full documentation page and
relevant tutorial, **so that** I can dive deeper when the tooltip alone is insufficient.

## US-15.19.5.4 Engine Tester Validates Localized Help

**As an** engine tester (P-27), **I want** to verify that help content is stored as localized string
assets and displays correctly in all supported languages, **so that** non-English users receive
accurate contextual help.

## F-15.19.6 Sample Projects and Template Library

## US-15.19.6.1 Designer Opens Genre Sample

**As a** designer (P-5), **I want** to browse genre sample projects (FPS, RPG, RTS, platformer,
racing, VR, survival, action/adventure) from the project creation wizard, **so that** I can start
with pre-built gameplay systems and focus on my unique mechanics.

## US-15.19.6.2 Artist Studies Annotated Assets

**As an** artist (P-8), **I want** sample projects to include organized asset structures with
explanatory comments, **so that** I can learn production asset organization from working examples.

## US-15.19.6.3 Modder Downloads Samples from Store

**As a** modder (P-24), **I want** sample projects downloadable from the asset store, **so that** I
can learn engine capabilities outside the project creation wizard.

## US-15.19.6.4 Engine Tester Validates Sample Builds

**As an** engine tester (P-27), **I want** to verify that every sample project produces a playable
build on all target platforms, **so that** samples are always functional.

## F-15.19.7 Inline Code Examples in Documentation

## US-15.19.7.1 Developer Copies Working Examples

**As a** developer (P-15), **I want** every API doc page to include runnable Rust code examples
guaranteed to compile against the current engine version, **so that** I can copy and adapt working
code.

## US-15.19.7.2 Designer Sees Visual Examples

**As a** designer (P-5), **I want** visual examples (logic graph screenshots with before/after
screenshots) alongside code examples, **so that** no-code users also have reference material.

## US-15.19.7.3 Engine Dev Keeps Examples Current

**As an** engine developer (P-26), **I want** examples versioned alongside engine source and updated
by the project upgrade system, **so that** documentation never references deprecated APIs.

## US-15.19.7.4 Engine Tester Validates Doc-Tests

**As an** engine tester (P-27), **I want** all doc-test code examples extracted and compiled
automatically in CI, **so that** stale examples are caught immediately and never reach users.
