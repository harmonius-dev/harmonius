# Remote Collaboration Test Cases

Companion test cases for [collaboration.md](collaboration.md).

## Unit Tests

### TC-15.12.8.1 Tree CRDT Add Remove

| # | Requirement |
|---|-------------|
| 1 | R-15.12.8   |
| 2 | R-15.12.8   |
| 3 | R-15.12.8   |

1. **#1** — `add_node(root, data)`
   - **Expected:** Node present in tree, parent is root
2. **#2** — `remove_node(child_id)`
   - **Expected:** Node absent from tree, children reparented or removed
3. **#3** — Add 100 nodes, remove 50
   - **Expected:** Tree contains exactly 50 nodes plus root

### TC-15.12.8.2 Tree CRDT Concurrent Reparent

| # | Requirement |
|---|-------------|
| 1 | R-15.12.8   |
| 2 | R-15.12.8   |

1. **#1** — User A: `reparent(N, P1)`, User B: `reparent(N, P2)`, merge
   - **Expected:** Node N has one parent (LWW by HLC), no cycles
2. **#2** — Both reparent different nodes
   - **Expected:** Both reparents applied, tree consistent

### TC-15.12.8.3 OpLog Concurrent Connect

| # | Requirement |
|---|-------------|
| 1 | R-15.12.8   |

1. **#1** — User A: `connect(src1, dst1)`, User B: `connect(src2, dst2)`, merge
   - **Expected:** Both connections present in graph

### TC-15.12.8.4 Map CRDT Concurrent Set

| # | Requirement |
|---|-------------|
| 1 | R-15.12.8   |

1. **#1** — User A: `set("hp", 100)` at HLC t1, User B: `set("hp", 200)` at HLC t2 > t1, merge
   - **Expected:** `get("hp") == 200` (higher HLC wins)

### TC-15.12.8.5 LWW Register Ordering

| # | Requirement |
|---|-------------|
| 1 | R-15.12.8   |
| 2 | R-15.12.8   |

1. **#1** — `write(data_A, hlc_1)`, `apply_remote(data_B, hlc_2)` where hlc_2 > hlc_1
   - **Expected:** `read() == data_B`
2. **#2** — `write(data_A, hlc_2)`, `apply_remote(data_B, hlc_1)` where hlc_2 > hlc_1
   - **Expected:** `read() == data_A` (local wins, higher HLC)

### TC-15.12.3.1 HLC Monotonic

| # | Requirement |
|---|-------------|
| 1 | R-15.12.3   |
| 2 | R-15.12.3   |

1. **#1** — Call `now()` 1000 times in sequence
   - **Expected:** Each timestamp strictly greater than the previous
2. **#2** — `update(remote_ts)` where remote is ahead
   - **Expected:** Local clock advances past remote timestamp

### TC-15.12.3.2 Per-User Undo

| # | Requirement |
|---|-------------|
| 1 | R-15.12.3   |
| 2 | R-15.12.3   |

1. **#1** — User A records 3 ops, user B records 2 ops, user A undoes 1
   - **Expected:** User A has 2 ops, user B still has 2 ops
2. **#2** — User A undoes all, user B state unchanged
   - **Expected:** User B ops remain applied

### TC-15.12.3.3 Presence Cursor Update

| # | Requirement |
|---|-------------|
| 1 | R-15.12.3   |

1. **#1** — `set_cursor_position([1.0, 2.0, 3.0])`
   - **Expected:** `remote_cursors()` on other client shows position `[1.0, 2.0, 3.0]`

### TC-15.12.9.1 Role Viewer No Edit

| # | Requirement |
|---|-------------|
| 1 | R-15.12.9   |
| 2 | R-15.12.9   |

1. **#1** — User with `Role::Viewer` calls `push_ops([edit_op])`
   - **Expected:** Returns `Err(PermissionDenied { action: Edit })`
2. **#2** — User with `Role::Editor` calls `push_ops([edit_op])`
   - **Expected:** Returns `Ok(())`

### TC-15.12.9.2 Role Admin Set Role

| # | Requirement |
|---|-------------|
| 1 | R-15.12.9   |
| 2 | R-15.12.9   |

1. **#1** — Admin calls `set_role(user_B, Editor)`
   - **Expected:** User B role changed to Editor
2. **#2** — Editor calls `set_role(user_B, Viewer)`
   - **Expected:** Returns `Err(PermissionDenied)`

### TC-15.12.6.1 Bandwidth Tier Selection

| # | Requirement |
|---|-------------|
| 1 | R-15.12.6   |
| 2 | R-15.12.6   |
| 3 | R-15.12.6   |

1. **#1** — Measured bandwidth 150 Mbps
   - **Expected:** `current_tier() == QualityTier::High`
2. **#2** — Measured bandwidth 50 Mbps
   - **Expected:** `current_tier() == QualityTier::Medium`
3. **#3** — Measured bandwidth 5 Mbps
   - **Expected:** `current_tier() == QualityTier::Low`

### TC-15.12.11.1 Workgroup Isolation

| # | Requirement |
|---|-------------|
| 1 | R-15.12.11  |
| 2 | R-15.12.11  |

1. **#1** — Group A adds entity, group B queries entities
   - **Expected:** Group B does not see group A's entity
2. **#2** — Group A shares workspace
   - **Expected:** Entity now visible to all groups

## Integration Tests

### TC-15.12.3.I1 Three User Convergence

| # | Requirement |
|---|-------------|
| 1 | R-15.12.3   |

1. **#1** — Users A, B, C each push 10 concurrent ops
   - **Expected:** All three local states are identical after sync

### TC-15.12.5.I1 Session Suspend Resume

| # | Requirement |
|---|-------------|
| 1 | R-15.12.5   |

1. **#1** — Suspend session on client A, resume on client B
   - **Expected:** Camera position, selection, and undo history match pre-suspend state

### TC-15.12.3.I2 P2P LAN Discovery

| # | Requirement |
|---|-------------|
| 1 | R-15.12.3   |

1. **#1** — Two editors on same LAN, `connect_p2p()`
   - **Expected:** Both discover each other via mDNS, CRDT sync succeeds

### TC-15.12.2.I1 QUIC to TCP Fallback

| # | Requirement |
|---|-------------|
| 1 | R-15.12.2   |

1. **#1** — Block UDP on firewall, start remote stream
   - **Expected:** Stream falls back to TCP+TLS 1.3, video received

### TC-15.12.7.I1 100 Concurrent Sessions

| # | Requirement |
|---|-------------|
| 1 | R-15.12.7   |

1. **#1** — Create 100 sessions, each pushing 10 ops
   - **Expected:** All sessions converge, no data loss

### TC-15.12.9.I1 OAuth2 Authentication

| # | Requirement |
|---|-------------|
| 1 | R-15.12.9   |

1. **#1** — OAuth2 flow with mock provider
   - **Expected:** Session created with correct `Role` from token claims

### TC-15.12.10.I1 Chat Message Delivery

| # | Requirement |
|---|-------------|
| 1 | R-15.12.10  |

1. **#1** — User A sends "hello", user B receives
   - **Expected:** Message content matches "hello", correct author

### TC-15.12.10.I2 Chat Search

| # | Requirement |
|---|-------------|
| 1 | R-15.12.10  |

1. **#1** — Send 50 messages, search for keyword in message 25
   - **Expected:** Search returns message 25

### TC-15.12.1.I1 Encoding Overhead

| # | Requirement |
|---|-------------|
| 1 | R-15.12.1   |

1. **#1** — Encode 1920x1080 frame via HW encoder
   - **Expected:** `last_encode_us() < 2000`

### TC-15.12.14.I1 PR Review Structural Diff

| # | Requirement |
|---|-------------|
| 1 | R-15.12.14  |

1. **#1** — PR with changed scene asset, view diff in editor
   - **Expected:** Structural diff shows added/removed/modified entities

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

| # | Metric              | Target | Requirement |
|---|---------------------|--------|-------------|
| 1 | Bandwidth reduction | >= 60% | R-15.12.2   |

1. **1** — 1080p editor session, compare bandwidth to VNC baseline

### TC-15.12.5.B1 Session Resume Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Suspend session with 100 open assets, resume | Resume duration | < 5 s | R-15.12.5 |

### TC-15.12.3.B2 CRDT Convergence Three Users

| # | Metric           | Target   | Requirement |
|---|------------------|----------|-------------|
| 1 | Convergence time | < 200 ms | R-15.12.3   |

1. **1** — 3 users push 100 ops each concurrently, measure convergence

### TC-15.12.10.B1 Chat Message Delivery

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Send message from A, measure until B receives | Delivery latency | < 100 ms | R-15.12.10 |
