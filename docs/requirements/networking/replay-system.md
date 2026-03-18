# R-8.6 — Replay System Requirements

## Recording

| ID      | Derived From                                          |
|---------|-------------------------------------------------------|
| R-8.6.1 | [F-8.6.1](../../features/networking/replay-system.md) |
| R-8.6.2 | [F-8.6.2](../../features/networking/replay-system.md) |

1. **R-8.6.1** — State Recording with Snapshots and Deltas: The engine **SHALL** record gameplay
   state as periodic full snapshots interleaved with per-tick delta streams, with configurable
   snapshot intervals, producing compact replay files where full snapshots serve as seek keyframes
   and deltas provide frame-accurate fidelity, with negligible CPU overhead on the server.
   - **Rationale:** Replay recording must not degrade live server performance; snapshot keyframes
     enable efficient seeking while deltas minimize file size.
   - **Verification:** Benchmark: record a 30-minute session with 1,000 entities and verify server
     CPU overhead is below 1%. Verify replay file size is at least 70% smaller than equivalent
     full-snapshot recording. Verify every Nth frame is a full snapshot matching the configured
     interval.
2. **R-8.6.2** — Deterministic Playback: The engine **SHALL** replay recorded state
   deterministically by feeding snapshots and deltas into the client's simulation and rendering
   pipeline without a live server connection, reproducing the exact visual result of the original
   session including interpolated entity motion and RPC-triggered effects.
   - **Rationale:** Replays must be visually identical to the original session for competitive
     review, dispute resolution, and content creation.
   - **Verification:** Integration test: record a 60-second session, play it back twice, and capture
     frame checksums at 10 sample points. Verify all checksums match between the two playbacks.
     Verify RPC-triggered particle effects and UI events fire at the correct timestamps. Verify
     playback works without any network connection.

## Playback Control

| ID      | Derived From                                          |
|---------|-------------------------------------------------------|
| R-8.6.3 | [F-8.6.3](../../features/networking/replay-system.md) |

1. **R-8.6.3** — Seek, Fast-Forward, and Slow Motion: The engine **SHALL** support seeking to any
   point in a replay by loading the nearest snapshot keyframe and replaying deltas forward,
   fast-forwarding at configurable speed multipliers (2x, 4x, 8x), slowing to frame-by-frame, and
   pausing, with seek completing within 1 second regardless of replay length.
   - **Rationale:** Competitive play review and content creation require precise navigation through
     recorded sessions.
   - **Verification:** Integration test: record a 2-hour replay. Seek to the midpoint and verify it
     completes within 1 second. Verify fast-forward at 8x produces correct state. Verify
     frame-by-frame advance shows exactly one tick of state change. Verify pause freezes all entity
     state.

## Spectating

| ID      | Derived From                                          |
|---------|-------------------------------------------------------|
| R-8.6.4 | [F-8.6.4](../../features/networking/replay-system.md) |
| R-8.6.5 | [F-8.6.5](../../features/networking/replay-system.md) |

1. **R-8.6.4** — Live Spectator Mode: The engine **SHALL** allow designated clients to observe an
   in-progress game session with a configurable delay (e.g., 30 seconds), receiving the full
   replication stream without the ability to send gameplay RPCs, supporting free camera,
   player-locked camera, and overhead tactical view, and scaling to thousands of spectators per
   match via fan-out relay servers.
   - **Rationale:** Esports broadcast and community spectating require high-capacity spectator
     support without burdening the game server.
   - **Verification:** Integration test: connect 1,000 spectators to a match via relay servers.
     Verify all receive the replication stream with the configured delay. Verify spectators cannot
     send gameplay RPCs. Verify free camera, player-locked, and tactical view modes produce correct
     rendering. Verify game server CPU is unaffected by spectator count.
2. **R-8.6.5** — Kill Cam and Highlight Extraction: The engine **SHALL** capture a rolling buffer of
   the last 10-15 seconds of state on the server and deliver it to the victim's client on death as a
   mini-replay from the attacker's perspective, using the deterministic playback pipeline, and
   support extracting highlight clips from ongoing or completed replays.
   - **Rationale:** Kill cams provide feedback that helps players improve and validate that deaths
     were legitimate; highlights enable social sharing and content creation.
   - **Verification:** Integration test: trigger a player death and verify the victim receives a
     kill cam replay within 2 seconds showing the attacker's perspective for the configured buffer
     duration. Verify the kill cam uses the same deterministic playback pipeline as full replays.
     Verify a highlight clip can be extracted from a completed replay and saved as an independent
     replay file.
