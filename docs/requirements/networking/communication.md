# R-8.9 -- Communication Framework Requirements

## Channel System

1. **R-8.9.1** — The engine **SHALL** provide a unified channel abstraction shared by game runtime
   and editor that supports text, voice, or both simultaneously per channel. Each channel **SHALL**
   have a unique ID, type (global, team, party, whisper, custom, editor), membership list, and
   permission set. Channels **SHALL** persist across zone transitions and server migrations.
   - **Rationale:** A single channel abstraction eliminates duplicate infrastructure across game and
     editor, reducing maintenance burden and ensuring consistent behavior.
   - **Verification:** Integration test: create channels of each type and verify membership
     operations. Trigger a zone transition and verify membership survives. Verify the same API works
     from both game runtime and editor contexts.
2. **R-8.9.2** — The engine **SHALL** support at least 256 concurrent channels per server process
   and at least 64 simultaneous channel memberships per client.
   - **Rationale:** MMO social structures (guilds, parties, raids, custom channels, whispers)
     require many concurrent channels.
   - **Verification:** Load test: create 256 channels with 100 members each. Verify no degradation.
     Verify a single client can join 64 channels and receive messages on all.

## Text Chat

1. **R-8.9.3** — The engine **SHALL** deliver text messages over communication channels via the
   reliable ordered transport channel with server-side persistence and full-text search. Messages
   **SHALL** support @-mentions, inline asset references, and emoji. Chat history **SHALL** load on
   channel join.
   - **Rationale:** Text chat is the primary communication method for players who cannot or prefer
     not to use voice, and provides a persistent conversation record.
   - **Verification:** Integration test: send a text message on each channel type and verify
     delivery to all members. Verify @-mentions trigger notifications. Verify chat history loads on
     join. Search history by keyword and verify results.
2. **R-8.9.4** — The engine **SHALL** deliver text messages to all channel members within 100 ms
   under normal network conditions (< 50 ms RTT, < 1% packet loss).
   - **Rationale:** Text chat must feel instantaneous; delays above 100 ms are perceptible.
   - **Verification:** Benchmark: send 1,000 messages on a channel with 50 members. Assert p99
     latency below 100 ms on a link with 20 ms RTT.
3. **R-8.9.5** — The engine **SHALL** persist chat history server-side with a configurable retention
   period (default 30 days), queryable with full-text search returning results within 200 ms for
   indices up to 10 million messages.
   - **Rationale:** Persistent history enables review and moderation evidence.
   - **Verification:** Insert 10 million messages. Execute a full-text search and verify results
     within 200 ms. Verify messages older than retention period are pruned.

## Voice Chat

1. **R-8.9.6** — The engine **SHALL** integrate voice channels into the unified channel system,
   transmitting Opus-encoded audio over unreliable unordered channels, decoding into the mixer bus
   hierarchy. Proximity channels **SHALL** spatialize audio using the shared BVH. Party and team
   channels **SHALL** mix as non-spatialized bus sends. VAD **SHALL** gate transmission. AEC
   **SHALL** prevent feedback.
   - **Rationale:** Voice chat shares the same abstraction as text, enabling unified membership,
     permissions, and moderation across both modalities.
   - **Verification:** Integration test: join a voice channel and transmit audio. Verify decoded
     audio in the mixer. Verify proximity attenuation. Verify VAD gates silence. Verify per-speaker
     volume and mute controls.
2. **R-8.9.7** — The engine **SHALL** deliver voice audio with mouth-to-ear latency not exceeding
   150 ms under normal conditions (< 50 ms RTT, < 1% loss), inclusive of capture, encoding, transit,
   jitter buffer, decoding, and mixer output.
   - **Rationale:** Voice latency above 150 ms causes conversational overlap and frustration.
   - **Verification:** Benchmark: measure mouth-to-ear latency on a loopback with 50 ms RTT. Assert
     total below 150 ms.

## Direct Messaging

1. **R-8.9.8** — The engine **SHALL** support private 1:1 conversations as special-case channels
   with exactly two members. Messages **SHALL** be end-to-end encrypted using keys derived from
   session keys via X25519 Diffie-Hellman. DM history **SHALL** persist server-side encrypted at
   rest with AES-256-GCM. Offline messages **SHALL** be queued and delivered on next login.
   - **Rationale:** Players need private, secure communication that works across sessions and login
     states.
   - **Verification:** Integration test: start a DM and verify delivery. Verify end-to-end
     encryption by inspecting wire payloads. Send to an offline user and verify delivery on login.
     Verify DM history loads on reconnection.

## Chat Moderation

1. **R-8.9.9** — The engine **SHALL** provide server-side moderation for text and voice channels:
   configurable profanity filter with locale-aware word lists, per-user mute (text and voice
   independently), block (hides all communication from blocked user), report (flags user with
   evidence), and rate limiting (messages per second per user per channel). All moderation actions
   **SHALL** be logged with timestamps.
   - **Rationale:** Moderation tools protect the community from harassment and spam while providing
     admins with auditable evidence for sanctions.
   - **Verification:** Integration test: configure a filter and verify word replacement. Mute a user
     and verify they cannot send. Block a user and verify hidden. Submit a report and verify it
     appears in the moderation queue. Verify rate limiting.
2. **R-8.9.10** — The profanity filter **SHALL** evaluate a message against the full word list in
   under 1 ms, supporting up to 50,000 entries across all configured locales.
   - **Rationale:** The filter runs on every message and must not add perceptible latency.
   - **Verification:** Benchmark: load 50,000 entries and filter 10,000 messages. Assert p99 filter
     time below 1 ms.

## VR Chat

1. **R-8.9.11** — The engine **SHALL** position voice audio sources at avatar head positions in 3D
   space using HRTF binaural rendering for VR contexts. Text input **SHALL** use a virtual keyboard
   in VR worldspace. Chat bubbles **SHALL** appear above speaking avatars. Spatial voice positioning
   **SHALL** update at the VR render frame rate (minimum 72 Hz) within 1 frame of avatar head
   position.
   - **Rationale:** VR requires spatial voice and alternative text input because traditional
     keyboard input is unavailable and non-spatialized audio breaks immersion.
   - **Verification:** Integration test: join a voice channel in VR and verify audio at avatar
     heads. Verify HRTF directional rendering. Verify virtual keyboard sends messages. Verify chat
     bubbles. Benchmark spatial update latency within 1 frame (< 14 ms at 72 Hz).

## Audio Streaming

1. **R-8.9.12** — The engine **SHALL** stream audio using Opus at configurable bitrates (6-64 kbps)
   with an adaptive jitter buffer and Opus built-in PLC. Bandwidth **SHALL** integrate with the
   congestion controller. The engine **SHALL** mix at least 32 simultaneous incoming voice streams
   without artifacts.
   - **Rationale:** Voice requires specialized low-latency transport integrated with congestion
     control to avoid starving gameplay traffic.
   - **Verification:** Integration test: stream at 24 kbps and verify under 50 ms one-way latency.
     Simulate 5% loss and verify PLC. Feed 32 streams and verify no underruns over 60 seconds.
     Verify throttling when congestion controller reduces the budget.
2. **R-8.9.13** — Voice audio **SHOULD** consume no more than 10% of the per-connection bandwidth
   budget. When constrained, the engine **SHALL** reduce bitrate before dropping voice entirely.
   - **Rationale:** Voice must coexist with gameplay traffic and degrade gracefully.
   - **Verification:** Limit connection to 100 kbps. Verify voice drops to 6 kbps before suspension.
     Verify gameplay replication continues uninterrupted.

## Editor Bridge

1. **R-8.9.14** — The engine **SHALL** provide an adapter connecting the communication framework to
   the editor collaboration system (F-15.12.10). Editor users **SHALL** send and receive messages on
   game channels. The bridge **SHALL** support follow/track for other users and AI agents
   (F-15.12.12). Presence **SHALL** show editor users alongside game users. @-mentions **SHALL**
   route across contexts.
   - **Rationale:** Unified communication between editor and game enables QA testers to chat with
     developers while testing live builds.
   - **Verification:** Integration test: connect editor and game clients to the same channel. Verify
     cross-context message delivery. Follow an AI agent and verify real-time visibility. Verify
     presence and @-mention routing.
2. **R-8.9.15** — Messages bridged between editor and game contexts **SHALL** add no more than 10 ms
   of additional latency beyond base channel delivery.
   - **Rationale:** Bridging overhead must be negligible to maintain real-time feel.
   - **Verification:** Benchmark: measure delivery within a single context vs. across the bridge.
     Assert the bridge adds less than 10 ms.

## Non-Functional

1. **R-8.9.16** — The communication framework **SHALL** consume no more than 16 MiB of memory per
   client for channel state, message buffers, and voice codec state across all active channels.
   - **Rationale:** Communication must fit within the per-subsystem memory budget.
   - **Verification:** Profile memory with 64 active channels (32 text, 32 voice). Assert total
     below 16 MiB.
2. **R-8.9.17** — The server **SHALL** process at least 10,000 text messages per second across all
   channels without delivery degradation.
   - **Rationale:** MMO servers must handle high chat volumes during peak activity.
   - **Verification:** Load test: send 10,000 messages per second across 100 channels with 50
     members each. Assert all delivered within the 100 ms latency target.
3. **R-8.9.18** — The server **SHALL** relay voice streams for at least 200 concurrent voice channel
   members without exceeding the audio processing budget.
   - **Rationale:** Large raids and events may have many simultaneous voice participants.
   - **Verification:** Load test: create 10 voice channels with 20 active speakers each. Assert
     relay CPU below 5% of a single core per channel.
