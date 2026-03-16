# 15.23 -- Cloud AI Backend Integration

Cloud AI backend integration enables the editor to call external AI services using the customer's
own API keys. The engine is a thin client: it formats requests, sends them to the provider's API,
and parses responses. No intermediary server, no proxy, no reselling.

## Provider Abstraction

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.23.1 | AI Provider Abstraction | A pluggable backend trait for AI services. Ships with Claude (Anthropic), Copilot (GitHub), and Cursor adapters. Third-party developers can implement the trait via the plugin API (F-13.1.10) to add any provider. Each adapter handles authentication, request formatting, and response parsing for its provider. Multiple adapters can be registered simultaneously, with one active per project. | R-15.23.1 | F-13.1.10, F-15.7.4 (AI Toggle) | None |

## Customer Authentication

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.23.2 | Customer Authentication | All AI requests use the customer's own API key or OAuth token. Credentials are stored in the platform-native encrypted credential store: Keychain on macOS, Credential Manager on Windows, libsecret on Linux. Keys are loaded at editor startup and held only in memory during the session. Credentials are never transmitted to any server other than the customer's chosen AI provider endpoint. No engine telemetry or analytics includes credential material. | R-15.23.2 | F-15.23.1 | Keychain on macOS via Security.framework (cxx.rs bridge). Credential Manager on Windows via wincred. libsecret on Linux via D-Bus. |

## AI Code Assistant

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.23.3 | AI Code Assistant | Context-aware suggestions in the editor that understand the project structure, ECS components, and visual graph nodes. Because the engine enforces the no-code constraint, the assistant suggests visual graph nodes and wiring rather than text code. The assistant reads the current graph, selected entities, and recent changes to provide relevant suggestions. Suggestions appear as ghost nodes in the graph editor that the user can accept or dismiss. | R-15.23.3 | F-15.23.1, F-15.23.8, F-15.8.4 (Logic Graphs) | Desktop only. Not available in runtime. |

## AI Content Assistant

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.23.4 | AI Content Assistant | AI-powered content creation for textures, meshes, levels, dialogue, and quests using cloud inference. Generated content is returned as standard asset data and imported through the existing asset import pipeline (F-12.1.1). Provenance tags (F-15.7.1) are attached automatically. The user specifies generation parameters via the editor UI, and the assistant formats the request for the active provider. | R-15.23.4 | F-15.23.1, F-12.1.1, F-15.7.1 | None |

## AI Chat Panel

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.23.5 | AI Chat Panel | A persistent chat interface docked in the editor. Users ask questions about the engine, debug issues, and get suggestions. Conversation history is stored per-project in local storage and survives editor restarts. The chat panel displays token usage for the current session and supports rich content (graphs, entity references, asset previews) in responses. The panel integrates with the context injection system (F-15.23.8) to automatically attach relevant project state. | R-15.23.5 | F-15.23.1, F-15.23.8 | Desktop only. Not available in runtime. |

## AI Provider Settings

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.23.6 | AI Provider Settings | Per-project AI provider selection with configurable model, temperature, and token limits. A cost tracking dashboard displays token usage per session, per day, and cumulative per project with estimated cost in the provider's pricing. Switching providers preserves conversation context by converting the history to the new provider's format. Settings are stored in the project file and excluded from version control by default to avoid leaking preferences. | R-15.23.6 | F-15.23.1, F-15.23.2 | None |

## Offline / Local AI Fallback

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.23.7 | Offline/Local AI Fallback | When no cloud API key is configured or the provider is unreachable, the editor falls back to local inference models for basic assistance. Local models are smaller and lower quality but provide shortcut suggestions, simple graph completions, and basic Q&A without any network dependency. Local models are bundled as optional downloads via the launcher (F-15.15.1). The engine functions fully without any AI provider configured -- AI features degrade gracefully rather than producing errors. | R-15.23.7 | F-15.23.1, F-15.15.1 | Local inference uses platform GPU compute: Metal on macOS, Vulkan compute on Windows/Linux. CPU fallback available. |

## AI Context Injection

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-15.23.8 | AI Context Injection | Automatically provides relevant project context to AI prompts without manual copy-paste. Context sources include: selected entities and their components, the current graph and visible nodes, recent changes from the undo stack, error logs from the console, and active asset metadata. The user grants consent per-session for each context category before any data is sent to the AI provider. A context preview panel shows exactly what will be sent before each request. | R-15.23.8 | F-15.23.1, F-15.1.3 (Undo/Redo), F-15.7.4 (AI Toggle) | None |
