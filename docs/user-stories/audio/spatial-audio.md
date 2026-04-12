# User Stories -- 5.2 Spatial Audio

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-5.2.1.1  | audio designer (P-14)   |
| US-5.2.1.2  | audio designer (P-14)   |
| US-5.2.1.3  | engine developer (P-26) |
| US-5.2.1.4  | player (P-23)           |
| US-5.2.2.1  | audio designer (P-14)   |
| US-5.2.2.2  | audio designer (P-14)   |
| US-5.2.2.3  | game designer (P-5)     |
| US-5.2.2.4  | player (P-23)           |
| US-5.2.3.1  | audio designer (P-14)   |
| US-5.2.3.2  | engine developer (P-26) |
| US-5.2.3.3  | game designer (P-5)     |
| US-5.2.3.4  | player (P-23)           |
| US-5.2.4.1  | audio designer (P-14)   |
| US-5.2.4.2  | engine developer (P-26) |
| US-5.2.4.3  | game designer (P-5)     |
| US-5.2.4.4  | player (P-23)           |
| US-5.2.5.1  | audio designer (P-14)   |
| US-5.2.5.2  | engine developer (P-26) |
| US-5.2.5.3  | game designer (P-5)     |
| US-5.2.5.4  | player (P-23)           |
| US-5.2.6.1  | audio designer (P-14)   |
| US-5.2.6.2  | engine developer (P-26) |
| US-5.2.6.3  | game designer (P-5)     |
| US-5.2.6.4  | player (P-23)           |
| US-5.2.7.1  | audio designer (P-14)   |
| US-5.2.7.2  | audio designer (P-14)   |
| US-5.2.7.3  | engine developer (P-26) |
| US-5.2.7.4  | game designer (P-5)     |
| US-5.2.7.5  | player (P-23)           |
| US-5.2.8.1  | level designer (P-6)    |
| US-5.2.8.2  | audio designer (P-14)   |
| US-5.2.8.3  | player (P-23)           |
| US-5.2.9.1  | engine developer (P-26) |
| US-5.2.9.2  | engine developer (P-26) |
| US-5.2.10.1 | game designer (P-5)     |
| US-5.2.10.2 | audio designer (P-14)   |

1. **US-5.2.1.1** — **As a** audio designer (P-14), **I want** sounds positioned in world space with
   per-listener panning, distance gain, and Doppler pitch shift every audio frame, **so that** 3D
   audio placement matches visual positions.

2. **US-5.2.1.2** — **As a** audio designer (P-14), **I want** to configure Doppler scale factor per
   source, **so that** the intensity of pitch shift is tunable per sound type.

3. **US-5.2.1.3** — **As a** engine developer (P-26), **I want** source and listener transforms
   interpolated between game ticks, **so that** audio motion is smooth at the audio buffer callback
   rate.

4. **US-5.2.1.4** — **As a** player (P-23), **I want** to hear enemies approaching from behind and
   passing vehicles pitch-shift realistically, **so that** spatial audio provides tactical and
   immersive information.

5. **US-5.2.2.1** — **As a** audio designer (P-14), **I want** configurable distance attenuation
   models (inverse, inverse-squared, linear, logarithmic, custom curves) per source, **so that**
   each sound has appropriate rolloff behavior.

6. **US-5.2.2.2** — **As a** audio designer (P-14), **I want** per-source minimum and maximum
   distance parameters, **so that** the rolloff range is controlled independently per emitter.

7. **US-5.2.2.3** — **As a** game designer (P-5), **I want** to configure attenuation model and
   distances in the visual editor, **so that** distance falloff is tuned alongside level layout.

8. **US-5.2.2.4** — **As a** player (P-23), **I want** sounds to fade naturally with distance so
   nearby footsteps are clear over distant battle sounds, **so that** the soundscape has realistic
   depth.

9. **US-5.2.3.1** — **As a** audio designer (P-14), **I want** HRTF binaural rendering using
   swappable SOFA-format datasets, **so that** headphone spatialization provides accurate elevation
   and front-back cues.

10. **US-5.2.3.2** — **As a** engine developer (P-26), **I want** a fast frequency-domain
    convolution path for per-voice HRTF that delegates to platform spatial audio APIs where
    available, **so that** binaural rendering fits within the voice budget.

11. **US-5.2.3.3** — **As a** game designer (P-5), **I want** HRTF binaural mode selectable in audio
    settings, **so that** players with headphones can enable personalized spatial audio.

12. **US-5.2.3.4** — **As a** player (P-23), **I want** sounds above and below me distinguishable on
    headphones, **so that** I can pinpoint sound sources in 3D space.

13. **US-5.2.4.1** — **As a** audio designer (P-14), **I want** sound sources encoded into first- or
    third-order Ambisonics and decoded to stereo, 5.1, 7.1, or binaural, **so that** mixing is
    speaker-layout-agnostic.

14. **US-5.2.4.2** — **As a** engine developer (P-26), **I want** Ambisonics order scaled per
    platform tier (mobile first-order, desktop third-order) with the field rotating for head
    tracking, **so that** CPU cost and quality match each platform.

15. **US-5.2.4.3** — **As a** game designer (P-5), **I want** output format (stereo, 5.1, 7.1,
    binaural) selectable in settings, **so that** Ambisonics decodes to the player's speaker setup.

16. **US-5.2.4.4** — **As a** player (P-23), **I want** immersive surround sound on my speaker setup
    with smooth source panning, **so that** audio fills the room correctly.

17. **US-5.2.5.1** — **As a** audio designer (P-14), **I want** to configure per-material
    transmission loss coefficients (wood, stone, glass, metal), **so that** different barriers
    produce distinct muffling levels.

18. **US-5.2.5.2** — **As a** engine developer (P-26), **I want** occlusion ray-casts against the
    shared BVH spatial index with per-tier ray counts (mobile 1, Switch 2, desktop 4), **so that**
    audio occlusion uses the same structure as physics and rendering.

19. **US-5.2.5.3** — **As a** game designer (P-5), **I want** to assign audio occlusion materials to
    walls and barriers in the editor, **so that** sound blocking matches the visual material.

20. **US-5.2.5.4** — **As a** player (P-23), **I want** sounds behind walls to be muffled and
    closing a door to muffle sounds from the other side, **so that** occlusion enhances spatial
    awareness.

21. **US-5.2.6.1** — **As a** audio designer (P-14), **I want** to define portals (doorways,
    windows, tunnels) in the audio propagation graph, **so that** sound travels through openings in
    geometry.

22. **US-5.2.6.2** — **As a** engine developer (P-26), **I want** a hybrid ray-and-portal
    propagation solver running asynchronously at a reduced rate, **so that** indirect sound paths
    are modeled without blocking the audio thread.

23. **US-5.2.6.3** — **As a** game designer (P-5), **I want** to configure propagation complexity
    per platform (portal-only mobile, multi-bounce desktop) in the editor, **so that** propagation
    cost matches platform budget.

24. **US-5.2.6.4** — **As a** player (P-23), **I want** combat sounds to echo through dungeon
    corridors and outdoor sounds to come through open windows, **so that** indirect paths feel
    natural and immersive.

25. **US-5.2.7.1** — **As a** audio designer (P-14), **I want** to define reverb zones as
    axis-aligned or convex volumes with configurable decay, diffusion, and early reflection
    patterns, **so that** enclosed spaces have distinct acoustic character.

26. **US-5.2.7.2** — **As a** audio designer (P-14), **I want** nested zones to combine via priority
    ordering with smooth blending at boundaries, **so that** transitions between spaces are
    seamless.

27. **US-5.2.7.3** — **As a** engine developer (P-26), **I want** active reverb zone count scaled
    per tier (mobile 1-2, Switch 3-4, desktop 6+) with early reflections disabled on mobile,
    **so that** reverb CPU cost stays within platform budgets.

28. **US-5.2.7.4** — **As a** game designer (P-5), **I want** to place and configure reverb zones in
    the level editor, **so that** room acoustics are part of level design.

29. **US-5.2.7.5** — **As a** player (P-23), **I want** echo to increase when entering a cave and
    acoustics to change noticeably between indoor and outdoor spaces, **so that** the audio
    environment matches the visual space.
