# User Stories -- 15.9 AI Editor Assistant

## Stories

| ID           | Persona                 |
|--------------|-------------------------|
| US-15.9.1.1  | game designer (P-5)     |
| US-15.9.1.2  | game designer (P-5)     |
| US-15.9.1.3  | game designer (P-5)     |
| US-15.9.2.1  | game designer (P-5)     |
| US-15.9.2.2  | engine developer (P-26) |
| US-15.9.3.1  | technical artist (P-13) |
| US-15.9.3.2  | level designer (P-6)    |
| US-15.9.4.1  | game designer (P-5)     |
| US-15.9.4.2  | game designer (P-5)     |
| US-15.9.5.1  | game designer (P-5)     |
| US-15.9.5.2  | game designer (P-5)     |
| US-15.9.6.1  | build engineer (P-16)   |
| US-15.9.6.2  | build engineer (P-16)   |
| US-15.9.6.3  | build engineer (P-16)   |
| US-15.9.7.1  | technical artist (P-13) |
| US-15.9.7.2  | engine developer (P-26) |
| US-15.9.8.1  | engine developer (P-26) |
| US-15.9.8.2  | game designer (P-5)     |
| US-15.9.9.1  | game designer (P-5)     |
| US-15.9.9.2  | technical artist (P-13) |
| US-15.9.10.1 | build engineer (P-16)   |
| US-15.9.10.2 | engine developer (P-26) |

1. **US-15.9.1.1** — **As a** game designer (P-5), **I want** voice input captured and transcribed
   with word-level timestamps via a configurable STT backend, **so that** I can issue commands
   hands-free.

2. **US-15.9.1.2** — **As a** game designer (P-5), **I want** natural language commands translated
   into editor tool invocations by an LLM with conversational context, **so that** follow-up
   commands reference previously created objects.

3. **US-15.9.1.3** — **As a** game designer (P-5), **I want** push-to-talk and always-listening
   activation modes configurable per user, **so that** the microphone activates only when I intend.

4. **US-15.9.2.1** — **As a** game designer (P-5), **I want** the assistant to access the full
   editor tool API covering scene, asset, graph, and build operations, **so that** any manual action
   can also be done via voice.

5. **US-15.9.2.2** — **As a** engine developer (P-26), **I want** all assistant-driven actions to
   participate in the undo/redo stack, **so that** AI changes are reversible.

6. **US-15.9.3.1** — **As a** technical artist (P-13), **I want** the assistant to adjust material
   colors, move graph nodes, and reshape splines using input-action APIs, **so that** AI operations
   follow the same constraints as manual edits.

7. **US-15.9.3.2** — **As a** level designer (P-6), **I want** the assistant to paint terrain and
   place entities, **so that** I can direct edits verbally while focusing on the viewport.

8. **US-15.9.4.1** — **As a** game designer (P-5), **I want** a subtle tooltip showing keyboard
   shortcuts when I perform actions via menus, **so that** I learn faster workflows.

9. **US-15.9.4.2** — **As a** game designer (P-5), **I want** shortcut tips shown at most once per
   session per shortcut, **so that** recommendations are helpful not annoying.

10. **US-15.9.5.1** — **As a** game designer (P-5), **I want** step-by-step visual overlays for help
    queries highlighting target UI elements, **so that** I learn by following guided instructions.

11. **US-15.9.5.2** — **As a** game designer (P-5), **I want** overlays to fade after I complete the
    guided action, **so that** help does not permanently obscure my workspace.

12. **US-15.9.6.1** — **As a** build engineer (P-16), **I want** a headless editor API layer for
    CI/CD automation, **so that** editor operations run on build servers without a display.

13. **US-15.9.6.2** — **As a** build engineer (P-16), **I want** multiple concurrent AI agents with
    isolated command contexts, **so that** parallel automation tasks do not interfere.

14. **US-15.9.6.3** — **As a** build engineer (P-16), **I want** agents integrated into CI pipelines
    for automated content generation and regression testing, **so that** repetitive tasks run
    unattended.

15. **US-15.9.7.1** — **As a** technical artist (P-13), **I want** the assistant to capture
    screenshots fed into the AI visual pipeline, **so that** it can diagnose material issues.

16. **US-15.9.7.2** — **As a** engine developer (P-26), **I want** screen capture to use platform
    APIs with proper OS permissions, **so that** capture works without security violations.

17. **US-15.9.8.1** — **As a** engine developer (P-26), **I want** the assistant to read the UI
    accessibility tree for structured widget state, **so that** it resolves context without relying
    on pixel analysis.

18. **US-15.9.8.2** — **As a** game designer (P-5), **I want** the assistant to know which panel is
    active and what is selected, **so that** its responses are contextually accurate.

19. **US-15.9.9.1** — **As a** game designer (P-5), **I want** the assistant to combine voice,
    screenshots, accessibility tree, and conversation history for intent resolution, **so that**
    ambiguous commands like "make that brighter" resolve correctly.

20. **US-15.9.9.2** — **As a** technical artist (P-13), **I want** structured data to take
    precedence over pixel analysis, **so that** assistant responses are fast and accurate.

21. **US-15.9.10.1** — **As a** build engineer (P-16), **I want** per-user tool invocation
    allowlists, usage quotas, and rate limits, **so that** AI service costs are controlled.

22. **US-15.9.10.2** — **As a** engine developer (P-26), **I want** assistant interaction logs
    stored in a separate audit trail from content generation logs, **so that** compliance review is
    independent.
