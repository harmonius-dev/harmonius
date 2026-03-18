# R-15.23 -- Cloud AI Backend Integration Requirements

## Provider Abstraction

| ID        | Derived From                                                 |
|-----------|--------------------------------------------------------------|
| R-15.23.1 | [F-15.23.1](../../features/tools-editor/ai-cloud-backend.md) |

1. **R-15.23.1** — The engine **SHALL** provide a pluggable backend trait for AI services with
   built-in adapters for Claude (Anthropic), Copilot (GitHub), and Cursor, and support for
   third-party adapters via the plugin API (F-13.1.10).
   - **Rationale:** Studios use different AI providers; a trait-based abstraction avoids lock-in and
     enables custom integrations.
   - **Verification:** Unit test: register a custom adapter via the plugin API and verify it handles
     authentication, request formatting, and response parsing.

## Customer Authentication

| ID        | Derived From                                                 |
|-----------|--------------------------------------------------------------|
| R-15.23.2 | [F-15.23.2](../../features/tools-editor/ai-cloud-backend.md) |

1. **R-15.23.2** — The engine **SHALL** store all AI API keys and OAuth tokens exclusively in the
   platform-native encrypted credential store (Keychain on macOS, Credential Manager on Windows,
   libsecret on Linux), load them only into memory at editor startup, and never transmit credentials
   to any server other than the customer's chosen AI provider endpoint.
   - **Rationale:** Customer credentials must be protected at rest and in transit; no engine
     telemetry or intermediary server may touch them.
   - **Verification:** Integration test: store a key in the platform keychain, restart the editor,
     verify the key is loaded from the keychain and only sent to the configured provider endpoint.

## AI Code Assistant

| ID        | Derived From                                                 |
|-----------|--------------------------------------------------------------|
| R-15.23.3 | [F-15.23.3](../../features/tools-editor/ai-cloud-backend.md) |

1. **R-15.23.3** — The engine **SHALL** provide context-aware AI suggestions in the editor that
   understand project structure, ECS components, and visual graph nodes, suggesting graph nodes and
   wiring (not text code) to respect the no-code constraint.
   - **Rationale:** The no-code engine requires AI assistance to operate within visual graph
     paradigms, not traditional code completion.
   - **Verification:** Unit test: open a logic graph, trigger AI suggestion, verify the response
     contains graph node suggestions (not text code).

## AI Content Assistant

| ID        | Derived From                                                 |
|-----------|--------------------------------------------------------------|
| R-15.23.4 | [F-15.23.4](../../features/tools-editor/ai-cloud-backend.md) |

1. **R-15.23.4** — The engine **SHALL** support AI-powered content creation for textures, meshes,
   levels, dialogue, and quests via cloud inference, importing results through the standard asset
   pipeline (F-12.1.1) with automatic provenance tagging (F-15.7.1).
   - **Rationale:** AI content generation accelerates production; pipeline integration and
     provenance tagging ensure governance compliance.
   - **Verification:** Integration test: generate a texture via AI, verify it imports through the
     asset pipeline with a provenance tag attached.

## AI Chat Panel

| ID        | Derived From                                                 |
|-----------|--------------------------------------------------------------|
| R-15.23.5 | [F-15.23.5](../../features/tools-editor/ai-cloud-backend.md) |

1. **R-15.23.5** — The engine **SHALL** provide a persistent per-project chat panel in the editor
   with conversation history surviving editor restarts, token usage display, rich content rendering,
   and context injection integration (F-15.23.8).
   - **Rationale:** A persistent chat interface lets users ask questions, debug issues, and get
     suggestions without losing context across sessions.
   - **Verification:** Integration test: send a chat message, restart the editor, verify
     conversation history is restored and token usage is displayed.

## AI Provider Settings

| ID        | Derived From                                                 |
|-----------|--------------------------------------------------------------|
| R-15.23.6 | [F-15.23.6](../../features/tools-editor/ai-cloud-backend.md) |

1. **R-15.23.6** — The engine **SHALL** provide per-project AI provider selection with configurable
   model, temperature, and token limits, a cost tracking dashboard (per session, per day, cumulative
   per project), and provider switching that preserves conversation context.
   - **Rationale:** Studios need cost visibility and the ability to switch providers without losing
     work context.
   - **Verification:** Unit test: switch providers mid-conversation, verify history is converted to
     the new provider's format. Verify cost tracking updates after each request.

## Offline / Local AI Fallback

| ID        | Derived From                                                 |
|-----------|--------------------------------------------------------------|
| R-15.23.7 | [F-15.23.7](../../features/tools-editor/ai-cloud-backend.md) |

1. **R-15.23.7** — The engine **SHALL** function fully without any AI provider configured, degrading
   AI features gracefully rather than producing errors, and **SHALL** support optional local
   inference models for basic assistance when cloud APIs are unreachable.
   - **Rationale:** The engine must never require cloud AI to operate; local fallback serves
     air-gapped and cost-conscious studios.
   - **Verification:** Unit test: start editor with no API key configured, verify all non-AI
     features work and AI features show graceful degradation UI. Integration test: disconnect
     network, verify local model provides basic suggestions.

## AI Context Injection

| ID        | Derived From                                                 |
|-----------|--------------------------------------------------------------|
| R-15.23.8 | [F-15.23.8](../../features/tools-editor/ai-cloud-backend.md) |

1. **R-15.23.8** — The engine **SHALL** automatically provide project context (selected entities,
   current graph, recent undo history, error logs, asset metadata) to AI prompts with per-session
   per-category user consent, and **SHALL** display a preview of exactly what will be sent before
   each request.
   - **Rationale:** Project-aware prompts improve AI response quality; consent and preview ensure
     users control what data leaves their machine.
   - **Verification:** Unit test: enable entity context, verify the context preview shows selected
     entity data. Disable entity context, verify entity data is excluded from the preview.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/ai-cloud-backend.md](../../user-stories/tools-editor/ai-cloud-backend.md).
Requirements in this document are derived from those user stories.
