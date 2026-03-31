# R-15.23 -- Cloud AI Backend Requirements

## Requirements

1. **R-15.23.1** — The engine **SHALL** provide a pluggable AI provider trait with built-in Claude,
   Copilot, and Cursor adapters and plugin API support for custom adapters.
   - **Rationale:** Provider abstraction avoids vendor lock-in.
   - **Verification:** Register a custom adapter and verify it handles authentication and requests.

2. **R-15.23.2** — The engine **SHALL** store AI credentials in the platform-native encrypted
   keychain, load them into memory at startup only, and transmit them only to the chosen provider
   endpoint.
   - **Rationale:** Credential security is non-negotiable; keys must never touch disk or third
     parties.
   - **Verification:** Verify the key is loaded from keychain, capture network traffic, and confirm
     only the configured provider receives it.

3. **R-15.23.3** — The engine **SHALL** provide context-aware AI graph node suggestions (not text
   code) displayed as ghost nodes with accept/dismiss controls.
   - **Rationale:** The no-code constraint requires AI to operate within the visual graph paradigm.
   - **Verification:** Trigger a suggestion and verify it is a graph node placement, not text code.

4. **R-15.23.4** — The engine **SHALL** support AI content generation for textures, meshes, levels,
   and dialogue, imported through the standard asset pipeline with automatic provenance tagging.
   - **Rationale:** Pipeline integration ensures AI content follows the same processing as manual
     assets.
   - **Verification:** Generate a texture, verify it imports through the pipeline with a provenance
     tag.

5. **R-15.23.5** — The engine **SHALL** provide a persistent per-project chat panel with token usage
   display, rich content responses, and context injection integration.
   - **Rationale:** A chat panel provides on-demand AI assistance without context switching.
   - **Verification:** Send a message, restart the editor, and verify conversation history is
     restored.

6. **R-15.23.6** — The engine **SHALL** support per-project AI provider selection with model,
   temperature, and token limit configuration, a cost tracking dashboard, and provider switching
   that preserves conversation context.
   - **Rationale:** Studios need cost visibility and flexibility to compare providers.
   - **Verification:** Switch providers and verify conversation context converts to the new format.

7. **R-15.23.7** — The engine **SHALL** function fully without any AI provider configured, degrading
   AI features gracefully, with optional local inference on platform GPU compute.
   - **Rationale:** AI must be optional; local fallback serves offline and cost-conscious studios.
   - **Verification:** Start the editor with no API key and verify all non-AI features work
     normally.

8. **R-15.23.8** — The engine **SHALL** auto-inject project context (entities, graphs, undo history,
   error logs) into AI prompts with per-session per-category consent and a preview showing exactly
   what will be sent.
   - **Rationale:** Context injection improves AI quality; consent and preview protect user data.
   - **Verification:** Enable entity context, verify the preview shows selected entities; disable it
     and verify exclusion.
