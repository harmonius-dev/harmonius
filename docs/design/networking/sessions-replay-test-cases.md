# Sessions and Replay Test Cases

Companion test cases for [sessions-replay.md](sessions-replay.md).

## Unit Tests

### TC-8.5.1.1 Auth Valid Token

| # | Requirement |
|---|-------------|
| 1 | R-8.5.1     |
| 2 | R-8.5.1     |

1. **#1** — Issue token for account_id=42, validate same token
   - **Expected:** Returns account_id=42
2. **#2** — Token issued at t=0, validated at t=TTL-1
   - **Expected:** Validation succeeds

### TC-8.5.1.2 Auth Expired Token

| # | Requirement |
|---|-------------|
| 1 | R-8.5.1     |

1. **#1** — Token issued at t=0, validated at t=TTL+1
   - **Expected:** Validation rejected (expired)

### TC-8.5.1.3 Auth MFA Challenge

| # | Requirement |
|---|-------------|
| 1 | R-8.5.1     |
| 2 | R-8.5.1     |

1. **#1** — MFA challenge issued, correct code submitted
   - **Expected:** Authentication succeeds
2. **#2** — MFA challenge issued, incorrect code submitted
   - **Expected:** Authentication fails

### TC-8.5.9.1 Glicko-2 Rating Update

| # | Requirement |
|---|-------------|
| 1 | R-8.5.9     |
| 2 | R-8.5.9     |

1. **#1** — Player rating=1500, RD=200, vol=0.06; wins against 1400-rated opponent
   - **Expected:** Rating increases, RD decreases, volatility adjusts
2. **#2** — Player rating=1500; loses against 1600-rated opponent
   - **Expected:** Rating decreases

### TC-8.5.9.2 Glicko-2 Window Widening

| # | Requirement |
|---|-------------|
| 1 | R-8.5.9     |
| 2 | R-8.5.9     |

1. **#1** — Queue time exceeds widen_interval (e.g., 30 s), initial window=100 rating, step=50
   - **Expected:** Window expands to 150 rating
2. **#2** — Queue time exceeds 2x widen_interval
   - **Expected:** Window expands to 200 rating

### TC-8.5.3.1 Party Create Invite Accept

| # | Requirement |
|---|-------------|
| 1 | R-8.5.3     |
| 2 | R-8.5.3     |

1. **#1** — create_party(leader=A), invite(B), accept(B)
   - **Expected:** Party has 2 members; A=Leader, B=Member
2. **#2** — Invite already-in-party player
   - **Expected:** `Err(AlreadyInParty)`

### TC-8.5.3.2 Party Persist Across Snapshot

| # | Requirement |
|---|-------------|
| 1 | R-8.5.3     |

1. **#1** — Party with 4 members (Leader, Tank, Healer, DPS), serialize and deserialize
   - **Expected:** All 4 members and roles preserved exactly

### TC-8.5.3.3 Ready Check All Ready

| # | Requirement |
|---|-------------|
| 1 | R-8.5.3     |

1. **#1** — 4-member party, all respond ready within timeout
   - **Expected:** `ReadyResult::AllReady`

### TC-8.5.3.4 Ready Check Timeout

| # | Requirement |
|---|-------------|
| 1 | R-8.5.3     |

1. **#1** — 4-member party, 3 respond ready, 1 does not respond within timeout
   - **Expected:** `ReadyResult::Timeout { not_ready: [member_4] }`

### TC-8.5.5.1 Reconnect Within Grace

| # | Requirement |
|---|-------------|
| 1 | R-8.5.5     |

1. **#1** — Preserve session state, reconnect at t=grace_window-10s
   - **Expected:** Full state restored (position, buffs, party)

### TC-8.5.5.2 Reconnect Expired Grace

| # | Requirement |
|---|-------------|
| 1 | R-8.5.5     |

1. **#1** — Attempt restore at t=grace_window+1s (e.g., 121 s after disconnect)
   - **Expected:** `Err(GraceExpired)`

### TC-8.5.7.1 Login Queue Priority

| # | Requirement |
|---|-------------|
| 1 | R-8.5.7     |

1. **#1** — Enqueue standard player (priority=0) then VIP (priority=1)
   - **Expected:** VIP dequeues first despite later enqueue

### TC-8.5.7.2 Login Queue Position Preserve

| # | Requirement |
|---|-------------|
| 1 | R-8.5.7     |
| 2 | R-8.5.7     |

1. **#1** — Player at position 50, disconnects, reconnects within timeout
   - **Expected:** Position restored to 50 (not back of queue)
2. **#2** — Reconnect after timeout
   - **Expected:** Position lost, placed at back of queue

### TC-8.6.1.1 Replay Delta Compression

| # | Requirement |
|---|-------------|
| 1 | R-8.6.1     |
| 2 | R-8.6.1     |

1. **#1** — Record 100 ticks with 5% entity change rate
   - **Expected:** Total delta size < total full-snapshot size
2. **#2** — Compression ratio
   - **Expected:** >= 70% smaller than full-snapshot recording

### TC-8.6.1.2 Replay Snapshot Interval

| # | Requirement |
|---|-------------|
| 1 | R-8.6.1     |
| 2 | R-8.6.1     |

1. **#1** — Record with snapshot_interval=10, record 50 ticks
   - **Expected:** Full snapshots at ticks 0, 10, 20, 30, 40 (5 total)
2. **#2** — Remaining 45 ticks
   - **Expected:** Stored as deltas

### TC-8.6.3.1 Replay Seek to Keyframe

| # | Requirement |
|---|-------------|
| 1 | R-8.6.3     |

1. **#1** — Keyframes at ticks 0, 100, 200; seek to tick 100
   - **Expected:** Index returns exact match at tick 100

### TC-8.6.3.2 Replay Seek Between Keyframes

| # | Requirement |
|---|-------------|
| 1 | R-8.6.3     |

1. **#1** — Keyframes at ticks 0, 100, 200; seek to tick 150
   - **Expected:** Loads keyframe at tick 100, applies 50 deltas

### TC-8.6.5.1 Killcam Buffer Rolling

| # | Requirement |
|---|-------------|
| 1 | R-8.6.5     |

1. **#1** — Buffer capacity=15 s, push 20 s of tick data
   - **Expected:** Oldest 5 s evicted, buffer contains most recent 15 s

### TC-8.6.5.2 Killcam Extract

| # | Requirement |
|---|-------------|
| 1 | R-8.6.5     |

1. **#1** — Death event at tick 600, extract 10 s clip (ticks 300-600), attacker=entity_42
   - **Expected:** Clip contains ticks 300-600 with attacker entity_42 focus

### TC-8.6.5.3 Highlight Extract Clip

| # | Requirement |
|---|-------------|
| 1 | R-8.6.5     |

1. **#1** — Extract sub-replay from tick 500 to tick 800
   - **Expected:** Self-contained file with header, index, and data for tick range

## Integration Tests

### TC-8.5.1.I1 5000 Concurrent Logins

| # | Requirement |
|---|-------------|
| 1 | R-8.5.1     |

1. **#1** — 5,000 concurrent authentication requests
   - **Expected:** All complete within 5 s, all return valid session tokens

### TC-8.5.6.I1 Cross-Play Matchmaking

| # | Requirement |
|---|-------------|
| 1 | R-8.5.6     |

1. **#1** — Queue 1 PC, 1 PS, 1 Xbox player with similar ratings
   - **Expected:** All 3 matched to same instance

### TC-8.5.6.I2 Cross-Play Opt Out

| # | Requirement |
|---|-------------|
| 1 | R-8.5.6     |

1. **#1** — PC player opts out of cross-play, queues with PS players
   - **Expected:** PC player matches only with other PC players

### TC-8.5.2.I1 Matchmaking 20K Players

| # | Requirement |
|---|-------------|
| 1 | R-8.5.2     |

1. **#1** — Enqueue 20,000 players with varied ratings
   - **Expected:** All matched within 120 s, skill variance per match under configured threshold

### TC-8.5.5.I1 Reconnect Full State

| # | Requirement |
|---|-------------|
| 1 | R-8.5.5     |

1. **#1** — Connect, play 30 s, simulate 10 s network outage, reconnect
   - **Expected:** Position, buffs, party membership all restored

### TC-8.5.8.I1 Headless 64 Players

| # | Requirement |
|---|-------------|
| 1 | R-8.5.8     |

1. **#1** — Launch headless Docker container, connect 64 players, run 60 s
   - **Expected:** Tick rate maintained at target, health endpoint returns healthy

### TC-8.5.8.I2 Headless Memory Budget

| # | Requirement |
|---|-------------|
| 1 | R-8.5.8     |

1. **#1** — 64 players, 30 tps, 10 min sustained
   - **Expected:** RSS under 512 MB

### TC-8.5.4.I1 Rolling Restart Zero Disconnects

| # | Requirement |
|---|-------------|
| 1 | R-8.5.4     |

1. **#1** — Initiate rolling restart across server cluster with active players
   - **Expected:** All players drained and migrated, zero involuntary disconnects

### TC-8.6.2.I1 Replay Determinism

| # | Requirement |
|---|-------------|
| 1 | R-8.6.2     |

1. **#1** — Record 60 s session, play back twice
   - **Expected:** Frame checksums match at 10 evenly-spaced sample points

### TC-8.6.3.I1 Replay Seek 2 Hour File

| # | Requirement |
|---|-------------|
| 1 | R-8.6.3     |

1. **#1** — 2-hour replay file, seek to midpoint (tick at 1 hour)
   - **Expected:** Seek completes within 1 s

### TC-8.6.4.I1 Spectator 1000 Viewers

| # | Requirement |
|---|-------------|
| 1 | R-8.6.4     |

1. **#1** — Connect 1,000 spectators via relay to active match
   - **Expected:** All receive stream with configured delay, no gameplay RPCs sent to spectators

### TC-8.6.4.I2 Spectator Delay Prevents Ghosting

| # | Requirement |
|---|-------------|
| 1 | R-8.6.4     |

1. **#1** — Configured delay=10 s, verify spectator data arrival time
   - **Expected:** Data arrives no earlier than 10 s after real-time event

### TC-8.6.5.I1 Killcam Delivery

| # | Requirement |
|---|-------------|
| 1 | R-8.6.5     |

1. **#1** — Trigger player death event in match
   - **Expected:** Victim receives kill cam clip within 2 s

### TC-8.6.2.I2 Cross Platform Replay

| # | Requirement |
|---|-------------|
| 1 | R-8.6.2     |

1. **#1** — Record replay on PC, play back on mobile
   - **Expected:** Visual hash matches at sample points

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

| # | Metric                | Target      | Requirement |
|---|-----------------------|-------------|-------------|
| 1 | Size vs full-snapshot | 70% smaller | R-8.6.1     |

1. **1** — 30-minute session recording with delta compression

### TC-8.5.8.B1 Headless Server RSS

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 64 players, 30 tps, sustained | Resident set size | < 512 MB | R-8.5.8 |
