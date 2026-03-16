# R-15.9 -- AI Editor Assistant Requirements

## Voice Interaction

### R-15.9.1a Speech-to-Text Pipeline

The editor **SHALL** capture live voice input via platform-native audio APIs (TCC on macOS, WASAPI
on Windows, PipeWire on Linux) and process it through a configurable local or remote STT backend,
streaming transcription results with word-level timestamps.

- **Derived from:**
  [F-15.9.1a](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Voice input requires platform-native microphone access and configurable privacy
  tradeoffs.
- **Verification:** Integration test: speak a known command and verify transcription accuracy for
  common editor commands.

### R-15.9.1b Voice Command Interpretation

The editor **SHALL** translate natural language voice commands into structured editor tool
invocations via an LLM agent, maintaining conversational context so follow-up commands reference the
previously created or selected object.

- **Derived from:**
  [F-15.9.1b](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Natural language interaction removes the need for memorized command syntax.
- **Verification:** Unit test: issue a creation command followed by "rotate it 10 degrees" and
  verify the follow-up resolves to the previously created object.

### R-15.9.1c Voice Activation Modes

The editor **SHALL** support push-to-talk and always-listening voice activation modes, configurable
per user in editor preferences, with push-to-talk activating the microphone only while the
configured key is held.

- **Derived from:**
  [F-15.9.1c](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Different work environments require different activation modes.
- **Verification:** Unit test: verify push-to-talk captures audio only while the key is held and
  stops on release.

## Tool Interface

### R-15.9.2 AI Assistant Tool Interface

The editor **SHALL** expose every user-facing feature as a tool definition with parameter schemas
and validation, enabling the AI assistant to invoke any editor feature with all actions
participating in the undo/redo stack.

- **Derived from:**
  [F-15.9.2](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Complete tool coverage ensures the assistant has no capability gaps vs manual
  interaction.
- **Verification:** Unit test: verify every plugin API feature has a corresponding tool definition.
  Invoke a tool via the assistant and verify it appears in the undo stack.

### R-15.9.3 Visual and Graphical Tool Access

The editor **SHALL** enable the AI assistant to manipulate visual elements (material colors, graph
nodes, splines, terrain heightmaps, entity transforms) using the same input-action APIs as
mouse/keyboard, ensuring identical validation, snapping, and constraint behavior.

- **Derived from:**
  [F-15.9.3](../../features/tools-editor/ai-assistant.md)
- **Rationale:** API parity ensures assistant operations behave identically to manual operations.
- **Verification:** Unit test: perform a terrain paint via the assistant and verify it matches
  equivalent manual paint output.

## Recommendations

### R-15.9.4 Keyboard Shortcut Recommendations

The editor **SHALL** display shortcut tooltips when users perform actions via menus that have
keyboard shortcuts, shown at most once per shortcut per session, with platform-appropriate modifier
keys (Cmd/Ctrl).

- **Derived from:**
  [F-15.9.4](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Progressive shortcut discovery improves long-term productivity without being
  intrusive.
- **Verification:** Unit test: trigger a shortcutable action via menu twice and verify the tooltip
  appears only on the first occurrence.

### R-15.9.5 Contextual Action Reminders

The editor **SHALL** provide step-by-step visual overlays for "how do I" queries, highlighting
target UI elements with a pulse and fading after the guided action is completed, with overlay
positions correct regardless of panel layout.

- **Derived from:**
  [F-15.9.5](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Visual guidance is more effective than text-only help for spatial editor workflows.
- **Verification:** Unit test: rearrange panels and verify overlay positions match the correct
  widgets.

## Automation

### R-15.9.6a Headless Editor API

The editor **SHALL** provide a headless API mirroring the UI interaction model for automation, with
low-level UI primitives for widget interaction and a GPU context via EGL or headless Metal for CI
servers without a display.

- **Derived from:**
  [F-15.9.6a](../../features/tools-editor/ai-assistant.md)
- **Rationale:** CI pipelines require headless editor automation for content generation and testing.
- **Verification:** Integration test: run a headless operation and verify it produces identical
  results to the interactive workflow.

### R-15.9.6b Agent Orchestration

The editor **SHALL** support multiple concurrent AI agents with isolated contexts and independent
undo stacks, with programmatic lifecycle management (create, run, terminate).

- **Derived from:**
  [F-15.9.6b](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Parallel agent workflows require isolation to prevent cross-contamination.
- **Verification:** Unit test: run two agents concurrently and verify each has a fully isolated
  command context.

### R-15.9.6c CI/CD Agent Integration

The editor **SHALL** support agents running in CI pipelines for content generation and regression
testing, with build triggers and result reporting as headless API operations.

- **Derived from:**
  [F-15.9.6c](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Automated content production and testing must integrate into CI/CD workflows.
- **Verification:** Integration test: run a CI pipeline agent and verify it produces artifacts and
  reports results.

## Capture and Analysis

### R-15.9.7 Screenshot and Screen Recording Capture

The editor **SHALL** capture viewport screenshots and screen recordings using platform-specific APIs
(ScreenCaptureKit, DXGI, PipeWire), feeding captures into the AI visual understanding pipeline.

- **Derived from:**
  [F-15.9.7](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Visual context enables the assistant to diagnose lighting and material issues.
- **Verification:** Unit test: capture a screenshot and verify dimensions match configured settings.

### R-15.9.8 UI Accessibility Tree Analysis

The editor **SHALL** expose a UI accessibility tree with widget types, labels, values, and hierarchy
via platform-native APIs (NSAccessibility, UIA, AT-SPI2), with tree queries completing within 5 ms.

- **Derived from:**
  [F-15.9.8](../../features/tools-editor/ai-assistant.md)
- **Rationale:** The assistant needs structured UI state for intent resolution without pixel
  analysis.
- **Verification:** Benchmark: query the accessibility tree and verify completion within 5 ms.
  Verify the tree reflects current selection and panel state.

### R-15.9.9 Multi-Modal Understanding

The editor **SHALL** combine voice, screenshots, and selection state for intent resolution, with
structured data taking precedence over pixel analysis, and a unified context window for the LLM
agent.

- **Derived from:**
  [F-15.9.9](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Ambiguous voice commands require spatial and selection context for correct
  resolution.
- **Verification:** Unit test: issue an ambiguous voice command and verify it resolves correctly
  using the current selection context.

## Administration

### R-15.9.10 AI Assistance Administration

The engine **SHALL** support per-user and per-team voice control enable/disable, tool invocation
allowlists and blocklists, usage quotas and rate limits, configurable AI provider and model
selection, and a separate audit trail for assistant interactions.

- **Derived from:**
  [F-15.9.10](../../features/tools-editor/ai-assistant.md)
- **Rationale:** Enterprise environments require granular control over AI assistance access and
  cost.
- **Verification:** Unit test: block a tool invocation and verify the assistant rejects it.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/ai-assistant.md](../../user-stories/tools-editor/ai-assistant.md).
Requirements in this document are derived from those
user stories.
