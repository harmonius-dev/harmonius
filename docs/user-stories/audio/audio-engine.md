# User Stories — 5.1 Audio Engine

## F-5.1.1

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.1.1.1 | audio designer (P-14) | I want to attach a point sound emitter to any entity via a lightweight ECS component | campfires, torches, and NPCs emit sounds | F-5.1.1 | R-5.1.1 |
| US-5.1.1.2 | audio designer (P-14) | I want to attach line-shaped emitters along rivers and streams | running water sounds natural along its length | F-5.1.1 | R-5.1.1 |
| US-5.1.1.3 | audio designer (P-14) | I want to attach area emitters for ambient zones (forests, markets) | ambient sound fills a region | F-5.1.1 | R-5.1.1 |
| US-5.1.1.4 | engine developer (P-26) | I want to implement a SoundSource component carrying gain, pitch, looping, and attenuation | emitters are lightweight ECS data | F-5.1.1 | R-5.1.1 |
| US-5.1.1.5 | player (P-23) | I want to hear campfires crackle and waterfalls roar when near them | the world sounds alive | F-5.1.1 | R-5.1.1 |
| US-5.1.1.6 | engine tester (P-27) | I want to verify SoundSource component overhead is negligible per entity | hundreds of emitters are feasible | F-5.1.1 | R-5.1.1 |
| US-5.1.1.7 | audio designer (P-14) | I want to set gain and pitch per sound source | individual emitters are tunable without code | F-5.1.1 | R-5.1.1 |
| US-5.1.1.8 | designer (P-5) | I want to enable looping on ambient sound sources | continuous environmental sounds play indefinitely | F-5.1.1 | R-5.1.1 |
| US-5.1.1.9 | engine tester (P-27) | I want to test 200+ simultaneous emitters | dense open-world scenes are validated | F-5.1.1 | R-5.1.1 |
| US-5.1.1.10 | player (P-23) | I want to hear distinct sounds from different nearby objects | I can identify sources by ear | F-5.1.1 | R-5.1.1 |
| US-5.1.1.11 | audio designer (P-14) | I want to set attenuation curve references per source | each emitter has appropriate distance rolloff | F-5.1.1 | R-5.1.1 |
| US-5.1.1.12 | designer (P-5) | I want to assign sound source components in the visual editor | audio is set up without code | F-5.1.1 | R-5.1.1 |

## F-5.1.2

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.1.2.1 | audio designer (P-14) | I want to designate one or more listener entities | the player's auditory perspective is defined | F-5.1.2 | R-5.1.2 |
| US-5.1.2.2 | engine developer (P-26) | I want to implement a Listener component with position, orientation, and velocity | spatial audio has a reference point | F-5.1.2 | R-5.1.2 |
| US-5.1.2.3 | engine developer (P-26) | I want multiple listeners for split-screen and spectator modes | each view has its own audio perspective | F-5.1.2 | R-5.1.2 |
| US-5.1.2.4 | player (P-23) | I want audio positioned relative to my camera | sounds come from the correct direction | F-5.1.2 | R-5.1.2 |
| US-5.1.2.5 | designer (P-5) | I want the default listener assigned to the active camera | audio perspective matches visual perspective automatically | F-5.1.2 | R-5.1.2 |
| US-5.1.2.6 | engine tester (P-27) | I want to verify listener velocity is used for Doppler calculations | moving listener produces correct pitch shifts | F-5.1.2 | R-5.1.2 |
| US-5.1.2.7 | engine tester (P-27) | I want to test multiple listeners in split-screen mode | each listener has independent spatial audio | F-5.1.2 | R-5.1.2 |
| US-5.1.2.8 | audio designer (P-14) | I want each player in split-screen to have an independent listener | audio perspective matches each player's view | F-5.1.2 | R-5.1.2 |
| US-5.1.2.9 | designer (P-5) | I want spectator cameras to have their own listener | spectators hear from their perspective | F-5.1.2 | R-5.1.2 |
| US-5.1.2.10 | engine tester (P-27) | I want to verify listener position updates every frame | spatial audio stays synchronized with visuals | F-5.1.2 | R-5.1.2 |
| US-5.1.2.11 | player (P-23) | I want sound panning to update as I move and rotate the camera | audio tracking is smooth | F-5.1.2 | R-5.1.2 |
| US-5.1.2.12 | designer (P-5) | I want to assign listener components in the visual editor | listener configuration is code-free | F-5.1.2 | R-5.1.2 |

## F-5.1.3

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.1.3.1 | audio designer (P-14) | I want to define a hierarchy of mixer buses (master, music, SFX, ambient, voice, UI) | audio is organized by category | F-5.1.3 | R-5.1.3 |
| US-5.1.3.2 | engine developer (P-26) | I want child buses to inherit parent gain | global volume control works hierarchically | F-5.1.3 | R-5.1.3 |
| US-5.1.3.3 | audio designer (P-14) | I want to insert effect chains on any bus | per-category processing is possible | F-5.1.3 | R-5.1.3 |
| US-5.1.3.4 | player (P-23) | I want to mute the music bus independently | I can play with only sound effects | F-5.1.3 | R-5.1.3 |
| US-5.1.3.5 | engine developer (P-26) | I want buses added and rewired at runtime | dynamic mix states (underwater, pause menu) are supported | F-5.1.3 | R-5.1.3 |
| US-5.1.3.6 | audio designer (P-14) | I want mute and solo controls per bus | I can isolate categories during mixing | F-5.1.3 | R-5.1.3 |
| US-5.1.3.7 | designer (P-5) | I want to configure the mixer bus graph in the visual editor | audio routing is visual | F-5.1.3 | R-5.1.3 |
| US-5.1.3.8 | engine tester (P-27) | I want to verify mixer bus hierarchy depth is uniform across platforms | audio structure is consistent | F-5.1.3 | R-5.1.3 |
| US-5.1.3.9 | audio designer (P-14) | I want to duck the SFX bus when dialogue plays | speech is always intelligible | F-5.1.3 | R-5.1.3 |
| US-5.1.3.10 | engine tester (P-27) | I want to verify insert effect chain length scales per tier (mobile limited) | DSP budget is controlled | F-5.1.3 | R-5.1.3 |
| US-5.1.3.11 | player (P-23) | I want separate volume sliders for music, SFX, voice, and ambient | I can customize my audio mix | F-5.1.3 | R-5.1.3 |
| US-5.1.3.12 | engine developer (P-26) | I want buses wired as a DAG | complex routing (send buses, sub-mixes) is possible without cycles | F-5.1.3 | R-5.1.3 |

## F-5.1.4

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.1.4.1 | engine developer (P-26) | I want to manage a fixed pool of voices with priority-based allocation | critical sounds always play | F-5.1.4 | R-5.1.4 |
| US-5.1.4.2 | audio designer (P-14) | I want to declare a priority class (critical, high, medium, low) per sound source | importance is ranked | F-5.1.4 | R-5.1.4 |
| US-5.1.4.3 | engine developer (P-26) | I want lowest-scoring voices virtualized (tracked but silent) when the pool is exceeded | sounds resume when headroom returns | F-5.1.4 | R-5.1.4 |
| US-5.1.4.4 | player (P-23) | I want critical sounds (alerts, dialogue) to always play even in noisy combat | important audio is never missed | F-5.1.4 | R-5.1.4 |
| US-5.1.4.5 | engine developer (P-26) | I want audibility scores derived from distance and occlusion | voice stealing prioritizes audible sounds | F-5.1.4 | R-5.1.4 |
| US-5.1.4.6 | engine tester (P-27) | I want to confirm voice pool scales per tier (mobile 16-32, Switch 32-64, desktop 128-256) | platform limits are enforced | F-5.1.4 | R-5.1.4 |
| US-5.1.4.7 | engine developer (P-26) | I want virtualized voices restored seamlessly when headroom returns | the player hears no audible gaps | F-5.1.4 | R-5.1.4 |
| US-5.1.4.8 | audio designer (P-14) | I want to configure the virtualization threshold | the audibility cutoff is tunable | F-5.1.4 | R-5.1.4 |
| US-5.1.4.9 | engine tester (P-27) | I want to test voice stealing with all pool slots filled | priority ordering works correctly under pressure | F-5.1.4 | R-5.1.4 |
| US-5.1.4.10 | player (P-23) | I want critical alert sounds to play over ambient noise | I am always warned of important events | F-5.1.4 | R-5.1.4 |
| US-5.1.4.11 | designer (P-5) | I want virtualization thresholds adjusted per platform tier | more sounds play on capable hardware | F-5.1.4 | R-5.1.4 |
| US-5.1.4.12 | engine tester (P-27) | I want to test voice devirtualization and confirm seamless restoration | returning sounds have no audible pop | F-5.1.4 | R-5.1.4 |

## F-5.1.5

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.1.5.1 | engine developer (P-26) | I want to stream long-duration audio in ring-buffer chunks using platform-native async I/O | large files need not load fully | F-5.1.5 | R-5.1.5 |
| US-5.1.5.2 | audio designer (P-14) | I want prefetch hinting so streaming begins before playback triggers | startup latency is eliminated for cinematic cues | F-5.1.5 | R-5.1.5 |
| US-5.1.5.3 | player (P-23) | I want music and ambience to start instantly without audible loading gaps | audio transitions are seamless | F-5.1.5 | R-5.1.5 |
| US-5.1.5.4 | engine developer (P-26) | I want streaming to use IOCP on Windows, GCD async I/O on macOS, and io_uring on Linux | I/O is platform-optimal | F-5.1.5 | R-5.1.5 |
| US-5.1.5.5 | engine tester (P-27) | I want to verify ring-buffer chunk sizes prevent underruns during playback | streaming is gap-free | F-5.1.5 | R-5.1.5 |
| US-5.1.5.6 | audio designer (P-14) | I want dialogue lines streamed on demand without preloading | memory is conserved for large VO databases | F-5.1.5 | R-5.1.5 |
| US-5.1.5.7 | designer (P-5) | I want to configure prefetch timing per audio asset | important cues start streaming early | F-5.1.5 | R-5.1.5 |
| US-5.1.5.8 | engine tester (P-27) | I want to test audio streaming under heavy I/O load | streaming remains glitch-free during asset loading | F-5.1.5 | R-5.1.5 |
| US-5.1.5.9 | player (P-23) | I want ambient loops to stream continuously without gaps | background atmosphere is unbroken | F-5.1.5 | R-5.1.5 |
| US-5.1.5.10 | engine developer (P-26) | I want the streaming system to support multiple concurrent streams within the voice budget | music and dialogue stream together | F-5.1.5 | R-5.1.5 |
| US-5.1.5.11 | designer (P-5) | I want to set ring buffer sizes per platform | memory usage matches device capability | F-5.1.5 | R-5.1.5 |
| US-5.1.5.12 | engine tester (P-27) | I want to verify streaming does not underrun during zone transitions | audio remains continuous during loading | F-5.1.5 | R-5.1.5 |

## F-5.1.6

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.1.6.1 | audio designer (P-14) | I want to schedule sound start at a precise sample offset | layered loops sync perfectly | F-5.1.6 | R-5.1.6 |
| US-5.1.6.2 | engine developer (P-26) | I want parameter changes scheduled at sample-accurate offsets | gain, pitch, and filter changes are click-free | F-5.1.6 | R-5.1.6 |
| US-5.1.6.3 | engine developer (P-26) | I want audio commands queued on the game thread and executed on the audio thread at the precise sample | timing is exact | F-5.1.6 | R-5.1.6 |
| US-5.1.6.4 | player (P-23) | I want layered music stems to be perfectly in sync | music sounds cohesive | F-5.1.6 | R-5.1.6 |
| US-5.1.6.5 | audio designer (P-14) | I want gameplay events to trigger sounds at exact timing | audio-visual sync is tight | F-5.1.6 | R-5.1.6 |
| US-5.1.6.6 | engine tester (P-27) | I want to verify scheduled events fire at the exact sample offset | timing accuracy is validated | F-5.1.6 | R-5.1.6 |
| US-5.1.6.7 | designer (P-5) | I want audio buffer size configurable per platform | latency-battery tradeoffs are tunable | F-5.1.6 | R-5.1.6 |
| US-5.1.6.8 | audio designer (P-14) | I want sound stops scheduled at precise samples | loops end cleanly without artifacts | F-5.1.6 | R-5.1.6 |
| US-5.1.6.9 | engine tester (P-27) | I want to test scheduling accuracy under heavy audio load | timing remains precise even with many voices | F-5.1.6 | R-5.1.6 |
| US-5.1.6.10 | designer (P-5) | I want musical cues to trigger at exact gameplay moments | cinematic moments have perfectly timed audio | F-5.1.6 | R-5.1.6 |
| US-5.1.6.11 | player (P-23) | I want sound effects to match visual events exactly | impacts, footsteps, and abilities feel connected | F-5.1.6 | R-5.1.6 |
| US-5.1.6.12 | engine tester (P-27) | I want to verify game-thread commands arrive on the audio thread within one buffer latency | cross-thread timing is reliable | F-5.1.6 | R-5.1.6 |

## F-5.1.7

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.1.7.1 | engine developer (P-26) | I want to decode PCM WAV files at load time | uncompressed reference audio is supported | F-5.1.7 | R-5.1.7 |
| US-5.1.7.2 | engine developer (P-26) | I want to decode Opus streams in real time | voice chat and bandwidth-constrained streaming work | F-5.1.7 | R-5.1.7 |
| US-5.1.7.3 | engine developer (P-26) | I want to decode Vorbis for legacy assets | existing content libraries are supported | F-5.1.7 | R-5.1.7 |
| US-5.1.7.4 | audio designer (P-14) | I want FLAC support for lossless reference audio | high-fidelity recordings are available | F-5.1.7 | R-5.1.7 |
| US-5.1.7.5 | engine developer (P-26) | I want the codec registry to be extensible via plugins | custom or proprietary codecs can be added | F-5.1.7 | R-5.1.7 |
| US-5.1.7.6 | player (P-23) | I want all audio to play at high quality | the underlying format is transparent to me | F-5.1.7 | R-5.1.7 |
| US-5.1.7.7 | engine developer (P-26) | I want sample rate, channel count, and loop points extracted during asset import | playback setup is fast | F-5.1.7 | R-5.1.7 |
| US-5.1.7.8 | engine developer (P-26) | I want platform-supplied hardware decoders used opportunistically (Apple Audio Toolbox) | CPU is saved where available | F-5.1.7 | R-5.1.7 |
| US-5.1.7.9 | engine tester (P-27) | I want to verify all codecs decode correctly on all platforms | format support is cross-platform | F-5.1.7 | R-5.1.7 |
| US-5.1.7.10 | audio designer (P-14) | I want to import audio in WAV, Vorbis, Opus, and FLAC formats | I can use whichever format suits the asset | F-5.1.7 | R-5.1.7 |
| US-5.1.7.11 | designer (P-5) | I want to configure preferred codec per asset type | each category uses optimal encoding | F-5.1.7 | R-5.1.7 |
| US-5.1.7.12 | engine tester (P-27) | I want to verify loop points decode correctly in each format | seamless looping works regardless of codec | F-5.1.7 | R-5.1.7 |
