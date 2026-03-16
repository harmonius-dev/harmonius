# User Stories: AI Editor Assistant

## F-15.9.1a Speech-to-Text Pipeline

## US-15.9.1a.1 Designer Speaks Commands to Editor

**As a** designer (P-5), **I want** live voice input captured by my microphone and processed through
a speech-to-text pipeline with word-level timestamps, **so that** I can issue voice commands
hands-free while working in the viewport.

## US-15.9.1a.2 Developer Configures STT Backend

**As a** developer (P-15), **I want** speech-to-text processing on a configurable backend (local or
remote), **so that** studios can choose between on-premise processing for privacy or cloud
processing for accuracy.

## US-15.9.1a.3 Engine Dev Handles Platform Audio APIs

**As an** engine developer (P-26), **I want** microphone capture using platform-native audio APIs
(CoreAudio on macOS, WASAPI on Windows, PipeWire on Linux), **so that** voice input works reliably
on all supported platforms.

## US-15.9.1a.4 Engine Tester Validates Transcription Accuracy

**As an** engine tester (P-27), **I want** to verify that transcription results are streamed with
word-level timestamps at low latency, **so that** voice commands feel responsive.

## F-15.9.1b Voice Command Interpretation

## US-15.9.1b.1 Designer Issues Natural Language Commands

**As a** designer (P-5), **I want** transcribed voice input interpreted by an LLM agent that
translates natural language into editor tool invocations (e.g., "place a directional light facing
east at 45 degrees"), **so that** I can manipulate scenes without navigating menus.

## US-15.9.1b.2 Artist Uses Conversational Context

**As an** artist (P-8), **I want** the assistant to maintain conversational context so follow-up
commands like "rotate it 10 degrees north" reference the previously created or selected object,
**so that** I can work naturally without repeating context.

## US-15.9.1b.3 Creative Director Directs Scene Changes Verbally

**As a** creative director (P-2), **I want** to issue high-level verbal directions ("make this area
darker", "add fog to the valley"), **so that** I can guide art direction during reviews without
touching the editor controls.

## US-15.9.1b.4 Engine Tester Validates Command Parsing

**As an** engine tester (P-27), **I want** to verify that a standard set of voice commands are
parsed correctly into the expected tool invocations, **so that** voice control is reliable for
common operations.

## F-15.9.1c Voice Activation Modes

## US-15.9.1c.1 Designer Chooses Push-to-Talk

**As a** designer (P-5), **I want** push-to-talk activation with a configurable key, **so that** the
microphone only activates when I intend to speak a command.

## US-15.9.1c.2 Artist Uses Always-Listening Mode

**As an** artist (P-8), **I want** an always-listening mode with wake word or silence detection,
**so that** I can issue commands without pressing any key while my hands are on a pen tablet.

## US-15.9.1c.3 Tech Artist Saves Preference Per-User

**As a** tech artist (P-13), **I want** voice activation mode stored per-user in editor preferences,
**so that** each team member can choose the mode that suits their work environment.

## US-15.9.1c.4 Engine Tester Validates Mode Switching

**As an** engine tester (P-27), **I want** to verify that switching between push-to-talk and
always-listening modes works without restarting the editor, **so that** users can change modes
mid-session.

## F-15.9.2 AI Assistant Tool Interface

## US-15.9.2.1 Designer Gets Full Editor Access via AI

**As a** designer (P-5), **I want** the assistant to access the full editor tool API (scene
manipulation, asset management, graph editing, material adjustment), **so that** any editor action I
can perform manually can also be done via voice or text command.

## US-15.9.2.2 Developer Ensures Undo Integration

**As a** developer (P-15), **I want** all assistant-driven actions to participate in the standard
undo/redo stack, **so that** AI-initiated changes can be reverted just like manual edits.

## US-15.9.2.3 Tech Artist Validates Tool Definitions

**As a** tech artist (P-13), **I want** tool definitions generated from the same metadata as the
editor plugin API, **so that** the assistant's capabilities stay in sync with available editor
features.

## US-15.9.2.4 Engine Tester Validates Tool API Coverage

**As an** engine tester (P-27), **I want** to verify that every user-facing editor feature has a
corresponding tool definition accessible to the assistant, **so that** no editor capability is
missing from the AI tool interface.

## F-15.9.3 Visual and Graphical Tool Access

## US-15.9.3.1 Artist Adjusts Materials via AI

**As an** artist (P-8), **I want** the assistant to adjust color parameters in material editors,
move nodes in logic graphs, and reshape splines in the viewport, **so that** I can direct visual
changes verbally while focusing on the viewport.

## US-15.9.3.2 Designer Paints Terrain via AI

**As a** designer (P-5), **I want** the assistant to paint terrain heightmaps and splat maps using
the same input-action APIs as mouse input, **so that** AI-driven painting follows the same snapping
and constraint rules as manual painting.

## US-15.9.3.3 Tech Artist Automates Repetitive Visual Edits

**As a** tech artist (P-13), **I want** all visual manipulations performed by the assistant recorded
as undoable commands, **so that** I can undo AI-driven edits just like manual ones.

## US-15.9.3.4 Engine Tester Validates Manipulation Consistency

**As an** engine tester (P-27), **I want** to verify that assistant visual manipulations produce
identical results to manual mouse/keyboard input, **so that** AI editing is consistent with human
editing.

## F-15.9.4 Keyboard Shortcut Recommendations

## US-15.9.4.1 Designer Learns Shortcuts

**As a** designer (P-5), **I want** the assistant to display a subtle tooltip showing the keyboard
shortcut when I perform an action through menus, **so that** I gradually learn faster workflows
without interrupting my current task.

## US-15.9.4.2 Artist Controls Recommendation Frequency

**As an** artist (P-8), **I want** shortcut recommendations shown at most once per shortcut per
session and only when I repeat the slower interaction path, **so that** recommendations are helpful
rather than annoying.

## US-15.9.4.3 Developer Disables Recommendations

**As a** developer (P-15), **I want** to disable shortcut recommendations globally or per- category
in preferences, **so that** experienced users are not bothered by tips they already know.

## US-15.9.4.4 Engine Tester Validates Platform Adaptation

**As an** engine tester (P-27), **I want** shortcut display to adapt to platform modifier key
conventions (Cmd on macOS, Ctrl on Windows/Linux), **so that** recommendations are correct on each
platform.

## F-15.9.5 Contextual Action Reminders

## US-15.9.5.1 Designer Gets Step-by-Step Overlays

**As a** designer (P-5), **I want** "how do I..." queries answered with step-by-step visual overlays
that highlight relevant UI elements in sequence, **so that** I can learn workflows by following
guided visual instructions.

## US-15.9.5.2 Artist Dismisses Overlays

**As an** artist (P-8), **I want** overlays to fade after I complete the guided action or explicitly
dismiss them, **so that** help overlays do not permanently obscure my workspace.

## US-15.9.5.3 Modder Gets Contextual Guidance

**As a** modder (P-24), **I want** contextual action reminders that work in the mod SDK editor,
**so that** I can learn the subset of available tools without separate documentation.

## US-15.9.5.4 Engine Tester Validates Widget Targeting

**As an** engine tester (P-27), **I want** to verify that overlay highlights target the correct UI
widgets regardless of panel layout, **so that** guided instructions work with any workspace
configuration.

## F-15.9.6a Headless Editor API Layer

## US-15.9.6a.1 DevOps Automates via Headless API

**As a** DevOps engineer (P-16), **I want** a headless API layer that mirrors the UI interaction
model without a visible display, **so that** I can automate editor operations in CI/CD pipelines.

## US-15.9.6a.2 Developer Drives UI Programmatically

**As a** developer (P-15), **I want** the headless layer to expose low-level UI automation
primitives (opening panels, clicking buttons, entering values), **so that** I can write automated
test scripts for editor features.

## US-15.9.6a.3 Engine Dev Provides GPU Context

**As an** engine developer (P-26), **I want** headless mode to use a virtual framebuffer (EGL on
Linux, headless Metal on macOS) for viewport operations, **so that** headless automation works on CI
servers without a display.

## US-15.9.6a.4 Engine Tester Validates Headless Parity

**As an** engine tester (P-27), **I want** to verify that headless API operations produce identical
results to interactive UI operations, **so that** automated tests are reliable proxies for manual
testing.

## F-15.9.6b Agent Orchestration

## US-15.9.6b.1 DevOps Runs Parallel Agents

**As a** DevOps engineer (P-16), **I want** multiple concurrent AI agents with isolated command
contexts (independent undo stacks, selection state, tool history), **so that** parallel automation
tasks do not interfere with each other.

## US-15.9.6b.2 Developer Manages Agent Lifecycle

**As a** developer (P-15), **I want** agent lifecycle management (create, run, terminate) through
the headless API, **so that** I can programmatically control automation workflows.

## US-15.9.6b.3 Engine Dev Ensures Context Isolation

**As an** engine developer (P-26), **I want** agents unable to observe or modify each other's
contexts, **so that** concurrent agents cannot cause race conditions or data corruption.

## US-15.9.6b.4 Engine Tester Validates Agent Isolation

**As an** engine tester (P-27), **I want** to verify that concurrent agents operate in fully
isolated contexts with no shared mutable state, **so that** parallel automation is safe and
deterministic.

## F-15.9.6c CI/CD Agent Integration

## US-15.9.6c.1 DevOps Triggers Agents from CI

**As a** DevOps engineer (P-16), **I want** agents to integrate with CI/CD pipelines for automated
content generation and regression testing, **so that** repetitive asset production runs unattended
as part of the build process.

## US-15.9.6c.2 Developer Generates Build Artifacts

**As a** developer (P-15), **I want** agents to produce build artifacts and test reports without
human interaction, **so that** automated builds include content validation.

## US-15.9.6c.3 Server Admin Monitors Agent Jobs

**As a** server admin (P-22), **I want** agent job status and results reported through CI
dashboards, **so that** I can monitor automated content generation alongside other build steps.

## US-15.9.6c.4 Engine Tester Validates CI Integration

**As an** engine tester (P-27), **I want** to verify that headless agents on CI servers with virtual
framebuffers produce the same results as interactive sessions, **so that** CI-generated content
matches developer expectations.

## F-15.9.7 Screenshot and Screen Recording Capture

## US-15.9.7.1 Artist Captures Screenshots for Documentation

**As an** artist (P-8), **I want** the assistant to capture screenshots of the viewport or specific
panels, **so that** I can create documentation and bug reports without third-party tools.

## US-15.9.7.2 Tech Artist Feeds Screenshots to AI

**As a** tech artist (P-13), **I want** captured screenshots fed into the AI's visual understanding
pipeline, **so that** the assistant can analyze a screenshot to diagnose lighting or material setup
issues.

## US-15.9.7.3 Developer Records Screen for Bug Reports

**As a** developer (P-15), **I want** screen recordings saved to disk with configurable resolution
and format, **so that** I can attach reproduction videos to bug reports.

## US-15.9.7.4 Engine Tester Validates Capture Permissions

**As an** engine tester (P-27), **I want** to verify that screen capture requests proper OS
permissions (TCC on macOS, DXGI on Windows, PipeWire on Linux), **so that** capture works without
security violations.

## F-15.9.8 UI Accessibility Tree Analysis

## US-15.9.8.1 Designer Gets Accurate AI Responses

**As a** designer (P-5), **I want** the assistant to read the UI accessibility tree to understand
which panel is active, what is selected, and which tool mode is engaged, **so that** the AI responds
accurately to my context without relying on screenshot analysis alone.

## US-15.9.8.2 Developer Integrates Platform Accessibility

**As a** developer (P-15), **I want** the accessibility tree to integrate with platform-native APIs
(NSAccessibility on macOS, UIA on Windows, AT-SPI2 on Linux), **so that** the AI leverages the same
accessibility infrastructure as screen readers.

## US-15.9.8.3 Engine Dev Exposes Widget Metadata

**As an** engine developer (P-26), **I want** the editor's custom widget toolkit to expose
accessibility metadata (widget type, text label, value, enabled state, hierarchy) on all platforms,
**so that** the accessibility tree is complete and accurate.

## US-15.9.8.4 Engine Tester Validates Tree Completeness

**As an** engine tester (P-27), **I want** to verify that every editor widget is represented in the
accessibility tree, **so that** the AI assistant and screen readers both have complete interface
information.

## F-15.9.9 Multi-Modal Understanding

## US-15.9.9.1 Designer Gets Context-Aware Responses

**As a** designer (P-5), **I want** the assistant to combine voice input, screenshot analysis,
accessibility tree state, and conversation history to resolve ambiguous references (e.g., "make that
brighter"), **so that** I can interact naturally without specifying every detail.

## US-15.9.9.2 Artist Relies on Structured Data Priority

**As an** artist (P-8), **I want** structured data (accessibility tree, selection state) to take
precedence over pixel analysis when both are available, **so that** the assistant's responses are
fast and accurate.

## US-15.9.9.3 Tech Artist Uses Combined Modalities

**As a** tech artist (P-13), **I want** the assistant to use viewport capture alongside the
accessibility tree to understand visual context that the tree alone cannot convey (e.g., "why does
this material look wrong?"), **so that** visual diagnosis is possible.

## US-15.9.9.4 Engine Tester Validates Modality Fallback

**As an** engine tester (P-27), **I want** to verify that the assistant degrades gracefully when a
modality is unavailable (e.g., no microphone), **so that** the system works with partial input
sources.

## F-15.9.10 AI Assistance Administration

## US-15.9.10.1 Server Admin Restricts Tool Access

**As a** server admin (P-22), **I want** to restrict which tools the assistant can invoke via an
allowlist or blocklist, **so that** sensitive operations are protected from AI-driven changes.

## US-15.9.10.2 DevOps Configures AI Provider

**As a** DevOps engineer (P-16), **I want** to configure the AI provider and model backing the
assistant, **so that** the studio can choose between cloud and on-premise LLM providers.

## US-15.9.10.3 Server Admin Sets Usage Quotas

**As a** server admin (P-22), **I want** per-user or per-team usage quotas (requests per hour,
tokens per day) with rate limits, **so that** AI service costs are controlled.

## US-15.9.10.4 Engine Tester Validates Audit Separation

**As an** engine tester (P-27), **I want** to verify that assistant interaction logs are stored in a
separate audit trail from content generation logs, **so that** compliance review of AI assistance is
independent from AI content review.
