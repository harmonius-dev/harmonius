# User Stories -- 5.1 Audio Engine

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-5.1.1.1  | audio designer (P-14)   |
| US-5.1.1.2  | audio designer (P-14)   |
| US-5.1.1.3  | game designer (P-5)     |
| US-5.1.1.4  | engine developer (P-26) |
| US-5.1.1.5  | player (P-23)           |
| US-5.1.2.1  | audio designer (P-14)   |
| US-5.1.2.2  | audio designer (P-14)   |
| US-5.1.2.3  | game designer (P-5)     |
| US-5.1.2.4  | engine developer (P-26) |
| US-5.1.2.5  | player (P-23)           |
| US-5.1.3.1  | audio designer (P-14)   |
| US-5.1.3.2  | audio designer (P-14)   |
| US-5.1.3.3  | game designer (P-5)     |
| US-5.1.3.4  | engine developer (P-26) |
| US-5.1.3.5  | player (P-23)           |
| US-5.1.4.1  | audio designer (P-14)   |
| US-5.1.4.2  | engine developer (P-26) |
| US-5.1.4.3  | engine developer (P-26) |
| US-5.1.4.4  | game designer (P-5)     |
| US-5.1.4.5  | player (P-23)           |
| US-5.1.5.1  | audio designer (P-14)   |
| US-5.1.5.2  | audio designer (P-14)   |
| US-5.1.5.3  | engine developer (P-26) |
| US-5.1.5.4  | game designer (P-5)     |
| US-5.1.5.5  | player (P-23)           |
| US-5.1.6.1  | audio designer (P-14)   |
| US-5.1.6.2  | audio designer (P-14)   |
| US-5.1.6.3  | engine developer (P-26) |
| US-5.1.6.4  | game designer (P-5)     |
| US-5.1.6.5  | player (P-23)           |
| US-5.1.7.1  | audio designer (P-14)   |
| US-5.1.7.2  | audio designer (P-14)   |
| US-5.1.7.3  | engine developer (P-26) |
| US-5.1.7.4  | engine developer (P-26) |
| US-5.1.7.5  | game designer (P-5)     |

1. **US-5.1.1.1** — **As a** audio designer (P-14), **I want** to attach point, line, and area sound
   emitters to entities via ECS components, **so that** I can place ambient sounds throughout a
   scene without writing code.

2. **US-5.1.1.2** — **As a** audio designer (P-14), **I want** to set gain, pitch, looping, and
   attenuation curve per sound source component, **so that** each emitter has individually tuned
   playback behavior.

3. **US-5.1.1.3** — **As a** game designer (P-5), **I want** to assign sound source components to
   entities in the visual editor, **so that** audio placement is part of the level design workflow
   without code.

4. **US-5.1.1.4** — **As a** engine developer (P-26), **I want** the sound source component to store
   all data in 128 bytes or fewer, **so that** hundreds of simultaneous emitters do not degrade
   frame time.

5. **US-5.1.1.5** — **As a** player (P-23), **I want** to hear distinct ambient sounds from nearby
   campfires, waterfalls, and NPCs, **so that** the world feels alive and spatially believable.

6. **US-5.1.2.1** — **As a** audio designer (P-14), **I want** to designate one or more entities as
   audio listeners with position, orientation, and velocity, **so that** the player's auditory
   perspective is well-defined.

7. **US-5.1.2.2** — **As a** audio designer (P-14), **I want** each split-screen player to have an
   independent listener, **so that** spatial audio is correct for every viewport.

8. **US-5.1.2.3** — **As a** game designer (P-5), **I want** the default listener to attach to the
   active camera automatically, **so that** audio perspective matches the visual perspective without
   manual setup.

9. **US-5.1.2.4** — **As a** engine developer (P-26), **I want** listener velocity to feed into
   Doppler calculations, **so that** moving listeners produce correct pitch shifts on nearby
   sources.

10. **US-5.1.2.5** — **As a** player (P-23), **I want** sound panning and distance to update as I
    move and rotate the camera, **so that** audio tracking is smooth and consistent with visuals.

11. **US-5.1.3.1** — **As a** audio designer (P-14), **I want** a DAG of mixer buses (master, music,
    SFX, ambient, voice, UI) with gain, mute, and solo controls, **so that** I can organize and
    balance the audio mix by category.

12. **US-5.1.3.2** — **As a** audio designer (P-14), **I want** to insert effect chains on any mixer
    bus and rewire buses at runtime, **so that** dynamic mix states like underwater ducking work
    without code changes.

13. **US-5.1.3.3** — **As a** game designer (P-5), **I want** to configure the mixer bus graph in
    the visual editor, **so that** audio routing is designed alongside level layout.

14. **US-5.1.3.4** — **As a** engine developer (P-26), **I want** child buses to inherit parent gain
    and the hierarchy to be rewirable without audio discontinuity, **so that** global volume control
    propagates correctly.

15. **US-5.1.3.5** — **As a** player (P-23), **I want** separate volume sliders for music, SFX,
    voice, and ambient, **so that** I can customize the audio mix to my preference.

16. **US-5.1.4.1** — **As a** audio designer (P-14), **I want** to declare a priority class per
    sound source (critical, high, medium, low), **so that** important sounds always play even in
    dense scenes.

17. **US-5.1.4.2** — **As a** engine developer (P-26), **I want** a fixed voice pool with
    priority-based allocation and virtualization, **so that** the lowest-audibility voices are
    silenced and restored seamlessly.

18. **US-5.1.4.3** — **As a** engine developer (P-26), **I want** configurable voice pool sizes per
    platform tier (mobile 16-32, Switch 32-64, desktop 128-256), **so that** audio budgets match
    hardware capability.

19. **US-5.1.4.4** — **As a** game designer (P-5), **I want** to configure virtualization thresholds
    per platform in the editor, **so that** more sounds play on capable hardware without manual
    tuning.

20. **US-5.1.4.5** — **As a** player (P-23), **I want** critical alerts and dialogue to always play
    over ambient noise, **so that** I never miss important gameplay audio.

21. **US-5.1.5.1** — **As a** audio designer (P-14), **I want** long-duration audio (music,
    ambience, dialogue) streamed from disk in ring-buffer chunks, **so that** large files need not
    load fully into memory.

22. **US-5.1.5.2** — **As a** audio designer (P-14), **I want** prefetch hints that begin streaming
    before playback triggers, **so that** cinematic cues and zone transitions start with zero
    audible latency.

23. **US-5.1.5.3** — **As a** engine developer (P-26), **I want** streaming to use platform-native
    async I/O (IOCP, GCD, io_uring) with peak memory under 256 KiB per stream, **so that** I/O is
    optimal and memory-bounded.

24. **US-5.1.5.4** — **As a** game designer (P-5), **I want** to configure prefetch timing and ring
    buffer sizes per platform in the editor, **so that** streaming behavior adapts to device
    capability.

25. **US-5.1.5.5** — **As a** player (P-23), **I want** music and ambience to start instantly
    without audible loading gaps, **so that** audio transitions feel seamless during exploration.

26. **US-5.1.6.1** — **As a** audio designer (P-14), **I want** to schedule sound start, stop, and
    parameter changes at precise sample offsets, **so that** layered loops and musical cues sync
    perfectly.

27. **US-5.1.6.2** — **As a** audio designer (P-14), **I want** gameplay events to trigger sounds at
    exact timing relative to the audio buffer, **so that** audio-visual synchronization is tight.

28. **US-5.1.6.3** — **As a** engine developer (P-26), **I want** a lock-free SPSC command queue
    between the game thread and audio thread, **so that** scheduling never blocks the real-time
    audio callback.

29. **US-5.1.6.4** — **As a** game designer (P-5), **I want** audio buffer size configurable per
    platform, **so that** I can balance latency and battery usage per device.

30. **US-5.1.6.5** — **As a** player (P-23), **I want** sound effects to match visual events exactly
    (impacts, footsteps, abilities), **so that** actions feel connected and responsive.

31. **US-5.1.7.1** — **As a** audio designer (P-14), **I want** to import audio in WAV, Vorbis,
    Opus, and FLAC formats, **so that** I can use whichever format suits each asset type.

32. **US-5.1.7.2** — **As a** audio designer (P-14), **I want** format metadata (sample rate,
    channel count, loop points) extracted and cached at import time, **so that** playback setup is
    fast and reliable.

33. **US-5.1.7.3** — **As a** engine developer (P-26), **I want** the codec registry to support
    runtime plugin registration, **so that** custom or proprietary codecs can be added without
    engine recompilation.

34. **US-5.1.7.4** — **As a** engine developer (P-26), **I want** platform hardware decoders (e.g.,
    Apple Audio Toolbox) used opportunistically, **so that** CPU is saved where available.

35. **US-5.1.7.5** — **As a** game designer (P-5), **I want** to configure preferred codec per asset
    type in the editor, **so that** each audio category uses optimal encoding.
