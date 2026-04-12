# User Stories -- 15.17 Asset Marketplace

## Stories

| ID            | Persona                    |
|---------------|----------------------------|
| US-15.17.1.1  | game designer (P-5)        |
| US-15.17.1.2  | technical artist (P-13)    |
| US-15.17.2.1  | game designer (P-5)        |
| US-15.17.2.2  | engine developer (P-26)    |
| US-15.17.3.1  | game designer (P-5)        |
| US-15.17.3.2  | extension developer (P-25) |
| US-15.17.4.1  | extension developer (P-25) |
| US-15.17.4.2  | extension developer (P-25) |
| US-15.17.5.1  | extension developer (P-25) |
| US-15.17.5.2  | build engineer (P-16)      |
| US-15.17.6.1  | extension developer (P-25) |
| US-15.17.6.2  | game designer (P-5)        |
| US-15.17.7.1  | extension developer (P-25) |
| US-15.17.7.2  | engine developer (P-26)    |
| US-15.17.8.1  | game designer (P-5)        |
| US-15.17.8.2  | engine developer (P-26)    |
| US-15.17.9.1  | game designer (P-5)        |
| US-15.17.9.2  | extension developer (P-25) |
| US-15.17.10.1 | extension developer (P-25) |
| US-15.17.10.2 | build engineer (P-16)      |
| US-15.17.11.1 | studio head (P-1)          |

1. **US-15.17.1.1** — **As a** game designer (P-5), **I want** an in-editor marketplace browser with
   search, filters, and 3D previews, **so that** I can find assets without leaving the editor.

2. **US-15.17.1.2** — **As a** technical artist (P-13), **I want** compatibility badges and engine
   version filtering, **so that** I only see assets that work with my project.

3. **US-15.17.2.1** — **As a** game designer (P-5), **I want** one-click import that validates
   compatibility and resolves dependencies automatically, **so that** assets integrate seamlessly.

4. **US-15.17.2.2** — **As a** engine developer (P-26), **I want** version conflict warnings with
   upgrade guidance, **so that** incompatible imports are caught before they break the project.

5. **US-15.17.3.1** — **As a** game designer (P-5), **I want** ratings, reviews, and curated
   collections, **so that** I can assess asset quality before purchasing.

6. **US-15.17.3.2** — **As a** extension developer (P-25), **I want** reviews moderated for spam and
   abuse, **so that** my store listing reputation is fair.

7. **US-15.17.4.1** — **As a** extension developer (P-25), **I want** a publisher dashboard with
   revenue analytics, compatibility results, and review management, **so that** I can run my asset
   business.

8. **US-15.17.4.2** — **As a** extension developer (P-25), **I want** regional pricing, time-limited
   sales, and bundle packs, **so that** I can optimize revenue.

9. **US-15.17.5.1** — **As a** extension developer (P-25), **I want** automated compatibility
   testing against new engine versions, **so that** my assets stay up to date.

10. **US-15.17.5.2** — **As a** build engineer (P-16), **I want** compatibility testing on CI
    infrastructure using the shared build cache, **so that** testing is efficient.

11. **US-15.17.6.1** — **As a** extension developer (P-25), **I want** fair revenue sharing with
    monthly payouts and detailed per-asset reports, **so that** I can sustain development.

12. **US-15.17.6.2** — **As a** game designer (P-5), **I want** free assets published with no cost
    to the creator, **so that** the community contributes openly.

13. **US-15.17.7.1** — **As a** extension developer (P-25), **I want** the marketplace to support
    all engine asset types including logic graph templates and plugins, **so that** any creation can
    be sold or shared.

14. **US-15.17.7.2** — **As a** engine developer (P-26), **I want** per-platform compiled plugin
    binaries hosted, **so that** plugin distribution covers all targets.

15. **US-15.17.8.1** — **As a** game designer (P-5), **I want** license metadata on imported assets
    with no runtime DRM, **so that** imported assets work offline.

16. **US-15.17.8.2** — **As a** engine developer (P-26), **I want** license validation at import
    time only, **so that** runtime performance is unaffected.

17. **US-15.17.9.1** — **As a** game designer (P-5), **I want** a dedicated open-source asset tab
    with CC0/MIT/Apache filtering, **so that** I can find free assets quickly.

18. **US-15.17.9.2** — **As a** extension developer (P-25), **I want** to publish open-source assets
    via pull request with automated CI validation, **so that** contributions are quality-checked.

19. **US-15.17.10.1** — **As a** extension developer (P-25), **I want** a plugin API for integrating
    third-party stores beyond the built-in options, **so that** any marketplace can connect to the
    editor.

20. **US-15.17.10.2** — **As a** build engineer (P-16), **I want** per-project license compliance
    reports listing all third-party assets with attribution requirements, **so that** legal
    obligations are tracked.

21. **US-15.17.11.1** — **As a** studio head (P-1), **I want** to deploy a self-hosted private asset
    marketplace using the same open-source code with team-only OAuth authentication, **so that** my
    studio can share proprietary assets securely without exposing them publicly.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.17.1 | game designer (P-5) |
| US-15.17.10 | extension developer (P-25) |
| US-15.17.11 | studio head (P-1) |
| US-15.17.2 | game designer (P-5) |
| US-15.17.3 | game designer (P-5) |
| US-15.17.4 | extension developer (P-25) |
| US-15.17.5 | extension developer (P-25) |
| US-15.17.6 | extension developer (P-25) |
| US-15.17.7 | extension developer (P-25) |
| US-15.17.8 | game designer (P-5) |
| US-15.17.9 | game designer (P-5) |

1. **US-15.17.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.17.1.1 through US-15.17.1.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-15.17.10** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.17.10.1 through US-15.17.10.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.17.11** -- **As a** studio head (P-1), **I want** the capabilities defined in sub-stories
   US-15.17.11.1 through US-15.17.11.1 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-15.17.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.17.2.1 through US-15.17.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-15.17.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.17.3.1 through US-15.17.3.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-15.17.4** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.17.4.1 through US-15.17.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-15.17.5** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.17.5.1 through US-15.17.5.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-15.17.6** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.17.6.1 through US-15.17.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

9. **US-15.17.7** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.17.7.1 through US-15.17.7.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

10. **US-15.17.8** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-15.17.8.1 through US-15.17.8.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

11. **US-15.17.9** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-15.17.9.1 through US-15.17.9.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.
