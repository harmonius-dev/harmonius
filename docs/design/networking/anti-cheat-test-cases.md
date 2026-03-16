# Anti-Cheat and Security Test Cases

Companion test cases for [anti-cheat.md](anti-cheat.md).

## Unit Tests

### TC-8.8.1.1 Movement Within Bounds

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | client_pos=(10.0, 0, 10.0), server_pos=(10.0, 0, 10.0), dt=0.016, rtt=0.05, max_speed=10.0, is_mobile=false | `Ok(())` | R-8.8.1 |
| 2 | client_pos=(10.15, 0, 10.0), server_pos=(10.0, 0, 10.0), dt=0.016, rtt=0.05, max_speed=10.0, is_mobile=false | `Ok(())` (within max_distance) | R-8.8.1 |

### TC-8.8.1.2 Movement Speed Hack Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | client_pos=(50.0, 0, 0), server_pos=(10.0, 0, 0), dt=0.016, rtt=0.05, max_speed=10.0, is_mobile=false | `Err((SpeedHack, severity))` where severity > 0.0 | R-8.8.1 |
| 2 | client_pos=(100.0, 0, 0), server_pos=(10.0, 0, 0), dt=0.016, rtt=0.05, max_speed=10.0, is_mobile=false | `Err((SpeedHack, severity))` where severity > case 1 | R-8.8.1 |

### TC-8.8.1.3 Movement Teleport Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | client_pos=(500.0, 0, 500.0), server_pos=(10.0, 0, 10.0), dt=0.016, rtt=0.05, max_speed=10.0, is_mobile=false | `Err((Teleport, severity))` where severity > SpeedHack severity | R-8.8.1 |
| 2 | Position delta exceeds max_delta_per_tick by 10x | `Err((Teleport, high_severity))` | R-8.8.1 |

### TC-8.8.1.4 Movement RTT Tolerance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | client_pos=(10.5, 0, 10.0), server_pos=(10.0, 0, 10.0), dt=0.016, rtt=0.150, max_speed=10.0, is_mobile=false | `Ok(())` (RTT tolerance widens bounds) | R-8.8.1 |
| 2 | max_distance(0.016, 10.0, 0.150, false) | `10.0 * 0.016 * (1.0 + rtt_tolerance * 0.150)` | R-8.8.1 |

### TC-8.8.1.5 Movement Mobile Tolerance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | client_pos=(10.5, 0, 10.0), server_pos=(10.0, 0, 10.0), dt=0.016, rtt=0.100, max_speed=10.0, is_mobile=true | `Ok(())` (mobile multiplier widens bounds) | R-8.8.1 |
| 2 | max_distance(dt, speed, rtt, true) | max_distance(dt, speed, rtt, false) * mobile_tolerance_multiplier | R-8.8.1 |

### TC-8.8.1.6 Damage Within Bounds

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | reported_damage=100.0, expected_damage=100.0, weapon_stats=(base=50, range=(50,100), crit=2.0), multipliers=[1.0], ticks_since=10 | `Ok(())` | R-8.8.1 |
| 2 | reported_damage=105.0, expected_damage=100.0, tolerance_multiplier=1.1 | `Ok(())` (within tolerance) | R-8.8.1 |

### TC-8.8.1.7 Damage Manipulation Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | reported_damage=500.0, expected_damage=100.0, tolerance_multiplier=1.1 | `Err((DamageManipulation, severity))` | R-8.8.1 |
| 2 | reported_damage=200.0, expected_damage=100.0, tolerance_multiplier=1.1 | `Err((DamageManipulation, severity))` | R-8.8.1 |

### TC-8.8.1.8 Cooldown Circumvention Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | ticks_since_last_damage=1, min_damage_interval=10 | `Err((CooldownCircumvention, severity))` | R-8.8.1 |
| 2 | ticks_since_last_damage=10, min_damage_interval=10 | `Ok(())` | R-8.8.1 |

### TC-8.8.4.1 Craft Without Ingredients

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | player has empty inventory, recipe requires 5 iron ore | `Err((InventoryExploit, severity))` | R-8.8.4 |
| 2 | player has 3 iron ore, recipe requires 5 | `Err((InventoryExploit, severity))` | R-8.8.4 |

### TC-8.8.4.2 Double Spend Prevention

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trade A: account_1 sends 1000 gold to account_2; Trade B: account_1 sends 1000 gold to account_3 (concurrent, balance=1000) | One returns `Ok(())`, one returns `Err((DoubleSpend, severity))` | R-8.8.4 |
| 2 | acquire(account_1) returns Some(seq_1), second acquire(account_1) returns None | Second transaction blocked until first completes | R-8.8.4 |

### TC-8.8.4.3 Gold Farming Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 identical TransactionRecord entries (same resource loop) followed by bulk transfer to different account | `Some((GoldFarming, severity))` | R-8.8.4 |
| 2 | 100 varied TransactionRecord entries (different types, counterparties) | `None` (no farming pattern) | R-8.8.4 |

### TC-8.8.4.4 High Value Rate Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trade amount=15000, high_value_threshold=10000, high_value_delay=5 | Delay of 5 s applied before execution | R-8.8.4 |
| 2 | Trade amount=5000, high_value_threshold=10000 | No delay applied | R-8.8.4 |

### TC-8.8.2.1 Integrity Valid Client

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Known-good hashes registered for build v1, client responds with matching hashes for all requested regions | `Ok(())` | R-8.8.2 |
| 2 | All hash comparisons match expected values | integrity_ok returned to game server | R-8.8.2 |

### TC-8.8.2.2 Integrity Tampered Client

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Known-good hash for region 0x1000..0x2000 is `[0xAB; 32]`, client returns `[0xCD; 32]` | `Err((TamperedBinary, high_severity))` | R-8.8.2 |
| 2 | 1 of 4 regions returns mismatched hash | `Err((TamperedBinary, severity))` | R-8.8.2 |

### TC-8.8.2.3 Integrity Replay Attack

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Response from challenge_id=100 replayed against challenge_id=101 | `Err((TamperedBinary, severity))` (nonce mismatch) | R-8.8.2 |
| 2 | Same encrypted response submitted twice for same challenge_id | Second submission rejected | R-8.8.2 |

### TC-8.8.3.1 Behavioral Normal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 matches with aim_accuracy mean=0.30, std_dev=0.05; latest match aim_accuracy=0.32 | `None` (Z-score < threshold) | R-8.8.3 |
| 2 | Consistent metrics across all 100 matches (low variance) | No anomaly flagged | R-8.8.3 |

### TC-8.8.3.2 Behavioral Sudden Jump

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 matches with aim_accuracy mean=0.30, std_dev=0.03; latest match aim_accuracy=0.60 (10-sigma) | `Some((BehavioralAnomaly, severity))` | R-8.8.3 |
| 2 | Z-score = (0.60 - 0.30) / 0.03 = 10.0, threshold=3.0 | Anomaly detected, severity proportional to Z-score | R-8.8.3 |

### TC-8.8.3.3 Behavioral Gradual Improvement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50 matches with aim_accuracy gradually increasing from 0.30 to 0.40 (0.002 per match) | `None` (baseline shifts with improvement) | R-8.8.3 |
| 2 | Running mean tracks gradual trend, Z-score stays below threshold | No false flag | R-8.8.3 |

### TC-8.8.3.4 Behavioral Input Segmentation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50 Touch matches (mean accuracy 0.20), 50 KeyboardMouse matches (mean accuracy 0.45) | Two separate baselines stored, neither contaminates the other | R-8.8.3 |
| 2 | baseline(account, Touch).aim_accuracy.mean != baseline(account, KeyboardMouse).aim_accuracy.mean | Independent statistics confirmed | R-8.8.3 |

### TC-8.8.5.1 Rate Limit Allow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | rpc_type=1, calls_per_second=10.0, 1 call after bucket is full | `RateLimitResult::Allow` | R-8.8.5 |
| 2 | tokens_remaining(account, rpc_type) = max_tokens - 1.0 after call | Token consumed | R-8.8.5 |

### TC-8.8.5.2 Rate Limit Throttle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | rpc_type=1, calls_per_second=10.0, 20 calls in 1 second (2x rate), burst_allowance=5 | `RateLimitResult::Throttle { delay_ms }` for calls 11-15 | R-8.8.5 |
| 2 | delay_ms > 0 | Throttle delay applied | R-8.8.5 |

### TC-8.8.5.3 Rate Limit Reject

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | rpc_type=1, calls_per_second=10.0, 100 calls in 1 second (10x rate), burst_allowance=5 | `RateLimitResult::Reject` after burst_allowance exceeded | R-8.8.5 |
| 2 | burst_exceeded() returns true | Escalation triggered | R-8.8.5 |

### TC-8.8.5.4 Rate Limit Hot Reload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Initial calls_per_second=10.0; reload_config with calls_per_second=20.0 | New limit active within 5 s | R-8.8.5 |
| 2 | 15 calls in 1 second after reload | `RateLimitResult::Allow` for all 15 | R-8.8.5 |

### TC-8.8.1.9 Scorer Accumulation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Report 5 SpeedHack violations with severity=1.0 each, weight=1.0 | current_score(account) = 5.0 | R-8.8.1 |
| 2 | Report 3 more violations | current_score(account) = 8.0 | R-8.8.1 |

### TC-8.8.1.10 Scorer Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | current_score=10.0, decay_rate=1.0, dt=5.0 | current_score = 10.0 - 1.0 * 5.0 = 5.0 | R-8.8.1 |
| 2 | current_score=3.0, decay_rate=1.0, dt=5.0 | current_score = max(0.0, 3.0 - 5.0) = 0.0 | R-8.8.1 |

### TC-8.8.1.11 Escalation Warn

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | score=5.0, tiers=[{threshold:10, action:Warn}, {threshold:50, action:Kick}, {threshold:100, action:PermaBan}] | `Some(Warn)` | R-8.8.1 |
| 2 | score=3.0, lowest tier threshold=10.0 | `None` (below all tiers) | R-8.8.1 |

### TC-8.8.1.12 Escalation Kick

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | score=60.0, tiers=[{threshold:10, Warn}, {threshold:50, Kick}, {threshold:100, PermaBan}] | `Some(Kick)` | R-8.8.1 |
| 2 | Execute Kick action | Player disconnected with reason code | R-8.8.1 |

### TC-8.8.1.13 Escalation Ban

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | score=150.0, tiers=[{threshold:10, Warn}, {threshold:50, Kick}, {threshold:100, PermaBan}] | `Some(PermaBan)` | R-8.8.1 |
| 2 | Execute PermaBan action | Auth service records permanent ban | R-8.8.1 |

## Integration Tests

### TC-8.8.1.I1 Speed Hack Detection Live

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client submits movement at 5x max velocity for 10 ticks | SpeedHack violation detected, client position corrected to server state | R-8.8.1 |
| 2 | Continued abuse exceeds escalation threshold | Client kicked | R-8.8.1 |

### TC-8.8.1.I2 Teleport Detection Live

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client reports position 1000 units from server position in single tick | Teleport violation detected, position corrected | R-8.8.1 |

### TC-8.8.1.I3 Damage Hack Live

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client reports 10x expected damage from weapon_stats | Damage rejected, DamageManipulation violation reported | R-8.8.1 |

### TC-8.8.1.I4 No False Positive at 150ms RTT

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Legitimate client at 150 ms RTT with normal movement and prediction error | Zero violations after 60 s of play | R-8.8.1 |

### TC-8.8.4.I1 Economy Concurrent Trade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player balance=1000g; two concurrent trades each spending 1000g | Exactly one succeeds, one fails with DoubleSpend | R-8.8.4 |

### TC-8.8.4.I2 Farming Pattern Live

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate 100 identical loot-sell-transfer loops over 2 hours | GoldFarming pattern detected and flagged | R-8.8.4 |

### TC-8.8.2.I1 Integrity Full Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Server issues challenge, unmodified client computes hashes, server validates | integrity_ok, no violations | R-8.8.2 |

### TC-8.8.3.I1 Behavioral 100 Match History

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record 100 matches with consistent metrics (aim_accuracy ~0.30); match 101 has aim_accuracy=0.90 | BehavioralAnomaly flagged at match 101 | R-8.8.3 |

### TC-8.8.5.I1 Rate Limit Sustained Abuse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send RPCs at 10x configured rate for 60 s | Throttle initially, then Reject, then escalation to Kick | R-8.8.5 |

### TC-8.8.5.I2 Hot Reload Live

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Active session, reload config doubling calls_per_second | New limits take effect within 5 s, no disconnection | R-8.8.5 |

### TC-8.8.1.I5 Replay Based Verification

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record session with injected cheat, replay through validation pipeline | Same violations detected during replay as during live session | R-8.8.1 |

### TC-8.8.3.I2 Legitimate High Skill No Flag

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Replay 50 recorded sessions from top-1% skill players | Zero BehavioralAnomaly violations | R-8.8.3 |

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
