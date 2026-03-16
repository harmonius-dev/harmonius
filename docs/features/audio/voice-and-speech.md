# 5.5 — Voice & Speech

## Voice Chat

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.5.1 | Voice Chat Codec and Transport | Capture microphone input, encode with Opus at configurable bitrates (6-64 kbps), and transmit voice packets over the engine's networking layer with minimal added latency. Decode incoming streams and route them into the mixer bus hierarchy as spatialized 3D sources (proximity chat) or non-spatialized bus sends (party/raid channels). | R-5.5.1 | F-5.1.3, F-5.1.7 | Microphone capture uses platform-native APIs: WASAPI on Windows, CoreAudio/AVAudioEngine on macOS, and PipeWire/ALSA on Linux. |
| F-5.5.2 | Jitter Buffer and Packet Loss Concealment | Buffer incoming voice packets in an adaptive jitter buffer that dynamically adjusts its depth based on observed network jitter, balancing latency against dropout risk. When packets are lost or arrive too late, apply packet-loss concealment (PLC) via Opus's built-in decoder interpolation to mask gaps. | R-5.5.2 | F-5.5.1 | Jitter buffer depth default is higher on mobile (accounts for variable network conditions on cellular). PLC quality is uniform (Opus built-in). |
| F-5.5.3 | Voice Activity Detection and Noise Suppression | Detect speech onset and offset using a lightweight voice activity detector (VAD) to gate transmission, reducing upstream bandwidth and mixer load when a player is not speaking. Pair VAD with a noise suppression filter that attenuates keyboard clicks, fans, and background noise before encoding. | R-5.5.3 | F-5.5.1 | VAD and noise suppression run on all platforms. Mobile uses lighter noise suppression model to reduce CPU overhead. Platform-native noise suppression used where available (iOS, Android). |

## Text-to-Speech

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.5.4 | Text-to-Speech Integration | Generate speech audio from text strings using platform TTS services, feeding the result into the mixer as a standard sound source. TTS serves accessibility needs (reading UI text, quest descriptions, chat messages aloud) and can provide placeholder dialogue during development before voice-over recordings are available. | R-5.5.4 | F-5.1.1, F-5.1.3 | Uses SAPI / Windows.Media.SpeechSynthesis on Windows, AVSpeechSynthesizer on macOS/iOS, and Speech Dispatcher on Linux. |

## Lip Sync

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.5.5 | Viseme Generation for Lip Sync | Analyze audio streams in real time to extract phoneme-to-viseme mappings that drive character facial animation blend shapes. The viseme generator operates on both pre-recorded dialogue (offline or at load time) and live voice chat streams (real time), enabling lip-synced NPCs and player avatars alike. Output is a timestamped viseme track consumed by the animation system. | R-5.5.5 | F-5.1.1 | Viseme generation runs on all platforms. Active lip-synced character count: mobile 1-2, Switch 4, desktop 8+. Distant characters use random mouth animation on mobile. |

## Dialogue System

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.5.6 | Dialogue Playback and Queuing | Play authored voice-over lines with subtitle synchronization, managing a priority queue that prevents overlapping dialogue. Lines declare priority (critical narrative, bark, ambient chatter) and the system interrupts or defers lower-priority lines when a higher-priority line is triggered. Queued lines expire after a configurable timeout. | R-5.5.6 | F-5.1.1, F-5.1.4, F-5.5.5 | Dialogue queue depth uniform across platforms. Voice-over streaming draws from voice pool budget (see F-5.1.4). |
| F-5.5.7 | Branching Dialogue Graph | Represent dialogue trees as directed graphs with condition-gated edges evaluated against gameplay state (quest progress, reputation, inventory). The graph supports branching choices, random variation nodes, and loopback edges for repeatable NPC interactions. Each node references a voice-over asset and subtitle text. | R-5.5.7 | F-5.5.6 | Dialogue graph evaluation is lightweight CPU-side on all platforms. No platform-specific scaling required. |

## Voice Chat Channels

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.5.8 | Voice Chat Channel Management | Manage multiple concurrent voice chat channels with per-channel permissions and routing. Channel types: proximity, party, guild, raid, broadcast, and custom. Each channel supports independent volume, muting per-speaker, priority levels, and administrative controls (kick, mute, ban). Players can monitor multiple channels simultaneously. Channel membership is replicated through the networking system (F-8.2.1). | R-5.5.8 | F-5.5.1 (Voice Chat Codec and Transport), F-5.5.3 (Voice Activity Detection), F-8.2.1 (Property Replication) | Console platforms require using platform-native voice chat APIs for party channels (PlayStation Party, Xbox Party Chat). Game-managed channels use the engine's transport. |

## Acoustic Echo Cancellation

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.5.9 | Acoustic Echo Cancellation | Real-time acoustic echo cancellation (AEC) for players using speakers instead of headphones. The AEC module subtracts the known speaker output signal from the microphone input. Supports both linear adaptive filtering (NLMS) and non-linear processing for handling loudspeaker distortion. A comfort noise generator fills gaps left by echo removal. AEC processing runs on the audio thread with sub-millisecond latency. | R-5.5.9 | F-5.5.1 (Voice Chat Codec and Transport), F-5.5.3 (Voice Activity Detection) | On platforms with native AEC (iOS, Android system-level AEC), the engine defers to the platform implementation. Custom AEC is used on PC and consoles where platform AEC is unavailable or insufficient. |
