# R-15.9 -- AI Editor Assistant Requirements

## Requirements

1. **R-15.9.1** — The engine **SHALL** capture voice input via platform-native audio APIs, process
   it through a configurable STT backend, and translate transcribed commands into editor tool
   invocations via an LLM agent with conversational context.
   - **Rationale:** Voice interaction enables hands-free editing for viewport-focused workflows.
   - **Verification:** Issue a voice command to place a light, follow up with "rotate it 10
     degrees," and verify the correct object is modified.

2. **R-15.9.2** — The engine **SHALL** expose every user-facing editor feature as a tool definition
   accessible to the AI assistant, with all assistant actions participating in the undo/redo stack.
   - **Rationale:** Complete tool coverage ensures the assistant has no capability gaps versus
     manual interaction.
   - **Verification:** Invoke an assistant tool action and verify it appears in the undo stack.

3. **R-15.9.3** — The engine **SHALL** enable the AI assistant to manipulate visual elements
   (material colors, graph nodes, splines, terrain, entities) using the same input-action APIs as
   manual input.
   - **Rationale:** API parity ensures assistant operations behave identically to manual operations.
   - **Verification:** Paint terrain via assistant and verify the result matches equivalent manual
     brush strokes.

4. **R-15.9.4** — The engine **SHALL** display shortcut tooltips when users perform actions via
   menus, shown at most once per shortcut per session with platform-appropriate modifier keys.
   - **Rationale:** Progressive discovery improves productivity without being intrusive.
   - **Verification:** Trigger a shortcutable action via menu twice and verify the tooltip appears
     only once.

5. **R-15.9.5** — The engine **SHALL** provide step-by-step visual overlays for help queries,
   highlighting target UI elements and fading after the guided action completes.
   - **Rationale:** Visual guidance is more effective than text for spatial editor workflows.
   - **Verification:** Rearrange panels and verify overlay positions match the correct widgets.

6. **R-15.9.6** — The engine **SHALL** provide a headless editor API with UI automation primitives,
   support for concurrent isolated AI agents, and CI/CD pipeline integration.
   - **Rationale:** Headless automation enables unattended content generation and testing in CI.
   - **Verification:** Run two concurrent agents and verify each has a fully isolated undo stack and
     selection state.

7. **R-15.9.7** — The engine **SHALL** capture screenshots and screen recordings using platform APIs
   (ScreenCaptureKit, DXGI, PipeWire), feeding captures into the AI visual pipeline.
   - **Rationale:** Visual context enables the assistant to diagnose material and lighting issues
     from screenshots.
   - **Verification:** Capture a screenshot and verify dimensions match the configured resolution.

8. **R-15.9.8** — The engine **SHALL** expose a UI accessibility tree with widget types, labels,
   values, and hierarchy via platform-native APIs, with queries completing within 5 ms.
   - **Rationale:** Structured UI state enables fast intent resolution without pixel analysis.
   - **Verification:** Query the accessibility tree and verify completion within 5 ms.

9. **R-15.9.9** — The engine **SHALL** combine voice, screenshots, accessibility tree, and
   conversation history for multi-modal intent resolution, with structured data taking precedence.
   - **Rationale:** Ambiguous commands require spatial and selection context for correct resolution.
   - **Verification:** Issue an ambiguous command and verify it resolves using the current selection
     context.

10. **R-15.9.10** — The engine **SHALL** support per-user tool invocation allowlists, usage quotas,
    rate limits, configurable AI provider selection, and a separate audit trail for assistant
    interactions.
    - **Rationale:** Enterprise environments require granular control over AI assistance access and
      cost.
    - **Verification:** Block a tool invocation via allowlist and verify the assistant rejects it.
