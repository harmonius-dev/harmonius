# User Stories -- 5.5 Voice & Speech

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-5.5.1.1  | audio designer (P-14)   |
| US-5.5.1.2  | engine developer (P-26) |
| US-5.5.1.3  | engine developer (P-26) |
| US-5.5.1.4  | game designer (P-5)     |
| US-5.5.1.5  | player (P-23)           |
| US-5.5.2.1  | engine developer (P-26) |
| US-5.5.2.2  | engine developer (P-26) |
| US-5.5.2.3  | game designer (P-5)     |
| US-5.5.2.4  | player (P-23)           |
| US-5.5.3.1  | engine developer (P-26) |
| US-5.5.3.2  | engine developer (P-26) |
| US-5.5.3.3  | audio designer (P-14)   |
| US-5.5.3.4  | player (P-23)           |
| US-5.5.4.1  | engine developer (P-26) |
| US-5.5.4.2  | audio designer (P-14)   |
| US-5.5.4.3  | game designer (P-5)     |
| US-5.5.4.4  | player (P-23)           |
| US-5.5.5.1  | engine developer (P-26) |
| US-5.5.5.2  | audio designer (P-14)   |
| US-5.5.5.3  | engine developer (P-26) |
| US-5.5.5.4  | player (P-23)           |
| US-5.5.6.1  | audio designer (P-14)   |
| US-5.5.6.2  | audio designer (P-14)   |
| US-5.5.6.3  | engine developer (P-26) |
| US-5.5.6.4  | game designer (P-5)     |
| US-5.5.6.5  | player (P-23)           |
| US-5.5.7.1  | audio designer (P-14)   |
| US-5.5.7.2  | engine developer (P-26) |
| US-5.5.7.3  | game designer (P-5)     |
| US-5.5.7.4  | player (P-23)           |
| US-5.5.8.1  | engine developer (P-26) |
| US-5.5.8.2  | engine developer (P-26) |
| US-5.5.8.3  | game designer (P-5)     |
| US-5.5.8.4  | player (P-23)           |
| US-5.5.8.5  | player (P-23)           |
| US-5.5.9.1  | engine developer (P-26) |
| US-5.5.9.2  | engine developer (P-26) |
| US-5.5.9.3  | game designer (P-5)     |
| US-5.5.9.4  | player (P-23)           |

1. **US-5.5.1.1** — **As a** audio designer (P-14), **I want** proximity chat routed as spatialized
   3D sources and party channels as non-spatialized bus sends, **so that** voice routing matches
   social context.

2. **US-5.5.1.2** — **As a** engine developer (P-26), **I want** microphone capture using
   platform-native APIs (WASAPI, CoreAudio, PipeWire/ALSA), **so that** device access is correct on
   every platform.

3. **US-5.5.1.3** — **As a** engine developer (P-26), **I want** Opus encoding at configurable
   bitrates (6-64 kbps) with decoded streams routed into the mixer bus hierarchy, **so that** voice
   chat is bandwidth-efficient and integrated with the audio pipeline.

4. **US-5.5.1.4** — **As a** game designer (P-5), **I want** voice chat quality selectable in
   settings, **so that** players can trade quality for bandwidth.

5. **US-5.5.1.5** — **As a** player (P-23), **I want** low-latency voice chat with nearby players
   sounding spatially correct, **so that** social interaction is natural and immersive.

6. **US-5.5.2.1** — **As a** engine developer (P-26), **I want** incoming voice packets buffered in
   an adaptive jitter buffer that adjusts depth based on observed jitter, **so that** latency is
   minimized while preventing dropouts.

7. **US-5.5.2.2** — **As a** engine developer (P-26), **I want** Opus built-in PLC applied when
   packets are lost or arrive late, **so that** gaps are masked without silence exceeding one Opus
   frame.

8. **US-5.5.2.3** — **As a** game designer (P-5), **I want** jitter buffer depth configurable in
   settings, **so that** advanced users can tune latency vs. stability.

9. **US-5.5.2.4** — **As a** player (P-23), **I want** voice chat to sound smooth without gaps even
   under poor network conditions, **so that** communication remains intelligible.

10. **US-5.5.3.1** — **As a** engine developer (P-26), **I want** a lightweight VAD gating
    transmission within 20 ms of speech onset, **so that** bandwidth and mixer load are reduced when
    a player is silent.

11. **US-5.5.3.2** — **As a** engine developer (P-26), **I want** noise suppression attenuating
    keyboard clicks and fans before encoding, improving SNR by at least 20 dB, **so that**
    intelligibility is improved for all participants.

12. **US-5.5.3.3** — **As a** audio designer (P-14), **I want** suppression aggressiveness
    configurable, **so that** noise removal vs. voice quality is tunable per project.

13. **US-5.5.3.4** — **As a** player (P-23), **I want** others' voices clear without background
    noise or keyboard clicks, **so that** communication is intelligible.

14. **US-5.5.4.1** — **As a** engine developer (P-26), **I want** TTS generating speech from text
    using platform-native services (SAPI, AVSpeechSynthesizer, Speech Dispatcher), **so that**
    accessibility needs are met on every platform.

15. **US-5.5.4.2** — **As a** audio designer (P-14), **I want** TTS output routed through the mixer
    as a standard sound source with configurable voice, speed, and pitch, **so that** generated
    speech uses the full audio pipeline.

16. **US-5.5.4.3** — **As a** game designer (P-5), **I want** TTS toggleable in accessibility
    settings, **so that** players opt in as needed.

17. **US-5.5.4.4** — **As a** player (P-23), **I want** UI text and chat messages read aloud on
    demand, **so that** I can play with visual impairments.

18. **US-5.5.5.1** — **As a** engine developer (P-26), **I want** real-time phoneme-to-viseme
    extraction from audio streams with timestamps within 30 ms for pre-recorded and 50 ms for live,
    **so that** lip-sync data drives facial animation blend shapes.

19. **US-5.5.5.2** — **As a** audio designer (P-14), **I want** viseme-to-blend-shape mappings
    configurable per character rig, **so that** each character's mouth animation matches their
    model.

20. **US-5.5.5.3** — **As a** engine developer (P-26), **I want** active lip-synced character count
    scaled per tier (mobile 1-2, Switch 4, desktop 8+) with distant characters using random mouth
    animation, **so that** budget is controlled.

21. **US-5.5.5.4** — **As a** player (P-23), **I want** NPC lips matching speech and my avatar's
    lips moving when I speak, **so that** dialogue feels natural and voice chat is visually
    represented.

22. **US-5.5.6.1** — **As a** audio designer (P-14), **I want** voice-over lines with synchronized
    subtitles and a priority queue preventing overlap, **so that** dialogue is accessible and
    intelligible.

23. **US-5.5.6.2** — **As a** audio designer (P-14), **I want** priority classes per line (critical
    narrative, bark, ambient chatter) with configurable expiry timeout, **so that** stale barks do
    not play late and critical lines are never missed.

24. **US-5.5.6.3** — **As a** engine developer (P-26), **I want** higher-priority lines to interrupt
    lower-priority lines with subtitle events firing within 16 ms of voice-over playback,
    **so that** speech priority and subtitle sync are reliable.

25. **US-5.5.6.4** — **As a** game designer (P-5), **I want** line priority and expiry timeout
    configurable in the editor, **so that** dialogue management is data-driven.

26. **US-5.5.6.5** — **As a** player (P-23), **I want** NPC dialogue to be clear, non-overlapping,
    and subtitled, **so that** I can follow the story with hearing difficulties.

27. **US-5.5.7.1** — **As a** audio designer (P-14), **I want** dialogue trees as directed graphs
    with condition-gated edges, random variation nodes, and loopback edges, **so that** NPCs have
    branching, repeatable conversations.

28. **US-5.5.7.2** — **As a** engine developer (P-26), **I want** conditions evaluated against
    gameplay state (quest progress, reputation, inventory) with each node referencing a voice-over
    asset and subtitle text, **so that** dialogue reacts to player choices.

29. **US-5.5.7.3** — **As a** game designer (P-5), **I want** to author dialogue trees and edge
    conditions visually in the editor, **so that** branching narrative is designed without code.

30. **US-5.5.7.4** — **As a** player (P-23), **I want** NPCs to respond differently based on my
    choices, reputation, and quest progress, **so that** conversations feel personalized.

31. **US-5.5.8.1** — **As a** engine developer (P-26), **I want** multiple concurrent voice channels
    with per-channel permissions, independent volume, and per-speaker muting, **so that** chat is
    organized by social context.

32. **US-5.5.8.2** — **As a** engine developer (P-26), **I want** proximity channels using distance
    attenuation via the shared spatial index and membership replicated via networking, **so that**
    spatial voice and state sync are unified with existing systems.

33. **US-5.5.8.3** — **As a** game designer (P-5), **I want** proximity, party, guild, raid,
    broadcast, and custom channel types with kick/mute/ban controls in the editor, **so that**
    communication covers all social needs.

34. **US-5.5.8.4** — **As a** player (P-23), **I want** a private party voice channel separate from
    proximity chat, **so that** group coordination is distance-independent.

35. **US-5.5.8.5** — **As a** player (P-23), **I want** to mute disruptive speakers and monitor
    multiple channels simultaneously with independent volume, **so that** I control my voice chat
    experience.

36. **US-5.5.9.1** — **As a** engine developer (P-26), **I want** AEC subtracting speaker output
    from mic input using NLMS and non-linear processing with sub-millisecond latency, **so that**
    echo is eliminated for speaker users.

37. **US-5.5.9.2** — **As a** engine developer (P-26), **I want** the adaptive filter to reconverge
    within 2 seconds after room changes and comfort noise to fill gaps, **so that** AEC adapts to
    changing environments without unnatural silence.

38. **US-5.5.9.3** — **As a** game designer (P-5), **I want** AEC toggleable in settings with
    platform-native AEC used on iOS and Android, **so that** echo cancellation is accessible and
    platform-optimal.

39. **US-5.5.9.4** — **As a** player (P-23), **I want** echo-free voice chat when using speakers,
    **so that** headphones are not required for clear communication.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-5.5.1 | audio designer (P-14) |
| US-5.5.2 | engine developer (P-26) |
| US-5.5.3 | engine developer (P-26) |
| US-5.5.4 | engine developer (P-26) |
| US-5.5.5 | engine developer (P-26) |
| US-5.5.6 | audio designer (P-14) |
| US-5.5.7 | audio designer (P-14) |
| US-5.5.8 | engine developer (P-26) |
| US-5.5.9 | engine developer (P-26) |

1. **US-5.5.1** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.5.1.1 through US-5.5.1.5 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-5.5.2** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-5.5.2.1 through US-5.5.2.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

3. **US-5.5.3** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-5.5.3.1 through US-5.5.3.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

4. **US-5.5.4** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-5.5.4.1 through US-5.5.4.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

5. **US-5.5.5** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-5.5.5.1 through US-5.5.5.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

6. **US-5.5.6** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.5.6.1 through US-5.5.6.5 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

7. **US-5.5.7** -- **As a** audio designer (P-14), **I want** the capabilities defined in
   sub-stories US-5.5.7.1 through US-5.5.7.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

8. **US-5.5.8** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-5.5.8.1 through US-5.5.8.5 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

9. **US-5.5.9** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-5.5.9.1 through US-5.5.9.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.
