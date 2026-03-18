# Version Control Integration Test Cases

Companion test cases for [version-control.md](version-control.md).

## Unit Tests

### TC-15.10.2.1 Stage LFS Auto-Tracking

| # | Requirement |
|---|-------------|
| 1 | R-15.10.2   |
| 2 | R-15.10.2   |

1. **#1** — Stage `character.png` with rule `"*.png"`
   - **Expected:** LFS pointer file created, `is_lfs_tracked("character.png") == true`
2. **#2** — Stage `readme.txt` with no matching rule
   - **Expected:** direct staging via libgit2, `is_lfs_tracked("readme.txt") == false`

### TC-15.10.1.1 Stage Non-LFS File

| # | Requirement |
|---|-------------|
| 1 | R-15.10.1   |

1. **#1** — Stage `config.ron`, no LFS rules match
   - **Expected:** file staged directly in index, `FileStatus.is_lfs == false`

### TC-15.10.2.2 LFS Size Threshold

| # | Requirement |
|---|-------------|
| 1 | R-15.10.2   |
| 2 | R-15.10.2   |

1. **#1** — Rule `{ pattern: "*", min_size: Some(1_000_000) }`, stage 2 MB file
   - **Expected:** LFS pointer created
2. **#2** — Same rule, stage 500 KB file
   - **Expected:** direct staging (below threshold)

### TC-15.10.3.1 Structural Merge Clean

| # | Requirement |
|---|-------------|
| 1 | R-15.10.3   |

1. **#1** — Base asset, ours modifies field A, theirs modifies field B
   - **Expected:** `MergeResult::Clean { merged }` containing both changes

### TC-15.10.3.2 Structural Merge Conflict

| # | Requirement |
|---|-------------|
| 1 | R-15.10.3   |

1. **#1** — Base asset, ours sets position=(1,0,0), theirs sets position=(0,1,0)
   - **Expected:** `MergeResult::Conflicted` with 1 `MergeConflict` on position path

### TC-15.10.3.3 Merge Driver Registration

| # | Requirement |
|---|-------------|
| 1 | R-15.10.3   |

1. **#1** — `register(repo_path)`, read `.gitattributes`
   - **Expected:** contains `*.logicgraph merge=harmonius` entry

### TC-15.10.4.1 Branch Create and Switch

| # | Requirement |
|---|-------------|
| 1 | R-15.10.4   |

1. **#1** — `create("feature-x", "main")`, `switch("feature-x")`
   - **Expected:** HEAD points to `feature-x`, same commit as `main`

### TC-15.10.4.2 Branch Switch Warns Dirty

| # | Requirement |
|---|-------------|
| 1 | R-15.10.4   |

1. **#1** — Modify file, attempt `switch("other-branch")`
   - **Expected:** returns warning or error indicating dirty working tree

### TC-15.10.7.1 Shelf Create and Apply

| # | Requirement |
|---|-------------|
| 1 | R-15.10.7   |
| 2 | R-15.10.7   |

1. **#1** — Modify 3 files, `create("wip-shelf")`, working tree reverts
   - **Expected:** `ShelfInfo.file_count == 3`, working tree clean
2. **#2** — `apply(shelf_id)`
   - **Expected:** 3 files restored to modified state, `MergeResult::Clean`

### TC-15.10.7.2 Shelf Structural Merge

| # | Requirement |
|---|-------------|
| 1 | R-15.10.7   |

1. **#1** — Shelve changes to file X, modify different fields in X on working tree, apply shelf
   - **Expected:** structural merge succeeds, both changes present

### TC-15.10.8.1 Provider Detect GitHub

| # | Requirement |
|---|-------------|
| 1 | R-15.10.8   |
| 2 | R-15.10.8   |

1. **#1** — `detect("https://github.com/org/repo.git")`
   - **Expected:** `Some(GitHubProvider)`
2. **#2** — `detect("git@github.com:org/repo.git")`
   - **Expected:** `Some(GitHubProvider)`

### TC-15.10.8.2 Provider Detect GitLab

| # | Requirement |
|---|-------------|
| 1 | R-15.10.8   |

1. **#1** — `detect("https://gitlab.com/org/repo.git")`
   - **Expected:** `Some(GitLabProvider)`

### TC-15.10.6.1 Sparse Checkout Patterns

| # | Requirement |
|---|-------------|
| 1 | R-15.10.6   |
| 2 | R-15.10.6   |

1. **#1** — Role "Artist" with patterns `["assets/textures/**", "assets/meshes/**"]`
   - **Expected:** sparse checkout config contains both patterns
2. **#2** — Role "Programmer" with patterns `["src/**", "Cargo.toml"]`
   - **Expected:** sparse checkout config contains both patterns

### TC-15.10.1.2 Changelist Grouping

| # | Requirement |
|---|-------------|
| 1 | R-15.10.1   |

1. **#1** — Assign files A, B to changelist "UI fixes", file C to "Physics"
   - **Expected:** changelist "UI fixes" contains [A, B], "Physics" contains [C]

### TC-15.10.1.3 Credential Store Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-15.10.1   |
| 2 | R-15.10.1   |

1. **#1** — Store credential for `"github.com"`, retrieve
   - **Expected:** retrieved credential matches stored value
2. **#2** — Retrieve credential for `"unknown.host"`
   - **Expected:** returns `None` or error

## Integration Tests

### TC-15.10.1.I1 Commit Push Pull

| # | Requirement |
|---|-------------|
| 1 | R-15.10.1   |

1. **#1** — Stage files, commit, push to local bare repo, clone into second dir, verify
   - **Expected:** second clone has committed files with correct content

### TC-15.10.2.I1 LFS Lock and Unlock

| # | Requirement |
|---|-------------|
| 1 | R-15.10.2   |
| 2 | R-15.10.2   |

1. **#1** — `lock("asset.fbx")`, `locks()`
   - **Expected:** lock list contains `"asset.fbx"` with correct owner
2. **#2** — `unlock("asset.fbx")`, `locks()`
   - **Expected:** lock list no longer contains `"asset.fbx"`

### TC-15.10.3.I1 Merge Driver Git Invoke

| # | Requirement |
|---|-------------|
| 1 | R-15.10.3   |

1. **#1** — Two branches with non-overlapping asset edits, `git merge`
   - **Expected:** custom merge driver invoked, clean merge output, valid asset

### TC-15.10.4.I1 Branch Switch Cache Preserved

| # | Requirement |
|---|-------------|
| 1 | R-15.10.4   |

1. **#1** — Compile assets on branch A, switch to branch B where 80% of sources unchanged
   - **Expected:** 80% of cache entries survive, 20% evicted

### TC-15.10.5.I1 Presence WebSocket

| # | Requirement |
|---|-------------|
| 1 | R-15.10.5   |

1. **#1** — Instance A calls `broadcast_open(asset_X)`, instance B reads `active_editors()`
   - **Expected:** B sees user A editing asset_X

### TC-15.10.5.I2 Presence Fallback LFS

| # | Requirement |
|---|-------------|
| 1 | R-15.10.5   |

1. **#1** — WebSocket disconnected, user A locks asset_X via LFS
   - **Expected:** other users see A's presence via LFS lock polling within 5 seconds

### TC-15.10.6.I1 Partial Clone Size

| # | Requirement |
|---|-------------|
| 1 | R-15.10.6   |

1. **#1** — Blobless clone vs full clone of same repo
   - **Expected:** blobless clone size < 50% of full clone size

### TC-15.10.6.I2 On-Demand Fetch

| # | Requirement |
|---|-------------|
| 1 | R-15.10.6   |

1. **#1** — Sparse checkout missing blob, access triggers fetch
   - **Expected:** blob fetched on demand, operation completes without blocking UI thread

### TC-15.10.7.I1 Shelf Share via Cache

| # | Requirement |
|---|-------------|
| 1 | R-15.10.7   |

1. **#1** — User A creates shelf, shares via cache, User B downloads
   - **Expected:** User B can `apply` shelf, modifications match A's original changes

### TC-15.10.8.I1 PR Create GitHub

| # | Requirement |
|---|-------------|
| 1 | R-15.10.8   |

1. **#1** — `create_pr("Fix bug", "Description", "main")` against GitHub
   - **Expected:** `PrInfo` returned with `state == Open`, URL valid

## Benchmarks

### TC-15.10.1.B1 Status of 100k Files

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `status()` on repository with 100,000 files | latency | < 500 ms | R-15.10.1 |

### TC-15.10.1.B2 Stage 1000 Files

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `stage()` 1000 files into index | latency | < 200 ms | R-15.10.1 |

### TC-15.10.1.B3 Commit Creation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `commit()` with 1000 staged files | latency | < 50 ms | R-15.10.1 |

### TC-15.10.4.B1 Branch Switch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Switch branch with no asset rebuild needed | latency | < 2 s | R-15.10.4 |

### TC-15.10.3.B1 Structural Merge

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Three-way structural merge of 1 MB asset | latency | < 100 ms | R-15.10.3 |

### TC-15.10.5.B1 Presence Broadcast Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Broadcast open event, measure receipt at peer | latency | < 100 ms | R-15.10.5 |
