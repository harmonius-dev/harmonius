# User Stories — 5.1 Audio Engine

## F-5.1.1

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.1.1.1  | audio designer (P-14)   | F-5.1.1  | R-5.1.1      |
| US-5.1.1.2  | audio designer (P-14)   | F-5.1.1  | R-5.1.1      |
| US-5.1.1.3  | audio designer (P-14)   | F-5.1.1  | R-5.1.1      |
| US-5.1.1.4  | engine developer (P-26) | F-5.1.1  | R-5.1.1      |
| US-5.1.1.5  | player (P-23)           | F-5.1.1  | R-5.1.1      |
| US-5.1.1.6  | engine tester (P-27)    | F-5.1.1  | R-5.1.1      |
| US-5.1.1.7  | audio designer (P-14)   | F-5.1.1  | R-5.1.1      |
| US-5.1.1.8  | designer (P-5)          | F-5.1.1  | R-5.1.1      |
| US-5.1.1.9  | engine tester (P-27)    | F-5.1.1  | R-5.1.1      |
| US-5.1.1.10 | player (P-23)           | F-5.1.1  | R-5.1.1      |
| US-5.1.1.11 | audio designer (P-14)   | F-5.1.1  | R-5.1.1      |
| US-5.1.1.12 | designer (P-5)          | F-5.1.1  | R-5.1.1      |

1. **US-5.1.1.1** — I want to attach a point sound emitter to any entity via a lightweight ECS
   component
   - **Acceptance:** campfires, torches, and NPCs emit sounds
2. **US-5.1.1.2** — I want to attach line-shaped emitters along rivers and streams
   - **Acceptance:** running water sounds natural along its length
3. **US-5.1.1.3** — I want to attach area emitters for ambient zones (forests, markets)
   - **Acceptance:** ambient sound fills a region
4. **US-5.1.1.4** — I want to implement a SoundSource component carrying gain, pitch, looping, and
   attenuation
   - **Acceptance:** emitters are lightweight ECS data
5. **US-5.1.1.5** — I want to hear campfires crackle and waterfalls roar when near them
   - **Acceptance:** the world sounds alive
6. **US-5.1.1.6** — I want to verify SoundSource component overhead is negligible per entity
   - **Acceptance:** hundreds of emitters are feasible
7. **US-5.1.1.7** — I want to set gain and pitch per sound source
   - **Acceptance:** individual emitters are tunable without code
8. **US-5.1.1.8** — I want to enable looping on ambient sound sources
   - **Acceptance:** continuous environmental sounds play indefinitely
9. **US-5.1.1.9** — I want to test 200+ simultaneous emitters
   - **Acceptance:** dense open-world scenes are validated
10. **US-5.1.1.10** — I want to hear distinct sounds from different nearby objects
    - **Acceptance:** I can identify sources by ear
11. **US-5.1.1.11** — I want to set attenuation curve references per source
    - **Acceptance:** each emitter has appropriate distance rolloff
12. **US-5.1.1.12** — I want to assign sound source components in the visual editor
    - **Acceptance:** audio is set up without code

## F-5.1.2

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.1.2.1  | audio designer (P-14)   | F-5.1.2  | R-5.1.2      |
| US-5.1.2.2  | engine developer (P-26) | F-5.1.2  | R-5.1.2      |
| US-5.1.2.3  | engine developer (P-26) | F-5.1.2  | R-5.1.2      |
| US-5.1.2.4  | player (P-23)           | F-5.1.2  | R-5.1.2      |
| US-5.1.2.5  | designer (P-5)          | F-5.1.2  | R-5.1.2      |
| US-5.1.2.6  | engine tester (P-27)    | F-5.1.2  | R-5.1.2      |
| US-5.1.2.7  | engine tester (P-27)    | F-5.1.2  | R-5.1.2      |
| US-5.1.2.8  | audio designer (P-14)   | F-5.1.2  | R-5.1.2      |
| US-5.1.2.9  | designer (P-5)          | F-5.1.2  | R-5.1.2      |
| US-5.1.2.10 | engine tester (P-27)    | F-5.1.2  | R-5.1.2      |
| US-5.1.2.11 | player (P-23)           | F-5.1.2  | R-5.1.2      |
| US-5.1.2.12 | designer (P-5)          | F-5.1.2  | R-5.1.2      |

1. **US-5.1.2.1** — I want to designate one or more listener entities
   - **Acceptance:** the player's auditory perspective is defined
2. **US-5.1.2.2** — I want to implement a Listener component with position, orientation, and
   velocity
   - **Acceptance:** spatial audio has a reference point
3. **US-5.1.2.3** — I want multiple listeners for split-screen and spectator modes
   - **Acceptance:** each view has its own audio perspective
4. **US-5.1.2.4** — I want audio positioned relative to my camera
   - **Acceptance:** sounds come from the correct direction
5. **US-5.1.2.5** — I want the default listener assigned to the active camera
   - **Acceptance:** audio perspective matches visual perspective automatically
6. **US-5.1.2.6** — I want to verify listener velocity is used for Doppler calculations
   - **Acceptance:** moving listener produces correct pitch shifts
7. **US-5.1.2.7** — I want to test multiple listeners in split-screen mode
   - **Acceptance:** each listener has independent spatial audio
8. **US-5.1.2.8** — I want each player in split-screen to have an independent listener
   - **Acceptance:** audio perspective matches each player's view
9. **US-5.1.2.9** — I want spectator cameras to have their own listener
   - **Acceptance:** spectators hear from their perspective
10. **US-5.1.2.10** — I want to verify listener position updates every frame
    - **Acceptance:** spatial audio stays synchronized with visuals
11. **US-5.1.2.11** — I want sound panning to update as I move and rotate the camera
    - **Acceptance:** audio tracking is smooth
12. **US-5.1.2.12** — I want to assign listener components in the visual editor
    - **Acceptance:** listener configuration is code-free

## F-5.1.3

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.1.3.1  | audio designer (P-14)   | F-5.1.3  | R-5.1.3      |
| US-5.1.3.2  | engine developer (P-26) | F-5.1.3  | R-5.1.3      |
| US-5.1.3.3  | audio designer (P-14)   | F-5.1.3  | R-5.1.3      |
| US-5.1.3.4  | player (P-23)           | F-5.1.3  | R-5.1.3      |
| US-5.1.3.5  | engine developer (P-26) | F-5.1.3  | R-5.1.3      |
| US-5.1.3.6  | audio designer (P-14)   | F-5.1.3  | R-5.1.3      |
| US-5.1.3.7  | designer (P-5)          | F-5.1.3  | R-5.1.3      |
| US-5.1.3.8  | engine tester (P-27)    | F-5.1.3  | R-5.1.3      |
| US-5.1.3.9  | audio designer (P-14)   | F-5.1.3  | R-5.1.3      |
| US-5.1.3.10 | engine tester (P-27)    | F-5.1.3  | R-5.1.3      |
| US-5.1.3.11 | player (P-23)           | F-5.1.3  | R-5.1.3      |
| US-5.1.3.12 | engine developer (P-26) | F-5.1.3  | R-5.1.3      |

1. **US-5.1.3.1** — I want to define a hierarchy of mixer buses (master, music, SFX, ambient, voice,
   UI)
   - **Acceptance:** audio is organized by category
2. **US-5.1.3.2** — I want child buses to inherit parent gain
   - **Acceptance:** global volume control works hierarchically
3. **US-5.1.3.3** — I want to insert effect chains on any bus
   - **Acceptance:** per-category processing is possible
4. **US-5.1.3.4** — I want to mute the music bus independently
   - **Acceptance:** I can play with only sound effects
5. **US-5.1.3.5** — I want buses added and rewired at runtime
   - **Acceptance:** dynamic mix states (underwater, pause menu) are supported
6. **US-5.1.3.6** — I want mute and solo controls per bus
   - **Acceptance:** I can isolate categories during mixing
7. **US-5.1.3.7** — I want to configure the mixer bus graph in the visual editor
   - **Acceptance:** audio routing is visual
8. **US-5.1.3.8** — I want to verify mixer bus hierarchy depth is uniform across platforms
   - **Acceptance:** audio structure is consistent
9. **US-5.1.3.9** — I want to duck the SFX bus when dialogue plays
   - **Acceptance:** speech is always intelligible
10. **US-5.1.3.10** — I want to verify insert effect chain length scales per tier (mobile limited)
    - **Acceptance:** DSP budget is controlled
11. **US-5.1.3.11** — I want separate volume sliders for music, SFX, voice, and ambient
    - **Acceptance:** I can customize my audio mix
12. **US-5.1.3.12** — I want buses wired as a DAG
    - **Acceptance:** complex routing (send buses, sub-mixes) is possible without cycles

## F-5.1.4

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.1.4.1  | engine developer (P-26) | F-5.1.4  | R-5.1.4      |
| US-5.1.4.2  | audio designer (P-14)   | F-5.1.4  | R-5.1.4      |
| US-5.1.4.3  | engine developer (P-26) | F-5.1.4  | R-5.1.4      |
| US-5.1.4.4  | player (P-23)           | F-5.1.4  | R-5.1.4      |
| US-5.1.4.5  | engine developer (P-26) | F-5.1.4  | R-5.1.4      |
| US-5.1.4.6  | engine tester (P-27)    | F-5.1.4  | R-5.1.4      |
| US-5.1.4.7  | engine developer (P-26) | F-5.1.4  | R-5.1.4      |
| US-5.1.4.8  | audio designer (P-14)   | F-5.1.4  | R-5.1.4      |
| US-5.1.4.9  | engine tester (P-27)    | F-5.1.4  | R-5.1.4      |
| US-5.1.4.10 | player (P-23)           | F-5.1.4  | R-5.1.4      |
| US-5.1.4.11 | designer (P-5)          | F-5.1.4  | R-5.1.4      |
| US-5.1.4.12 | engine tester (P-27)    | F-5.1.4  | R-5.1.4      |

1. **US-5.1.4.1** — I want to manage a fixed pool of voices with priority-based allocation
   - **Acceptance:** critical sounds always play
2. **US-5.1.4.2** — I want to declare a priority class (critical, high, medium, low) per sound
   source
   - **Acceptance:** importance is ranked
3. **US-5.1.4.3** — I want lowest-scoring voices virtualized (tracked but silent) when the pool is
   exceeded
   - **Acceptance:** sounds resume when headroom returns
4. **US-5.1.4.4** — I want critical sounds (alerts, dialogue) to always play even in noisy combat
   - **Acceptance:** important audio is never missed
5. **US-5.1.4.5** — I want audibility scores derived from distance and occlusion
   - **Acceptance:** voice stealing prioritizes audible sounds
6. **US-5.1.4.6** — I want to confirm voice pool scales per tier (mobile 16-32, Switch 32-64,
   desktop 128-256)
   - **Acceptance:** platform limits are enforced
7. **US-5.1.4.7** — I want virtualized voices restored seamlessly when headroom returns
   - **Acceptance:** the player hears no audible gaps
8. **US-5.1.4.8** — I want to configure the virtualization threshold
   - **Acceptance:** the audibility cutoff is tunable
9. **US-5.1.4.9** — I want to test voice stealing with all pool slots filled
   - **Acceptance:** priority ordering works correctly under pressure
10. **US-5.1.4.10** — I want critical alert sounds to play over ambient noise
    - **Acceptance:** I am always warned of important events
11. **US-5.1.4.11** — I want virtualization thresholds adjusted per platform tier
    - **Acceptance:** more sounds play on capable hardware
12. **US-5.1.4.12** — I want to test voice devirtualization and confirm seamless restoration
    - **Acceptance:** returning sounds have no audible pop

## F-5.1.5

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.1.5.1  | engine developer (P-26) | F-5.1.5  | R-5.1.5      |
| US-5.1.5.2  | audio designer (P-14)   | F-5.1.5  | R-5.1.5      |
| US-5.1.5.3  | player (P-23)           | F-5.1.5  | R-5.1.5      |
| US-5.1.5.4  | engine developer (P-26) | F-5.1.5  | R-5.1.5      |
| US-5.1.5.5  | engine tester (P-27)    | F-5.1.5  | R-5.1.5      |
| US-5.1.5.6  | audio designer (P-14)   | F-5.1.5  | R-5.1.5      |
| US-5.1.5.7  | designer (P-5)          | F-5.1.5  | R-5.1.5      |
| US-5.1.5.8  | engine tester (P-27)    | F-5.1.5  | R-5.1.5      |
| US-5.1.5.9  | player (P-23)           | F-5.1.5  | R-5.1.5      |
| US-5.1.5.10 | engine developer (P-26) | F-5.1.5  | R-5.1.5      |
| US-5.1.5.11 | designer (P-5)          | F-5.1.5  | R-5.1.5      |
| US-5.1.5.12 | engine tester (P-27)    | F-5.1.5  | R-5.1.5      |

1. **US-5.1.5.1** — I want to stream long-duration audio in ring-buffer chunks using platform-native
   async I/O
   - **Acceptance:** large files need not load fully
2. **US-5.1.5.2** — I want prefetch hinting so streaming begins before playback triggers
   - **Acceptance:** startup latency is eliminated for cinematic cues
3. **US-5.1.5.3** — I want music and ambience to start instantly without audible loading gaps
   - **Acceptance:** audio transitions are seamless
4. **US-5.1.5.4** — I want streaming to use IOCP on Windows, GCD async I/O on macOS, and io_uring on
   Linux
   - **Acceptance:** I/O is platform-optimal
5. **US-5.1.5.5** — I want to verify ring-buffer chunk sizes prevent underruns during playback
   - **Acceptance:** streaming is gap-free
6. **US-5.1.5.6** — I want dialogue lines streamed on demand without preloading
   - **Acceptance:** memory is conserved for large VO databases
7. **US-5.1.5.7** — I want to configure prefetch timing per audio asset
   - **Acceptance:** important cues start streaming early
8. **US-5.1.5.8** — I want to test audio streaming under heavy I/O load
   - **Acceptance:** streaming remains glitch-free during asset loading
9. **US-5.1.5.9** — I want ambient loops to stream continuously without gaps
   - **Acceptance:** background atmosphere is unbroken
10. **US-5.1.5.10** — I want the streaming system to support multiple concurrent streams within the
    voice budget
    - **Acceptance:** music and dialogue stream together
11. **US-5.1.5.11** — I want to set ring buffer sizes per platform
    - **Acceptance:** memory usage matches device capability
12. **US-5.1.5.12** — I want to verify streaming does not underrun during zone transitions
    - **Acceptance:** audio remains continuous during loading

## F-5.1.6

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.1.6.1  | audio designer (P-14)   | F-5.1.6  | R-5.1.6      |
| US-5.1.6.2  | engine developer (P-26) | F-5.1.6  | R-5.1.6      |
| US-5.1.6.3  | engine developer (P-26) | F-5.1.6  | R-5.1.6      |
| US-5.1.6.4  | player (P-23)           | F-5.1.6  | R-5.1.6      |
| US-5.1.6.5  | audio designer (P-14)   | F-5.1.6  | R-5.1.6      |
| US-5.1.6.6  | engine tester (P-27)    | F-5.1.6  | R-5.1.6      |
| US-5.1.6.7  | designer (P-5)          | F-5.1.6  | R-5.1.6      |
| US-5.1.6.8  | audio designer (P-14)   | F-5.1.6  | R-5.1.6      |
| US-5.1.6.9  | engine tester (P-27)    | F-5.1.6  | R-5.1.6      |
| US-5.1.6.10 | designer (P-5)          | F-5.1.6  | R-5.1.6      |
| US-5.1.6.11 | player (P-23)           | F-5.1.6  | R-5.1.6      |
| US-5.1.6.12 | engine tester (P-27)    | F-5.1.6  | R-5.1.6      |

1. **US-5.1.6.1** — I want to schedule sound start at a precise sample offset
   - **Acceptance:** layered loops sync perfectly
2. **US-5.1.6.2** — I want parameter changes scheduled at sample-accurate offsets
   - **Acceptance:** gain, pitch, and filter changes are click-free
3. **US-5.1.6.3** — I want audio commands queued on the game thread and executed on the audio thread
   at the precise sample
   - **Acceptance:** timing is exact
4. **US-5.1.6.4** — I want layered music stems to be perfectly in sync
   - **Acceptance:** music sounds cohesive
5. **US-5.1.6.5** — I want gameplay events to trigger sounds at exact timing
   - **Acceptance:** audio-visual sync is tight
6. **US-5.1.6.6** — I want to verify scheduled events fire at the exact sample offset
   - **Acceptance:** timing accuracy is validated
7. **US-5.1.6.7** — I want audio buffer size configurable per platform
   - **Acceptance:** latency-battery tradeoffs are tunable
8. **US-5.1.6.8** — I want sound stops scheduled at precise samples
   - **Acceptance:** loops end cleanly without artifacts
9. **US-5.1.6.9** — I want to test scheduling accuracy under heavy audio load
   - **Acceptance:** timing remains precise even with many voices
10. **US-5.1.6.10** — I want musical cues to trigger at exact gameplay moments
    - **Acceptance:** cinematic moments have perfectly timed audio
11. **US-5.1.6.11** — I want sound effects to match visual events exactly
    - **Acceptance:** impacts, footsteps, and abilities feel connected
12. **US-5.1.6.12** — I want to verify game-thread commands arrive on the audio thread within one
    buffer latency
    - **Acceptance:** cross-thread timing is reliable

## F-5.1.7

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.1.7.1  | engine developer (P-26) | F-5.1.7  | R-5.1.7      |
| US-5.1.7.2  | engine developer (P-26) | F-5.1.7  | R-5.1.7      |
| US-5.1.7.3  | engine developer (P-26) | F-5.1.7  | R-5.1.7      |
| US-5.1.7.4  | audio designer (P-14)   | F-5.1.7  | R-5.1.7      |
| US-5.1.7.5  | engine developer (P-26) | F-5.1.7  | R-5.1.7      |
| US-5.1.7.6  | player (P-23)           | F-5.1.7  | R-5.1.7      |
| US-5.1.7.7  | engine developer (P-26) | F-5.1.7  | R-5.1.7      |
| US-5.1.7.8  | engine developer (P-26) | F-5.1.7  | R-5.1.7      |
| US-5.1.7.9  | engine tester (P-27)    | F-5.1.7  | R-5.1.7      |
| US-5.1.7.10 | audio designer (P-14)   | F-5.1.7  | R-5.1.7      |
| US-5.1.7.11 | designer (P-5)          | F-5.1.7  | R-5.1.7      |
| US-5.1.7.12 | engine tester (P-27)    | F-5.1.7  | R-5.1.7      |

1. **US-5.1.7.1** — I want to decode PCM WAV files at load time
   - **Acceptance:** uncompressed reference audio is supported
2. **US-5.1.7.2** — I want to decode Opus streams in real time
   - **Acceptance:** voice chat and bandwidth-constrained streaming work
3. **US-5.1.7.3** — I want to decode Vorbis for legacy assets
   - **Acceptance:** existing content libraries are supported
4. **US-5.1.7.4** — I want FLAC support for lossless reference audio
   - **Acceptance:** high-fidelity recordings are available
5. **US-5.1.7.5** — I want the codec registry to be extensible via plugins
   - **Acceptance:** custom or proprietary codecs can be added
6. **US-5.1.7.6** — I want all audio to play at high quality
   - **Acceptance:** the underlying format is transparent to me
7. **US-5.1.7.7** — I want sample rate, channel count, and loop points extracted during asset import
   - **Acceptance:** playback setup is fast
8. **US-5.1.7.8** — I want platform-supplied hardware decoders used opportunistically (Apple Audio
   Toolbox)
   - **Acceptance:** CPU is saved where available
9. **US-5.1.7.9** — I want to verify all codecs decode correctly on all platforms
   - **Acceptance:** format support is cross-platform
10. **US-5.1.7.10** — I want to import audio in WAV, Vorbis, Opus, and FLAC formats
    - **Acceptance:** I can use whichever format suits the asset
11. **US-5.1.7.11** — I want to configure preferred codec per asset type
    - **Acceptance:** each category uses optimal encoding
12. **US-5.1.7.12** — I want to verify loop points decode correctly in each format
    - **Acceptance:** seamless looping works regardless of codec
