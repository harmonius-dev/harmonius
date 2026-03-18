# 8.9 — Communication Framework

## Channel System

| ID      | Feature                      | Requirements |
|---------|------------------------------|--------------|
| F-8.9.1 | Communication Channel System | R-8.9.1      |

1. **F-8.9.1** — A unified channel abstraction shared by both the game runtime and the editor.
   Channels are polymorphic containers that carry text, voice, or both over the same transport. Each
   channel has a unique ID, a type (global, team, party, whisper, custom, editor), membership list,
   and permission set. The channel system integrates with the QUIC transport (F-8.1.3, F-8.1.4) for
   delivery and with the editor collaboration service (F-15.12.10) for editor-side communication.
   Channels persist across zone transitions (F-8.7.2) and server migrations (F-8.7.4). Acceptance:
   Create a channel with a specified type and membership; Join and leave channels at runtime;
   Channel membership survives zone transitions; Same channel API works in game runtime and editor;
   Channels support text, voice, or both simultaneously
   - **Deps:** F-8.1.3, F-8.1.4, F-8.5.3, F-15.12.10

## Text Chat

| ID      | Feature   | Requirements |
|---------|-----------|--------------|
| F-8.9.2 | Text Chat | R-8.9.2      |

1. **F-8.9.2** — Real-time text messaging over communication channels with support for global, team,
   party, whisper, and custom channel types. Messages are delivered via the reliable ordered channel
   (F-8.1.3) with server-side persistence for chat history and search. Messages support @-mentions,
   inline asset references, and emoji. Text input in VR uses the virtual keyboard (F-8.9.6). Chat
   history is queryable with full-text search. Acceptance: Send and receive text messages on any
   channel type; Messages persist server-side and are searchable; @-mentions notify the mentioned
   user; Messages display sender name, timestamp, and channel; Chat history loads on channel join
   - **Deps:** F-8.9.1, F-8.1.3

## Voice Chat

| ID      | Feature                | Requirements |
|---------|------------------------|--------------|
| F-8.9.3 | Voice Chat Integration | R-8.9.3      |

1. **F-8.9.3** — Voice channels sharing the same channel system as text. Voice data is captured,
   encoded with Opus (F-5.5.1), transmitted over unreliable unordered channels (F-8.1.4), and
   decoded into the audio engine mixer bus hierarchy (F-5.1.3). Proximity voice channels spatialize
   audio using the shared BVH (F-1.9.8). Party and team channels mix as non-spatialized bus sends.
   Voice activity detection (F-5.5.3) gates transmission. Acoustic echo cancellation (F-5.5.9)
   prevents feedback. Acceptance: Join a voice channel and transmit/receive audio; Proximity
   channels attenuate by distance; Party/team channels are non-spatialized; VAD gates transmission
   when not speaking; Per-speaker volume and mute controls
   - **Deps:** F-8.9.1, F-8.1.4, F-5.5.1, F-5.5.3, F-5.5.9, F-5.1.3

## Direct Messaging

| ID      | Feature          | Requirements |
|---------|------------------|--------------|
| F-8.9.4 | Direct Messaging | R-8.9.4      |

1. **F-8.9.4** — Private 1:1 text and voice conversations implemented as special-case channels with
   exactly two members. Direct messages are end-to-end encrypted using a key derived from the
   participants' session keys. DM history persists server-side (encrypted at rest) and is accessible
   across sessions. DM channels support the same text features as group channels (@-mentions, asset
   references) and can escalate to 1:1 voice calls. Offline messages are queued and delivered on
   next login. Acceptance: Start a DM conversation with any online player; Messages are end-to-end
   encrypted; Offline messages are delivered on next login; DM history persists across sessions;
   Escalate a text DM to a voice call
   - **Deps:** F-8.9.1, F-8.9.2, F-8.1.5

## Chat Moderation

| ID      | Feature         | Requirements |
|---------|-----------------|--------------|
| F-8.9.5 | Chat Moderation | R-8.9.5      |

1. **F-8.9.5** — Server-side moderation tools for text and voice channels. Includes a configurable
   profanity filter with locale-aware word lists, per-user mute (text and voice independently),
   block (prevents all communication from blocked user), report (flags a user with evidence for
   admin review), and rate limiting (messages per second per user per channel). Moderation actions
   are logged with timestamps for audit. Server admins (P-22) access a moderation dashboard for
   reviewing reports and applying sanctions. Acceptance: Profanity filter blocks configured words;
   Muted users cannot send text or voice in channel; Blocked users are invisible to the blocker;
   Reports include message evidence and timestamps; Rate limiting throttles excessive messaging
   - **Deps:** F-8.9.1, F-8.9.2, F-8.9.3, F-8.8.5

## VR Chat Integration

| ID      | Feature             | Requirements |
|---------|---------------------|--------------|
| F-8.9.6 | VR Chat Integration | R-8.9.6      |

1. **F-8.9.6** — Communication framework integration for VR contexts. Spatial voice positions audio
   sources at avatar head positions in 3D space using HRTF binaural rendering (F-5.2.3). Text input
   uses a virtual keyboard rendered in VR worldspace with controller or hand-tracking input. VR
   users can see floating chat bubbles above avatars for recent text messages. Spatial voice
   attenuation follows the same distance curves as the audio engine (F-5.2.2). Acceptance: Voice
   sources positioned at avatar heads in VR; HRTF renders directional audio for voice; Virtual
   keyboard accepts text input in VR; Chat bubbles appear above speaking avatars; Voice attenuates
   with distance in VR
   - **Deps:** F-8.9.2, F-8.9.3, F-5.2.2, F-5.2.3

## Networked Audio Streaming

| ID      | Feature                   | Requirements |
|---------|---------------------------|--------------|
| F-8.9.7 | Networked Audio Streaming | R-8.9.7      |

1. **F-8.9.7** — Low-latency audio transport for voice and game audio over the network. Opus codec
   at configurable bitrates (6-64 kbps) with an adaptive jitter buffer (F-5.5.2) that adjusts depth
   based on network conditions. Packet loss concealment via Opus built-in decoder interpolation.
   Bandwidth allocation integrates with the congestion controller (F-8.1.7) to avoid starving
   gameplay traffic. Audio packets use unreliable unordered delivery (F-8.1.4) for minimal latency.
   Supports mixing multiple incoming voice streams on the client. Acceptance: Voice packets
   delivered under 50 ms one-way latency; Jitter buffer adapts to network conditions; PLC masks gaps
   from lost packets; Multiple voice streams mix without clipping; Audio bandwidth respects
   congestion controller
   - **Deps:** F-8.1.4, F-8.1.7, F-5.5.1, F-5.5.2, F-5.1.3

## Editor Communication Bridge

| ID      | Feature                     | Requirements |
|---------|-----------------------------|--------------|
| F-8.9.8 | Editor Communication Bridge | R-8.9.8      |

1. **F-8.9.8** — An adapter that connects the communication framework to the editor collaboration
   system (F-15.12.10). Editor users share communication channels with game runtime users when
   desired (e.g., QA testing a live build while chatting with designers in the editor). The bridge
   supports follow/track for other users and AI agents (F-15.12.12), presence indicators showing who
   is online and what they are editing, and notification routing for @-mentions across editor and
   game contexts. Acceptance: Editor users send/receive messages on game channels; Follow an AI
   agent and see its edits in real time; Presence shows editor users alongside game users;
   @-mentions route notifications across contexts; Editor chat integrates with collaboration
   presence
   - **Deps:** F-8.9.1, F-15.12.10, F-15.12.12
