# Prediction and Rollback User Stories

## Client-Side Prediction

| ID       | Persona               |
|----------|-----------------------|
| US-8.4.1 | player (P-23)         |
| US-8.4.2 | game developer (P-15) |
| US-8.4.3 | game developer (P-15) |

1. **US-8.4.1** — **As a** player (P-23), **I want** my character to respond instantly to movement
   input even at 100+ ms ping, **so that** the game feels responsive and my actions are not delayed
   by network round-trip time.
2. **US-8.4.2** — **As a** game developer (P-15), **I want** to execute player inputs immediately on
   the client and reconcile with the server's authoritative result by replaying unacknowledged
   inputs, **so that** movement and abilities feel instant while the server remains authoritative.
3. **US-8.4.3** — **As a** game developer (P-15), **I want** redundant input transmission with
   configurable buffer depth that auto-tunes based on measured RTT and jitter, **so that** the
   server processes inputs in order even when packets are lost or arrive out of sequence.

## Interpolation

| ID       | Persona               |
|----------|-----------------------|
| US-8.4.4 | player (P-23)         |
| US-8.4.5 | player (P-23)         |

1. **US-8.4.4** — **As a** player (P-23), **I want** remote entities to interpolate smoothly between
   server snapshots at my render frame rate, **so that** other players and NPCs move fluidly rather
   than teleporting between tick updates.
2. **US-8.4.5** — **As a** player (P-23), **I want** remote entities to extrapolate using velocity
   when snapshot data is late, with smooth error correction when the next snapshot arrives,
   **so that** characters do not freeze during brief packet loss or jitter spikes.

## Lag Compensation

| ID       | Persona                 |
|----------|-------------------------|
| US-8.4.6 | player (P-23)           |
| US-8.4.7 | engine developer (P-26) |
| US-8.4.8 | engine developer (P-26) |

1. **US-8.4.6** — **As a** player (P-23), **I want** shots that looked correct on my screen to
   register on the server via hitbox rewinding, **so that** I am not penalized for network latency
   when I aim accurately.
2. **US-8.4.7** — **As an** engine developer (P-26), **I want** hitbox rewinding bounded by a
   maximum compensation window (e.g., 250 ms) that favors the defender when ambiguous, **so that**
   lag compensation feels fair without allowing high-latency players to exploit excessive rewind.
3. **US-8.4.8** — **As an** engine developer (P-26), **I want** the jitter buffer to dynamically
   expand during instability and contract when the connection stabilizes, **so that** added latency
   is minimized on good connections while stutter is prevented on unstable ones.

## Platform Tuning

| ID       | Persona               |
|----------|-----------------------|
| US-8.4.9 | game developer (P-15) |

1. **US-8.4.9** — **As a** game developer (P-15), **I want** mobile-specific defaults (deeper input
   buffer, wider interpolation window, longer extrapolation), **so that** players on cellular
   networks with higher jitter and RTT experience smooth gameplay without manual tuning.

## Testing

| ID        | Persona              |
|-----------|----------------------|
| US-8.4.10 | QA tester (P-19)     |
| US-8.4.11 | engine developer (P-26) |
| US-8.4.12 | engine developer (P-26) |

1. **US-8.4.10** — **As a** QA tester (P-19), **I want** automated tests that inject latency, packet
   loss, and jitter to verify that prediction, reconciliation, and interpolation produce correct
   visual results, **so that** prediction regressions are detected before release.
2. **US-8.4.11** — **As an** engine developer (P-26), **I want** performance benchmarks that measure
   rollback replay cost when replaying 4-8 frames of unacknowledged inputs, **so that** the CPU
   budget for reconciliation stays within frame time on mobile (4 frames) and desktop (8+ frames).
3. **US-8.4.12** — **As an** engine developer (P-26), **I want** test scenarios that verify hitbox
   rewinding produces identical hit results for the same input across RTT values from 20 ms to 250
   ms, **so that** lag compensation is deterministic and fair regardless of player latency.
