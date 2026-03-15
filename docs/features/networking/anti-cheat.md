# 8.8 — Anti-Cheat and Security

## Client Integrity

### F-8.8.1 Server-Side Cheat Detection

Server-authoritative validation of all client actions. The server independently simulates
movement, ability usage, damage, and economy transactions, comparing client-reported state
against server-computed bounds. Detects speed hacks (movement exceeding maximum velocity),
teleportation (position discontinuities), damage manipulation (damage exceeding weapon
stats x multipliers), cooldown circumvention, and impossible inventory operations. Violations
are scored and escalated through configurable severity tiers (warn, flag, kick, ban).
Detection thresholds account for legitimate network latency and prediction error.

- **Requirements:** R-8.8.1
- **Dependencies:** F-8.4.1 (Client-Side Prediction), F-8.2.1 (Property Replication)
- **Platform notes:** Server-side; platform-agnostic. Mobile detection thresholds are wider
  to account for higher latency and more frequent jitter spikes.

### F-8.8.2 Client Integrity Verification

Periodic client integrity checks to detect modified game binaries, injected code, and memory
tampering. The server challenges the client with encrypted memory region queries; the client
responds with hashed snapshots of code segments and critical data structures. Responses are
validated against known-good hashes for the client's reported build version. Failure triggers
escalation through the same severity pipeline as gameplay cheat detection. The system is
designed to raise the difficulty of tampering, not to be unbreakable — server-side validation
(F-8.8.1) remains the primary defense.

- **Requirements:** R-8.8.2
- **Dependencies:** F-8.1.5 (Encryption)
- **Platform notes:** Platform-specific integrity APIs are used where available (GameGuard
  on consoles).

### F-8.8.3 Behavioral Analysis and Anomaly Detection

Statistical analysis of player behavior patterns over time to detect sophisticated cheats that
pass individual frame checks. Tracks distributions of reaction times, aim accuracy percentiles,
movement pattern entropy, resource acquisition rates, and win/loss streaks. Anomalous patterns
are flagged for review using configurable Z-score thresholds. The system builds per-player
baselines and detects deviations, distinguishing improving skill from sudden capability jumps.
Analysis runs server-side on aggregated telemetry data, not in real-time game simulation.

- **Requirements:** R-8.8.3
- **Dependencies:** F-8.8.1
- **Platform notes:** Server-side analytics; platform-agnostic. Per-player baselines are
  segmented by input type (touch vs. controller vs. keyboard/mouse).

### F-8.8.4 Economy Exploit Prevention

Server-side validation of all economy transactions: currency generation, item creation, trades,
auction house listings, crafting recipes, and loot drops. Every transaction is checked against
game rule constraints (e.g., crafting requires owned ingredients, trade values are within
bounds, drop rates match server-side loot tables). Double-spend prevention using transaction
sequencing. Automated detection of gold farming patterns (repetitive behavior, bulk transfers
to mule accounts). Rate limiting on high-value transactions with escalating delays.

- **Requirements:** R-8.8.4
- **Dependencies:** F-8.8.1, F-13.9.1 (Inventory Containers)
- **Platform notes:** Server-side validation; platform-agnostic. All client platforms
  submit economy operations through the same validated RPC path.

## Session Security

### F-8.8.5 Rate Limiting and Abuse Prevention

Per-connection and per-account rate limiting on all RPC calls, state updates, and resource
requests. Each RPC type has a configurable budget (calls per second), cooldown, and burst
allowance. Exceeding limits triggers escalating responses: throttle (delay processing), warn
(log + notify), kick (disconnect), and temporary ban. Protects against denial-of-service from
malicious clients, chat spam, and API abuse. Rate limit configurations are hot-reloadable
without server restart.

- **Requirements:** R-8.8.5
- **Dependencies:** F-8.3.1 (Server RPC), F-8.1.1 (Connection Management)
- **Platform notes:** Rate limit profiles are tuned per platform; mobile touch input
  generates fewer RPCs/s than keyboard/mouse, so limits are lower by default.
