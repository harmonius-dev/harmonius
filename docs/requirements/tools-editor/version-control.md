# R-15.10 -- Version Control & Git Integration User Stories

## US-15.10.1 Native Git Integration

### US-15.10.1.1
As a **developer (P-15)**, I want stage, commit, push, and pull from the editor UI so that I can
perform version control without a command line.

### US-15.10.1.2
As a **developer (P-15)**, I want a visual branch graph in the commit history browser so that I can
understand branch topology at a glance.

### US-15.10.1.3
As a **developer (P-15)**, I want per-asset structural diffs in the history browser so that I can
see what changed in binary assets without external tools.

### US-15.10.1.4
As a **developer (P-15)**, I want branch, merge, rebase, and stash operations in-editor so that I
can manage complex branch workflows without leaving the editor.

### US-15.10.1.5
As a **artist (P-8)**, I want a simplified commit workflow with staging and commit so that I can
save my work to version control without Git expertise.

### US-15.10.1.6
As an **engine dev (P-26)**, I want libgit2-based Git operations for cross-platform consistency so
that repository operations behave identically on macOS, Windows, and Linux.

### US-15.10.1.7
As an **engine dev (P-26)**, I want SSH key access via platform credential stores so that
authentication uses Keychain, Credential Manager, or libsecret.

### US-15.10.1.8
As an **engine tester (P-27)**, I want to verify all Git operations produce identical results to
command-line equivalents so that the embedded client is regression-tested against git CLI.

---

## US-15.10.2 Git LFS Management

### US-15.10.2.1
As a **developer (P-15)**, I want automatic LFS tracking based on file extension so that binary
assets are tracked by LFS without manual configuration.

### US-15.10.2.2
As a **developer (P-15)**, I want configurable size thresholds for LFS tracking so that files above
a set size are automatically LFS-managed.

### US-15.10.2.3
As a **artist (P-8)**, I want lock owner icons on LFS-tracked assets in the browser so that I can
see who is editing a binary asset before opening it.

### US-15.10.2.4
As a **artist (P-8)**, I want lock/unlock from the asset context menu so that I can claim exclusive
edit rights on non-mergeable assets.

### US-15.10.2.5
As a **DevOps (P-16)**, I want bulk LFS operations (migrate, fetch, prune) so that I can manage LFS
storage efficiently.

### US-15.10.2.6
As a **DevOps (P-16)**, I want an LFS storage quota monitor so that I can track usage against team
or organization limits.

### US-15.10.2.7
As an **engine tester (P-27)**, I want to verify assets exceeding the size threshold are
automatically LFS-tracked so that auto-tracking is regression-tested.

---

## US-15.10.3 Asset-Aware Merge Driver

### US-15.10.3.1
As a **developer (P-15)**, I want a custom merge driver for structured asset formats so that binary
assets merge at the semantic level instead of failing.

### US-15.10.3.2
As a **developer (P-15)**, I want three-way structural merge on logic graphs and prefabs so that
concurrent graph edits merge automatically when compatible.

### US-15.10.3.3
As a **developer (P-15)**, I want fallback to visual diff for unresolvable conflicts so that I can
resolve asset conflicts per-node or per-property.

### US-15.10.3.4
As a **DevOps (P-16)**, I want the merge driver auto-registered via .gitattributes so that it works
automatically on project clone without manual setup.

### US-15.10.3.5
As an **engine tester (P-27)**, I want to verify structural merge resolves compatible changes
automatically so that merge correctness is regression-tested with known conflict scenarios.

---

## US-15.10.4 Branch-Per-Feature Workflow

### US-15.10.4.1
As a **developer (P-15)**, I want to create feature branches from the editor UI so that I can start
isolated work without the command line.

### US-15.10.4.2
As a **developer (P-15)**, I want branch switching with asset cache preservation so that switching
branches does not require full rebuild.

### US-15.10.4.3
As a **developer (P-15)**, I want pull/merge request creation from the editor so that I can submit
work for review without a web browser.

### US-15.10.4.4
As a **developer (P-15)**, I want warnings on unsaved changes before branch switch so that I do not
lose work when changing branches.

### US-15.10.4.5
As a **artist (P-8)**, I want auto-populated PR descriptions from commit history so that my pull
request summarizes my work automatically.

### US-15.10.4.6
As a **DevOps (P-16)**, I want integration with GitHub, GitLab, Bitbucket, and Azure DevOps APIs so
that PR workflows work with any hosting provider.

### US-15.10.4.7
As an **engine tester (P-27)**, I want to verify branch switch preserves compiled asset caches so
that cache preservation is regression-tested.

---

## US-15.10.5 Collaborative Presence

### US-15.10.5.1
As a **developer (P-15)**, I want real-time presence indicators showing who edits what so that I can
avoid concurrent edits on the same asset.

### US-15.10.5.2
As a **artist (P-8)**, I want colored avatar badges on assets in the browser so that I can see at a
glance which assets are being edited.

### US-15.10.5.3
As a **artist (P-8)**, I want pessimistic locking for non-mergeable assets so that concurrent edits
on heightmaps and lightmaps are prevented.

### US-15.10.5.4
As a **artist (P-8)**, I want automatic lock queue with holder notification so that I can wait for a
lock without repeatedly checking.

### US-15.10.5.5
As an **engine dev (P-26)**, I want WebSocket-based presence with LFS lock fallback so that presence
works even when the WebSocket service is unavailable.

### US-15.10.5.6
As an **engine tester (P-27)**, I want to verify presence indicators show correct editing user
across two editor instances so that presence accuracy is regression-tested.

---

## US-15.10.6 Partial Clone and Sparse Checkout

### US-15.10.6.1
As a **developer (P-15)**, I want Git partial clone (blobless/treeless) support so that initial
clone downloads minimal data.

### US-15.10.6.2
As a **developer (P-15)**, I want sparse checkout with role-configurable patterns so that artists
get art assets and programmers get source code.

### US-15.10.6.3
As a **artist (P-8)**, I want placeholder thumbnails for missing assets so that the browser is
usable even when assets are not checked out.

### US-15.10.6.4
As a **artist (P-8)**, I want one-click on-demand fetch for missing assets so that I can download
specific assets without fetching the entire repo.

### US-15.10.6.5
As a **DevOps (P-16)**, I want fetch operations to run in the background so that downloading missing
assets does not block the editor.

### US-15.10.6.6
As an **engine tester (P-27)**, I want to verify partial clone size is significantly smaller than
full clone so that partial clone effectiveness is regression-tested.

---

## US-15.10.7 Shelving and Local Stash

### US-15.10.7.1
As a **developer (P-15)**, I want named shelves for work-in-progress changes so that I can save
uncommitted work with descriptive names.

### US-15.10.7.2
As a **developer (P-15)**, I want shelves to store structural diffs alongside assets so that I can
review shelved changes with rich context.

### US-15.10.7.3
As a **developer (P-15)**, I want to share shelves via the shared cache so that I can hand off
work-in-progress to a colleague.

### US-15.10.7.4
As a **developer (P-15)**, I want shelf application to use structural merge so that applying a shelf
handles overlapping edits correctly.

### US-15.10.7.5
As an **engine tester (P-27)**, I want to verify shelf data is stored in .harmonius/shelves/ and
excluded from VCS so that shelf isolation is regression-tested.

---

## US-15.10.8 Multi-Provider Git Hosting Support

### US-15.10.8.1
As a **developer (P-15)**, I want auto-detection of hosting provider from remote URL so that the
editor configures itself without manual provider selection.

### US-15.10.8.2
As a **developer (P-15)**, I want pull request creation, review, and merge in-editor so that I never
need to open a web browser for PR workflows.

### US-15.10.8.3
As a **developer (P-15)**, I want issue linking and CI status display in-editor so that I can track
work items alongside code changes.

### US-15.10.8.4
As a **DevOps (P-16)**, I want support for self-hosted instances with configurable base URL so that
the editor works with on-premise Git servers.

### US-15.10.8.5
As a **DevOps (P-16)**, I want API tokens stored in the platform credential store so that hosting
credentials are secured natively per platform.

### US-15.10.8.6
As an **engine tester (P-27)**, I want to verify PR workflows against GitHub, GitLab, and Bitbucket
so that multi-provider support is regression-tested.
