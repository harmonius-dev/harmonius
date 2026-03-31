# User Stories -- 15.23 Cloud AI Backend

## Stories

| ID           | Persona                    |
|--------------|----------------------------|
| US-15.23.1.1 | game designer (P-5)        |
| US-15.23.1.2 | extension developer (P-25) |
| US-15.23.2.1 | game designer (P-5)        |
| US-15.23.2.2 | engine developer (P-26)    |
| US-15.23.3.1 | game designer (P-5)        |
| US-15.23.3.2 | technical artist (P-13)    |
| US-15.23.4.1 | game designer (P-5)        |
| US-15.23.4.2 | technical artist (P-13)    |
| US-15.23.5.1 | game designer (P-5)        |
| US-15.23.5.2 | engine developer (P-26)    |
| US-15.23.6.1 | game designer (P-5)        |
| US-15.23.6.2 | technical artist (P-13)    |
| US-15.23.7.1 | game designer (P-5)        |
| US-15.23.7.2 | engine developer (P-26)    |
| US-15.23.8.1 | game designer (P-5)        |
| US-15.23.8.2 | engine developer (P-26)    |

1. **US-15.23.1.1** — **As a** game designer (P-5), **I want** to choose between Claude, Copilot,
   Cursor, or a custom AI provider, **so that** I use the provider my studio licenses.

2. **US-15.23.1.2** — **As a** extension developer (P-25), **I want** to register custom AI provider
   adapters via the plugin API, **so that** self-hosted models integrate with the same editor
   interface.

3. **US-15.23.2.1** — **As a** game designer (P-5), **I want** my API key stored in the platform
   keychain and never transmitted except to my chosen provider, **so that** credentials are secure.

4. **US-15.23.2.2** — **As a** engine developer (P-26), **I want** credentials loaded only into
   memory at startup and cleared on shutdown, **so that** keys do not persist in swap or dumps.

5. **US-15.23.3.1** — **As a** game designer (P-5), **I want** context-aware graph node suggestions
   displayed as ghost nodes I can accept or dismiss, **so that** I build logic faster.

6. **US-15.23.3.2** — **As a** technical artist (P-13), **I want** the code assistant to suggest
   only graph nodes (never text code), **so that** the no-code constraint is enforced.

7. **US-15.23.4.1** — **As a** game designer (P-5), **I want** AI-generated textures, meshes,
   levels, and dialogue imported through the standard asset pipeline, **so that** generated content
   is treated like any other asset.

8. **US-15.23.4.2** — **As a** technical artist (P-13), **I want** automatic provenance tagging on
   all AI-generated content, **so that** governance tracks all AI output.

9. **US-15.23.5.1** — **As a** game designer (P-5), **I want** a persistent chat panel with token
   usage display and rich content responses, **so that** I get AI help without leaving the editor.

10. **US-15.23.5.2** — **As a** engine developer (P-26), **I want** chat history stored per-project
    surviving editor restarts, **so that** conversation context is preserved.

11. **US-15.23.6.1** — **As a** game designer (P-5), **I want** per-project provider selection with
    model, temperature, and token limit configuration, **so that** I balance quality and cost.

12. **US-15.23.6.2** — **As a** technical artist (P-13), **I want** a cost tracking dashboard per
    session and project, **so that** I stay within my AI budget.

13. **US-15.23.7.1** — **As a** game designer (P-5), **I want** the engine to function fully without
    any AI provider and degrade AI features gracefully, **so that** AI is never a blocker.

14. **US-15.23.7.2** — **As a** engine developer (P-26), **I want** optional local inference models
    using platform GPU compute, **so that** basic AI works offline.

15. **US-15.23.8.1** — **As a** game designer (P-5), **I want** AI prompts auto-enriched with my
    selected entities, graph state, and recent changes, **so that** responses are relevant without
    manual context.

16. **US-15.23.8.2** — **As a** engine developer (P-26), **I want** per-session consent controls and
    a context preview before each request, **so that** users control what data leaves their machine.
