# R-5.5 -- Voice & Speech Requirements

## Voice Chat

1. **R-5.5.1** — The engine **SHALL** capture microphone input using platform-native APIs (WASAPI on
   Windows, CoreAudio on macOS, PipeWire/ALSA on Linux), encode with Opus at configurable bitrates
   (6-64 kbps), and transmit voice packets over the networking layer. Decoded streams **SHALL** be
   routable as spatialized 3D sources or non-spatialized bus sends.
   - **Rationale:** Opus provides excellent quality-to-bandwidth ratio for multiplayer scenarios
     with many voice streams. Platform-native capture ensures correct device access.
   - **Verification:** Integration test: encode and decode a 10 s recording at 24 and 64 kbps and
     verify PESQ score exceeds 3.0. Measure capture-to-output latency below 40 ms.

2. **R-5.5.2** — The engine **SHALL** buffer incoming voice packets in an adaptive jitter buffer
   that adjusts depth based on observed jitter, not exceeding 80 ms total. Lost packets **SHALL** be
   filled via Opus PLC with no silence gap exceeding one Opus frame.
   - **Rationale:** Network jitter and loss are inevitable; adaptive buffering and PLC maintain
     quality without excessive latency.
   - **Verification:** Integration test: simulate 50 ms jitter and 5% loss and verify buffer depth
     does not exceed 80 ms. Verify PLC fills gaps. Measure output MOS above 2.5 under loss.

3. **R-5.5.3** — The engine **SHALL** detect speech onset within 20 ms using a VAD to gate
   transmission. Noise suppression **SHALL** improve output SNR by at least 20 dB at 0 dB input SNR.
   Mobile **SHALL** use a lighter suppression model.
   - **Rationale:** VAD reduces bandwidth when silent. Noise suppression improves intelligibility
     for all participants.
   - **Verification:** Unit test: feed 10 s silence then 5 s speech and verify zero packets during
     silence and transmission within 20 ms of onset. Verify SNR improvement of at least 20 dB.

## Text-to-Speech

4. **R-5.5.4** — The engine **SHALL** generate speech from text using platform-native TTS (SAPI on
   Windows, AVSpeechSynthesizer on macOS, Speech Dispatcher on Linux). TTS output **SHALL** route
   through the mixer bus hierarchy. Generation for a 20-word string **SHALL** complete within 500
   ms.
   - **Rationale:** TTS serves accessibility needs and provides placeholder dialogue during
     development.
   - **Verification:** Integration test: submit a 20-word string and verify audio within 500 ms.
     Verify TTS output is routable through the mixer with gain, panning, and effects.

## Lip Sync

5. **R-5.5.5** — The engine **SHALL** extract phoneme-to-viseme mappings from pre-recorded dialogue
   at load time and from live voice chat in real time. Pre-recorded timestamps **SHALL** be within
   30 ms of audio onset. Live output latency **SHALL** be below 50 ms. Output **SHALL** be a
   timestamped viseme track consumable by the animation system.
   - **Rationale:** Automated viseme generation drives lip-sync for both NPCs and player avatars
     without manual authoring.
   - **Verification:** Unit test: process dialogue with known phonemes and verify timestamps within
     30 ms. Feed a live stream and verify output latency below 50 ms.

## Dialogue

6. **R-5.5.6** — The engine **SHALL** manage a priority queue for dialogue with classes (critical
   narrative, bark, ambient chatter). Higher-priority lines **SHALL** interrupt lower-priority
   lines. Queued lines **SHALL** expire after a configurable timeout. Subtitle events **SHALL** fire
   within 16 ms of voice-over playback.
   - **Rationale:** Dialogue management prevents overlapping speech, ensures critical lines are
     never missed, and synchronizes subtitles for accessibility.
   - **Verification:** Unit test: queue a bark then trigger a critical line and verify interruption.
     Verify subtitle events fire within 16 ms of playback.

7. **R-5.5.7** — The engine **SHALL** represent dialogue trees as directed graphs with
   condition-gated edges evaluated against gameplay state (quest progress, reputation, inventory).
   The graph **SHALL** support branching choices, random variation nodes, and loopback edges. Each
   node **SHALL** reference a voice-over asset and subtitle text.
   - **Rationale:** Branching dialogue enables dynamic NPC conversations that respond to player
     choices and progress.
   - **Verification:** Unit test: set quest progress to 1 in a 3-branch graph and verify correct
     branch. Verify random variation over 100 runs. Verify loopback edges allow re-entry.

## Voice Chat Channels

8. **R-5.5.8** — The engine **SHALL** support proximity, party, guild, raid, broadcast, and custom
   voice channels with independent volume and routing. Each channel **SHALL** support per-speaker
   muting, priority levels, and admin controls. Players **SHALL** monitor multiple channels
   simultaneously. Proximity **SHALL** use distance attenuation via the shared spatial index.
   - **Rationale:** Diverse social structures require flexible channel management with independent
     routing and admin control.
   - **Verification:** Integration test: create proximity, party, and raid channels and verify
     independent volume. Mute a speaker in one channel and verify audibility in others. Kick a
     player and verify they can no longer participate.

## Acoustic Echo Cancellation

9. **R-5.5.9** — The engine **SHALL** implement real-time AEC for speaker users. ERLE **SHALL**
   exceed 30 dB. The adaptive filter **SHALL** reconverge within 2 seconds after acoustic changes.
   Processing latency **SHALL** be below 1 ms. Comfort noise **SHALL** fill gaps left by echo
   removal.
   - **Rationale:** AEC prevents echo when others use speakers. Comfort noise prevents unnatural
     silence during cancelled gaps.
   - **Verification:** Integration test: play a reference signal through speakers, capture mic, and
     verify ERLE above 30 dB. Change acoustics and verify reconvergence within 2 seconds. Verify
     comfort noise and latency below 1 ms.

10. **R-5.5.9a** — The engine **SHALL** defer to native AEC on iOS and Android. Custom AEC **SHALL**
    be used on PC and consoles where platform AEC is unavailable.
    - **Rationale:** Mobile platforms provide optimized system-level AEC tuned for their hardware;
      deferring avoids duplicate processing.
    - **Verification:** Integration test: verify native AEC on iOS and Android. Verify custom AEC on
      PC and consoles.

## Non-Functional Requirements

11. **R-5.5.NF1** — The engine **SHALL** deliver voice chat with end-to-end latency not exceeding
    150 ms under normal conditions (< 50 ms RTT, < 1% loss).
    - **Rationale:** Latency above 150 ms causes conversational overlap and frustration.
    - **Verification:** Integration test: measure mic input to decoded output on a loopback with 50
      ms simulated RTT. Assert total latency below 150 ms.

12. **R-5.5.NF2** — The engine **SHALL** decode and mix at least 32 simultaneous incoming voice
    streams without exceeding the audio thread budget or causing artifacts.
    - **Rationale:** Large raid groups may have multiple participants transmitting simultaneously.
    - **Verification:** Stress test: feed 32 concurrent Opus streams at 24 kbps. Assert no underruns
      over 60 seconds.
