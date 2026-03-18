# User Stories -- 8.6 Replay System

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-8.6.1  | player (P-23)           | —        | —            |
| US-8.6.2  | player (P-23)           | —        | —            |
| US-8.6.3  | player (P-23)           | —        | —            |
| US-8.6.4  | player (P-23)           | —        | —            |
| US-8.6.5  | server admin (P-22)     | —        | —            |
| US-8.6.6  | server admin (P-22)     | —        | —            |
| US-8.6.7  | game developer (P-15)   | —        | —            |
| US-8.6.8  | engine developer (P-26) | —        | —            |
| US-8.6.9  | engine tester (P-27)    | —        | —            |
| US-8.6.10 | engine tester (P-27)    | —        | —            |
| US-8.6.11 | player (P-23)           | —        | —            |
| US-8.6.12 | designer (P-5)          | —        | —            |

1. **US-8.6.1** — As a player (P-23), I want a kill cam that replays the last 10 seconds from my
   attacker's viewpoint, so that I can understand how I was killed and improve my positioning.
   - **Acceptance:** —
2. **US-8.6.2** — As a player (P-23), I want to spectate live matches with a 30-second delay using
   free camera, player-locked camera, and overhead tactical view, so that I can watch competitive
   play from any angle without impacting the game server.
   - **Acceptance:** —
3. **US-8.6.3** — As a player (P-23), I want to seek, fast-forward (2x/4x/8x), slow down, and pause
   replays at any point with sub-second seek times, so that I can analyze my play in detail for
   competitive improvement or content creation.
   - **Acceptance:** —
4. **US-8.6.4** — As a player (P-23), I want to extract highlight clips from replays and encode them
   as video using platform media APIs, so that I can share memorable moments on social media without
   third-party screen recording.
   - **Acceptance:** —
5. **US-8.6.5** — As a server admin (P-22), I want replay recording to operate with negligible CPU
   overhead using periodic snapshots and per-tick deltas, so that recording every match does not
   reduce server capacity or increase tick time.
   - **Acceptance:** —
6. **US-8.6.6** — As a server admin (P-22), I want configurable snapshot intervals per game mode to
   trade file size against seek granularity, so that short competitive matches use frequent
   snapshots for precise analysis while long open-world sessions use sparser snapshots to conserve
   storage.
   - **Acceptance:** —
7. **US-8.6.7** — As a game developer (P-15), I want replays to feed snapshots and deltas into the
   client's simulation and rendering pipeline deterministically, so that playback reproduces the
   exact visual result of the original session including interpolated motion, particle effects, and
   UI events.
   - **Acceptance:** —
8. **US-8.6.8** — As an engine developer (P-26), I want spectator mode to use fan-out relay servers
   for replication stream distribution, so that thousands of spectators can watch a live match
   without burdening the authoritative game server.
   - **Acceptance:** —
9. **US-8.6.9** — As an engine tester (P-27), I want automated tests that record a match on one
   platform and play it back on all supported platforms, comparing visual output hash per frame, so
   that I can detect any cross-platform determinism regressions.
   - **Acceptance:** —
10. **US-8.6.10** — As an engine tester (P-27), I want tests that confirm spectators never receive
    replication data earlier than the configured delay, so that competitive integrity is maintained
    and spectator ghosting is impossible.
    - **Acceptance:** —
11. **US-8.6.11** — As a player (P-23), I want to download and watch replays on my mobile device
    with compressed file sizes and playback at my device's quality tier, so that I can review
    matches on the go without exceeding my storage or data budget.
    - **Acceptance:** —
12. **US-8.6.12** — As a designer (P-5), I want to configure spectator camera presets (overhead
    angle, follow distance, transition speed) per game mode, so that esports broadcasts have
    polished, game-mode-appropriate camera work out of the box.
    - **Acceptance:** —
