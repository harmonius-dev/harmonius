# R-8.6 -- Replay System Requirements

## Recording

1. **R-8.6.1** — The engine **SHALL** record gameplay state as periodic full snapshots interleaved
   with per-tick delta streams, with configurable snapshot intervals, producing compact replay files
   where full snapshots serve as seek keyframes and deltas provide frame-accurate fidelity, with
   negligible CPU overhead on the server.
   - **Rationale:** Replay recording must not degrade live server performance; snapshot keyframes
     enable efficient seeking while deltas minimize file size.
   - **Verification:** Benchmark: record a 30-minute session with 1,000 entities. Verify server CPU
     overhead below 1%. Verify replay file size at least 70% smaller than full-snapshot recording.
     Verify every Nth frame is a full snapshot at the configured interval.
2. **R-8.6.2** — The engine **SHALL** replay recorded state deterministically by feeding snapshots
   and deltas into the client's simulation and rendering pipeline without a live server connection,
   reproducing the exact visual result including interpolated motion and RPC-triggered effects.
   - **Rationale:** Replays must be visually identical to the original session for competitive
     review, dispute resolution, and content creation.
   - **Verification:** Integration test: record a 60-second session, play back twice, capture frame
     checksums at 10 sample points. Verify all checksums match. Verify RPC-triggered effects fire at
     correct timestamps. Verify playback works without a network connection.

## Playback Control

1. **R-8.6.3** — The engine **SHALL** support seeking to any point in a replay by loading the
   nearest snapshot keyframe and replaying deltas forward, fast-forwarding at configurable speed
   multipliers (2x, 4x, 8x), slowing to frame-by-frame, and pausing, with seek completing within 1
   second regardless of replay length.
   - **Rationale:** Competitive play review and content creation require precise navigation through
     recorded sessions.
   - **Verification:** Integration test: record a 2-hour replay. Seek to midpoint and verify under 1
     second. Verify 8x fast-forward produces correct state. Verify frame-by-frame shows exactly one
     tick of state change. Verify pause freezes all state.

## Spectating

1. **R-8.6.4** — The engine **SHALL** allow designated clients to observe an in-progress session
   with a configurable delay (e.g., 30 seconds), receiving the full replication stream without the
   ability to send gameplay RPCs, supporting free camera, player-locked camera, and overhead
   tactical view, scaling to thousands of spectators via fan-out relay servers.
   - **Rationale:** Esports broadcast and community spectating require high-capacity support without
     burdening the game server.
   - **Verification:** Integration test: connect 1,000 spectators via relay servers. Verify all
     receive the stream with configured delay. Verify spectators cannot send RPCs. Verify camera
     modes render correctly. Verify game server CPU unaffected by spectator count.
2. **R-8.6.5** — The engine **SHALL** capture a rolling buffer of the last 10-15 seconds of state on
   the server and deliver it to the victim's client on death as a mini-replay from the attacker's
   perspective, and support extracting highlight clips from replays.
   - **Rationale:** Kill cams help players improve and validate deaths; highlights enable social
     sharing and content creation.
   - **Verification:** Integration test: trigger a death and verify the kill cam arrives within 2
     seconds showing the attacker's perspective. Verify the kill cam uses the deterministic playback
     pipeline. Extract a highlight clip and verify it saves as an independent file.
