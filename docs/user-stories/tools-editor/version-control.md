# User Stories: Version Control and Git Integration

## F-15.10.1 Native Git Integration

## US-15.10.1.1 Designer Uses Git In-Editor

**As a** designer (P-5), **I want** to stage, commit, push, pull, branch, merge, rebase, and stash
through the editor UI without a command line, **so that** I can manage version control without
leaving my workspace.

## US-15.10.1.2 Developer Browses Commit History

**As a** developer (P-15), **I want** a commit history browser with a visual branch graph and
per-asset structural diffs, **so that** I can review project history and understand changes at the
asset level.

## US-15.10.1.3 Engine Dev Uses libgit2 Directly

**As an** engine developer (P-26), **I want** all Git operations implemented via libgit2 for direct
repository access, **so that** shell-out overhead is eliminated and behavior is consistent across
platforms.

## US-15.10.1.4 Engine Tester Validates Platform Credentials

**As an** engine tester (P-27), **I want** to verify that SSH key access uses platform-native
credential stores (Keychain on macOS, Credential Manager on Windows, libsecret on Linux),
**so that** authentication is secure on each platform.

## F-15.10.2 Git LFS Management

## US-15.10.2.1 Artist Locks LFS Assets

**As an** artist (P-8), **I want** automatic LFS tracking for binary assets based on file extension
and size thresholds with lock/unlock from the asset browser context menu, **so that** I can prevent
concurrent edits on large binary files.

## US-15.10.2.2 DevOps Manages Bulk LFS Operations

**As a** DevOps engineer (P-16), **I want** bulk LFS operations (migrate, fetch, prune) accessible
from the version control panel, **so that** I can manage LFS storage without CLI commands.

## US-15.10.2.3 Server Admin Monitors Storage Quota

**As a** server admin (P-22), **I want** an LFS storage quota monitor showing current usage against
team limits with configurable warnings, **so that** I can act before storage runs out.

## US-15.10.2.4 Engine Tester Validates Platform TLS

**As an** engine tester (P-27), **I want** to verify that LFS transfers use the platform-native TLS
stack (Secure Transport on macOS, Schannel on Windows, OpenSSL on Linux), **so that** transfers are
secure and performant.

## F-15.10.3 Asset-Aware Merge Driver

## US-15.10.3.1 Developer Merges Binary Assets Structurally

**As a** developer (P-15), **I want** a custom Git merge driver that performs three-way structural
merge on binary assets (logic graphs, material graphs, prefabs, data tables), **so that** concurrent
edits are resolved at the node/property level instead of failing as binary conflicts.

## US-15.10.3.2 Designer Resolves Conflicts Visually

**As a** designer (P-5), **I want** unresolvable conflicts to open in the visual diff tool with
per-node accept-mine/accept-theirs options, **so that** I can resolve merge conflicts without
understanding raw binary data.

## US-15.10.3.3 DevOps Registers Driver Automatically

**As a** DevOps engineer (P-16), **I want** the merge driver registered automatically on project
clone via `.gitattributes`, **so that** every developer gets structural merge without manual
configuration.

## US-15.10.3.4 Engine Tester Validates Merge Correctness

**As an** engine tester (P-27), **I want** to verify that three-way structural merge produces valid
assets when changes do not conflict, **so that** automated merges can be trusted.

## F-15.10.4 Branch-Per-Feature Workflow

## US-15.10.4.1 Designer Creates Feature Branch

**As a** designer (P-5), **I want** to create feature branches, switch branches with asset cache
preservation, and create pull requests from the editor UI, **so that** I can follow a
branch-per-feature workflow without external tools.

## US-15.10.4.2 Developer Sees Branch Graph

**As a** developer (P-15), **I want** the branch selector to display the branch graph inline with
remote tracking status and ahead/behind counts, **so that** I can understand the branching state at
a glance.

## US-15.10.4.3 DevOps Integrates Multiple Providers

**As a** DevOps engineer (P-16), **I want** pull/merge request creation to integrate with GitHub,
GitLab, Bitbucket, and Azure DevOps APIs, **so that** the workflow works with our hosting provider.

## US-15.10.4.4 Engine Tester Validates Stash on Switch

**As an** engine tester (P-27), **I want** to verify that branch switching warns about unsaved
changes and offers to stash them, **so that** no work is lost during branch transitions.

## F-15.10.5 Collaborative Presence

## US-15.10.5.1 Artist Sees Who Is Editing What

**As an** artist (P-8), **I want** real-time presence indicators showing which team members are
editing which assets with colored avatar badges, **so that** I avoid opening assets someone else is
actively modifying.

## US-15.10.5.2 Designer Requests Lock Queuing

**As a** designer (P-5), **I want** lock requests to queue automatically when an asset is already
locked with notification to the lock holder, **so that** I do not need to repeatedly check whether a
locked asset is available.

## US-15.10.5.3 Server Admin Configures Pessimistic Locking

**As a** server admin (P-22), **I want** optional pessimistic locking for assets that do not merge
well (heightmaps, lightmaps, navmeshes), **so that** concurrent edits on non-mergeable assets are
prevented.

## US-15.10.5.4 Engine Tester Validates Presence Fallback

**As an** engine tester (P-27), **I want** to verify that presence falls back to polling LFS lock
state when the WebSocket presence service is unavailable, **so that** presence information is always
available.

## F-15.10.6 Partial Clone and Sparse Checkout

## US-15.10.6.1 Artist Downloads Only Art Assets

**As an** artist (P-8), **I want** sparse checkout patterns configurable per team role so I receive
only art assets, **so that** I do not download source code and configuration files that I never
open.

## US-15.10.6.2 Designer Fetches Missing Assets On-Demand

**As a** designer (P-5), **I want** missing assets to display placeholder thumbnails with a
one-click on-demand fetch action, **so that** I can pull specific assets as needed without
downloading the full repository.

## US-15.10.6.3 DevOps Enables Partial Clone

**As a** DevOps engineer (P-16), **I want** the editor to support Git partial clone (blobless and
treeless modes) for massive repositories, **so that** initial clone time is reduced from hours to
minutes.

## US-15.10.6.4 Engine Tester Validates Background Fetch

**As an** engine tester (P-27), **I want** to verify that on-demand fetch operations run in the
background without blocking the editor, **so that** designers can continue working while assets
download.

## F-15.10.7 Shelving and Local Stash

## US-15.10.7.1 Developer Shelves Work-in-Progress

**As a** developer (P-15), **I want** to save work-in-progress changes as named shelves without
committing, **so that** I can switch context to another task and return to my changes later.

## US-15.10.7.2 Designer Shares Shelves with Team

**As a** designer (P-5), **I want** shelves shareable via the shared cache for handoff scenarios,
**so that** I can pass in-progress work to a teammate who continues where I left off.

## US-15.10.7.3 Tech Artist Merges Shelf Structurally

**As a** tech artist (P-13), **I want** applying a shelf to merge shelved changes with the current
working state using the structural merge system, **so that** overlapping edits are resolved rather
than overwriting.

## US-15.10.7.4 Engine Tester Validates Shelf Storage

**As an** engine tester (P-27), **I want** to verify that shelf data is stored in the project's
`.harmonius/shelves/` directory and excluded from version control, **so that** shelves do not
pollute the repository.

## F-15.10.8 Multi-Provider Git Hosting Support

## US-15.10.8.1 Developer Works with Any Provider

**As a** developer (P-15), **I want** first-class support for GitHub, GitLab, Bitbucket, and
self-hosted Git servers through a provider abstraction layer, **so that** switching hosting
providers does not require editor reconfiguration.

## US-15.10.8.2 DevOps Auto-Detects Provider

**As a** DevOps engineer (P-16), **I want** the editor to auto-detect the provider from the remote
URL and store API tokens in the platform credential store, **so that** initial setup is automatic.

## US-15.10.8.3 Designer Reviews PRs In-Editor

**As a** designer (P-5), **I want** all provider interactions (PR creation, review, merge, comment)
to happen within the editor, **so that** I never need to open a browser for Git hosting operations.

## US-15.10.8.4 Engine Tester Validates Provider APIs

**As an** engine tester (P-27), **I want** to verify that the editor works correctly with GitHub
REST v3/GraphQL v4, GitLab REST v4, Bitbucket REST 2.0, and self-hosted instances with configurable
base URLs, **so that** all supported providers function identically.
