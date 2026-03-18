# R-5.5 -- Voice & Speech Requirements

## Voice Chat

| ID      | Derived From                                        |
|---------|-----------------------------------------------------|
| R-5.5.1 | [F-5.5.1](../../features/audio/voice-and-speech.md) |
| R-5.5.2 | [F-5.5.2](../../features/audio/voice-and-speech.md) |
| R-5.5.3 | [F-5.5.3](../../features/audio/voice-and-speech.md) |

1. **R-5.5.1** — The engine **SHALL** capture microphone input using platform-native APIs (WASAPI on
   Windows, CoreAudio on macOS, PipeWire/ALSA on Linux), encode with Opus at configurable bitrates
   (6-64 kbps), and transmit voice packets over the networking layer. Decoded streams **SHALL** be
   routable as spatialized 3D sources (proximity chat) or non-spatialized bus sends (party channel).
   - **Rationale:** Opus provides excellent quality-to-bandwidth ratio critical for MMO scenarios
     with dozens of simultaneous voice streams. Platform-native capture ensures correct device
     access.
   - **Verification:** Integration test: encode and decode a 10s recording at 24 and 64 kbps and
     verify PESQ score exceeds 3.0. Measure capture-to-decoded-output latency and verify it is below
     40 ms. Verify platform-native capture API usage via instrumentation.
2. **R-5.5.2** — The engine **SHALL** buffer incoming voice packets in an adaptive jitter buffer
   that adjusts depth based on observed network jitter, not exceeding 80 ms total. When packets are
   lost, the engine **SHALL** apply Opus built-in PLC to fill gaps, with no silence gap exceeding
   one Opus frame.
   - **Rationale:** Network jitter and packet loss are inevitable in multiplayer games; adaptive
     buffering and PLC maintain voice quality without excessive latency.
   - **Verification:** Integration test: simulate 50 ms jitter and 5% packet loss and verify the
     buffer adjusts depth without exceeding 80 ms total. Verify PLC fills gaps with interpolated
     audio. Measure output MOS score under loss conditions and verify it exceeds 2.5.
3. **R-5.5.3** — The engine **SHALL** detect speech onset within 20 ms using a voice activity
   detector (VAD) to gate transmission. The engine **SHALL** suppress background noise (keyboard
   clicks, fans) before encoding, improving output SNR by at least 20 dB at 0 dB input SNR. Mobile
   **SHALL** use a lighter noise suppression model.
   - **Rationale:** VAD reduces upstream bandwidth when players are silent. Noise suppression
     improves intelligibility for all participants.
   - **Verification:** Unit test: feed 10s silence then 5s speech and verify zero packets during
     silence and transmission within 20 ms of speech onset. Feed speech mixed with keyboard noise at
     0 dB SNR and verify output SNR improves by at least 20 dB.

## Text-to-Speech

| ID      | Derived From                                        |
|---------|-----------------------------------------------------|
| R-5.5.4 | [F-5.5.4](../../features/audio/voice-and-speech.md) |

1. **R-5.5.4** — The engine **SHALL** generate speech audio from text strings using platform-native
   TTS services (SAPI on Windows, AVSpeechSynthesizer on macOS, Speech Dispatcher on Linux). TTS
   output **SHALL** route through the mixer bus hierarchy as a standard sound source with gain,
   panning, and effects. TTS generation for a 20-word string **SHALL** complete within 500 ms.
   - **Rationale:** TTS serves accessibility needs (reading UI text, quest descriptions, chat
     messages aloud) and provides placeholder dialogue during development.
   - **Verification:** Integration test: submit a 20-word string and verify audio output is
     generated within 500 ms. Verify TTS output is routable through the mixer with gain, panning,
     and effects. Verify the correct platform-native TTS API is used.

## Lip Sync

| ID      | Derived From                                        |
|---------|-----------------------------------------------------|
| R-5.5.5 | [F-5.5.5](../../features/audio/voice-and-speech.md) |

1. **R-5.5.5** — The engine **SHALL** extract phoneme-to-viseme mappings from pre-recorded dialogue
   at load time and from live voice chat in real time. Viseme timestamps **SHALL** be within 30 ms
   of audio onset for pre-recorded content. Live viseme output latency **SHALL** be below 50 ms.
   Output **SHALL** be a timestamped viseme track consumable by the animation system.
   - **Rationale:** Automated viseme generation drives character lip-sync animation for both NPCs
     and player avatars without manual animation authoring.
   - **Verification:** Unit test: process dialogue with known phoneme content and verify viseme
     timestamps are within 30 ms of audio onset. Feed a live voice stream and verify viseme output
     latency is below 50 ms.

## Dialogue

| ID      | Derived From                                        |
|---------|-----------------------------------------------------|
| R-5.5.6 | [F-5.5.6](../../features/audio/voice-and-speech.md) |
| R-5.5.7 | [F-5.5.7](../../features/audio/voice-and-speech.md) |

1. **R-5.5.6** — The engine **SHALL** manage a priority queue for dialogue playback with priority
   classes (critical narrative, bark, ambient chatter). A higher-priority line **SHALL** interrupt
   lower-priority lines immediately. Queued lines **SHALL** expire after a configurable timeout.
   Subtitle events **SHALL** fire in sync with voice-over playback within 16 ms tolerance.
   - **Rationale:** Dialogue management prevents overlapping speech, ensures critical narrative
     lines are never missed, and synchronizes subtitles for accessibility.
   - **Verification:** Unit test: queue a bark then trigger a critical line and verify the bark is
     interrupted. Queue 5 barks and verify sequential playback without overlap. Verify subtitle
     events fire in sync with voice-over within 16 ms tolerance.
2. **R-5.5.7** — The engine **SHALL** represent dialogue trees as directed graphs with
   condition-gated edges evaluated against gameplay state (quest progress, reputation, inventory).
   The graph **SHALL** support branching choices, random variation nodes, and loopback edges for
   repeatable NPC interactions. Each node **SHALL** reference a voice-over asset and subtitle text.
   - **Rationale:** Branching dialogue enables dynamic NPC conversations that respond to player
     choices and quest progress, increasing narrative immersion.
   - **Verification:** Unit test: set quest progress to 1 in a 3-branch graph and verify the correct
     branch is selected. Verify random variation nodes produce different selections over 100 runs.
     Verify loopback edges allow re-entering a conversation node.

## Voice Chat Channels

| ID      | Derived From                                        |
|---------|-----------------------------------------------------|
| R-5.5.8 | [F-5.5.8](../../features/audio/voice-and-speech.md) |

1. **R-5.5.8** — The engine **SHALL** support proximity, party, guild, raid, broadcast, and custom
   voice channels with independent volume and routing. Each channel **SHALL** support per-speaker
   muting, priority levels, and administrative controls (kick, mute, ban). Players **SHALL** monitor
   multiple channels simultaneously. Proximity channels **SHALL** use distance attenuation via the
   shared spatial index.
   - **Rationale:** Diverse social structures in MMOs require flexible voice channel management with
     independent routing and administrative control.
   - **Verification:** Integration test: create proximity, party, and raid channels and verify audio
     from each is received with independent volume levels. Mute a speaker in one channel and verify
     they remain audible in other channels. Kick a player and verify they can no longer transmit or
     receive on that channel.

## Acoustic Echo Cancellation

| ID       | Derived From                                        |
|----------|-----------------------------------------------------|
| R-5.5.9  | [F-5.5.9](../../features/audio/voice-and-speech.md) |
| R-5.5.9a | [F-5.5.9](../../features/audio/voice-and-speech.md) |

1. **R-5.5.9** — The engine **SHALL** implement real-time acoustic echo cancellation for players
   using speakers. The AEC module **SHALL** achieve echo return loss enhancement (ERLE) exceeding 30
   dB. The adaptive filter **SHALL** reconverge within 2 seconds after room acoustic changes. AEC
   processing latency **SHALL** be below 1 ms on the audio thread. A comfort noise generator
   **SHALL** fill gaps left by echo removal.
   - **Rationale:** Echo cancellation prevents players from hearing their own voices echoed back
     when others use speakers. Comfort noise prevents unnatural silence during cancelled gaps.
   - **Verification:** Integration test: play a reference signal through speakers, capture mic
     input, and verify ERLE exceeds 30 dB. Change room acoustics and verify filter reconverges
     within 2 seconds. Verify comfort noise is present during cancelled silence. Verify AEC
     processing latency is below 1 ms.
2. **R-5.5.9a** — The engine **SHALL** defer to native AEC on iOS and Android where platform-level
   AEC is available. Custom AEC **SHALL** be used on PC and consoles where platform AEC is
   unavailable or insufficient.
   - **Rationale:** Mobile platforms provide optimized system-level AEC tuned for their hardware;
     deferring avoids duplicate processing and saves CPU.
   - **Verification:** Integration test: verify the engine uses native AEC on iOS and Android.
     Verify custom AEC is active on PC and console platforms.

## Non-Functional Requirements

| ID        | Derived From       |
|-----------|--------------------|
| R-5.5.NF1 | F-5.5.1, F-5.5.2   |
| R-5.5.NF2 | F-5.5.8, R-5.1.NF1 |

1. **R-5.5.NF1** — The engine **SHALL** deliver voice chat audio with end-to-end latency not
   exceeding 150 ms under normal network conditions (< 50 ms RTT, < 1% packet loss).
   - **Rationale:** Voice chat latency above 150 ms causes conversational overlap and frustration.
   - **Verification:** Integration test: measure time from microphone input to decoded output on a
     loopback with 50 ms simulated RTT. Assert total latency is below 150 ms.
2. **R-5.5.NF2** — The engine **SHALL** decode and mix at least 32 simultaneous incoming voice
   streams without exceeding the audio thread budget or causing artifacts.
   - **Rationale:** Large raid groups may have multiple participants transmitting simultaneously.
   - **Verification:** Stress test: feed 32 concurrent Opus streams at 24 kbps. Assert no audio
     underruns over 60 seconds.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/audio/voice-and-speech.md](../../user-stories/audio/voice-and-speech.md).
