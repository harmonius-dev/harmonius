# Communication Framework User Stories

## Channel System

| ID       | Persona                     |
|----------|-----------------------------|
| US-8.9.1 | server administrator (P-22) |
| US-8.9.2 | player (P-23)               |
| US-8.9.3 | player (P-23)               |
| US-8.9.4 | game developer (P-15)       |

1. **US-8.9.1** — **As a** server administrator (P-22), **I want** a global chat channel to be
   automatically created when the server starts, **so that** all connected players can communicate
   in a server-wide channel without manual setup.
2. **US-8.9.2** — **As a** player (P-23), **I want** a team channel to be automatically created and
   joined when I form or join a party, **so that** my team has a private communication channel
   without manual channel management.
3. **US-8.9.3** — **As a** player (P-23), **I want** my channel memberships to persist when I move
   between zones, **so that** I stay connected to my party and guild channels without rejoining
   after every zone transition.
4. **US-8.9.4** — **As a** game developer (P-15), **I want** the channel API to work identically in
   both the game runtime and the editor, **so that** I write communication code once and it works in
   both contexts without conditional logic.

## Text Chat

| ID       | Persona           |
|----------|-------------------|
| US-8.9.5 | player (P-23)     |
| US-8.9.6 | player (P-23)     |
| US-8.9.7 | player (P-23)     |
| US-8.9.8 | QA tester (P-19)  |

1. **US-8.9.5** — **As a** player (P-23), **I want** to type a message and send it to my party chat
   channel, **so that** my teammates see the message instantly and can respond.
2. **US-8.9.6** — **As a** player (P-23), **I want** to @-mention another player in chat,
   **so that** they receive a notification even if they are not actively watching the channel.
3. **US-8.9.7** — **As a** player (P-23), **I want** to search chat history by keyword, **so that**
   I can find information shared earlier in a conversation without scrolling through hundreds of
   messages.
4. **US-8.9.8** — **As a** QA tester (P-19), **I want** to send text messages from the game client
   during a test session, **so that** I can report issues to designers in real time without leaving
   the game.

## Voice Chat

| ID        | Persona           |
|-----------|-------------------|
| US-8.9.9  | player (P-23)     |
| US-8.9.10 | player (P-23)     |
| US-8.9.11 | player (P-23)     |
| US-8.9.12 | player (P-23)     |

1. **US-8.9.9** — **As a** player (P-23), **I want** to join my party's voice channel and talk to
   teammates, **so that** we can coordinate gameplay actions with voice communication.
2. **US-8.9.10** — **As a** player (P-23), **I want** to hear nearby players' voices attenuated by
   distance in proximity chat, **so that** social interactions in the game world feel spatially
   natural.
3. **US-8.9.11** — **As a** player (P-23), **I want** to mute an individual speaker in a voice
   channel without muting others, **so that** I can silence a disruptive player while still hearing
   the rest of the team.
4. **US-8.9.12** — **As a** player (P-23), **I want** voice chat latency to be below 150 ms,
   **so that** conversations during cooperative gameplay feel natural and responsive.

## Direct Messaging

| ID        | Persona           |
|-----------|-------------------|
| US-8.9.13 | player (P-23)     |
| US-8.9.14 | player (P-23)     |
| US-8.9.15 | player (P-23)     |

1. **US-8.9.13** — **As a** player (P-23), **I want** to send a direct message to a friend,
   **so that** I can have a private conversation without other players seeing it.
2. **US-8.9.14** — **As a** player (P-23), **I want** to receive messages that were sent while I was
   offline when I log back in, **so that** I do not miss communications from friends.
3. **US-8.9.15** — **As a** player (P-23), **I want** to start a 1:1 voice call from an existing
   text DM conversation, **so that** I can switch to voice when typing becomes inconvenient.

## Chat Moderation

| ID        | Persona                     |
|-----------|-----------------------------|
| US-8.9.16 | server administrator (P-22) |
| US-8.9.17 | player (P-23)               |
| US-8.9.18 | player (P-23)               |
| US-8.9.19 | server administrator (P-22) |

1. **US-8.9.16** — **As a** server administrator (P-22), **I want** a profanity filter to block or
   replace offensive words in chat with locale-aware word lists, **so that** the chat environment
   remains appropriate for all players.
2. **US-8.9.17** — **As a** player (P-23), **I want** to block another player so that all their
   messages and voice are hidden from me, **so that** I am not exposed to harassment or toxic
   behavior.
3. **US-8.9.18** — **As a** player (P-23), **I want** to report a player with attached message
   evidence, **so that** server admins can review the report and take appropriate action.
4. **US-8.9.19** — **As a** server administrator (P-22), **I want** to see reported players with
   timestamped message evidence in a moderation dashboard, **so that** I can make informed decisions
   about sanctions.

## VR Chat

| ID        | Persona       |
|-----------|---------------|
| US-8.9.20 | player (P-23) |
| US-8.9.21 | player (P-23) |
| US-8.9.22 | player (P-23) |

1. **US-8.9.20** — **As a** player (P-23), **I want** to hear other players' voices coming from the
   direction of their avatars in VR, **so that** voice chat feels spatially immersive.
2. **US-8.9.21** — **As a** player (P-23), **I want** to use a virtual keyboard in VR to type chat
   messages, **so that** I can communicate via text even while wearing a headset.
3. **US-8.9.22** — **As a** player (P-23), **I want** floating chat bubbles above avatars who are
   speaking or who sent recent text messages, **so that** I can identify who is communicating at a
   glance.

## Audio Streaming

| ID        | Persona                     |
|-----------|-----------------------------|
| US-8.9.23 | player (P-23)               |
| US-8.9.24 | server administrator (P-22) |

1. **US-8.9.23** — **As a** player (P-23), **I want** voice chat to remain intelligible even with 5%
   packet loss, **so that** network issues do not prevent communication during gameplay.
2. **US-8.9.24** — **As a** server administrator (P-22), **I want** to configure the Opus voice
   bitrate (6-64 kbps) per channel, **so that** I can balance voice quality against bandwidth usage
   for the server's network capacity.

## Editor Bridge

| ID        | Persona              |
|-----------|----------------------|
| US-8.9.25 | game developer (P-15) |
| US-8.9.26 | QA tester (P-19)     |
| US-8.9.27 | game developer (P-15) |

1. **US-8.9.25** — **As a** game developer (P-15), **I want** to send and receive chat messages on
   the same channel as QA testers playing the game, **so that** I can coordinate with them during
   live testing without switching applications.
2. **US-8.9.26** — **As a** QA tester (P-19), **I want** to send a message in the editor-game shared
   channel that includes an asset reference, **so that** developers can click the reference and jump
   directly to the problematic asset.
3. **US-8.9.27** — **As a** game developer (P-15), **I want** to see both editor users and game
   users in the same presence list, **so that** I know who is online and available regardless of
   their context.

## Testing

| ID        | Persona                 |
|-----------|-------------------------|
| US-8.9.28 | engine developer (P-26) |
| US-8.9.29 | engine developer (P-26) |

1. **US-8.9.28** — **As an** engine developer (P-26), **I want** to capture DM traffic at the server
   and verify no plaintext content is extractable, **so that** end-to-end encryption is confirmed.
2. **US-8.9.29** — **As an** engine developer (P-26), **I want** to measure message delivery latency
   within a single context vs. across the editor-game bridge, **so that** the bridge adds less than
   10 ms of overhead.
