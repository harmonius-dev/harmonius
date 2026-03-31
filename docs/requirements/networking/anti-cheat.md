# R-8.8 -- Anti-Cheat and Security Requirements

## Server-Side Detection

1. **R-8.8.1** — The engine **SHALL** perform server-authoritative validation of all client actions
   by independently simulating movement, ability usage, damage, and economy transactions, comparing
   client-reported state against server-computed bounds, detecting violations (speed hacks,
   teleportation, damage manipulation, cooldown circumvention), scoring them, and escalating through
   configurable severity tiers (warn, flag, kick, ban) with thresholds that account for legitimate
   network latency and prediction error.
   - **Rationale:** Server-authoritative validation is the primary defense against cheating; all
     other anti-cheat measures are supplementary.
   - **Verification:** Integration test: submit movement exceeding maximum velocity and verify speed
     hack detection. Submit a position discontinuity and verify teleportation detection. Verify
     violation scoring and escalation. Verify a legitimate client with 150 ms RTT is not falsely
     flagged.

## Client Integrity

1. **R-8.8.2** — The engine **SHALL** perform periodic client integrity checks by issuing encrypted
   memory region challenges from the server, validating responses against known-good hashes for the
   client's build version, and escalating failures through the same severity pipeline as gameplay
   cheat detection.
   - **Rationale:** Detecting modified binaries raises the difficulty of tampering, complementing
     server-side validation as defense-in-depth.
   - **Verification:** Integration test: connect an unmodified client and verify checks pass. Modify
     a code segment and verify hash mismatch detection. Verify the challenge-response is encrypted
     and cannot be replayed.

## Behavioral Analysis

1. **R-8.8.3** — The engine **SHALL** perform server-side statistical analysis of player behavior
   patterns over time (reaction times, aim accuracy, movement entropy, resource acquisition rates,
   win/loss streaks), building per-player baselines and flagging anomalous deviations using
   configurable Z-score thresholds, distinguishing improving skill from sudden capability jumps.
   - **Rationale:** Sophisticated cheats that pass individual frame checks can be detected through
     statistical patterns over time.
   - **Verification:** Integration test: simulate consistent accuracy for 100 matches, then inject a
     3-sigma jump. Verify the anomaly is flagged. Simulate gradual improvement over 50 matches and
     verify no false flag. Verify analysis runs without impacting real-time simulation.

## Economy Protection

1. **R-8.8.4** — The engine **SHALL** validate all economy transactions server-side (currency
   generation, item creation, trades, auction listings, crafting, loot drops) against game rule
   constraints, prevent double-spend using transaction sequencing, detect gold farming patterns, and
   rate-limit high-value transactions with escalating delays.
   - **Rationale:** Economy exploits undermine the game's core progression loop and can cause
     irreversible damage to the game economy.
   - **Verification:** Integration test: attempt to craft without ingredients and verify rejection.
     Execute two concurrent trades spending the same currency and verify double-spend prevention.
     Simulate a gold farming pattern and verify flagging. Verify high-value rate limiting.

## Rate Limiting

1. **R-8.8.5** — The engine **SHALL** enforce per-connection and per-account rate limiting on all
   RPC calls, state updates, and resource requests, with configurable per-RPC-type budgets, burst
   allowances, and escalating responses (throttle, warn, kick, temporary ban), where rate limit
   configurations are hot-reloadable without server restart.
   - **Rationale:** Rate limiting protects against denial-of-service, chat spam, and API abuse from
     malicious or compromised clients.
   - **Verification:** Integration test: send RPCs at 2x the rate limit and verify throttling. Send
     at 10x and verify kick. Verify per-account limits across multiple connections. Hot reload
     configuration and verify new limits take effect within 5 seconds.

## Non-Functional

1. **R-8.8.6** — The server-side cheat detection system **SHALL** maintain a false positive rate
   below 0.1% (no more than 1 false flag per 1,000 legitimate players) when detection thresholds
   account for network latency up to 200 ms and prediction drift up to 50 ms.
   - **Rationale:** False positives erode player trust and generate support burden.
   - **Verification:** Load test: simulate 10,000 legitimate clients with RTTs from 20 to 200 ms.
     Verify no more than 10 falsely flagged over 60 minutes. Simulate 100 cheating clients and
     verify at least 95% detected.
