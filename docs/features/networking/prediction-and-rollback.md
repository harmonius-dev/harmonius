# 8.4 — Prediction and Rollback

## Client-Side Prediction

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-8.4.1 | Input Prediction and Server Reconciliation | Execute player movement and ability inputs immediately on the client using the same simulation logic as the server, then reconcile when the server's authoritative result arrives. On mismatch, replay all unacknowledged inputs atop the corrected server state within a single frame. This eliminates perceived input latency for the local player while preserving server authority — essential for responsive combat in an MMO where round-trip times commonly reach 80-150 ms. | R-8.4.1 | F-8.2.1, F-8.1.3 | Mobile limits rollback replay to 4 frames (vs. 8+ on desktop) to bound per-frame CPU cost. Higher mobile RTT makes prediction more important. |
| F-8.4.2 | Input Buffering and Redundant Transmission | Buffer the last N client inputs and include redundant copies of recent inputs in each packet to tolerate packet loss without stalling the server simulation. The server uses the input buffer to process inputs in order even when packets arrive out of sequence or are lost. Buffer depth is tunable based on measured round-trip time and jitter. | R-8.4.2 | F-8.4.1 | Mobile uses deeper default buffer (6 frames vs. 3) to absorb higher jitter on cellular networks. Redundancy level is auto-tuned from loss rate. |

## Interpolation

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-8.4.3 | Snapshot Interpolation | Interpolate remote entity state between two buffered server snapshots to produce smooth visual motion at the client's render frame rate, independent of the server tick rate. The interpolation delay (typically one server tick period) is configurable and trades visual latency for smoothness. Handles variable server tick rates and network jitter by dynamically adjusting the interpolation window. | R-8.4.3 | F-8.2.1 | Mobile uses a wider interpolation window (2 ticks vs. 1) to compensate for higher jitter, trading slightly more visual latency for smoothness. |
| F-8.4.4 | Entity Extrapolation with Error Correction | Extrapolate remote entity positions using velocity and acceleration when snapshot data is late, with smooth error correction (exponential decay) when the next snapshot arrives and reveals prediction error. Extrapolation prevents entities from freezing during packet loss or jitter spikes, which is especially visible during large-scale PvP where dozens of players move simultaneously. | R-8.4.4 | F-8.4.3 | Mobile extrapolation window is longer (200 ms vs. 100 ms) to cover more frequent late packets. Error correction uses faster decay on mobile. |

## Lag Compensation

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-8.4.5 | Server-Side Lag Compensation (Hitbox Rewinding) | Rewind authoritative hitbox positions on the server to match what the attacking client saw at the time of input, accounting for the client's round-trip latency and interpolation delay. This ensures that hits that looked correct on the attacker's screen register on the server, even at high latency. Rewinding is bounded by a maximum compensation window (e.g., 250 ms) to prevent abuse, and favors the defender when ambiguous. | R-8.4.5 | F-8.4.1, F-8.4.3 | None |
| F-8.4.6 | Jitter Buffer and Adaptive Tick Alignment | Maintain a jitter buffer on both client and server to absorb network timing variance and deliver packets at a steady cadence to the simulation. The buffer depth adapts dynamically based on measured jitter — expanding during instability and contracting when the connection is stable — to minimize added latency while preventing stutter. Critical for maintaining smooth gameplay over consumer internet connections. | R-8.4.6 | F-8.1.7 | Mobile defaults to a deeper jitter buffer (3-5 ticks vs. 1-3 on desktop) to absorb cellular network variance. Wi-Fi vs. cellular is auto-detected. |
