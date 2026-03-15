# User Stories — 5.5 Voice & Speech

## US-5.5.1.1 Capture Microphone and Encode with Opus
**As an** engine developer (P-26), **I want to** capture microphone input and encode with Opus at
configurable bitrates, **so that** voice chat transmission is bandwidth-efficient.

## US-5.5.1.2 Transmit Voice Over Networking Layer
**As an** engine developer (P-26), **I want** voice packets transmitted over the engine's networking
layer with minimal latency, **so that** real-time chat works.

## US-5.5.1.3 Route Proximity Chat as 3D Sources
**As an** audio designer (P-14), **I want** proximity chat routed as spatialized 3D sources, **so
that** nearby player voices come from their positions.

## US-5.5.1.4 Talk to Nearby Players
**As a** player (P-23), **I want** low-latency voice chat with nearby players, **so that** social
interaction is natural and immersive.

## US-5.5.1.5 Route Party Chat as Non-Spatialized
**As an** engine developer (P-26), **I want** party/raid channels routed as non-spatialized bus
sends, **so that** group communication is distance-independent.

## US-5.5.1.6 Configure Opus Bitrate
**As an** audio designer (P-14), **I want** Opus bitrate configurable (6-64 kbps), **so that**
quality vs bandwidth is tunable per channel.

## US-5.5.1.7 Use Platform-Native Mic Capture
**As an** engine developer (P-26), **I want** WASAPI on Windows, CoreAudio on macOS, and
PipeWire/ALSA on Linux for mic capture, **so that** capture is platform-native.

## US-5.5.1.8 Verify Voice Chat Latency
**As an** engine tester (P-27), **I want to** measure end-to-end voice chat latency, **so that**
round-trip delay stays under acceptable thresholds.

## US-5.5.1.9 Hear Party Members Clearly
**As a** player (P-23), **I want** party members' voices clear regardless of distance, **so that**
group coordination works.

## US-5.5.1.10 Configure Voice Chat Quality in Settings
**As a** designer (P-5), **I want** voice chat quality selectable in settings, **so that** players
can trade quality for bandwidth.

## US-5.5.1.11 Test Simultaneous Voice Streams
**As an** engine tester (P-27), **I want to** test dozens of simultaneous voice streams, **so that**
raid scenarios are validated.

## US-5.5.1.12 Decode Incoming Opus Streams
**As an** engine developer (P-26), **I want** incoming Opus streams decoded and routed into the
mixer, **so that** received voice plays through the audio system.

## US-5.5.2.1 Buffer Incoming Packets in Jitter Buffer
**As an** engine developer (P-26), **I want** incoming voice packets buffered in an adaptive jitter
buffer, **so that** network jitter does not cause dropouts.

## US-5.5.2.2 Adjust Buffer Depth Dynamically
**As an** engine developer (P-26), **I want** buffer depth adjusted dynamically based on observed
jitter, **so that** latency is minimized while preventing dropouts.

## US-5.5.2.3 Hear Smooth Voice Without Gaps
**As a** player (P-23), **I want** voice chat to sound smooth without gaps, **so that** conversation
is natural.

## US-5.5.2.4 Apply Packet-Loss Concealment
**As an** engine developer (P-26), **I want** PLC via Opus decoder interpolation when packets are
lost, **so that** gaps are masked.

## US-5.5.2.5 Verify Higher Default Buffer on Mobile
**As an** engine tester (P-27), **I want to** confirm mobile uses higher default jitter buffer
depth, **so that** cellular variance is accommodated.

## US-5.5.2.6 Test Under Packet Loss Conditions
**As an** engine tester (P-27), **I want to** test voice chat under 5%, 10%, and 20% packet loss,
**so that** PLC handles degradation gracefully.

## US-5.5.2.7 Configure Buffer Depth in Settings
**As a** designer (P-5), **I want** jitter buffer depth configurable, **so that** advanced users can
tune latency vs stability.

## US-5.5.2.8 Hear Continuous Voice Under Bad Network
**As a** player (P-23), **I want** voice chat intelligible under poor conditions, **so that**
communication works despite lag.

## US-5.5.2.9 Verify PLC Quality
**As an** engine tester (P-27), **I want to** verify PLC produces acceptable quality during packet
loss, **so that** concealment is effective.

## US-5.5.2.10 Implement Adaptive Buffer Algorithm
**As an** engine developer (P-26), **I want** the adaptive algorithm to converge within seconds of
network changes, **so that** depth adjusts quickly.

## US-5.5.2.11 Monitor Jitter Statistics
**As an** audio designer (P-14), **I want** jitter statistics for debugging, **so that** quality
issues can be diagnosed.

## US-5.5.2.12 Test Under Network Spikes
**As an** engine tester (P-27), **I want to** test under sudden network spikes, **so that** adaptive
depth handles transients.

## US-5.5.3.1 Detect Voice Activity for Gating
**As an** engine developer (P-26), **I want** a lightweight VAD to gate transmission, **so that**
bandwidth and mixer load are reduced when silent.

## US-5.5.3.2 Suppress Background Noise
**As an** engine developer (P-26), **I want** noise suppression to attenuate keyboard clicks and
fans, **so that** intelligibility is improved.

## US-5.5.3.3 Hear Clear Voice Without Background Noise
**As a** player (P-23), **I want** others' voices clear without background noise, **so that**
communication is intelligible.

## US-5.5.3.4 Use Lighter Model on Mobile
**As an** engine developer (P-26), **I want** lighter noise suppression on mobile, **so that** CPU
overhead is reduced.

## US-5.5.3.5 Use Platform-Native Suppression
**As an** engine developer (P-26), **I want** platform-native noise suppression on iOS and Android,
**so that** OS integration is leveraged.

## US-5.5.3.6 Verify VAD Gates Silence
**As an** engine tester (P-27), **I want to** verify VAD gates transmission during silence, **so
that** no silent packets are sent.

## US-5.5.3.7 Configure Suppression Level
**As an** audio designer (P-14), **I want** suppression aggressiveness configurable, **so that**
noise removal vs voice quality is tunable.

## US-5.5.3.8 Test VAD with Various Noise
**As an** engine tester (P-27), **I want to** test VAD with quiet speech and loud noise, **so that**
detection thresholds are validated.

## US-5.5.3.9 Speak Without Keyboard Noise
**As a** player (P-23), **I want** keyboard clicks filtered from voice chat, **so that** others hear
only my voice.

## US-5.5.3.10 Configure VAD Sensitivity
**As a** designer (P-5), **I want** VAD sensitivity configurable, **so that** players adjust for
their environment.

## US-5.5.3.11 Profile Noise Suppression Cost
**As an** engine tester (P-27), **I want to** profile noise suppression CPU cost per platform, **so
that** overhead is within budget.

## US-5.5.3.12 Verify VAD and Suppression Compose
**As an** engine tester (P-27), **I want to** verify VAD and noise suppression work together, **so
that** both features compose correctly.

## US-5.5.4.1 Generate Speech from Text
**As an** engine developer (P-26), **I want** TTS generating speech from text using platform
services, **so that** accessibility needs are met.

## US-5.5.4.2 Feed TTS into Mixer
**As an** engine developer (P-26), **I want** TTS output fed into the mixer as a sound source, **so
that** generated speech uses the audio pipeline.

## US-5.5.4.3 Hear UI Text Read Aloud
**As a** player (P-23), **I want** UI text read aloud on demand, **so that** I can play with visual
impairments.

## US-5.5.4.4 Use Platform TTS Services
**As an** engine developer (P-26), **I want** SAPI on Windows, AVSpeechSynthesizer on macOS, Speech
Dispatcher on Linux, **so that** TTS uses native APIs.

## US-5.5.4.5 Verify TTS on All Platforms
**As an** engine tester (P-27), **I want to** verify TTS works on all platforms, **so that**
accessibility is cross-platform.

## US-5.5.4.6 Use TTS for Placeholder Dialogue
**As an** audio designer (P-14), **I want** TTS for placeholder dialogue during dev, **so that** VO
recordings are not needed for testing.

## US-5.5.4.7 Configure TTS Voice and Speed
**As a** designer (P-5), **I want** TTS voice, speed, and pitch configurable, **so that** speech
matches the desired tone.

## US-5.5.4.8 Test TTS with Long Text
**As an** engine tester (P-27), **I want to** test TTS with long strings, **so that** extended
passages render correctly.

## US-5.5.4.9 Hear Chat Messages Read Aloud
**As a** player (P-23), **I want** chat messages read aloud optionally, **so that** I follow chat
without reading.

## US-5.5.4.10 Configure TTS in Accessibility Settings
**As a** designer (P-5), **I want** TTS toggleable in accessibility settings, **so that** players
opt in as needed.

## US-5.5.4.11 Queue TTS Requests
**As an** engine developer (P-26), **I want** TTS requests queued, **so that** multiple requests
play sequentially.

## US-5.5.4.12 Verify TTS Latency
**As an** engine tester (P-27), **I want to** measure TTS latency, **so that** response time is
acceptable.

## US-5.5.5.1 Extract Visemes from Audio
**As an** engine developer (P-26), **I want** real-time phoneme-to-viseme extraction, **so that**
lip sync data is available.

## US-5.5.5.2 Drive Facial Blend Shapes
**As an** audio designer (P-14), **I want** viseme output driving facial blend shapes, **so that**
character mouths match speech.

## US-5.5.5.3 See NPC Lips Move with Dialogue
**As a** player (P-23), **I want** NPC lips moving matching speech, **so that** dialogue feels
natural.

## US-5.5.5.4 Generate from Pre-Recorded Dialogue
**As an** engine developer (P-26), **I want** visemes from pre-recorded dialogue at load time, **so
that** NPC lip sync is pre-computed.

## US-5.5.5.5 Generate from Live Voice Chat
**As an** engine developer (P-26), **I want** real-time visemes from live voice chat, **so that**
player avatars lip-sync.

## US-5.5.5.6 Verify Character Count Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile supports 1-2 lip-synced characters,
Switch 4, desktop 8+, **so that** budget scales.

## US-5.5.5.7 Output Timestamped Viseme Track
**As an** engine developer (P-26), **I want** timestamped viseme tracks consumed by animation, **so
that** lip sync drives facial animation.

## US-5.5.5.8 See Avatar Lip-Sync in Voice Chat
**As a** player (P-23), **I want** my avatar's lips moving when I speak, **so that** voice chat is
visually represented.

## US-5.5.5.9 Use Random Mouth on Mobile for Distant
**As an** engine developer (P-26), **I want** distant characters on mobile to use random mouth
animation, **so that** budget is controlled.

## US-5.5.5.10 Test Viseme Accuracy
**As an** engine tester (P-27), **I want to** test viseme accuracy against known phonemes, **so
that** lip sync is correct.

## US-5.5.5.11 Configure Lip-Sync Quality
**As a** designer (P-5), **I want** lip-sync quality configurable, **so that** performance vs
accuracy is tunable.

## US-5.5.5.12 Author Viseme Mappings Per Character
**As an** audio designer (P-14), **I want** viseme-to-blend-shape mappings per rig, **so that** each
character's mouth matches their model.

## US-5.5.6.1 Play VO Lines with Subtitles
**As an** audio designer (P-14), **I want** VO lines with synchronized subtitles, **so that**
dialogue is accessible.

## US-5.5.6.2 Manage Priority Queue
**As an** engine developer (P-26), **I want** a priority queue preventing overlap, **so that**
critical lines are not overwritten.

## US-5.5.6.3 Hear Clear NPC Dialogue
**As a** player (P-23), **I want** NPC dialogue clear and non-overlapping, **so that** story is
understandable.

## US-5.5.6.4 Declare Priority Per Line
**As an** audio designer (P-14), **I want** priority per line (critical, bark, chatter), **so that**
importance ordering is explicit.

## US-5.5.6.5 Expire Stale Lines
**As an** engine developer (P-26), **I want** queued lines expired after timeout, **so that** stale
barks do not play late.

## US-5.5.6.6 Verify Queue Depth Uniform
**As an** engine tester (P-27), **I want to** verify queue depth is uniform across platforms, **so
that** logic is consistent.

## US-5.5.6.7 Configure Expiry Timeout
**As a** designer (P-5), **I want** line expiry timeout configurable, **so that** stale timing is
tunable.

## US-5.5.6.8 Test Priority Interruption
**As an** engine tester (P-27), **I want to** trigger high-priority lines during low-priority and
verify interruption, **so that** priority works.

## US-5.5.6.9 Read Subtitles
**As a** player (P-23), **I want** subtitles during dialogue, **so that** I follow speech with
hearing difficulties.

## US-5.5.6.10 Defer Lower-Priority Lines
**As an** engine developer (P-26), **I want** lower-priority lines deferred during higher- priority
playback, **so that** important speech is uninterrupted.

## US-5.5.6.11 Sync Subtitle Timing
**As an** audio designer (P-14), **I want** subtitles synchronized with VO, **so that** text matches
spoken words.

## US-5.5.6.12 Verify VO from Voice Pool
**As an** engine tester (P-27), **I want to** verify VO streaming draws from voice pool, **so that**
dialogue respects voice limits.

## US-5.5.7.1 Represent Dialogue as Directed Graphs
**As an** engine developer (P-26), **I want** dialogue trees as directed graphs with condition-gated
edges, **so that** branching works.

## US-5.5.7.2 Evaluate Conditions Against Game State
**As an** engine developer (P-26), **I want** conditions evaluated against quest progress,
reputation, and inventory, **so that** dialogue reacts to state.

## US-5.5.7.3 Choose Dialogue Options
**As a** player (P-23), **I want to** choose dialogue options, **so that** I influence
conversations.

## US-5.5.7.4 Author Graphs in Visual Editor
**As an** audio designer (P-14), **I want to** author dialogue trees visually, **so that** branching
is intuitive.

## US-5.5.7.5 Support Random Variation Nodes
**As an** engine developer (P-26), **I want** random variation nodes, **so that** NPCs have varied
responses.

## US-5.5.7.6 Verify Lightweight Evaluation
**As an** engine tester (P-27), **I want to** verify graph evaluation is lightweight, **so that** no
platform scaling is needed.

## US-5.5.7.7 Reference VO Per Node
**As an** engine developer (P-26), **I want** each node referencing VO and subtitle text, **so
that** audio and text are paired.

## US-5.5.7.8 Support Loopback for Repeatable NPCs
**As an** engine developer (P-26), **I want** loopback edges, **so that** merchants and quest givers
can be revisited.

## US-5.5.7.9 Hear Different Responses by Reputation
**As a** player (P-23), **I want** NPCs to respond differently based on reputation, **so that**
choices matter.

## US-5.5.7.10 Configure Conditions in Editor
**As a** designer (P-5), **I want** edge conditions configurable visually, **so that** dialogue
gating is code-free.

## US-5.5.7.11 Test All Graph Paths
**As an** engine tester (P-27), **I want to** test all dialogue paths, **so that** every branch
produces valid audio and text.

## US-5.5.7.12 Wait for Player Input on Choices
**As an** engine developer (P-26), **I want** playback to wait for input on choice nodes, **so
that** branching is interactive.

## US-5.5.8.1 Manage Multiple Voice Channels
**As an** engine developer (P-26), **I want** multiple concurrent voice channels with permissions,
**so that** chat is organized.

## US-5.5.8.2 Support Channel Types
**As a** designer (P-5), **I want** proximity, party, guild, raid, broadcast, and custom channels,
**so that** communication covers all needs.

## US-5.5.8.3 Talk in Party Channel
**As a** player (P-23), **I want** a private party voice channel, **so that** group chat is separate
from proximity.

## US-5.5.8.4 Monitor Multiple Channels
**As a** player (P-23), **I want to** monitor multiple channels with configurable mix levels, **so
that** I hear party and proximity separately.

## US-5.5.8.5 Mute Individual Speakers
**As a** player (P-23), **I want to** mute speakers per channel, **so that** disruptive players are
silenced.

## US-5.5.8.6 Override with Leader Priority
**As an** engine developer (P-26), **I want** raid leader voice to override chatter, **so that**
leadership is heard.

## US-5.5.8.7 Replicate Channel Membership
**As an** engine developer (P-26), **I want** membership replicated via networking, **so that**
channel state is synchronized.

## US-5.5.8.8 Use Platform-Native Party Chat
**As an** engine tester (P-27), **I want to** verify consoles use native party chat APIs, **so
that** certification is met.

## US-5.5.8.9 Moderate Channels
**As a** designer (P-5), **I want** kick, mute, and ban controls per channel, **so that** moderation
is possible.

## US-5.5.8.10 Use Distance-Attenuated Proximity
**As an** engine developer (P-26), **I want** proximity chat using distance attenuation via shared
spatial index, **so that** nearby voices are louder.

## US-5.5.8.11 Test Many Channels
**As an** engine tester (P-27), **I want to** test many concurrent channels, **so that** scaling is
validated.

## US-5.5.8.12 Configure Channel Volume
**As an** audio designer (P-14), **I want** independent volume per channel, **so that** each
channel's loudness is adjustable.

## US-5.5.9.1 Cancel Echo from Speakers
**As an** engine developer (P-26), **I want** AEC subtracting speaker output from mic input, **so
that** echo is eliminated.

## US-5.5.9.2 Support Linear and Non-Linear AEC
**As an** engine developer (P-26), **I want** NLMS and non-linear processing, **so that**
loudspeaker distortion is handled.

## US-5.5.9.3 Speak Without Echo
**As a** player (P-23), **I want** no echo when using speakers, **so that** chat is clean.

## US-5.5.9.4 Adapt to Changing Acoustics
**As an** engine developer (P-26), **I want** AEC adapting to changing environments, **so that**
room changes are handled.

## US-5.5.9.5 Generate Comfort Noise
**As an** engine developer (P-26), **I want** comfort noise filling gaps, **so that** silence after
echo removal is not unnatural.

## US-5.5.9.6 Verify Sub-Millisecond Latency
**As an** engine tester (P-27), **I want to** verify AEC adds sub-millisecond latency, **so that**
no perceptible delay is introduced.

## US-5.5.9.7 Defer to Platform AEC
**As an** engine developer (P-26), **I want** platform AEC used on iOS and Android, **so that** OS
integration is leveraged.

## US-5.5.9.8 Test AEC with Volume Changes
**As an** engine tester (P-27), **I want to** test AEC after volume adjustments, **so that** filter
adapts to level changes.

## US-5.5.9.9 Hear Clean Chat on Speakers
**As a** player (P-23), **I want** echo-free chat on speakers, **so that** headphones are not
required for clear communication.

## US-5.5.9.10 Configure AEC in Settings
**As a** designer (P-5), **I want** AEC toggleable in settings, **so that** players control echo
cancellation.

## US-5.5.9.11 Test Custom AEC on PC/Console
**As an** engine tester (P-27), **I want to** test custom AEC where platform AEC is unavailable,
**so that** fallback is validated.

## US-5.5.9.12 Run AEC on Audio Thread
**As an** engine developer (P-26), **I want** AEC on the audio thread, **so that** latency is
minimal and processing is real-time.
