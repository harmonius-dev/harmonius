# Communication and Replay

RPC systems, player-to-player messaging, and deterministic replay.

## What it covers

- Remote procedure calls (RPC): invoking functions on remote peers.
- RPC targeting: sending to specific players or broadcasting.
- Reliability: ensuring critical RPCs arrive.
- Message routing: directing messages to intended recipients.
- Player chat and messaging: text communication.
- Voice chat integration: audio communication network.
- Emotes and animations: synchronized cosmetic animations.
- Deterministic replay: recording and replaying match exactly.
- Replay recording: capturing all inputs and events.
- Playback and seeking: watching replays at any speed.

## Concepts

### Remote Procedure Calls

RPC functions run on remote machines. Client calls server-authority function (server validates); server
calls client functions (showing effects). RPC parameters serialize and transmit. Server validates RPC
calls for cheating: server never trusts client commands, only client inputs (move, attack direction).

### RPC Reliability and Delivery

Some RPCs are critical (activate ability, confirm kill) and must deliver; others are cosmetic and
can drop. Reliable RPCs acknowledge delivery; unreliable RPCs send once. Targeted RPCs reach specific
players; broadcasts reach all players. Routing addresses messages by player ID or team.

### Chat and Voice Communication

Text chat messages route through server to target recipients (all players, team, whisper). Voice chat
captures audio, encodes, sends to recipients; recipients decode and play. Voice chat is typically
unreliable: dropped packets cause audio glitches, but missing a syllable is acceptable.

### Deterministic Replay

Replay records all inputs and RNG seeds. Playing back the replay re-simulates from initial state
using recorded inputs and seeds. Deterministic physics and AI produce identical behavior each
replay. Seeking jumps to frame N by fast-forwarding or rewinding (if rewinding is supported). This
enables spectators watching matches or players reviewing their performance.

### Recording and Playback

Replay file stores frame data: input frame N had movement direction X and button presses Y. On
playback, inject frame N's inputs into frame N's simulation. All clients see identical behavior
(deterministic). Interpolation between frames smooths appearance.

## How it fits

- See [transport-and-replication.md](./transport-and-replication.md) for packet transmission.
- See [session-and-prediction.md](./session-and-prediction.md) for session management and
  validation.
- See [infrastructure-and-anti-cheat.md](./infrastructure-and-anti-cheat.md) for anti-cheat
  validation.
