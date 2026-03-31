# R-8.4 -- Prediction and Rollback Requirements

## Client-Side Prediction

1. **R-8.4.1** — The engine **SHALL** execute player movement and ability inputs immediately on the
   client using the same simulation logic as the server, and on server state mismatch, replay all
   unacknowledged inputs atop the corrected server state within a single frame.
   - **Rationale:** Perceived input latency must be eliminated for the local player while preserving
     server authority, especially at 80-150 ms RTTs common in MMO play.
   - **Verification:** Integration test: simulate a client with 100 ms RTT. Issue a movement input
     and verify display within 1 frame. Inject a server correction. Verify the client replays
     unacknowledged inputs and converges within 1 frame with no visible teleport.
2. **R-8.4.2** — The engine **SHALL** buffer the last N client inputs and include redundant copies
   of recent inputs in each outgoing packet, where buffer depth is tunable based on measured RTT and
   jitter, so that the server processes inputs in order even when packets are lost.
   - **Rationale:** Packet loss must not stall the server simulation; redundant input transmission
     provides resilience proportional to network quality.
   - **Verification:** Integration test: send inputs over a link with 10% packet loss. Verify the
     server processes all inputs in order with zero gaps. Verify buffer depth adapts upward when
     jitter increases and downward when the connection stabilizes.

## Interpolation

1. **R-8.4.3** — The engine **SHALL** interpolate remote entity state between two buffered server
   snapshots to produce smooth visual motion at the client's render frame rate, with a configurable
   interpolation delay that dynamically adjusts based on server tick rate variability and network
   jitter.
   - **Rationale:** Clients rendering at 60-240 FPS must display smooth motion from server updates
     arriving at 20-60 Hz; interpolation bridges the rate gap.
   - **Verification:** Integration test: connect a 144 FPS client to a 20 Hz server. Verify smooth
     remote entity motion. Introduce 30 ms jitter and verify the interpolation window expands.
     Verify it returns to minimum when jitter subsides.
2. **R-8.4.4** — The engine **SHALL** extrapolate remote entity positions using velocity and
   acceleration when snapshot data is late, and apply smooth error correction via exponential decay
   when the next snapshot arrives.
   - **Rationale:** Entities must not freeze during packet loss or jitter spikes; extrapolation
     maintains visual continuity while error correction prevents accumulated drift.
   - **Verification:** Integration test: delay a server snapshot by 200 ms. Verify the client
     extrapolates position using last known velocity. When the delayed snapshot arrives, verify
     error correction converges within 200 ms with no visible snap.

## Lag Compensation

1. **R-8.4.5** — The engine **SHALL** rewind authoritative hitbox positions on the server to match
   what the attacking client observed at the time of input, bounded by a configurable maximum
   compensation window (e.g., 250 ms), and favor the defender when the result is ambiguous.
   - **Rationale:** Hits that appeared correct on the attacker's screen must register despite
     latency; bounding the window prevents abuse by high-latency or artificial-lag clients.
   - **Verification:** Integration test: simulate an attacker at 100 ms RTT firing at a moving
     target. Verify the server rewinds and registers the hit. Simulate 300 ms RTT exceeding the 250
     ms window and verify the server does not rewind beyond the maximum. Verify the defender is
     favored on ambiguous hits.
2. **R-8.4.6** — The engine **SHALL** maintain a jitter buffer on both client and server that
   absorbs network timing variance, dynamically expanding during instability and contracting when
   stable. Game state packets use this buffer. Voice chat packets **SHALL** be routed to the
   audio-domain jitter buffer and **SHALL NOT** pass through this buffer.
   - **Rationale:** Consumer internet connections exhibit variable latency; the jitter buffer must
     minimize added latency while preventing stutter.
   - **Verification:** Integration test: introduce 0-50 ms random jitter. Verify the buffer expands
     to absorb variance. Remove jitter and verify the buffer contracts within 5 seconds. Verify
     added latency does not exceed 2x the measured jitter standard deviation.

## Non-Functional

1. **R-8.4.7** — Client-side prediction **SHALL** make player inputs appear to execute within 1
   render frame (< 16.7 ms at 60 fps) regardless of network round-trip time. Server reconciliation
   **SHALL** complete within a single simulation tick with no visible teleport for corrections under
   50 ms of positional drift.
   - **Rationale:** Perceived input latency is the single most important factor in action gameplay
     feel.
   - **Verification:** Integration test: simulate 150 ms RTT. Verify local input response within 1
     frame. Inject a 30 ms positional correction and verify no visible snap. Inject a 100 ms
     correction and verify smooth convergence within 200 ms.
