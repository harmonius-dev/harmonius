# User Stories: Cloud AI Backend Integration

## F-15.23.1 AI Provider Abstraction

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.23.1.1 | designer (P-5)           |          |              |
| US-15.23.1.2 | environment artist (P-8) |          |              |
| US-15.23.1.3 | tech artist (P-13)       |          |              |
| US-15.23.1.4 | developer (P-15)         |          |              |
| US-15.23.1.5 | modder (P-24)            |          |              |
| US-15.23.1.6 | engine developer (P-26)  |          |              |

1. **US-15.23.1.1** — to choose between Claude, Copilot, Cursor, or a custom AI provider in the
   editor settings
   - **Acceptance:** I can use the provider my studio has already licensed
2. **US-15.23.1.2** — to switch AI providers without losing my current conversation or project
   settings
   - **Acceptance:** I can compare provider quality for my asset creation workflow
3. **US-15.23.1.3** — to register a custom AI provider adapter via the plugin API
   - **Acceptance:** my studio can use our self-hosted model endpoint with the same editor
     integration
4. **US-15.23.1.4** — a well-defined provider trait covering authentication, request formatting, and
   response parsing
   - **Acceptance:** I can implement adapters for new AI services without modifying engine internals
5. **US-15.23.1.5** — AI provider integration available in the mod SDK editor
   - **Acceptance:** I can use AI assistance for creating mod content with my own API key
6. **US-15.23.1.6** — each built-in provider adapter to handle authentication, request formatting,
   and response parsing independently
   - **Acceptance:** provider-specific quirks do not leak across adapters

## F-15.23.2 Customer Authentication

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.23.2.1 | designer (P-5)           |          |              |
| US-15.23.2.2 | environment artist (P-8) |          |              |
| US-15.23.2.3 | developer (P-15)         |          |              |
| US-15.23.2.4 | modder (P-24)            |          |              |
| US-15.23.2.5 | engine developer (P-26)  |          |              |

1. **US-15.23.2.1** — my AI API key stored in the platform keychain (Keychain on macOS, Credential
   Manager on Windows, libsecret on Linux)
   - **Acceptance:** my credentials are encrypted at rest and never stored in plain text project
     files
2. **US-15.23.2.2** — assurance that my API key is never transmitted to any server other than my
   chosen AI provider
   - **Acceptance:** my credentials are not exposed to the engine vendor or third parties
3. **US-15.23.2.3** — credentials loaded only into memory at editor startup and cleared on shutdown
   - **Acceptance:** API keys do not persist in swap files, crash dumps, or core dumps
4. **US-15.23.2.4** — to use my own API key for AI features in the mod SDK
   - **Acceptance:** AI costs are billed to my account and not to the game developer
5. **US-15.23.2.5** — platform-native keychain access via C ABIs (Security.framework on
   macOS, wincred on Windows, D-Bus for libsecret on Linux)
   - **Acceptance:** credential storage integrates with each OS's security model

## F-15.23.3 AI Code Assistant

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.23.3.1 | designer (P-5)          |          |              |
| US-15.23.3.2 | tech artist (P-13)      |          |              |
| US-15.23.3.3 | developer (P-15)        |          |              |
| US-15.23.3.4 | engine developer (P-26) |          |              |

1. **US-15.23.3.1** — the AI assistant to suggest visual graph nodes and wiring based on my current
   graph state and selected entities
   - **Acceptance:** I can build gameplay logic faster without writing code
2. **US-15.23.3.2** — AI suggestions displayed as ghost nodes in the graph editor that I can accept
   or dismiss
   - **Acceptance:** I can evaluate suggestions before committing them
3. **US-15.23.3.3** — the AI code assistant to suggest only visual graph nodes (never text code)
   - **Acceptance:** the no-code constraint is enforced even when AI generates suggestions
4. **US-15.23.3.4** — the code assistant to read the current graph, selected entities, ECS component
   definitions, and recent changes for context
   - **Acceptance:** suggestions are relevant to the user's current work

## F-15.23.4 AI Content Assistant

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.23.4.1 | designer (P-5)           |          |              |
| US-15.23.4.2 | environment artist (P-8) |          |              |
| US-15.23.4.3 | tech artist (P-13)       |          |              |
| US-15.23.4.4 | modder (P-24)            |          |              |
| US-15.23.4.5 | engine developer (P-26)  |          |              |

1. **US-15.23.4.1** — to describe a level layout in natural language and have the AI generate a
   starting point imported through the asset pipeline
   - **Acceptance:** I can iterate on level designs faster
2. **US-15.23.4.2** — to generate textures via cloud AI with parameters I specify in the editor UI
   - **Acceptance:** I can rapidly prototype material variations without manual painting
3. **US-15.23.4.3** — every AI-generated asset to receive an automatic provenance tag (F-15.7.1)
   - **Acceptance:** the governance system tracks all AI content origin
4. **US-15.23.4.4** — to use AI content generation for textures and meshes in the mod SDK
   - **Acceptance:** I can create mod assets without professional DCC tools
5. **US-15.23.4.5** — AI-generated content returned as standard asset data and imported through the
   existing asset pipeline (F-12.1.1)
   - **Acceptance:** generated content follows the same processing, validation, and cooking path as
     manually created assets

## F-15.23.5 AI Chat Panel

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.23.5.1 | designer (P-5)           |          |              |
| US-15.23.5.2 | environment artist (P-8) |          |              |
| US-15.23.5.3 | tech artist (P-13)       |          |              |
| US-15.23.5.4 | developer (P-15)         |          |              |
| US-15.23.5.5 | modder (P-24)            |          |              |
| US-15.23.5.6 | engine developer (P-26)  |          |              |

1. **US-15.23.5.1** — a persistent chat panel docked in the editor where I can ask questions about
   the engine and get contextual suggestions
   - **Acceptance:** I do not need to leave the editor to search documentation
2. **US-15.23.5.2** — to describe a visual problem in the chat panel and receive diagnostic
   suggestions
   - **Acceptance:** I can troubleshoot material and lighting issues without engineering support
3. **US-15.23.5.3** — the chat panel to display token usage for the current session
   - **Acceptance:** I can monitor AI costs during my workflow
4. **US-15.23.5.4** — chat responses to include rich content (graph references, entity references,
   asset previews)
   - **Acceptance:** AI suggestions link directly to relevant editor objects
5. **US-15.23.5.5** — the chat panel to understand mod SDK constraints and suggest only features
   available in the mod editor
   - **Acceptance:** I receive relevant guidance for modding workflows
6. **US-15.23.5.6** — chat history stored per-project in local storage surviving editor restarts
   - **Acceptance:** users do not lose conversation context between sessions

## F-15.23.6 AI Provider Settings

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.23.6.1 | designer (P-5)           |          |              |
| US-15.23.6.2 | environment artist (P-8) |          |              |
| US-15.23.6.3 | tech artist (P-13)       |          |              |
| US-15.23.6.4 | developer (P-15)         |          |              |
| US-15.23.6.5 | engine developer (P-26)  |          |              |

1. **US-15.23.6.1** — to select the AI model (e.g., Claude Sonnet, Claude Opus) and adjust
   temperature and token limits per project
   - **Acceptance:** I can balance quality and cost for my workflow
2. **US-15.23.6.2** — a cost tracking dashboard showing token usage per session, per day, and
   cumulative per project with estimated cost
   - **Acceptance:** I can stay within my studio's AI budget
3. **US-15.23.6.3** — provider switching to preserve conversation context by converting history to
   the new provider's format
   - **Acceptance:** I can compare model quality without starting over
4. **US-15.23.6.4** — AI provider settings stored in the project file but excluded from version
   control by default
   - **Acceptance:** personal API preferences and key references are not leaked to the repository
5. **US-15.23.6.5** — provider settings stored per-project (not globally)
   - **Acceptance:** different projects can use different providers and models

## F-15.23.7 Offline/Local AI Fallback

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.23.7.1 | designer (P-5)           |          |              |
| US-15.23.7.2 | environment artist (P-8) |          |              |
| US-15.23.7.3 | tech artist (P-13)       |          |              |
| US-15.23.7.4 | modder (P-24)            |          |              |
| US-15.23.7.5 | engine developer (P-26)  |          |              |

1. **US-15.23.7.1** — the engine to function fully without any AI provider configured, with AI
   features degrading gracefully (no error dialogs)
   - **Acceptance:** AI is optional and never blocks my work
2. **US-15.23.7.2** — optional local inference models for basic AI assistance when cloud APIs are
   unreachable
   - **Acceptance:** I can get simple suggestions even when offline
3. **US-15.23.7.3** — local AI models bundled as optional downloads via the launcher (F-15.15.1)
   - **Acceptance:** I can install them only when I need offline AI capability
4. **US-15.23.7.4** — basic AI features (shortcut suggestions, simple graph completions) available
   via local models without requiring a cloud API account
   - **Acceptance:** I can benefit from AI assistance for free
5. **US-15.23.7.5** — local inference to use platform GPU compute (Metal on macOS, Vulkan compute on
   Windows/Linux) with CPU fallback
   - **Acceptance:** local models run efficiently on available hardware

## F-15.23.8 AI Context Injection

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.23.8.1 | designer (P-5)           |          |              |
| US-15.23.8.2 | environment artist (P-8) |          |              |
| US-15.23.8.3 | tech artist (P-13)       |          |              |
| US-15.23.8.4 | developer (P-15)         |          |              |
| US-15.23.8.5 | engine developer (P-26)  |          |              |

1. **US-15.23.8.1** — AI prompts automatically enriched with my selected entities, current graph
   state, and recent changes
   - **Acceptance:** AI responses are relevant to my current work without manual copy-paste
2. **US-15.23.8.2** — to grant or deny consent per session for each context category (entities,
   graphs, error logs, asset metadata) before any data is sent
   - **Acceptance:** I control exactly what project data leaves my machine
3. **US-15.23.8.3** — a context preview panel showing exactly what will be sent to the AI provider
   before each request
   - **Acceptance:** I can verify no sensitive data is included
4. **US-15.23.8.4** — the context injection system to support extensible context sources via the
   plugin API
   - **Acceptance:** custom plugins can contribute project-specific context to AI prompts
5. **US-15.23.8.5** — context injection to read recent changes from the undo stack (F-15.1.3) and
   error logs from the console
   - **Acceptance:** the AI can understand the user's recent actions and current problems
