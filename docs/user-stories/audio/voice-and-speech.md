# User Stories — 5.5 Voice & Speech

## F-5.5.1

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.5.1.1 | engine developer (P-26) | I want to capture microphone input and encode with Opus at configurable bitrates | voice chat transmission is bandwidth-efficient | F-5.5.1 | R-5.5.1 |
| US-5.5.1.2 | engine developer (P-26) | I want voice packets transmitted over the engine's networking layer with minimal latency | real-time chat works | F-5.5.1 | R-5.5.1 |
| US-5.5.1.3 | audio designer (P-14) | I want proximity chat routed as spatialized 3D sources | nearby player voices come from their positions | F-5.5.1 | R-5.5.1 |
| US-5.5.1.4 | player (P-23) | I want low-latency voice chat with nearby players | social interaction is natural and immersive | F-5.5.1 | R-5.5.1 |
| US-5.5.1.5 | engine developer (P-26) | I want party/raid channels routed as non-spatialized bus sends | group communication is distance-independent | F-5.5.1 | R-5.5.1 |
| US-5.5.1.6 | audio designer (P-14) | I want Opus bitrate configurable (6-64 kbps) | quality vs bandwidth is tunable per channel | F-5.5.1 | R-5.5.1 |
| US-5.5.1.7 | engine developer (P-26) | I want WASAPI on Windows, CoreAudio on macOS, and PipeWire/ALSA on Linux for mic capture | capture is platform-native | F-5.5.1 | R-5.5.1 |
| US-5.5.1.8 | engine tester (P-27) | I want to measure end-to-end voice chat latency | round-trip delay stays under acceptable thresholds | F-5.5.1 | R-5.5.1 |
| US-5.5.1.9 | player (P-23) | I want party members' voices clear regardless of distance | group coordination works | F-5.5.1 | R-5.5.1 |
| US-5.5.1.10 | designer (P-5) | I want voice chat quality selectable in settings | players can trade quality for bandwidth | F-5.5.1 | R-5.5.1 |
| US-5.5.1.11 | engine tester (P-27) | I want to test dozens of simultaneous voice streams | raid scenarios are validated | F-5.5.1 | R-5.5.1 |
| US-5.5.1.12 | engine developer (P-26) | I want incoming Opus streams decoded and routed into the mixer | received voice plays through the audio system | F-5.5.1 | R-5.5.1 |

## F-5.5.2

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.5.2.1 | engine developer (P-26) | I want incoming voice packets buffered in an adaptive jitter buffer | network jitter does not cause dropouts | F-5.5.2 | R-5.5.2 |
| US-5.5.2.2 | engine developer (P-26) | I want buffer depth adjusted dynamically based on observed jitter | latency is minimized while preventing dropouts | F-5.5.2 | R-5.5.2 |
| US-5.5.2.3 | player (P-23) | I want voice chat to sound smooth without gaps | conversation is natural | F-5.5.2 | R-5.5.2 |
| US-5.5.2.4 | engine developer (P-26) | I want PLC via Opus decoder interpolation when packets are lost | gaps are masked | F-5.5.2 | R-5.5.2 |
| US-5.5.2.5 | engine tester (P-27) | I want to confirm mobile uses higher default jitter buffer depth | cellular variance is accommodated | F-5.5.2 | R-5.5.2 |
| US-5.5.2.6 | engine tester (P-27) | I want to test voice chat under 5%, 10%, and 20% packet loss | PLC handles degradation gracefully | F-5.5.2 | R-5.5.2 |
| US-5.5.2.7 | designer (P-5) | I want jitter buffer depth configurable | advanced users can tune latency vs stability | F-5.5.2 | R-5.5.2 |
| US-5.5.2.8 | player (P-23) | I want voice chat intelligible under poor conditions | communication works despite lag | F-5.5.2 | R-5.5.2 |
| US-5.5.2.9 | engine tester (P-27) | I want to verify PLC produces acceptable quality during packet loss | concealment is effective | F-5.5.2 | R-5.5.2 |
| US-5.5.2.10 | engine developer (P-26) | I want the adaptive algorithm to converge within seconds of network changes | depth adjusts quickly | F-5.5.2 | R-5.5.2 |
| US-5.5.2.11 | audio designer (P-14) | I want jitter statistics for debugging | quality issues can be diagnosed | F-5.5.2 | R-5.5.2 |
| US-5.5.2.12 | engine tester (P-27) | I want to test under sudden network spikes | adaptive depth handles transients | F-5.5.2 | R-5.5.2 |

## F-5.5.3

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.5.3.1 | engine developer (P-26) | I want a lightweight VAD to gate transmission | bandwidth and mixer load are reduced when silent | F-5.5.3 | R-5.5.3 |
| US-5.5.3.2 | engine developer (P-26) | I want noise suppression to attenuate keyboard clicks and fans | intelligibility is improved | F-5.5.3 | R-5.5.3 |
| US-5.5.3.3 | player (P-23) | I want others' voices clear without background noise | communication is intelligible | F-5.5.3 | R-5.5.3 |
| US-5.5.3.4 | engine developer (P-26) | I want lighter noise suppression on mobile | CPU overhead is reduced | F-5.5.3 | R-5.5.3 |
| US-5.5.3.5 | engine developer (P-26) | I want platform-native noise suppression on iOS and Android | OS integration is leveraged | F-5.5.3 | R-5.5.3 |
| US-5.5.3.6 | engine tester (P-27) | I want to verify VAD gates transmission during silence | no silent packets are sent | F-5.5.3 | R-5.5.3 |
| US-5.5.3.7 | audio designer (P-14) | I want suppression aggressiveness configurable | noise removal vs voice quality is tunable | F-5.5.3 | R-5.5.3 |
| US-5.5.3.8 | engine tester (P-27) | I want to test VAD with quiet speech and loud noise | detection thresholds are validated | F-5.5.3 | R-5.5.3 |
| US-5.5.3.9 | player (P-23) | I want keyboard clicks filtered from voice chat | others hear only my voice | F-5.5.3 | R-5.5.3 |
| US-5.5.3.10 | designer (P-5) | I want VAD sensitivity configurable | players adjust for their environment | F-5.5.3 | R-5.5.3 |
| US-5.5.3.11 | engine tester (P-27) | I want to profile noise suppression CPU cost per platform | overhead is within budget | F-5.5.3 | R-5.5.3 |
| US-5.5.3.12 | engine tester (P-27) | I want to verify VAD and noise suppression work together | both features compose correctly | F-5.5.3 | R-5.5.3 |

## F-5.5.4

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.5.4.1 | engine developer (P-26) | I want TTS generating speech from text using platform services | accessibility needs are met | F-5.5.4 | R-5.5.4 |
| US-5.5.4.2 | engine developer (P-26) | I want TTS output fed into the mixer as a sound source | generated speech uses the audio pipeline | F-5.5.4 | R-5.5.4 |
| US-5.5.4.3 | player (P-23) | I want UI text read aloud on demand | I can play with visual impairments | F-5.5.4 | R-5.5.4 |
| US-5.5.4.4 | engine developer (P-26) | I want SAPI on Windows, AVSpeechSynthesizer on macOS, Speech Dispatcher on Linux | TTS uses native APIs | F-5.5.4 | R-5.5.4 |
| US-5.5.4.5 | engine tester (P-27) | I want to verify TTS works on all platforms | accessibility is cross-platform | F-5.5.4 | R-5.5.4 |
| US-5.5.4.6 | audio designer (P-14) | I want TTS for placeholder dialogue during dev | VO recordings are not needed for testing | F-5.5.4 | R-5.5.4 |
| US-5.5.4.7 | designer (P-5) | I want TTS voice, speed, and pitch configurable | speech matches the desired tone | F-5.5.4 | R-5.5.4 |
| US-5.5.4.8 | engine tester (P-27) | I want to test TTS with long strings | extended passages render correctly | F-5.5.4 | R-5.5.4 |
| US-5.5.4.9 | player (P-23) | I want chat messages read aloud optionally | I follow chat without reading | F-5.5.4 | R-5.5.4 |
| US-5.5.4.10 | designer (P-5) | I want TTS toggleable in accessibility settings | players opt in as needed | F-5.5.4 | R-5.5.4 |
| US-5.5.4.11 | engine developer (P-26) | I want TTS requests queued | multiple requests play sequentially | F-5.5.4 | R-5.5.4 |
| US-5.5.4.12 | engine tester (P-27) | I want to measure TTS latency | response time is acceptable | F-5.5.4 | R-5.5.4 |

## F-5.5.5

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.5.5.1 | engine developer (P-26) | I want real-time phoneme-to-viseme extraction | lip sync data is available | F-5.5.5 | R-5.5.5 |
| US-5.5.5.2 | audio designer (P-14) | I want viseme output driving facial blend shapes | character mouths match speech | F-5.5.5 | R-5.5.5 |
| US-5.5.5.3 | player (P-23) | I want NPC lips moving matching speech | dialogue feels natural | F-5.5.5 | R-5.5.5 |
| US-5.5.5.4 | engine developer (P-26) | I want visemes from pre-recorded dialogue at load time | NPC lip sync is pre-computed | F-5.5.5 | R-5.5.5 |
| US-5.5.5.5 | engine developer (P-26) | I want real-time visemes from live voice chat | player avatars lip-sync | F-5.5.5 | R-5.5.5 |
| US-5.5.5.6 | engine tester (P-27) | I want to confirm mobile supports 1-2 lip-synced characters, Switch 4, desktop 8+ | budget scales | F-5.5.5 | R-5.5.5 |
| US-5.5.5.7 | engine developer (P-26) | I want timestamped viseme tracks consumed by animation | lip sync drives facial animation | F-5.5.5 | R-5.5.5 |
| US-5.5.5.8 | player (P-23) | I want my avatar's lips moving when I speak | voice chat is visually represented | F-5.5.5 | R-5.5.5 |
| US-5.5.5.9 | engine developer (P-26) | I want distant characters on mobile to use random mouth animation | budget is controlled | F-5.5.5 | R-5.5.5 |
| US-5.5.5.10 | engine tester (P-27) | I want to test viseme accuracy against known phonemes | lip sync is correct | F-5.5.5 | R-5.5.5 |
| US-5.5.5.11 | designer (P-5) | I want lip-sync quality configurable | performance vs accuracy is tunable | F-5.5.5 | R-5.5.5 |
| US-5.5.5.12 | audio designer (P-14) | I want viseme-to-blend-shape mappings per rig | each character's mouth matches their model | F-5.5.5 | R-5.5.5 |

## F-5.5.6

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.5.6.1 | audio designer (P-14) | I want VO lines with synchronized subtitles | dialogue is accessible | F-5.5.6 | R-5.5.6 |
| US-5.5.6.2 | engine developer (P-26) | I want a priority queue preventing overlap | critical lines are not overwritten | F-5.5.6 | R-5.5.6 |
| US-5.5.6.3 | player (P-23) | I want NPC dialogue clear and non-overlapping | story is understandable | F-5.5.6 | R-5.5.6 |
| US-5.5.6.4 | audio designer (P-14) | I want priority per line (critical, bark, chatter) | importance ordering is explicit | F-5.5.6 | R-5.5.6 |
| US-5.5.6.5 | engine developer (P-26) | I want queued lines expired after timeout | stale barks do not play late | F-5.5.6 | R-5.5.6 |
| US-5.5.6.6 | engine tester (P-27) | I want to verify queue depth is uniform across platforms | logic is consistent | F-5.5.6 | R-5.5.6 |
| US-5.5.6.7 | designer (P-5) | I want line expiry timeout configurable | stale timing is tunable | F-5.5.6 | R-5.5.6 |
| US-5.5.6.8 | engine tester (P-27) | I want to trigger high-priority lines during low-priority and verify interruption | priority works | F-5.5.6 | R-5.5.6 |
| US-5.5.6.9 | player (P-23) | I want subtitles during dialogue | I follow speech with hearing difficulties | F-5.5.6 | R-5.5.6 |
| US-5.5.6.10 | engine developer (P-26) | I want lower-priority lines deferred during higher- priority playback | important speech is uninterrupted | F-5.5.6 | R-5.5.6 |
| US-5.5.6.11 | audio designer (P-14) | I want subtitles synchronized with VO | text matches spoken words | F-5.5.6 | R-5.5.6 |
| US-5.5.6.12 | engine tester (P-27) | I want to verify VO streaming draws from voice pool | dialogue respects voice limits | F-5.5.6 | R-5.5.6 |

## F-5.5.7

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.5.7.1 | engine developer (P-26) | I want dialogue trees as directed graphs with condition-gated edges | branching works | F-5.5.7 | R-5.5.7 |
| US-5.5.7.2 | engine developer (P-26) | I want conditions evaluated against quest progress, reputation, and inventory | dialogue reacts to state | F-5.5.7 | R-5.5.7 |
| US-5.5.7.3 | player (P-23) | I want to choose dialogue options | I influence conversations | F-5.5.7 | R-5.5.7 |
| US-5.5.7.4 | audio designer (P-14) | I want to author dialogue trees visually | branching is intuitive | F-5.5.7 | R-5.5.7 |
| US-5.5.7.5 | engine developer (P-26) | I want random variation nodes | NPCs have varied responses | F-5.5.7 | R-5.5.7 |
| US-5.5.7.6 | engine tester (P-27) | I want to verify graph evaluation is lightweight | no platform scaling is needed | F-5.5.7 | R-5.5.7 |
| US-5.5.7.7 | engine developer (P-26) | I want each node referencing VO and subtitle text | audio and text are paired | F-5.5.7 | R-5.5.7 |
| US-5.5.7.8 | engine developer (P-26) | I want loopback edges | merchants and quest givers can be revisited | F-5.5.7 | R-5.5.7 |
| US-5.5.7.9 | player (P-23) | I want NPCs to respond differently based on reputation | choices matter | F-5.5.7 | R-5.5.7 |
| US-5.5.7.10 | designer (P-5) | I want edge conditions configurable visually | dialogue gating is code-free | F-5.5.7 | R-5.5.7 |
| US-5.5.7.11 | engine tester (P-27) | I want to test all dialogue paths | every branch produces valid audio and text | F-5.5.7 | R-5.5.7 |
| US-5.5.7.12 | engine developer (P-26) | I want playback to wait for input on choice nodes | branching is interactive | F-5.5.7 | R-5.5.7 |

## F-5.5.8

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.5.8.1 | engine developer (P-26) | I want multiple concurrent voice channels with permissions | chat is organized | F-5.5.8 | R-5.5.8 |
| US-5.5.8.2 | designer (P-5) | I want proximity, party, guild, raid, broadcast, and custom channels | communication covers all needs | F-5.5.8 | R-5.5.8 |
| US-5.5.8.3 | player (P-23) | I want a private party voice channel | group chat is separate from proximity | F-5.5.8 | R-5.5.8 |
| US-5.5.8.4 | player (P-23) | I want to monitor multiple channels with configurable mix levels | I hear party and proximity separately | F-5.5.8 | R-5.5.8 |
| US-5.5.8.5 | player (P-23) | I want to mute speakers per channel | disruptive players are silenced | F-5.5.8 | R-5.5.8 |
| US-5.5.8.6 | engine developer (P-26) | I want raid leader voice to override chatter | leadership is heard | F-5.5.8 | R-5.5.8 |
| US-5.5.8.7 | engine developer (P-26) | I want membership replicated via networking | channel state is synchronized | F-5.5.8 | R-5.5.8 |
| US-5.5.8.8 | engine tester (P-27) | I want to verify consoles use native party chat APIs | certification is met | F-5.5.8 | R-5.5.8 |
| US-5.5.8.9 | designer (P-5) | I want kick, mute, and ban controls per channel | moderation is possible | F-5.5.8 | R-5.5.8 |
| US-5.5.8.10 | engine developer (P-26) | I want proximity chat using distance attenuation via shared spatial index | nearby voices are louder | F-5.5.8 | R-5.5.8 |
| US-5.5.8.11 | engine tester (P-27) | I want to test many concurrent channels | scaling is validated | F-5.5.8 | R-5.5.8 |
| US-5.5.8.12 | audio designer (P-14) | I want independent volume per channel | each channel's loudness is adjustable | F-5.5.8 | R-5.5.8 |

## F-5.5.9

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-5.5.9.1 | engine developer (P-26) | I want AEC subtracting speaker output from mic input | echo is eliminated | F-5.5.9 | R-5.5.9 |
| US-5.5.9.2 | engine developer (P-26) | I want NLMS and non-linear processing | loudspeaker distortion is handled | F-5.5.9 | R-5.5.9 |
| US-5.5.9.3 | player (P-23) | I want no echo when using speakers | chat is clean | F-5.5.9 | R-5.5.9 |
| US-5.5.9.4 | engine developer (P-26) | I want AEC adapting to changing environments | room changes are handled | F-5.5.9 | R-5.5.9 |
| US-5.5.9.5 | engine developer (P-26) | I want comfort noise filling gaps | silence after echo removal is not unnatural | F-5.5.9 | R-5.5.9 |
| US-5.5.9.6 | engine tester (P-27) | I want to verify AEC adds sub-millisecond latency | no perceptible delay is introduced | F-5.5.9 | R-5.5.9 |
| US-5.5.9.7 | engine developer (P-26) | I want platform AEC used on iOS and Android | OS integration is leveraged | F-5.5.9 | R-5.5.9 |
| US-5.5.9.8 | engine tester (P-27) | I want to test AEC after volume adjustments | filter adapts to level changes | F-5.5.9 | R-5.5.9 |
| US-5.5.9.9 | player (P-23) | I want echo-free chat on speakers | headphones are not required for clear communication | F-5.5.9 | R-5.5.9 |
| US-5.5.9.10 | designer (P-5) | I want AEC toggleable in settings | players control echo cancellation | F-5.5.9 | R-5.5.9 |
| US-5.5.9.11 | engine tester (P-27) | I want to test custom AEC where platform AEC is unavailable | fallback is validated | F-5.5.9 | R-5.5.9 |
| US-5.5.9.12 | engine developer (P-26) | I want AEC on the audio thread | latency is minimal and processing is real-time | F-5.5.9 | R-5.5.9 |
