# Remote Collaboration Test Cases

Companion test cases for [collaboration.md](collaboration.md).

## Unit Tests

### TC-15.12.8.1 Tree CRDT Add Remove

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_node(root, data)` | Node present in tree, parent is root | R-15.12.8 |
| 2 | `remove_node(child_id)` | Node absent from tree, children reparented or removed | R-15.12.8 |
| 3 | Add 100 nodes, remove 50 | Tree contains exactly 50 nodes plus root | R-15.12.8 |

### TC-15.12.8.2 Tree CRDT Concurrent Reparent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A: `reparent(N, P1)`, User B: `reparent(N, P2)`, merge | Node N has one parent (LWW by HLC), no cycles | R-15.12.8 |
| 2 | Both reparent different nodes | Both reparents applied, tree consistent | R-15.12.8 |

### TC-15.12.8.3 OpLog Concurrent Connect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A: `connect(src1, dst1)`, User B: `connect(src2, dst2)`, merge | Both connections present in graph | R-15.12.8 |

### TC-15.12.8.4 Map CRDT Concurrent Set

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A: `set("hp", 100)` at HLC t1, User B: `set("hp", 200)` at HLC t2 > t1, merge | `get("hp") == 200` (higher HLC wins) | R-15.12.8 |

### TC-15.12.8.5 LWW Register Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `write(data_A, hlc_1)`, `apply_remote(data_B, hlc_2)` where hlc_2 > hlc_1 | `read() == data_B` | R-15.12.8 |
| 2 | `write(data_A, hlc_2)`, `apply_remote(data_B, hlc_1)` where hlc_2 > hlc_1 | `read() == data_A` (local wins, higher HLC) | R-15.12.8 |

### TC-15.12.3.1 HLC Monotonic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Call `now()` 1000 times in sequence | Each timestamp strictly greater than the previous | R-15.12.3 |
| 2 | `update(remote_ts)` where remote is ahead | Local clock advances past remote timestamp | R-15.12.3 |

### TC-15.12.3.2 Per-User Undo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A records 3 ops, user B records 2 ops, user A undoes 1 | User A has 2 ops, user B still has 2 ops | R-15.12.3 |
| 2 | User A undoes all, user B state unchanged | User B ops remain applied | R-15.12.3 |

### TC-15.12.3.3 Presence Cursor Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_cursor_position([1.0, 2.0, 3.0])` | `remote_cursors()` on other client shows position `[1.0, 2.0, 3.0]` | R-15.12.3 |

### TC-15.12.9.1 Role Viewer No Edit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User with `Role::Viewer` calls `push_ops([edit_op])` | Returns `Err(PermissionDenied { action: Edit })` | R-15.12.9 |
| 2 | User with `Role::Editor` calls `push_ops([edit_op])` | Returns `Ok(())` | R-15.12.9 |

### TC-15.12.9.2 Role Admin Set Role

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Admin calls `set_role(user_B, Editor)` | User B role changed to Editor | R-15.12.9 |
| 2 | Editor calls `set_role(user_B, Viewer)` | Returns `Err(PermissionDenied)` | R-15.12.9 |

### TC-15.12.6.1 Bandwidth Tier Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Measured bandwidth 150 Mbps | `current_tier() == QualityTier::High` | R-15.12.6 |
| 2 | Measured bandwidth 50 Mbps | `current_tier() == QualityTier::Medium` | R-15.12.6 |
| 3 | Measured bandwidth 5 Mbps | `current_tier() == QualityTier::Low` | R-15.12.6 |

### TC-15.12.11.1 Workgroup Isolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Group A adds entity, group B queries entities | Group B does not see group A's entity | R-15.12.11 |
| 2 | Group A shares workspace | Entity now visible to all groups | R-15.12.11 |

## Integration Tests

### TC-15.12.3.I1 Three User Convergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Users A, B, C each push 10 concurrent ops | All three local states are identical after sync | R-15.12.3 |

### TC-15.12.5.I1 Session Suspend Resume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Suspend session on client A, resume on client B | Camera position, selection, and undo history match pre-suspend state | R-15.12.5 |

### TC-15.12.3.I2 P2P LAN Discovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two editors on same LAN, `connect_p2p()` | Both discover each other via mDNS, CRDT sync succeeds | R-15.12.3 |

### TC-15.12.2.I1 QUIC to TCP Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Block UDP on firewall, start remote stream | Stream falls back to TCP+TLS 1.3, video received | R-15.12.2 |

### TC-15.12.7.I1 100 Concurrent Sessions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create 100 sessions, each pushing 10 ops | All sessions converge, no data loss | R-15.12.7 |

### TC-15.12.9.I1 OAuth2 Authentication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | OAuth2 flow with mock provider | Session created with correct `Role` from token claims | R-15.12.9 |

### TC-15.12.10.I1 Chat Message Delivery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A sends "hello", user B receives | Message content matches "hello", correct author | R-15.12.10 |

### TC-15.12.10.I2 Chat Search

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send 50 messages, search for keyword in message 25 | Search returns message 25 | R-15.12.10 |

### TC-15.12.1.I1 Encoding Overhead

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Encode 1920x1080 frame via HW encoder | `last_encode_us() < 2000` | R-15.12.1 |

### TC-15.12.14.I1 PR Review Structural Diff

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | PR with changed scene asset, view diff in editor | Structural diff shows added/removed/modified entities | R-15.12.14 |

## Benchmarks

### TC-15.12.3.B1 CRDT Op Broadcast Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Push op from client A, measure until client B receives (LAN) | Latency | < 50 ms | R-15.12.3 |

### TC-15.12.4.B1 Remote Rendering Round Trip

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Input event to rendered frame received (LAN) | Round-trip latency | < 16 ms | R-15.12.4 |

### TC-15.12.1.B1 HW Encoding Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Encode 1080p frame via platform HW encoder | Encoding time | < 2 ms/frame | R-15.12.1 |

### TC-15.12.2.B1 Bandwidth vs VNC

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p editor session, compare bandwidth to VNC baseline | Bandwidth reduction | >= 60% | R-15.12.2 |

### TC-15.12.5.B1 Session Resume Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Suspend session with 100 open assets, resume | Resume duration | < 5 s | R-15.12.5 |

### TC-15.12.3.B2 CRDT Convergence Three Users

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 3 users push 100 ops each concurrently, measure convergence | Convergence time | < 200 ms | R-15.12.3 |

### TC-15.12.10.B1 Chat Message Delivery

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Send message from A, measure until B receives | Delivery latency | < 100 ms | R-15.12.10 |
