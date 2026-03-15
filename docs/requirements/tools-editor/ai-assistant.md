# R-15.9 -- AI Editor Assistant Requirements

## R-15.9.1a Speech-to-Text Pipeline

The engine **SHALL** capture live voice input from the system microphone and process it through
a configurable speech-to-text backend (local or remote), streaming transcription results with
word-level timestamps to the command interpreter.

- **Derived from:** [F-15.9.1a](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Reliable speech-to-text is the foundation of voice-controlled editing,
  requiring platform-specific audio capture and flexible backend selection.
- **Verification:** Speak a command and verify the transcription is accurate. Verify
  word-level timestamps are present. Test on macOS (TCC), Windows (WASAPI), and Linux
  (PipeWire/PulseAudio).

## R-15.9.1b Voice Command Interpretation

The engine **SHALL** translate transcribed voice input into structured editor tool invocations
via an LLM agent that maintains conversational context across follow-up commands.

- **Derived from:** [F-15.9.1b](../../features/tools-editor/ai-assistant.md)
- **Rationale:** LLM-based interpretation enables natural language commands and contextual
  follow-ups without requiring users to memorize rigid command syntax.
- **Verification:** Issue the voice command "place a directional light facing east at 45
  degrees" and verify the entity is created. Follow up with "rotate it 10 degrees north"
  and verify the same light is rotated.

## R-15.9.1c Voice Activation Modes

The engine **SHALL** support push-to-talk and always-listening voice activation modes,
configurable per user preference and stored in editor preferences.

- **Derived from:** [F-15.9.1c](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Different work environments require different activation modes: push-to-talk
  for noisy offices, always-listening for solo work.
- **Verification:** Enable push-to-talk; verify the microphone activates only while the
  configured key is held. Enable always-listening; verify commands are detected from
  continuous speech.

## R-15.9.2 AI Assistant Tool Interface

The engine **SHALL** expose every user-facing editor feature as a tool definition with parameter
schemas, validation rules, and undo integration so the AI assistant can invoke any editor operation
programmatically through a unified tool API.

- **Derived from:** [F-15.9.2](../../features/tools-editor/ai-assistant.md)
- **Rationale:** A complete tool interface ensures the assistant has no capability gaps relative to
  manual interaction, and undo integration prevents assistant-driven actions from being
  irreversible.
- **Verification:** Enumerate all editor features exposed via the plugin API (F-15.1.8) and verify
  each has a corresponding tool definition. Invoke a tool to create an entity, modify a material
  parameter, and trigger a build. Verify each action appears in the undo stack and can be undone.

## R-15.9.3 Visual and Graphical Tool Access

The engine **SHALL** allow the AI assistant to manipulate visual editor elements -- including
material color parameters, graph nodes, splines, terrain maps, and entity transforms -- through the
same input-action APIs used by mouse and keyboard input, with identical validation, snapping, and
constraint behavior.

- **Derived from:** [F-15.9.3](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Routing assistant actions through the same input-action pipeline guarantees
  consistent behavior and prevents the assistant from bypassing editor constraints.
- **Verification:** Use the assistant to connect two nodes in a logic graph and verify the
  connection passes validation. Use the assistant to paint a terrain heightmap region and verify
  the result matches an equivalent manual paint stroke. Confirm all operations are recorded as
  undoable commands.

## R-15.9.4 Keyboard Shortcut Recommendations

The engine **SHALL** display a transient, non-intrusive tooltip showing the keyboard shortcut when
the user performs an action through menus or toolbars that has a shortcut binding, shown at most
once per shortcut per session.

- **Derived from:** [F-15.9.4](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Progressive shortcut discovery accelerates user proficiency without requiring
  users to study keybinding documentation.
- **Verification:** Perform an action via the menu that has a keyboard shortcut. Verify a tooltip
  appears near the action site displaying the correct shortcut. Repeat the same menu action and
  verify the tooltip does not reappear in the same session. Verify the tooltip uses platform-
  appropriate modifier keys (Cmd on macOS, Ctrl on Windows/Linux).

## R-15.9.5 Contextual Action Reminders

The engine **SHALL** respond to user help queries with step-by-step visual overlays that highlight
target UI elements in sequence using the accessibility tree for widget location, with overlays that
fade after completion or dismissal.

- **Derived from:** [F-15.9.5](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Visual guided walkthroughs are more effective than text instructions for spatial
  editor tasks, reducing the learning curve for new users.
- **Verification:** Ask the assistant "how do I create a material?" and verify sequential highlight
  overlays appear on each required UI element. Verify overlays reference accurate widget
  positions regardless of panel layout. Verify overlays dismiss after the user completes the
  guided action.

## R-15.9.6a Headless Editor API Layer

The engine **SHALL** provide a headless API layer that mirrors the UI interaction model without
a visible display, exposing the same tool interface as the interactive assistant plus low-level
UI automation primitives for widget interaction.

- **Derived from:** [F-15.9.6a](../../features/tools-editor/ai-assistant.md)
- **Rationale:** A headless API enables programmatic editor control for automation scenarios
  where no display is available.
- **Verification:** Launch the editor in headless mode. Execute a script that creates an
  entity, sets its properties, and triggers a build. Verify the output matches the
  equivalent interactive workflow.

## R-15.9.6b Agent Orchestration

The engine **SHALL** support multiple concurrent AI agents operating in isolated command
contexts with independent undo stacks, selection state, and tool invocation history.

- **Derived from:** [F-15.9.6b](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Isolated agent contexts prevent cross-contamination when multiple agents
  operate concurrently on the same editor instance.
- **Verification:** Run two concurrent agents and verify their command contexts are isolated
  with no cross-contamination of state.

## R-15.9.6c CI/CD Agent Integration

The engine **SHALL** integrate headless agents with CI/CD pipelines for automated content
generation and regression testing, exposing build triggers, test execution, and result
reporting as headless API operations.

- **Derived from:** [F-15.9.6c](../../features/tools-editor/ai-assistant.md)
- **Rationale:** CI/CD integration enables automated asset production and regression testing
  as part of the standard build pipeline.
- **Verification:** Configure a CI pipeline that launches a headless agent, generates content,
  runs tests, and reports results. Verify the pipeline completes successfully.

## R-15.9.7 Screenshot and Screen Recording Capture

The engine **SHALL** capture screenshots and screen recordings of the editor viewport, individual
panels, or the full window, and feed captures into the AI visual understanding pipeline for
context-aware assistance.

- **Derived from:** [F-15.9.7](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Visual capture enables the assistant to diagnose spatial problems (lighting,
  material, layout) that are difficult to describe through structured data alone.
- **Verification:** Capture a screenshot of the viewport and verify the image dimensions and format
  match the configured settings. Start and stop a screen recording and verify the output file is
  playable. Submit a viewport capture to the AI pipeline and verify the assistant can identify
  visible objects in the scene.

## R-15.9.8 UI Accessibility Tree Analysis

The engine **SHALL** expose a structured accessibility tree of all editor panels, widgets, menus,
and tool modes -- including widget types, labels, values, enabled/disabled state, focus state, and
parent-child hierarchy -- readable by the AI assistant at interaction speed.

- **Derived from:** [F-15.9.8](../../features/tools-editor/ai-assistant.md)
- **Rationale:** A structured accessibility tree provides fast, reliable editor state introspection
  without the latency and ambiguity of pixel-level screenshot analysis.
- **Verification:** Query the accessibility tree and verify it contains entries for all visible
  panels and widgets. Select an object in a property grid and verify the tree reflects the
  selection. Verify tree queries complete within 5ms. Confirm integration with platform
  accessibility APIs (NSAccessibility, UIA, AT-SPI2).

## R-15.9.9 Multi-Modal Understanding

The engine **SHALL** combine voice input, screenshot analysis, accessibility tree state, and
conversation history into a unified context window for LLM-based intent resolution, with structured
data (accessibility tree, selection state) taking precedence over pixel analysis when both are
available.

- **Derived from:** [F-15.9.9](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Multi-modal context resolution enables the assistant to interpret ambiguous
  references like "make that brighter" by cross-referencing selection state with visual context.
- **Verification:** Select a light in the viewport and issue the voice command "make that
  brighter." Verify the assistant correctly identifies the selected light and increases its
  intensity. Verify that when accessibility tree data is available, it is prioritized over
  screenshot analysis for object identification.

## R-15.9.10 AI Assistance Administration

The engine **SHALL** provide administrator controls to enable or disable voice control per user or
team, restrict permitted tool invocations via allowlist or blocklist, configure the AI provider and
model, set usage quotas and rate limits, and log all assistant interactions to a separate audit
trail.

- **Derived from:** [F-15.9.10](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Enterprise deployments require granular control over AI capabilities, usage
  limits, and audit logging for cost management and compliance.
- **Verification:** Configure a tool blocklist excluding build operations. Verify the assistant
  rejects build invocation requests. Set a per-user quota of 10 requests per hour, exceed the
  quota, and verify the assistant returns a rate-limit response. Verify all interactions appear
  in the audit trail with timestamps, user identity, and tool invocations.
