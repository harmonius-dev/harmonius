# User Stories -- 15.12 Remote Editing and Collaboration

## Stories

| ID            | Persona                    |
|---------------|----------------------------|
| US-15.12.1.1  | engine developer (P-26)    |
| US-15.12.1.2  | game designer (P-5)        |
| US-15.12.2.1  | engine developer (P-26)    |
| US-15.12.2.2  | build engineer (P-16)      |
| US-15.12.3.1  | level designer (P-6)       |
| US-15.12.3.2  | game designer (P-5)        |
| US-15.12.4.1  | build engineer (P-16)      |
| US-15.12.4.2  | engine developer (P-26)    |
| US-15.12.5.1  | game designer (P-5)        |
| US-15.12.5.2  | engine developer (P-26)    |
| US-15.12.6.1  | game designer (P-5)        |
| US-15.12.6.2  | technical artist (P-13)    |
| US-15.12.7.1  | build engineer (P-16)      |
| US-15.12.7.2  | engine developer (P-26)    |
| US-15.12.8.1  | engine developer (P-26)    |
| US-15.12.8.2  | game designer (P-5)        |
| US-15.12.9.1  | build engineer (P-16)      |
| US-15.12.9.2  | engine developer (P-26)    |
| US-15.12.10.1 | level designer (P-6)       |
| US-15.12.10.2 | game designer (P-5)        |
| US-15.12.11.1 | level designer (P-6)       |
| US-15.12.11.2 | game designer (P-5)        |
| US-15.12.12.1 | game designer (P-5)        |
| US-15.12.12.2 | build engineer (P-16)      |
| US-15.12.13.1 | level designer (P-6)       |
| US-15.12.13.2 | game designer (P-5)        |
| US-15.12.14.1 | engine developer (P-26)    |
| US-15.12.14.2 | extension developer (P-25) |

1. **US-15.12.1.1** — **As a** engine developer (P-26), **I want** the editor viewport streamed as
   compressed video with adaptive bitrate, **so that** remote work uses a thin client.

2. **US-15.12.1.2** — **As a** game designer (P-5), **I want** GPU-accelerated encoding below 2 ms
   per frame, **so that** remote editing feels responsive.

3. **US-15.12.2.1** — **As a** engine developer (P-26), **I want** a custom protocol prioritizing
   viewport at full quality and UI at lower rate, **so that** bandwidth drops 60-80% versus generic
   RDP.

4. **US-15.12.2.2** — **As a** build engineer (P-16), **I want** the protocol over QUIC with TLS 1.3
   TCP fallback, **so that** firewalled networks are supported.

5. **US-15.12.3.1** — **As a** level designer (P-6), **I want** CRDT-based real-time collaborative
   editing with per-user undo stacks, **so that** multiple designers edit the same world
   simultaneously.

6. **US-15.12.3.2** — **As a** game designer (P-5), **I want** presence indicators showing cursor
   positions and selections of all connected users, **so that** I know where teammates are working.

7. **US-15.12.4.1** — **As a** build engineer (P-16), **I want** the editor running headless on a
   remote GPU server streaming to thin clients, **so that** developers need no dedicated GPU.

8. **US-15.12.4.2** — **As a** engine developer (P-26), **I want** multiple concurrent sessions on a
   multi-GPU server, **so that** infrastructure cost is shared.

9. **US-15.12.5.1** — **As a** game designer (P-5), **I want** session suspend and resume preserving
   all editor state, **so that** I can start at the office and continue from home.

10. **US-15.12.5.2** — **As a** engine developer (P-26), **I want** session state serialized using
    the crash recovery format, **so that** resume restores the exact editor state.

11. **US-15.12.6.1** — **As a** game designer (P-5), **I want** automatic quality tier adjustment
    based on bandwidth, **so that** remote editing works from home broadband or mobile.

12. **US-15.12.6.2** — **As a** technical artist (P-13), **I want** to pin a specific quality level,
    **so that** I can force high-fidelity preview when bandwidth allows.

13. **US-15.12.7.1** — **As a** build engineer (P-16), **I want** a centralized collaboration cloud
    service managing CRDT sync, sessions, and presence, **so that** all team features are backed by
    a single backend.

14. **US-15.12.7.2** — **As a** engine developer (P-26), **I want** the service written in Rust with
    PostgreSQL and S3, **so that** it matches our stack and scales horizontally.

15. **US-15.12.8.1** — **As a** engine developer (P-26), **I want** CRDT document schemas per asset
    type (tree for scenes, op-log for graphs, map for tables), **so that** concurrent edits merge
    correctly.

16. **US-15.12.8.2** — **As a** game designer (P-5), **I want** editors to read/write through CRDT
    accessors transparently, **so that** collaboration is invisible to my workflow.

17. **US-15.12.9.1** — **As a** build engineer (P-16), **I want** per-project role-based access
    control enforced server-side, **so that** unauthorized users cannot edit.

18. **US-15.12.9.2** — **As a** engine developer (P-26), **I want** asset-level locks within
    collaborative sessions, **so that** critical assets can be exclusively edited.

19. **US-15.12.10.1** — **As a** level designer (P-6), **I want** built-in spatial and non-spatial
    voice chat with text chat, **so that** I can coordinate with teammates during editing.

20. **US-15.12.10.2** — **As a** game designer (P-5), **I want** text chat with threads, mentions,
    and inline asset references, **so that** conversations link to specific content.

21. **US-15.12.11.1** — **As a** level designer (P-6), **I want** named work groups with their own
    voice channel and isolated workspace layer, **so that** my team's edits are invisible to others
    until shared.

22. **US-15.12.11.2** — **As a** game designer (P-5), **I want** work groups mapped to tasks or
    tickets, **so that** editing sessions are traceable to work items.

23. **US-15.12.12.1** — **As a** game designer (P-5), **I want** AI agents participating in
    collaborative sessions as virtual users, **so that** I can assign automated tasks like zone
    vegetation.

24. **US-15.12.12.2** — **As a** build engineer (P-16), **I want** AI agent actions tagged with
    provenance, **so that** AI work is tracked in the governance system.

25. **US-15.12.13.1** — **As a** level designer (P-6), **I want** to attach threaded comments to any
    entity, node, or spatial location, **so that** feedback is anchored in context.

26. **US-15.12.13.2** — **As a** game designer (P-5), **I want** comments displayed as viewport pins
    and graph annotations, **so that** I see feedback without switching panels.

27. **US-15.12.14.1** — **As a** engine developer (P-26), **I want** to view and review pull
    requests in the editor with structural diffs, **so that** I never need a web browser.

28. **US-15.12.14.2** — **As a** extension developer (P-25), **I want** PR review comments linked to
    specific nodes and properties, **so that** feedback is precise.
