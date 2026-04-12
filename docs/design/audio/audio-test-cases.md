# Audio System — Test Cases

Companion to [audio.md](audio.md).

Test case IDs use `TC-5.G.Z.N` format where G is the group (1=engine, 2=spatial, 3=DSP, 4=music,
5=voice). Every row links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                                | Req         |
|---------------|-------------------------------------|-------------|
| TC-5.1.1.1    | `test_audio_source_size_budget`     | R-5.1.1     |
| TC-5.1.1.2    | `test_emitter_shape_runtime_swap`   | R-5.1.1a    |
| TC-5.1.2.1    | `test_listener_camera_fallback`     | R-5.1.2     |
| TC-5.1.3.1    | `test_bus_gain_inheritance`         | R-5.1.3     |
| TC-5.1.3.2    | `test_bus_enum_dispatch`            | R-5.1.3a    |
| TC-5.1.4.1    | `test_voice_priority_steal`         | R-5.1.4     |
| TC-5.1.4.2    | `test_voice_pool_tier_size`         | R-5.1.4a    |
| TC-5.1.5.1    | `test_stream_ring_budget`           | R-5.1.5     |
| TC-5.1.6.1    | `test_command_phase_alignment`      | R-5.1.6     |
| TC-5.1.7.1    | `test_codec_metadata_extract`       | R-5.1.7     |
| TC-5.2.1.1    | `test_doppler_pitch_ratio`          | R-5.2.1     |
| TC-5.2.2.1    | `test_attenuation_curves`           | R-5.2.2     |
| TC-5.2.3.1    | `test_hrtf_azimuth_response`        | R-5.2.3     |
| TC-5.2.4.1    | `test_ambisonics_encode_w_x_y_z`    | R-5.2.4     |
| TC-5.2.5.1    | `test_occlusion_wood_wall_loss`     | R-5.2.5     |
| TC-5.2.7.1    | `test_reverb_zone_blend`            | R-5.2.7     |
| TC-5.3.1.1    | `test_biquad_lowpass_response`      | R-5.3.1     |
| TC-5.3.2.1    | `test_param_eq_8_band_gain`         | R-5.3.2     |
| TC-5.3.3.1    | `test_fdn_reverb_decay_curve`       | R-5.3.3     |
| TC-5.3.5.1    | `test_compressor_4to1_ratio`        | R-5.3.5     |
| TC-5.3.5.2    | `test_master_limiter_clamp`         | R-5.3.5     |
| TC-5.3.7.1    | `test_pitch_shift_octave_up`        | R-5.3.7     |
| TC-5.3.8.1    | `test_custom_dsp_node_register`     | R-5.3.8     |
| TC-5.4.1.1    | `test_stem_sample_alignment`        | R-5.4.1     |
| TC-5.4.2.1    | `test_segment_graph_bar_quantize`   | R-5.4.2     |
| TC-5.4.4.1    | `test_beat_clock_120_bpm_count`     | R-5.4.4     |
| TC-5.4.5.1    | `test_stinger_cooldown_suppress`    | R-5.4.5     |
| TC-5.4.6.1    | `test_playlist_no_immediate_repeat` | R-5.4.6     |
| TC-5.4.7.1    | `test_intensity_clamp_and_drive`    | R-5.4.7     |
| TC-5.5.2.1    | `test_jitter_buffer_depth_cap`      | R-5.5.2     |
| TC-5.5.3.1    | `test_vad_speech_onset_20ms`        | R-5.5.3     |
| TC-5.5.6.1    | `test_dialogue_priority_interrupt`  | R-5.5.6     |
| TC-5.5.7.1    | `test_dialogue_branch_gated_edge`   | R-5.5.7     |
| TC-5.3.6.1    | `test_delay_chorus_flanger_variants`| R-5.3.6     |
| TC-5.4.3.1    | `test_music_transition_edge_modes`  | R-5.4.3     |
| TC-5.5.1.1    | `test_mic_capture_opus_encode`      | R-5.5.1     |
| TC-5.5.4.1    | `test_tts_synthesis_routes_to_bus`  | R-5.5.4     |
| TC-5.5.5.1    | `test_phoneme_viseme_mapping`       | R-5.5.5     |
| TC-5.5.8.1    | `test_voice_channel_routing`        | R-5.5.8     |

1. **TC-5.1.1.1** `test_audio_source_size_budget` — Allocate `AudioSource` with each emitter shape;
   assert `size_of::<AudioSource>() <= 128` bytes for each variant.
   - Input: `AudioSource { shape, gain, pitch, looping, attenuation }` for each
     `EmitterShape ∈ { Point, Line, Area }`
   - Expected: `size_of_val(&source) <= 128` for all three; component layout is `#[repr(C)]`

2. **TC-5.1.1.2** `test_emitter_shape_runtime_swap` — Create active source with `Point`; swap to
   `Line { length: 4.0 }` mid-playback. Assert `VoiceState::Playing` retained, no allocation, no
   discontinuity event emitted.
   - Input: source playing on voice 0, command
     `AudioCommand::SetEmitterShape(VoiceId(0), EmitterShape::Line { length: 4.0 })`
   - Expected: `voice.state == VoiceState::Playing`, zero `Discontinuity` events, allocator counter
     unchanged

3. **TC-5.1.2.1** `test_listener_camera_fallback` — World contains a camera entity but no
   `AudioListener`. Run `audio_sync_system`. Assert spatializer uses the camera entity's transform
   as the active listener.
   - Input: world with one `Transform` + `Camera` entity, zero `AudioListener` entities
   - Expected: `Spatializer::active_listener_id() == camera_entity`, no panic

4. **TC-5.1.3.1** `test_bus_gain_inheritance` — Master bus gain 0.5; child SFX bus gain 1.0; two
   leaves at 1.0. Process unit-DC signal through each leaf.
   - Input: bus tree `Master(0.5) -> SFX(1.0) -> { Footsteps(1.0), Impacts(1.0) }`, DC input 1.0
   - Expected: `mix_output[footsteps] == 0.5`, `mix_output[impacts] == 0.5` within 1e-6

5. **TC-5.1.3.2** `test_bus_enum_dispatch` — Build a 4-insert chain of `AudioEffect` variants.
   Assert process loop performs no `Box<dyn>` allocation and the disassembly contains no
   `callq *(%rax)` indirect call (mocked via `std::any::TypeId` count check at runtime).
   - Input: `EffectChain::with_inserts([Biquad, ParamEq, Compressor, Delay])`
   - Expected: heap allocations during 1024-sample process == 0; static dispatch verified by
     `TypeId` enumeration of the enum variants

6. **TC-5.1.4.1** `test_voice_priority_steal` — Voice pool size 4, all 4 occupied by `Low` priority.
   Request a `Critical` voice. Assert lowest-audibility low-priority voice transitions to
   `VoiceState::Virtualized` and the critical voice receives the slot.
   - Input: 4 voices with `AudibilityScore(0.1..0.4)` and `VoicePriority::Low`; new request with
     `VoicePriority::Critical, AudibilityScore(0.9)`
   - Expected: voice with `AudibilityScore(0.1)` becomes `Virtualized`; critical voice has
     `state == Playing`

7. **TC-5.1.4.2** `test_voice_pool_tier_size` — Initialize `VoiceManager` with
   `AudioConfig { tier: PlatformTier::Mobile }`. Assert pool capacity in `[16, 32]`. Repeat for
   `Switch` `[32, 64]` and `Desktop` `[128, 256]`.
   - Input: three configs varying only `tier`
   - Expected: `voice_manager.capacity() in 16..=32`, `32..=64`, `128..=256` respectively

8. **TC-5.1.5.1** `test_stream_ring_budget` — Stream a 5-minute Vorbis file via `StreamManager`;
   sample peak resident bytes per stream every 100 ms.
   - Input: `stream_manager.open("long.ogg", PrefetchHint::None)`; sample for 60 s
   - Expected: `max(peak) <= 256 * 1024`; stream completes without underrun

9. **TC-5.1.6.1** `test_command_phase_alignment` — Schedule two `PlaySound` commands at the same
   `AudioTimestamp::SampleOffset(128)`. Process buffer; cross-correlate the two output voices.
   - Input: two identical clips, both with `start_offset_samples = 128`
   - Expected: cross-correlation peak at lag 0 with deviation == 0 samples

10. **TC-5.1.7.1** `test_codec_metadata_extract` — Load one PCM, Vorbis, Opus, and FLAC sample.
    Assert `AudioClipMeta` returns the expected sample rate, channel count, and loop points cached
    from the file header.
    - Input: 4 reference assets with known metadata
    - Expected: each `clip_meta.sample_rate`, `channels`, `loop_start`, `loop_end` matches reference

11. **TC-5.2.1.1** `test_doppler_pitch_ratio` — Source moving toward listener at 34 m/s (~0.1c_sound
    at 343 m/s). Assert reported pitch ratio is `1 / (1 - 0.1) ≈ 1.111` within 1%.
    - Input:
      `SpatialParams { source_velocity: Vec3::new(0.0, 0.0, -34.0), listener_velocity: Vec3::ZERO }`
    - Expected: `Spatializer::doppler_ratio()` in `[1.100, 1.122]`

12. **TC-5.2.2.1** `test_attenuation_curves` — For each `AttenuationModel` (Linear, Inverse,
    InverseSquare, Logarithmic), evaluate at distances `[min, 1.5*min, 2*min, ..., max]`. Compare
    against analytic formula.
    - Input: `min_distance = 1.0`, `max_distance = 100.0`, 10 sample points
    - Expected: per-point gain matches formula within `0.001` (linear delta)

13. **TC-5.2.3.1** `test_hrtf_azimuth_response` — Render a mono click through `HrtfRenderer` at
    azimuths `0°, 90°, 180°, 270°`. Compare ITD and ILD against the SOFA reference dataset.
    - Input: SOFA dataset `mit_kemar_normal.sofa`, click at each azimuth
    - Expected: ITD within `0.1 ms`, ILD within `1 dB` of dataset values per ear

14. **TC-5.2.4.1** `test_ambisonics_encode_w_x_y_z` — Encode unit DC at azimuth 90°, elevation 0°
    into first-order Ambisonics. Verify W, X, Y, Z spherical-harmonic coefficients.
    - Input: mono signal level 1.0, az 90°, el 0°
    - Expected: `(W, X, Y, Z) ≈ (0.7071, 0.0, 1.0, 0.0)` within 0.001

15. **TC-5.2.5.1** `test_occlusion_wood_wall_loss` — Source behind a wood wall registered with
    `AcousticMaterial::Wood { transmission_db: -12.0 }`. Cast against shared BVH.
    - Input: wall AABB between source and listener; material loss 12 dB
    - Expected: `OcclusionSystem::compute_loss()` returns `-12.0 dB ± 1 dB`; low-pass cutoff applied

16. **TC-5.2.7.1** `test_reverb_zone_blend` — Two `ReverbZone` AABBs overlapping by 1 m. Decays 1.0
    s and 3.0 s. Step listener through the overlap in 0.1 m increments.
    - Input: zones at `[0..2]` and `[1..3]`, listener at `x = 0.0..3.0`
    - Expected: reverb decay parameter linearly interpolates `1.0 -> 3.0` across the overlap; no
      discontinuity > 0.05 s between adjacent samples

17. **TC-5.3.1.1** `test_biquad_lowpass_response` — Process white noise through
    `AudioEffect::Biquad(BiquadKind::LowPass { cutoff: 1000.0, q: 0.707 })`. FFT output and verify
    -3 dB at 1 kHz, -12 dB/oct rolloff above.
    - Input: 65536-sample white noise, sample rate 48000
    - Expected: magnitude at 1 kHz = `-3 dB ± 0.5 dB`; magnitude at 2 kHz = `-15 dB ± 1 dB`

18. **TC-5.3.2.1** `test_param_eq_8_band_gain` — Configure 8 peak bands at 100 Hz, 200 Hz,... 12.8
    kHz, each at +6 dB, Q=1.0. Process flat-spectrum signal.
    - Input: pink-equivalent flat input, eight `EqBand` entries
    - Expected: each band peak gain = `+6 dB ± 0.5 dB` at its center frequency

19. **TC-5.3.3.1** `test_fdn_reverb_decay_curve` — Process a unit impulse through `FdnReverb` with
    decay 2.0 s. Measure RMS energy in 100 ms windows; fit to exponential.
    - Input: impulse 1.0 at sample 0, `decay_time = 2.0`, sample rate 48000
    - Expected: time for RMS to drop 60 dB = `2.0 s ± 10%`

20. **TC-5.3.5.1** `test_compressor_4to1_ratio` — Feed a 1 kHz sine 12 dB above threshold into a
    compressor with `ratio = 4.0, threshold = -20.0 dBFS, attack = 5 ms, release = 50 ms`.
    - Input: sine `-8 dBFS`, threshold `-20 dBFS`
    - Expected: steady-state output level = `-17 dBFS ± 0.5 dB` (12 dB / 4 = 3 dB over threshold)

21. **TC-5.3.5.2** `test_master_limiter_clamp` — Inject signal +6 dB above 0 dBFS into the master
    bus look-ahead limiter. Sample peak across 1 s of output.
    - Input: 1 kHz sine `+6 dBFS` for 1 s
    - Expected: `max(abs(output)) <= 1.0` (0 dBFS) for all samples; no overshoot

22. **TC-5.3.7.1** `test_pitch_shift_octave_up` — Shift 440 Hz sine up 12 semitones via
    `PitchShifter` (phase-vocoder on desktop). Measure peak frequency via FFT.
    - Input: 440 Hz sine, 1 s, `semitones = 12.0`
    - Expected: peak frequency = `880 Hz ± 1%`; output duration matches input within 1 ms; artifacts
      below `-40 dB`

23. **TC-5.3.8.1** `test_custom_dsp_node_register` — Register a `+6 dB` gain node via
    `DspNodeRegistry::register("test_gain", ...)`, insert into chain, process unit DC.
    - Input: registered factory closure, chain `[CustomNode("test_gain")]`, DC input 0.5
    - Expected: output == `1.0 ± 0.001`; node looked up by name

24. **TC-5.4.1.1** `test_stem_sample_alignment` — Play a 4-stem `MusicCueComponent`. After 60 s of
    playback, cross-correlate each stem against stem 0.
    - Input: `MusicCueComponent { stems: [stem0, stem1, stem2, stem3] }`
    - Expected: cross-correlation peak at lag 0 for all pairs; `peak_offset == 0` samples

25. **TC-5.4.2.1** `test_segment_graph_bar_quantize` — Trigger `SegmentGraph::transition_to(B)` at
    offset `0.5 * bar_period` into segment A (4/4 at 120 BPM, bar = 2.0 s).
    - Input: trigger at `t = 1.0 s` into bar
    - Expected: actual switch occurs at `t = 2.0 s ± 1 sample`

26. **TC-5.4.4.1** `test_beat_clock_120_bpm_count` — Run `BeatClock` at 120 BPM 4/4 for 10 s.
    Capture all `BeatEvent` and `BarEvent` entries.
    - Input: `BeatClock::start(120.0, TimeSig { num: 4, den: 4 })`, run for 10 s
    - Expected: 20 `BeatEvent` and 5 `BarEvent` entries; intervals = `0.5 s ± 1 sample`

27. **TC-5.4.5.1** `test_stinger_cooldown_suppress` — Cooldown 1 s. Trigger stinger at `t=0`, again
    at `t=0.5 s`.
    - Input: two `StingerScheduler::trigger("hit")` calls 0.5 s apart
    - Expected: first plays; second is suppressed (`SuppressedReason::Cooldown` recorded)

28. **TC-5.4.6.1** `test_playlist_no_immediate_repeat` — Run `Playlist::Shuffle` with 4 tracks for
    20 selections.
    - Input: tracks `[A, B, C, D]`, `mode = Shuffle`
    - Expected: zero adjacent duplicates in the 20-element selection log

29. **TC-5.4.7.1** `test_intensity_clamp_and_drive` — Set music intensity via
    `MusicStateMachine::set_intensity()` to `-0.5, 0.0, 0.5, 1.0, 2.0`.
    - Input: five intensity values
    - Expected: stored intensity clamps to `[0.0, 0.0, 0.5, 1.0, 1.0]`; stem 0 gain == 1.0 at
      `intensity == 0.0`; all stems active at `intensity == 1.0`

30. **TC-5.5.2.1** `test_jitter_buffer_depth_cap` — Inject 200 packets with simulated 50 ms jitter
    and 5% drop. Sample `JitterBuffer::depth_ms()` at each step.
    - Input: 200 Opus 20 ms packets, jitter `N(0, 50)` ms, 5% drop probability
    - Expected: `max(depth_ms) <= 80.0`; gaps filled by Opus PLC with no silence > 20 ms

31. **TC-5.5.3.1** `test_vad_speech_onset_20ms` — Feed 10 s of digital silence followed by
    `VoiceActivityDetector` ingesting a 5 s speech clip.
    - Input: silence then speech sample
    - Expected: zero packets emitted during silence; first packet emitted within 20 ms of first
      speech sample (`vad.first_voice_ts - speech_start <= 0.020 s`)

32. **TC-5.5.6.1** `test_dialogue_priority_interrupt` — Queue a `Bark` line. While playing, queue a
    `CriticalNarrative` line.
    - Input: `DialoguePriority::Bark` followed by `DialoguePriority::Critical`
    - Expected: bark voice transitions to `VoiceState::Stopped`; critical voice begins; subtitle
      event for critical fires within 16 ms of audio start

33. **TC-5.5.7.1** `test_dialogue_branch_gated_edge` — Three-branch dialogue graph; edges gated by
    `quest_progress == 0`, `1`, and `2`. Set state to `1` and traverse.
    - Input: dialogue graph rooted at `node_0` with three condition-gated edges
    - Expected: traversal selects the `quest_progress == 1` branch; non-matching edges skipped

34. **TC-5.3.6.1** `test_delay_chorus_flanger_variants` — Build an effect chain containing
    `AudioEffect::Delay`, `AudioEffect::Chorus`, and `AudioEffect::Flanger` enum variants and
    process an impulse through each in isolation.
    - Input: each variant with default configuration, impulse input
    - Expected: delay output has a secondary impulse at configured delay time; chorus and flanger
      outputs exhibit expected modulated delay taps, all three dispatched via enum (no `Box<dyn>`)

35. **TC-5.4.3.1** `test_music_transition_edge_modes` — Configure segment graph edges for each
    transition mode (immediate cut, timed crossfade, beat-synced crossfade, next-bar switch, custom
    curve) and trigger each.
    - Input: edges `A->B` configured with each of the 5 modes, trigger mid-bar
    - Expected: switch timing matches mode rule: immediate=same sample, timed=configured ms,
      beat-synced=next beat, next-bar=next bar boundary, custom=follows supplied curve

36. **TC-5.5.1.1** `test_mic_capture_opus_encode` — Capture 2 s of microphone input via platform
    backend and encode via Opus at 32 kbps.
    - Input: platform mic input stream, `OpusEncoder { bitrate: 32_000 }`
    - Expected: encoded frame stream produced; decoded output within perceptual tolerance of input;
      platform API used matches selected backend (WASAPI/CoreAudio/PipeWire)

37. **TC-5.5.4.1** `test_tts_synthesis_routes_to_bus` — Submit text "hello world" to TTS with voice
    id, verify output routes into `DialogueBus`.
    - Input: `TtsRequest { text: "hello world", voice: default }`
    - Expected: synthesized PCM lands on `DialogueBus`, platform-native backend used, voice
      lifecycle ends `Stopped` after synthesis completes

38. **TC-5.5.5.1** `test_phoneme_viseme_mapping` — Analyze a pre-recorded clip with known phonemes
    and capture the phoneme-to-viseme timeline.
    - Input: reference clip with hand-labeled phonemes
    - Expected: phoneme timestamps within 20 ms of reference; viseme ids mapped to expected set for
      both pre-recorded and live voice paths

39. **TC-5.5.8.1** `test_voice_channel_routing` — Configure proximity, party, guild, raid,
    broadcast, and custom channels and send a voice packet on each.
    - Input: six channel types with distinct volumes and route masks
    - Expected: each packet appears on its channel bus only; per-speaker volume setters take effect
      per channel independently

## Integration Tests

| ID            | Name                              | Req         |
|---------------|-----------------------------------|-------------|
| TC-5.1.I.1    | `test_split_screen_two_listeners` | R-5.1.2     |
| TC-5.1.I.2    | `test_prefetch_startup_under_10ms`| R-5.1.5a    |
| TC-5.2.I.1    | `test_propagation_through_portal` | R-5.2.6     |
| TC-5.2.I.2    | `test_shared_bvh_occlusion`       | R-5.2.5     |
| TC-5.3.I.1    | `test_convolution_vs_reference`   | R-5.3.4     |
| TC-5.4.I.1    | `test_horizontal_resequence_combat`| R-5.4.2    |
| TC-5.5.I.1    | `test_voice_chat_loopback_latency`| R-5.5.NF1   |
| TC-5.5.I.2    | `test_aec_erle_30db`              | R-5.5.9     |

1. **TC-5.1.I.1** `test_split_screen_two_listeners` — Spawn two `AudioListener` entities at distinct
   positions, each tagged to a `PlayerSlot`. Play one omnidirectional source between them. Assert
   each player's mix differs.
   - Input: listener A at `(-5, 0, 0)`, listener B at `(+5, 0, 0)`, source at `(0, 0, 0)`
   - Expected: per-listener panning differs; gains match symmetric attenuation; both buses receive
     output simultaneously

2. **TC-5.1.I.2** `test_prefetch_startup_under_10ms` — Issue `PrefetchHint::start("cinematic.ogg")`
   500 ms before triggering playback. Measure time from `play_command` to first non-zero output
   sample.
   - Input: prefetch at `t = -0.5 s`, play at `t = 0`
   - Expected: first non-zero output sample at `<= +10 ms` from play command

3. **TC-5.2.I.1** `test_propagation_through_portal` — Listener and source in adjacent rooms
   connected by an open portal. Run `PropagationSolver` async; verify per-voice tap delay and filter
   parameters. Close portal; verify path removed.
   - Input: two-room scene with one portal asset; source in room A, listener in room B
   - Expected: `PropagationSolver::tap_for(voice)` returns non-empty delay tap; after portal closes,
     `tap_for` returns empty within 1 solver tick

4. **TC-5.2.I.2** `test_shared_bvh_occlusion` — Verify `OcclusionSystem` rays are issued via
   `BvhResource::raycast` (not a private structure). Instrument the BVH and assert ray count.
   - Input: 64 voices with occlusion enabled; one frame
   - Expected: `bvh.raycast_count_this_frame() == 64`; no separate spatial index allocated

5. **TC-5.3.I.1** `test_convolution_vs_reference` — Convolve a recorded snare hit with a 1.5 s
   cathedral IR using `ConvolutionReverb`. Compare against an offline reference computed by direct
   FFT convolution.
   - Input: snare WAV + cathedral IR WAV (loaded via streaming system)
   - Expected: per-sample error `<= -60 dB` (relative to reference RMS); output latency
     `<= 1 buffer period`

6. **TC-5.4.I.1** `test_horizontal_resequence_combat` — Music is in `Exploration` segment; gameplay
   raises threat level. Assert `MusicStateMachine` transitions to `Combat` segment at the next bar
   boundary, with no audible click.
   - Input: graph `Exploration -> Combat` with bar-quantized edge; threat trigger at random offset
   - Expected: switch occurs at next bar boundary; no underrun; cross-correlation between adjacent
     buffers shows continuous waveform

7. **TC-5.5.I.1** `test_voice_chat_loopback_latency` — Capture mic input, encode Opus, route via
   loopback transport with simulated 50 ms RTT, decode, mix. Measure end-to-end latency.
   - Input: 1 kHz tone burst at mic; loopback `RTT = 50 ms`, `loss = 0%`
   - Expected: total latency `<= 150 ms`; Opus quality `PESQ >= 3.0`

8. **TC-5.5.I.2** `test_aec_erle_30db` — Play a reference signal through speakers (loopback into mic
   via virtual device). Run `AcousticEchoCanceller`. Measure ERLE.
   - Input: pink noise reference, 30 s capture
   - Expected: ERLE `>= 30 dB`; reconvergence after acoustic change `<= 2 s`; comfort noise present
     during PLC gaps

## Benchmarks

| ID           | Benchmark                              | Target     | Req          |
|--------------|----------------------------------------|------------|--------------|
| TC-5.1.B.1   | Audio thread budget (256 voices)       | < 0.5 ms   | R-5.1.NF1    |
| TC-5.1.B.2   | Mixer output latency                   | < 20 ms    | R-5.1.NF4    |
| TC-5.2.B.1   | Per-voice spatialization               | < 2 us     | R-5.2.NF1    |
| TC-5.2.B.2   | Propagation solver update              | < 4 ms     | R-5.2.NF2    |
| TC-5.3.B.1   | 4-insert DSP chain per sample          | < 1 us     | R-5.3.NF1    |
| TC-5.5.B.1   | 32 simultaneous Opus decode + mix      | < 0.5 ms   | R-5.5.NF2    |

1. **TC-5.1.B.1** — Worst-case mix loop: 128 active voices + 128 virtualized, each with full
   spatialization and a 4-insert DSP chain. 10,000 buffer iterations of 512 samples at 48 kHz.
   Assert p99 wall time per buffer below 0.5 ms. Measured with `criterion`.

2. **TC-5.1.B.2** — End-to-end output latency from `AudioCommand::PlaySound` to first non-zero
   sample at the platform backend. Measured via virtual loopback device on each backend (WASAPI,
   CoreAudio, ALSA).

3. **TC-5.2.B.1** — `Spatializer::process_voice` over 256 voices with HRTF, distance, Doppler, and
   panning. Wall time per voice. Assert p99 below 2 microseconds. No allocation in the hot path.

4. **TC-5.2.B.2** — `PropagationSolver::update` over a scene with 20 portals, 50 reflective
   surfaces, and 64 sources. Wall time on the worker thread. Assert p99 below 4 ms; runs at no more
   than 10 Hz.

5. **TC-5.3.B.1** — `EffectChain::process` with `[Biquad, ParamEq, Compressor, Delay]` on a single
   voice. Per-sample wall time at 48 kHz. Assert below 1 microsecond. No vtable indirection in the
   disassembly.

6. **TC-5.5.B.1** — Decode and mix 32 simultaneous Opus streams at 24 kbps into a stereo bus. Wall
   time per buffer. Assert p99 below 0.5 ms. No `Box<dyn>` allocation in the decoder pool.
