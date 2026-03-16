# R-15.19 -- Documentation and Learning Requirements

## API Reference

### R-15.19.1 Auto-Generated API Reference

The engine **SHALL** auto-generate API documentation from
Rust doc comments as a searchable static website per
release, with in-editor "?" buttons opening the relevant
page, and CI enforcing #![deny(missing_docs)] on all
public items.

- **Derived from:**
  [F-15.19.1](../../features/tools-editor/documentation.md)
- **Rationale:** Complete, up-to-date API docs are
  essential for developer productivity.
- **Verification:** Benchmark: verify the in-editor help
  panel opens within 500 ms.

### R-15.19.2 Logic Graph Node Documentation

Every built-in logic graph node **SHALL** include a
description tooltip, input/output port documentation with
types and valid ranges, usage examples as embedded
mini-graphs, and a dedicated doc panel on selection.
Custom nodes **SHALL** inherit a documentation template.

- **Derived from:**
  [F-15.19.2](../../features/tools-editor/documentation.md)
- **Rationale:** No-code users need inline documentation
  to understand node behavior without external references.
- **Verification:** Unit test: verify all built-in nodes
  have description, port docs, and examples.

## Tutorials

### R-15.19.3 Interactive In-Editor Tutorials

The editor **SHALL** provide step-by-step guided tutorials
with spotlight overlays highlighting UI elements, waiting
for user completion per step, pause/resume/restart
controls, and community-created tutorials via the asset
store.

- **Derived from:**
  [F-15.19.3](../../features/tools-editor/documentation.md)
- **Rationale:** Hands-on learning inside the editor is
  more effective than external documentation.
- **Verification:** Benchmark: verify each tutorial
  category completes under 30 minutes.

### R-15.19.4 Video Tutorial Integration

The editor **SHALL** embed a dockable video player for
tutorials with chapter timestamps linking to editor panels,
and local caching for offline playback.

- **Derived from:**
  [F-15.19.4](../../features/tools-editor/documentation.md)
- **Rationale:** Video tutorials within the editor enable
  step-by-step following without context switching.
- **Verification:** Unit test: cache a video and verify
  playback works without network connectivity.

### R-15.19.5 Contextual Help and Tooltip System

The editor **SHALL** provide tooltips on every inspector
property showing description, type, range, and default,
with "?" button and F1 access to contextual help, a
"What's This?" mode for clicking any UI element, and help
content as localized string assets updated independently
of engine releases.

- **Derived from:**
  [F-15.19.5](../../features/tools-editor/documentation.md)
- **Rationale:** Contextual help at the point of use
  reduces documentation lookup time.
- **Verification:** Benchmark: verify tooltips appear
  within 200 ms on hover for all inspector properties.

## Sample Projects

### R-15.19.6 Sample Projects and Template Library

The engine **SHALL** provide sample projects for at least 8
game genres (action, platformer, FPS, RTS, RPG, racing, VR,
survival) with annotated logic graphs, playable builds,
and companion walkthrough documents.

- **Derived from:**
  [F-15.19.6](../../features/tools-editor/documentation.md)
- **Rationale:** Working reference projects accelerate
  learning across all game types.
- **Verification:** Integration test: verify each sample
  project loads and runs without errors.

### R-15.19.7 Inline Code Examples

API documentation **SHALL** include runnable code examples,
visual examples (logic graph screenshots) for no-code
users, CI-compiled doc-tests, and visual examples updated
by the project upgrade system.

- **Derived from:**
  [F-15.19.7](../../features/tools-editor/documentation.md)
- **Rationale:** Stale examples break trust in
  documentation; CI testing prevents this.
- **Verification:** Unit test: compile and run all
  doc-test examples and verify they pass.

---

## User Stories

## US-15.19.1 Auto-Generated API Reference

### US-15.19.1.1
As a **developer (P-15)**, I want auto-generated API docs from Rust doc comments so that I can
discover engine APIs without reading source code.

### US-15.19.1.2
As a **developer (P-15)**, I want a searchable static website published per release so that I can
find API documentation quickly.

### US-15.19.1.3
As a **developer (P-15)**, I want in-editor "?" buttons opening the relevant doc page so that I can
access documentation in context.

### US-15.19.1.4
As a **DevOps (P-16)**, I want CI enforcing #![deny(missing_docs)] on all public items so that
documentation completeness is guaranteed.

### US-15.19.1.5
As an **engine tester (P-27)**, I want to verify the in-editor help panel opens within 500ms so that
help response time is regression-tested.

---

## US-15.19.2 Logic Graph Node Documentation

### US-15.19.2.1
As a **designer (P-5)**, I want description tooltips when hovering over graph nodes so that I can
understand node behavior without leaving the editor.

### US-15.19.2.2
As a **designer (P-5)**, I want input/output port documentation with types and ranges so that I know
valid values for each node pin.

### US-15.19.2.3
As a **designer (P-5)**, I want usage examples as embedded mini-graphs so that I can see how nodes
are used in context.

### US-15.19.2.4
As a **designer (P-5)**, I want a dedicated doc panel when selecting a node so that I can read full
documentation alongside the graph.

### US-15.19.2.5
As a **developer (P-15)**, I want custom nodes to inherit a documentation template so that
user-created nodes are prompted for documentation.

### US-15.19.2.6
As an **engine tester (P-27)**, I want to verify all built-in nodes have description, port docs, and
examples so that node documentation completeness is regression-tested.

---

## US-15.19.3 Interactive In-Editor Tutorials

### US-15.19.3.1
As a **designer (P-5)**, I want step-by-step guided tutorials inside the editor so that I can learn
features through hands-on exercises.

### US-15.19.3.2
As a **designer (P-5)**, I want spotlight overlays highlighting UI elements at each step so that I
can locate the correct button or panel.

### US-15.19.3.3
As a **designer (P-5)**, I want tutorials that wait for me to complete each step so that I learn by
doing rather than just reading.

### US-15.19.3.4
As a **designer (P-5)**, I want pause, resume, and restart for tutorials so that I can learn at my
own pace.

### US-15.19.3.5
As a **modder (P-24)**, I want community-created tutorials via the asset store so that I can learn
from content created by other users.

### US-15.19.3.6
As a **creative director (P-2)**, I want tutorials covering all major areas (gameplay, art, audio,
networking, advanced) so that every team role has guided onboarding.

### US-15.19.3.7
As an **engine tester (P-27)**, I want to verify each tutorial category completes under 30 minutes
so that tutorial length is regression-tested.

---

## US-15.19.4 Video Tutorial Integration

### US-15.19.4.1
As a **designer (P-5)**, I want an embedded video player within the editor so that I can follow
tutorials without leaving the editor.

### US-15.19.4.2
As a **designer (P-5)**, I want the video panel dockable alongside the viewport so that I can follow
along step-by-step while working.

### US-15.19.4.3
As a **designer (P-5)**, I want chapter timestamps that link to editor panels so that clicking a
chapter opens the relevant tool.

### US-15.19.4.4
As a **developer (P-15)**, I want videos cached locally for offline playback so that tutorials work
without internet connectivity.

### US-15.19.4.5
As an **engine tester (P-27)**, I want to verify cached videos play without network connectivity so
that offline playback is regression-tested.

---

## US-15.19.5 Contextual Help and Tooltip System

### US-15.19.5.1
As a **designer (P-5)**, I want tooltips on every inspector property showing description, type,
range, and default so that I understand each property without external docs.

### US-15.19.5.2
As a **designer (P-5)**, I want "?" button and F1 key access to contextual help so that I can access
detailed documentation from any UI element.

### US-15.19.5.3
As a **designer (P-5)**, I want a "What's This?" mode for clicking any UI element so that I can
explore editor features by direct interaction.

### US-15.19.5.4
As a **developer (P-15)**, I want help content as localized string assets updated independently of
engine releases so that documentation stays current between versions.

### US-15.19.5.5
As an **engine tester (P-27)**, I want to verify tooltips appear within 200ms on hover for all
inspector properties so that tooltip response time is regression-tested.

---

## US-15.19.6 Sample Projects and Template Library

### US-15.19.6.1
As a **designer (P-5)**, I want sample projects for each major game genre so that I have a working
reference for my target game type.

### US-15.19.6.2
As a **designer (P-5)**, I want annotated logic graphs with explanatory comments so that I can learn
graph patterns from working examples.

### US-15.19.6.3
As a **developer (P-15)**, I want playable builds included with each sample so that I can test the
sample without building from source.

### US-15.19.6.4
As a **developer (P-15)**, I want companion walkthrough documents per sample so that key design
decisions are explained.

### US-15.19.6.5
As a **creative director (P-2)**, I want at least 8 genre samples (action, platformer, FPS, RTS,
RPG, racing, VR, survival) so that the engine demonstrates breadth across game types.

### US-15.19.6.6
As an **engine tester (P-27)**, I want to verify each sample project loads and runs without errors
so that sample project integrity is regression-tested.

---

## US-15.19.7 Inline Code Examples in Documentation

### US-15.19.7.1
As a **developer (P-15)**, I want runnable code examples in API documentation so that I can copy
working code directly from docs.

### US-15.19.7.2
As a **developer (P-15)**, I want visual examples (logic graph screenshots) for no-code users so
that non-programmers can follow along.

### US-15.19.7.3
As a **DevOps (P-16)**, I want code examples compiled as CI doc-tests so that stale examples fail
the build before shipping.

### US-15.19.7.4
As a **DevOps (P-16)**, I want visual examples updated by the project upgrade system so that
screenshots match the current engine version.

### US-15.19.7.5
As an **engine tester (P-27)**, I want to verify all doc-test examples compile and pass so that
example correctness is regression-tested.
