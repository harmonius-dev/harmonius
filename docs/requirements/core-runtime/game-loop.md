# R-1.11 — Game Loop Requirements

## Frame Pipeline

1. **R-1.11.1** — The engine **SHALL** execute each frame as an 8-phase pipeline (Input, Network Rx,
   Simulation, AI, Physics, Animation, Frame Snapshot, Frame End) with deterministic ordering and
   configurable phase-to-ECS-stage mapping.
   - **Rationale:** A fixed phase pipeline ensures all subsystems run in a predictable order with
     clear data dependencies, enabling deterministic simulation.
   - **Verification:** Register systems in each phase; verify execution order matches phase ordering
     across 1,000 frames. Remap a phase to a different ECS stage; verify systems move accordingly.

## Thread Architecture

2. **R-1.11.2** — The engine **SHALL** use three thread roles: main thread (OS event loop + platform
   I/O polling), worker threads (job system, game loop driver), and render thread (GPU command
   recording + submission).
   - **Rationale:** Separating thread roles prevents the OS event loop from blocking game logic and
     isolates GPU submission latency from simulation.
   - **Verification:** Verify main thread runs only OS event loop and I/O polling. Verify game loop
     executes on a worker thread. Verify render thread records and submits GPU commands
     independently.

## Render Frame Snapshot

3. **R-1.11.3** — The engine **SHALL** produce an immutable RenderFrame snapshot during the Frame
   Snapshot phase containing interpolated transforms, draw lists, camera, lights, VFX state, and UI
   layout, submitted to the render thread via triple buffer.
   - **Rationale:** An immutable snapshot decouples the game loop from the render thread, allowing
     both to run at independent rates.
   - **Verification:** Produce a RenderFrame; verify all fields are populated. Verify the render
     thread reads a consistent snapshot with no partial updates.

## Triple Buffer

4. **R-1.11.4** — The engine **SHALL** use a lock-free triple buffer for game-loop-to-render-thread
   data transfer, ensuring the game loop never stalls waiting for the render thread.
   - **Rationale:** Triple buffering eliminates producer-consumer contention so the game loop
     maintains consistent frame pacing.
   - **Verification:** Run game loop at 60 Hz and render thread at 30 Hz; verify game loop never
     stalls. Verify render thread always reads the most recent completed snapshot.

## Main Thread Event Queue

5. **R-1.11.5** — The engine **SHALL** use a lock-free SPSC queue to forward platform events from
   the main thread to the game loop thread with zero contention.
   - **Rationale:** Platform events (input, window resize, focus) originate on the main thread but
     must be processed by the game loop without blocking the OS event loop.
   - **Verification:** Enqueue 10,000 events from the main thread; dequeue from the game loop
     thread; verify all events received in order with no loss. Verify zero contention via profiling.

## Game State and Mode Management

6. **R-1.11.6** — The engine **SHALL** provide a game state machine with request_transition()
   applied at sync points, and a mode manager supporting mode graph transitions and sub-mode
   push/pop stacking.
   - **Rationale:** State transitions must be deterministic and never interrupt mid-frame
     processing; mode stacking enables pause overlays and loading screens.
   - **Verification:** Request transition from MainMenu to InGame; verify transition applies at
     frame boundary. Push Paused sub-mode; verify InGame systems are suspended and Paused systems
     run. Pop Paused; verify InGame resumes.

## Pipelined Rendering

7. **R-1.11.7** — The engine **SHALL** pipeline rendering one frame behind the game loop, with the
   render thread consuming the previous frame's RenderFrame while the game loop produces the current
   frame's data.
   - **Rationale:** Pipelining hides render latency and maximizes GPU utilization by overlapping
     simulation and rendering.
   - **Verification:** Verify render thread processes frame N-1 while game loop processes frame N.
     Verify no data race between concurrent frames via ThreadSanitizer.

## Compiled Frame Reuse

8. **R-1.11.8** — The engine **SHALL** compile the Schedule into a CompiledFrame reused across
   frames, recompiling only when the system set changes (plugin load/unload, mode switch).
   - **Rationale:** Recompiling the schedule every frame wastes CPU time; caching the compiled frame
     amortizes the cost over hundreds of frames.
   - **Verification:** Run 1,000 frames with no system changes; verify CompiledFrame is compiled
     once. Hot-reload a plugin; verify recompilation occurs. Verify schedule build under 50 ms for
     500 systems.

## Custom Phases

9. **R-1.11.9** — The engine **SHALL** support user-defined custom phases with explicit ordering
   relative to built-in phases.
   - **Rationale:** Game-specific subsystems (economy ticks, quest updates) need dedicated phases
     with deterministic ordering relative to built-in phases.
   - **Verification:** Insert a custom phase between Simulation and AI; register a system in it;
     verify it executes after Simulation and before AI.

## Frame Pacing

10. **R-1.11.10** — The engine **SHALL** support platform-specific frame pacing: VSync via swapchain
    on desktop, Vulkan WSI present timing/Vulkan WSI present timing on mobile, and
    reprojection/timewarp with half-rate fallback for VR.
    - **Rationale:** Platform-native frame pacing prevents screen tearing and maintains smooth
      visual output on each target platform.
    - **Verification:** Enable VSync on desktop; verify frame times align with display refresh. On
      mobile, verify frame submission aligns with display link callbacks. In VR, verify half-rate
      fallback activates when frame time exceeds target.
