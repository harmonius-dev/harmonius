# 5.1 — Audio Engine

## Sound Sources

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.1.1 | Sound Source Component | Attach point, line, and area sound emitters to entities via an ECS component, each carrying gain, pitch, looping, and attenuation references. Lightweight component essential for hundreds of ambient emitters (campfires, waterfalls, NPC chatter). | R-5.1.1 | None | Lightweight on all platforms. Active emitter count governed by voice management budget (see F-5.1.4). |
| F-5.1.2 | Listener Component | Designate one or more listener entities defining the player's auditory perspective, including position, orientation, and velocity for Doppler. Multiple listeners support split-screen and spectator modes; default assigns to the active camera. | R-5.1.2 | F-5.1.1 | Lightweight on all platforms. Multiple listeners supported on all tiers (split-screen may be limited on mobile by voice budget). |

## Mixer Bus Hierarchy

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.1.3 | Hierarchical Mixer Bus Graph | DAG of mixer buses (master, music, SFX, ambient, voice, UI) with gain, mute, solo, and insert effect chains. Child buses inherit parent gain. Buses can be added or rewired at runtime for dynamic mix states (underwater, pause-menu ducking). | R-5.1.3 | None | Hierarchy depth uniform across platforms. Insert effect chain length scales per tier (see F-5.3.8). Mobile limits total insert count. |

## Voice Management

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.1.4 | Voice Management and Priority System | Fixed pool of voices with priority-based allocation, virtualization, and stealing. Each source declares a priority class and audibility score from distance and occlusion. Lowest-scoring voices are virtualized and restored seamlessly when headroom returns. | R-5.1.4 | F-5.1.1, F-5.1.3 | Voice pool size scales per tier: mobile 16-32, Switch 32-64, desktop 128-256. Virtualization threshold adjusted per tier. |

## Playback

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.1.5 | Streaming Playback | Stream long-duration audio (music, ambience, dialogue) from disk in ring-buffer chunks using platform-native async I/O. Prefetch hinting begins streaming before playback, eliminating startup latency for cinematic cues and zone transitions. | R-5.1.5 | F-5.1.1 | Uses IOCP on Windows, GCD async I/O on macOS, io_uring on Linux. |
| F-5.1.6 | Sample-Accurate Scheduling | Schedule sound start, stop, and parameter changes at precise sample offsets within the audio buffer, enabling tight synchronization between layered loops, musical cues, and gameplay events. Lock-free SPSC command queue between game thread and audio thread. | R-5.1.6 | F-5.1.1, F-5.1.3 | Supported on all platforms. Audio buffer size may be larger on mobile (higher latency) to reduce CPU wake-ups. |

## Formats and Codecs

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.1.7 | Audio Format and Codec Support | Decode PCM (WAV), Vorbis, Opus, and FLAC at load or stream time, with an extensible codec registry. Opus for voice chat, Vorbis for legacy assets, FLAC for lossless reference audio. Format metadata extracted during asset import and cached. | R-5.1.7 | F-5.1.5 | Platform hardware decoders (e.g., Apple Audio Toolbox) used opportunistically but never required. |
