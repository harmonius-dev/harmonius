# Network Services — Test Cases

Companion to [network-services.md](network-services.md).

Test case IDs use `TC-8.5.Y.N`, `TC-8.6.Y.N`, and `TC-8.9.Y.N` formats. Every row links to a
specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                                    | Req       |
|---------------|-----------------------------------------|-----------|
| TC-8.5.1.1    | `test_oauth_token_issued`               | R-8.5.1   |
| TC-8.5.1.2    | `test_token_expires`                    | R-8.5.1   |
| TC-8.5.2.1    | `test_matchmaker_skill_window`          | R-8.5.2   |
| TC-8.5.2.2    | `test_matchmaker_widens_over_time`      | R-8.5.2   |
| TC-8.5.3.1    | `test_party_role_assignment`            | R-8.5.3   |
| TC-8.5.3.2    | `test_party_ready_check`                | R-8.5.3   |
| TC-8.5.5.1    | `test_reconnect_within_grace`           | R-8.5.5   |
| TC-8.5.5.2    | `test_reconnect_after_grace`            | R-8.5.5   |
| TC-8.5.7.1    | `test_login_queue_position`             | R-8.5.7   |
| TC-8.5.7.2    | `test_login_queue_vip_priority`         | R-8.5.7   |
| TC-8.5.8.1    | `test_headless_no_gpu_init`             | R-8.5.8   |
| TC-8.5.8.2    | `test_headless_health_endpoint`         | R-8.5.8   |
| TC-8.5.9.1    | `test_glicko2_recalc`                   | R-8.5.9   |
| TC-8.5.18.1   | `test_backfill_replacement`             | R-8.5.18  |
| TC-8.5.19.1   | `test_abandon_escalation`               | R-8.5.19  |
| TC-8.5.20.1   | `test_rating_decay_inactivity`          | R-8.5.20  |
| TC-8.6.1.1    | `test_replay_keyframe_interval`         | R-8.6.1   |
| TC-8.6.2.1    | `test_replay_deterministic_playback`    | R-8.6.2   |
| TC-8.6.3.1    | `test_replay_seek_to_midpoint`          | R-8.6.3   |
| TC-8.6.5.1    | `test_kill_cam_rolling_buffer`          | R-8.6.5   |
| TC-8.9.1.1    | `test_channel_create_and_join`          | R-8.9.1   |
| TC-8.9.3.1    | `test_text_message_mention`             | R-8.9.3   |
| TC-8.9.6.1    | `test_voice_proximity_attenuation`      | R-8.9.6   |
| TC-8.9.8.1    | `test_dm_e2e_encrypted`                 | R-8.9.8   |

1. **TC-8.5.1.1** `test_oauth_token_issued` — `AuthService::login_oauth(provider, code)` returns a
   short-lived `SessionToken` for a valid code.
   - Input: `Provider::Steam`, `code = "valid"`, mock IdP returning `sub = "76561198000000001"`
   - Expected: `Ok(SessionToken { player_id, expires_at })`, `expires_at - now() < ttl + 1s`,
     `player_id` resolves to the IdP subject

2. **TC-8.5.1.2** `test_token_expires` — A session token issued with TTL 60 s is rejected after the
   TTL elapses.
   - Input: token issued at `t0` with `ttl = 60s`, validated at `t0 + 61s`
   - Expected: `validate_token(token)` returns `Err(TokenError::Expired)`

3. **TC-8.5.2.1** `test_matchmaker_skill_window` — Matchmaker pairs two players whose Glicko-2
   ratings are within the configured window.
   - Input: queue with `Player(rating=1500)` and `Player(rating=1530)`, window = 50
   - Expected: `Match { players: [a, b], avg_rating: 1515 }` returned within one tick; both players
     removed from the queue

4. **TC-8.5.2.2** `test_matchmaker_widens_over_time` — Initial window is 25; after 30 s a player has
   not matched; window widens to 50.
   - Input: solitary `Player(rating=1500)` queued at `t0`, no compatible players, widen interval
     `10 s`, widen step 25
   - Expected: `effective_window(player, t0 + 30s) == 100`

5. **TC-8.5.3.1** `test_party_role_assignment` — Party leader assigns roles; assignment persists in
   the `PartyComponent`.
   - Input: party with 3 members, `assign_role(member_a, Role::Tank)`,
     `assign_role(member_b, Role::Healer)`
   - Expected: `party.role_of(member_a) == Some(Role::Tank)`, only one tank, only one healer

6. **TC-8.5.3.2** `test_party_ready_check` — All members respond ready within the timeout; ready
   check completes.
   - Input: 5-player party, `start_ready_check(timeout: 30s)`, all 5 send `Ready`
   - Expected: `ReadyCheckResult::AllReady` emitted; no timeout fires

7. **TC-8.5.5.1** `test_reconnect_within_grace` — Player disconnects for 10 s, reconnects within
   grace; full state restored.
   - Input: session with `grace_window = 120s`, disconnect at `t0`, reconnect at `t0 + 10s`
   - Expected: `reconnect()` returns `Ok(SessionRestored { player_id })`; player entity has
     identical `Transform`, `Health`, buffs as before disconnect

8. **TC-8.5.5.2** `test_reconnect_after_grace` — Player disconnects for 200 s; grace expires;
   reconnect produces a clean logout.
   - Input: `grace_window = 120s`, reconnect at `t0 + 200s`
   - Expected: `reconnect()` returns `Err(SessionError::GraceExpired)`; session fully torn down;
     player entity removed

9. **TC-8.5.7.1** `test_login_queue_position` — Server at capacity; new login enters the queue.
   - Input: `LoginQueue { capacity: 100, current: 100 }`, `enqueue(player_id)`
   - Expected: `QueuePosition { player_id, position: 1, eta: Some(_) }`; queue length = 1

10. **TC-8.5.7.2** `test_login_queue_vip_priority` — VIP-tier player jumps ahead of regular tier.
    - Input: queue with 3 regular players, then `enqueue(vip_player, tier: Vip)`
    - Expected: `position_of(vip_player) == 1`; regular players' positions shifted by +1

11. **TC-8.5.8.1** `test_headless_no_gpu_init` — Headless mode initializes the simulation without
    constructing GPU, window, audio, or input subsystems.
    - Input: `HeadlessServer::new(Config { players: 0 })` on a machine with no display
    - Expected: `init()` returns `Ok(())`; `subsystems()` does not contain `Renderer`,
      `WindowSystem`, `AudioMixer`, or `InputDevice`

12. **TC-8.5.8.2** `test_headless_health_endpoint` — `GET /health` returns 200 with the current tick
    rate and player count.
    - Input: HTTP request `GET /health` against the headless server
    - Expected: HTTP 200, body `{ "status": "ok", "tick_hz": 30, "players": _ }`

13. **TC-8.5.9.1** `test_glicko2_recalc` — Match completes; Glicko-2 update produces new rating,
    deviation, and volatility for both players.
    - Input: `PlayerA(rating=1500, rd=200, vol=0.06)`, `PlayerB(rating=1500, rd=200, vol=0.06)`,
      result `A wins`
    - Expected: `A.rating > 1500`, `B.rating < 1500`, both `rd` decrease, both `vol` updated per
      Glicko-2 formula

14. **TC-8.5.18.1** `test_backfill_replacement` — Player leaves mid-match; backfill request finds a
    skill-compatible replacement within wait window.
    - Input: 5v5 match, player leaves at `t0`, queue contains a candidate within rating window
    - Expected: `BackfillRequest` issued; replacement matched within configured wait window;
      replacement's rating within window of remaining team average

15. **TC-8.5.19.1** `test_abandon_escalation` — Three abandons within rolling window; penalties
    escalate cooldown -> rating penalty -> temporary ban.
    - Input: `record_abandon(player)` x3 within 24 h
    - Expected: penalties emitted in order `[Cooldown(15m), RatingPenalty(-50), TempBan(24h)]`; 4th
      abandon within window emits `TempBan(7d)`

16. **TC-8.5.20.1** `test_rating_decay_inactivity` — Player inactive 30 days; rating deviation
    grows.
    - Input: `Player { rating: 1800, rd: 60 }`, last active 30 days ago, decay rate per Glicko-2
      step
    - Expected: `effective_rd(player, now) > 60` after decay applied; rating unchanged

17. **TC-8.6.1.1** `test_replay_keyframe_interval` — Recorder writes a full snapshot every N ticks;
    ticks in between are deltas.
    - Input: `ReplayRecorder { keyframe_interval: 30 }`, record 90 ticks
    - Expected: replay file has 3 keyframe entries at ticks 0, 30, 60; 87 delta entries; total size
      at least 70% smaller than full-snapshot recording

18. **TC-8.6.2.1** `test_replay_deterministic_playback` — Playing the same replay twice produces
    identical frame checksums at sample points.
    - Input: 60-second replay, sample tick checksums at ticks 0, 30, 60, ..., 1800
    - Expected: checksums identical across two playback runs; deviation count == 0

19. **TC-8.6.3.1** `test_replay_seek_to_midpoint` — `ReplayPlayer::seek(t)` loads nearest keyframe
    and replays deltas forward.
    - Input: 2-hour replay, `seek(Duration::from_secs(3600))`
    - Expected: `seek` returns within 1 s; resulting frame checksum matches the recorded checksum at
      tick `30 * 3600`

20. **TC-8.6.5.1** `test_kill_cam_rolling_buffer` — Rolling buffer keeps last 10 s; on death,
    extracts the cam from the buffer.
    - Input: `KillCamBuffer { window: 10s }`, simulation runs for 60 s, death at `t = 60`
    - Expected: extracted clip covers `t in [50, 60]`; clip is independently playable via the replay
      pipeline

21. **TC-8.9.1.1** `test_channel_create_and_join` — Create a `Party` channel and join two members.
    - Input: `channels.create(ChannelKind::Party)`, `join(member_a)`, `join(member_b)`
    - Expected: `channel.members().len() == 2`, both members listed; channel has unique `ChannelId`

22. **TC-8.9.3.1** `test_text_message_mention` — Message containing `@playerB` triggers a mention
    event for player B.
    - Input: `chat.send(channel, sender: A, "hello @playerB")`
    - Expected: B receives the message and a `MentionNotification { sender: A, channel }`; A
      receives no notification

23. **TC-8.9.6.1** `test_voice_proximity_attenuation` — Two players in the same proximity voice
    channel; attenuation applied based on distance.
    - Input: speakers at distance `10 m` and `100 m` from listener; attenuation curve linear to
      `120 m`
    - Expected: gain at 10 m > gain at 100 m; both gains in `(0.0, 1.0]`; gain at 130 m == 0.0

24. **TC-8.9.8.1** `test_dm_e2e_encrypted` — DM payload is encrypted with X25519-derived key; server
    cannot decrypt.
    - Input: DM `from=A, to=B, plaintext="hi"`, observe wire bytes
    - Expected: wire payload != `"hi"`; decryption with B's session-derived key returns `"hi"`;
      decryption with a third party's key fails

## Integration Tests

| ID            | Name                                  | Req       |
|---------------|---------------------------------------|-----------|
| TC-8.5.I.1    | `test_oauth_5k_concurrent_logins`     | R-8.5.1   |
| TC-8.5.I.2    | `test_matchmake_20k_queue`            | R-8.5.2   |
| TC-8.5.I.3    | `test_reconnect_full_restore`         | R-8.5.10  |
| TC-8.5.I.4    | `test_headless_64_players_30hz`       | R-8.5.11  |
| TC-8.5.I.5    | `test_cross_play_three_platforms`     | R-8.5.6   |
| TC-8.6.I.1    | `test_spectator_1k_relay`             | R-8.6.4   |
| TC-8.9.I.1    | `test_text_chat_50_member_p99`        | R-8.9.4   |
| TC-8.9.I.2    | `test_voice_mouth_to_ear_latency`     | R-8.9.7   |

1. **TC-8.5.I.1** `test_oauth_5k_concurrent_logins` — Submit 5,000 concurrent OAuth logins; verify
   all complete within 5 seconds.
   - Input: 5,000 concurrent `login_oauth(Steam, code_n)` calls against a mock IdP at p99 50 ms
     response time
   - Expected: all return `Ok(SessionToken)`; total wall time < 5 s; no auth service crashes; issued
     tokens are unique

2. **TC-8.5.I.2** `test_matchmake_20k_queue` — Enqueue 20,000 players with varied skill ratings and
   regions; assert all matched within 120 s.
   - Input: 20,000 players sampled from `Normal(rating=1500, sd=200)`, 4 regions evenly
   - Expected: all enqueue calls succeed; all matched within 120 s; matched groups have
     `rating_variance < window`; intra-region matching preferred

3. **TC-8.5.I.3** `test_reconnect_full_restore` — 10-second disconnect; assert exact-state
   restoration within 3 s.
   - Input: player in active combat, disconnect at `t0`, reconnect at `t0 + 10s`
   - Expected: restoration wall time < 3 s; `Transform`, `Health`, buffs, group, combat target, and
     pending RPCs all match the pre-disconnect snapshot

4. **TC-8.5.I.4** `test_headless_64_players_30hz` — Launch headless server in Docker; connect 64
   simulated clients; run 10 minutes; verify 30 Hz and < 512 MB RSS.
   - Input: headless container with `--players=64 --tick-hz=30 --duration=10m`
   - Expected: tick rate stays at 30 Hz over the run; peak RSS < 512 MB; no OOM; graceful shutdown
     saves all 64 player states

5. **TC-8.5.I.5** `test_cross_play_three_platforms` — Match a PC, PlayStation, and Xbox player into
   the same instance.
   - Input: 3 players each authenticated via a different platform IdP, all in the same queue with
     cross-play opt-in
   - Expected: matched into the same `InstanceId`; each player resolves the others' display names
     via the unified `PlatformAdapter`; cross-play opt-out filters by platform

6. **TC-8.6.I.1** `test_spectator_1k_relay` — Connect 1,000 spectators via fan-out relays to a live
   session.
   - Input: 1 game server, 4 spectator relays, 1,000 spectator clients
   - Expected: all 1,000 receive the replication stream with the configured 30-s delay; spectators
     cannot send gameplay RPCs; game server CPU unchanged within ±2% vs no spectators

7. **TC-8.9.I.1** `test_text_chat_50_member_p99` — Send 1,000 messages on a 50-member channel;
   measure p99 delivery latency.
   - Input: 50-member channel, 1,000 sequential messages, 20 ms RTT, 0% loss
   - Expected: every member receives every message; p99 send-to-receive < 100 ms; per-message
     ordering preserved per member

8. **TC-8.9.I.2** `test_voice_mouth_to_ear_latency` — Measure capture-to-output latency over
   loopback at 50 ms RTT.
   - Input: capture device feeds known impulse, decode through mixer, measure timestamp at listener
     output
   - Expected: total mouth-to-ear latency < 150 ms; jitter buffer expands during 30 ms induced
     jitter and contracts when stable

## Benchmarks

| ID           | Benchmark                            | Target      | Req        |
|--------------|--------------------------------------|-------------|------------|
| TC-8.5.B.1   | OAuth login throughput               | > 1k logins/s | R-8.5.1  |
| TC-8.5.B.2   | Matchmaker time-to-match (p95)       | < 60 s      | R-8.5.12   |
| TC-8.5.B.3   | Headless RSS at 64 players           | < 512 MB    | R-8.5.11   |
| TC-8.6.B.1   | Replay seek (2 h replay)             | < 1 s       | R-8.6.3    |
| TC-8.9.B.1   | Text message p99 (50-member channel) | < 100 ms    | R-8.9.4    |

1. **TC-8.5.B.1** — Sustained OAuth logins per second processed by `AuthService` over a 60-s window
   with mock IdP responding at p99 50 ms. Measured as completed authentications per second; failures
   excluded.

2. **TC-8.5.B.2** — Time-to-match measured at the 95th percentile across a 10,000-player synthetic
   queue with rating distribution `Normal(1500, 200)` and 4 regions. Measured by `criterion` with
   warmup excluded.

3. **TC-8.5.B.3** — Peak resident set size of the headless server hosting a 64-player session
   running physics, AI, networking, and game framework systems at 30 Hz. Measured over 10 minutes
   via `proc/self/status`.

4. **TC-8.6.B.1** — Wall time of `ReplayPlayer::seek(t)` on a 2-hour replay file (~3.6 GiB) stored
   on local NVMe. Includes keyframe load + delta replay to target tick.

5. **TC-8.9.B.1** — p99 delivery latency for text messages on a 50-member channel over the reliable
   ordered transport channel with 20 ms RTT and 0% loss. Measured over 1,000 messages.
