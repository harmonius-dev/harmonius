# Anti-Cheat and Security Test Cases

Companion test cases for [anti-cheat.md](anti-cheat.md).

## Unit Tests

### TC-8.8.1.1 Movement Within Bounds

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — client_pos=(10.0, 0, 10.0), server_pos=(10.0, 0, 10.0), dt=0.016, rtt=0.05,
   max_speed=10.0, is_mobile=false
   - **Expected:** `Ok(())`
2. **#2** — client_pos=(10.15, 0, 10.0), server_pos=(10.0, 0, 10.0), dt=0.016, rtt=0.05,
   max_speed=10.0, is_mobile=false
   - **Expected:** `Ok(())` (within max_distance)

### TC-8.8.1.2 Movement Speed Hack Detection

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — client_pos=(50.0, 0, 0), server_pos=(10.0, 0, 0), dt=0.016, rtt=0.05, max_speed=10.0,
   is_mobile=false
   - **Expected:** `Err((SpeedHack, severity))` where severity > 0.0
2. **#2** — client_pos=(100.0, 0, 0), server_pos=(10.0, 0, 0), dt=0.016, rtt=0.05, max_speed=10.0,
   is_mobile=false
   - **Expected:** `Err((SpeedHack, severity))` where severity > case 1

### TC-8.8.1.3 Movement Teleport Detection

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — client_pos=(500.0, 0, 500.0), server_pos=(10.0, 0, 10.0), dt=0.016, rtt=0.05,
   max_speed=10.0, is_mobile=false
   - **Expected:** `Err((Teleport, severity))` where severity > SpeedHack severity
2. **#2** — Position delta exceeds max_delta_per_tick by 10x
   - **Expected:** `Err((Teleport, high_severity))`

### TC-8.8.1.4 Movement RTT Tolerance

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — client_pos=(10.5, 0, 10.0), server_pos=(10.0, 0, 10.0), dt=0.016, rtt=0.150,
   max_speed=10.0, is_mobile=false
   - **Expected:** `Ok(())` (RTT tolerance widens bounds)
2. **#2** — max_distance(0.016, 10.0, 0.150, false)
   - **Expected:** `10.0 * 0.016 * (1.0 + rtt_tolerance * 0.150)`

### TC-8.8.1.5 Movement Mobile Tolerance

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — client_pos=(10.5, 0, 10.0), server_pos=(10.0, 0, 10.0), dt=0.016, rtt=0.100,
   max_speed=10.0, is_mobile=true
   - **Expected:** `Ok(())` (mobile multiplier widens bounds)
2. **#2** — max_distance(dt, speed, rtt, true)
   - **Expected:** max_distance(dt, speed, rtt, false) * mobile_tolerance_multiplier

### TC-8.8.1.6 Damage Within Bounds

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — reported_damage=100.0, expected_damage=100.0, weapon_stats=(base=50, range=(50,100),
   crit=2.0), multipliers=[1.0], ticks_since=10
   - **Expected:** `Ok(())`
2. **#2** — reported_damage=105.0, expected_damage=100.0, tolerance_multiplier=1.1
   - **Expected:** `Ok(())` (within tolerance)

### TC-8.8.1.7 Damage Manipulation Detection

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — reported_damage=500.0, expected_damage=100.0, tolerance_multiplier=1.1
   - **Expected:** `Err((DamageManipulation, severity))`
2. **#2** — reported_damage=200.0, expected_damage=100.0, tolerance_multiplier=1.1
   - **Expected:** `Err((DamageManipulation, severity))`

### TC-8.8.1.8 Cooldown Circumvention Detection

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — ticks_since_last_damage=1, min_damage_interval=10
   - **Expected:** `Err((CooldownCircumvention, severity))`
2. **#2** — ticks_since_last_damage=10, min_damage_interval=10
   - **Expected:** `Ok(())`

### TC-8.8.4.1 Craft Without Ingredients

| # | Requirement |
|---|-------------|
| 1 | R-8.8.4     |
| 2 | R-8.8.4     |

1. **#1** — player has empty inventory, recipe requires 5 iron ore
   - **Expected:** `Err((InventoryExploit, severity))`
2. **#2** — player has 3 iron ore, recipe requires 5
   - **Expected:** `Err((InventoryExploit, severity))`

### TC-8.8.4.2 Double Spend Prevention

| # | Requirement |
|---|-------------|
| 1 | R-8.8.4     |
| 2 | R-8.8.4     |

1. **#1** — Trade A: account_1 sends 1000 gold to account_2; Trade B: account_1 sends 1000 gold to
   account_3 (concurrent, balance=1000)
   - **Expected:** One returns `Ok(())`, one returns `Err((DoubleSpend, severity))`
2. **#2** — acquire(account_1) returns Some(seq_1), second acquire(account_1) returns None
   - **Expected:** Second transaction blocked until first completes

### TC-8.8.4.3 Gold Farming Detection

| # | Requirement |
|---|-------------|
| 1 | R-8.8.4     |
| 2 | R-8.8.4     |

1. **#1** — 100 identical TransactionRecord entries (same resource loop) followed by bulk transfer
   to different account
   - **Expected:** `Some((GoldFarming, severity))`
2. **#2** — 100 varied TransactionRecord entries (different types, counterparties)
   - **Expected:** `None` (no farming pattern)

### TC-8.8.4.4 High Value Rate Limit

| # | Requirement |
|---|-------------|
| 1 | R-8.8.4     |
| 2 | R-8.8.4     |

1. **#1** — Trade amount=15000, high_value_threshold=10000, high_value_delay=5
   - **Expected:** Delay of 5 s applied before execution
2. **#2** — Trade amount=5000, high_value_threshold=10000
   - **Expected:** No delay applied

### TC-8.8.2.1 Integrity Valid Client

| # | Requirement |
|---|-------------|
| 1 | R-8.8.2     |
| 2 | R-8.8.2     |

1. **#1** — Known-good hashes registered for build v1, client responds with matching hashes for all
   requested regions
   - **Expected:** `Ok(())`
2. **#2** — All hash comparisons match expected values
   - **Expected:** integrity_ok returned to game server

### TC-8.8.2.2 Integrity Tampered Client

| # | Requirement |
|---|-------------|
| 1 | R-8.8.2     |
| 2 | R-8.8.2     |

1. **#1** — Known-good hash for region 0x1000..0x2000 is `[0xAB; 32]`, client returns `[0xCD; 32]`
   - **Expected:** `Err((TamperedBinary, high_severity))`
2. **#2** — 1 of 4 regions returns mismatched hash
   - **Expected:** `Err((TamperedBinary, severity))`

### TC-8.8.2.3 Integrity Replay Attack

| # | Requirement |
|---|-------------|
| 1 | R-8.8.2     |
| 2 | R-8.8.2     |

1. **#1** — Response from challenge_id=100 replayed against challenge_id=101
   - **Expected:** `Err((TamperedBinary, severity))` (nonce mismatch)
2. **#2** — Same encrypted response submitted twice for same challenge_id
   - **Expected:** Second submission rejected

### TC-8.8.3.1 Behavioral Normal

| # | Requirement |
|---|-------------|
| 1 | R-8.8.3     |
| 2 | R-8.8.3     |

1. **#1** — 100 matches with aim_accuracy mean=0.30, std_dev=0.05; latest match aim_accuracy=0.32
   - **Expected:** `None` (Z-score < threshold)
2. **#2** — Consistent metrics across all 100 matches (low variance)
   - **Expected:** No anomaly flagged

### TC-8.8.3.2 Behavioral Sudden Jump

| # | Requirement |
|---|-------------|
| 1 | R-8.8.3     |
| 2 | R-8.8.3     |

1. **#1** — 100 matches with aim_accuracy mean=0.30, std_dev=0.03; latest match aim_accuracy=0.60
   (10-sigma)
   - **Expected:** `Some((BehavioralAnomaly, severity))`
2. **#2** — Z-score = (0.60 - 0.30) / 0.03 = 10.0, threshold=3.0
   - **Expected:** Anomaly detected, severity proportional to Z-score

### TC-8.8.3.3 Behavioral Gradual Improvement

| # | Requirement |
|---|-------------|
| 1 | R-8.8.3     |
| 2 | R-8.8.3     |

1. **#1** — 50 matches with aim_accuracy gradually increasing from 0.30 to 0.40 (0.002 per match)
   - **Expected:** `None` (baseline shifts with improvement)
2. **#2** — Running mean tracks gradual trend, Z-score stays below threshold
   - **Expected:** No false flag

### TC-8.8.3.4 Behavioral Input Segmentation

| # | Requirement |
|---|-------------|
| 1 | R-8.8.3     |
| 2 | R-8.8.3     |

1. **#1** — 50 Touch matches (mean accuracy 0.20), 50 KeyboardMouse matches (mean accuracy 0.45)
   - **Expected:** Two separate baselines stored, neither contaminates the other
2. **#2** — baseline(account, Touch).aim_accuracy.mean != baseline(account,
   KeyboardMouse).aim_accuracy.mean
   - **Expected:** Independent statistics confirmed

### TC-8.8.5.1 Rate Limit Allow

| # | Requirement |
|---|-------------|
| 1 | R-8.8.5     |
| 2 | R-8.8.5     |

1. **#1** — rpc_type=1, calls_per_second=10.0, 1 call after bucket is full
   - **Expected:** `RateLimitResult::Allow`
2. **#2** — tokens_remaining(account, rpc_type) = max_tokens - 1.0 after call
   - **Expected:** Token consumed

### TC-8.8.5.2 Rate Limit Throttle

| # | Requirement |
|---|-------------|
| 1 | R-8.8.5     |
| 2 | R-8.8.5     |

1. **#1** — rpc_type=1, calls_per_second=10.0, 20 calls in 1 second (2x rate), burst_allowance=5
   - **Expected:** `RateLimitResult::Throttle { delay_ms }` for calls 11-15
2. **#2** — delay_ms > 0
   - **Expected:** Throttle delay applied

### TC-8.8.5.3 Rate Limit Reject

| # | Requirement |
|---|-------------|
| 1 | R-8.8.5     |
| 2 | R-8.8.5     |

1. **#1** — rpc_type=1, calls_per_second=10.0, 100 calls in 1 second (10x rate), burst_allowance=5
   - **Expected:** `RateLimitResult::Reject` after burst_allowance exceeded
2. **#2** — burst_exceeded() returns true
   - **Expected:** Escalation triggered

### TC-8.8.5.4 Rate Limit Hot Reload

| # | Requirement |
|---|-------------|
| 1 | R-8.8.5     |
| 2 | R-8.8.5     |

1. **#1** — Initial calls_per_second=10.0; reload_config with calls_per_second=20.0
   - **Expected:** New limit active within 5 s
2. **#2** — 15 calls in 1 second after reload
   - **Expected:** `RateLimitResult::Allow` for all 15

### TC-8.8.1.9 Scorer Accumulation

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — Report 5 SpeedHack violations with severity=1.0 each, weight=1.0
   - **Expected:** current_score(account) = 5.0
2. **#2** — Report 3 more violations
   - **Expected:** current_score(account) = 8.0

### TC-8.8.1.10 Scorer Decay

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — current_score=10.0, decay_rate=1.0, dt=5.0
   - **Expected:** current_score = 10.0 - 1.0 * 5.0 = 5.0
2. **#2** — current_score=3.0, decay_rate=1.0, dt=5.0
   - **Expected:** current_score = max(0.0, 3.0 - 5.0) = 0.0

### TC-8.8.1.11 Escalation Warn

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — score=5.0,
   tiers=[{threshold:10, action:Warn}, {threshold:50, action:Kick}, {threshold:100, action:PermaBan}]
   - **Expected:** `Some(Warn)`
2. **#2** — score=3.0, lowest tier threshold=10.0
   - **Expected:** `None` (below all tiers)

### TC-8.8.1.12 Escalation Kick

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — score=60.0,
   tiers=[{threshold:10, Warn}, {threshold:50, Kick}, {threshold:100, PermaBan}]
   - **Expected:** `Some(Kick)`
2. **#2** — Execute Kick action
   - **Expected:** Player disconnected with reason code

### TC-8.8.1.13 Escalation Ban

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — score=150.0,
   tiers=[{threshold:10, Warn}, {threshold:50, Kick}, {threshold:100, PermaBan}]
   - **Expected:** `Some(PermaBan)`
2. **#2** — Execute PermaBan action
   - **Expected:** Auth service records permanent ban

## Integration Tests

### TC-8.8.1.I1 Speed Hack Detection Live

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |
| 2 | R-8.8.1     |

1. **#1** — Client submits movement at 5x max velocity for 10 ticks
   - **Expected:** SpeedHack violation detected, client position corrected to server state
2. **#2** — Continued abuse exceeds escalation threshold
   - **Expected:** Client kicked

### TC-8.8.1.I2 Teleport Detection Live

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |

1. **#1** — Client reports position 1000 units from server position in single tick
   - **Expected:** Teleport violation detected, position corrected

### TC-8.8.1.I3 Damage Hack Live

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |

1. **#1** — Client reports 10x expected damage from weapon_stats
   - **Expected:** Damage rejected, DamageManipulation violation reported

### TC-8.8.1.I4 No False Positive at 150ms RTT

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |

1. **#1** — Legitimate client at 150 ms RTT with normal movement and prediction error
   - **Expected:** Zero violations after 60 s of play

### TC-8.8.4.I1 Economy Concurrent Trade

| # | Requirement |
|---|-------------|
| 1 | R-8.8.4     |

1. **#1** — Player balance=1000g; two concurrent trades each spending 1000g
   - **Expected:** Exactly one succeeds, one fails with DoubleSpend

### TC-8.8.4.I2 Farming Pattern Live

| # | Requirement |
|---|-------------|
| 1 | R-8.8.4     |

1. **#1** — Simulate 100 identical loot-sell-transfer loops over 2 hours
   - **Expected:** GoldFarming pattern detected and flagged

### TC-8.8.2.I1 Integrity Full Cycle

| # | Requirement |
|---|-------------|
| 1 | R-8.8.2     |

1. **#1** — Server issues challenge, unmodified client computes hashes, server validates
   - **Expected:** integrity_ok, no violations

### TC-8.8.3.I1 Behavioral 100 Match History

| # | Requirement |
|---|-------------|
| 1 | R-8.8.3     |

1. **#1** — Record 100 matches with consistent metrics (aim_accuracy ~0.30); match 101 has
   aim_accuracy=0.90
   - **Expected:** BehavioralAnomaly flagged at match 101

### TC-8.8.5.I1 Rate Limit Sustained Abuse

| # | Requirement |
|---|-------------|
| 1 | R-8.8.5     |

1. **#1** — Send RPCs at 10x configured rate for 60 s
   - **Expected:** Throttle initially, then Reject, then escalation to Kick

### TC-8.8.5.I2 Hot Reload Live

| # | Requirement |
|---|-------------|
| 1 | R-8.8.5     |

1. **#1** — Active session, reload config doubling calls_per_second
   - **Expected:** New limits take effect within 5 s, no disconnection

### TC-8.8.1.I5 Replay Based Verification

| # | Requirement |
|---|-------------|
| 1 | R-8.8.1     |

1. **#1** — Record session with injected cheat, replay through validation pipeline
   - **Expected:** Same violations detected during replay as during live session

### TC-8.8.3.I2 Legitimate High Skill No Flag

| # | Requirement |
|---|-------------|
| 1 | R-8.8.3     |

1. **#1** — Replay 50 recorded sessions from top-1% skill players
   - **Expected:** Zero BehavioralAnomaly violations

## Benchmarks

### TC-8.8.1.B1 Movement Validation Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Movement validation for 1 player per tick | Latency | < 1 us | R-8.8.1 |
| 2 | Movement validation for 1000 players per tick | Total time | < 1 ms | R-8.8.1 |

### TC-8.8.1.B2 Damage Validation Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Damage validation per combat event | Latency | < 500 ns | R-8.8.1 |

### TC-8.8.5.B1 Rate Limit Check Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single rate limit check per RPC | Latency | < 100 ns | R-8.8.5 |

### TC-8.8.2.B1 Integrity Challenge Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Generate integrity challenge with 4 regions | Latency | < 10 us | R-8.8.2 |

### TC-8.8.2.B2 Integrity Response Validation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Validate response with 4 region hashes | Latency | < 50 us | R-8.8.2 |

### TC-8.8.3.B1 Behavioral Analysis Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Analyze single player (100-match baseline) | Latency | < 1 ms | R-8.8.3 |

### TC-8.8.1.B3 Score Decay Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Decay scores for 1000 players | Total time | < 100 us | R-8.8.1 |

### TC-8.8.5.B2 Hot Reload Application

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Apply new AntiCheatConfig to all subsystems | Time to effect | < 5 s | R-8.8.5 |
