# User Stories — 5.1 Audio Engine

## US-5.1.1.1 Attach Point Sound Source to Entity

**As an** audio designer (P-14), **I want to** attach a point sound emitter to any entity via a
lightweight ECS component, **so that** campfires, torches, and NPCs emit sounds.

## US-5.1.1.2 Attach Line Sound Source for Rivers

**As an** audio designer (P-14), **I want to** attach line-shaped emitters along rivers and streams,
**so that** running water sounds natural along its length.

## US-5.1.1.3 Attach Area Sound Source for Ambience

**As an** audio designer (P-14), **I want to** attach area emitters for ambient zones (forests,
markets), **so that** ambient sound fills a region.

## US-5.1.1.4 Implement Sound Source Component

**As an** engine developer (P-26), **I want to** implement a SoundSource component carrying gain,
pitch, looping, and attenuation, **so that** emitters are lightweight ECS data.

## US-5.1.1.5 Hear Ambient Sounds from World Objects

**As a** player (P-23), **I want to** hear campfires crackle and waterfalls roar when near them,
**so that** the world sounds alive.

## US-5.1.1.6 Verify Component Overhead is Minimal

**As an** engine tester (P-27), **I want to** verify SoundSource component overhead is negligible
per entity, **so that** hundreds of emitters are feasible.

## US-5.1.1.7 Configure Gain and Pitch Per Source

**As an** audio designer (P-14), **I want to** set gain and pitch per sound source, **so that**
individual emitters are tunable without code.

## US-5.1.1.8 Set Looping Per Sound Source

**As a** designer (P-5), **I want to** enable looping on ambient sound sources, **so that**
continuous environmental sounds play indefinitely.

## US-5.1.1.9 Test Multiple Simultaneous Emitters

**As an** engine tester (P-27), **I want to** test 200+ simultaneous emitters, **so that** dense
open-world scenes are validated.

## US-5.1.1.10 Hear Distinct Sounds from Different Sources

**As a** player (P-23), **I want to** hear distinct sounds from different nearby objects,
**so that** I can identify sources by ear.

## US-5.1.1.11 Configure Attenuation References Per Source

**As an** audio designer (P-14), **I want to** set attenuation curve references per source,
**so that** each emitter has appropriate distance rolloff.

## US-5.1.1.12 Assign Sound Sources in Visual Editor

**As a** designer (P-5), **I want to** assign sound source components in the visual editor,
**so that** audio is set up without code.

## US-5.1.2.1 Designate Listener Entity

**As an** audio designer (P-14), **I want to** designate one or more listener entities, **so that**
the player's auditory perspective is defined.

## US-5.1.2.2 Implement Listener Component

**As an** engine developer (P-26), **I want to** implement a Listener component with position,
orientation, and velocity, **so that** spatial audio has a reference point.

## US-5.1.2.3 Support Multiple Listeners for Split-Screen

**As an** engine developer (P-26), **I want** multiple listeners for split-screen and spectator
modes, **so that** each view has its own audio perspective.

## US-5.1.2.4 Hear Audio from My Camera Perspective

**As a** player (P-23), **I want** audio positioned relative to my camera, **so that** sounds come
from the correct direction.

## US-5.1.2.5 Default Listener to Active Camera

**As a** designer (P-5), **I want** the default listener assigned to the active camera, **so that**
audio perspective matches visual perspective automatically.

## US-5.1.2.6 Verify Listener Velocity for Doppler

**As an** engine tester (P-27), **I want to** verify listener velocity is used for Doppler
calculations, **so that** moving listener produces correct pitch shifts.

## US-5.1.2.7 Test Multiple Listeners Simultaneously

**As an** engine tester (P-27), **I want to** test multiple listeners in split-screen mode,
**so that** each listener has independent spatial audio.

## US-5.1.2.8 Configure Listener Per Player

**As an** audio designer (P-14), **I want** each player in split-screen to have an independent
listener, **so that** audio perspective matches each player's view.

## US-5.1.2.9 Support Spectator Listener

**As a** designer (P-5), **I want** spectator cameras to have their own listener, **so that**
spectators hear from their perspective.

## US-5.1.2.10 Verify Listener Position Updates Per Frame

**As an** engine tester (P-27), **I want to** verify listener position updates every frame,
**so that** spatial audio stays synchronized with visuals.

## US-5.1.2.11 Hear Correct Panning When Camera Moves

**As a** player (P-23), **I want** sound panning to update as I move and rotate the camera,
**so that** audio tracking is smooth.

## US-5.1.2.12 Assign Listener in Visual Editor

**As a** designer (P-5), **I want to** assign listener components in the visual editor, **so that**
listener configuration is code-free.

## US-5.1.3.1 Define Mixer Bus Hierarchy

**As an** audio designer (P-14), **I want to** define a hierarchy of mixer buses (master, music,
SFX, ambient, voice, UI), **so that** audio is organized by category.

## US-5.1.3.2 Implement Bus Gain Inheritance

**As an** engine developer (P-26), **I want** child buses to inherit parent gain, **so that** global
volume control works hierarchically.

## US-5.1.3.3 Insert Effects on Buses

**As an** audio designer (P-14), **I want to** insert effect chains on any bus, **so that**
per-category processing is possible.

## US-5.1.3.4 Mute Music and Hear Only SFX

**As a** player (P-23), **I want to** mute the music bus independently, **so that** I can play with
only sound effects.

## US-5.1.3.5 Add Buses at Runtime

**As an** engine developer (P-26), **I want** buses added and rewired at runtime, **so that**
dynamic mix states (underwater, pause menu) are supported.

## US-5.1.3.6 Implement Mute and Solo Per Bus

**As an** audio designer (P-14), **I want** mute and solo controls per bus, **so that** I can
isolate categories during mixing.

## US-5.1.3.7 Configure Bus Hierarchy in Visual Editor

**As a** designer (P-5), **I want to** configure the mixer bus graph in the visual editor,
**so that** audio routing is visual.

## US-5.1.3.8 Verify Bus Hierarchy Depth Across Platforms

**As an** engine tester (P-27), **I want to** verify mixer bus hierarchy depth is uniform across
platforms, **so that** audio structure is consistent.

## US-5.1.3.9 Duck SFX During Dialogue

**As an** audio designer (P-14), **I want to** duck the SFX bus when dialogue plays, **so that**
speech is always intelligible.

## US-5.1.3.10 Test Insert Effect Chain Limits

**As an** engine tester (P-27), **I want to** verify insert effect chain length scales per tier
(mobile limited), **so that** DSP budget is controlled.

## US-5.1.3.11 Adjust Individual Bus Volumes

**As a** player (P-23), **I want** separate volume sliders for music, SFX, voice, and ambient,
**so that** I can customize my audio mix.

## US-5.1.3.12 Wire Buses as Directed Acyclic Graph

**As an** engine developer (P-26), **I want** buses wired as a DAG, **so that** complex routing
(send buses, sub-mixes) is possible without cycles.

## US-5.1.4.1 Manage Fixed Voice Pool with Priority

**As an** engine developer (P-26), **I want to** manage a fixed pool of voices with priority-based
allocation, **so that** critical sounds always play.

## US-5.1.4.2 Declare Priority Class Per Sound Source

**As an** audio designer (P-14), **I want to** declare a priority class (critical, high, medium,
low) per sound source, **so that** importance is ranked.

## US-5.1.4.3 Virtualize Low-Priority Voices

**As an** engine developer (P-26), **I want** lowest-scoring voices virtualized (tracked but silent)
when the pool is exceeded, **so that** sounds resume when headroom returns.

## US-5.1.4.4 Hear All Important Sounds in Combat

**As a** player (P-23), **I want** critical sounds (alerts, dialogue) to always play even in noisy
combat, **so that** important audio is never missed.

## US-5.1.4.5 Compute Audibility Score from Distance and Occlusion

**As an** engine developer (P-26), **I want** audibility scores derived from distance and occlusion,
**so that** voice stealing prioritizes audible sounds.

## US-5.1.4.6 Verify Voice Pool Sizes Per Platform

**As an** engine tester (P-27), **I want to** confirm voice pool scales per tier (mobile 16-32,
Switch 32-64, desktop 128-256), **so that** platform limits are enforced.

## US-5.1.4.7 Restore Virtualized Voices Seamlessly

**As an** engine developer (P-26), **I want** virtualized voices restored seamlessly when headroom
returns, **so that** the player hears no audible gaps.

## US-5.1.4.8 Configure Virtualization Threshold

**As an** audio designer (P-14), **I want to** configure the virtualization threshold, **so that**
the audibility cutoff is tunable.

## US-5.1.4.9 Test Voice Stealing Under Maximum Load

**As an** engine tester (P-27), **I want to** test voice stealing with all pool slots filled,
**so that** priority ordering works correctly under pressure.

## US-5.1.4.10 Hear Critical Alert Over Background Noise

**As a** player (P-23), **I want** critical alert sounds to play over ambient noise, **so that** I
am always warned of important events.

## US-5.1.4.11 Adjust Virtualization Per Platform Tier

**As a** designer (P-5), **I want** virtualization thresholds adjusted per platform tier,
**so that** more sounds play on capable hardware.

## US-5.1.4.12 Test Seamless Voice Restoration

**As an** engine tester (P-27), **I want to** test voice devirtualization and confirm seamless
restoration, **so that** returning sounds have no audible pop.

## US-5.1.5.1 Stream Long Audio from Disk

**As an** engine developer (P-26), **I want to** stream long-duration audio in ring-buffer chunks
using platform-native async I/O, **so that** large files need not load fully.

## US-5.1.5.2 Prefetch Audio Before Playback

**As an** audio designer (P-14), **I want** prefetch hinting so streaming begins before playback
triggers, **so that** startup latency is eliminated for cinematic cues.

## US-5.1.5.3 Hear Music Without Startup Gaps

**As a** player (P-23), **I want** music and ambience to start instantly without audible loading
gaps, **so that** audio transitions are seamless.

## US-5.1.5.4 Use Platform-Native Async I/O

**As an** engine developer (P-26), **I want** streaming to use IOCP on Windows, GCD async I/O on
macOS, and io_uring on Linux, **so that** I/O is platform-optimal.

## US-5.1.5.5 Verify Streaming Chunk Size

**As an** engine tester (P-27), **I want to** verify ring-buffer chunk sizes prevent underruns
during playback, **so that** streaming is gap-free.

## US-5.1.5.6 Stream Dialogue Lines On Demand

**As an** audio designer (P-14), **I want** dialogue lines streamed on demand without preloading,
**so that** memory is conserved for large VO databases.

## US-5.1.5.7 Configure Prefetch Timing Per Asset

**As a** designer (P-5), **I want to** configure prefetch timing per audio asset, **so that**
important cues start streaming early.

## US-5.1.5.8 Test Streaming Under I/O Contention

**As an** engine tester (P-27), **I want to** test audio streaming under heavy I/O load, **so that**
streaming remains glitch-free during asset loading.

## US-5.1.5.9 Hear Ambient Loops Without Gaps

**As a** player (P-23), **I want** ambient loops to stream continuously without gaps, **so that**
background atmosphere is unbroken.

## US-5.1.5.10 Support Simultaneous Streams

**As an** engine developer (P-26), **I want** the streaming system to support multiple concurrent
streams within the voice budget, **so that** music and dialogue stream together.

## US-5.1.5.11 Configure Ring Buffer Size Per Platform

**As a** designer (P-5), **I want to** set ring buffer sizes per platform, **so that** memory usage
matches device capability.

## US-5.1.5.12 Verify No Underruns During Zone Transitions

**As an** engine tester (P-27), **I want to** verify streaming does not underrun during zone
transitions, **so that** audio remains continuous during loading.

## US-5.1.6.1 Schedule Sound Start at Precise Sample

**As an** audio designer (P-14), **I want to** schedule sound start at a precise sample offset,
**so that** layered loops sync perfectly.

## US-5.1.6.2 Schedule Parameter Changes at Sample Offset

**As an** engine developer (P-26), **I want** parameter changes scheduled at sample-accurate
offsets, **so that** gain, pitch, and filter changes are click-free.

## US-5.1.6.3 Queue Commands from Game Thread

**As an** engine developer (P-26), **I want** audio commands queued on the game thread and executed
on the audio thread at the precise sample, **so that** timing is exact.

## US-5.1.6.4 Hear Perfectly Synchronized Layers

**As a** player (P-23), **I want** layered music stems to be perfectly in sync, **so that** music
sounds cohesive.

## US-5.1.6.5 Synchronize Gameplay Events with Audio

**As an** audio designer (P-14), **I want** gameplay events to trigger sounds at exact timing,
**so that** audio-visual sync is tight.

## US-5.1.6.6 Verify Sample-Accurate Timing

**As an** engine tester (P-27), **I want to** verify scheduled events fire at the exact sample
offset, **so that** timing accuracy is validated.

## US-5.1.6.7 Configure Audio Buffer Size Per Platform

**As a** designer (P-5), **I want** audio buffer size configurable per platform, **so that**
latency-battery tradeoffs are tunable.

## US-5.1.6.8 Schedule Sound Stop at Precise Sample

**As an** audio designer (P-14), **I want** sound stops scheduled at precise samples, **so that**
loops end cleanly without artifacts.

## US-5.1.6.9 Test Sample Accuracy Under Load

**As an** engine tester (P-27), **I want to** test scheduling accuracy under heavy audio load,
**so that** timing remains precise even with many voices.

## US-5.1.6.10 Sync Musical Cues to Gameplay

**As a** designer (P-5), **I want** musical cues to trigger at exact gameplay moments, **so that**
cinematic moments have perfectly timed audio.

## US-5.1.6.11 Hear Tight Audio-Visual Sync

**As a** player (P-23), **I want** sound effects to match visual events exactly, **so that**
impacts, footsteps, and abilities feel connected.

## US-5.1.6.12 Verify Cross-Thread Command Delivery

**As an** engine tester (P-27), **I want to** verify game-thread commands arrive on the audio thread
within one buffer latency, **so that** cross-thread timing is reliable.

## US-5.1.7.1 Decode PCM WAV Files

**As an** engine developer (P-26), **I want to** decode PCM WAV files at load time, **so that**
uncompressed reference audio is supported.

## US-5.1.7.2 Decode Opus for Voice Chat

**As an** engine developer (P-26), **I want to** decode Opus streams in real time, **so that** voice
chat and bandwidth-constrained streaming work.

## US-5.1.7.3 Decode Vorbis for Legacy Assets

**As an** engine developer (P-26), **I want to** decode Vorbis for legacy assets, **so that**
existing content libraries are supported.

## US-5.1.7.4 Decode FLAC for Lossless Audio

**As an** audio designer (P-14), **I want** FLAC support for lossless reference audio, **so that**
high-fidelity recordings are available.

## US-5.1.7.5 Register Custom Codecs via Plugin

**As an** engine developer (P-26), **I want** the codec registry to be extensible via plugins,
**so that** custom or proprietary codecs can be added.

## US-5.1.7.6 Hear High-Quality Audio Regardless of Format

**As a** player (P-23), **I want** all audio to play at high quality, **so that** the underlying
format is transparent to me.

## US-5.1.7.7 Extract Format Metadata During Import

**As an** engine developer (P-26), **I want** sample rate, channel count, and loop points extracted
during asset import, **so that** playback setup is fast.

## US-5.1.7.8 Use Platform Hardware Decoders Opportunistically

**As an** engine developer (P-26), **I want** platform-supplied hardware decoders used
opportunistically (Apple Audio Toolbox), **so that** CPU is saved where available.

## US-5.1.7.9 Verify Codec Support Across Platforms

**As an** engine tester (P-27), **I want to** verify all codecs decode correctly on all platforms,
**so that** format support is cross-platform.

## US-5.1.7.10 Import Audio in Multiple Formats

**As an** audio designer (P-14), **I want to** import audio in WAV, Vorbis, Opus, and FLAC formats,
**so that** I can use whichever format suits the asset.

## US-5.1.7.11 Configure Preferred Codec Per Asset Type

**As a** designer (P-5), **I want to** configure preferred codec per asset type, **so that** each
category uses optimal encoding.

## US-5.1.7.12 Test Loop Point Accuracy Per Format

**As an** engine tester (P-27), **I want to** verify loop points decode correctly in each format,
**so that** seamless looping works regardless of codec.
