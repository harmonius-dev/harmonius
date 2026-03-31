# 15.10 — Version Control & Git Integration

## Native Git

| ID | Feature |
| ----------- | ------------------------ |
| F-15.10.1 | Native Git Integration |
| F-15.10.2 | Git LFS Management |

1. **F-15.10.1** — A fully integrated Git client embedded in the editor, providing stage, commit,
   push, pull, branch, merge, rebase, and stash operations through the editor UI without requiring a
   command line. The commit history browser displays a visual branch graph with per-asset structural
   diffs. All operations use libgit2 for direct repository access, avoiding shell-out overhead and
   ensuring consistent behavior across platforms. Windows, uses the Windows Credential Manager. On
   Linux, uses the Secret Service API (libsecret) or ssh-agent.
   - **Deps:** F-15.1.1, F-15.1.8
   - **Platform:** On macOS, SSH key access uses the Keychain for passphrase management. On
2. **F-15.10.2** — Automatic LFS tracking for binary assets based on file extension and configurable
   size thresholds. The asset browser displays lock owner icons on LFS-tracked assets and provides
   lock/unlock actions directly from the context menu. Bulk LFS operations — migrate, fetch, prune —
   are accessible from the version control panel. An LFS storage quota monitor displays current
   usage against team or organization limits, with warnings when usage exceeds configurable
   thresholds. Schannel on Windows, OpenSSL on Linux.
   - **Deps:** F-15.10.1, F-12.5.1 (Asset Database)
   - **Platform:** LFS transfers use the platform-native TLS stack: Secure Transport on macOS,

## Merge and Diff

| ID | Feature |
| ----------- | -------------------------- |
| F-15.10.3 | Asset-Aware Merge Driver |

1. **F-15.10.3** — A custom Git merge driver registered automatically on project clone that invokes
   the engine's structural merge system (F-12.7.4) for binary and structured asset formats instead
   of failing with a binary conflict. The driver performs three-way structural merge on assets such
   as logic graphs, material graphs, prefabs, and data tables. Unresolvable conflicts fall back to
   the visual diff tool where users accept changes per-node or per-property. `.git/config` entry
   written by the editor's project setup wizard.
   - **Deps:** F-15.10.1, F-12.7.4 (Structural Merge), F-15.8.13 (Graph Diffing)
   - **Platform:** The merge driver is registered via `.gitattributes` and a project-local

## Branch Workflow

| ID | Feature |
| ----------- | ----------------------------- |
| F-15.10.4 | Branch-Per-Feature Workflow |

1. **F-15.10.4** — The editor supports creating feature branches, switching branches with asset
   cache preservation, and creating pull/merge requests directly from the editor UI. Branch
   switching warns about unsaved changes and offers to stash or shelve them before proceeding. Pull
   request descriptions are auto-populated from the commit history on the branch. The branch
   selector displays the branch graph inline with remote tracking status and ahead/behind counts.
   creation integrates with GitHub, GitLab, Bitbucket, and Azure DevOps APIs.
   - **Deps:** F-15.10.1, F-15.10.7
   - **Platform:** Desktop only. Not available on mobile or console runtime. Pull/merge request

## Collaboration

| ID | Feature |
| ----------- | ------------------------ |
| F-15.10.5 | Collaborative Presence |

1. **F-15.10.5** — Real-time presence indicators show which team members are currently editing which
   assets. The asset browser displays a colored avatar badge on each asset that is open in another
   user's editor. For assets that do not merge well — large terrain heightmaps, baked lightmaps,
   navigation meshes — optional pessimistic locking prevents concurrent edits. Lock requests queue
   automatically when an asset is already locked, and the lock holder receives a notification with
   the option to release. polling the LFS lock state when the presence service is unavailable.
   - **Deps:** F-15.10.1, F-15.10.2, F-15.12.3
   - **Platform:** Presence is broadcast via a lightweight WebSocket service. Falls back to

## Repository Optimization

| ID | Feature |
| ----------- | ----------------------------------- |
| F-15.10.6 | Partial Clone and Sparse Checkout |

1. **F-15.10.6** — For massive repositories, the editor supports Git partial clone (blobless and
   treeless modes) and sparse checkout so developers only download the assets they need. Sparse
   checkout patterns are configurable per team role — artists receive art assets, designers receive
   levels and data tables, programmers receive source and configuration. Missing assets display
   placeholder thumbnails in the asset browser with a one-click on-demand fetch action. Fetch
   operations run in the background without blocking the editor. cone-mode pattern format introduced
   in Git 2.25.
   - **Deps:** F-15.10.1, F-15.10.2, F-12.5.1 (Asset Database)
   - **Platform:** Partial clone requires Git 2.22+ on the server. Sparse checkout uses the

## Local Work-in-Progress

| ID | Feature |
| ----------- | -------------------------- |
| F-15.10.7 | Shelving and Local Stash |

1. **F-15.10.7** — Save work-in-progress changes locally without committing to the repository. Named
   shelves store all modified assets along with their structural diffs, providing a richer view than
   a plain Git stash. Shelves can be shared with team members via the shared cache (F-15.11.1) for
   handoff scenarios. Applying a shelf merges the shelved changes with the current working state
   using the same structural merge system as the merge driver (F-15.10.3), with conflict resolution
   for overlapping edits. excluded from version control via `.gitignore`.
   - **Deps:** F-15.10.1, F-15.10.3, F-15.11.1
   - **Platform:** Shelf data is stored in the project's `.harmonius/shelves/` directory and

## Multi-Provider

| ID | Feature |
| ----------- | ------------------------------------ |
| F-15.10.8 | Multi-Provider Git Hosting Support |

1. **F-15.10.8** — First-class support for GitHub, GitLab, Bitbucket, and self-hosted Git servers.
   All hosting- provider-specific features (pull requests, merge requests, code review, issue
   linking, CI status) are accessed through a provider abstraction layer. The editor auto-detects
   the provider from the remote URL. Provider-specific API tokens are stored in the platform
   credential store. All provider interactions (PR creation, review, merge, comment) happen within
   the editor — no browser required. REST 2.0 API. Self-hosted instances require configurable base
   URL.
   - **Deps:** F-15.10.1, F-15.12.14 (PR Review in Editor)
   - **Platform:** GitHub uses REST v3 + GraphQL v4 APIs. GitLab uses REST v4 API. Bitbucket uses
