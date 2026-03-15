# R-8.8 — Anti-Cheat and Security Requirements

## Client Integrity

### R-8.8.1 Server-Side Cheat Detection

The engine **SHALL** perform server-authoritative validation of all client actions by independently
simulating movement, ability usage, damage, and economy transactions, comparing client-reported
state against server-computed bounds, detecting violations (speed hacks, teleportation, damage
manipulation, cooldown circumvention), scoring them, and escalating through configurable severity
tiers (warn, flag, kick, ban) with thresholds that account for legitimate network latency and
prediction error.

- **Derived from:** [F-8.8.1](../../features/networking/anti-cheat.md)
- **Rationale:** Server-authoritative validation is the primary defense against cheating; all other
  anti-cheat measures are supplementary.
- **Verification:** Integration test: submit movement inputs exceeding maximum velocity and verify
  the server detects and flags a speed hack. Submit a position discontinuity and verify
  teleportation detection. Submit damage exceeding weapon stats and verify damage manipulation
  detection. Verify violation scoring accumulates correctly and triggers the configured escalation
  tier. Verify a legitimate client with 150 ms RTT and prediction correction is not falsely flagged.

### R-8.8.2 Client Integrity Verification

The engine **SHALL** perform periodic client integrity checks by issuing encrypted memory region
challenges from the server, validating client responses against known-good hashes for the client's
build version, and escalating failures through the same severity pipeline as gameplay cheat
detection.

- **Derived from:** [F-8.8.2](../../features/networking/anti-cheat.md)
- **Rationale:** Detecting modified binaries and injected code raises the difficulty of tampering,
  complementing server-side validation as a defense-in-depth measure.
- **Verification:** Integration test: connect an unmodified client and verify integrity checks pass.
  Modify a code segment in a test client and verify the server detects the hash mismatch. Verify the
  challenge-response is encrypted and cannot be replayed from a captured packet. Verify failure
  triggers the configured escalation tier.

### R-8.8.3 Behavioral Analysis and Anomaly Detection

The engine **SHALL** perform server-side statistical analysis of player behavior patterns over time
(reaction times, aim accuracy, movement entropy, resource acquisition rates, win/loss streaks),
building per-player baselines and flagging anomalous deviations using configurable Z-score
thresholds, distinguishing improving skill from sudden capability jumps.

- **Derived from:** [F-8.8.3](../../features/networking/anti-cheat.md)
- **Rationale:** Sophisticated cheats that pass individual frame checks can be detected through
  statistical analysis of behavior patterns over time.
- **Verification:** Integration test: simulate a player with consistent aim accuracy for 100
  matches, then inject a sudden 3-standard-deviation accuracy jump. Verify the anomaly detection
  system flags the deviation. Simulate gradual accuracy improvement over 50 matches and verify no
  false flag. Verify analysis runs on aggregated telemetry data without impacting real-time game
  simulation performance.

### R-8.8.4 Economy Exploit Prevention

The engine **SHALL** validate all economy transactions server-side (currency generation, item
creation, trades, auction listings, crafting, loot drops) against game rule constraints, prevent
double-spend using transaction sequencing, detect gold farming patterns (repetitive behavior, bulk
transfers), and rate-limit high-value transactions with escalating delays.

- **Derived from:** [F-8.8.4](../../features/networking/anti-cheat.md)
- **Rationale:** Economy exploits undermine the game's core progression loop and can cause
  irreversible damage to the game economy.
- **Verification:** Integration test: attempt to craft an item without owning the required
  ingredients and verify rejection. Execute two concurrent trades spending the same currency and
  verify double-spend prevention (one succeeds, one fails). Simulate a gold farming pattern (100
  identical resource-gathering loops followed by a bulk transfer) and verify the pattern is flagged.
  Verify high-value transaction rate limiting applies escalating delays after the configured
  threshold.

## Session Security

### R-8.8.5 Rate Limiting and Abuse Prevention

The engine **SHALL** enforce per-connection and per-account rate limiting on all RPC calls, state
updates, and resource requests, with configurable per-RPC-type budgets (calls per second), cooldown
periods, burst allowances, and escalating responses (throttle, warn, kick, temporary ban), where
rate limit configurations are hot-reloadable without server restart.

- **Derived from:** [F-8.8.5](../../features/networking/anti-cheat.md)
- **Rationale:** Rate limiting protects against denial-of-service, chat spam, and API abuse from
  malicious or compromised clients.
- **Verification:** Integration test: send RPCs at 2x the configured rate limit and verify excess
  calls are throttled. Send at 10x and verify the connection is kicked after the configured
  escalation threshold. Verify per-account limits apply across multiple connections from the same
  account. Hot reload a rate limit configuration and verify the new limits take effect within 5
  seconds without server restart. Verify burst allowance permits short spikes within the configured
  burst window.
