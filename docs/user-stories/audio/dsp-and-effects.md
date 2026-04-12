# User Stories -- 5.3 DSP & Effects

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-5.3.1.1  | audio designer (P-14)   |
| US-5.3.1.2  | audio designer (P-14)   |
| US-5.3.1.3  | engine developer (P-26) |
| US-5.3.1.4  | player (P-23)           |
| US-5.3.2.1  | audio designer (P-14)   |
| US-5.3.2.2  | audio designer (P-14)   |
| US-5.3.2.3  | game designer (P-5)     |
| US-5.3.2.4  | player (P-23)           |
| US-5.3.3.1  | audio designer (P-14)   |
| US-5.3.3.2  | audio designer (P-14)   |
| US-5.3.3.3  | engine developer (P-26) |
| US-5.3.3.4  | player (P-23)           |
| US-5.3.4.1  | audio designer (P-14)   |
| US-5.3.4.2  | audio designer (P-14)   |
| US-5.3.4.3  | engine developer (P-26) |
| US-5.3.4.4  | player (P-23)           |
| US-5.3.5.1  | audio designer (P-14)   |
| US-5.3.5.2  | engine developer (P-26) |
| US-5.3.5.3  | game designer (P-5)     |
| US-5.3.5.4  | player (P-23)           |
| US-5.3.6.1  | audio designer (P-14)   |
| US-5.3.6.2  | audio designer (P-14)   |
| US-5.3.6.3  | engine developer (P-26) |
| US-5.3.6.4  | player (P-23)           |
| US-5.3.7.1  | audio designer (P-14)   |
| US-5.3.7.2  | audio designer (P-14)   |
| US-5.3.7.3  | engine developer (P-26) |
| US-5.3.7.4  | player (P-23)           |
| US-5.3.8.1  | audio designer (P-14)   |
| US-5.3.8.2  | engine developer (P-26) |
| US-5.3.8.3  | engine developer (P-26) |
| US-5.3.8.4  | game designer (P-5)     |

1. **US-5.3.1.1** — **As a** audio designer (P-14), **I want** low-pass, high-pass, band-pass, and
   notch biquad filters with configurable cutoff, Q, and gain, **so that** I can shape tonal
   character for occlusion and radio effects.

2. **US-5.3.1.2** — **As a** audio designer (P-14), **I want** filter coefficient updates smoothed
   per-sample, **so that** runtime parameter sweeps produce no zipper noise.

3. **US-5.3.1.3** — **As a** engine developer (P-26), **I want** biquad filters implemented as enum
   variants with no dynamic dispatch, **so that** per-voice filter chains run within the audio
   thread budget.

4. **US-5.3.1.4** — **As a** player (P-23), **I want** sounds to muffle believably when I go
   underwater or behind a wall, **so that** the acoustic environment responds to my position.

5. **US-5.3.2.1** — **As a** audio designer (P-14), **I want** a multi-band parametric EQ (up to 8
   bands) as a bus insert with peak, shelf, and pass shapes, **so that** I can sculpt tonal
   character per audio category.

6. **US-5.3.2.2** — **As a** audio designer (P-14), **I want** EQ profiles assignable per reverb
   zone and switchable at runtime, **so that** different environments have distinct tonal
   signatures.

7. **US-5.3.2.3** — **As a** game designer (P-5), **I want** to configure EQ band parameters in the
   visual editor, **so that** equalization is tuned alongside level design.

8. **US-5.3.2.4** — **As a** player (P-23), **I want** underwater audio to sound muffled and
   bass-heavy while cathedrals feel reverberant, **so that** each environment has a unique acoustic
   identity.

9. **US-5.3.3.1** — **As a** audio designer (P-14), **I want** algorithmic reverb (feedback delay
   network) with pre-delay, decay, diffusion, damping, and wet/dry controls, **so that** open-world
   spaces have spatial reverb without needing individual impulse responses.

10. **US-5.3.3.2** — **As a** audio designer (P-14), **I want** reverb parameters configurable per
    zone with smooth blending at transitions, **so that** each space sounds unique without audible
    pops.

11. **US-5.3.3.3** — **As a** engine developer (P-26), **I want** FDN delay line count scaled per
    platform tier (mobile 4, Switch 8, desktop 16), **so that** reverb CPU cost fits each platform
    budget.

12. **US-5.3.3.4** — **As a** player (P-23), **I want** canyon environments to produce expansive
    echo and small rooms to feel tight, **so that** acoustics match the visual space.

13. **US-5.3.4.1** — **As a** audio designer (P-14), **I want** convolution reverb using
    user-supplied impulse responses for hero locations, **so that** throne rooms and boss arenas
    have realistic acoustic signatures.

14. **US-5.3.4.2** — **As a** audio designer (P-14), **I want** IR assets assigned to specific zones
    and loaded via the streaming system, **so that** large impulse responses do not block engine
    startup.

15. **US-5.3.4.3** — **As a** engine developer (P-26), **I want** partitioned FFT convolution
    completing within one audio buffer period, **so that** convolution reverb runs in real time
    without added latency.

16. **US-5.3.4.4** — **As a** player (P-23), **I want** boss arenas to have unique, dramatic reverb,
    **so that** encounters feel epic and acoustically distinct.

17. **US-5.3.5.1** — **As a** audio designer (P-14), **I want** per-bus compressor inserts with
    threshold, ratio, attack, release, knee, and makeup gain, **so that** I can control dynamic
    range per audio category.

18. **US-5.3.5.2** — **As a** engine developer (P-26), **I want** a look-ahead limiter on the master
    bus preventing output from exceeding 0 dBFS, **so that** digital clipping never reaches the
    player.

19. **US-5.3.5.3** — **As a** game designer (P-5), **I want** to configure dynamics processing in
    the visual editor, **so that** compression and limiting are tuned visually.

20. **US-5.3.5.4** — **As a** player (P-23), **I want** audio to stay balanced during intense combat
    without distortion, **so that** the mix remains clear and pleasant.

21. **US-5.3.6.1** — **As a** audio designer (P-14), **I want** delay effects with configurable
    delay time and feedback on send buses, **so that** canyon echoes and environmental reverb are
    easy to author.

22. **US-5.3.6.2** — **As a** audio designer (P-14), **I want** chorus and flanger effects as bus
    inserts, **so that** modulation effects add richness to ambient and magical sounds.

23. **US-5.3.6.3** — **As a** engine developer (P-26), **I want** delay time optionally synced to
    the beat clock, **so that** rhythmic delays align with the music score.

24. **US-5.3.6.4** — **As a** player (P-23), **I want** spells and abilities to have distinctive
    modulated audio, **so that** magical effects sound otherworldly.

25. **US-5.3.7.1** — **As a** audio designer (P-14), **I want** pitch shifting independent of
    playback speed within +/- 12 semitones, **so that** I can create demon voices, chipmunk effects,
    and slow-motion audio.

26. **US-5.3.7.2** — **As a** audio designer (P-14), **I want** pitch modulated by gameplay
    parameters, **so that** sounds dynamically respond to game state.

27. **US-5.3.7.3** — **As a** engine developer (P-26), **I want** desktop to use phase-vocoder and
    mobile to use OLA, **so that** quality and CPU cost match each platform.

28. **US-5.3.7.4** — **As a** player (P-23), **I want** audio to pitch-shift during slow-motion
    sequences, **so that** time dilation feels immersive.

29. **US-5.3.8.1** — **As a** audio designer (P-14), **I want** custom DSP nodes insertable at any
    point in the mixer bus graph, **so that** I can experiment with new effects without waiting for
    engine changes.

30. **US-5.3.8.2** — **As a** engine developer (P-26), **I want** custom nodes registered at runtime
    via a plugin API with stateless process callbacks, **so that** third-party effects integrate
    without recompilation.

31. **US-5.3.8.3** — **As a** engine developer (P-26), **I want** total DSP chain length capped per
    tier (mobile 8-12, Switch 16-24, desktop 32+), **so that** custom effects do not exceed the
    audio thread budget.

32. **US-5.3.8.4** — **As a** game designer (P-5), **I want** to configure custom node parameters
    visually in the editor, **so that** plugin effects are tuned alongside built-in effects.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-5.3.1 | audio designer (P-14) |
| US-5.3.2 | audio designer (P-14) |
| US-5.3.3 | audio designer (P-14) |
| US-5.3.4 | audio designer (P-14) |
| US-5.3.5 | audio designer (P-14) |
| US-5.3.6 | audio designer (P-14) |
| US-5.3.7 | audio designer (P-14) |
| US-5.3.8 | audio designer (P-14) |

1. **US-5.3.1** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.3.1.1 through US-5.3.1.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-5.3.2** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.3.2.1 through US-5.3.2.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

3. **US-5.3.3** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.3.3.1 through US-5.3.3.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

4. **US-5.3.4** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.3.4.1 through US-5.3.4.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

5. **US-5.3.5** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.3.5.1 through US-5.3.5.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

6. **US-5.3.6** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.3.6.1 through US-5.3.6.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

7. **US-5.3.7** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.3.7.1 through US-5.3.7.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

8. **US-5.3.8** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.3.8.1 through US-5.3.8.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.
