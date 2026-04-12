# User Stories -- 15.19 Documentation and Learning

## Stories

| ID           | Persona                    |
|--------------|----------------------------|
| US-15.19.1.1 | engine developer (P-26)    |
| US-15.19.1.2 | extension developer (P-25) |
| US-15.19.2.1 | game designer (P-5)        |
| US-15.19.2.2 | extension developer (P-25) |
| US-15.19.3.1 | game designer (P-5)        |
| US-15.19.3.2 | level designer (P-6)       |
| US-15.19.4.1 | game designer (P-5)        |
| US-15.19.4.2 | technical artist (P-13)    |
| US-15.19.5.1 | game designer (P-5)        |
| US-15.19.5.2 | localization specialist (P-18) |
| US-15.19.6.1 | game designer (P-5)        |
| US-15.19.6.2 | level designer (P-6)       |
| US-15.19.7.1 | engine developer (P-26)    |
| US-15.19.7.2 | build engineer (P-16)      |

1. **US-15.19.1.1** — **As a** engine developer (P-26), **I want** auto-generated API reference from
   Rust doc comments published as a searchable static site, **so that** documentation stays in sync
   with code.

2. **US-15.19.1.2** — **As a** extension developer (P-25), **I want** in-editor help that opens the
   relevant API page when clicking "?" on any property, **so that** I find documentation
   contextually.

3. **US-15.19.2.1** — **As a** game designer (P-5), **I want** every logic graph node to include
   inline documentation with port descriptions, examples, and related nodes, **so that** I
   understand nodes while editing.

4. **US-15.19.2.2** — **As a** extension developer (P-25), **I want** custom nodes to inherit a
   documentation template, **so that** my published nodes have consistent descriptions.

5. **US-15.19.3.1** — **As a** game designer (P-5), **I want** step-by-step in-editor tutorials with
   spotlight overlays, **so that** I learn features by doing.

6. **US-15.19.3.2** — **As a** level designer (P-6), **I want** community-created tutorials
   distributed via the asset store, **so that** learning resources grow with the community.

7. **US-15.19.4.1** — **As a** game designer (P-5), **I want** an embedded video player for tutorial
   videos with chapter links to editor actions, **so that** I follow along without leaving the
   editor.

8. **US-15.19.4.2** — **As a** technical artist (P-13), **I want** a curated video library organized
   by topic and difficulty, **so that** I find relevant videos quickly.

9. **US-15.19.5.1** — **As a** game designer (P-5), **I want** contextual help on every panel,
   field, and button via hover tooltips and F1, **so that** I get instant explanations.

10. **US-15.19.5.2** — **As a** localization specialist (P-18), **I want** help content stored as
    localized string assets, **so that** documentation is available in all supported languages.

11. **US-15.19.6.1** — **As a** game designer (P-5), **I want** sample projects demonstrating engine
    capabilities across genres, **so that** I learn by examining working examples.

12. **US-15.19.6.2** — **As a** level designer (P-6), **I want** annotated logic graphs in sample
    projects explaining design decisions, **so that** I understand the reasoning behind patterns.

13. **US-15.19.7.1** — **As a** engine developer (P-26), **I want** code examples tested in CI so
    they never go stale, **so that** documentation is always accurate.

14. **US-15.19.7.2** — **As a** build engineer (P-16), **I want** CI to fail if a documentation
    example does not compile, **so that** stale examples are caught before release.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.19.1 | engine developer (P-26) |
| US-15.19.2 | game designer (P-5) |
| US-15.19.3 | game designer (P-5) |
| US-15.19.4 | game designer (P-5) |
| US-15.19.5 | game designer (P-5) |
| US-15.19.6 | game designer (P-5) |
| US-15.19.7 | engine developer (P-26) |

1. **US-15.19.1** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-15.19.1.1 through US-15.19.1.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-15.19.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.19.2.1 through US-15.19.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.19.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.19.3.1 through US-15.19.3.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-15.19.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.19.4.1 through US-15.19.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-15.19.5** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.19.5.1 through US-15.19.5.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-15.19.6** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.19.6.1 through US-15.19.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-15.19.7** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-15.19.7.1 through US-15.19.7.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
