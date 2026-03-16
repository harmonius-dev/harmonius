# Sessions and Replay Test Cases

Companion test cases for [sessions-replay.md](sessions-replay.md).

## Unit Tests

### TC-8.5.1.1 Auth Valid Token

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Issue token for account_id=42, validate same token | Returns account_id=42 | R-8.5.1 |
| 2 | Token issued at t=0, validated at t=TTL-1 | Validation succeeds | R-8.5.1 |

### TC-8.5.1.2 Auth Expired Token

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Token issued at t=0, validated at t=TTL+1 | Validation rejected (expired) | R-8.5.1 |

### TC-8.5.1.3 Auth MFA Challenge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | MFA challenge issued, correct code submitted | Authentication succeeds | R-8.5.1 |
| 2 | MFA challenge issued, incorrect code submitted | Authentication fails | R-8.5.1 |

### TC-8.5.9.1 Glicko-2 Rating Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player rating=1500, RD=200, vol=0.06; wins against 1400-rated opponent | Rating increases, RD decreases, volatility adjusts | R-8.5.9 |
| 2 | Player rating=1500; loses against 1600-rated opponent | Rating decreases | R-8.5.9 |

### TC-8.5.9.2 Glicko-2 Window Widening

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Queue time exceeds widen_interval (e.g., 30 s), initial window=100 rating, step=50 | Window expands to 150 rating | R-8.5.9 |
| 2 | Queue time exceeds 2x widen_interval | Window expands to 200 rating | R-8.5.9 |

### TC-8.5.3.1 Party Create Invite Accept

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | create_party(leader=A), invite(B), accept(B) | Party has 2 members; A=Leader, B=Member | R-8.5.3 |
| 2 | Invite already-in-party player | `Err(AlreadyInParty)` | R-8.5.3 |

### TC-8.5.3.2 Party Persist Across Snapshot

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Party with 4 members (Leader, Tank, Healer, DPS), serialize and deserialize | All 4 members and roles preserved exactly | R-8.5.3 |

### TC-8.5.3.3 Ready Check All Ready

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4-member party, all respond ready within timeout | `ReadyResult::AllReady` | R-8.5.3 |

### TC-8.5.3.4 Ready Check Timeout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4-member party, 3 respond ready, 1 does not respond within timeout | `ReadyResult::Timeout { not_ready: [member_4] }` | R-8.5.3 |

### TC-8.5.5.1 Reconnect Within Grace

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Preserve session state, reconnect at t=grace_window-10s | Full state restored (position, buffs, party) | R-8.5.5 |

### TC-8.5.5.2 Reconnect Expired Grace

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attempt restore at t=grace_window+1s (e.g., 121 s after disconnect) | `Err(GraceExpired)` | R-8.5.5 |

### TC-8.5.7.1 Login Queue Priority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue standard player (priority=0) then VIP (priority=1) | VIP dequeues first despite later enqueue | R-8.5.7 |

### TC-8.5.7.2 Login Queue Position Preserve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player at position 50, disconnects, reconnects within timeout | Position restored to 50 (not back of queue) | R-8.5.7 |
| 2 | Reconnect after timeout | Position lost, placed at back of queue | R-8.5.7 |

### TC-8.6.1.1 Replay Delta Compression

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record 100 ticks with 5% entity change rate | Total delta size < total full-snapshot size | R-8.6.1 |
| 2 | Compression ratio | >= 70% smaller than full-snapshot recording | R-8.6.1 |

### TC-8.6.1.2 Replay Snapshot Interval

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record with snapshot_interval=10, record 50 ticks | Full snapshots at ticks 0, 10, 20, 30, 40 (5 total) | R-8.6.1 |
| 2 | Remaining 45 ticks | Stored as deltas | R-8.6.1 |

### TC-8.6.3.1 Replay Seek to Keyframe

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Keyframes at ticks 0, 100, 200; seek to tick 100 | Index returns exact match at tick 100 | R-8.6.3 |

### TC-8.6.3.2 Replay Seek Between Keyframes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Keyframes at ticks 0, 100, 200; seek to tick 150 | Loads keyframe at tick 100, applies 50 deltas | R-8.6.3 |

### TC-8.6.5.1 Killcam Buffer Rolling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Buffer capacity=15 s, push 20 s of tick data | Oldest 5 s evicted, buffer contains most recent 15 s | R-8.6.5 |

### TC-8.6.5.2 Killcam Extract

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Death event at tick 600, extract 10 s clip (ticks 300-600), attacker=entity_42 | Clip contains ticks 300-600 with attacker entity_42 focus | R-8.6.5 |

### TC-8.6.5.3 Highlight Extract Clip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Extract sub-replay from tick 500 to tick 800 | Self-contained file with header, index, and data for tick range | R-8.6.5 |

## Integration Tests

### TC-8.5.1.I1 5000 Concurrent Logins

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5,000 concurrent authentication requests | All complete within 5 s, all return valid session tokens | R-8.5.1 |

### TC-8.5.6.I1 Cross-Play Matchmaking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Queue 1 PC, 1 PS, 1 Xbox player with similar ratings | All 3 matched to same instance | R-8.5.6 |

### TC-8.5.6.I2 Cross-Play Opt Out

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | PC player opts out of cross-play, queues with PS players | PC player matches only with other PC players | R-8.5.6 |

### TC-8.5.2.I1 Matchmaking 20K Players

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue 20,000 players with varied ratings | All matched within 120 s, skill variance per match under configured threshold | R-8.5.2 |

### TC-8.5.5.I1 Reconnect Full State

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect, play 30 s, simulate 10 s network outage, reconnect | Position, buffs, party membership all restored | R-8.5.5 |

### TC-8.5.8.I1 Headless 64 Players

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Launch headless Docker container, connect 64 players, run 60 s | Tick rate maintained at target, health endpoint returns healthy | R-8.5.8 |

### TC-8.5.8.I2 Headless Memory Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 64 players, 30 tps, 10 min sustained | RSS under 512 MB | R-8.5.8 |

### TC-8.5.4.I1 Rolling Restart Zero Disconnects

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Initiate rolling restart across server cluster with active players | All players drained and migrated, zero involuntary disconnects | R-8.5.4 |

### TC-8.6.2.I1 Replay Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record 60 s session, play back twice | Frame checksums match at 10 evenly-spaced sample points | R-8.6.2 |

### TC-8.6.3.I1 Replay Seek 2 Hour File

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2-hour replay file, seek to midpoint (tick at 1 hour) | Seek completes within 1 s | R-8.6.3 |

### TC-8.6.4.I1 Spectator 1000 Viewers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect 1,000 spectators via relay to active match | All receive stream with configured delay, no gameplay RPCs sent to spectators | R-8.6.4 |

### TC-8.6.4.I2 Spectator Delay Prevents Ghosting

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Configured delay=10 s, verify spectator data arrival time | Data arrives no earlier than 10 s after real-time event | R-8.6.4 |

### TC-8.6.5.I1 Killcam Delivery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger player death event in match | Victim receives kill cam clip within 2 s | R-8.6.5 |

### TC-8.6.2.I2 Cross Platform Replay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record replay on PC, play back on mobile | Visual hash matches at sample points | R-8.6.2 |

## Benchmarks

### TC-8.5.1.B1 Auth Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Concurrent login authentication | Throughput | 5,000 logins / 5 s | R-8.5.1 |

### TC-8.5.9.B1 Matchmaking Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Peak load matchmaking | p95 match time | < 60 s | R-8.5.9 |

### TC-8.5.5.B1 Reconnect Restore Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full state restore after reconnection | Time to playable | < 3 s | R-8.5.5 |

### TC-8.6.1.B1 Replay Recording Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Recording active during normal gameplay | Server CPU overhead | < 1% | R-8.6.1 |

### TC-8.6.3.B1 Replay Seek Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Seek to arbitrary point in 2-hour replay | Seek time | < 1 s | R-8.6.3 |

### TC-8.6.5.B1 Kill Cam Delivery Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | From death event to client receiving clip | Delivery time | < 2 s | R-8.6.5 |

### TC-8.6.1.B2 Replay File Size

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 30-minute session recording with delta compression | Size vs full-snapshot | 70% smaller | R-8.6.1 |

### TC-8.5.8.B1 Headless Server RSS

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 64 players, 30 tps, sustained | Resident set size | < 512 MB | R-8.5.8 |
