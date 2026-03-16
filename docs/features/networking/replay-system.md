# 8.6 — Replay System

## Recording

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-8.6.1 | State Recording with Snapshots and Deltas | Record gameplay state as periodic full snapshots interleaved with per-tick delta streams, producing compact replay files that capture all networked state changes. Full snapshots serve as seek keyframes while deltas provide frame-accurate fidelity between them. Recording must operate with negligible CPU overhead on the server and support configurable snapshot intervals to trade file size against seek granularity. | R-8.6.1 | F-8.2.1, F-8.2.2 | Server-side recording; platform-agnostic. Mobile clients downloading replays receive compressed files sized for mobile storage constraints. |
| F-8.6.2 | Deterministic Playback | Replay recorded state deterministically by feeding snapshots and deltas into the client's simulation and rendering pipeline without a live server connection. Playback must reproduce the exact visual result of the original session, including interpolated entity motion, particle effects triggered by RPCs, and UI events. Non-deterministic systems (ambient particles, procedural audio) are seeded from replay metadata. | R-8.6.2 | F-8.6.1, F-8.4.3 | Mobile playback renders at the device's quality tier; visual fidelity matches live gameplay on that platform, not the recording platform. |

## Playback Control

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-8.6.3 | Seek, Fast-Forward, and Slow Motion | Allow the viewer to seek to any point in the replay by loading the nearest snapshot keyframe and replaying deltas forward, fast-forward at configurable speed multipliers (2x, 4x, 8x), slow down to frame-by-frame for analysis, and pause. Seeking must complete within one second regardless of replay length by maintaining a keyframe index. Essential for competitive play review and content creation. | R-8.6.3 | F-8.6.2 | Mobile limits fast-forward to 4x (vs. 8x on desktop) to avoid exceeding CPU budget during accelerated delta replay. |

## Spectating

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-8.6.4 | Live Spectator Mode | Allow designated clients to observe an in-progress game session with a configurable delay (e.g., 30 seconds to prevent ghosting in competitive play). Spectators receive the full replication stream but cannot send gameplay RPCs. Support free camera, player-locked camera, and overhead tactical view. Must scale to thousands of spectators per match for esports broadcast by using fan-out relay servers rather than burdening the game server. | R-8.6.4 | F-8.2.1, F-8.2.3 | Mobile spectators receive the same stream via relay servers. Mobile UI limits camera modes to player-locked and overhead (no free camera with touch). |
| F-8.6.5 | Kill Cam and Highlight Extraction | Capture a short rolling buffer of recent state (last 10-15 seconds) on the server and deliver it to the victim's client on death as a mini-replay viewed from the attacker's perspective. The kill cam replays use the same deterministic playback pipeline as full replays. Additionally, support extracting highlight clips from ongoing or completed replays for social sharing. | R-8.6.5 | F-8.6.1, F-8.6.2, F-8.4.5 | Video encoding for social sharing uses platform media APIs (Media Foundation on Windows, AVFoundation on macOS, VA-API/V4L2 on Linux). |
