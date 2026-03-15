# User Stories -- 8.6 Replay System

## US-8.6.1 Review My Death from the Attacker's Perspective

**As a** player (P-23), **I want** a kill cam that replays the last 10 seconds from my attacker's
viewpoint, **so that** I can understand how I was killed and improve my positioning.

## US-8.6.2 Spectate Matches for Esports Broadcast

**As a** player (P-23), **I want** to spectate live matches with a 30-second delay using free
camera, player-locked camera, and overhead tactical view, **so that** I can watch competitive play
from any angle without impacting the game server.

## US-8.6.3 Review My Gameplay Frame-by-Frame

**As a** player (P-23), **I want** to seek, fast-forward (2x/4x/8x), slow down, and pause replays at
any point with sub-second seek times, **so that** I can analyze my play in detail for competitive
improvement or content creation.

## US-8.6.4 Share Highlight Clips with Friends

**As a** player (P-23), **I want** to extract highlight clips from replays and encode them as video
using platform media APIs, **so that** I can share memorable moments on social media without
third-party screen recording.

## US-8.6.5 Record Replays with Minimal Server Overhead

**As a** server admin (P-22), **I want** replay recording to operate with negligible CPU overhead
using periodic snapshots and per-tick deltas, **so that** recording every match does not reduce
server capacity or increase tick time.

## US-8.6.6 Configure Replay Snapshot Intervals per Game Mode

**As a** server admin (P-22), **I want** configurable snapshot intervals per game mode to trade file
size against seek granularity, **so that** short competitive matches use frequent snapshots for
precise analysis while long open-world sessions use sparser snapshots to conserve storage.

## US-8.6.7 Implement Deterministic Playback Pipeline

**As a** game developer (P-15), **I want** replays to feed snapshots and deltas into the client's
simulation and rendering pipeline deterministically, **so that** playback reproduces the exact
visual result of the original session including interpolated motion, particle effects, and UI
events.

## US-8.6.8 Scale Spectator Mode to Thousands of Viewers

**As an** engine developer (P-26), **I want** spectator mode to use fan-out relay servers for
replication stream distribution, **so that** thousands of spectators can watch a live match without
burdening the authoritative game server.

## US-8.6.9 Verify Replay Determinism Across Platforms

**As an** engine tester (P-27), **I want** automated tests that record a match on one platform and
play it back on all supported platforms, comparing visual output hash per frame, **so that** I can
detect any cross-platform determinism regressions.

## US-8.6.10 Verify Spectator Delay Prevents Ghosting

**As an** engine tester (P-27), **I want** tests that confirm spectators never receive replication
data earlier than the configured delay, **so that** competitive integrity is maintained and
spectator ghosting is impossible.

## US-8.6.11 Watch Replays on Mobile with Adapted Quality

**As a** player (P-23), **I want** to download and watch replays on my mobile device with compressed
file sizes and playback at my device's quality tier, **so that** I can review matches on the go
without exceeding my storage or data budget.

## US-8.6.12 Design Spectator Camera Presets for Broadcast

**As a** designer (P-5), **I want** to configure spectator camera presets (overhead angle, follow
distance, transition speed) per game mode, **so that** esports broadcasts have polished,
game-mode-appropriate camera work out of the box.
