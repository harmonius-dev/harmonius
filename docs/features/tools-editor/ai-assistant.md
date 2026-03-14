# 15.9 — AI Editor Assistant

## Voice Interaction

### F-15.9.1 Voice-Controlled Editor Interaction

Live voice input is captured by the system microphone, processed through a speech-to-text pipeline,
and interpreted by an LLM agent that translates natural language into editor tool invocations.
Commands such as "place a directional light facing east at 45 degrees" are parsed into structured
tool calls against the editor API. The assistant maintains conversational context so follow-up
commands like "rotate it 10 degrees north" reference the previously created or selected object.
Supports push-to-talk and always-listening activation modes, configurable per user preference.

- **Requirements:** R-15.9.1
- **Dependencies:** F-15.9.2, F-15.9.9, F-15.7.4
- **Platform notes:** On macOS, microphone access requires user permission via the TCC framework.
  On Windows, uses WASAPI for low-latency audio capture. On Linux, uses PipeWire or PulseAudio.
  Speech-to-text processing runs on a configurable backend (local or remote).

## Tool Interface

### F-15.9.2 AI Assistant Tool Interface

The assistant has access to the full editor tool API, covering scene manipulation, asset management,
PCG graph editing, logic graph editing, material parameter adjustment, animation preview and
scrubbing, and build system invocations. Every user-facing editor feature exposes a corresponding
tool definition that the assistant can invoke programmatically. Tool definitions include parameter
schemas, validation rules, and undo integration so that all assistant-driven actions participate
in the standard undo/redo stack (F-15.1.3).

- **Requirements:** R-15.9.2
- **Dependencies:** F-15.1.3, F-15.1.8, F-15.2.1, F-15.3.1, F-15.4.1
- **Platform notes:** Tool definitions are generated from the same metadata used by the editor
  plugin API (F-15.1.8), ensuring the assistant's capabilities stay in sync with the editor.

### F-15.9.3 Visual and Graphical Tool Access

The assistant can manipulate visual and graphical elements directly: adjusting color parameters in
material editors, moving and connecting nodes in logic and PCG graphs, reshaping splines in the
viewport, painting terrain heightmaps and splat maps, and placing or transforming entities. These
operations use the same underlying input-action APIs as mouse and keyboard input, ensuring
identical validation, snapping, and constraint behavior. All visual manipulations are recorded
as undoable commands.

- **Requirements:** R-15.9.3
- **Dependencies:** F-15.9.2, F-15.1.5, F-15.2.1, F-15.3.1, F-15.6.1
- **Platform notes:** None

## Recommendations

### F-15.9.4 Keyboard Shortcut Recommendations

When the assistant observes the user performing an action through menus, toolbars, or multi-click
sequences that has a keyboard shortcut, it displays a non-intrusive visual cue near the action
site. The cue is a subtle tooltip showing the shortcut binding, rendered silently with no audio
alert. Cues are transient — they fade after a few seconds — and are shown at most once per
shortcut per session unless the user repeats the slower interaction path. Users can disable
shortcut recommendations globally or per-category in preferences.

- **Requirements:** R-15.9.4
- **Dependencies:** F-15.1.7, F-15.7.4
- **Platform notes:** Shortcut display adapts to the current platform's modifier key conventions
  (Cmd on macOS, Ctrl on Windows/Linux).

### F-15.9.5 Contextual Action Reminders

When the user asks "how do I..." or similar help queries, the assistant responds with step-by-step
visual overlays that highlight the relevant UI elements in sequence rather than returning text-only
responses. Each step pulses the target button, menu item, or panel with a highlight ring and an
annotation label. Overlays fade after the user completes the guided action or explicitly dismisses
them. The overlay system reads the UI accessibility tree (F-15.9.8) to locate target widgets
accurately regardless of panel layout.

- **Requirements:** R-15.9.5
- **Dependencies:** F-15.9.8, F-15.9.9, F-15.7.4
- **Platform notes:** None

## Automation

### F-15.9.6 Agent-Driven Editor Automation

Generative AI agents can fully drive the editor programmatically: opening panels, clicking buttons,
entering values, navigating viewports, and triggering builds. Agents operate through a headless API
layer that mirrors the UI interaction model, enabling automated content generation, regression
testing, and CI/CD pipeline integration without a visible display. The headless layer exposes the
same tool interface as the interactive assistant (F-15.9.2) plus low-level UI automation primitives
for widget interaction. Multiple concurrent agents are supported with isolated command contexts.

- **Requirements:** R-15.9.6
- **Dependencies:** F-15.9.2, F-15.1.8
- **Platform notes:** Headless mode requires a GPU context for viewport operations. On CI servers
  without a display, a virtual framebuffer (EGL on Linux, headless Metal on macOS) is used.

### F-15.9.7 Screenshot and Screen Recording Capture

The assistant can capture screenshots and screen recordings of the editor viewport, individual
panels, or the full editor window. Captures are fed into the AI's visual understanding pipeline
for context-aware assistance — for example, analyzing a screenshot to diagnose a lighting or
material setup issue. Screen recordings can be saved to disk for documentation, bug reports, or
team sharing. Capture resolution and format are configurable, and recordings support annotation
overlays added after capture.

- **Requirements:** R-15.9.7
- **Dependencies:** F-15.9.9, F-15.1.2
- **Platform notes:** On macOS, screen capture uses ScreenCaptureKit and requires user permission
  via TCC. On Windows, uses DXGI Desktop Duplication or the Windows Graphics Capture API.
  On Linux, uses PipeWire screen capture or direct framebuffer readback.

## Interface Understanding

### F-15.9.8 UI Accessibility Tree Analysis

The assistant reads the editor's UI accessibility tree to obtain a fast, structured representation
of all panels, widgets, menus, and tool modes without relying solely on pixel-level screenshot
analysis. The accessibility tree exposes widget types, text labels, current values, enabled or
disabled state, focus state, and the full parent-child hierarchy. This allows the assistant to
understand the editor state at interaction speed — identifying which panel is active, what object
is selected in a property grid, and which tool mode is engaged — enabling precise tool
invocations and contextual responses.

- **Requirements:** R-15.9.8
- **Dependencies:** F-15.1.1, F-14.1.1
- **Platform notes:** On macOS, the accessibility tree integrates with NSAccessibility. On Windows,
  it integrates with UI Automation (UIA). On Linux, it integrates with AT-SPI2. The editor's
  custom widget toolkit exposes accessibility metadata on all platforms.

### F-15.9.9 Multi-Modal Understanding

The assistant combines voice input, screenshot analysis, accessibility tree state, and conversation
history to resolve ambiguous references and understand user intent in context. "Make that brighter"
is interpreted correctly because the assistant sees the currently selected light in the accessibility
tree and viewport capture simultaneously. Each input modality feeds into a unified context window
that the LLM agent uses for intent resolution. Modality weights are tuned so that structured data
(accessibility tree, selection state) takes precedence over pixel analysis when both are available.

- **Requirements:** R-15.9.9
- **Dependencies:** F-15.9.1, F-15.9.7, F-15.9.8
- **Platform notes:** None

## Administration

### F-15.9.10 AI Assistance Administration

AI assistance is governed by a dedicated toggle (F-15.7.4) that operates independently from the
generative AI content toggle (F-15.7.3). Administrators can enable or disable voice control
per user or team, restrict which tools the assistant is permitted to invoke via an allowlist or
blocklist, configure the AI provider and model backing the assistant, set per-user or per-team
usage quotas (requests per hour, tokens per day), and enforce rate limits. All assistant
interactions — voice transcripts, tool invocations, and responses — are logged to a separate
audit trail from content generation logs (F-15.7.6), enabling independent compliance review.

- **Requirements:** R-15.9.10
- **Dependencies:** F-15.7.3, F-15.7.4, F-15.7.5, F-15.7.6
- **Platform notes:** Usage quota enforcement requires connectivity to the admin policy server.
  Offline editors default to the last-synced policy and log interactions locally for deferred
  upload.
