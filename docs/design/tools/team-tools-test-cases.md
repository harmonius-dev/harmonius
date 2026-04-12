# Team Tools Test Cases

Companion test cases for [team-tools.md](team-tools.md).

## Unit Tests

| ID | Req | Description |
|----|-----|-------------|
| TC-15.12.1.1 | R-15.12.1 | 1 |
| TC-15.12.2.1 | R-15.12.2 | 2 |
| TC-15.12.6.1 | R-15.12.6 | 3 |
| TC-15.10.1.1 | R-15.10.1 | 4 |
| TC-15.10.1.2 | R-15.10.1 | 5 |
| TC-15.10.1.3 | R-15.10.1 | 6 |
| TC-15.10.2.1 | R-15.10.2 | 7 |
| TC-15.10.2.2 | R-15.10.2 | 8 |
| TC-15.10.2.3 | R-15.10.2 | 9 |
| TC-15.10.2.4 | R-15.10.2 | 10 |
| TC-15.10.3.1 | R-15.10.3 | 11 |
| TC-15.10.3.2 | R-15.10.3 | 12 |
| TC-15.10.3.3 | R-15.10.3 | 13 |
| TC-15.10.3.4 | R-15.10.3 | 14 |
| TC-15.10.3.5 | R-15.10.3 | 15 |
| TC-15.10.3.6 | R-15.10.3 | 16 |
| TC-15.10.4.1 | R-15.10.4 | 17 |
| TC-15.10.5.1 | R-15.10.5 | 18 |
| TC-15.10.6.1 | R-15.10.6 | 19 |
| TC-15.10.7.1 | R-15.10.7 | 20 |
| TC-15.10.8.1 | R-15.10.8 | 21 |
| TC-15.12.12.1 | R-15.12.12 | 22 |
| TC-15.10.3.7 | R-15.10.3 | 23 |
| TC-15.10.3.8 | R-15.10.3 | 24 |
| TC-15.10.3.9 | R-15.10.3 | 25 |
| TC-15.10.3.10 | R-15.10.3 | 26 |
| TC-15.12.4.1 | R-15.12.4 | 27 |
| TC-15.12.13.1 | R-15.12.13 | 28 |
| TC-15.12.14.1 | R-15.12.14 | 29 |

1. Video encode pipeline produces H.264/H.265 frames
2. QUIC transport round-trip delivers input and frames
3. Bandwidth tier selection adapts stream quality
4. Git status parsing returns correct file states
5. Git stage and commit produce valid commit via libgit2
6. Git push with platform credentials succeeds
7. LFS tracking rules match by extension and size
8. LFS lock acquires server-side lock for path
9. LFS unlock releases lock on commit or discard
10. LFS locks query returns all current locks with holders
11. Structural merge with no conflicts produces clean merge
12. Structural merge with conflicts opens conflict UI
13. Binary conflict parse deserializes base/ours/theirs
14. Binary conflict diff reports per-property changes
15. Binary conflict resolve with ours pick keeps ours value
16. Binary conflict resolve with theirs pick keeps theirs
17. Branch create and switch updates HEAD correctly
18. LFS lock attempt on locked path returns lock holder
19. Sparse checkout config filters worktree correctly
20. Shelf create and apply round-trips WIP changes
21. Hosting provider detect identifies GitHub/GitLab/etc.
22. AI agent session joins as virtual user
23. Binary conflict resolve with Manual pick uses edited value
24. Binary conflict diff on nested properties reports full path
25. Binary conflict resolve with mixed picks across properties
26. Binary conflict parse fails gracefully on unknown type
27. Remote GPU server allocates a session with isolated VRAM partition and returns session token
28. Asset comment thread creation persists author, target ref, body, and timestamp
29. PR review viewer renders diff hunks and accepts inline comment submission

## Integration Tests

| ID | Req | Description |
|----|-----|-------------|
| TC-15.12.I.1 | R-15.12.1 | 1 |
| TC-15.12.I.2 | R-15.12.5 | 2 |
| TC-15.10.I.1 | R-15.10.1 | 3 |
| TC-15.10.I.2 | R-15.10.3 | 4 |
| TC-15.10.I.3 | R-15.10.3 | 5 |
| TC-15.10.I.4 | R-15.10.6 | 6 |
| TC-15.10.I.5 | R-15.10.2 | 7 |
| TC-15.10.I.6 | R-15.10.2 | 8 |
| TC-15.10.I.7 | R-15.10.5 | 9 |
| TC-15.10.I.8 | R-15.10.3 | 10 |

1. Remote rendering session start/stop round-trip
2. Session suspend and resume preserves full state
3. Commit-push-pull cycle with LFS tracked files
4. Structural merge conflict resolution end-to-end
5. Binary conflict resolution: parse, diff, pick, write
6. Partial clone workflow with sparse checkout
7. LFS lock contention between two editors
8. Lock-edit-commit-unlock full cycle releases lock
9. Lock attempt returns holder info for locked asset
10. Side-by-side conflict UI renders and resolves per-property

## Benchmarks

| ID | Req | Description |
|----|-----|-------------|
| TC-15.12.B.1 | R-15.12.1 | 1 |
| TC-15.10.B.1 | R-15.10.3 | 2 |
| TC-15.10.B.2 | R-15.10.3 | 3 |
| TC-15.10.B.3 | R-15.10.1 | 4 |

1. Remote frame latency under load
2. Structural merge latency for large scene files
3. Binary conflict resolution latency for large assets
4. Git clone time for repositories with LFS objects
