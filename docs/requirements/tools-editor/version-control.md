# R-15.10 -- Version Control & Git Integration Requirements

## R-15.10.1 Native Git Integration

The engine **SHALL** embed a fully integrated Git client using libgit2 that provides stage, commit,
push, pull, branch, merge, rebase, and stash operations through the editor UI, including a commit
history browser with a visual branch graph and per-asset structural diffs.

- **Derived from:** [F-15.10.1](../../features/tools-editor/version-control.md)
- **Rationale:** An embedded Git client eliminates context switching to external tools and enables
  asset-aware diffs that raw text-based Git tools cannot provide.
- **Verification:** Clone a repository from the editor UI. Stage, commit, and push a modified
  asset. Verify the commit appears in the branch graph. Perform a branch, merge, and rebase
  operation and verify repository state matches the equivalent command-line operations. Verify
  SSH key access uses the platform credential store (Keychain, Credential Manager, libsecret).

## R-15.10.2 Git LFS Management

The engine **SHALL** automatically track binary assets via Git LFS based on file extension and
configurable size thresholds, display lock owner icons on LFS-tracked assets in the asset browser,
and provide lock/unlock and bulk LFS operations (migrate, fetch, prune) from the version control
panel.

- **Derived from:** [F-15.10.2](../../features/tools-editor/version-control.md)
- **Rationale:** Automatic LFS tracking prevents accidental storage of large binaries in the Git
  object store, and integrated lock management avoids merge conflicts on non-mergeable assets.
- **Verification:** Add a binary asset exceeding the size threshold and verify it is automatically
  tracked by LFS. Lock an asset, verify the lock icon appears for other users, and verify a
  second user cannot edit the locked asset. Run a bulk fetch operation and verify all LFS objects
  download. Verify the storage quota monitor displays accurate usage.

## R-15.10.3 Asset-Aware Merge Driver

The engine **SHALL** register a custom Git merge driver that invokes the engine's structural merge
system for binary and structured asset formats, performing three-way structural merge on assets such
as logic graphs, material graphs, prefabs, and data tables, with fallback to the visual diff tool
for unresolvable conflicts.

- **Derived from:** [F-15.10.3](../../features/tools-editor/version-control.md)
- **Rationale:** Structural merge resolves asset conflicts at the semantic level (per-node,
  per-property) instead of failing outright on binary files, reducing merge friction for asset-
  heavy workflows.
- **Verification:** Create conflicting edits on a logic graph asset in two branches. Merge the
  branches and verify the merge driver performs a three-way structural merge resolving compatible
  changes automatically. Introduce an unresolvable conflict and verify the visual diff tool opens
  for manual resolution. Confirm the driver is registered via `.gitattributes`.

## R-15.10.4 Branch-Per-Feature Workflow

The engine **SHALL** support creating feature branches, switching branches with asset cache
preservation, and creating pull/merge requests from the editor UI, with warnings on unsaved changes
and auto-populated pull request descriptions from the branch commit history.

- **Derived from:** [F-15.10.4](../../features/tools-editor/version-control.md)
- **Rationale:** Integrated branch workflow reduces the barrier to adopting branch-per-feature
  practices, especially for non-programmers who may not be comfortable with command-line Git.
- **Verification:** Create a feature branch from the editor. Make changes and switch back to the
  main branch. Verify a warning appears for unsaved changes with stash/shelve options. Switch
  branches and verify compiled asset caches are preserved. Create a pull request and verify the
  description is auto-populated from commit messages. Verify integration with GitHub, GitLab,
  Bitbucket, and Azure DevOps APIs.

## R-15.10.5 Collaborative Presence

The engine **SHALL** display real-time presence indicators showing which team members are editing
which assets, with colored avatar badges in the asset browser, and provide optional pessimistic
locking for non-mergeable assets with automatic lock queue management and holder notification.

- **Derived from:** [F-15.10.5](../../features/tools-editor/version-control.md)
- **Rationale:** Presence visibility prevents wasted effort from concurrent edits on non-mergeable
  assets and enables coordination without external communication tools.
- **Verification:** Open the same project in two editor instances. Verify user A sees user B's
  avatar badge on the asset user B has open. Enable pessimistic locking on a terrain heightmap.
  Verify a second user's edit attempt queues a lock request and the lock holder receives a
  notification. Verify presence falls back to LFS lock state polling when the WebSocket service
  is unavailable.

## R-15.10.6 Partial Clone and Sparse Checkout

The engine **SHALL** support Git partial clone (blobless and treeless modes) and sparse checkout with
role-configurable patterns so developers download only the assets relevant to their role, with
placeholder thumbnails and one-click on-demand fetch for missing assets.

- **Derived from:** [F-15.10.6](../../features/tools-editor/version-control.md)
- **Rationale:** Partial clone and sparse checkout reduce clone times and local disk usage for
  massive repositories where individual developers need only a subset of assets.
- **Verification:** Clone a repository using blobless mode and verify the clone size is
  significantly smaller than a full clone. Configure sparse checkout patterns for an artist role
  and verify only art assets are checked out. Verify missing assets display placeholder
  thumbnails. Click the on-demand fetch button and verify the asset downloads in the background
  without blocking the editor.

## R-15.10.7 Shelving and Local Stash

The engine **SHALL** allow saving work-in-progress changes to named shelves that store modified
assets with structural diffs, support sharing shelves via the shared cache, and merge shelved
changes with the current working state using the structural merge system.

- **Derived from:** [F-15.10.7](../../features/tools-editor/version-control.md)
- **Rationale:** Named shelves provide richer work-in-progress management than plain Git stash by
  preserving structural diffs and enabling team handoff scenarios.
- **Verification:** Create a named shelf with modified assets. Verify the shelf includes
  structural diffs. Apply the shelf to a modified working state and verify the structural merge
  system resolves compatible changes. Share a shelf via the shared cache and verify another
  developer can retrieve and apply it. Verify shelf data is stored in `.harmonius/shelves/` and
  excluded from version control.

## Non-Functional Requirements

### R-15.10.NF1 Version Control Operation Performance

Git operations invoked through the editor **SHALL** meet the following performance targets:
- `git status` equivalent: under 500ms for repositories with up to 100,000 tracked files.
- `git commit` (staged files): under 2 seconds for commits touching up to 1,000 files.
- Branch switch with asset cache preservation: under 10 seconds for branches diverging by up to
  5,000 files.
- LFS lock query: under 1 second for repositories with up to 10,000 LFS-tracked files.

All Git operations **SHALL** execute on a background thread and **SHALL NOT** block the editor UI.
Progress indicators **SHALL** display for operations exceeding 1 second.

- **Derived from:** F-15.10.1 through F-15.10.8 (all version control features).
- **Rationale:** Version control operations occur frequently during development. Blocking the UI
  during Git operations disrupts the editing workflow and discourages frequent commits.
- **Verification:** Create a repository with 100,000 tracked files. Measure `git status` time and
  assert under 500ms. Stage 1,000 files and measure commit time; assert under 2 seconds. Switch
  branches with 5,000 file differences and assert completion under 10 seconds. Verify a progress
  indicator appears for operations exceeding 1 second. Verify the UI remains interactive during
  all operations.

## R-15.10.8 Multi-Provider Git Hosting Support

The engine **SHALL** support GitHub, GitLab, Bitbucket, and self-hosted Git servers through a
provider abstraction layer, with auto-detection from the remote URL, provider-specific features
(pull requests, code review, issue linking, CI status), and all provider interactions accessible
within the editor.

- **Derived from:** [F-15.10.8](../../features/tools-editor/version-control.md)
- **Rationale:** A provider abstraction ensures teams are not locked into a single hosting
  platform and can perform all version control workflows without leaving the editor.
- **Verification:** Configure projects with GitHub, GitLab, and Bitbucket remotes. Verify the
  provider is auto-detected from each remote URL. Create a pull request, add a review comment,
  and merge -- all from the editor UI. Verify API tokens are stored in the platform credential
  store. Verify a self-hosted instance works with a manually configured base URL.
