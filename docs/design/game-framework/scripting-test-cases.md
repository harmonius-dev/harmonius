# Gameplay Scripting Test Cases

Companion test cases for [scripting.md](scripting.md).

## Unit Tests

### TC-13.4.1.1 Opcode Add F32

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `AddF32` opcode, slot A=3.0, slot B=4.0 | Destination slot = 7.0 | R-13.4.1 |

### TC-13.4.1.2 Opcode Jump If

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `JumpIf` with condition=true, target=PC+5 | PC advances to PC+5 | R-13.4.1 |
| 2 | `JumpIf` with condition=false | PC advances to next opcode | R-13.4.1 |

### TC-13.4.1.3 Opcode Get Component

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `GetComponent(Health)` on entity with Health(80.0) | Destination slot = 80.0 | R-13.4.1 |

### TC-13.4.1.4 Opcode Set Component

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `SetComponent(Health, 50.0)` | Command buffer contains deferred Health write | R-13.4.1 |

### TC-13.4.1.5 Opcode Emit Event

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `EmitEvent(DamageDealt { amount: 25 })` | Event appears in event channel | R-13.4.1 |

### TC-13.4.1.6 Opcode Query Begin Next

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `QueryBegin(Health)`, 3 entities with Health | `QueryNext` yields 3 results then terminates | R-13.4.1 |

### TC-13.4.1.7 Opcode Blackboard Get Set

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `BlackboardSet("target", entity_42)`, then `BlackboardGet("target")` | Returns entity_42 | R-13.4.1 |

### TC-13.4.1.8 Opcode State Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `StateTransition("Patrol", "Chase")` | State machine transitions to Chase | R-13.4.1 |

### TC-13.4.1.9 Variable Store Get Set

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set(slot_0, 42.0)`, `get(slot_0)` | Returns 42.0 | R-13.4.1 |
| 2 | `set(graph_var_1, true)`, `get(graph_var_1)` | Returns true | R-13.4.1 |

### TC-13.4.3.1 Variable Layout Hash

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two identical `VariableLayout` with same slots | `compute_hash()` returns identical values | R-13.4.3 |

### TC-13.4.3.2 Variable Migration Compatible

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Old layout [a:f32, b:bool], new [a:f32, b:bool, c:i32] | `a` and `b` preserved, `c` gets default | R-13.4.3 |

### TC-13.4.3.3 Variable Migration Incompatible

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Old layout [a:f32], new [a:bool] (type changed) | `ReloadResult::Incompatible`, instance reset | R-13.4.3 |

### TC-15.8.4.1 Coroutine Yield Next Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `YieldNextFrame` at frame 10 | Suspends at frame 10, resumes at frame 11 | R-15.8.4 |

### TC-15.8.4.2 Coroutine Yield Frames

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `YieldFrames(3)` at frame 20 | Resumes at frame 23 | R-15.8.4 |

### TC-15.8.4.3 Coroutine Yield Delay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `YieldDelay(1.0)` at t=5.0 | Resumes when accumulated dt >= 1.0 | R-15.8.4 |

### TC-15.8.4.4 Coroutine Yield Event

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `YieldUntilEvent(DamageEvent)` | Suspends until `DamageEvent` fires, then resumes | R-15.8.4 |

### TC-13.4.1.10 Sandbox Budget Exhaustion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tight loop, budget=100 opcodes | `StepResult::Error(BudgetExhausted { limit: 100 })` | R-13.4.1 |

### TC-13.4.1.11 Sandbox Unauthorized Component

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph accesses disallowed component at compile | `Err(Sandbox(UnauthorizedAccess))` | R-13.4.1 |

### TC-13.4.1.12 Sandbox Unbounded Loop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Loop node without yield point | `Err(Sandbox(UnboundedLoop { node_id }))` | R-13.4.1 |

### TC-15.8.12.1 Compiler Dead Node Elimination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with 5 reachable + 3 unreachable nodes | Compiled bytecode has 0 opcodes from unreachable nodes | R-15.8.12 |

### TC-15.8.12.2 Compiler Constant Folding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Node: `Add(Const(3.0), Const(4.0))` | Compiled as `LoadConst(7.0)`, no Add opcode | R-15.8.12 |

### TC-15.8.12.3 Compiler Type Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect f32 output to bool input | `Err(TypeMismatch { expected: bool, actual: f32 })` | R-15.8.12 |

### TC-15.8.12.4 Compiler Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dataflow cycle in graph | `Err(DataflowCycle { nodes })` | R-15.8.12 |

### TC-13.4.2.1 Debug Breakpoint Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set breakpoint on node 5, execute to node 5 | `StepResult::Breakpoint(5)`, state = Paused | R-13.4.2 |

### TC-13.4.2.2 Debug Step

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Single step from paused state | Exactly 1 opcode executed, PC advances by 1 | R-13.4.2 |

### TC-13.4.2.3 Debug Watch Read

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set watch on slot 3, pause at breakpoint | `read_watch(instance, slot_3)` returns current value | R-13.4.2 |

### TC-13.4.NF1.1 Profiler Instruction Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Execute 50-opcode graph with profiler enabled | `GraphProfile.instructions_executed == 50` | R-13.4.NF1 |

### TC-13.4.NF1.2 Profiler Execution Time

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Execute graph with profiler enabled | `execution_time_us > 0` | R-13.4.NF1 |

## Integration Tests

### TC-13.4.1.I1 Graph ECS Component Read Write

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph reads Health, subtracts 20, writes Health | After command flush, Health = original - 20 | R-13.4.1 |

### TC-13.4.1.I2 Graph Event Emission

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph emits `DamageDealt` event | Listener system receives it next sync point | R-13.4.1 |

### TC-13.4.1.I3 Graph AI Blackboard

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph writes `patrol_target = pos(10,0,5)` to blackboard | Blackboard key matches written value | R-13.4.1 |

### TC-13.4.1.I4 Graph State Machine Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph triggers `Patrol -> Chase` transition | AI state machine state == Chase | R-13.4.1 |

### TC-13.4.1.I5 Graph ECS Schedule Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph A writes val=1, Graph B reads; A ordered before B | B reads val=1 | R-13.4.1 |

### TC-15.8.4.I1 Graph Coroutine Boss Encounter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-phase boss: phase 1 (5 frames), phase 2 (3 frames), phase 3 | Phase transitions at exact frame boundaries | R-15.8.4 |

### TC-13.4.3.I1 Hot Reload Compatible

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add node to graph, keep variables | Hot reload preserves variable values | R-13.4.3 |

### TC-13.4.3.I2 Hot Reload Incompatible

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove a variable from graph | Instance reset with warning event | R-13.4.3 |

### TC-13.4.3.I3 Hot Reload Concurrent 100

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hot reload while 100 instances execute | No crashes or corruption | R-13.4.3 |

### TC-13.4.NF2.I1 Hot Reload Turnaround

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify 50-node graph, trigger reload | Reload completes in < 1 second | R-13.4.NF2 |

### TC-13.4.2.I1 Debug Remote Connect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect remote debugger over TCP, set breakpoint | Instance pauses at breakpoint | R-13.4.2 |

### TC-13.4.2.I2 Debug No Side Effects

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run identical sessions with and without debugger | World state identical | R-13.4.2 |

### TC-13.4.2.I3 Debug Remote Stability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect/disconnect 50 times during gameplay | No crashes | R-13.4.2 |

### TC-13.4.1.I6 No Text Scripting API

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scan public API for text script/code string parameters | Zero matches | R-13.4.1 |

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
