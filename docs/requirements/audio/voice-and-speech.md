# R-5.5 — Voice & Speech Requirements

## Voice Chat

### R-5.5.1 Voice Chat Codec and Transport

The engine **SHALL** capture microphone input using platform-native APIs (WASAPI on Windows,
CoreAudio on macOS, PipeWire/ALSA on Linux), encode with Opus at configurable bitrates (6-64 kbps),
and transmit voice packets over the networking layer. Decoded streams **SHALL** be routable as
spatialized 3D sources (proximity chat) or non-spatialized bus sends (party channel).

- **Derived from:**
  [F-5.5.1](../../features/audio/voice-and-speech.md)
- **Rationale:** Opus provides excellent quality-to- bandwidth ratio critical for MMO scenarios with
  dozens of simultaneous voice streams. Platform-native capture ensures correct device access.
- **Verification:** Integration test: encode and decode a 10s recording at 24 and 64 kbps and verify
  PESQ score exceeds 3.0. Measure capture-to-decoded-output latency and verify it is below 40 ms.
  Verify platform- native capture API usage via instrumentation.

### R-5.5.2 Jitter Buffer and Packet Loss Concealment

The engine **SHALL** buffer incoming voice packets in an adaptive jitter buffer that adjusts depth
based on observed network jitter, not exceeding 80 ms total. When packets are lost, the engine
**SHALL** apply Opus built-in PLC to fill gaps, with no silence gap exceeding one Opus frame.

- **Derived from:**
  [F-5.5.2](../../features/audio/voice-and-speech.md)
- **Rationale:** Network jitter and packet loss are inevitable in multiplayer games; adaptive
  buffering and PLC maintain voice quality without excessive latency.
- **Verification:** Integration test: simulate 50 ms jitter and 5% packet loss and verify the buffer
  adjusts depth without exceeding 80 ms total. Verify PLC fills gaps with interpolated audio.
  Measure output MOS score under loss conditions and verify it exceeds 2.5.

### R-5.5.3 Voice Activity Detection and Noise Suppression

The engine **SHALL** detect speech onset within 20 ms using a voice activity detector (VAD) to gate
transmission. The engine **SHALL** suppress background noise (keyboard clicks, fans) before
encoding, improving output SNR by at least 20 dB at 0 dB input SNR. Mobile **SHALL** use a lighter
noise suppression model.

- **Derived from:**
  [F-5.5.3](../../features/audio/voice-and-speech.md)
- **Rationale:** VAD reduces upstream bandwidth when players are silent. Noise suppression improves
  intelligibility for all participants.
- **Verification:** Unit test: feed 10s silence then 5s speech and verify zero packets during
  silence and transmission within 20 ms of speech onset. Feed speech mixed with keyboard noise at 0
  dB SNR and verify output SNR improves by at least 20 dB.

## Text-to-Speech

### R-5.5.4 Text-to-Speech Integration

The engine **SHALL** generate speech audio from text strings using platform-native TTS services
(SAPI on Windows, AVSpeechSynthesizer on macOS, Speech Dispatcher on Linux). TTS output **SHALL**
route through the mixer bus hierarchy as a standard sound source with gain, panning, and effects.
TTS generation for a 20-word string **SHALL** complete within 500 ms.

- **Derived from:**
  [F-5.5.4](../../features/audio/voice-and-speech.md)
- **Rationale:** TTS serves accessibility needs (reading UI text, quest descriptions, chat messages
  aloud) and provides placeholder dialogue during development.
- **Verification:** Integration test: submit a 20-word string and verify audio output is generated
  within 500 ms. Verify TTS output is routable through the mixer with gain, panning, and effects.
  Verify the correct platform-native TTS API is used.

## Lip Sync

### R-5.5.5 Viseme Generation for Lip Sync

The engine **SHALL** extract phoneme-to-viseme mappings from pre-recorded dialogue at load time and
from live voice chat in real time. Viseme timestamps **SHALL** be within 30 ms of audio onset for
pre-recorded content. Live viseme output latency **SHALL** be below 50 ms. Output **SHALL** be a
timestamped viseme track consumable by the animation system.

- **Derived from:**
  [F-5.5.5](../../features/audio/voice-and-speech.md)
- **Rationale:** Automated viseme generation drives character lip-sync animation for both NPCs and
  player avatars without manual animation authoring.
- **Verification:** Unit test: process dialogue with known phoneme content and verify viseme
  timestamps are within 30 ms of audio onset. Feed a live voice stream and verify viseme output
  latency is below 50 ms.

## Dialogue

### R-5.5.6 Dialogue Playback and Queuing

The engine **SHALL** manage a priority queue for dialogue playback with priority classes (critical
narrative, bark, ambient chatter). A higher-priority line **SHALL** interrupt lower-priority lines
immediately. Queued lines **SHALL** expire after a configurable timeout. Subtitle events **SHALL**
fire in sync with voice-over playback within 16 ms tolerance.

- **Derived from:**
  [F-5.5.6](../../features/audio/voice-and-speech.md)
- **Rationale:** Dialogue management prevents overlapping speech, ensures critical narrative lines
  are never missed, and synchronizes subtitles for accessibility.
- **Verification:** Unit test: queue a bark then trigger a critical line and verify the bark is
  interrupted. Queue 5 barks and verify sequential playback without overlap. Verify subtitle events
  fire in sync with voice-over within 16 ms tolerance.

### R-5.5.7 Branching Dialogue Graph

The engine **SHALL** represent dialogue trees as directed graphs with condition-gated edges
evaluated against gameplay state (quest progress, reputation, inventory). The graph **SHALL**
support branching choices, random variation nodes, and loopback edges for repeatable NPC
interactions. Each node **SHALL** reference a voice-over asset and subtitle text.

- **Derived from:**
  [F-5.5.7](../../features/audio/voice-and-speech.md)
- **Rationale:** Branching dialogue enables dynamic NPC conversations that respond to player choices
  and quest progress, increasing narrative immersion.
- **Verification:** Unit test: set quest progress to 1 in a 3-branch graph and verify the correct
  branch is selected. Verify random variation nodes produce different selections over 100 runs.
  Verify loopback edges allow re-entering a conversation node.

## Voice Chat Channels

### R-5.5.8 Voice Chat Channel Management

The engine **SHALL** support proximity, party, guild, raid, broadcast, and custom voice channels
with independent volume and routing. Each channel **SHALL** support per-speaker muting, priority
levels, and administrative controls (kick, mute, ban). Players **SHALL** monitor multiple channels
simultaneously. Proximity channels **SHALL** use distance attenuation via the shared spatial index.

- **Derived from:**
  [F-5.5.8](../../features/audio/voice-and-speech.md)
- **Rationale:** Diverse social structures in MMOs require flexible voice channel management with
  independent routing and administrative control.
- **Verification:** Integration test: create proximity, party, and raid channels and verify audio
  from each is received with independent volume levels. Mute a speaker in one channel and verify
  they remain audible in other channels. Kick a player and verify they can no longer transmit or
  receive on that channel.

## Acoustic Echo Cancellation

### R-5.5.9 Acoustic Echo Cancellation

The engine **SHALL** implement real-time acoustic echo cancellation for players using speakers. The
AEC module **SHALL** achieve echo return loss enhancement (ERLE) exceeding 30 dB. The adaptive
filter **SHALL** reconverge within 2 seconds after room acoustic changes. AEC processing latency
**SHALL** be below 1 ms on the audio thread. A comfort noise generator **SHALL** fill gaps left by
echo removal.

- **Derived from:**
  [F-5.5.9](../../features/audio/voice-and-speech.md)
- **Rationale:** Echo cancellation prevents players from hearing their own voices echoed back when
  others use speakers. Comfort noise prevents unnatural silence during cancelled gaps.
- **Verification:** Integration test: play a reference signal through speakers, capture mic input,
  and verify ERLE exceeds 30 dB. Change room acoustics and verify filter reconverges within 2
  seconds. Verify comfort noise is present during cancelled silence. Verify AEC processing latency
  is below 1 ms.

### R-5.5.9a Platform AEC Delegation

The engine **SHALL** defer to native AEC on iOS and Android where platform-level AEC is available.
Custom AEC **SHALL** be used on PC and consoles where platform AEC is unavailable or insufficient.

- **Derived from:**
  [F-5.5.9](../../features/audio/voice-and-speech.md)
- **Rationale:** Mobile platforms provide optimized system-level AEC tuned for their hardware;
  deferring avoids duplicate processing and saves CPU.
- **Verification:** Integration test: verify the engine uses native AEC on iOS and Android. Verify
  custom AEC is active on PC and console platforms.

---

## User Stories

## F-5.5.1 Voice Chat Codec and Transport

## US-5.5.1.1 Configure Voice Chat Bitrate

**As an** audio designer (P-14), **I want to** configure Opus encoding bitrate between 6-64 kbps,
**so that** voice quality and bandwidth are balanced.

## US-5.5.1.2 Route Voice as 3D or Non-Spatialized Source

**As an** audio designer (P-14), **I want to** route decoded voice streams as spatialized 3D sources
(proximity chat) or non-spatialized bus sends (party channel), **so that** voice routing matches the
channel type.

## US-5.5.1.3 Set Up Voice Chat in Editor

**As a** designer (P-5), **I want to** enable and configure voice chat settings in the project
editor, **so that** voice chat is available without code.

## US-5.5.1.4 Verify Opus Encode/Decode Quality

**As an** engine tester (P-27), **I want to** encode and decode a 10s recording at 24 and 64 kbps
and verify PESQ score exceeds 3.0, **so that** voice quality meets standards.

## US-5.5.1.5 Verify End-to-End Voice Latency

**As an** engine tester (P-27), **I want to** measure capture-to-decoded-output latency and verify
it is below 40ms, **so that** voice chat latency is acceptable.

## US-5.5.1.6 Verify Platform-Native Capture APIs

**As an** engine tester (P-27), **I want to** verify microphone capture uses WASAPI (Windows),
CoreAudio (macOS), and PipeWire/ALSA (Linux), **so that** platform compliance is confirmed.

## US-5.5.1.7 Implement Voice Chat Pipeline

**As an** engine developer (P-26), **I want to** implement the voice chat pipeline (capture, Opus
encode, network transmit, decode, mixer routing) using platform-native capture APIs, **so that**
voice chat is fully integrated.

## US-5.5.1.8 Talk to Other Players In-Game

**As a** player (P-23), **I want to** talk to other players with clear voice quality, **so that**
multiplayer communication is natural.

---

## F-5.5.2 Jitter Buffer and Packet Loss Concealment

## US-5.5.2.1 Configure Jitter Buffer Parameters

**As an** audio designer (P-14), **I want to** adjust jitter buffer depth defaults per platform
(higher on mobile), **so that** buffering accounts for network variability.

## US-5.5.2.2 Verify Adaptive Jitter Buffer

**As an** engine tester (P-27), **I want to** simulate 50ms jitter and 5% packet loss and verify the
buffer adjusts depth without exceeding 80ms total, **so that** adaptation works.

## US-5.5.2.3 Verify Packet Loss Concealment

**As an** engine tester (P-27), **I want to** verify PLC fills gaps with interpolated audio (no
silence gaps exceeding one Opus frame), **so that** loss concealment is effective.

## US-5.5.2.4 Verify Voice MOS Score Under Loss

**As an** engine tester (P-27), **I want to** measure output MOS score under 50ms jitter and 5% loss
and verify it exceeds 2.5, **so that** voice quality degrades gracefully.

## US-5.5.2.5 Implement Adaptive Jitter Buffer

**As an** engine developer (P-26), **I want to** implement an adaptive jitter buffer that adjusts
depth based on observed jitter, with Opus PLC for lost packets, **so that** voice streams handle
variable network conditions.

## US-5.5.2.6 Hear Clear Voice Chat Despite Network Issues

**As a** player (P-23), **I want** voice chat to remain clear even when network conditions
fluctuate, **so that** communication is reliable.

---

## F-5.5.3 Voice Activity Detection and Noise Suppression

## US-5.5.3.1 Configure VAD Sensitivity

**As an** audio designer (P-14), **I want to** adjust VAD sensitivity thresholds, **so that**
transmission gates correctly between speech and silence.

## US-5.5.3.2 Configure Noise Suppression Strength

**As a** designer (P-5), **I want to** set noise suppression strength in the audio settings,
**so that** background noise reduction is balanced against voice quality.

## US-5.5.3.3 Verify VAD Gating

**As an** engine tester (P-27), **I want to** feed 10s silence then 5s speech into the VAD and
verify zero packets during silence and transmission within 20ms of speech onset, **so that** gating
timing is correct.

## US-5.5.3.4 Verify Noise Suppression Effectiveness

**As an** engine tester (P-27), **I want to** feed speech mixed with keyboard noise at 0 dB SNR and
verify output SNR improves by at least 20 dB, **so that** noise suppression is effective.

## US-5.5.3.5 Implement VAD and Noise Suppression

**As an** engine developer (P-26), **I want to** implement voice activity detection and noise
suppression before encoding, with lighter models on mobile, **so that** transmission is
bandwidth-efficient and clean.

## US-5.5.3.6 Not Hear Background Noise from Other Players

**As a** player (P-23), **I want** other players' keyboard clicks and fan noise to be suppressed,
**so that** voice chat is clean and intelligible.

---

## F-5.5.4 Text-to-Speech Integration

## US-5.5.4.1 Generate Speech from Text

**As a** designer (P-5), **I want to** submit text strings to TTS and have the result play as a
standard sound source, **so that** UI text and chat messages are read aloud for accessibility.

## US-5.5.4.2 Verify Platform TTS APIs

**As an** engine tester (P-27), **I want to** verify TTS uses SAPI on Windows, AVSpeechSynthesizer
on macOS, and Speech Dispatcher on Linux, **so that** platform API compliance is confirmed.

## US-5.5.4.3 Verify TTS Latency

**As an** engine tester (P-27), **I want to** submit a 20-word string and verify audio output is
generated within 500ms, **so that** TTS responsiveness is acceptable.

## US-5.5.4.4 Verify TTS Routes Through Mixer

**As an** engine tester (P-27), **I want to** verify TTS output is routable through the mixer bus
hierarchy with gain, panning, and effects, **so that** TTS integrates with the audio pipeline.

## US-5.5.4.5 Implement TTS Integration

**As an** engine developer (P-26), **I want to** implement TTS integration using platform- native
speech APIs, routing output into the mixer as standard sound sources, **so that** text- to-speech is
available on all platforms.

## US-5.5.4.6 Hear UI Text Read Aloud

**As a** player (P-23), **I want** the game to read UI text and quest descriptions aloud when
accessibility features are enabled, **so that** the game is accessible to visually impaired players.

---

## F-5.5.5 Viseme Generation for Lip Sync

## US-5.5.5.1 Generate Visemes from Dialogue Audio

**As an** audio designer (P-14), **I want to** generate viseme tracks from pre-recorded dialogue at
load time, **so that** NPC lip sync is driven by audio analysis.

## US-5.5.5.2 Generate Visemes from Live Voice Chat

**As an** audio designer (P-14), **I want to** generate visemes from live voice chat in real time,
**so that** player avatars lip-sync during voice chat.

## US-5.5.5.3 Verify Viseme Timing Accuracy

**As an** engine tester (P-27), **I want to** process dialogue with known phoneme content and verify
viseme timestamps are within 30ms of audio onset, **so that** lip sync timing is accurate.

## US-5.5.5.4 Verify Live Viseme Latency

**As an** engine tester (P-27), **I want to** feed a live voice stream and verify viseme output
latency is below 50ms, **so that** real-time lip sync is responsive.

## US-5.5.5.5 Implement Viseme Generation System

**As an** engine developer (P-26), **I want to** implement real-time phoneme-to-viseme extraction
that produces timestamped viseme tracks consumable by the animation system, **so that** lip sync is
automated.

## US-5.5.5.6 See Characters' Lips Move When They Speak

**As a** player (P-23), **I want** NPC and player character lips to move in sync with speech,
**so that** dialogue feels natural and immersive.

---

## F-5.5.6 Dialogue Playback and Queuing

## US-5.5.6.1 Configure Dialogue Priority Classes

**As an** audio designer (P-14), **I want to** assign priority classes (critical narrative, bark,
ambient chatter) to dialogue lines, **so that** important lines are never interrupted.

## US-5.5.6.2 Configure Dialogue Timeout

**As an** audio designer (P-14), **I want to** set expiry timeouts on queued dialogue, **so that**
stale barks are discarded when their context has passed.

## US-5.5.6.3 Set Up Dialogue in Editor

**As a** designer (P-5), **I want to** assign voice-over assets and subtitle text to dialogue nodes
in the editor, **so that** dialogue content is authored visually.

## US-5.5.6.4 Verify Priority Interruption

**As an** engine tester (P-27), **I want to** queue a bark then trigger a critical line and verify
the bark is interrupted and the critical line plays immediately, **so that** priority works
correctly.

## US-5.5.6.5 Verify Sequential Playback Without Overlap

**As an** engine tester (P-27), **I want to** queue 5 barks and verify they play sequentially
without overlap, **so that** queuing prevents simultaneous dialogue.

## US-5.5.6.6 Verify Subtitle Synchronization

**As an** engine tester (P-27), **I want to** verify subtitle events fire in sync with voice- over
playback within 16ms tolerance, **so that** subtitles are synchronized.

## US-5.5.6.7 Implement Dialogue Playback System

**As an** engine developer (P-26), **I want to** implement the priority queue for dialogue with
interruption, timeout expiry, and subtitle synchronization, **so that** dialogue management is
automated.

## US-5.5.6.8 Read Subtitles Matching Spoken Dialogue

**As a** player (P-23), **I want** subtitles to appear in sync with spoken dialogue, **so that** I
can follow conversations with subtitles enabled.

---

## F-5.5.7 Branching Dialogue Graph

## US-5.5.7.1 Author Branching Dialogue Trees

**As an** audio designer (P-14), **I want to** build dialogue trees as directed graphs with
condition-gated edges, random variation, and loopback nodes, **so that** NPC conversations branch
based on gameplay state.

## US-5.5.7.2 Set Up Dialogue Graphs in Editor

**As a** designer (P-5), **I want to** build dialogue graphs visually in the editor with condition
nodes gated on quest progress and reputation, **so that** branching dialogue is authored without
code.

## US-5.5.7.3 Verify Condition-Gated Branch Selection

**As an** engine tester (P-27), **I want to** set quest progress to 1 in a 3-branch graph and verify
the correct branch is selected, **so that** condition evaluation is correct.

## US-5.5.7.4 Verify Random Variation

**As an** engine tester (P-27), **I want to** verify random variation nodes produce different
selections over 100 runs, **so that** dialogue variety is confirmed.

## US-5.5.7.5 Verify Loopback Edges

**As an** engine tester (P-27), **I want to** verify loopback edges allow re-entering a conversation
node, **so that** repeatable NPC interactions work.

## US-5.5.7.6 Implement Dialogue Graph Evaluator

**As an** engine developer (P-26), **I want to** implement the dialogue graph evaluator with
condition-gated edges, random variation, and loopback, **so that** branching dialogue is
data-driven.

## US-5.5.7.7 Experience Dynamic NPC Conversations

**As a** player (P-23), **I want** NPCs to say different things based on my quest progress and
choices, **so that** conversations feel dynamic and responsive.

---

## F-5.5.8 Voice Chat Channel Management

## US-5.5.8.1 Create Channel Types

**As an** audio designer (P-14), **I want to** set up proximity, party, guild, raid, broadcast, and
custom voice channels with independent volume and routing, **so that** voice chat supports diverse
social structures.

## US-5.5.8.2 Configure Per-Channel Permissions

**As an** audio designer (P-14), **I want to** set per-channel permissions, speaker muting, and
priority levels, **so that** channel administration is flexible.

## US-5.5.8.3 Set Up Voice Channels in Editor

**As a** designer (P-5), **I want to** configure default voice channel types and settings in the
project editor, **so that** voice chat structure is defined at the project level.

## US-5.5.8.4 Verify Multi-Channel Audio Routing

**As an** engine tester (P-27), **I want to** create proximity, party, and raid channels and verify
audio from each is received with independent volume levels, **so that** channel isolation is
correct.

## US-5.5.8.5 Verify Per-Speaker Muting

**As an** engine tester (P-27), **I want to** mute a speaker in one channel and verify they remain
audible in other channels, **so that** per-channel muting is independent.

## US-5.5.8.6 Verify Administrative Controls

**As an** engine tester (P-27), **I want to** kick a player from a channel and verify they can no
longer transmit or receive on it, **so that** admin controls work.

## US-5.5.8.7 Verify Proximity Channel Distance Attenuation

**As an** engine tester (P-27), **I want to** verify the proximity channel uses distance attenuation
via the shared spatial index, **so that** nearby voices are louder.

## US-5.5.8.8 Implement Voice Channel Management

**As an** engine developer (P-26), **I want to** implement multi-channel voice management with
per-channel permissions, muting, priority, and administrative controls, **so that** voice social
features are complete.

## US-5.5.8.9 Talk in Party and Proximity Channels Simultaneously

**As a** player (P-23), **I want to** monitor party and proximity voice channels simultaneously at
different volumes, **so that** I hear both my team and nearby players.

---

## F-5.5.9 Acoustic Echo Cancellation

## US-5.5.9.1 Configure AEC Settings

**As an** audio designer (P-14), **I want to** configure AEC filter parameters and comfort noise
level, **so that** echo cancellation is tuned for the game's audio profile.

## US-5.5.9.2 Enable AEC in Audio Settings

**As a** designer (P-5), **I want to** enable AEC in the project audio settings for speaker- using
players, **so that** echo prevention is available without code.

## US-5.5.9.3 Verify Echo Return Loss Enhancement

**As an** engine tester (P-27), **I want to** play a reference signal through speakers, capture mic
input, and verify ERLE exceeds 30 dB, **so that** echo cancellation is effective.

## US-5.5.9.4 Verify Adaptive Filter Reconvergence

**As an** engine tester (P-27), **I want to** change room acoustics and verify the filter re-
converges within 2 seconds, **so that** AEC adapts to environment changes.

## US-5.5.9.5 Verify Comfort Noise Generation

**As an** engine tester (P-27), **I want to** verify comfort noise is present during echo- cancelled
silence (not digital silence), **so that** cancelled gaps sound natural.

## US-5.5.9.6 Verify AEC Processing Latency

**As an** engine tester (P-27), **I want to** verify AEC processing latency is below 1ms on the
audio thread, **so that** AEC does not introduce perceptible delay.

## US-5.5.9.7 Verify Platform AEC Delegation

**As an** engine tester (P-27), **I want to** verify the engine defers to native AEC on iOS and
Android where available, **so that** platform-optimal AEC is used.

## US-5.5.9.8 Implement AEC Module

**As an** engine developer (P-26), **I want to** implement acoustic echo cancellation with adaptive
filtering, non-linear processing, and comfort noise, running on the audio thread with sub-ms
latency, **so that** speaker-using players do not echo.

## US-5.5.9.9 Not Hear Echo When Players Use Speakers

**As a** player (P-23), **I want** voice chat to be echo-free even when other players use speakers
instead of headphones, **so that** conversations are clear.

---

## Non-Functional Requirements

### R-5.5.NF1 Voice Chat End-to-End Latency

The engine **SHALL** deliver voice chat audio with end-to-end latency not exceeding 150 ms under
normal network conditions (< 50 ms RTT, < 1% packet loss).

- **Derived from:** F-5.5.1, F-5.5.2
- **Rationale:** Voice chat latency above 150 ms causes conversational overlap and frustration.
- **Verification:** Integration test: measure time from microphone input to decoded output on a
  loopback with 50 ms simulated RTT. Assert total latency is below 150 ms.

### R-5.5.NF2 Simultaneous Voice Stream Capacity

The engine **SHALL** decode and mix at least 32 simultaneous incoming voice streams without
exceeding the audio thread budget or causing artifacts.

- **Derived from:** F-5.5.8, R-5.1.NF1
- **Rationale:** Large raid groups may have multiple participants transmitting simultaneously.
- **Verification:** Stress test: feed 32 concurrent Opus streams at 24 kbps. Assert no audio
  underruns over 60 seconds.
