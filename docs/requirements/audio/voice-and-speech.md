# R-5.5 — Voice & Speech Requirements

## Voice Chat

### R-5.5.1 Voice Chat Codec and Transport

The engine **SHALL** capture microphone input, encode with Opus at configurable bitrates
(6-64 kbps), transmit voice packets over the networking layer, and decode incoming streams
routable as spatialized 3D sources or non-spatialized bus sends, using platform-native
capture APIs (WASAPI on Windows, CoreAudio on macOS, PipeWire/ALSA on Linux).

- **Derived from:** [F-5.5.1](../../features/audio/voice-and-speech.md)
- **Rationale:** Opus provides excellent quality-to-bandwidth ratio for scenarios where
  dozens of players may transmit simultaneously in a raid or city zone.
- **Verification:** Integration test: capture a 10 s microphone recording, encode at 24 kbps
  and 64 kbps, decode, and verify PESQ score exceeds 3.0 at both bitrates. Measure
  end-to-end latency (capture to decoded output) and verify it is below 40 ms. Verify
  decoded streams can be routed as both 3D-spatialized and non-spatialized bus sends.

### R-5.5.2 Jitter Buffer and Packet Loss Concealment

The engine **SHALL** buffer incoming voice packets in an adaptive jitter buffer that
dynamically adjusts depth based on observed network jitter, and **SHALL** apply Opus built-in
packet-loss concealment to mask gaps when packets are lost or arrive late. Voice packets are
routed directly from the network transport layer to this buffer, bypassing the game state jitter
buffer (R-8.4.6). The network layer identifies voice packets by channel type field and delivers
them to the audio system without game-state buffering.

- **Derived from:** [F-5.5.2](../../features/audio/voice-and-speech.md)
- **Rationale:** Adaptive buffering and PLC are essential for playable voice chat under
  variable network conditions typical of internet gaming.
- **Verification:** Unit test: simulate a stream with 50 ms jitter and 5% packet loss.
  Verify the jitter buffer adjusts depth to accommodate the jitter without exceeding 80 ms
  total buffering. Verify PLC fills gaps with interpolated audio (no silence gaps exceeding
  one Opus frame). Measure output MOS score and verify it exceeds 2.5 under these
  conditions.

### R-5.5.3 Voice Activity Detection and Noise Suppression

The engine **SHALL** detect speech onset and offset using a voice activity detector to gate
transmission, and **SHALL** apply noise suppression that attenuates non-speech input (keyboard
clicks, fans, background noise) by at least 20 dB before encoding.

- **Derived from:** [F-5.5.3](../../features/audio/voice-and-speech.md)
- **Rationale:** VAD reduces upstream bandwidth and mixer load when a player is silent, while
  noise suppression improves intelligibility for all participants.
- **Verification:** Unit test: feed 10 s of silence followed by 5 s of speech into the VAD.
  Verify transmission is gated off during silence (zero packets sent) and gated on within
  20 ms of speech onset. Feed speech mixed with keyboard noise at 0 dB SNR into the noise
  suppressor and verify output SNR improves by at least 20 dB.

## Text-to-Speech

### R-5.5.4 Text-to-Speech Integration

The engine **SHALL** generate speech audio from text strings using platform TTS services
(SAPI on Windows, AVSpeechSynthesizer on macOS, Speech Dispatcher on Linux) and feed the
result into the mixer as a standard sound source.

- **Derived from:** [F-5.5.4](../../features/audio/voice-and-speech.md)
- **Rationale:** TTS serves accessibility needs by reading UI text, quest descriptions, and
  chat messages aloud, and provides placeholder dialogue during development.
- **Verification:** Integration test: submit a 20-word text string to TTS on each supported
  platform. Verify audio output is generated within 500 ms of the request. Verify the output
  is routable through the mixer bus hierarchy as a standard sound source with gain, panning,
  and effects processing. Verify the output is intelligible (manual listening test or STT
  round-trip accuracy exceeds 90%).

## Lip Sync

### R-5.5.5 Viseme Generation for Lip Sync

The engine **SHALL** analyze audio streams in real time to extract phoneme-to-viseme mappings
that produce a timestamped viseme track consumable by the animation system, supporting both
pre-recorded dialogue and live voice chat streams.

- **Derived from:** [F-5.5.5](../../features/audio/voice-and-speech.md)
- **Rationale:** Real-time viseme generation enables lip-synced NPCs and player avatars for
  both authored dialogue and live voice chat.
- **Verification:** Integration test: process a 10 s dialogue recording with known phoneme
  content. Verify the generated viseme track contains entries for all expected phoneme
  groups. Verify viseme timestamps are within 30 ms of the corresponding audio onset. Feed
  a live voice stream and verify viseme output latency is below 50 ms from audio input.
  Verify the animation system can consume the viseme track without format conversion.

## Dialogue System

### R-5.5.6 Dialogue Playback and Queuing

The engine **SHALL** play authored voice-over lines with subtitle synchronization via a
priority queue (critical narrative, bark, ambient chatter) that prevents overlapping
dialogue, interrupting or deferring lower-priority lines when a higher-priority line is
triggered, with queued lines expiring after a configurable timeout.

- **Derived from:** [F-5.5.6](../../features/audio/voice-and-speech.md)
- **Rationale:** Priority-based dialogue queuing ensures critical narrative lines are never
  missed while stale ambient barks do not play long after their context has passed.
- **Verification:** Unit test: queue a low-priority bark, then trigger a critical narrative
  line. Verify the bark is interrupted and the narrative line plays immediately. Queue 5
  barks and verify they play sequentially without overlap. Set a 3 s timeout on a bark,
  delay playback by 4 s, and verify the bark is discarded. Verify subtitle events fire in
  sync with voice-over playback within 16 ms tolerance.

### R-5.5.7 Branching Dialogue Graph

The engine **SHALL** represent dialogue trees as directed graphs with condition-gated edges
evaluated against gameplay state (quest progress, reputation, inventory), supporting
branching choices, random variation nodes, and loopback edges, where each node references a
voice-over asset and subtitle text.

- **Derived from:** [F-5.5.7](../../features/audio/voice-and-speech.md)
- **Rationale:** Directed dialogue graphs support complex NPC interactions with branching
  choices and repeatable conversations without custom scripting.
- **Verification:** Integration test: construct a dialogue graph with 3 branches gated on
  quest progress values (0, 1, 2). Set quest progress to 1 and verify the correct branch
  is selected. Verify random variation nodes produce different selections over 100 runs.
  Verify loopback edges allow re-entering a conversation node. Verify each node triggers
  playback of its referenced voice-over asset and subtitle text.

## Voice Chat Channels

### R-5.5.8 Voice Chat Channel Management

The engine **SHALL** manage multiple concurrent voice chat channels (proximity, party, guild,
raid, broadcast, custom) with per-channel permissions, per-speaker muting, priority levels,
independent volume controls, and administrative controls (kick, mute, ban), allowing players
to monitor multiple channels simultaneously.

- **Derived from:** [F-5.5.8](../../features/audio/voice-and-speech.md)
- **Rationale:** Multiple channel types with independent controls are required for MMO
  social structures ranging from small parties to large raid groups and public broadcasts.
- **Verification:** Integration test: create proximity, party, and raid channels
  simultaneously. Verify audio from each channel is received with independent volume levels.
  Mute a speaker in one channel and verify they remain audible in other channels. Apply
  raid-leader priority and verify their voice overrides background chatter. Kick a player
  from a channel and verify they can no longer transmit or receive on that channel. Verify
  proximity channel uses distance attenuation via the shared spatial index.

## Acoustic Echo Cancellation

### R-5.5.9 Acoustic Echo Cancellation

The engine **SHALL** perform real-time acoustic echo cancellation that subtracts the known
speaker output signal from microphone input, supporting both linear adaptive filtering and
non-linear processing, with a comfort noise generator filling gaps left by echo removal and
AEC processing latency below 1 ms on the audio thread.

- **Derived from:** [F-5.5.9](../../features/audio/voice-and-speech.md)
- **Rationale:** AEC prevents other players from hearing their own voices echoed back when a
  player uses speakers instead of headphones, which is critical for usable voice chat.
- **Verification:** Integration test: play a reference signal through speakers and capture
  microphone input in a simulated room. Verify echo return loss enhancement (ERLE) exceeds
  30 dB. Change the simulated room acoustics and verify the adaptive filter re-converges
  within 2 seconds. Verify comfort noise is present during echo-cancelled silence (output
  is not digital silence). Measure AEC processing latency and verify it is below 1 ms.
  On platforms with native AEC (iOS, Android), verify the engine defers to the platform
  implementation.

---

## Non-Functional Requirements

### R-5.5.NF1 Voice Chat End-to-End Latency

The engine **SHALL** deliver voice chat audio with end-to-end latency (microphone capture to
remote speaker output) not exceeding 150 ms under normal network conditions (< 50 ms RTT,
< 1% packet loss), including encoding, transmission, jitter buffering, and decoding.

- **Derived from:** [F-5.5.1](../../features/audio/voice-and-speech.md),
  [F-5.5.2](../../features/audio/voice-and-speech.md)
- **Rationale:** Voice chat latency above 150 ms causes conversational overlap and frustration.
  The budget must account for codec latency (~20 ms), jitter buffer (~40 ms), and network
  transit.
- **Verification:** Integration test: measure time from microphone input to decoded output on
  a loopback connection with 50 ms simulated RTT. Assert total latency is below 150 ms.

### R-5.5.NF2 Simultaneous Voice Stream Capacity

The engine **SHALL** decode and mix at least 32 simultaneous incoming voice streams without
exceeding the audio thread budget (R-5.1.NF1) or causing audible artifacts.

- **Derived from:** [F-5.5.8](../../features/audio/voice-and-speech.md),
  [R-5.1.NF1](audio-engine.md)
- **Rationale:** Large raid groups (20-40 players) may have multiple participants transmitting
  simultaneously. The system must handle peak concurrent streams without degradation.
- **Verification:** Stress test: feed 32 concurrent Opus streams at 24 kbps into the decoder
  and mixer. Assert no audio underruns over 60 seconds and output quality remains
  intelligible.
