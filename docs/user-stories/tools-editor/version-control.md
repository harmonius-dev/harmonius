# User Stories: Version Control and Git Integration

## F-15.10.1 Native Git Integration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.10.1.1 | designer (P-5) | to stage, commit, push, pull, branch, merge, rebase, and stash through the editor UI without a command line | I can manage version control without leaving my workspace |  |  |
| US-15.10.1.2 | developer (P-15) | a commit history browser with a visual branch graph and per-asset structural diffs | I can review project history and understand changes at the asset level |  |  |
| US-15.10.1.3 | engine developer (P-26) | all Git operations implemented via libgit2 for direct repository access | shell-out overhead is eliminated and behavior is consistent across platforms |  |  |
| US-15.10.1.4 | engine tester (P-27) | to verify that SSH key access uses platform-native credential stores (Keychain on macOS, Credential Manager on Windows, libsecret on Linux) | authentication is secure on each platform |  |  |

## F-15.10.2 Git LFS Management

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.10.2.1 | artist (P-8) | automatic LFS tracking for binary assets based on file extension and size thresholds with lock/unlock from the asset browser context menu | I can prevent concurrent edits on large binary files |  |  |
| US-15.10.2.2 | DevOps engineer (P-16) | bulk LFS operations (migrate, fetch, prune) accessible from the version control panel | I can manage LFS storage without CLI commands |  |  |
| US-15.10.2.3 | server admin (P-22) | an LFS storage quota monitor showing current usage against team limits with configurable warnings | I can act before storage runs out |  |  |
| US-15.10.2.4 | engine tester (P-27) | to verify that LFS transfers use the platform-native TLS stack (Secure Transport on macOS, Schannel on Windows, OpenSSL on Linux) | transfers are secure and performant |  |  |

## F-15.10.3 Asset-Aware Merge Driver

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.10.3.1 | developer (P-15) | a custom Git merge driver that performs three-way structural merge on binary assets (logic graphs, material graphs, prefabs, data tables) | concurrent edits are resolved at the node/property level instead of failing as binary conflicts |  |  |
| US-15.10.3.2 | designer (P-5) | unresolvable conflicts to open in the visual diff tool with per-node accept-mine/accept-theirs options | I can resolve merge conflicts without understanding raw binary data |  |  |
| US-15.10.3.3 | DevOps engineer (P-16) | the merge driver registered automatically on project clone via `.gitattributes` | every developer gets structural merge without manual configuration |  |  |
| US-15.10.3.4 | engine tester (P-27) | to verify that three-way structural merge produces valid assets when changes do not conflict | automated merges can be trusted |  |  |

## F-15.10.4 Branch-Per-Feature Workflow

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.10.4.1 | designer (P-5) | to create feature branches, switch branches with asset cache preservation, and create pull requests from the editor UI | I can follow a branch-per-feature workflow without external tools |  |  |
| US-15.10.4.2 | developer (P-15) | the branch selector to display the branch graph inline with remote tracking status and ahead/behind counts | I can understand the branching state at a glance |  |  |
| US-15.10.4.3 | DevOps engineer (P-16) | pull/merge request creation to integrate with GitHub, GitLab, Bitbucket, and Azure DevOps APIs | the workflow works with our hosting provider |  |  |
| US-15.10.4.4 | engine tester (P-27) | to verify that branch switching warns about unsaved changes and offers to stash them | no work is lost during branch transitions |  |  |

## F-15.10.5 Collaborative Presence

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.10.5.1 | artist (P-8) | real-time presence indicators showing which team members are editing which assets with colored avatar badges | I avoid opening assets someone else is actively modifying |  |  |
| US-15.10.5.2 | designer (P-5) | lock requests to queue automatically when an asset is already locked with notification to the lock holder | I do not need to repeatedly check whether a locked asset is available |  |  |
| US-15.10.5.3 | server admin (P-22) | optional pessimistic locking for assets that do not merge well (heightmaps, lightmaps, navmeshes) | concurrent edits on non-mergeable assets are prevented |  |  |
| US-15.10.5.4 | engine tester (P-27) | to verify that presence falls back to polling LFS lock state when the WebSocket presence service is unavailable | presence information is always available |  |  |

## F-15.10.6 Partial Clone and Sparse Checkout

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.10.6.1 | artist (P-8) | sparse checkout patterns configurable per team role so I receive only art assets | I do not download source code and configuration files that I never open |  |  |
| US-15.10.6.2 | designer (P-5) | missing assets to display placeholder thumbnails with a one-click on-demand fetch action | I can pull specific assets as needed without downloading the full repository |  |  |
| US-15.10.6.3 | DevOps engineer (P-16) | the editor to support Git partial clone (blobless and treeless modes) for massive repositories | initial clone time is reduced from hours to minutes |  |  |
| US-15.10.6.4 | engine tester (P-27) | to verify that on-demand fetch operations run in the background without blocking the editor | designers can continue working while assets download |  |  |

## F-15.10.7 Shelving and Local Stash

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.10.7.1 | developer (P-15) | to save work-in-progress changes as named shelves without committing | I can switch context to another task and return to my changes later |  |  |
| US-15.10.7.2 | designer (P-5) | shelves shareable via the shared cache for handoff scenarios | I can pass in-progress work to a teammate who continues where I left off |  |  |
| US-15.10.7.3 | tech artist (P-13) | applying a shelf to merge shelved changes with the current working state using the structural merge system | overlapping edits are resolved rather than overwriting |  |  |
| US-15.10.7.4 | engine tester (P-27) | to verify that shelf data is stored in the project's `.harmonius/shelves/` directory and excluded from version control | shelves do not pollute the repository |  |  |

## F-15.10.8 Multi-Provider Git Hosting Support

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.10.8.1 | developer (P-15) | first-class support for GitHub, GitLab, Bitbucket, and self-hosted Git servers through a provider abstraction layer | switching hosting providers does not require editor reconfiguration |  |  |
| US-15.10.8.2 | DevOps engineer (P-16) | the editor to auto-detect the provider from the remote URL and store API tokens in the platform credential store | initial setup is automatic |  |  |
| US-15.10.8.3 | designer (P-5) | all provider interactions (PR creation, review, merge, comment) to happen within the editor | I never need to open a browser for Git hosting operations |  |  |
| US-15.10.8.4 | engine tester (P-27) | to verify that the editor works correctly with GitHub REST v3/GraphQL v4, GitLab REST v4, Bitbucket REST 2.0, and self-hosted instances with configurable base URLs | all supported providers function identically |  |  |
