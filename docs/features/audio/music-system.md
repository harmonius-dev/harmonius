# 5.4 — Music System

## Adaptive Layers

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.4.1 | Vertical Re-Mixing (Layered Stems) | Play multiple synchronized stems (e.g., percussion, strings, brass, choir) for a single music cue and crossfade individual stem volumes in response to gameplay state. Vertical re-mixing lets a single composed piece scale from quiet exploration to full combat intensity by adding or removing layers, avoiding jarring track switches during MMO world traversal. | R-5.4.1 | F-5.1.6, F-5.1.3 | Concurrent stem count scales per tier: mobile 4-6 stems, Switch 8, desktop 12+. Mobile uses lower-bitrate stem encoding to save bandwidth. |
| F-5.4.2 | Horizontal Re-Sequencing | Arrange music segments (intro, loop-A, loop-B, transition, outro) in a directed graph and select the next segment at runtime based on gameplay conditions. Branch points are quantized to bar or beat boundaries so that transitions always land musically. Supports seamless zone-crossing music in open-world MMOs. | R-5.4.2 | F-5.1.6, F-5.4.4 | Segment graph complexity is uniform across platforms. Prefetch buffer size for next segment reduced on mobile due to memory constraints. |

## Transitions

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.4.3 | Transition Rules (Crossfade and Beat-Sync) | Define per-edge transition behaviors in the music graph: immediate cut, timed crossfade, beat-synced crossfade, next-bar switch, and custom fade curves. Transition rules ensure musically coherent shifts between exploration, combat, victory, and social themes even when gameplay state changes arrive at unpredictable times. | R-5.4.3 | F-5.4.2, F-5.4.4 | Transition rules are lightweight on all platforms. Crossfade duration may be shortened on mobile to reduce concurrent stream overlap. |

## Tempo and Beat Tracking

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.4.4 | Tempo and Beat Clock | Maintain a global music clock that tracks tempo (BPM), time signature, and current beat/bar position derived from the playing music segment's metadata. The beat clock publishes beat and bar events consumed by the transition system, stinger scheduler, and gameplay code (e.g., rhythm-synced ability cooldowns or UI pulse effects). | R-5.4.4 | F-5.1.6 | Beat clock tracking is lightweight on all platforms. No platform-specific scaling required. |

## Stingers and One-Shots

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.4.5 | Stinger Playback | Trigger short musical phrases (stingers) on gameplay events -- level-up fanfares, boss aggro hits, loot drops -- that layer on top of or momentarily duck the current score. Stingers can be beat-quantized or immediate, and the system prevents pile-up by enforcing cooldowns and priority rules when multiple triggers arrive in rapid succession. | R-5.4.5 | F-5.4.4, F-5.1.4 | Stinger playback draws from the voice pool (see F-5.1.4). Stinger pile-up prevention is more aggressive on mobile (longer cooldowns). |

## Playlists and Randomization

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-5.4.6 | Playlists and Weighted Randomization | Organize music cues into playlists with sequential, shuffle, and weighted-random playback modes. Weighting parameters include time-since-last-play, player location, and gameplay tags. Playlists support non-repeating constraints so that the same track is not heard twice in succession. | R-5.4.6 | F-5.4.2 | Playlist logic is lightweight CPU-side on all platforms. No platform-specific scaling required. |
| F-5.4.7 | Dynamic Intensity Parameter | Expose a normalized intensity parameter (0.0 to 1.0) driven by gameplay systems (combat threat, proximity to points of interest, narrative tension) that simultaneously controls vertical stem mixing, horizontal segment selection, and stinger likelihood. A single authored intensity curve per music cue simplifies designer workflow while producing rich adaptive scores. | R-5.4.7 | F-5.4.1, F-5.4.2, F-5.4.5 | Intensity parameter is a single float on all platforms. Stem/segment responses inherit per-tier stem limits from F-5.4.1. |
