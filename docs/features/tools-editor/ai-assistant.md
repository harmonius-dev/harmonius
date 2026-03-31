# 15.9 — AI Editor Assistant

## Voice Interaction

| ID | Feature |
| ----------- | ------------------------------ |
| F-15.9.1a | Speech-to-Text Pipeline |
| F-15.9.1b | Voice Command Interpretation |
| F-15.9.1c | Voice Activation Modes |

1. **F-15.9.1a** — Live voice input is captured by the system microphone and processed through a
   speech-to-text pipeline. On macOS, microphone access requires user permission via the TCC
   framework. On Windows, uses WASAPI for low-latency audio capture. On Linux, uses PipeWire or
   PulseAudio. Speech-to-text processing runs on a configurable backend (local or remote).
   Transcription results are streamed to the command interpreter with word-level timestamps.
   Windows, uses WASAPI. On Linux, uses PipeWire or PulseAudio.
   - **Deps:** F-15.7.4 (AI Toggle)
   - **Platform:** On macOS, microphone access requires user permission via the TCC framework. On
2. **F-15.9.1b** — Transcribed voice input is interpreted by an LLM agent that translates natural
   language into structured editor tool invocations. Commands such as "place a directional light
   facing east at 45 degrees" are parsed into tool calls against the editor API. The assistant
   maintains conversational context so follow-up commands like "rotate it 10 degrees north"
   reference the previously created or selected object.
   - **Deps:** F-15.9.1a, F-15.9.2, F-15.9.9
3. **F-15.9.1c** — Supports push-to-talk and always-listening activation modes, configurable per
   user preference. Push-to-talk requires holding a configurable key to activate the microphone.
   Always-listening uses a wake word or silence detection to segment commands. Mode selection is
   stored per-user in editor preferences.
   - **Deps:** F-15.9.1a

## Tool Interface

| ID | Feature |
| ---------- | ---------------------------------- |
| F-15.9.2 | AI Assistant Tool Interface |
| F-15.9.3 | Visual and Graphical Tool Access |

1. **F-15.9.2** — The assistant has access to the full editor tool API, covering scene manipulation,
   asset management, PCG graph editing, logic graph editing, material parameter adjustment,
   animation preview and scrubbing, and build system invocations. Every user-facing editor feature
   exposes a corresponding tool definition that the assistant can invoke programmatically. Tool
   definitions include parameter schemas, validation rules, and undo integration so that all
   assistant-driven actions participate in the standard undo/redo stack (F-15.1.3). plugin API
   (F-15.1.8), ensuring the assistant's capabilities stay in sync with the editor.
   - **Deps:** F-15.1.3, F-15.1.8, F-15.2.1, F-15.3.1, F-15.4.1
   - **Platform:** Tool definitions are generated from the same metadata used by the editor
2. **F-15.9.3** — The assistant can manipulate visual and graphical elements directly: adjusting
   color parameters in material editors, moving and connecting nodes in logic and PCG graphs,
   reshaping splines in the viewport, painting terrain heightmaps and splat maps, and placing or
   transforming entities. These operations use the same underlying input-action APIs as mouse and
   keyboard input, ensuring identical validation, snapping, and constraint behavior. All visual
   manipulations are recorded as undoable commands.
   - **Deps:** F-15.9.2, F-15.1.5, F-15.2.1, F-15.3.1, F-15.6.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Recommendations

| ID | Feature |
| ---------- | ----------------------------------- |
| F-15.9.4 | Keyboard Shortcut Recommendations |
| F-15.9.5 | Contextual Action Reminders |

1. **F-15.9.4** — When the assistant observes the user performing an action through menus, toolbars,
   or multi-click sequences that has a keyboard shortcut, it displays a non-intrusive visual cue
   near the action site. The cue is a subtle tooltip showing the shortcut binding, rendered silently
   with no audio alert. Cues are transient — they fade after a few seconds — and are shown at most
   once per shortcut per session unless the user repeats the slower interaction path. Users can
   disable shortcut recommendations globally or per-category in preferences. (Cmd on macOS, Ctrl on
   Windows/Linux).
   - **Deps:** F-15.1.7, F-15.7.4
   - **Platform:** Shortcut display adapts to the current platform's modifier key conventions
2. **F-15.9.5** — When the user asks "how do I..." or similar help queries, the assistant responds
   with step-by-step visual overlays that highlight the relevant UI elements in sequence rather than
   returning text-only responses. Each step pulses the target button, menu item, or panel with a
   highlight ring and an annotation label. Overlays fade after the user completes the guided action
   or explicitly dismisses them. The overlay system reads the UI accessibility tree (F-15.9.8) to
   locate target widgets accurately regardless of panel layout.
   - **Deps:** F-15.9.8, F-15.9.9, F-15.7.4
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Automation

| ID | Feature |
| ----------- | ----------------------------------------- |
| F-15.9.6a | Headless Editor API Layer |
| F-15.9.6b | Agent Orchestration |
| F-15.9.6c | CI/CD Agent Integration |
| F-15.9.7 | Screenshot and Screen Recording Capture |

1. **F-15.9.6a** — A headless API layer that mirrors the UI interaction model without a visible
   display. The headless layer exposes the same tool interface as the interactive assistant
   (F-15.9.2) plus low-level UI automation primitives for widget interaction (opening panels,
   clicking buttons, entering values, navigating viewports). without a display, a virtual
   framebuffer (EGL on Linux, headless Metal on macOS) is used.
   - **Deps:** F-15.9.2, F-15.1.8
   - **Platform:** Headless mode requires a GPU context for viewport operations. On CI servers
2. **F-15.9.6b** — Multiple concurrent AI agents are supported with isolated command contexts. Each
   agent operates in its own context with independent undo stacks, selection state, and tool
   invocation history. Agents cannot observe or modify each other's contexts. Agent lifecycle
   (create, run, terminate) is managed through the headless API.
   - **Deps:** F-15.9.6a
3. **F-15.9.6c** — Agents integrate with CI/CD pipelines for automated content generation and
   regression testing. Build triggers, test execution, and result reporting are exposed as headless
   API operations. Agents can run unattended on build servers, producing build artifacts and test
   reports without human interaction. macOS).
   - **Deps:** F-15.9.6a, F-15.9.6b
   - **Platform:** Requires headless GPU context on CI servers (EGL on Linux, headless Metal on
4. **F-15.9.7** — The assistant can capture screenshots and screen recordings of the editor
   viewport, individual panels, or the full editor window. Captures are fed into the AI's visual
   understanding pipeline for context-aware assistance — for example, analyzing a screenshot to
   diagnose a lighting or material setup issue. Screen recordings can be saved to disk for
   documentation, bug reports, or team sharing. Capture resolution and format are configurable, and
   recordings support annotation overlays added after capture. via TCC. On Windows, uses DXGI
   Desktop Duplication or the Windows Graphics Capture API. On Linux, uses PipeWire screen capture
   or direct framebuffer readback.
   - **Deps:** F-15.9.9, F-15.1.2
   - **Platform:** On macOS, screen capture uses ScreenCaptureKit and requires user permission

## Interface Understanding

| ID | Feature |
| ---------- | -------------------------------- |
| F-15.9.8 | UI Accessibility Tree Analysis |
| F-15.9.9 | Multi-Modal Understanding |

1. **F-15.9.8** — The assistant reads the editor's UI accessibility tree to obtain a fast,
   structured representation of all panels, widgets, menus, and tool modes without relying solely on
   pixel-level screenshot analysis. The accessibility tree exposes widget types, text labels,
   current values, enabled or disabled state, focus state, and the full parent-child hierarchy. This
   allows the assistant to understand the editor state at interaction speed — identifying which
   panel is active, what object is selected in a property grid, and which tool mode is engaged —
   enabling precise tool invocations and contextual responses. it integrates with UI Automation
   (UIA). On Linux, it integrates with AT-SPI2. The editor's custom widget toolkit exposes
   accessibility metadata on all platforms.
   - **Deps:** F-15.1.1, F-14.1.1
   - **Platform:** On macOS, the accessibility tree integrates with NSAccessibility. On Windows,
2. **F-15.9.9** — The assistant combines voice input, screenshot analysis, accessibility tree state,
   and conversation history to resolve ambiguous references and understand user intent in context.
   "Make that brighter" is interpreted correctly because the assistant sees the currently selected
   light in the accessibility tree and viewport capture simultaneously. Each input modality feeds
   into a unified context window that the LLM agent uses for intent resolution. Modality weights are
   tuned so that structured data (accessibility tree, selection state) takes precedence over pixel
   analysis when both are available.
   - **Deps:** F-15.9.1, F-15.9.7, F-15.9.8
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Administration

| ID | Feature |
| ----------- | ------------------------------ |
| F-15.9.10 | AI Assistance Administration |

1. **F-15.9.10** — AI assistance is governed by a dedicated toggle (F-15.7.4) that operates
   independently from the generative AI content toggle (F-15.7.3). Administrators can enable or
   disable voice control per user or team, restrict which tools the assistant is permitted to invoke
   via an allowlist or blocklist, configure the AI provider and model backing the assistant, set
   per-user or per-team usage quotas (requests per hour, tokens per day), and enforce rate limits.
   All assistant interactions — voice transcripts, tool invocations, and responses — are logged to a
   separate audit trail from content generation logs (F-15.7.6), enabling independent compliance
   review. Offline editors default to the last-synced policy and log interactions locally for
   deferred upload.
   - **Deps:** F-15.7.3, F-15.7.4, F-15.7.5, F-15.7.6
   - **Platform:** Usage quota enforcement requires connectivity to the admin policy server.
