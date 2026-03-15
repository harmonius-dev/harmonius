# User Stories -- 8.4 Prediction and Rollback

## US-8.4.1 Move Without Feeling Input Lag

**As a** player (P-23), **I want** my character to respond instantly to movement input even
at 100+ ms ping, **so that** the game feels responsive and my actions are not delayed by
network round-trip time.

## US-8.4.2 Hit What I Aim At Despite Latency

**As a** player (P-23), **I want** shots that looked correct on my screen to register on
the server via hitbox rewinding, **so that** I am not penalized for network latency when I
aim accurately.

## US-8.4.3 See Remote Players Move Smoothly

**As a** player (P-23), **I want** remote entities to interpolate smoothly between server
snapshots at my render frame rate, **so that** other players and NPCs move fluidly rather
than teleporting between tick updates.

## US-8.4.4 Avoid Freezing When Packets Are Late

**As a** player (P-23), **I want** remote entities to extrapolate using velocity when
snapshot data is late, with smooth error correction when the next snapshot arrives, **so
that** characters do not freeze during brief packet loss or jitter spikes.

## US-8.4.5 Implement Client Prediction with Server Reconciliation

**As a** game developer (P-15), **I want** to execute player inputs immediately on the
client and reconcile with the server's authoritative result by replaying unacknowledged
inputs, **so that** movement and abilities feel instant while the server remains
authoritative.

## US-8.4.6 Buffer Inputs to Tolerate Packet Loss

**As a** game developer (P-15), **I want** redundant input transmission with configurable
buffer depth that auto-tunes based on measured RTT and jitter, **so that** the server
processes inputs in order even when packets are lost or arrive out of sequence.

## US-8.4.7 Limit Lag Compensation Window to Prevent Abuse

**As an** engine developer (P-26), **I want** hitbox rewinding bounded by a maximum
compensation window (e.g., 250 ms) that favors the defender when ambiguous, **so that**
lag compensation feels fair without allowing high-latency players to exploit excessive
rewind distances.

## US-8.4.8 Adapt Jitter Buffer Depth to Network Conditions

**As an** engine developer (P-26), **I want** the jitter buffer to dynamically expand
during instability and contract when the connection stabilizes, **so that** added latency
is minimized on good connections while stutter is prevented on unstable ones.

## US-8.4.9 Tune Prediction for Mobile Network Characteristics

**As a** game developer (P-15), **I want** mobile-specific defaults (deeper input buffer,
wider interpolation window, longer extrapolation), **so that** players on cellular networks
with higher jitter and RTT experience smooth gameplay without manual tuning.

## US-8.4.10 Verify Prediction Correctness Under Adversarial Conditions

**As an** engine tester (P-27), **I want** automated tests that inject latency, packet
loss, and jitter to verify that prediction, reconciliation, and interpolation produce
correct visual results without desyncs, **so that** I can detect prediction regressions
across all platforms before release.

## US-8.4.11 Benchmark Rollback Replay Performance

**As an** engine tester (P-27), **I want** performance benchmarks that measure rollback
replay cost when replaying 4-8 frames of unacknowledged inputs, **so that** I can verify
the CPU budget for reconciliation stays within frame time on mobile (4 frames) and
desktop (8+ frames).

## US-8.4.12 Verify Deterministic Hit Registration Across Latency Ranges

**As an** engine tester (P-27), **I want** test scenarios that verify hitbox rewinding
produces identical hit results for the same input across RTT values from 20 ms to 250 ms,
**so that** lag compensation is verified as deterministic and fair regardless of player
latency.
