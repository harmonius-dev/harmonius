# R-15.9 -- AI Editor Assistant User Stories

## US-15.9.1a Speech-to-Text Pipeline

### US-15.9.1a.1
As a **designer (P-5)**, I want live voice input captured from my microphone
so that I can issue commands by speaking.

### US-15.9.1a.2
As a **designer (P-5)**, I want word-level timestamps on transcription results
so that the system can precisely identify command boundaries.

### US-15.9.1a.3
As an **engine dev (P-26)**, I want configurable local or remote STT backend
so that teams choose between privacy and accuracy tradeoffs.

### US-15.9.1a.4
As an **engine dev (P-26)**, I want platform-specific audio capture (TCC, WASAPI,
PipeWire)
so that microphone access works natively on each platform.

### US-15.9.1a.5
As an **engine tester (P-27)**, I want to verify transcription accuracy for common
editor commands
so that STT quality is regression-tested.

---

## US-15.9.1b Voice Command Interpretation

### US-15.9.1b.1
As a **designer (P-5)**, I want natural language commands translated to editor actions
so that I can say "place a directional light" and have it created.

### US-15.9.1b.2
As a **designer (P-5)**, I want conversational context for follow-up commands
so that "rotate it 10 degrees" refers to the last created object.

### US-15.9.1b.3
As a **artist (P-8)**, I want voice commands for material parameter adjustment
so that I can say "make it more metallic" while viewing the preview.

### US-15.9.1b.4
As an **engine tester (P-27)**, I want to verify follow-up commands resolve to the
previously referenced object
so that conversational context is regression-tested.

---

## US-15.9.1c Voice Activation Modes

### US-15.9.1c.1
As a **designer (P-5)**, I want push-to-talk activation with a configurable key
so that the microphone activates only when I hold a key.

### US-15.9.1c.2
As a **designer (P-5)**, I want always-listening activation with wake word detection
so that I can speak commands hands-free.

### US-15.9.1c.3
As a **developer (P-15)**, I want mode selection stored in per-user preferences
so that each team member uses their preferred activation mode.

### US-15.9.1c.4
As an **engine tester (P-27)**, I want to verify push-to-talk only captures while
the key is held
so that activation mode correctness is regression-tested.

---

## US-15.9.2 AI Assistant Tool Interface

### US-15.9.2.1
As a **designer (P-5)**, I want the assistant to invoke any editor feature
so that voice commands cover the full editor capability set.

### US-15.9.2.2
As a **designer (P-5)**, I want assistant-driven actions in the undo stack
so that I can undo anything the assistant does.

### US-15.9.2.3
As a **developer (P-15)**, I want every editor feature exposed as a tool definition
so that the assistant has no capability gaps vs manual interaction.

### US-15.9.2.4
As a **developer (P-15)**, I want tool definitions with parameter schemas and validation
so that the assistant invokes features with correct parameters.

### US-15.9.2.5
As an **engine dev (P-26)**, I want tool definitions auto-generated from plugin API
metadata
so that assistant capabilities stay in sync with the editor.

### US-15.9.2.6
As an **engine tester (P-27)**, I want to verify every plugin API feature has a
corresponding tool definition
so that tool coverage is regression-tested.

---

## US-15.9.3 Visual and Graphical Tool Access

### US-15.9.3.1
As a **artist (P-8)**, I want the assistant to adjust material color parameters
so that I can refine materials via voice commands.

### US-15.9.3.2
As a **designer (P-5)**, I want the assistant to connect graph nodes
so that I can build logic graphs by describing connections.

### US-15.9.3.3
As a **designer (P-5)**, I want the assistant to reshape splines in the viewport
so that I can adjust road paths via voice instructions.

### US-15.9.3.4
As a **artist (P-8)**, I want the assistant to paint terrain heightmaps
so that I can describe terrain features and have them sculpted.

### US-15.9.3.5
As a **developer (P-15)**, I want assistant visual operations to use the same
input-action APIs as mouse/keyboard
so that validation and snapping behavior is identical.

### US-15.9.3.6
As an **engine tester (P-27)**, I want to verify assistant-driven terrain paint matches
equivalent manual paint
so that API parity is regression-tested.

---

## US-15.9.4 Keyboard Shortcut Recommendations

### US-15.9.4.1
As a **designer (P-5)**, I want shortcut tooltips when I use menus for shortcutable
actions
so that I discover faster workflows progressively.

### US-15.9.4.2
As a **designer (P-5)**, I want tooltips shown at most once per shortcut per session
so that recommendations do not become intrusive.

### US-15.9.4.3
As a **developer (P-15)**, I want platform-appropriate modifier keys in tooltips
(Cmd/Ctrl)
so that displayed shortcuts match my operating system.

### US-15.9.4.4
As an **engine tester (P-27)**, I want to verify tooltips do not reappear for the same
shortcut in one session
so that once-per-session behavior is regression-tested.

---

## US-15.9.5 Contextual Action Reminders

### US-15.9.5.1
As a **designer (P-5)**, I want step-by-step visual overlays for "how do I" queries
so that I learn editor features through guided walkthroughs.

### US-15.9.5.2
As a **designer (P-5)**, I want overlays that highlight target UI elements with a pulse
so that I can locate the correct button or panel.

### US-15.9.5.3
As a **designer (P-5)**, I want overlays that fade after completing the guided action
so that the assistant guidance is non-intrusive.

### US-15.9.5.4
As an **engine tester (P-27)**, I want to verify overlay positions match the correct
widgets regardless of panel layout
so that layout-independent guidance is regression-tested.

---

## US-15.9.6a Headless Editor API Layer

### US-15.9.6a.1
As a **DevOps (P-16)**, I want a headless API that mirrors the UI interaction model
so that I can automate editor operations without a display.

### US-15.9.6a.2
As a **DevOps (P-16)**, I want low-level UI automation primitives for widget interaction
so that automated scripts can open panels and enter values.

### US-15.9.6a.3
As an **engine dev (P-26)**, I want headless mode with a GPU context via EGL or
headless Metal
so that viewport operations work on CI servers without a display.

### US-15.9.6a.4
As an **engine tester (P-27)**, I want to verify headless operations produce identical
results to interactive workflows
so that headless parity is regression-tested.

---

## US-15.9.6b Agent Orchestration

### US-15.9.6b.1
As a **developer (P-15)**, I want multiple concurrent AI agents with isolated contexts
so that agents can work on different tasks independently.

### US-15.9.6b.2
As a **developer (P-15)**, I want independent undo stacks per agent
so that one agent's undo does not affect another.

### US-15.9.6b.3
As a **developer (P-15)**, I want agent lifecycle management (create, run, terminate)
so that agents are controlled programmatically.

### US-15.9.6b.4
As an **engine tester (P-27)**, I want to verify two concurrent agents have fully
isolated command contexts
so that agent isolation is regression-tested.

---

## US-15.9.6c CI/CD Agent Integration

### US-15.9.6c.1
As a **DevOps (P-16)**, I want agents that run in CI pipelines for content generation
so that automated asset production is part of the build process.

### US-15.9.6c.2
As a **DevOps (P-16)**, I want agents that execute regression tests headlessly
so that visual and functional tests run on build servers.

### US-15.9.6c.3
As a **DevOps (P-16)**, I want build triggers and result reporting as headless API ops
so that CI integration is seamless.

### US-15.9.6c.4
As an **engine tester (P-27)**, I want to verify a CI pipeline agent produces artifacts
and reports results
so that CI agent integration is regression-tested.

---

## US-15.9.7 Screenshot and Screen Recording Capture

### US-15.9.7.1
As a **designer (P-5)**, I want the assistant to capture viewport screenshots
so that it can analyze my scene for context-aware help.

### US-15.9.7.2
As a **designer (P-5)**, I want screen recordings saved to disk
so that I can share sessions for documentation or bug reports.

### US-15.9.7.3
As a **developer (P-15)**, I want captures fed into the AI visual understanding pipeline
so that the assistant can diagnose lighting and material issues.

### US-15.9.7.4
As an **engine dev (P-26)**, I want platform-specific capture APIs (ScreenCaptureKit,
DXGI, PipeWire)
so that capture is hardware-accelerated on each platform.

### US-15.9.7.5
As an **engine tester (P-27)**, I want to verify screenshot dimensions match configured
settings
so that capture correctness is regression-tested.

---

## US-15.9.8 UI Accessibility Tree Analysis

### US-15.9.8.1
As an **engine dev (P-26)**, I want the assistant to read the UI accessibility tree
so that it understands editor state without pixel analysis.

### US-15.9.8.2
As an **engine dev (P-26)**, I want the tree exposing widget types, labels, values,
and hierarchy
so that the assistant can identify active panels and selected objects.

### US-15.9.8.3
As an **engine dev (P-26)**, I want tree queries completing within 5ms
so that accessibility introspection is fast enough for real-time use.

### US-15.9.8.4
As an **engine dev (P-26)**, I want integration with NSAccessibility, UIA, and AT-SPI2
so that the tree uses platform-native accessibility APIs.

### US-15.9.8.5
As an **engine tester (P-27)**, I want to verify the tree reflects the current selection
and panel state
so that accessibility tree accuracy is regression-tested.

---

## US-15.9.9 Multi-Modal Understanding

### US-15.9.9.1
As a **designer (P-5)**, I want the assistant to combine voice, screenshots, and
selection state
so that "make that brighter" resolves correctly to the selected light.

### US-15.9.9.2
As a **designer (P-5)**, I want structured data to take precedence over pixel analysis
so that the assistant uses the most reliable input source.

### US-15.9.9.3
As an **engine dev (P-26)**, I want a unified context window for the LLM agent
so that all input modalities are available for intent resolution.

### US-15.9.9.4
As an **engine tester (P-27)**, I want to verify ambiguous voice commands resolve
correctly using selection context
so that multi-modal resolution is regression-tested.

---

## US-15.9.10 AI Assistance Administration

### US-15.9.10.1
As a **server admin (P-22)**, I want to enable or disable voice control per user or team
so that AI assistance is granted selectively.

### US-15.9.10.2
As a **server admin (P-22)**, I want tool invocation allowlists and blocklists
so that I can restrict which operations the assistant can perform.

### US-15.9.10.3
As a **server admin (P-22)**, I want per-user usage quotas and rate limits
so that AI service costs are controlled.

### US-15.9.10.4
As a **server admin (P-22)**, I want all assistant interactions logged to a separate
audit trail
so that AI assistance usage is auditable independently from content generation.

### US-15.9.10.5
As a **developer (P-15)**, I want configurable AI provider and model selection
so that I can choose the best LLM for my team's needs.

### US-15.9.10.6
As an **engine tester (P-27)**, I want to verify a blocked tool invocation is rejected
by the assistant
so that tool restriction enforcement is regression-tested.
