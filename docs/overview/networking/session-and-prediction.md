# Session and Prediction

Server architecture, client prediction, and server reconciliation.

## What it covers

- Server authority: server owns authoritative state.
- Client prediction: clients simulate ahead of server updates.
- Server reconciliation: correcting client predictions when they diverge from server.
- Lag compensation: adjusting aim and hit detection for network latency.
- Command queue: storing player inputs for reordering and prediction.
- Timestamp synchronization: aligning server and client time.
- State snapshot: capturing world state for rollback.
- Rollback and re-prediction: correcting mispredicted state.
- Connection dropout handling: disconnection and reconnection logic.
- Joining in-progress game: late players catching up to current state.

## Concepts

### Server Authority and Client Prediction

Server maintains authoritative world state: entity positions, health, state. Clients predict client
character motion before server confirmation. Client sends input (move direction), client simulates
locally to show immediate response, then server confirms or corrects. This reduces perceived latency.

### Prediction and Reconciliation

Client maintains predicted state separate from confirmed state. When server update arrives, compare
server state to predicted state. If they match, prediction was correct. If not, apply correction: use
server state, re-apply unconfirmed inputs, re-predict. This reconciles divergence without visible
teleporting.

### Lag Compensation and Hit Detection

Network latency means client actions arrive delayed at server. Hitscans (raycast attacks) might miss
on server despite seeming to hit on client. Lag compensation winds back time: when processing client
hit request, rewind server state to when client fired, perform hit test at that time. This feels
fair to client despite latency.

### Command Queue

Client stores input commands with timestamps. Server confirms received command number. Client
re-predicts from last confirmed command, applying unconfirmed commands. If server updates diverge
from prediction, re-prediction corrects. This enables smooth offline play with catch-up on
reconnection.

### Rollback and Reconstruction

State snapshots capture position, velocity, animation state. On divergence, rollback to last
confirmed snapshot, re-apply all unconfirmed inputs, re-predict. This produces deterministic
convergence to correct state without visible jumps.

## How it fits

- See [transport-and-replication.md](./transport-and-replication.md) for transmission and
  synchronization.
- See [communication-and-replay.md](./communication-and-replay.md) for RPC systems and replay.
- See [../core-runtime/simulation-loop.md](../core-runtime/simulation-loop.md) for deterministic
  frame updates.
