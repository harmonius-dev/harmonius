# R-15.19 — Documentation and Learning Requirements

## API Reference

### R-15.19.1 Auto-Generated API Reference Coverage

100% of public API types, traits, functions, and methods across all engine crates **SHALL** have
doc comments. The CI pipeline **SHALL** enforce this by failing the build when any public item
lacks a doc comment (`#![deny(missing_docs)]`). The generated API reference **SHALL** be published
as a searchable static website with each engine release, and the in-editor help panel **SHALL**
open the relevant documentation page within 500 ms of clicking "?" on any inspector property or
node type.

- **Derived from:** [F-15.19.1](../../features/tools-editor/documentation.md)
- **Rationale:** Complete API documentation enables plugin developers and engine contributors to
  discover and use engine APIs without reading source code. CI enforcement prevents documentation
  from going stale.
- **Verification:** Run `cargo doc` with `#![deny(missing_docs)]` on all public crates; verify
  the build succeeds with zero warnings. Spot-check 20 randomly selected public types and confirm
  each has a description, parameter types, return type, and at least one example. Click "?" on 10
  different inspector properties in the editor and verify the help panel opens the correct page
  within 500 ms.

### R-15.19.2 Logic Graph Node Documentation Completeness

Every built-in logic graph node **SHALL** have a description, input/output port documentation with
types and valid ranges, and at least one usage example as an embedded mini-graph. The editor
**SHALL** display the node description in a tooltip on hover (within 200 ms) and show the full
documentation in a dedicated panel when the node is selected. Custom nodes **SHALL** inherit a
documentation template that prompts the author to fill in descriptions for each port.

- **Derived from:** [F-15.19.2](../../features/tools-editor/documentation.md)
- **Rationale:** Inline node documentation eliminates the need to leave the graph editor to look
  up node behavior, reducing context switching and errors.
- **Verification:** Enumerate all built-in nodes; verify each has a non-empty description, port
  documentation for every input and output, and at least one example graph. Hover over 10 nodes
  and verify tooltips appear within 200 ms. Create a custom node and verify the documentation
  template is presented with prompts for each port.

## In-Editor Tutorials

### R-15.19.3 Interactive Tutorial Coverage and Completion Time

The engine **SHALL** ship with at least 6 tutorial categories (Getting Started, Gameplay, Art,
Audio, Networking, Advanced), each completable in under 30 minutes by a first-time user following
the instructions. Each tutorial step **SHALL** highlight the relevant UI element with a spotlight
overlay and wait for the user to complete the action before advancing. Tutorials **SHALL** support
pause, resume, and restart. Tutorial assets **SHALL** be authored using the logic graph system,
enabling community-created tutorials.

- **Derived from:** [F-15.19.3](../../features/tools-editor/documentation.md)
- **Rationale:** Interactive tutorials reduce time-to-productivity for new users and provide a
  guided onboarding path that covers all major engine areas.
- **Verification:** Complete each of the 6 tutorial categories from a fresh project; record
  completion time and verify each finishes under 30 minutes. Verify spotlight overlays appear on
  the correct UI elements at each step. Pause, close the editor, reopen, resume, and verify
  progress is preserved. Restart a tutorial and verify it begins from step 1.

### R-15.19.4 Video Tutorial Offline Playback

The embedded video player **SHALL** support offline playback from a local cache. When a video is
first streamed, it **SHALL** be cached locally for subsequent offline viewing. The video panel
**SHALL** be dockable alongside the viewport. Chapter timestamps **SHALL** be clickable and
optionally open the relevant editor panel or tool. The video library **SHALL** be organized by
topic and difficulty level and accessible from the Help menu.

- **Derived from:** [F-15.19.4](../../features/tools-editor/documentation.md)
- **Rationale:** Many development environments have intermittent or restricted internet access.
  Local caching ensures tutorials remain accessible regardless of connectivity.
- **Verification:** Stream a tutorial video with internet connected; disconnect the network and
  replay the same video from cache; verify uninterrupted playback. Dock the video panel beside
  the viewport and verify both remain functional. Click a chapter timestamp and verify the
  linked editor panel opens. Browse the video library and verify entries are grouped by topic
  and difficulty.

### R-15.19.5 Contextual Help Coverage for Inspector Properties

100% of inspector properties **SHALL** have tooltips showing a short description, value type,
valid range, and default value. The tooltip **SHALL** appear within 200 ms of hovering. Every
editor panel, toolbar button, and menu item **SHALL** include contextual help accessible via
hover, "?" button, or F1 key. Help content **SHALL** be stored as localized string assets
and updatable independently of engine releases. A "What's This?" mode **SHALL** allow clicking
any UI element to see its full documentation.

- **Derived from:** [F-15.19.5](../../features/tools-editor/documentation.md)
- **Rationale:** Contextual help eliminates the need to search external documentation for
  common questions about property meanings, valid values, and effects.
- **Verification:** Enumerate all inspector properties across all built-in component types;
  hover over each and verify a tooltip appears within 200 ms containing a description, type,
  range, and default. Activate "What's This?" mode, click 10 different UI elements, and verify
  documentation is displayed for each. Change the locale and verify tooltips appear in the
  selected language.

## Sample Projects

### R-15.19.6 Genre Sample Project Coverage

The engine **SHALL** ship with at least 8 genre sample projects: 3D action/adventure, 2D
platformer, FPS, RTS, RPG, racing, VR experience, and survival. Each sample **SHALL** include
a playable build, annotated logic graphs with explanatory comments, an organized asset structure,
and a companion walkthrough document. Samples **SHALL** be downloadable from the project creation
wizard and the asset store.

- **Derived from:** [F-15.19.6](../../features/tools-editor/documentation.md)
- **Rationale:** Genre-specific samples demonstrate engine capabilities in realistic production
  contexts and provide starting points for new projects.
- **Verification:** Download each of the 8 sample projects from the creation wizard; open each
  and verify it loads without errors. Build and run each sample; verify gameplay is functional.
  Open 3 logic graphs per sample and verify explanatory comments are present. Verify each sample
  has a companion walkthrough document.

### R-15.19.7 Documentation Code Example CI Validation

100% of code examples in engine API documentation **SHALL** compile successfully against the
current engine version. Stale examples that fail to compile **SHALL** fail the CI build. The
CI pipeline **SHALL** extract and compile all doc-test examples as part of the standard test
suite. Visual examples (logic graph screenshots) **SHALL** be versioned alongside engine source
and updated by the project upgrade system.

- **Derived from:** [F-15.19.7](../../features/tools-editor/documentation.md)
- **Rationale:** Untested code examples become incorrect over time as APIs evolve. CI validation
  guarantees every published example works with the current engine version.
- **Verification:** Run `cargo test --doc` across all engine crates; verify all doc-test examples
  compile and pass. Intentionally break an API used in an example; verify the CI build fails
  with a clear error referencing the stale example. Fix the example and verify the build passes.
  Upgrade a project and verify visual examples are updated to match the new version.
