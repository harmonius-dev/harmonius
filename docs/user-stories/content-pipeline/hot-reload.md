# User Stories -- 12.4 Hot Reload

## File Watching

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.4.1 | technical artist (P-13)    |
| US-12.4.2 | engine developer (P-26)    |

1. **US-12.4.1** — **As a** technical artist (P-13), **I want** source asset directories monitored
   for file changes with sub-second detection, **so that** my edits trigger re-import automatically
   without a manual refresh.
2. **US-12.4.2** — **As an** engine developer (P-26), **I want** file change events debounced and
   deduplicated before dispatch, **so that** rapid successive saves produce only one re-import per
   logical change.

## Asset Hot Reload

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.4.3 | environment artist (P-8)   |
| US-12.4.4 | environment artist (P-8)   |
| US-12.4.5 | technical artist (P-13)    |
| US-12.4.6 | technical artist (P-13)    |
| US-12.4.7 | game designer (P-5)        |
| US-12.4.8 | game designer (P-5)        |

1. **US-12.4.3** — **As an** environment artist (P-8), **I want** modified textures to swap in the
   running viewport via descriptor heap updates without restarting, **so that** I see material
   changes in real time.
2. **US-12.4.4** — **As an** environment artist (P-8), **I want** modified meshes and materials
   swapped via atomic pointer replacement behind a frame fence, **so that** no rendering artifacts
   appear during hot reload.
3. **US-12.4.5** — **As a** technical artist (P-13), **I want** shader source or graph changes to
   trigger recompilation of only affected permutations, **so that** shader iteration does not
   recompile the entire library.
4. **US-12.4.6** — **As a** technical artist (P-13), **I want** shader compilation errors displayed
   as a viewport overlay while the previous valid shader remains active, **so that** I can fix
   errors without losing my visual reference.
5. **US-12.4.7** — **As a** game designer (P-5), **I want** logic graph hot reload to preserve
   execution state when the variable layout is unchanged, **so that** I do not have to reproduce
   gameplay scenarios after every edit.
6. **US-12.4.8** — **As a** game designer (P-5), **I want** UI layout, style sheet, and widget
   template changes to rebuild only the affected subtree while preserving scroll position and focus
   state, **so that** UI iteration does not reset the interface.

## Partial Re-Import and Sync

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.4.9  | technical artist (P-13)   |
| US-12.4.10 | engine developer (P-26)   |
| US-12.4.11 | environment artist (P-8)  |

1. **US-12.4.9** — **As a** technical artist (P-13), **I want** only modified sub-assets within a
   composite file re-imported rather than the entire file, **so that** hot reload latency stays low
   for large multi-asset exports.
2. **US-12.4.10** — **As an** engine developer (P-26), **I want** a bidirectional sync channel
   between editor and runtime that streams asset changes and runtime state in real time, **so that**
   live preview works without manual refresh.
3. **US-12.4.11** — **As an** environment artist (P-8), **I want** material parameter tweaks made in
   the editor to appear on connected runtime instances within 100 ms, **so that** I can tune
   materials while viewing the game.
