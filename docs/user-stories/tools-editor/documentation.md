# User Stories: Documentation and Learning

## US-15.19.1 Designer Discovers Property Help

**As a** designer (P-5) learning the editor, **I want** to hover over any inspector property
and see a tooltip with its description, type, valid range, and default value, **so that** I can
understand what each setting does without searching external documentation.

## US-15.19.2 Designer Follows Getting Started Tutorial

**As a** designer (P-5) new to the engine, **I want** to launch an interactive "Getting Started"
tutorial that highlights each UI element with a spotlight overlay and waits for me to complete
each step, **so that** I can learn the editor workflow through hands-on practice in under 30
minutes.

## US-15.19.3 Artist Watches Embedded Video Tutorial

**As an** artist (P-8) importing my first asset, **I want** to watch a video tutorial docked
beside the viewport without leaving the editor, **so that** I can follow along step-by-step
while working in the actual editor panels.

## US-15.19.4 Artist Uses What's This Mode

**As an** artist (P-8) unfamiliar with the material editor, **I want** to activate "What's
This?" mode and click any toolbar button or panel to see its documentation, **so that** I can
quickly learn the purpose of each tool without trial and error.

## US-15.19.5 Developer Searches API Reference

**As a** developer (P-15) writing an engine plugin, **I want** to search the auto-generated
API reference for a trait and see its description, methods, parameter types, and runnable code
examples, **so that** I can integrate with the engine API without reading source code.

## US-15.19.6 Developer Views Inline Examples

**As a** developer (P-15) integrating a new system, **I want** every API doc page to include
runnable Rust code examples that are guaranteed to compile against the current engine version,
**so that** I can copy and adapt working code instead of guessing at correct usage.

## US-15.19.7 Engine Developer Adds Doc Comments

**As an** engine developer (P-26) adding a new public type, **I want** the CI build to fail if
I forget to add a doc comment, **so that** documentation coverage stays at 100% and no public
API goes undocumented.

## US-15.19.8 Engine Developer Updates Node Docs

**As an** engine developer (P-26) modifying a built-in logic graph node, **I want** a
documentation template that reminds me to update the description, port docs, and example graph,
**so that** node documentation stays accurate after every change.

## US-15.19.9 Tester Verifies Doc Examples Compile

**As an** engine tester (P-27) running the CI pipeline, **I want** all doc-test code examples
to be extracted and compiled automatically as part of the test suite, **so that** stale examples
are caught immediately and never reach users.

## US-15.19.10 Tester Validates Tutorial Completion

**As an** engine tester (P-27) verifying release readiness, **I want** to run each interactive
tutorial from start to finish and confirm it completes without errors in under 30 minutes,
**so that** new users have a reliable onboarding experience on every release.

## US-15.19.11 Prototyper Opens Sample Project

**As a** prototyper (P-7) starting a new game, **I want** to browse genre sample projects
(FPS, RPG, RTS, platformer, VR, racing, survival, action/adventure) from the project creation
wizard and open one as a starting point, **so that** I can begin with pre-built gameplay
systems and focus on my unique mechanics.

## US-15.19.12 Prototyper Studies Annotated Logic Graphs

**As a** prototyper (P-7) learning gameplay patterns, **I want** sample projects to include
annotated logic graphs with explanatory comments on each node group, **so that** I can
understand the design decisions behind each gameplay system and adapt them for my project.

## US-15.19.13 Modder Reads Mod SDK Documentation

**As a** modder (P-24) learning the mod SDK, **I want** to browse node documentation in the
graph editor by hovering over nodes to see descriptions, port types, valid ranges, and example
mini-graphs, **so that** I can build mod logic without access to engine source code.

## US-15.19.14 Modder Watches Offline Tutorial Video

**As a** modder (P-24) with limited internet access, **I want** tutorial videos I have
previously watched to be cached locally for offline playback, **so that** I can revisit
tutorials while working without an internet connection.
