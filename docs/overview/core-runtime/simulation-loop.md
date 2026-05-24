# Simulation Loop

How the engine divides each frame into ordered phases and executes gameplay logic.

## What it covers

- Deterministic 8-phase pipeline: Input, Network Receive, Simulation, AI, Physics, Animation,
  Frame Snapshot, Frame End.
- Three-thread architecture: main thread (OS events), worker threads (game loop + parallel
  tasks), render thread.
- Lock-free queues forwarding events from the main thread to game logic.
- Immutable per-frame render snapshots decoupling simulation from rendering.
- Triple buffering allowing the render thread to draw frame N-1 while workers compute frame N.
- Deterministic fixed timesteps at 30, 60, or 120 fps with rollback semantics for replays and
  netcode.
- User-defined custom phases interleaved with built-ins.

## Concepts

### Frame Phases

Each frame, the game loop runs eight phases in sequence. Each phase may fan out to multiple worker
threads executing tasks in parallel. Input processing happens on the main thread; most simulation
happens on workers. By frame end, a render snapshot (camera, transforms, draw lists) is ready for
the render thread to consume.

### Pipelined Rendering

The render thread operates independently one frame behind the game loop. While the game computes
frame N, the render thread renders frame N-1. Triple buffering ensures neither thread waits for the
other, maximizing both CPU and GPU utilization.

### Game State Machines

The engine runs a configurable state machine governing each frame (play, pause, load, etc.). States
have on-entry and on-exit hooks. Sub-state stacking enables in-game menus without halting the game
loop.

## How it fits

- See [world-and-entities](./world-and-entities.md) for how entities are created, removed, and
  modified during phases.
- See [spatial-indexing](./spatial-indexing.md) for when spatial queries are safe and when they
  become stale.
- Integrates with [platform](../platform/windowing-and-display.md) for frame pacing and VR
  timing.
