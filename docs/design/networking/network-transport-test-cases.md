# Network Transport — Test Cases

Companion to [network-transport.md](network-transport.md).

Test case IDs use `TC-8.1.Y.N`, `TC-8.2.Y.N`, `TC-8.3.Y.N`, and `TC-8.4.Y.N` formats. Every row
links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                                  | Req       |
|---------------|---------------------------------------|-----------|
| TC-8.1.1.1    | `test_handshake_authenticates`        | R-8.1.1   |
| TC-8.1.1.2    | `test_handshake_replay_rejected`      | R-8.1.1   |
| TC-8.1.2.1    | `test_connection_state_machine`       | R-8.1.2   |
| TC-8.1.2.2    | `test_keepalive_o1_overhead`          | R-8.1.2   |
| TC-8.1.3.1    | `test_reliable_ordered_in_order`      | R-8.1.3   |
| TC-8.1.3.2    | `test_reliable_ordered_sack`          | R-8.1.3   |
| TC-8.1.4.1    | `test_unreliable_unordered_no_retx`   | R-8.1.4   |
| TC-8.1.4.2    | `test_unreliable_sequenced_drop_old`  | R-8.1.4   |
| TC-8.1.5.1    | `test_dtls_encrypt_decrypt`           | R-8.1.5   |
| TC-8.1.5.2    | `test_dtls_key_rotation`              | R-8.1.5   |
| TC-8.1.6.1    | `test_fragment_reassemble_64k`        | R-8.1.6   |
| TC-8.1.6.2    | `test_pmtud_fallback_1200`            | R-8.1.6   |
| TC-8.1.7.1    | `test_congestion_converge`            | R-8.1.7   |
| TC-8.1.8.1    | `test_diagnostics_rtt_estimate`       | R-8.1.8   |
| TC-8.2.1.1    | `test_delta_only_changed_field`       | R-8.2.1   |
| TC-8.2.2.1    | `test_schema_version_negotiation`     | R-8.2.2   |
| TC-8.2.3.1    | `test_aoi_radius_filter`              | R-8.2.3   |
| TC-8.2.4.1    | `test_owner_only_property`            | R-8.2.4   |
| TC-8.2.5.1    | `test_priority_under_budget`          | R-8.2.5   |
| TC-8.2.6.1    | `test_dormant_zero_bandwidth`         | R-8.2.6   |
| TC-8.4.1.1    | `test_prediction_replay_inputs`       | R-8.4.1   |
| TC-8.4.2.1    | `test_input_buffer_redundancy`        | R-8.4.2   |
| TC-8.4.3.1    | `test_interpolation_smooth_motion`    | R-8.4.3   |
| TC-8.4.4.1    | `test_extrapolation_with_correction`  | R-8.4.4   |
| TC-8.4.5.1    | `test_lag_compensation_rewind`        | R-8.4.5   |
| TC-8.3.1.1    | `test_server_rpc_validation`          | R-8.3.1   |
| TC-8.3.2.1    | `test_client_rpc_one_shot`            | R-8.3.2   |
| TC-8.3.3.1    | `test_multicast_filtered`             | R-8.3.3   |
| TC-8.3.4.1    | `test_rpc_reliable_latest`            | R-8.3.4   |
| TC-8.3.5.1    | `test_rpc_param_validation_rejects`   | R-8.3.5   |

1. **TC-8.1.1.1** `test_handshake_authenticates` — Client presents a valid session token; server
   transitions the connection to `Authenticated`.
   - Input: `Handshake { token: SessionToken("valid"), client_nonce, version: 1 }`
   - Expected: server emits `ConnAuthenticated { player_id }`; connection state == `Authenticated`;
     ack returned within one tick

2. **TC-8.1.1.2** `test_handshake_replay_rejected` — Captured handshake replayed with the same nonce
   is rejected.
   - Input: handshake message replayed verbatim after a successful first handshake
   - Expected: server returns `Err(HandshakeError::ReplayDetected)`; the offending nonce logged in
     the anti-replay window

3. **TC-8.1.2.1** `test_connection_state_machine` — Connection transitions through
   `Connecting -> Authenticated -> Active -> Disconnected` and emits one event per transition.
   - Input: handshake, auth complete, idle, then disconnect
   - Expected: events `[ConnConnecting, ConnAuthenticated, ConnActive, ConnDisconnected]` in order;
     no duplicate transitions

4. **TC-8.1.2.2** `test_keepalive_o1_overhead` — Per-connection keepalive cost is constant in number
   of connections.
   - Input: tick 1k, 5k, and 10k idle connections; measure CPU time per tick
   - Expected: per-tick CPU time grows linearly in connection count (slope < 1.5x linear);
     per-connection time roughly constant (within ±10% across the 10x range)

5. **TC-8.1.3.1** `test_reliable_ordered_in_order` — Send 100 numbered messages over a lossy
   channel; assert in-order receipt with zero loss.
   - Input: 100 numbered messages, simulated 10% packet loss, 50 ms RTT
   - Expected: receiver ack'd messages 0..=99; receive sequence equal to send sequence; zero
     duplicates passed to application

6. **TC-8.1.3.2** `test_reliable_ordered_sack` — Selective ack covering received messages triggers
   retransmission of only the missing range.
   - Input: send messages 0..=9; messages 3 and 4 lost; receiver returns SACK `[0..=2, 5..=9]`
   - Expected: sender retransmits only messages 3 and 4; no other retransmissions until next RTO

7. **TC-8.1.4.1** `test_unreliable_unordered_no_retx` — Unreliable channel does not retransmit lost
   messages.
   - Input: 1,000 messages on `Channel::UnreliableUnordered`, 10% loss
   - Expected: ~900 messages delivered; no retransmissions observed; sender retx counter == 0

8. **TC-8.1.4.2** `test_unreliable_sequenced_drop_old` — Stale sequenced messages are dropped on
   arrival.
   - Input: send sequence numbers `[1, 2, 3]`, network reorders to `[1, 3, 2]`
   - Expected: receiver delivers `[1, 3]`; message 2 dropped as stale; drop counter == 1

9. **TC-8.1.5.1** `test_dtls_encrypt_decrypt` — Encrypt a payload and verify ciphertext differs from
   plaintext; decrypt produces the original.
   - Input: `payload = b"abilitycast(7)"`, DTLS 1.3 with AES-GCM
   - Expected: `ciphertext != payload`; `decrypt(ciphertext) == payload`; auth tag present and
     validates

10. **TC-8.1.5.2** `test_dtls_key_rotation` — Rotate keys mid-session; assert zero packet loss
    across the boundary.
    - Input: 1,000 messages spanning a key rotation event
    - Expected: receiver delivers all 1,000; rotation event observed in connection log; decryption
      uses the correct epoch on each side of the boundary

11. **TC-8.1.6.1** `test_fragment_reassemble_64k` — Send a 64 KiB payload over a link with 1,400
    byte MTU; verify reassembly.
    - Input: 65,536-byte payload, MTU 1,400
    - Expected: receiver receives identical 65,536-byte payload; reassembly buffer empty after
      delivery; fragment expiration timer cleared

12. **TC-8.1.6.2** `test_pmtud_fallback_1200` — PMTUD blocked by middlebox; sender falls back to
    1,200-byte default.
    - Input: PMTUD probes return no response within timeout
    - Expected: `effective_mtu(connection) == 1200`; subsequent sends respect 1,200-byte payload
      limit

13. **TC-8.1.7.1** `test_congestion_converge` — Bandwidth controller converges within 10% of
    available link bandwidth in 5 s.
    - Input: 1 Mbps link, 2% loss, 50 ms RTT, sender attempts unbounded send
    - Expected: send rate stabilizes between 0.9 and 1.1 Mbps within 5 s; no oscillation > 20% of
      average rate

14. **TC-8.1.8.1** `test_diagnostics_rtt_estimate` — Inject 100 ms latency; verify RTT diagnostic
    resource reports value within ±10%.
    - Input: simulated 100 ms link latency, send 100 round-trips
    - Expected: `Diagnostics { rtt_ms }` resource reads 90..=110; jitter, packet loss, and bandwidth
      utilization fields populated

15. **TC-8.2.1.1** `test_delta_only_changed_field` — Modify one field of a 20-field component; delta
    packet contains only that field.
    - Input: `Component20Fields`, mutate field 7 only
    - Expected: serialized delta contains exactly one field index (7) and one value; bytes-on- wire
      less than full-state serialization by at least 80%

16. **TC-8.2.2.1** `test_schema_version_negotiation` — Client schema version N connects to server
    version N+1 with one added optional field; data decodes.
    - Input: client schema rev N, server schema rev N+1 (added `field_x: Option<i32>`)
    - Expected: handshake succeeds with negotiated rev N; replicated component decodes correctly on
      the client; absent field defaults to `None`

17. **TC-8.2.3.1** `test_aoi_radius_filter` — Place 100 entities; client AOI radius 50 m;
    replication includes only entities within radius.
    - Input: entities at distances `[10, 20, ..., 1000]` m, client at origin, AOI 50 m
    - Expected: replicated set contains entities at 10, 20, 30, 40, 50 m only; entity count == 5

18. **TC-8.2.4.1** `test_owner_only_property` — Owner-only property is not sent to non-owners.
    - Input: entity with `Inventory` marked `Replicated::OwnerOnly`, two clients (owner + observer)
    - Expected: owner receives `Inventory`; observer's replicated component lacks the field

19. **TC-8.2.5.1** `test_priority_under_budget` — Bandwidth budget exceeded; high-priority entities
    replicate first.
    - Input: 500 entities (10 high priority, 490 low), budget 50 KBps with 200 KBps demand
    - Expected: all 10 high-priority entities updated every tick; low-priority subset rotated across
      ticks; total wire bytes/s <= 50 KBps

20. **TC-8.2.6.1** `test_dormant_zero_bandwidth` — Entity unchanged for the dormancy period consumes
    zero replication bandwidth.
    - Input: 1,000 entities, 700 unchanged for `dormancy_after = 5s`
    - Expected: dormant entities excluded from priority scheduling; bytes-on-wire for dormant set ==
      0; wake-up event re-enrolls a dormant entity within one tick of mutation

21. **TC-8.4.1.1** `test_prediction_replay_inputs` — Server correction triggers replay of
    unacknowledged inputs atop the new state.
    - Input: client predicts inputs at ticks 100..=110; server ack at tick 105 corrects position by
      0.5 m
    - Expected: client replays inputs 106..=110 atop the corrected state; final position matches
      server prediction within 0.01 m

22. **TC-8.4.2.1** `test_input_buffer_redundancy` — Redundant input copies recover from packet loss.
    - Input: input packets each carry the last 3 inputs; drop packet for tick 50; packet for tick 51
      carries inputs `[49, 50, 51]`
    - Expected: server processes input 50 from the redundant copy; no input gap; sequence remains
      intact

23. **TC-8.4.3.1** `test_interpolation_smooth_motion` — 144 FPS client rendering snapshots from a 20
    Hz server produces monotonic interpolated positions.
    - Input: snapshots at `t = 0, 50, 100 ms` with positions `[0, 5, 10]`; render frames at 144 Hz
    - Expected: rendered position is monotonic; values bounded by adjacent snapshots; no visual
      stutter (frame-to-frame position delta < 0.5 m)

24. **TC-8.4.4.1** `test_extrapolation_with_correction` — Snapshot delayed; client extrapolates;
    when snapshot arrives, error decays smoothly.
    - Input: last snapshot at `t = 100`, expected at `t = 150`, actually arrives at `t = 200`
    - Expected: extrapolated positions during `[150, 200]` follow last known velocity; on arrival,
      error correction converges to snapshot within 200 ms via exponential decay

25. **TC-8.4.5.1** `test_lag_compensation_rewind` — Server rewinds hitbox to attacker's observed
    time; hit registers within 250 ms compensation window.
    - Input: attacker RTT 100 ms, hit at attacker `t = 1000`, server now `t = 1100`
    - Expected: server rewinds defender hitbox by ~100 ms and registers the hit; same scenario with
      RTT 300 ms (exceeds 250 ms cap) does not rewind beyond 250 ms

26. **TC-8.3.1.1** `test_server_rpc_validation` — Out-of-range parameter rejected; valid call
    executes.
    - Input: RPC `CastAbility { ability_id: 99999 }` (invalid id), then `CastAbility { id: 7 }`
    - Expected: first returns `RpcError::Validation`; second executes; server does not crash on
      either; rate limit unaffected by valid call

27. **TC-8.3.3.1** `test_multicast_filtered` — Multicast RPC reaches only the filtered client set.
    - Input: 100 clients in zone, multicast filter `Spatial(center, radius=50m)`, 30 clients inside
      radius
    - Expected: 30 clients receive the RPC; 70 do not; payload sent once across the wire and fanned
      out by the server's multicast scheduler

28. **TC-8.3.4.1** `test_rpc_reliable_latest` — Ten rapid invocations of a `ReliableLatest` RPC;
    only the most recent is delivered.
    - Input: 10 invocations submitted within one tick on a `ReliableLatest` RPC
    - Expected: receiver observes exactly one invocation, equal to the 10th; intermediate
      invocations dropped server-side before transmission

29. **TC-8.3.2.1** `test_client_rpc_one_shot` — Server invokes a `DamageNumber` client RPC on one
    target client. Assert the target receives the RPC exactly once and no replicated component state
    is created.
    - Input: `rpc_client!(target, DamageNumber { amount: 42, pos: Vec3::ZERO })`
    - Expected: target receives 1 invocation; non-target clients receive 0; no `Component`
      registration touched; wire bytes equal RPC payload size

30. **TC-8.3.5.1** `test_rpc_param_validation_rejects` — Feed malformed and out-of-range RPC
    parameters through the validator; assert each is rejected with a typed error without crashing.
    - Input: `CastAbility { ability_id: i32::MAX }`, `Chat { text: "\0".repeat(1 << 20) }`,
      `TradeOffer { entity: EntityRef::INVALID }`
    - Expected: three `RpcError::Validation { reason }` returns, process stable, rate-limit counter
      incremented once per invalid call

## Integration Tests

| ID            | Name                                  | Req        |
|---------------|---------------------------------------|------------|
| TC-8.1.I.1    | `test_handshake_10k_concurrent`       | R-8.1.1    |
| TC-8.1.I.2    | `test_reliable_lossy_15pct_200ms`     | R-8.1.9    |
| TC-8.1.I.3    | `test_dtls_native_per_platform`       | R-8.1.11   |
| TC-8.2.I.1    | `test_replicate_10k_entities_100c`    | R-8.2.7    |
| TC-8.2.I.2    | `test_aoi_100k_entities`              | R-8.2.3    |
| TC-8.4.I.1    | `test_prediction_150ms_rtt`           | R-8.4.7    |
| TC-8.4.I.2    | `test_jitter_buffer_adapt`            | R-8.4.6    |
| TC-8.3.I.1    | `test_rpc_50k_per_second`             | R-8.3.6    |
| TC-8.1.I.4    | `us_player_secure_handshake`          | US-8.1.1   |
| TC-8.1.I.5    | `us_player_wifi_dropout_survives`     | US-8.1.2   |
| TC-8.1.I.6    | `us_eng_lifecycle_states_tracked`     | US-8.1.3   |
| TC-8.1.I.7    | `us_game_dev_reliable_ordered_sack`   | US-8.1.4   |
| TC-8.1.I.8    | `us_game_dev_channel_mode_matrix`     | US-8.1.5   |
| TC-8.1.I.9    | `us_eng_dtls_key_rotation_smooth`     | US-8.1.6   |
| TC-8.1.I.10   | `us_eng_fragment_reassemble_pmtud`    | US-8.1.7   |
| TC-8.1.I.11   | `us_eng_congestion_smoothing_curve`   | US-8.1.8   |
| TC-8.1.I.12   | `us_eng_diagnostics_ecs_resource`     | US-8.1.9   |
| TC-8.1.I.13   | `us_eng_congestion_mobile_to_dc`      | US-8.1.10  |
| TC-8.1.I.14   | `us_player_hud_ping_loss_indicator`   | US-8.1.12  |
| TC-8.2.I.3    | `us_eng_delta_xor_baseline`           | US-8.2.1   |
| TC-8.2.I.4    | `us_admin_schema_rolling_update`      | US-8.2.2   |
| TC-8.2.I.5    | `us_game_dev_aoi_filter_scales`       | US-8.2.3   |
| TC-8.2.I.6    | `us_game_dev_tiered_replication`      | US-8.2.4   |
| TC-8.2.I.7    | `us_player_500_guild_siege_priority` | US-8.2.5   |
| TC-8.2.I.8    | `us_player_party_raid_always_replic`  | US-8.2.6   |
| TC-8.2.I.9    | `us_eng_priority_scheduler_relevancy` | US-8.2.7   |
| TC-8.2.I.10   | `us_admin_dormancy_saves_bandwidth`   | US-8.2.8   |
| TC-8.2.I.11   | `us_player_mobile_500kbps_adapts`     | US-8.2.9   |
| TC-8.2.I.12   | `us_qa_replication_verifier`          | US-8.2.10  |
| TC-8.2.I.13   | `us_game_dev_grid_cell_config`        | US-8.2.12  |
| TC-8.3.I.2    | `us_game_dev_rpc_validation_rate`     | US-8.3.1   |
| TC-8.3.I.3    | `us_game_dev_client_rpc_events`       | US-8.3.2   |
| TC-8.3.I.4    | `us_game_dev_multicast_zone`          | US-8.3.3   |
| TC-8.3.I.5    | `us_player_world_boss_multicast`      | US-8.3.4   |
| TC-8.3.I.6    | `us_game_dev_rpc_reliability_modes`   | US-8.3.5   |
| TC-8.3.I.7    | `us_eng_rpc_binary_type_check`        | US-8.3.6   |
| TC-8.3.I.8    | `us_player_cosmetic_rpc_throttled`    | US-8.3.7   |
| TC-8.3.I.9    | `us_qa_rpc_fuzz_no_crash`             | US-8.3.9   |
| TC-8.4.I.3    | `us_player_instant_input_100ms_rtt`   | US-8.4.1   |
| TC-8.4.I.4    | `us_game_dev_replay_unacked_inputs`   | US-8.4.2   |
| TC-8.4.I.5    | `us_game_dev_input_buffer_auto_tune`  | US-8.4.3   |
| TC-8.4.I.6    | `us_player_smooth_interp_snapshots`   | US-8.4.4   |
| TC-8.4.I.7    | `us_player_extrap_error_decay`        | US-8.4.5   |
| TC-8.4.I.8    | `us_player_hitbox_rewind_fair`        | US-8.4.6   |
| TC-8.4.I.9    | `us_eng_rewind_bounded_250ms`         | US-8.4.7   |
| TC-8.4.I.10   | `us_eng_jitter_buffer_expand_contr`   | US-8.4.8   |
| TC-8.4.I.11   | `us_game_dev_mobile_tuning_defaults`  | US-8.4.9   |

1. **TC-8.1.I.1** `test_handshake_10k_concurrent` — Submit 10,000 concurrent handshakes; assert all
   complete within 2 s.
   - Input: 10,000 simulated clients each sending a valid handshake within 100 ms of each other
   - Expected: all 10,000 reach `Authenticated`; total wall time < 2 s; server CPU stays below
     saturation; no handshake errors

2. **TC-8.1.I.2** `test_reliable_lossy_15pct_200ms` — 100,000 numbered messages over a 15% loss, 200
   ms RTT link; assert 100% in-order delivery and retransmission latency within 1.5x RTT.
   - Input: 100,000 ordered messages on `ReliableOrdered`, simulated 15% loss, 200 ms RTT
   - Expected: receiver delivers all 100,000 in order; mean retransmission latency < 300 ms; zero
     gaps in receive sequence

3. **TC-8.1.I.3** `test_dtls_native_per_platform` — Establish a DTLS connection on each platform;
   verify the platform-native backend is selected.
   - Input: same client/server pair built for Windows, macOS, Linux
   - Expected: Windows uses Schannel; macOS uses Security.framework; Linux uses rustls; each
     successfully establishes and exchanges encrypted traffic; cfg gating excludes other backends at
     compile time

4. **TC-8.2.I.1** `test_replicate_10k_entities_100c` — Replicate 10,000 entities to 100 clients with
   5% per-tick change rate; assert > 100,000 updates/s.
   - Input: 10,000 entities, 100 clients, 5% mutation rate per tick at 30 Hz
   - Expected: throughput > 100,000 entity updates/s sustained over 60 s; bandwidth at least 80%
     lower than full-state baseline

5. **TC-8.2.I.2** `test_aoi_100k_entities` — 100,000 entities in a zone; 1,000 clients each with AOI
   100 m; assert AOI evaluation within 2 ms per tick.
   - Input: 100,000 entities uniformly distributed in 1 km^2 zone, 1,000 clients placed randomly
   - Expected: per-tick AOI evaluation < 2 ms; each client receives only entities within 100 m;
     party-member exception correctly forces inclusion of out-of-range party members

6. **TC-8.4.I.1** `test_prediction_150ms_rtt` — Simulate 150 ms RTT; local input responds within 1
   frame; corrections converge smoothly.
   - Input: client at 60 fps with 150 ms RTT, mutate input at frame N
   - Expected: rendered frame N+1 reflects the input; injected 30 ms server correction shows no
     visible snap; injected 100 ms correction converges in < 200 ms via smoothing

7. **TC-8.4.I.2** `test_jitter_buffer_adapt` — Variable jitter between 0 and 50 ms; jitter buffer
   expands and contracts.
   - Input: introduce 0..=50 ms uniform jitter for 30 s, then return to 0
   - Expected: buffer depth grows during the jittery window; contracts within 5 s after jitter
     subsides; added latency never exceeds 2x measured jitter standard deviation; voice traffic
     bypasses this buffer

8. **TC-8.3.I.1** `test_rpc_50k_per_second` — Submit 50,000 5-parameter RPCs per second; assert
   networking frame budget < 0.5 ms.
   - Input: 50,000 RPCs/s with `(i32, f32, String, EntityRef, Enum)` payloads
   - Expected: all 50,000 validated and executed per second; networking system time < 0.5 ms per
     tick at 60 Hz; no validation false negatives

9. **TC-8.1.I.4** `us_player_secure_handshake` — As a player, connect to the server and observe that
   the handshake both authenticates my account token and encrypts all subsequent traffic with DTLS
   1.3.
   - Input: valid session token, packet capture on the client NIC
   - Expected: connection reaches `Authenticated`; captured packets after handshake are encrypted
     (no plaintext payload matches known sentinel)

10. **TC-8.1.I.5** `us_player_wifi_dropout_survives` — As a player, drop and restore Wi-Fi for 4 s
    during active gameplay. Assert the connection stays alive via heartbeat keep-alive and in-flight
    messages retransmit without disconnect.
    - Input: tun/tap link up for 10 s, down for 4 s, up for 10 s; heartbeat interval 2 s
    - Expected: `ConnState == Active` throughout; zero `Disconnected` events; pending reliable
      messages delivered after link restoration

11. **TC-8.1.I.6** `us_eng_lifecycle_states_tracked` — As an engine developer, drive a connection
    through all lifecycle states. Assert each transition fires exactly one event with configurable
    timeouts honored.
    - Input: `timeouts = { connect: 5s, auth: 2s, idle: 30s }`, scripted state transitions
    - Expected: event stream `[Connecting, Authenticated, Active, Migrating, Disconnected]` emitted
      once each; state survives configurable timeouts without premature drop

12. **TC-8.1.I.7** `us_game_dev_reliable_ordered_sack` — As a game developer, send 1,000
    inventory-update messages over a 5% loss, 150 ms RTT link via `ReliableOrdered`. Assert all
    1,000 arrive in order using SACK-driven retransmission.
    - Input: 1,000 ordered inventory messages, 5% loss, 150 ms RTT
    - Expected: receiver delivers all 1,000 in order; SACK acks observed; no duplicate app
      deliveries

13. **TC-8.1.I.8** `us_game_dev_channel_mode_matrix` — As a game developer, open one channel of each
    mode (`UnreliableUnordered`, `UnreliableSequenced`, `ReliableUnordered`, `ReliableOrdered`) and
    verify each delivers its documented semantics under 10% loss.
    - Input: four channels, 500 messages each, 10% loss
    - Expected: unreliable modes drop ~50 messages and never retransmit; reliable modes deliver
      500/500; sequenced drops stale; ordered preserves order

14. **TC-8.1.I.9** `us_eng_dtls_key_rotation_smooth` — As an engine developer, rotate DTLS keys
    mid-session while streaming 10 Mbps. Assert zero packet loss across the rotation.
    - Input: 10 Mbps stream, trigger rotation at 5 s mark
    - Expected: 0 packets lost at the rotation boundary; both epochs observed in the capture;
      session throughput unchanged within 1%

15. **TC-8.1.I.10** `us_eng_fragment_reassemble_pmtud` — As an engine developer, send a 60 KiB
    zone-transition snapshot across a link that blocks PMTUD probes. Assert the sender falls back to
    1,200 bytes and the receiver reassembles identical bytes.
    - Input: 60 KiB payload, middlebox blocking ICMP
    - Expected: `effective_mtu == 1200`; reassembled payload hash equals source hash

16. **TC-8.1.I.11** `us_eng_congestion_smoothing_curve` — As an engine developer, ramp a bottleneck
    from 500 Kbps to 10 Gbps over 30 s. Assert the congestion controller tracks available BW
    smoothly without bursty oscillation.
    - Input: tc-netem shaped link, 500 Kbps → 10 Gbps ramp
    - Expected: send rate tracks capacity within 15%; peak-to-mean burstiness < 1.2; no starvation
      event

17. **TC-8.1.I.12** `us_eng_diagnostics_ecs_resource` — As an engine developer, read the
    `Diagnostics` ECS resource each frame. Assert RTT, loss, jitter, and BW utilization are updated
    at tick rate and match injected conditions.
    - Input: injected `rtt = 75 ms`, `loss = 3%`, `jitter = 10 ms`
    - Expected: resource values within ±10% of injected for 60 consecutive ticks

18. **TC-8.1.I.13** `us_eng_congestion_mobile_to_dc` — As an engine developer, run the same
    congestion controller on a 500 Kbps cellular link and a 10 Gbps datacenter link. Assert both
    reach steady state within 5 s and within 15% of link capacity.
    - Input: two simulated links, same controller instance
    - Expected: both rates stable; no controller parameter tuning between runs

19. **TC-8.1.I.14** `us_player_hud_ping_loss_indicator` — As a player, watch a HUD bound to the
    `Diagnostics` resource. Change network conditions and assert the HUD updates ping, loss, and
    stability within 250 ms.
    - Input: toggled latency conditions, HUD polling at frame rate
    - Expected: HUD text reflects new values within 250 ms of change

20. **TC-8.2.I.3** `us_eng_delta_xor_baseline` — As an engine developer, replicate 2,000 entities
    with per-client baselines and XOR differencing. Assert only changed fields appear on the wire
    and the baseline converges to the latest acknowledged state.
    - Input: 2,000 entities, 2% per-tick mutation
    - Expected: ≥ 95% bandwidth reduction vs full-state; post-tick baselines equal delivered state
      at the client

21. **TC-8.2.I.4** `us_admin_schema_rolling_update` — As a server administrator, roll a server to
    schema rev N+1 while clients stay on rev N. Assert all existing connections continue replicating
    without disconnect.
    - Input: mixed client build (rev N) and server build (rev N+1 with one added optional field)
    - Expected: 0 disconnects; replicated components decode on clients; added field reads as default
      on rev-N clients

22. **TC-8.2.I.5** `us_game_dev_aoi_filter_scales` — As a game developer, place 200k entities in a
    zone and connect 50 clients. Assert per-client replication sets stay within AOI and total
    bandwidth stays within each client's budget.
    - Input: 200k entities, 50 clients, 64 KBps budget each
    - Expected: no client exceeds budget; replicated sets match AOI membership exactly

23. **TC-8.2.I.6** `us_game_dev_tiered_replication` — As a game developer, tag a `Cooldown` property
    `TeamOnly` and a `Position` property with distance tiers. Assert opponents never see `Cooldown`
    and distant `Position` uses coarser quantization.
    - Input: 2 teams × 10 players, tagged properties
    - Expected: opponents' `Cooldown` stream count == 0; distant position bytes < nearby bytes per
      update

24. **TC-8.2.I.7** `us_player_500_guild_siege_priority` — As a player in a 500-player siege, verify
    nearby and hostile entities continue to replicate every tick while distant NPCs fall back to
    every N ticks.
    - Input: 500-entity siege, mixed hostile/friendly
    - Expected: nearby hostile update latency ≤ 1 tick; distant NPC latency ≤ 4 ticks; budget
      respected

25. **TC-8.2.I.8** `us_player_party_raid_always_replic` — As a player in a party and raid, verify
    party members, guild members in zone, and the raid boss always replicate regardless of distance.
    - Input: 4-member party, 40-member raid, 1 raid boss, members scattered across zone
    - Expected: party + raid + boss are in every replication set every tick

26. **TC-8.2.I.9** `us_eng_priority_scheduler_relevancy` — As an engine developer, verify the
    scheduler prioritizes by `(relevancy × staleness × type_weight)` and fills remaining budget with
    low-priority entities.
    - Input: 300 entities with varied relevancy and staleness scores
    - Expected: sort order matches computed priority; low-priority tail only transmitted when budget
      remains

27. **TC-8.2.I.10** `us_admin_dormancy_saves_bandwidth` — As an administrator, measure bandwidth in
    a zone where 70% of entities are idle beyond the dormancy interval. Assert the dormant subset
    consumes zero bandwidth.
    - Input: 10k entities, 7k idle
    - Expected: dormant bytes-on-wire == 0; wake-on-mutation re-enrolls within one tick

28. **TC-8.2.I.11** `us_player_mobile_500kbps_adapts` — As a mobile player on a 500 Kbps link,
    verify replication adapts precision and frequency so gameplay remains playable within budget.
    - Input: client at 500 Kbps, full-world zone
    - Expected: total down traffic ≤ 500 Kbps; visible desyncs < 2% of frames

29. **TC-8.2.I.12** `us_qa_replication_verifier` — As a QA tester, run a scripted scenario with
    varying update rates and verify final client state equals server state for every entity.
    - Input: 5k entities, 2 minutes, randomized mutations
    - Expected: per-client final state equals server state; zero divergence

30. **TC-8.2.I.13** `us_game_dev_grid_cell_config` — As a game developer, configure the grid cell
    size and per-tier update frequencies in project settings. Assert the replication loop honors the
    config without code changes.
    - Input: `grid.cell = 64m`, tier frequencies `[30, 10, 2]`
    - Expected: grid lookup cell width == 64m; tier Hz matches config; no hardcoded values override
      user settings

31. **TC-8.3.I.2** `us_game_dev_rpc_validation_rate` — As a game developer, fire a client RPC at 10×
    its rate limit. Assert the first N execute and subsequent calls are rejected with
    `RpcError::RateLimit`.
    - Input: `CastAbility` rate limit 5/s, 50 invocations in 1 s
    - Expected: 5 executed, 45 rejected with `RateLimit`, server stable

32. **TC-8.3.I.3** `us_game_dev_client_rpc_events` — As a game developer, fire a server-to-client
    `DamageNumber` RPC. Assert the client receives it without creating a persistent replicated
    component.
    - Input: `rpc_client!(target, DamageNumber { amount: 42 })`
    - Expected: target receives RPC; `Component` table unchanged; wire traffic == RPC payload

33. **TC-8.3.I.4** `us_game_dev_multicast_zone` — As a game developer, multicast a `PhaseTransition`
    RPC to all clients inside a 100 m radius with one call. Assert it reaches only clients in the
    radius and is sent once on the wire.
    - Input: 200 clients, 80 inside radius
    - Expected: 80 clients receive it, 120 do not, sender transmits once; fan-out on server side

34. **TC-8.3.I.5** `us_player_world_boss_multicast` — As a player, subscribe to a world boss zone
    and verify phase-transition multicast RPCs arrive reliably even under packet loss.
    - Input: 50 clients in zone, 10% loss, 5 phase transitions
    - Expected: every subscribed client receives all 5 transitions; zero missed events

35. **TC-8.3.I.6** `us_game_dev_rpc_reliability_modes` — As a game developer, declare three RPCs
    with `Reliable`, `Unreliable`, and `ReliableLatest`. Assert each delivery semantic under 10%
    loss.
    - Input: 100 invocations per RPC, 10% loss
    - Expected: `Reliable` 100/100 delivered; `Unreliable` ≈ 90/100; `ReliableLatest` exactly 1
      (latest) delivered per burst

36. **TC-8.3.I.7** `us_eng_rpc_binary_type_check` — As an engine developer, serialize an RPC with a
    mismatched parameter type. Assert the server rejects it with `RpcError::TypeMismatch` and
    reports the offending parameter index.
    - Input: RPC expecting `(i32, f32)` invoked with `(String, bool)` bytes
    - Expected: `Err(RpcError::TypeMismatch { index: 0, expected: "i32", found: "string" })`

37. **TC-8.3.I.8** `us_player_cosmetic_rpc_throttled` — As a mobile player, verify cosmetic client
    RPCs are throttled under bandwidth pressure while gameplay-critical RPCs still arrive.
    - Input: 500 Kbps link saturated, mix of gameplay + cosmetic RPCs
    - Expected: gameplay RPC delivery 100%; cosmetic RPC delivery reduced; throttle counter > 0

38. **TC-8.3.I.9** `us_qa_rpc_fuzz_no_crash` — As a QA tester, fuzz 100,000 random RPC payloads at
    the server. Assert every invalid payload is rejected and the server does not crash, hang, or
    leak memory (RSS stable ±1%).
    - Input: 100k randomized byte streams addressed to RPC ids
    - Expected: all invalid rejected with typed errors; RSS within ±1%; zero panics

39. **TC-8.4.I.3** `us_player_instant_input_100ms_rtt` — As a player at 100 ms RTT, press a movement
    input and verify the rendered character moves on the next frame.
    - Input: 60 fps client, 100 ms RTT, single WASD press
    - Expected: frame N+1 position changes by prediction; server reconciliation does not snap
      visibly

40. **TC-8.4.I.4** `us_game_dev_replay_unacked_inputs` — As a game developer, verify a server
    correction replays all unacknowledged inputs over the corrected state.
    - Input: 8 unacknowledged inputs, correction at tick -4 relative to client
    - Expected: client replays inputs 5..=8 on top of correction; final state matches server

41. **TC-8.4.I.5** `us_game_dev_input_buffer_auto_tune` — As a game developer, measure the input
    buffer depth auto-tuning as RTT and jitter vary.
    - Input: RTT 20 ms then 200 ms, jitter 0 ms then 50 ms
    - Expected: buffer depth doubles when RTT/jitter rise, halves when they fall; latency never
      exceeds 2× jitter σ

42. **TC-8.4.I.6** `us_player_smooth_interp_snapshots` — As a player, verify remote entities
    interpolate smoothly between 20 Hz snapshots at 144 fps render rate.
    - Input: 20 Hz snapshot stream, 144 Hz render
    - Expected: rendered motion is monotonic; frame-to-frame delta < 0.5 m for a 10 m/s entity

43. **TC-8.4.I.7** `us_player_extrap_error_decay` — As a player, when a snapshot is late, verify the
    entity extrapolates via last-known velocity and the error decays smoothly on arrival.
    - Input: snapshot missed at `t = 100 ms`, arrives at `t = 150 ms`
    - Expected: extrapolation error ≤ `velocity × delay`; decay to zero within 200 ms exponential

44. **TC-8.4.I.8** `us_player_hitbox_rewind_fair` — As a player, assert that a hit that looked
    correct on my screen registers on the server via hitbox rewind within the 250 ms cap.
    - Input: attacker RTT 120 ms, defender in motion
    - Expected: hit registers; rewind amount == 120 ms; defender position at rewound time intersects
      hitbox

45. **TC-8.4.I.9** `us_eng_rewind_bounded_250ms` — As an engine developer, verify rewind is bounded
    at 250 ms regardless of RTT and favors the defender on ties.
    - Input: attacker RTT 300 ms (above cap), borderline ambiguous hit
    - Expected: rewind capped at 250 ms; ambiguous hit resolved in defender's favor

46. **TC-8.4.I.10** `us_eng_jitter_buffer_expand_contr` — As an engine developer, verify the jitter
    buffer expands when instability rises and contracts when it falls.
    - Input: 30 s of variable jitter, 30 s stable
    - Expected: buffer depth increases on instability; returns to baseline within 5 s of recovery;
      added latency ≤ 2× jitter σ

47. **TC-8.4.I.11** `us_game_dev_mobile_tuning_defaults` — As a game developer, apply the mobile
    defaults preset. Assert deeper input buffer, wider interpolation window, and longer
    extrapolation are active.
    - Input: `apply_defaults(PlatformProfile::Mobile)`
    - Expected: `input_buffer_depth >= 8`, `interp_window_ms >= 100`, `extrap_window_ms >= 250`

## Benchmarks

| ID           | Benchmark                                | Target     | Req        |
|--------------|------------------------------------------|------------|------------|
| TC-8.1.B.1   | DTLS AES-GCM throughput (HW accel)       | > 1 Gbps   | R-8.1.10   |
| TC-8.1.B.2   | Reliable ordered round-trip (50 ms RTT)  | < 75 ms p99| R-8.1.9    |
| TC-8.2.B.1   | Replication updates/s (10k entities)     | > 100k/s   | R-8.2.7    |
| TC-8.4.B.1   | Prediction reconciliation latency        | < 16.7 ms  | R-8.4.7    |
| TC-8.3.B.1   | RPC validation throughput                | > 50k/s    | R-8.3.6    |

1. **TC-8.1.B.1** — DTLS 1.3 AES-GCM throughput on hardware with AES-NI. Encrypts a contiguous 1 GiB
   stream and measures sustained Gbps. Software fallback variant measured separately and must hit at
   least 100 Mbps.

2. **TC-8.1.B.2** — End-to-end round-trip latency for a single reliable-ordered message over a
   loopback link with 50 ms simulated RTT and 1% loss. Reports p50 and p99 latency. Target: p99
   below 75 ms.

3. **TC-8.2.B.1** — Replication throughput over 10,000 entities with 5% per-tick mutation replicated
   to 100 simulated clients at 30 Hz. Reports total entity updates per second and average bytes per
   update.

4. **TC-8.4.B.1** — Wall time from server-correction receipt to fully reconciled client state after
   replaying unacknowledged inputs. Target one render frame at 60 fps (16.7 ms) over a buffer of 10
   unacknowledged inputs.

5. **TC-8.3.B.1** — RPC validation throughput measured by submitting 5-parameter RPCs through the
   validator pipeline (type check, range clamp, reference resolve, rate limit). Reports validations
   per second on a single core.
