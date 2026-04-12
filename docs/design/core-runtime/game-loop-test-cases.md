# Game Loop Test Cases

Companion test cases for [game-loop.md](game-loop.md).

## Unit Tests

### TC-1.1.1.1 Eight Phases Execute in Declared Order

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |

1. **#1** -- Build `GameLoopGraph` with all 8 phases (Input, NetworkReceive, Simulation, AiUpdate,
   PhysicsStep, AnimationUpdate, FrameSnapshot, FrameEnd), compile, record execution order
   - **Input:** Default 8-phase graph, no custom phases
   - **Expected:** Phases execute in order 1 through 8

### TC-1.1.1.2 Custom Phase Inserts at Correct Position

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |
| 2 | R-1.1.1     |

1. **#1** -- Add `Phase::Custom(0)` with dependency after Simulation and before AiUpdate, compile,
   record execution order
   - **Input:** 8 built-in phases + 1 custom phase
   - **Expected:** Custom phase runs after phase 3 and before phase 4
2. **#2** -- Add two custom phases with mutual ordering constraint, compile, record execution order
   - **Input:** `Custom(0)` before `Custom(1)`, both between Simulation and AiUpdate
   - **Expected:** `Custom(0)` runs before `Custom(1)`, both between phases 3 and 4

### TC-1.1.1.3 Cycle Detection Rejects Invalid DAG

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |

1. **#1** -- Build graph with cycle: Input depends on FrameEnd, FrameEnd depends on Input
   - **Input:** Two-node graph with circular edges
   - **Expected:** `compile()` returns `Err(CompileError::CycleDetected)`

### TC-1.1.1.4 Access Conflict Detected at Compile Time

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |
| 2 | R-1.1.1     |

1. **#1** -- Two systems in the same phase both write to the same component type with no ordering
   - **Input:** Phase with system A (`&mut Position`) and system B (`&mut Position`), no dependency
     edge
   - **Expected:** `compile()` returns `Err(CompileError::AccessConflict)`
2. **#2** -- Two systems where one reads and one writes the same component type with no ordering
   - **Input:** Phase with system A (`&Position`) and system B (`&mut Position`), no dependency edge
   - **Expected:** `compile()` returns `Err(CompileError::AccessConflict)`

### TC-1.1.1.5 Sync Barrier Insertion

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |

1. **#1** -- Two systems with conflicting access but explicit ordering (A before B), compile,
   inspect compiled task graph
   - **Input:** System A (`&mut Position`) ordered before system B (`&Position`)
   - **Expected:** `CompiledFrame` task graph contains a sync barrier between A and B

### TC-1.1.1.6 Schedule Builds GameLoopGraph

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |
| 2 | R-1.1.1     |

1. **#1** -- Register systems into ECS `Schedule` stages (PreUpdate, Update, FixedUpdate,
   PostUpdate, PreRender, Last), call `build_game_loop()`
   - **Input:** One system per ECS stage
   - **Expected:** Returned `GameLoopGraph` has 8 `PhaseNode` entries with correct stage mappings
2. **#2** -- Verify FixedUpdate systems produce a `PhaseNode::SubGraph` variant
   - **Input:** Physics system in FixedUpdate stage
   - **Expected:** Phase 5 node is `PhaseNode::SubGraph` containing the substep graph

### TC-1.1.2.1 Zero Delta Time Produces Zero Ticks

| # | Requirement |
|---|-------------|
| 1 | R-1.1.2     |

1. **#1** -- Create `FixedTimestep` with `tick_duration = 16.67 ms`, call
   `accumulate(Duration::ZERO)`, then `consume()`
   - **Input:** dt = 0
   - **Expected:** `consume()` returns 0

### TC-1.1.2.2 Max Ticks Per Frame Caps Accumulator

| # | Requirement |
|---|-------------|
| 1 | R-1.1.2     |
| 2 | R-1.1.2     |

1. **#1** -- Create `FixedTimestep` with `tick_duration = 16 ms`, `max_ticks_per_frame = 4`,
   accumulate 200 ms, call `consume()`
   - **Input:** dt = 200 ms (enough for 12 ticks)
   - **Expected:** `consume()` returns 4, not 12
2. **#2** -- After capped consume, check remaining accumulator
   - **Input:** State from step #1
   - **Expected:** Excess time is discarded (spiral of death protection); accumulator <=
     tick_duration

### TC-1.1.2.3 Alpha Returns 0.0 at Tick Boundary

| # | Requirement |
|---|-------------|
| 1 | R-1.1.2     |

1. **#1** -- Accumulate exactly `tick_duration`, call `consume()`, then `alpha()`
   - **Input:** dt = 16.67 ms, tick_duration = 16.67 ms
   - **Expected:** `alpha()` returns 0.0 (no remainder)

### TC-1.1.2.4 Alpha Returns 0.5 at Half-Tick

| # | Requirement |
|---|-------------|
| 1 | R-1.1.2     |

1. **#1** -- Accumulate 1.5x `tick_duration`, call `consume()` (returns 1), then `alpha()`
   - **Input:** dt = 25.0 ms, tick_duration = 16.67 ms
   - **Expected:** `alpha()` returns approximately 0.5 (8.33 ms / 16.67 ms)

### TC-1.1.2.5 Alpha Approaches 1.0 Before Next Tick

| # | Requirement |
|---|-------------|
| 1 | R-1.1.2     |

1. **#1** -- Accumulate `tick_duration - epsilon`, call `consume()` (returns 0), then `alpha()`
   - **Input:** dt = 16.66 ms, tick_duration = 16.67 ms
   - **Expected:** `alpha()` returns value close to 1.0 but strictly less than 1.0

### TC-14.3.1.1 SPSC Push Pop Single Element

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |

1. **#1** -- Create `SpscQueue`, push one `PlatformEvent::WindowClose`, pop
   - **Input:** Single push followed by single pop
   - **Expected:** `pop()` returns `Some(PlatformEvent::WindowClose)`

### TC-14.3.1.2 SPSC Drain Returns All in Order

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |

1. **#1** -- Push 100 `PlatformEvent::RawInput` events with sequential IDs, call `drain()`, collect
   - **Input:** 100 events pushed in order 0..100
   - **Expected:** `drain()` yields 100 events in insertion order (ID 0, 1, 2, ..., 99)

### TC-14.3.1.3 SPSC Push to Full Queue Returns Err

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |

1. **#1** -- Create bounded `SpscQueue` with capacity N, push N elements, push one more
   - **Input:** N+1 pushes on capacity-N queue
   - **Expected:** First N pushes return `Ok(())`, push N+1 returns `Err(value)`

### TC-14.3.1.4 SPSC Pop from Empty Returns None

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |

1. **#1** -- Create `SpscQueue`, pop without pushing
   - **Input:** Empty queue
   - **Expected:** `pop()` returns `None`

### TC-14.3.3.1 Triple Buffer Write Before Read

| # | Requirement |
|---|-------------|
| 1 | R-14.3.3    |

1. **#1** -- Create `TripleBuffer`, write a `RenderFrame` with `frame_index = 1`, then read
   - **Input:** Single write followed by single read
   - **Expected:** `read()` returns `Some(&RenderFrame)` with `frame_index == 1`

### TC-14.3.3.2 Triple Buffer Concurrent Access

| # | Requirement |
|---|-------------|
| 1 | R-14.3.3    |
| 2 | R-14.3.3    |

1. **#1** -- Writer thread writes 1000 frames concurrently with reader thread reading
   - **Input:** Writer publishes frames 0..1000, reader polls continuously
   - **Expected:** No data races (ThreadSanitizer clean), no panics
2. **#2** -- Writer thread never blocks during concurrent read
   - **Input:** Reader holds reference while writer writes next frame
   - **Expected:** Writer completes without stalling

### TC-14.3.3.3 Triple Buffer Read Returns Latest

| # | Requirement |
|---|-------------|
| 1 | R-14.3.3    |

1. **#1** -- Write frames with `frame_index` 1, 2, 3 without any reads, then read once
   - **Input:** Three sequential writes, one read
   - **Expected:** `read()` returns frame with `frame_index == 3` (most recent)

### TC-14.3.3.4 Triple Buffer Read Without Write

| # | Requirement |
|---|-------------|
| 1 | R-14.3.3    |

1. **#1** -- Create `TripleBuffer`, read before any write, then read again after one write and a
   subsequent read (no new write between reads)
   - **Input:** Read on fresh buffer, then write, read, read again
   - **Expected:** First `read()` returns `None`; second returns the frame; third returns `None`

### TC-1.1.1.7 RenderFrame Interpolation

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |
| 2 | R-1.1.2     |

1. **#1** -- Set `PreviousGlobalTransform` to identity, `GlobalTransform` to translate(10, 0, 0),
   alpha = 0.0, run snapshot phase
   - **Input:** prev = identity, current = (10,0,0), alpha = 0.0
   - **Expected:** `InterpolatedTransform` equals identity (previous position)
2. **#2** -- Same transforms, alpha = 0.5
   - **Input:** prev = identity, current = (10,0,0), alpha = 0.5
   - **Expected:** `InterpolatedTransform` equals translate(5, 0, 0)

### TC-1.1.1.8 RenderFrame Alpha Passthrough

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |
| 2 | R-1.1.2     |

1. **#1** -- Build `RenderFrame` during snapshot phase with `FixedTimestep::alpha()` returning 0.75
   - **Input:** Timestep accumulator at 75% of tick
   - **Expected:** `RenderFrame::alpha == 0.75`
2. **#2** -- Verify `RenderFrame::frame_index` increments each frame
   - **Input:** Execute two full frames
   - **Expected:** Second frame's `frame_index` equals first frame's `frame_index + 1`

### TC-1.1.1.9 Phase to ECS Stage Mapping

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |

1. **#1** -- Call `Schedule::build_game_loop()` with systems registered to each ECS stage, inspect
   the produced `PhaseNode` entries
   - **Input:** Systems in PreUpdate, Update, FixedUpdate, PostUpdate, PreRender, Last
   - **Expected:** Mapping matches design table: Input/NetworkRx = PreUpdate, Simulation/AI =
     Update, Physics = FixedUpdate, Animation = PostUpdate, Snapshot = PreRender, FrameEnd = Last

## Integration Tests

### TC-14.3.5.1 Input to Snapshot End-to-End Frame

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |
| 2 | R-14.3.5    |
| 3 | R-14.3.5    |

1. **#1** -- Push `PlatformEvent::RawInput` into SPSC queue, execute one full compiled frame,
   inspect resulting `RenderFrame` in triple buffer
   - **Input:** Raw keyboard event mapped to "MoveForward" action
   - **Expected:** Entity position updated by simulation; `RenderFrame::transforms` contains the
     interpolated result
2. **#2** -- Verify all 8 phases executed
   - **Input:** Instrumented phase callbacks recording execution timestamps
   - **Expected:** All 8 phases ran in order; timestamps are monotonically increasing
3. **#3** -- Verify render thread receives frame
   - **Input:** Triple buffer read after game loop frame completes
   - **Expected:** `read()` returns `Some` with matching `frame_index`

### TC-14.3.5.2 Cross-Phase Data Flow Correctness

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |
| 2 | R-14.3.5    |

1. **#1** -- Phase 1 (Input) writes `ActionEvent` to ECS; Phase 3 (Simulation) reads it and updates
   `Position`; Phase 7 (Snapshot) reads `Position` and builds `RenderFrame`
   - **Input:** "Jump" action event from input phase
   - **Expected:** Position.y increases after simulation; `RenderFrame` reflects new position
2. **#2** -- Phase 5 (Physics) writes collision result; Phase 6 (Animation) reads it to trigger
   state machine transition
   - **Input:** Collision event between two entities
   - **Expected:** Animation state transitions to "hit_react" after physics phase

### TC-14.3.5.3 Game State Transition at Frame End

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |
| 2 | R-14.3.5    |

1. **#1** -- During phase 3 (Simulation), call `GameStateManager::request_transition(Paused)`, run
   frame to completion, check state at next frame start
   - **Input:** Transition request mid-frame
   - **Expected:** `current()` still returns `InGame` until phase 8 completes; next frame starts
     with `current()` returning `Paused`
2. **#2** -- Request transition during loading, verify `CompiledFrame` recompiles
   - **Input:** Transition from `Loading` to `InGame`
   - **Expected:** New `CompiledFrame` compiled with InGame system set; loading-only systems removed

### TC-14.3.5.4 Worker Thread Drives Game Loop

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |
| 2 | R-14.3.5    |

1. **#1** -- Execute 10 frames via `job_system::scope`, verify calling worker participates in
   work-stealing
   - **Input:** CompiledFrame with fan-out phase dispatching 8 parallel jobs
   - **Expected:** Driver thread executes at least one stolen job (not idle during fan-out)
2. **#2** -- Verify main thread is not blocked during game loop execution
   - **Input:** Main thread pumps OS events while game loop runs on worker
   - **Expected:** Main thread SPSC pushes complete within 1 ms during game loop execution

## Benchmarks

### TC-14.3.1.1a Frame Time Within 16.6 ms Budget

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full 8-phase frame, 1000 entities | Total frame | < 16.6 ms | R-14.3.1 |
| 2 | Full 8-phase frame, 10000 entities | Total frame | < 16.6 ms | R-14.3.1 |

### TC-14.3.1.2a Per-Phase Latency Measurement

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Input phase drain 100 events | Phase latency | < 0.5 ms | R-14.3.1 |
| 2 | Simulation phase 1 fixed tick | Phase latency | < 4 ms | R-14.3.1 |
| 3 | Physics phase 1 substep | Phase latency | < 4 ms | R-14.3.1 |
| 4 | Snapshot phase 10000 transforms | Phase latency | < 2 ms | R-14.3.1 |

### TC-14.3.3.1a Job Dispatch Overhead Per Frame

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Dispatch 200 system jobs per frame | Per-dispatch | < 1 us | R-14.3.3 |
| 2 | Fan-out 8 parallel phase jobs | Fan-out + join | < 50 us | R-14.3.3 |

### TC-14.3.3.2a Triple Buffer Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Write + read RenderFrame at 60 Hz | Write latency | < 100 us | R-14.3.3 |
| 2 | Concurrent write/read for 10000 frames | Zero stalls | 0 writer blocks | R-14.3.3 |

### TC-14.3.1.3a SPSC Queue Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Push + drain 1000 events per frame | Per-op | < 100 ns | R-14.3.1 |
| 2 | Sustained 60 Hz with 500 events/frame | Throughput | 30K events/s | R-14.3.1 |

### TC-1.1.1.1a CompiledFrame Compilation Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Compile 200-system graph | Compile time | < 10 ms | R-1.1.1 |
| 2 | Compile 50-system graph | Compile time | < 2 ms | R-1.1.1 |
