# R-8.4 — Prediction and Rollback Requirements

## Client-Side Prediction

### R-8.4.1 Input Prediction and Server Reconciliation

The engine **SHALL** execute player movement and ability inputs immediately on the client using the
same simulation logic as the server, and on server state mismatch, replay all unacknowledged inputs
atop the corrected server state within a single frame.

- **Derived from:** [F-8.4.1](../../features/networking/prediction-and-rollback.md)
- **Rationale:** Perceived input latency must be eliminated for the local player while preserving
  server authority, especially at 80-150 ms round-trip times common in MMO play.
- **Verification:** Integration test: simulate a client with 100 ms RTT. Issue a movement input and
  verify the client displays movement immediately (within 1 frame). Inject a server correction that
  differs from prediction. Verify the client replays unacknowledged inputs atop the corrected state
  and converges within 1 frame. Verify no visible teleport or stutter during correction.

### R-8.4.2 Input Buffering and Redundant Transmission

The engine **SHALL** buffer the last N client inputs and include redundant copies of recent inputs in
each outgoing packet, where buffer depth is tunable based on measured RTT and jitter, so that the
server can process inputs in order even when packets arrive out of sequence or are lost.

- **Derived from:** [F-8.4.2](../../features/networking/prediction-and-rollback.md)
- **Rationale:** Packet loss must not stall the server simulation; redundant input transmission
  provides resilience proportional to network quality.
- **Verification:** Integration test: send inputs over a link with 10% packet loss. Verify the server
  processes all inputs in order with zero gaps. Measure that buffer depth adapts upward when jitter
  increases and downward when the connection stabilizes. Verify no duplicate input processing on the
  server when redundant copies arrive.

## Interpolation

### R-8.4.3 Snapshot Interpolation

The engine **SHALL** interpolate remote entity state between two buffered server snapshots to produce
smooth visual motion at the client's render frame rate, with a configurable interpolation delay that
dynamically adjusts based on server tick rate variability and network jitter.

- **Derived from:** [F-8.4.3](../../features/networking/prediction-and-rollback.md)
- **Rationale:** Clients rendering at 60-240 FPS must display smooth motion from server updates
  arriving at 20-60 Hz; interpolation bridges the rate gap.
- **Verification:** Integration test: connect a 144 FPS client to a 20 Hz server. Verify remote
  entity motion is visually smooth with no jitter or teleporting. Introduce 30 ms jitter and verify
  the interpolation window expands to absorb it. Verify interpolation delay returns to minimum when
  jitter subsides.

### R-8.4.4 Entity Extrapolation with Error Correction

The engine **SHALL** extrapolate remote entity positions using velocity and acceleration when snapshot
data is late, and apply smooth error correction via exponential decay when the next snapshot arrives
and reveals prediction error.

- **Derived from:** [F-8.4.4](../../features/networking/prediction-and-rollback.md)
- **Rationale:** Entities must not freeze during packet loss or jitter spikes; extrapolation maintains
  visual continuity while error correction prevents accumulated drift.
- **Verification:** Integration test: delay a server snapshot by 200 ms. Verify the client
  extrapolates the entity's position using its last known velocity without freezing. When the delayed
  snapshot arrives, verify the error correction converges to the true position within 200 ms using
  exponential decay. Verify no visible snap or teleport.

## Lag Compensation

### R-8.4.5 Server-Side Lag Compensation (Hitbox Rewinding)

The engine **SHALL** rewind authoritative hitbox positions on the server to match what the attacking
client observed at the time of input, bounded by a configurable maximum compensation window (e.g.,
250 ms), and favor the defender when the result is ambiguous.

- **Derived from:** [F-8.4.5](../../features/networking/prediction-and-rollback.md)
- **Rationale:** Hits that appeared correct on the attacker's screen must register on the server
  despite network latency; bounding the window prevents abuse by high-latency or artificial-lag
  clients.
- **Verification:** Integration test: simulate an attacker at 100 ms RTT firing at a moving target.
  Verify the server rewinds the target's hitbox to the attacker's observed position and registers the
  hit. Simulate an attacker at 300 ms RTT (exceeding the 250 ms window) and verify the server does
  not rewind beyond the maximum. Simulate an ambiguous hit at the boundary and verify the defender
  is favored.

### R-8.4.6 Jitter Buffer and Adaptive Tick Alignment

The engine **SHALL** maintain a jitter buffer on both client and server that absorbs network timing
variance, dynamically expanding during instability and contracting when the connection is stable, to
deliver packets at a steady cadence to the simulation. This jitter buffer handles game state update
packets only. Voice chat packets **SHALL** be routed directly to the audio-domain jitter buffer
(R-5.5.2) and **SHALL NOT** pass through this buffer. Packet type is distinguished by a channel type
field in the transport header.

- **Derived from:** [F-8.4.6](../../features/networking/prediction-and-rollback.md)
- **Rationale:** Consumer internet connections exhibit variable latency; the jitter buffer must
  minimize added latency while preventing stutter.
- **Verification:** Integration test: introduce 0-50 ms random jitter on a link. Verify the jitter
  buffer expands to absorb variance and the simulation receives packets at a steady rate with no
  stutter. Remove jitter and verify the buffer contracts within 5 seconds. Measure added latency
  and verify it does not exceed 2x the measured jitter standard deviation.
