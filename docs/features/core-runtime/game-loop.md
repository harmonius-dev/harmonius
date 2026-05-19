# 1.11 — Game Loop

## Frame Pipeline

| ID       | Feature                          |
|----------|----------------------------------|
| F-1.11.1 | 8-Phase Frame Pipeline           |
| F-1.11.2 | Custom Phase Support             |

1. **F-1.11.1** — Execute each frame as an 8-phase pipeline: Input, Network Rx, Simulation, AI,
   Physics, Animation, Frame Snapshot, and Frame End. Phases run in deterministic order with
   configurable phase-to-ECS-stage mapping. Each phase contains one or more ECS system groups
   scheduled by the dependency resolver.
   - **Deps:** F-1.1.25 (Dependency Resolution), F-1.1.26 (System Groups and Phases)
2. **F-1.11.2** — Support user-defined custom phases inserted with explicit before/after ordering
   relative to built-in phases. Custom phases enable game-specific subsystems (economy ticks, quest
   updates, cinematic sequencing) to run at deterministic points in the frame pipeline.
   - **Deps:** F-1.11.1

## Thread Architecture

| ID       | Feature                                |
|----------|----------------------------------------|
| F-1.11.3 | Three-Role Thread Architecture         |
| F-1.11.4 | SPSC Event Queue for Platform Events   |

1. **F-1.11.3** — Use three thread roles: the main thread runs the OS event loop and polls platform
   I/O completions; worker threads run the job system and drive the game loop; the render thread
   records and submits GPU commands. The game loop never runs on the main thread to avoid blocking
   the OS event loop.
   - **Deps:** F-1.8.1 (Async I/O), F-1.1.20 (Parallel Iteration)
   - **Platform:** Mobile: 1 main + 2 workers + 1 render. Switch: 1 main + 2 workers + 1 render.
     Desktop: 1 main + (cores - 2) workers + 1 render. All platforms: main thread is the OS event
     loop thread.
2. **F-1.11.4** — Forward platform events (input, window resize, focus change) from the main thread
   to the game loop thread via a lock-free SPSC queue. The queue has zero contention between
   producer (main thread) and consumer (game loop thread), ensuring the OS event loop is never
   blocked by game logic.
   - **Deps:** F-1.11.3

## Render Pipeline

| ID       | Feature                                     |
|----------|---------------------------------------------|
| F-1.11.5 | Immutable RenderFrame Snapshot              |
| F-1.11.6 | Lock-Free Triple Buffer for Render Transfer |
| F-1.11.7 | Pipelined Rendering One Frame Behind        |

1. **F-1.11.5** — During the Frame Snapshot phase, produce an immutable RenderFrame containing
   interpolated transforms, draw lists, camera parameters, light data, VFX state, and UI layout. The
   snapshot captures everything the render thread needs to produce a frame without accessing the ECS
   world.
   - **Deps:** F-1.11.1, F-1.2.4 (Transform Propagation)
2. **F-1.11.6** — Use a lock-free triple buffer to transfer RenderFrame snapshots from the game loop
   to the render thread. The game loop writes to one slot while the render thread reads from
   another; the third slot absorbs timing differences. Neither thread ever stalls waiting for the
   other.
   - **Deps:** F-1.11.5
3. **F-1.11.7** — Pipeline rendering one frame behind the game loop. The render thread consumes the
   previous frame's RenderFrame while the game loop produces the current frame's data. This hides
   render latency and maximizes GPU utilization by overlapping simulation and rendering.
   - **Deps:** F-1.11.5, F-1.11.6

## Game State Management

| ID       | Feature                                    |
|----------|--------------------------------------------|
| F-1.11.8 | Game State Machine with Sync Transitions   |
| F-1.11.9 | Mode Manager with Push/Pop Stacking        |

1. **F-1.11.8** — Provide a game state machine where transitions are requested via
   request_transition() and applied at frame-boundary sync points. States (MainMenu, Loading,
   InGame, Paused) control which system groups are active. Transitions trigger OnEnter/OnExit
   observers for cleanup and initialization.
   - **Deps:** F-1.1.38 (ECS-Integrated State Machine), F-1.1.26 (System Groups)
2. **F-1.11.9** — Provide a mode manager supporting mode graph transitions and sub-mode push/pop
   stacking. Pushing a sub-mode (e.g., Paused over InGame) suspends the parent mode's systems while
   the sub-mode runs. Popping restores the parent mode. Mode transitions follow a mode graph with
   validated edges.
   - **Deps:** F-1.11.8

## Compiled Frame

| ID        | Feature                         |
|-----------|---------------------------------|
| F-1.11.10 | Compiled Frame Schedule Caching|

1. **F-1.11.10** — Compile the ECS Schedule into a CompiledFrame that caches system ordering, thread
   assignments, and sync point locations. The CompiledFrame is reused across frames and recompiled
   only when the system set changes (plugin load/unload, mode switch). This amortizes schedule
   compilation cost over hundreds of frames.
   - **Deps:** F-1.1.25 (Dependency Resolution), F-1.11.1

## Frame Pacing

| ID        | Feature                              |
|-----------|--------------------------------------|
| F-1.11.11 | Platform-Native Frame Pacing        |

1. **F-1.11.11** — Support platform-specific frame pacing to synchronize frame presentation with
   display refresh. Desktop uses VSync via swapchain present modes. Mobile uses Vulkan WSI present
   timing (Apple) and Vulkan WSI present timing (Android) for display-aligned frame submission. VR
   uses reprojection/timewarp with automatic half-rate fallback when frame time exceeds target.
   - **Deps:** F-1.11.7
   - **Platform:** Desktop: VSync on/off/adaptive via swapchain. Mobile: display link callbacks. VR:
     compositor-driven pacing with reprojection.
