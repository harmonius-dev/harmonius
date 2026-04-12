# User Stories -- 15.10 Version Control

## Stories

| ID           | Persona                    |
|--------------|----------------------------|
| US-15.10.1.1 | game designer (P-5)        |
| US-15.10.1.2 | engine developer (P-26)    |
| US-15.10.2.1 | technical artist (P-13)    |
| US-15.10.2.2 | build engineer (P-16)      |
| US-15.10.3.1 | game designer (P-5)        |
| US-15.10.3.2 | engine developer (P-26)    |
| US-15.10.4.1 | game designer (P-5)        |
| US-15.10.4.2 | build engineer (P-16)      |
| US-15.10.5.1 | level designer (P-6)       |
| US-15.10.5.2 | game designer (P-5)        |
| US-15.10.6.1 | build engineer (P-16)      |
| US-15.10.6.2 | engine developer (P-26)    |
| US-15.10.7.1 | game designer (P-5)        |
| US-15.10.7.2 | level designer (P-6)       |
| US-15.10.8.1 | build engineer (P-16)      |
| US-15.10.8.2 | extension developer (P-25) |
| US-15.10.9.1 | character artist (P-9)     |

1. **US-15.10.1.1** — **As a** game designer (P-5), **I want** an integrated Git client with stage,
   commit, push, pull, and branch operations in the editor UI, **so that** I never need a command
   line.

2. **US-15.10.1.2** — **As a** engine developer (P-26), **I want** Git operations via libgit2 for
   consistent cross-platform behavior, **so that** the editor avoids shell-out overhead.

3. **US-15.10.2.1** — **As a** technical artist (P-13), **I want** automatic LFS tracking based on
   file extension and size thresholds, **so that** binary assets are stored efficiently.

4. **US-15.10.2.2** — **As a** build engineer (P-16), **I want** LFS storage quota monitoring with
   warnings, **so that** I can manage storage costs proactively.

5. **US-15.10.3.1** — **As a** game designer (P-5), **I want** asset-aware merge that structurally
   merges graphs, templates, and data tables instead of failing with binary conflicts, **so that** I
   can collaborate on the same assets.

6. **US-15.10.3.2** — **As a** engine developer (P-26), **I want** unresolvable merge conflicts to
   fall back to the visual diff tool for per-node resolution, **so that** merges are never silently
   incorrect.

7. **US-15.10.4.1** — **As a** game designer (P-5), **I want** to create feature branches, switch
   branches, and create pull requests from the editor UI, **so that** my workflow stays inside the
   editor.

8. **US-15.10.4.2** — **As a** build engineer (P-16), **I want** pull request descriptions
   auto-populated from branch commit history, **so that** review context is always available.

9. **US-15.10.5.1** — **As a** level designer (P-6), **I want** real-time presence indicators
   showing who is editing which assets, **so that** I avoid conflicting edits.

10. **US-15.10.5.2** — **As a** game designer (P-5), **I want** optional pessimistic locking for
    non-mergeable assets, **so that** terrain and lightmaps are not edited concurrently.

11. **US-15.10.6.1** — **As a** build engineer (P-16), **I want** partial clone and sparse checkout
    with per-role patterns, **so that** developers only download the assets they need.

12. **US-15.10.6.2** — **As a** engine developer (P-26), **I want** missing assets to show
    placeholder thumbnails with one-click on-demand fetch, **so that** sparse checkouts are
    transparent.

13. **US-15.10.7.1** — **As a** game designer (P-5), **I want** named shelves for work-in-progress
    with structural diffs, **so that** I can save and resume changes without committing.

14. **US-15.10.7.2** — **As a** level designer (P-6), **I want** shelves shareable via the shared
    cache for handoff, **so that** I can pass work to a colleague without a commit.

15. **US-15.10.8.1** — **As a** build engineer (P-16), **I want** first-class support for GitHub,
    GitLab, Bitbucket, and self-hosted Git servers with provider-specific features, **so that** all
    PR and review actions happen in-editor.

16. **US-15.10.8.2** — **As a** extension developer (P-25), **I want** the provider abstraction to
    auto-detect from the remote URL, **so that** I do not configure hosting manually.

17. **US-15.10.9.1** — **As a** character artist (P-9), **I want** a visual per-property binary
    conflict resolution UI that shows structural diffs between ours and theirs side by side for each
    field, **so that** I can pick the correct value field by field when merging a conflicted mesh
    asset.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.10.1 | game designer (P-5) |
| US-15.10.2 | technical artist (P-13) |
| US-15.10.3 | game designer (P-5) |
| US-15.10.4 | game designer (P-5) |
| US-15.10.5 | level designer (P-6) |
| US-15.10.6 | build engineer (P-16) |
| US-15.10.7 | game designer (P-5) |
| US-15.10.8 | build engineer (P-16) |
| US-15.10.9 | character artist (P-9) |

1. **US-15.10.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.10.1.1 through US-15.10.1.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-15.10.2** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-15.10.2.1 through US-15.10.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.10.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.10.3.1 through US-15.10.3.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-15.10.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.10.4.1 through US-15.10.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-15.10.5** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-15.10.5.1 through US-15.10.5.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-15.10.6** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.10.6.1 through US-15.10.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-15.10.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.10.7.1 through US-15.10.7.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-15.10.8** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.10.8.1 through US-15.10.8.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

9. **US-15.10.9** -- **As a** character artist (P-9), **I want** the capabilities defined in
   sub-stories US-15.10.9.1 through US-15.10.9.1 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
