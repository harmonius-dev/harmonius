# User Stories -- 5.5 Voice & Speech

## US-5.5.1 Talk to Nearby Players in Proximity Chat
**As a** player, **I want** low-latency voice chat with Opus encoding that routes nearby
players as spatialized 3D sources and party members as non-spatialized bus sends, **so that**
I hear proximity chat from the correct direction and group chat clearly regardless of
position.

## US-5.5.2 Hear Clear Voice Despite Network Jitter
**As a** player, **I want** an adaptive jitter buffer with packet-loss concealment to mask
gaps and maintain intelligible voice chat under variable network conditions, **so that**
voice quality remains acceptable even with moderate packet loss and jitter.

## US-5.5.3 Avoid Transmitting Background Noise
**As a** player, **I want** voice activity detection and noise suppression applied before
encoding, **so that** my keyboard clicks, fan noise, and background sounds are suppressed
and I only transmit when I am actually speaking.

## US-5.5.4 Hear UI Text Read Aloud
**As an** accessibility tester, **I want** the engine to generate speech from UI text, quest
descriptions, and chat messages via platform TTS services, **so that** visually impaired
players can access all textual content through audio.

## US-5.5.5 See NPCs Lip-Sync to Dialogue
**As a** player, **I want** NPC and avatar facial animations to match spoken dialogue via
real-time viseme generation, **so that** characters appear to speak naturally during both
pre-recorded dialogue and live voice chat.

## US-5.5.6 Never Miss Important Dialogue
**As a** player, **I want** a priority queue for dialogue playback that prevents overlap,
interrupts low-priority barks for critical narrative lines, and expires stale barks after a
timeout, **so that** I always hear important story dialogue without leftover ambient chatter
playing out of context.

## US-5.5.7 Branch Conversations Based on Gameplay State
**As an** audio designer, **I want** dialogue trees represented as directed graphs with
condition-gated edges (quest progress, reputation, inventory), **so that** NPCs respond
contextually with branching choices and random variation without custom scripting.

## US-5.5.8 Manage Multiple Voice Chat Channels
**As a** player, **I want** to join proximity, party, guild, and raid voice channels
simultaneously with independent volume and per-speaker muting, **so that** I can prioritize
raid-leader callouts while monitoring background chatter from other channels.

## US-5.5.9 Avoid Echo When Using Speakers
**As a** player, **I want** acoustic echo cancellation to prevent other players from hearing
their own voices echoed back when I use speakers instead of headphones, **so that** voice
chat remains clear for all participants regardless of my audio setup.
