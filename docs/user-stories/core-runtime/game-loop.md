# Game Loop User Stories

## Frame Pipeline

| ID       | Persona                 |
|----------|-------------------------|
| US-1.11.1 | engine developer (P-26) |
| US-1.11.2 | game developer (P-15)   |
| US-1.11.3 | engine tester (P-27)    |

1. **US-1.11.1** — **As an** engine developer (P-26), **I want** the game loop to execute 8
   deterministic phases per frame (Input, Network Rx, Simulation, AI, Physics, Animation, Frame
   Snapshot, Frame End), **so that** all subsystems run in a predictable order with clear
   dependencies.
2. **US-1.11.2** — **As a** game developer (P-15), **I want** to insert custom phases with explicit
   ordering relative to built-in phases, **so that** game-specific subsystems like economy ticks and
   quest updates run at deterministic points in the frame.
3. **US-1.11.3** — **As an** engine tester (P-27), **I want** to verify that frame phases execute in
   the correct order across thousands of frames, **so that** deterministic ordering is maintained
   under varying system loads.

## Thread Architecture

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.11.4 | engine developer (P-26) |
| US-1.11.5 | engine tester (P-27)    |

1. **US-1.11.4** — **As an** engine developer (P-26), **I want** three thread roles (main for OS
   events, workers for game loop, render for GPU submission) with platform events forwarded via SPSC
   queue, **so that** the OS event loop never blocks game logic and GPU submission is isolated.
2. **US-1.11.5** — **As an** engine tester (P-27), **I want** to verify that platform events are
   forwarded from main thread to game loop with zero loss and zero contention, **so that** input
   latency is minimal and no events are dropped.

## Render Pipeline

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.11.6 | engine developer (P-26) |
| US-1.11.7 | engine tester (P-27)    |

1. **US-1.11.6** — **As an** engine developer (P-26), **I want** the render thread pipelined one
   frame behind the game loop via triple buffer, **so that** neither thread stalls waiting for the
   other and GPU utilization is maximized.
2. **US-1.11.7** — **As an** engine tester (P-27), **I want** to verify that the triple buffer never
   causes the game loop to stall even when the render thread runs at half the game loop rate,
   **so that** frame pacing remains consistent under GPU load.

## Game State Management

| ID        | Persona               |
|-----------|-----------------------|
| US-1.11.8 | game developer (P-15) |
| US-1.11.9 | game developer (P-15) |
| US-1.11.10 | engine tester (P-27) |

1. **US-1.11.8** — **As a** game developer (P-15), **I want** game state transitions (MainMenu,
   Loading, InGame, Paused) applied at frame boundaries, **so that** state changes are deterministic
   and never interrupt mid-frame processing.
2. **US-1.11.9** — **As a** game developer (P-15), **I want** to push and pop sub-modes (e.g.,
   Paused over InGame) that suspend parent mode systems, **so that** pause overlays and loading
   screens integrate cleanly with the game loop.
3. **US-1.11.10** — **As an** engine tester (P-27), **I want** to verify that mode push/pop
   correctly suspends and resumes parent mode systems, **so that** no gameplay systems run while a
   sub-mode is active.

## Frame Pacing

| ID         | Persona               |
|------------|-----------------------|
| US-1.11.11 | game developer (P-15) |
| US-1.11.12 | engine tester (P-27)  |

1. **US-1.11.11** — **As a** game developer (P-15), **I want** platform-native frame pacing (VSync,
   display link, VR reprojection), **so that** frame presentation is synchronized with the display
   refresh rate on every target platform.
2. **US-1.11.12** — **As an** engine tester (P-27), **I want** to verify that frame pacing aligns
   with display refresh on each platform, **so that** no screen tearing or irregular frame timing
   occurs during normal operation.
