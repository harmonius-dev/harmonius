# User Stories — 5.5 Voice & Speech

## F-5.5.1

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.5.1.1  | engine developer (P-26) | F-5.5.1  | R-5.5.1      |
| US-5.5.1.2  | engine developer (P-26) | F-5.5.1  | R-5.5.1      |
| US-5.5.1.3  | audio designer (P-14)   | F-5.5.1  | R-5.5.1      |
| US-5.5.1.4  | player (P-23)           | F-5.5.1  | R-5.5.1      |
| US-5.5.1.5  | engine developer (P-26) | F-5.5.1  | R-5.5.1      |
| US-5.5.1.6  | audio designer (P-14)   | F-5.5.1  | R-5.5.1      |
| US-5.5.1.7  | engine developer (P-26) | F-5.5.1  | R-5.5.1      |
| US-5.5.1.8  | engine tester (P-27)    | F-5.5.1  | R-5.5.1      |
| US-5.5.1.9  | player (P-23)           | F-5.5.1  | R-5.5.1      |
| US-5.5.1.10 | designer (P-5)          | F-5.5.1  | R-5.5.1      |
| US-5.5.1.11 | engine tester (P-27)    | F-5.5.1  | R-5.5.1      |
| US-5.5.1.12 | engine developer (P-26) | F-5.5.1  | R-5.5.1      |

1. **US-5.5.1.1** — I want to capture microphone input and encode with Opus at configurable bitrates
   - **Acceptance:** voice chat transmission is bandwidth-efficient
2. **US-5.5.1.2** — I want voice packets transmitted over the engine's networking layer with minimal
   latency
   - **Acceptance:** real-time chat works
3. **US-5.5.1.3** — I want proximity chat routed as spatialized 3D sources
   - **Acceptance:** nearby player voices come from their positions
4. **US-5.5.1.4** — I want low-latency voice chat with nearby players
   - **Acceptance:** social interaction is natural and immersive
5. **US-5.5.1.5** — I want party/raid channels routed as non-spatialized bus sends
   - **Acceptance:** group communication is distance-independent
6. **US-5.5.1.6** — I want Opus bitrate configurable (6-64 kbps)
   - **Acceptance:** quality vs bandwidth is tunable per channel
7. **US-5.5.1.7** — I want WASAPI on Windows, CoreAudio on macOS, and PipeWire/ALSA on Linux for mic
   capture
   - **Acceptance:** capture is platform-native
8. **US-5.5.1.8** — I want to measure end-to-end voice chat latency
   - **Acceptance:** round-trip delay stays under acceptable thresholds
9. **US-5.5.1.9** — I want party members' voices clear regardless of distance
   - **Acceptance:** group coordination works
10. **US-5.5.1.10** — I want voice chat quality selectable in settings
    - **Acceptance:** players can trade quality for bandwidth
11. **US-5.5.1.11** — I want to test dozens of simultaneous voice streams
    - **Acceptance:** raid scenarios are validated
12. **US-5.5.1.12** — I want incoming Opus streams decoded and routed into the mixer
    - **Acceptance:** received voice plays through the audio system

## F-5.5.2

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.5.2.1  | engine developer (P-26) | F-5.5.2  | R-5.5.2      |
| US-5.5.2.2  | engine developer (P-26) | F-5.5.2  | R-5.5.2      |
| US-5.5.2.3  | player (P-23)           | F-5.5.2  | R-5.5.2      |
| US-5.5.2.4  | engine developer (P-26) | F-5.5.2  | R-5.5.2      |
| US-5.5.2.5  | engine tester (P-27)    | F-5.5.2  | R-5.5.2      |
| US-5.5.2.6  | engine tester (P-27)    | F-5.5.2  | R-5.5.2      |
| US-5.5.2.7  | designer (P-5)          | F-5.5.2  | R-5.5.2      |
| US-5.5.2.8  | player (P-23)           | F-5.5.2  | R-5.5.2      |
| US-5.5.2.9  | engine tester (P-27)    | F-5.5.2  | R-5.5.2      |
| US-5.5.2.10 | engine developer (P-26) | F-5.5.2  | R-5.5.2      |
| US-5.5.2.11 | audio designer (P-14)   | F-5.5.2  | R-5.5.2      |
| US-5.5.2.12 | engine tester (P-27)    | F-5.5.2  | R-5.5.2      |

1. **US-5.5.2.1** — I want incoming voice packets buffered in an adaptive jitter buffer
   - **Acceptance:** network jitter does not cause dropouts
2. **US-5.5.2.2** — I want buffer depth adjusted dynamically based on observed jitter
   - **Acceptance:** latency is minimized while preventing dropouts
3. **US-5.5.2.3** — I want voice chat to sound smooth without gaps
   - **Acceptance:** conversation is natural
4. **US-5.5.2.4** — I want PLC via Opus decoder interpolation when packets are lost
   - **Acceptance:** gaps are masked
5. **US-5.5.2.5** — I want to confirm mobile uses higher default jitter buffer depth
   - **Acceptance:** cellular variance is accommodated
6. **US-5.5.2.6** — I want to test voice chat under 5%, 10%, and 20% packet loss
   - **Acceptance:** PLC handles degradation gracefully
7. **US-5.5.2.7** — I want jitter buffer depth configurable
   - **Acceptance:** advanced users can tune latency vs stability
8. **US-5.5.2.8** — I want voice chat intelligible under poor conditions
   - **Acceptance:** communication works despite lag
9. **US-5.5.2.9** — I want to verify PLC produces acceptable quality during packet loss
   - **Acceptance:** concealment is effective
10. **US-5.5.2.10** — I want the adaptive algorithm to converge within seconds of network changes
    - **Acceptance:** depth adjusts quickly
11. **US-5.5.2.11** — I want jitter statistics for debugging
    - **Acceptance:** quality issues can be diagnosed
12. **US-5.5.2.12** — I want to test under sudden network spikes
    - **Acceptance:** adaptive depth handles transients

## F-5.5.3

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.5.3.1  | engine developer (P-26) | F-5.5.3  | R-5.5.3      |
| US-5.5.3.2  | engine developer (P-26) | F-5.5.3  | R-5.5.3      |
| US-5.5.3.3  | player (P-23)           | F-5.5.3  | R-5.5.3      |
| US-5.5.3.4  | engine developer (P-26) | F-5.5.3  | R-5.5.3      |
| US-5.5.3.5  | engine developer (P-26) | F-5.5.3  | R-5.5.3      |
| US-5.5.3.6  | engine tester (P-27)    | F-5.5.3  | R-5.5.3      |
| US-5.5.3.7  | audio designer (P-14)   | F-5.5.3  | R-5.5.3      |
| US-5.5.3.8  | engine tester (P-27)    | F-5.5.3  | R-5.5.3      |
| US-5.5.3.9  | player (P-23)           | F-5.5.3  | R-5.5.3      |
| US-5.5.3.10 | designer (P-5)          | F-5.5.3  | R-5.5.3      |
| US-5.5.3.11 | engine tester (P-27)    | F-5.5.3  | R-5.5.3      |
| US-5.5.3.12 | engine tester (P-27)    | F-5.5.3  | R-5.5.3      |

1. **US-5.5.3.1** — I want a lightweight VAD to gate transmission
   - **Acceptance:** bandwidth and mixer load are reduced when silent
2. **US-5.5.3.2** — I want noise suppression to attenuate keyboard clicks and fans
   - **Acceptance:** intelligibility is improved
3. **US-5.5.3.3** — I want others' voices clear without background noise
   - **Acceptance:** communication is intelligible
4. **US-5.5.3.4** — I want lighter noise suppression on mobile
   - **Acceptance:** CPU overhead is reduced
5. **US-5.5.3.5** — I want platform-native noise suppression on iOS and Android
   - **Acceptance:** OS integration is leveraged
6. **US-5.5.3.6** — I want to verify VAD gates transmission during silence
   - **Acceptance:** no silent packets are sent
7. **US-5.5.3.7** — I want suppression aggressiveness configurable
   - **Acceptance:** noise removal vs voice quality is tunable
8. **US-5.5.3.8** — I want to test VAD with quiet speech and loud noise
   - **Acceptance:** detection thresholds are validated
9. **US-5.5.3.9** — I want keyboard clicks filtered from voice chat
   - **Acceptance:** others hear only my voice
10. **US-5.5.3.10** — I want VAD sensitivity configurable
    - **Acceptance:** players adjust for their environment
11. **US-5.5.3.11** — I want to profile noise suppression CPU cost per platform
    - **Acceptance:** overhead is within budget
12. **US-5.5.3.12** — I want to verify VAD and noise suppression work together
    - **Acceptance:** both features compose correctly

## F-5.5.4

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.5.4.1  | engine developer (P-26) | F-5.5.4  | R-5.5.4      |
| US-5.5.4.2  | engine developer (P-26) | F-5.5.4  | R-5.5.4      |
| US-5.5.4.3  | player (P-23)           | F-5.5.4  | R-5.5.4      |
| US-5.5.4.4  | engine developer (P-26) | F-5.5.4  | R-5.5.4      |
| US-5.5.4.5  | engine tester (P-27)    | F-5.5.4  | R-5.5.4      |
| US-5.5.4.6  | audio designer (P-14)   | F-5.5.4  | R-5.5.4      |
| US-5.5.4.7  | designer (P-5)          | F-5.5.4  | R-5.5.4      |
| US-5.5.4.8  | engine tester (P-27)    | F-5.5.4  | R-5.5.4      |
| US-5.5.4.9  | player (P-23)           | F-5.5.4  | R-5.5.4      |
| US-5.5.4.10 | designer (P-5)          | F-5.5.4  | R-5.5.4      |
| US-5.5.4.11 | engine developer (P-26) | F-5.5.4  | R-5.5.4      |
| US-5.5.4.12 | engine tester (P-27)    | F-5.5.4  | R-5.5.4      |

1. **US-5.5.4.1** — I want TTS generating speech from text using platform services
   - **Acceptance:** accessibility needs are met
2. **US-5.5.4.2** — I want TTS output fed into the mixer as a sound source
   - **Acceptance:** generated speech uses the audio pipeline
3. **US-5.5.4.3** — I want UI text read aloud on demand
   - **Acceptance:** I can play with visual impairments
4. **US-5.5.4.4** — I want SAPI on Windows, AVSpeechSynthesizer on macOS, Speech Dispatcher on Linux
   - **Acceptance:** TTS uses native APIs
5. **US-5.5.4.5** — I want to verify TTS works on all platforms
   - **Acceptance:** accessibility is cross-platform
6. **US-5.5.4.6** — I want TTS for placeholder dialogue during dev
   - **Acceptance:** VO recordings are not needed for testing
7. **US-5.5.4.7** — I want TTS voice, speed, and pitch configurable
   - **Acceptance:** speech matches the desired tone
8. **US-5.5.4.8** — I want to test TTS with long strings
   - **Acceptance:** extended passages render correctly
9. **US-5.5.4.9** — I want chat messages read aloud optionally
   - **Acceptance:** I follow chat without reading
10. **US-5.5.4.10** — I want TTS toggleable in accessibility settings
    - **Acceptance:** players opt in as needed
11. **US-5.5.4.11** — I want TTS requests queued
    - **Acceptance:** multiple requests play sequentially
12. **US-5.5.4.12** — I want to measure TTS latency
    - **Acceptance:** response time is acceptable

## F-5.5.5

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.5.5.1  | engine developer (P-26) | F-5.5.5  | R-5.5.5      |
| US-5.5.5.2  | audio designer (P-14)   | F-5.5.5  | R-5.5.5      |
| US-5.5.5.3  | player (P-23)           | F-5.5.5  | R-5.5.5      |
| US-5.5.5.4  | engine developer (P-26) | F-5.5.5  | R-5.5.5      |
| US-5.5.5.5  | engine developer (P-26) | F-5.5.5  | R-5.5.5      |
| US-5.5.5.6  | engine tester (P-27)    | F-5.5.5  | R-5.5.5      |
| US-5.5.5.7  | engine developer (P-26) | F-5.5.5  | R-5.5.5      |
| US-5.5.5.8  | player (P-23)           | F-5.5.5  | R-5.5.5      |
| US-5.5.5.9  | engine developer (P-26) | F-5.5.5  | R-5.5.5      |
| US-5.5.5.10 | engine tester (P-27)    | F-5.5.5  | R-5.5.5      |
| US-5.5.5.11 | designer (P-5)          | F-5.5.5  | R-5.5.5      |
| US-5.5.5.12 | audio designer (P-14)   | F-5.5.5  | R-5.5.5      |

1. **US-5.5.5.1** — I want real-time phoneme-to-viseme extraction
   - **Acceptance:** lip sync data is available
2. **US-5.5.5.2** — I want viseme output driving facial blend shapes
   - **Acceptance:** character mouths match speech
3. **US-5.5.5.3** — I want NPC lips moving matching speech
   - **Acceptance:** dialogue feels natural
4. **US-5.5.5.4** — I want visemes from pre-recorded dialogue at load time
   - **Acceptance:** NPC lip sync is pre-computed
5. **US-5.5.5.5** — I want real-time visemes from live voice chat
   - **Acceptance:** player avatars lip-sync
6. **US-5.5.5.6** — I want to confirm mobile supports 1-2 lip-synced characters, Switch 4, desktop
   8+
   - **Acceptance:** budget scales
7. **US-5.5.5.7** — I want timestamped viseme tracks consumed by animation
   - **Acceptance:** lip sync drives facial animation
8. **US-5.5.5.8** — I want my avatar's lips moving when I speak
   - **Acceptance:** voice chat is visually represented
9. **US-5.5.5.9** — I want distant characters on mobile to use random mouth animation
   - **Acceptance:** budget is controlled
10. **US-5.5.5.10** — I want to test viseme accuracy against known phonemes
    - **Acceptance:** lip sync is correct
11. **US-5.5.5.11** — I want lip-sync quality configurable
    - **Acceptance:** performance vs accuracy is tunable
12. **US-5.5.5.12** — I want viseme-to-blend-shape mappings per rig
    - **Acceptance:** each character's mouth matches their model

## F-5.5.6

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.5.6.1  | audio designer (P-14)   | F-5.5.6  | R-5.5.6      |
| US-5.5.6.2  | engine developer (P-26) | F-5.5.6  | R-5.5.6      |
| US-5.5.6.3  | player (P-23)           | F-5.5.6  | R-5.5.6      |
| US-5.5.6.4  | audio designer (P-14)   | F-5.5.6  | R-5.5.6      |
| US-5.5.6.5  | engine developer (P-26) | F-5.5.6  | R-5.5.6      |
| US-5.5.6.6  | engine tester (P-27)    | F-5.5.6  | R-5.5.6      |
| US-5.5.6.7  | designer (P-5)          | F-5.5.6  | R-5.5.6      |
| US-5.5.6.8  | engine tester (P-27)    | F-5.5.6  | R-5.5.6      |
| US-5.5.6.9  | player (P-23)           | F-5.5.6  | R-5.5.6      |
| US-5.5.6.10 | engine developer (P-26) | F-5.5.6  | R-5.5.6      |
| US-5.5.6.11 | audio designer (P-14)   | F-5.5.6  | R-5.5.6      |
| US-5.5.6.12 | engine tester (P-27)    | F-5.5.6  | R-5.5.6      |

1. **US-5.5.6.1** — I want VO lines with synchronized subtitles
   - **Acceptance:** dialogue is accessible
2. **US-5.5.6.2** — I want a priority queue preventing overlap
   - **Acceptance:** critical lines are not overwritten
3. **US-5.5.6.3** — I want NPC dialogue clear and non-overlapping
   - **Acceptance:** story is understandable
4. **US-5.5.6.4** — I want priority per line (critical, bark, chatter)
   - **Acceptance:** importance ordering is explicit
5. **US-5.5.6.5** — I want queued lines expired after timeout
   - **Acceptance:** stale barks do not play late
6. **US-5.5.6.6** — I want to verify queue depth is uniform across platforms
   - **Acceptance:** logic is consistent
7. **US-5.5.6.7** — I want line expiry timeout configurable
   - **Acceptance:** stale timing is tunable
8. **US-5.5.6.8** — I want to trigger high-priority lines during low-priority and verify
   interruption
   - **Acceptance:** priority works
9. **US-5.5.6.9** — I want subtitles during dialogue
   - **Acceptance:** I follow speech with hearing difficulties
10. **US-5.5.6.10** — I want lower-priority lines deferred during higher- priority playback
    - **Acceptance:** important speech is uninterrupted
11. **US-5.5.6.11** — I want subtitles synchronized with VO
    - **Acceptance:** text matches spoken words
12. **US-5.5.6.12** — I want to verify VO streaming draws from voice pool
    - **Acceptance:** dialogue respects voice limits

## F-5.5.7

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.5.7.1  | engine developer (P-26) | F-5.5.7  | R-5.5.7      |
| US-5.5.7.2  | engine developer (P-26) | F-5.5.7  | R-5.5.7      |
| US-5.5.7.3  | player (P-23)           | F-5.5.7  | R-5.5.7      |
| US-5.5.7.4  | audio designer (P-14)   | F-5.5.7  | R-5.5.7      |
| US-5.5.7.5  | engine developer (P-26) | F-5.5.7  | R-5.5.7      |
| US-5.5.7.6  | engine tester (P-27)    | F-5.5.7  | R-5.5.7      |
| US-5.5.7.7  | engine developer (P-26) | F-5.5.7  | R-5.5.7      |
| US-5.5.7.8  | engine developer (P-26) | F-5.5.7  | R-5.5.7      |
| US-5.5.7.9  | player (P-23)           | F-5.5.7  | R-5.5.7      |
| US-5.5.7.10 | designer (P-5)          | F-5.5.7  | R-5.5.7      |
| US-5.5.7.11 | engine tester (P-27)    | F-5.5.7  | R-5.5.7      |
| US-5.5.7.12 | engine developer (P-26) | F-5.5.7  | R-5.5.7      |

1. **US-5.5.7.1** — I want dialogue trees as directed graphs with condition-gated edges
   - **Acceptance:** branching works
2. **US-5.5.7.2** — I want conditions evaluated against quest progress, reputation, and inventory
   - **Acceptance:** dialogue reacts to state
3. **US-5.5.7.3** — I want to choose dialogue options
   - **Acceptance:** I influence conversations
4. **US-5.5.7.4** — I want to author dialogue trees visually
   - **Acceptance:** branching is intuitive
5. **US-5.5.7.5** — I want random variation nodes
   - **Acceptance:** NPCs have varied responses
6. **US-5.5.7.6** — I want to verify graph evaluation is lightweight
   - **Acceptance:** no platform scaling is needed
7. **US-5.5.7.7** — I want each node referencing VO and subtitle text
   - **Acceptance:** audio and text are paired
8. **US-5.5.7.8** — I want loopback edges
   - **Acceptance:** merchants and quest givers can be revisited
9. **US-5.5.7.9** — I want NPCs to respond differently based on reputation
   - **Acceptance:** choices matter
10. **US-5.5.7.10** — I want edge conditions configurable visually
    - **Acceptance:** dialogue gating is code-free
11. **US-5.5.7.11** — I want to test all dialogue paths
    - **Acceptance:** every branch produces valid audio and text
12. **US-5.5.7.12** — I want playback to wait for input on choice nodes
    - **Acceptance:** branching is interactive

## F-5.5.8

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.5.8.1  | engine developer (P-26) | F-5.5.8  | R-5.5.8      |
| US-5.5.8.2  | designer (P-5)          | F-5.5.8  | R-5.5.8      |
| US-5.5.8.3  | player (P-23)           | F-5.5.8  | R-5.5.8      |
| US-5.5.8.4  | player (P-23)           | F-5.5.8  | R-5.5.8      |
| US-5.5.8.5  | player (P-23)           | F-5.5.8  | R-5.5.8      |
| US-5.5.8.6  | engine developer (P-26) | F-5.5.8  | R-5.5.8      |
| US-5.5.8.7  | engine developer (P-26) | F-5.5.8  | R-5.5.8      |
| US-5.5.8.8  | engine tester (P-27)    | F-5.5.8  | R-5.5.8      |
| US-5.5.8.9  | designer (P-5)          | F-5.5.8  | R-5.5.8      |
| US-5.5.8.10 | engine developer (P-26) | F-5.5.8  | R-5.5.8      |
| US-5.5.8.11 | engine tester (P-27)    | F-5.5.8  | R-5.5.8      |
| US-5.5.8.12 | audio designer (P-14)   | F-5.5.8  | R-5.5.8      |

1. **US-5.5.8.1** — I want multiple concurrent voice channels with permissions
   - **Acceptance:** chat is organized
2. **US-5.5.8.2** — I want proximity, party, guild, raid, broadcast, and custom channels
   - **Acceptance:** communication covers all needs
3. **US-5.5.8.3** — I want a private party voice channel
   - **Acceptance:** group chat is separate from proximity
4. **US-5.5.8.4** — I want to monitor multiple channels with configurable mix levels
   - **Acceptance:** I hear party and proximity separately
5. **US-5.5.8.5** — I want to mute speakers per channel
   - **Acceptance:** disruptive players are silenced
6. **US-5.5.8.6** — I want raid leader voice to override chatter
   - **Acceptance:** leadership is heard
7. **US-5.5.8.7** — I want membership replicated via networking
   - **Acceptance:** channel state is synchronized
8. **US-5.5.8.8** — I want to verify consoles use native party chat APIs
   - **Acceptance:** certification is met
9. **US-5.5.8.9** — I want kick, mute, and ban controls per channel
   - **Acceptance:** moderation is possible
10. **US-5.5.8.10** — I want proximity chat using distance attenuation via shared spatial index
    - **Acceptance:** nearby voices are louder
11. **US-5.5.8.11** — I want to test many concurrent channels
    - **Acceptance:** scaling is validated
12. **US-5.5.8.12** — I want independent volume per channel
    - **Acceptance:** each channel's loudness is adjustable

## F-5.5.9

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-5.5.9.1  | engine developer (P-26) | F-5.5.9  | R-5.5.9      |
| US-5.5.9.2  | engine developer (P-26) | F-5.5.9  | R-5.5.9      |
| US-5.5.9.3  | player (P-23)           | F-5.5.9  | R-5.5.9      |
| US-5.5.9.4  | engine developer (P-26) | F-5.5.9  | R-5.5.9      |
| US-5.5.9.5  | engine developer (P-26) | F-5.5.9  | R-5.5.9      |
| US-5.5.9.6  | engine tester (P-27)    | F-5.5.9  | R-5.5.9      |
| US-5.5.9.7  | engine developer (P-26) | F-5.5.9  | R-5.5.9      |
| US-5.5.9.8  | engine tester (P-27)    | F-5.5.9  | R-5.5.9      |
| US-5.5.9.9  | player (P-23)           | F-5.5.9  | R-5.5.9      |
| US-5.5.9.10 | designer (P-5)          | F-5.5.9  | R-5.5.9      |
| US-5.5.9.11 | engine tester (P-27)    | F-5.5.9  | R-5.5.9      |
| US-5.5.9.12 | engine developer (P-26) | F-5.5.9  | R-5.5.9      |

1. **US-5.5.9.1** — I want AEC subtracting speaker output from mic input
   - **Acceptance:** echo is eliminated
2. **US-5.5.9.2** — I want NLMS and non-linear processing
   - **Acceptance:** loudspeaker distortion is handled
3. **US-5.5.9.3** — I want no echo when using speakers
   - **Acceptance:** chat is clean
4. **US-5.5.9.4** — I want AEC adapting to changing environments
   - **Acceptance:** room changes are handled
5. **US-5.5.9.5** — I want comfort noise filling gaps
   - **Acceptance:** silence after echo removal is not unnatural
6. **US-5.5.9.6** — I want to verify AEC adds sub-millisecond latency
   - **Acceptance:** no perceptible delay is introduced
7. **US-5.5.9.7** — I want platform AEC used on iOS and Android
   - **Acceptance:** OS integration is leveraged
8. **US-5.5.9.8** — I want to test AEC after volume adjustments
   - **Acceptance:** filter adapts to level changes
9. **US-5.5.9.9** — I want echo-free chat on speakers
   - **Acceptance:** headphones are not required for clear communication
10. **US-5.5.9.10** — I want AEC toggleable in settings
    - **Acceptance:** players control echo cancellation
11. **US-5.5.9.11** — I want to test custom AEC where platform AEC is unavailable
    - **Acceptance:** fallback is validated
12. **US-5.5.9.12** — I want AEC on the audio thread
    - **Acceptance:** latency is minimal and processing is real-time
