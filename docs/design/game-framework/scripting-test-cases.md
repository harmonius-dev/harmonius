# Gameplay Scripting Test Cases

Companion test cases for [scripting.md](scripting.md).

## Unit Tests

### TC-13.4.1.1 Opcode Add F32

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — `AddF32` opcode, slot A=3.0, slot B=4.0
   - **Expected:** Destination slot = 7.0

### TC-13.4.1.2 Opcode Jump If

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |
| 2 | R-13.4.1    |

1. **#1** — `JumpIf` with condition=true, target=PC+5
   - **Expected:** PC advances to PC+5
2. **#2** — `JumpIf` with condition=false
   - **Expected:** PC advances to next opcode

### TC-13.4.1.3 Opcode Get Component

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — `GetComponent(Health)` on entity with Health(80.0)
   - **Expected:** Destination slot = 80.0

### TC-13.4.1.4 Opcode Set Component

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — `SetComponent(Health, 50.0)`
   - **Expected:** Command buffer contains deferred Health write

### TC-13.4.1.5 Opcode Emit Event

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — `EmitEvent(DamageDealt { amount: 25 })`
   - **Expected:** Event appears in event channel

### TC-13.4.1.6 Opcode Query Begin Next

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — `QueryBegin(Health)`, 3 entities with Health
   - **Expected:** `QueryNext` yields 3 results then terminates

### TC-13.4.1.7 Opcode Blackboard Get Set

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — `BlackboardSet("target", entity_42)`, then `BlackboardGet("target")`
   - **Expected:** Returns entity_42

### TC-13.4.1.8 Opcode State Transition

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — `StateTransition("Patrol", "Chase")`
   - **Expected:** State machine transitions to Chase

### TC-13.4.1.9 Variable Store Get Set

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |
| 2 | R-13.4.1    |

1. **#1** — `set(slot_0, 42.0)`, `get(slot_0)`
   - **Expected:** Returns 42.0
2. **#2** — `set(graph_var_1, true)`, `get(graph_var_1)`
   - **Expected:** Returns true

### TC-13.4.3.1 Variable Layout Hash

| # | Requirement |
|---|-------------|
| 1 | R-13.4.3    |

1. **#1** — Two identical `VariableLayout` with same slots
   - **Expected:** `compute_hash()` returns identical values

### TC-13.4.3.2 Variable Migration Compatible

| # | Requirement |
|---|-------------|
| 1 | R-13.4.3    |

1. **#1** — Old layout [a:f32, b:bool], new [a:f32, b:bool, c:i32]
   - **Expected:** `a` and `b` preserved, `c` gets default

### TC-13.4.3.3 Variable Migration Incompatible

| # | Requirement |
|---|-------------|
| 1 | R-13.4.3    |

1. **#1** — Old layout [a:f32], new [a:bool] (type changed)
   - **Expected:** `ReloadResult::Incompatible`, instance reset

### TC-15.8.4.1 Coroutine Yield Next Frame

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |

1. **#1** — `YieldNextFrame` at frame 10
   - **Expected:** Suspends at frame 10, resumes at frame 11

### TC-15.8.4.2 Coroutine Yield Frames

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |

1. **#1** — `YieldFrames(3)` at frame 20
   - **Expected:** Resumes at frame 23

### TC-15.8.4.3 Coroutine Yield Delay

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |

1. **#1** — `YieldDelay(1.0)` at t=5.0
   - **Expected:** Resumes when accumulated dt >= 1.0

### TC-15.8.4.4 Coroutine Yield Event

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |

1. **#1** — `YieldUntilEvent(DamageEvent)`
   - **Expected:** Suspends until `DamageEvent` fires, then resumes

### TC-13.4.1.10 Sandbox Budget Exhaustion

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — Tight loop, budget=100 opcodes
   - **Expected:** `StepResult::Error(BudgetExhausted { limit: 100 })`

### TC-13.4.1.11 Sandbox Unauthorized Component

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — Graph accesses disallowed component at compile
   - **Expected:** `Err(Sandbox(UnauthorizedAccess))`

### TC-13.4.1.12 Sandbox Unbounded Loop

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — Loop node without yield point
   - **Expected:** `Err(Sandbox(UnboundedLoop { node_id }))`

### TC-15.8.12.1 Compiler Dead Node Elimination

| # | Requirement |
|---|-------------|
| 1 | R-15.8.12   |

1. **#1** — Graph with 5 reachable + 3 unreachable nodes
   - **Expected:** Compiled bytecode has 0 opcodes from unreachable nodes

### TC-15.8.12.2 Compiler Constant Folding

| # | Requirement |
|---|-------------|
| 1 | R-15.8.12   |

1. **#1** — Node: `Add(Const(3.0), Const(4.0))`
   - **Expected:** Compiled as `LoadConst(7.0)`, no Add opcode

### TC-15.8.12.3 Compiler Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-15.8.12   |

1. **#1** — Connect f32 output to bool input
   - **Expected:** `Err(TypeMismatch { expected: bool, actual: f32 })`

### TC-15.8.12.4 Compiler Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-15.8.12   |

1. **#1** — Dataflow cycle in graph
   - **Expected:** `Err(DataflowCycle { nodes })`

### TC-13.4.2.1 Debug Breakpoint Hit

| # | Requirement |
|---|-------------|
| 1 | R-13.4.2    |

1. **#1** — Set breakpoint on node 5, execute to node 5
   - **Expected:** `StepResult::Breakpoint(5)`, state = Paused

### TC-13.4.2.2 Debug Step

| # | Requirement |
|---|-------------|
| 1 | R-13.4.2    |

1. **#1** — Single step from paused state
   - **Expected:** Exactly 1 opcode executed, PC advances by 1

### TC-13.4.2.3 Debug Watch Read

| # | Requirement |
|---|-------------|
| 1 | R-13.4.2    |

1. **#1** — Set watch on slot 3, pause at breakpoint
   - **Expected:** `read_watch(instance, slot_3)` returns current value

### TC-13.4.NF1.1 Profiler Instruction Count

| # | Requirement |
|---|-------------|
| 1 | R-13.4.NF1  |

1. **#1** — Execute 50-opcode graph with profiler enabled
   - **Expected:** `GraphProfile.instructions_executed == 50`

### TC-13.4.NF1.2 Profiler Execution Time

| # | Requirement |
|---|-------------|
| 1 | R-13.4.NF1  |

1. **#1** — Execute graph with profiler enabled
   - **Expected:** `execution_time_us > 0`

## Integration Tests

### TC-13.4.1.I1 Graph ECS Component Read Write

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — Graph reads Health, subtracts 20, writes Health
   - **Expected:** After command flush, Health = original - 20

### TC-13.4.1.I2 Graph Event Emission

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — Graph emits `DamageDealt` event
   - **Expected:** Listener system receives it next sync point

### TC-13.4.1.I3 Graph AI Blackboard

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — Graph writes `patrol_target = pos(10,0,5)` to blackboard
   - **Expected:** Blackboard key matches written value

### TC-13.4.1.I4 Graph State Machine Transition

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — Graph triggers `Patrol -> Chase` transition
   - **Expected:** AI state machine state == Chase

### TC-13.4.1.I5 Graph ECS Schedule Order

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — Graph A writes val=1, Graph B reads; A ordered before B
   - **Expected:** B reads val=1

### TC-15.8.4.I1 Graph Coroutine Boss Encounter

| # | Requirement |
|---|-------------|
| 1 | R-15.8.4    |

1. **#1** — 3-phase boss: phase 1 (5 frames), phase 2 (3 frames), phase 3
   - **Expected:** Phase transitions at exact frame boundaries

### TC-13.4.3.I1 Hot Reload Compatible

| # | Requirement |
|---|-------------|
| 1 | R-13.4.3    |

1. **#1** — Add node to graph, keep variables
   - **Expected:** Hot reload preserves variable values

### TC-13.4.3.I2 Hot Reload Incompatible

| # | Requirement |
|---|-------------|
| 1 | R-13.4.3    |

1. **#1** — Remove a variable from graph
   - **Expected:** Instance reset with warning event

### TC-13.4.3.I3 Hot Reload Concurrent 100

| # | Requirement |
|---|-------------|
| 1 | R-13.4.3    |

1. **#1** — Hot reload while 100 instances execute
   - **Expected:** No crashes or corruption

### TC-13.4.NF2.I1 Hot Reload Turnaround

| # | Requirement |
|---|-------------|
| 1 | R-13.4.NF2  |

1. **#1** — Modify 50-node graph, trigger reload
   - **Expected:** Reload completes in < 1 second

### TC-13.4.2.I1 Debug Remote Connect

| # | Requirement |
|---|-------------|
| 1 | R-13.4.2    |

1. **#1** — Connect remote debugger over TCP, set breakpoint
   - **Expected:** Instance pauses at breakpoint

### TC-13.4.2.I2 Debug No Side Effects

| # | Requirement |
|---|-------------|
| 1 | R-13.4.2    |

1. **#1** — Run identical sessions with and without debugger
   - **Expected:** World state identical

### TC-13.4.2.I3 Debug Remote Stability

| # | Requirement |
|---|-------------|
| 1 | R-13.4.2    |

1. **#1** — Connect/disconnect 50 times during gameplay
   - **Expected:** No crashes

### TC-13.4.1.I6 No Text Scripting API

| # | Requirement |
|---|-------------|
| 1 | R-13.4.1    |

1. **#1** — Scan public API for text script/code string parameters
   - **Expected:** Zero matches

## Benchmarks

### TC-13.4.NF1.B1 1000 Graphs 10 Nodes

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 active graphs, 10 nodes each | Total frame time | < 4 ms at 60 fps | R-13.4.NF1 |

### TC-13.4.NF1.B2 Single Graph 100 Nodes

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1 graph, 100 nodes | Execution time | < 50 us | R-13.4.NF1 |

### TC-15.8.12.B1 Opcode Dispatch Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Pure opcode dispatch loop | Per-opcode time | < 10 ns | R-15.8.12 |

### TC-13.4.NF2.B1 Hot Reload Turnaround

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50-node graph recompile + reload | Turnaround time | < 1 second | R-13.4.NF2 |

### TC-13.4.1.B1 Variable Store Access

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | get/set on VariableStore | Per-access time | < 5 ns | R-13.4.1 |

### TC-15.8.4.B1 Coroutine Suspend Resume

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Yield and resume cycle | Per-cycle time | < 100 ns | R-15.8.4 |

### TC-15.8.12.B2 Compiler Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Compile 50-node graph | Compilation time | < 100 ms | R-15.8.12 |

### TC-13.4.NF1.B3 GraphProgram Memory

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50-node compiled graph | Shared memory | < 4 KiB | R-13.4.NF1 |

### TC-13.4.NF1.B4 GraphInstance Memory

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Instance with 64 variable slots | Per-instance memory | < 1.5 KiB | R-13.4.NF1 |
