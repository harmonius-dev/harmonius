# Version Control Integration Test Cases

Companion test cases for [version-control.md](version-control.md).

## Unit Tests

### TC-15.10.2.1 Stage LFS Auto-Tracking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stage `character.png` with rule `"*.png"` | LFS pointer file created, `is_lfs_tracked("character.png") == true` | R-15.10.2 |
| 2 | Stage `readme.txt` with no matching rule | direct staging via libgit2, `is_lfs_tracked("readme.txt") == false` | R-15.10.2 |

### TC-15.10.1.1 Stage Non-LFS File

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stage `config.ron`, no LFS rules match | file staged directly in index, `FileStatus.is_lfs == false` | R-15.10.1 |

### TC-15.10.2.2 LFS Size Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rule `{ pattern: "*", min_size: Some(1_000_000) }`, stage 2 MB file | LFS pointer created | R-15.10.2 |
| 2 | Same rule, stage 500 KB file | direct staging (below threshold) | R-15.10.2 |

### TC-15.10.3.1 Structural Merge Clean

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base asset, ours modifies field A, theirs modifies field B | `MergeResult::Clean { merged }` containing both changes | R-15.10.3 |

### TC-15.10.3.2 Structural Merge Conflict

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base asset, ours sets position=(1,0,0), theirs sets position=(0,1,0) | `MergeResult::Conflicted` with 1 `MergeConflict` on position path | R-15.10.3 |

### TC-15.10.3.3 Merge Driver Registration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `register(repo_path)`, read `.gitattributes` | contains `*.logicgraph merge=harmonius` entry | R-15.10.3 |

### TC-15.10.4.1 Branch Create and Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `create("feature-x", "main")`, `switch("feature-x")` | HEAD points to `feature-x`, same commit as `main` | R-15.10.4 |

### TC-15.10.4.2 Branch Switch Warns Dirty

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify file, attempt `switch("other-branch")` | returns warning or error indicating dirty working tree | R-15.10.4 |

### TC-15.10.7.1 Shelf Create and Apply

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify 3 files, `create("wip-shelf")`, working tree reverts | `ShelfInfo.file_count == 3`, working tree clean | R-15.10.7 |
| 2 | `apply(shelf_id)` | 3 files restored to modified state, `MergeResult::Clean` | R-15.10.7 |

### TC-15.10.7.2 Shelf Structural Merge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shelve changes to file X, modify different fields in X on working tree, apply shelf | structural merge succeeds, both changes present | R-15.10.7 |

### TC-15.10.8.1 Provider Detect GitHub

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `detect("https://github.com/org/repo.git")` | `Some(GitHubProvider)` | R-15.10.8 |
| 2 | `detect("git@github.com:org/repo.git")` | `Some(GitHubProvider)` | R-15.10.8 |

### TC-15.10.8.2 Provider Detect GitLab

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `detect("https://gitlab.com/org/repo.git")` | `Some(GitLabProvider)` | R-15.10.8 |

### TC-15.10.6.1 Sparse Checkout Patterns

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Role "Artist" with patterns `["assets/textures/**", "assets/meshes/**"]` | sparse checkout config contains both patterns | R-15.10.6 |
| 2 | Role "Programmer" with patterns `["src/**", "Cargo.toml"]` | sparse checkout config contains both patterns | R-15.10.6 |

### TC-15.10.1.2 Changelist Grouping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Assign files A, B to changelist "UI fixes", file C to "Physics" | changelist "UI fixes" contains [A, B], "Physics" contains [C] | R-15.10.1 |

### TC-15.10.1.3 Credential Store Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store credential for `"github.com"`, retrieve | retrieved credential matches stored value | R-15.10.1 |
| 2 | Retrieve credential for `"unknown.host"` | returns `None` or error | R-15.10.1 |

## Integration Tests

### TC-15.10.1.I1 Commit Push Pull

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stage files, commit, push to local bare repo, clone into second dir, verify | second clone has committed files with correct content | R-15.10.1 |

### TC-15.10.2.I1 LFS Lock and Unlock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `lock("asset.fbx")`, `locks()` | lock list contains `"asset.fbx"` with correct owner | R-15.10.2 |
| 2 | `unlock("asset.fbx")`, `locks()` | lock list no longer contains `"asset.fbx"` | R-15.10.2 |

### TC-15.10.3.I1 Merge Driver Git Invoke

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two branches with non-overlapping asset edits, `git merge` | custom merge driver invoked, clean merge output, valid asset | R-15.10.3 |

### TC-15.10.4.I1 Branch Switch Cache Preserved

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compile assets on branch A, switch to branch B where 80% of sources unchanged | 80% of cache entries survive, 20% evicted | R-15.10.4 |

### TC-15.10.5.I1 Presence WebSocket

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Instance A calls `broadcast_open(asset_X)`, instance B reads `active_editors()` | B sees user A editing asset_X | R-15.10.5 |

### TC-15.10.5.I2 Presence Fallback LFS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | WebSocket disconnected, user A locks asset_X via LFS | other users see A's presence via LFS lock polling within 5 seconds | R-15.10.5 |

### TC-15.10.6.I1 Partial Clone Size

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Blobless clone vs full clone of same repo | blobless clone size < 50% of full clone size | R-15.10.6 |

### TC-15.10.6.I2 On-Demand Fetch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sparse checkout missing blob, access triggers fetch | blob fetched on demand, operation completes without blocking UI thread | R-15.10.6 |

### TC-15.10.7.I1 Shelf Share via Cache

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A creates shelf, shares via cache, User B downloads | User B can `apply` shelf, modifications match A's original changes | R-15.10.7 |

### TC-15.10.8.I1 PR Create GitHub

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `create_pr("Fix bug", "Description", "main")` against GitHub | `PrInfo` returned with `state == Open`, URL valid | R-15.10.8 |

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
