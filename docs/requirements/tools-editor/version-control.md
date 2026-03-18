# R-15.10 -- Version Control & Git Integration Requirements

## Native Git

| ID        | Derived From                                                |
|-----------|-------------------------------------------------------------|
| R-15.10.1 | [F-15.10.1](../../features/tools-editor/version-control.md) |
| R-15.10.2 | [F-15.10.2](../../features/tools-editor/version-control.md) |

1. **R-15.10.1** — The editor **SHALL** provide an embedded Git client via libgit2 supporting stage,
   commit, push, pull, branch, merge, rebase, and stash operations through the editor UI, with a
   visual branch graph, per-asset structural diffs, and SSH key access via platform credential
   stores (Keychain, Credential Manager, libsecret).
   - **Rationale:** Version control without command-line access is essential for non-programmer team
     members.
   - **Verification:** Integration test: perform all Git operations in-editor and verify results
     match git CLI equivalents.
2. **R-15.10.2** — The editor **SHALL** automatically track binary assets via LFS based on file
   extension and configurable size thresholds, display lock owner icons in the asset browser,
   provide lock/unlock from context menus, and support bulk LFS operations (migrate, fetch, prune)
   with a storage quota monitor.
   - **Rationale:** Binary assets require LFS for efficient storage; locking prevents concurrent
     edits on non-mergeable files.
   - **Verification:** Unit test: add a file exceeding the size threshold and verify it is
     automatically LFS-tracked.

## Merge and Diff

| ID        | Derived From                                                |
|-----------|-------------------------------------------------------------|
| R-15.10.3 | [F-15.10.3](../../features/tools-editor/version-control.md) |

1. **R-15.10.3** — The editor **SHALL** provide a custom Git merge driver auto-registered via
   .gitattributes that performs three-way structural merge on logic graphs, prefabs, and data
   tables, with fallback to visual diff for unresolvable conflicts.
   - **Rationale:** Binary assets fail with standard merge; structural merge enables collaboration
     on graph and prefab assets.
   - **Verification:** Integration test: merge two compatible changes and verify automatic
     resolution. Merge conflicting changes and verify visual diff fallback.

## Branch Workflow

| ID        | Derived From                                                |
|-----------|-------------------------------------------------------------|
| R-15.10.4 | [F-15.10.4](../../features/tools-editor/version-control.md) |

1. **R-15.10.4** — The editor **SHALL** support creating feature branches, switching branches with
   asset cache preservation, creating pull/merge requests from the editor UI, and integration with
   GitHub, GitLab, Bitbucket, and Azure DevOps APIs.
   - **Rationale:** In-editor branch management reduces context switching; cache preservation avoids
     full rebuilds on branch switch.
   - **Verification:** Unit test: switch branches and verify compiled asset caches are preserved.

## Collaboration

| ID        | Derived From                                                |
|-----------|-------------------------------------------------------------|
| R-15.10.5 | [F-15.10.5](../../features/tools-editor/version-control.md) |

1. **R-15.10.5** — The editor **SHALL** show real-time presence indicators for which users are
   editing which assets, with colored avatar badges, pessimistic locking for non-mergeable assets,
   and automatic lock queue with holder notification, using WebSocket with LFS lock fallback.
   - **Rationale:** Concurrent edit awareness prevents wasted work on non-mergeable assets.
   - **Verification:** Integration test: verify presence indicators show correct editing user across
     two editor instances.

## Repository Optimization

| ID        | Derived From                                                |
|-----------|-------------------------------------------------------------|
| R-15.10.6 | [F-15.10.6](../../features/tools-editor/version-control.md) |
| R-15.10.7 | [F-15.10.7](../../features/tools-editor/version-control.md) |
| R-15.10.8 | [F-15.10.8](../../features/tools-editor/version-control.md) |

1. **R-15.10.6** — The editor **SHALL** support Git partial clone (blobless/treeless) and sparse
   checkout with role-configurable patterns, placeholder thumbnails for missing assets, and
   one-click on-demand fetch with background download.
   - **Rationale:** Large repositories require selective checkout to reduce clone size and disk
     usage.
   - **Verification:** Unit test: verify partial clone size is significantly smaller than full
     clone.
2. **R-15.10.7** — The editor **SHALL** support named shelves for work-in-progress changes with
   structural diffs, shelf sharing via the shared cache, and shelf application using structural
   merge, stored in .harmonius/shelves/ and excluded from VCS.
   - **Rationale:** Named shelves provide richer WIP management than git stash with collaborative
     sharing.
   - **Verification:** Unit test: save a shelf, verify data is stored in .harmonius/shelves/ and
     excluded from VCS.
3. **R-15.10.8** — The editor **SHALL** auto-detect hosting providers from remote URLs, support pull
   request creation/review/merge in-editor, issue linking, CI status display, self-hosted instances
   with configurable base URLs, and API tokens stored in platform credential stores.
   - **Rationale:** Teams use diverse Git hosting; the editor must work with any provider without
     configuration.
   - **Verification:** Integration test: verify PR workflows against GitHub, GitLab, and Bitbucket
     APIs.

---

## User Stories

## US-15.10.1 Native Git Integration

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-15.10.1.1 | developer (P-15)     | F-15.10.1 | R-15.10.1    |
| US-15.10.1.2 | developer (P-15)     | F-15.10.1 | R-15.10.1    |
| US-15.10.1.3 | developer (P-15)     | F-15.10.1 | R-15.10.1    |
| US-15.10.1.4 | developer (P-15)     | F-15.10.1 | R-15.10.1    |
| US-15.10.1.5 | artist (P-8)         | F-15.10.1 | R-15.10.1    |
| US-15.10.1.6 | engine dev (P-26)    | F-15.10.1 | R-15.10.1    |
| US-15.10.1.7 | engine dev (P-26)    | F-15.10.1 | R-15.10.1    |
| US-15.10.1.8 | engine tester (P-27) | F-15.10.1 | R-15.10.1    |

1. **US-15.10.1.1** — I want stage, commit, push, and pull from the editor UI so that I can perform
   version control without a command line.
2. **US-15.10.1.2** — I want a visual branch graph in the commit history browser so that I can
   understand branch topology at a glance.
3. **US-15.10.1.3** — I want per-asset structural diffs in the history browser so that I can see
   what changed in binary assets without external tools.
4. **US-15.10.1.4** — I want branch, merge, rebase, and stash operations in-editor so that I can
   manage complex branch workflows without leaving the editor.
5. **US-15.10.1.5** — I want a simplified commit workflow with staging and commit so that I can save
   my work to version control without Git expertise.
6. **US-15.10.1.6** — I want libgit2-based Git operations for cross-platform consistency so that
   repository operations behave identically on macOS, Windows, and Linux.
7. **US-15.10.1.7** — I want SSH key access via platform credential stores so that authentication
   uses Keychain, Credential Manager, or libsecret.
8. **US-15.10.1.8** — I want to verify all Git operations produce identical results to command-line
   equivalents so that the embedded client is regression-tested against git CLI.

## US-15.10.2 Git LFS Management

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-15.10.2.1 | developer (P-15)     | F-15.10.2 | R-15.10.2    |
| US-15.10.2.2 | developer (P-15)     | F-15.10.2 | R-15.10.2    |
| US-15.10.2.3 | artist (P-8)         | F-15.10.2 | R-15.10.2    |
| US-15.10.2.4 | artist (P-8)         | F-15.10.2 | R-15.10.2    |
| US-15.10.2.5 | DevOps (P-16)        | F-15.10.2 | R-15.10.2    |
| US-15.10.2.6 | DevOps (P-16)        | F-15.10.2 | R-15.10.2    |
| US-15.10.2.7 | engine tester (P-27) | F-15.10.2 | R-15.10.2    |

1. **US-15.10.2.1** — I want automatic LFS tracking based on file extension so that binary assets
   are tracked by LFS without manual configuration.
2. **US-15.10.2.2** — I want configurable size thresholds for LFS tracking so that files above a set
   size are automatically LFS-managed.
3. **US-15.10.2.3** — I want lock owner icons on LFS-tracked assets in the browser so that I can see
   who is editing a binary asset before opening it.
4. **US-15.10.2.4** — I want lock/unlock from the asset context menu so that I can claim exclusive
   edit rights on non-mergeable assets.
5. **US-15.10.2.5** — I want bulk LFS operations (migrate, fetch, prune) so that I can manage LFS
   storage efficiently.
6. **US-15.10.2.6** — I want an LFS storage quota monitor so that I can track usage against team or
   organization limits.
7. **US-15.10.2.7** — I want to verify assets exceeding the size threshold are automatically
   LFS-tracked so that auto-tracking is regression-tested.

## US-15.10.3 Asset-Aware Merge Driver

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-15.10.3.1 | developer (P-15)     | F-15.10.3 | R-15.10.3    |
| US-15.10.3.2 | developer (P-15)     | F-15.10.3 | R-15.10.3    |
| US-15.10.3.3 | developer (P-15)     | F-15.10.3 | R-15.10.3    |
| US-15.10.3.4 | DevOps (P-16)        | F-15.10.3 | R-15.10.3    |
| US-15.10.3.5 | engine tester (P-27) | F-15.10.3 | R-15.10.3    |

1. **US-15.10.3.1** — I want a custom merge driver for structured asset formats so that binary
   assets merge at the semantic level instead of failing.
2. **US-15.10.3.2** — I want three-way structural merge on logic graphs and prefabs so that
   concurrent graph edits merge automatically when compatible.
3. **US-15.10.3.3** — I want fallback to visual diff for unresolvable conflicts so that I can
   resolve asset conflicts per-node or per-property.
4. **US-15.10.3.4** — I want the merge driver auto-registered via .gitattributes so that it works
   automatically on project clone without manual setup.
5. **US-15.10.3.5** — I want to verify structural merge resolves compatible changes automatically so
   that merge correctness is regression-tested with known conflict scenarios.

## US-15.10.4 Branch-Per-Feature Workflow

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-15.10.4.1 | developer (P-15)     | F-15.10.4 | R-15.10.4    |
| US-15.10.4.2 | developer (P-15)     | F-15.10.4 | R-15.10.4    |
| US-15.10.4.3 | developer (P-15)     | F-15.10.4 | R-15.10.4    |
| US-15.10.4.4 | developer (P-15)     | F-15.10.4 | R-15.10.4    |
| US-15.10.4.5 | artist (P-8)         | F-15.10.4 | R-15.10.4    |
| US-15.10.4.6 | DevOps (P-16)        | F-15.10.4 | R-15.10.4    |
| US-15.10.4.7 | engine tester (P-27) | F-15.10.4 | R-15.10.4    |

1. **US-15.10.4.1** — I want to create feature branches from the editor UI so that I can start
   isolated work without the command line.
2. **US-15.10.4.2** — I want branch switching with asset cache preservation so that switching
   branches does not require full rebuild.
3. **US-15.10.4.3** — I want pull/merge request creation from the editor so that I can submit work
   for review without a web browser.
4. **US-15.10.4.4** — I want warnings on unsaved changes before branch switch so that I do not lose
   work when changing branches.
5. **US-15.10.4.5** — I want auto-populated PR descriptions from commit history so that my pull
   request summarizes my work automatically.
6. **US-15.10.4.6** — I want integration with GitHub, GitLab, Bitbucket, and Azure DevOps APIs so
   that PR workflows work with any hosting provider.
7. **US-15.10.4.7** — I want to verify branch switch preserves compiled asset caches so that cache
   preservation is regression-tested.

## US-15.10.5 Collaborative Presence

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-15.10.5.1 | developer (P-15)     | F-15.10.5 | R-15.10.5    |
| US-15.10.5.2 | artist (P-8)         | F-15.10.5 | R-15.10.5    |
| US-15.10.5.3 | artist (P-8)         | F-15.10.5 | R-15.10.5    |
| US-15.10.5.4 | artist (P-8)         | F-15.10.5 | R-15.10.5    |
| US-15.10.5.5 | engine dev (P-26)    | F-15.10.5 | R-15.10.5    |
| US-15.10.5.6 | engine tester (P-27) | F-15.10.5 | R-15.10.5    |

1. **US-15.10.5.1** — I want real-time presence indicators showing who edits what so that I can
   avoid concurrent edits on the same asset.
2. **US-15.10.5.2** — I want colored avatar badges on assets in the browser so that I can see at a
   glance which assets are being edited.
3. **US-15.10.5.3** — I want pessimistic locking for non-mergeable assets so that concurrent edits
   on heightmaps and lightmaps are prevented.
4. **US-15.10.5.4** — I want automatic lock queue with holder notification so that I can wait for a
   lock without repeatedly checking.
5. **US-15.10.5.5** — I want WebSocket-based presence with LFS lock fallback so that presence works
   even when the WebSocket service is unavailable.
6. **US-15.10.5.6** — I want to verify presence indicators show correct editing user across two
   editor instances so that presence accuracy is regression-tested.

## US-15.10.6 Partial Clone and Sparse Checkout

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-15.10.6.1 | developer (P-15)     | F-15.10.6 | R-15.10.6    |
| US-15.10.6.2 | developer (P-15)     | F-15.10.6 | R-15.10.6    |
| US-15.10.6.3 | artist (P-8)         | F-15.10.6 | R-15.10.6    |
| US-15.10.6.4 | artist (P-8)         | F-15.10.6 | R-15.10.6    |
| US-15.10.6.5 | DevOps (P-16)        | F-15.10.6 | R-15.10.6    |
| US-15.10.6.6 | engine tester (P-27) | F-15.10.6 | R-15.10.6    |

1. **US-15.10.6.1** — I want Git partial clone (blobless/treeless) support so that initial clone
   downloads minimal data.
2. **US-15.10.6.2** — I want sparse checkout with role-configurable patterns so that artists get art
   assets and programmers get source code.
3. **US-15.10.6.3** — I want placeholder thumbnails for missing assets so that the browser is usable
   even when assets are not checked out.
4. **US-15.10.6.4** — I want one-click on-demand fetch for missing assets so that I can download
   specific assets without fetching the entire repo.
5. **US-15.10.6.5** — I want fetch operations to run in the background so that downloading missing
   assets does not block the editor.
6. **US-15.10.6.6** — I want to verify partial clone size is significantly smaller than full clone
   so that partial clone effectiveness is regression-tested.

## US-15.10.7 Shelving and Local Stash

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-15.10.7.1 | developer (P-15)     | F-15.10.7 | R-15.10.7    |
| US-15.10.7.2 | developer (P-15)     | F-15.10.7 | R-15.10.7    |
| US-15.10.7.3 | developer (P-15)     | F-15.10.7 | R-15.10.7    |
| US-15.10.7.4 | developer (P-15)     | F-15.10.7 | R-15.10.7    |
| US-15.10.7.5 | engine tester (P-27) | F-15.10.7 | R-15.10.7    |

1. **US-15.10.7.1** — I want named shelves for work-in-progress changes so that I can save
   uncommitted work with descriptive names.
2. **US-15.10.7.2** — I want shelves to store structural diffs alongside assets so that I can review
   shelved changes with rich context.
3. **US-15.10.7.3** — I want to share shelves via the shared cache so that I can hand off
   work-in-progress to a colleague.
4. **US-15.10.7.4** — I want shelf application to use structural merge so that applying a shelf
   handles overlapping edits correctly.
5. **US-15.10.7.5** — I want to verify shelf data is stored in .harmonius/shelves/ and excluded from
   VCS so that shelf isolation is regression-tested.

## US-15.10.8 Multi-Provider Git Hosting Support

| ID           | Persona              | Features  | Requirements |
|--------------|----------------------|-----------|--------------|
| US-15.10.8.1 | developer (P-15)     | F-15.10.8 | R-15.10.8    |
| US-15.10.8.2 | developer (P-15)     | F-15.10.8 | R-15.10.8    |
| US-15.10.8.3 | developer (P-15)     | F-15.10.8 | R-15.10.8    |
| US-15.10.8.4 | DevOps (P-16)        | F-15.10.8 | R-15.10.8    |
| US-15.10.8.5 | DevOps (P-16)        | F-15.10.8 | R-15.10.8    |
| US-15.10.8.6 | engine tester (P-27) | F-15.10.8 | R-15.10.8    |

1. **US-15.10.8.1** — I want auto-detection of hosting provider from remote URL so that the editor
   configures itself without manual provider selection.
2. **US-15.10.8.2** — I want pull request creation, review, and merge in-editor so that I never need
   to open a web browser for PR workflows.
3. **US-15.10.8.3** — I want issue linking and CI status display in-editor so that I can track work
   items alongside code changes.
4. **US-15.10.8.4** — I want support for self-hosted instances with configurable base URL so that
   the editor works with on-premise Git servers.
5. **US-15.10.8.5** — I want API tokens stored in the platform credential store so that hosting
   credentials are secured natively per platform.
6. **US-15.10.8.6** — I want to verify PR workflows against GitHub, GitLab, and Bitbucket so that
   multi-provider support is regression-tested.
