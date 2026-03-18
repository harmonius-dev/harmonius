# User Stories -- 8.9 Communication Framework

## F-8.9.1 Communication Channel System

| ID         | Persona               | Features | Requirements |
|------------|-----------------------|----------|--------------|
| US-8.9.1.1 | server admin (P-22)   | —        | —            |
| US-8.9.1.2 | player (P-23)         | —        | —            |
| US-8.9.1.3 | player (P-23)         | —        | —            |
| US-8.9.1.4 | game developer (P-15) | —        | —            |
| US-8.9.1.5 | designer (P-5)        | —        | —            |
| US-8.9.1.6 | engine tester (P-27)  | —        | —            |

1. **US-8.9.1.1** — As a server admin (P-22), I want a global chat channel to be automatically
   created when the server starts, so that all connected players can communicate in a server-wide
   channel without manual setup.
   - **Acceptance:** —
2. **US-8.9.1.2** — As a player (P-23), I want a team channel to be automatically created and joined
   when I form or join a party, so that my team has a private communication channel without manual
   channel management.
   - **Acceptance:** —
3. **US-8.9.1.3** — As a player (P-23), I want my channel memberships to persist when I move between
   zones, so that I stay connected to my party and guild channels without rejoining after every zone
   transition.
   - **Acceptance:** —
4. **US-8.9.1.4** — As a game developer (P-15), I want the channel API to work identically in both
   the game runtime and the editor, so that I write communication code once and it works in both
   contexts without conditional logic.
   - **Acceptance:** —
5. **US-8.9.1.5** — As a designer (P-5), I want to create custom channels with specific membership
   rules (e.g., all players in a raid zone), so that gameplay-specific communication groups are
   configurable without code changes.
   - **Acceptance:** —
6. **US-8.9.1.6** — As an engine tester (P-27), I want to trigger a server migration while a player
   is in multiple channels, so that I can verify all channel memberships survive the migration
   without message loss.
   - **Acceptance:** —

## F-8.9.2 Text Chat

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-8.9.2.1 | player (P-23)        | —        | —            |
| US-8.9.2.2 | player (P-23)        | —        | —            |
| US-8.9.2.3 | player (P-23)        | —        | —            |
| US-8.9.2.4 | player (P-23)        | —        | —            |
| US-8.9.2.5 | designer (P-5)       | —        | —            |
| US-8.9.2.6 | engine tester (P-27) | —        | —            |
| US-8.9.2.7 | QA tester (P-19)     | —        | —            |

1. **US-8.9.2.1** — As a player (P-23), I want to type a message and send it to my party chat
   channel, so that my teammates see the message instantly and can respond.
   - **Acceptance:** —
2. **US-8.9.2.2** — As a player (P-23), I want to @-mention another player in chat, so that they
   receive a notification even if they are not actively watching the channel.
   - **Acceptance:** —
3. **US-8.9.2.3** — As a player (P-23), I want to search chat history by keyword, so that I can find
   information shared earlier in a conversation without scrolling through hundreds of messages.
   - **Acceptance:** —
4. **US-8.9.2.4** — As a player (P-23), I want recent chat history to load when I join a channel, so
   that I have context for the ongoing conversation.
   - **Acceptance:** —
5. **US-8.9.2.5** — As a designer (P-5), I want to paste an asset reference in editor chat that
   other users can click to open, so that I can share specific assets with collaborators without
   navigating file paths.
   - **Acceptance:** —
6. **US-8.9.2.6** — As an engine tester (P-27), I want to measure text message delivery latency
   across 50 channel members, so that I can verify p99 delivery is below 100 ms under normal network
   conditions.
   - **Acceptance:** —
7. **US-8.9.2.7** — As a QA tester (P-19), I want to send text messages from the game client during
   a test session, so that I can report issues to designers in real time without leaving the game.
   - **Acceptance:** —

## F-8.9.3 Voice Chat Integration

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-8.9.3.1 | player (P-23)        | —        | —            |
| US-8.9.3.2 | player (P-23)        | —        | —            |
| US-8.9.3.3 | player (P-23)        | —        | —            |
| US-8.9.3.4 | player (P-23)        | —        | —            |
| US-8.9.3.5 | engine tester (P-27) | —        | —            |
| US-8.9.3.6 | player (P-23)        | —        | —            |
| US-8.9.3.7 | player (P-23)        | —        | —            |

1. **US-8.9.3.1** — As a player (P-23), I want to join my party's voice channel and talk to
   teammates, so that we can coordinate gameplay actions with voice communication.
   - **Acceptance:** —
2. **US-8.9.3.2** — As a player (P-23), I want to hear nearby players' voices attenuated by distance
   in proximity chat, so that social interactions in the game world feel spatially natural.
   - **Acceptance:** —
3. **US-8.9.3.3** — As a player (P-23), I want to mute an individual speaker in a voice channel
   without muting others, so that I can silence a disruptive player while still hearing the rest of
   the team.
   - **Acceptance:** —
4. **US-8.9.3.4** — As a player (P-23), I want to adjust the volume of individual speakers in a
   voice channel, so that I can balance quiet and loud speakers to my preference.
   - **Acceptance:** —
5. **US-8.9.3.5** — As an engine tester (P-27), I want to verify that no voice packets are
   transmitted when the local player is not speaking, so that bandwidth is conserved and background
   noise is not sent to other players.
   - **Acceptance:** —
6. **US-8.9.3.6** — As a player (P-23), I want to use voice chat in a 20-player raid group, so that
   the raid leader can call out mechanics and the team can respond quickly.
   - **Acceptance:** —
7. **US-8.9.3.7** — As a player (P-23), I want voice chat latency to be below 150 ms, so that
   conversations during cooperative gameplay feel natural and responsive.
   - **Acceptance:** —

## F-8.9.4 Direct Messaging

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-8.9.4.1 | player (P-23)        | —        | —            |
| US-8.9.4.2 | player (P-23)        | —        | —            |
| US-8.9.4.3 | player (P-23)        | —        | —            |
| US-8.9.4.4 | player (P-23)        | —        | —            |
| US-8.9.4.5 | engine tester (P-27) | —        | —            |
| US-8.9.4.6 | QA tester (P-19)     | —        | —            |
| US-8.9.4.7 | engine tester (P-27) | —        | —            |

1. **US-8.9.4.1** — As a player (P-23), I want to send a direct message to a friend, so that I can
   have a private conversation without other players seeing it.
   - **Acceptance:** —
2. **US-8.9.4.2** — As a player (P-23), I want to receive messages that were sent while I was
   offline when I log back in, so that I do not miss communications from friends.
   - **Acceptance:** —
3. **US-8.9.4.3** — As a player (P-23), I want my direct message history to persist across sessions,
   so that I can review past conversations with a friend after logging out and back in.
   - **Acceptance:** —
4. **US-8.9.4.4** — As a player (P-23), I want to start a 1:1 voice call from an existing text DM
   conversation, so that I can switch to voice when typing becomes inconvenient.
   - **Acceptance:** —
5. **US-8.9.4.5** — As an engine tester (P-27), I want to capture DM traffic at the server and
   verify no plaintext content is extractable, so that end-to-end encryption is confirmed.
   - **Acceptance:** —
6. **US-8.9.4.6** — As a QA tester (P-19), I want to send a direct message to a designer who is in
   the editor, so that I can privately report a sensitive issue without broadcasting it to the team
   channel.
   - **Acceptance:** —
7. **US-8.9.4.7** — As an engine tester (P-27), I want to send a DM to an offline user, log them in,
   and verify the message is delivered, so that offline queuing works correctly.
   - **Acceptance:** —

## F-8.9.5 Chat Moderation

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-8.9.5.1 | server admin (P-22)  | —        | —            |
| US-8.9.5.2 | server admin (P-22)  | —        | —            |
| US-8.9.5.3 | player (P-23)        | —        | —            |
| US-8.9.5.4 | player (P-23)        | —        | —            |
| US-8.9.5.5 | server admin (P-22)  | —        | —            |
| US-8.9.5.6 | server admin (P-22)  | —        | —            |
| US-8.9.5.7 | engine tester (P-27) | —        | —            |
| US-8.9.5.8 | player (P-23)        | —        | —            |

1. **US-8.9.5.1** — As a server admin (P-22), I want a profanity filter to block or replace
   offensive words in global chat, so that the chat environment remains appropriate for all players.
   - **Acceptance:** —
2. **US-8.9.5.2** — As a server admin (P-22), I want to mute a player in a specific channel (text,
   voice, or both), so that they cannot continue disrupting other players while I investigate.
   - **Acceptance:** —
3. **US-8.9.5.3** — As a player (P-23), I want to block another player so that all their messages
   and voice are hidden from me, so that I am not exposed to harassment or toxic behavior.
   - **Acceptance:** —
4. **US-8.9.5.4** — As a player (P-23), I want to report a player with attached message evidence, so
   that server admins can review the report and take appropriate action.
   - **Acceptance:** —
5. **US-8.9.5.5** — As a server admin (P-22), I want to see reported players with timestamped
   message evidence in a moderation dashboard, so that I can make informed decisions about
   sanctions.
   - **Acceptance:** —
6. **US-8.9.5.6** — As a server admin (P-22), I want to configure separate profanity filter word
   lists for each supported locale, so that the filter is effective across languages.
   - **Acceptance:** —
7. **US-8.9.5.7** — As an engine tester (P-27), I want to send messages exceeding the configured
   rate limit and verify throttling activates, so that spam prevention works as specified.
   - **Acceptance:** —
8. **US-8.9.5.8** — As a player (P-23), I want blocked players to be completely invisible in chat,
   so that I am never exposed to content from users I have blocked.
   - **Acceptance:** —

## F-8.9.6 VR Chat Integration

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-8.9.6.1 | player (P-23)        | —        | —            |
| US-8.9.6.2 | player (P-23)        | —        | —            |
| US-8.9.6.3 | player (P-23)        | —        | —            |
| US-8.9.6.4 | player (P-23)        | —        | —            |
| US-8.9.6.5 | engine tester (P-27) | —        | —            |
| US-8.9.6.6 | engine tester (P-27) | —        | —            |

1. **US-8.9.6.1** — As a player (P-23), I want to hear other players' voices coming from the
   direction of their avatars in VR, so that voice chat feels spatially immersive.
   - **Acceptance:** —
2. **US-8.9.6.2** — As a player (P-23), I want to use a virtual keyboard in VR to type chat
   messages, so that I can communicate via text even while wearing a headset.
   - **Acceptance:** —
3. **US-8.9.6.3** — As a player (P-23), I want floating chat bubbles above avatars who are speaking
   or who sent recent text messages, so that I can identify who is communicating at a glance.
   - **Acceptance:** —
4. **US-8.9.6.4** — As a player (P-23), I want voice volume to decrease as I move farther from a
   speaking avatar in VR, so that spatial audio realism is maintained.
   - **Acceptance:** —
5. **US-8.9.6.5** — As an engine tester (P-27), I want to verify that audio source positions update
   within 1 frame of avatar head position changes, so that VR spatial audio remains synchronized
   with visual cues.
   - **Acceptance:** —
6. **US-8.9.6.6** — As an engine tester (P-27), I want to verify the VR virtual keyboard sends typed
   text as chat messages, so that text input is functional in VR contexts.
   - **Acceptance:** —

## F-8.9.7 Networked Audio Streaming

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-8.9.7.1 | player (P-23)        | —        | —            |
| US-8.9.7.2 | server admin (P-22)  | —        | —            |
| US-8.9.7.3 | engine tester (P-27) | —        | —            |
| US-8.9.7.4 | player (P-23)        | —        | —            |
| US-8.9.7.5 | engine tester (P-27) | —        | —            |
| US-8.9.7.6 | engine tester (P-27) | —        | —            |

1. **US-8.9.7.1** — As a player (P-23), I want voice chat to remain intelligible even with 5% packet
   loss, so that network issues do not prevent communication during gameplay.
   - **Acceptance:** —
2. **US-8.9.7.2** — As a server admin (P-22), I want to configure the Opus voice bitrate (6-64 kbps)
   per channel, so that I can balance voice quality against bandwidth usage for the server's network
   capacity.
   - **Acceptance:** —
3. **US-8.9.7.3** — As an engine tester (P-27), I want to simulate varying network jitter and verify
   the jitter buffer adjusts depth automatically, so that voice quality is maintained under changing
   conditions.
   - **Acceptance:** —
4. **US-8.9.7.4** — As a player (P-23), I want to hear multiple speakers in a voice channel at the
   same time without audio clipping or distortion, so that group conversations are clear.
   - **Acceptance:** —
5. **US-8.9.7.5** — As an engine tester (P-27), I want to verify that voice audio bandwidth is
   throttled when the congestion controller reduces the send budget, so that gameplay replication is
   never starved by voice traffic.
   - **Acceptance:** —
6. **US-8.9.7.6** — As an engine tester (P-27), I want to feed 32 concurrent voice streams into the
   mixer and verify no audio underruns over 60 seconds, so that large group voice scales correctly.
   - **Acceptance:** —

## F-8.9.8 Editor Communication Bridge

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-8.9.8.1 | designer (P-5)       | —        | —            |
| US-8.9.8.2 | designer (P-5)       | —        | —            |
| US-8.9.8.3 | designer (P-5)       | —        | —            |
| US-8.9.8.4 | designer (P-5)       | —        | —            |
| US-8.9.8.5 | QA tester (P-19)     | —        | —            |
| US-8.9.8.6 | engine tester (P-27) | —        | —            |
| US-8.9.8.7 | designer (P-5)       | —        | —            |
| US-8.9.8.8 | engine tester (P-27) | —        | —            |

1. **US-8.9.8.1** — As a designer (P-5), I want to send and receive chat messages on the same
   channel as QA testers playing the game, so that I can coordinate with them during live testing
   without switching applications.
   - **Acceptance:** —
2. **US-8.9.8.2** — As a designer (P-5), I want to follow an AI agent in the editor and see its
   edits appear in real time, so that I can monitor automated content generation as it happens.
   - **Acceptance:** —
3. **US-8.9.8.3** — As a designer (P-5), I want to see both editor users and game users in the same
   presence list, so that I know who is online and available regardless of their context.
   - **Acceptance:** —
4. **US-8.9.8.4** — As a designer (P-5), I want @-mentions from game players to appear as
   notifications in the editor, so that I am alerted when someone in the game needs my attention.
   - **Acceptance:** —
5. **US-8.9.8.5** — As a QA tester (P-19), I want to send a message in the editor-game shared
   channel that includes an asset reference, so that designers can click the reference and jump
   directly to the problematic asset.
   - **Acceptance:** —
6. **US-8.9.8.6** — As an engine tester (P-27), I want to send a message from an editor client and
   verify it arrives on a game client in the same channel, so that editor-game bridging delivers
   messages correctly.
   - **Acceptance:** —
7. **US-8.9.8.7** — As a designer (P-5), I want to track an AI agent's activity via the
   collaboration panel in the editor, so that I can see what the agent is editing and follow its
   progress without searching manually.
   - **Acceptance:** —
8. **US-8.9.8.8** — As an engine tester (P-27), I want to measure message delivery latency within a
   single context vs. across the editor-game bridge, so that I can verify the bridge adds less than
   10 ms of overhead.
   - **Acceptance:** —
