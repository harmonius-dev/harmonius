# User Stories — 13.4 Gameplay Scripting Integration

## F-13.4.1 Gameplay Logic Graph Integration

## US-13.4.1.1 Access ECS World State From Logic Graphs

**As a** designer (P-5), **I want to** read and write ECS entity components, resources, and
queries from within a gameplay logic graph, **so that** I can implement gameplay mechanics
visually without code.

## US-13.4.1.2 Emit and Handle Gameplay Events in Graphs

**As a** designer (P-5), **I want to** emit and handle gameplay events (damage dealt, quest
completed, ability activated) from logic graph nodes, **so that** systems communicate through
events without custom wiring.

## US-13.4.1.3 Transition AI State Machines From Graphs

**As a** designer (P-5), **I want to** trigger AI blackboard reads/writes and state machine
transitions from gameplay logic graphs, **so that** AI behavior integrates with gameplay
logic seamlessly.

## US-13.4.1.4 Attach Logic Graphs to Entities as Components

**As a** developer (P-15), **I want to** attach gameplay logic graphs to entities as ECS
components that execute in the system schedule, **so that** per-entity behavior is data-driven
and runs within the standard frame pipeline.

## US-13.4.1.5 Verify Logic Graph Execution Matches ECS Schedule Order

**As an** engine tester (P-27), **I want to** attach graphs to multiple entities and verify they
execute in the declared ECS system schedule order, **so that** graph execution respects system
dependencies.

## US-13.4.1.6 Verify No Text Scripting Fallback Exists

**As an** engine tester (P-27), **I want to** confirm that no text-based scripting language is
available or required for any gameplay feature, **so that** the no-code visual-only principle
is enforced.

## F-13.4.2 Logic Graph Debugging for Gameplay

## US-13.4.2.1 Inspect ECS Query Results in Graph Debugger

**As a** designer (P-5), **I want to** inspect ECS query results at any graph node during
paused execution, **so that** I can see which entities matched a query and debug unexpected
behavior.

## US-13.4.2.2 Set Breakpoints That Pause Game Simulation

**As a** designer (P-5), **I want to** set breakpoints on gameplay logic graph nodes that pause
game simulation while allowing editor interaction, **so that** I can step through logic
one node at a time.

## US-13.4.2.3 Visualize Event Flow Between Graph Executions

**As a** developer (P-15), **I want to** see which events triggered which graph executions as a
visual flow, **so that** I can trace event-driven behavior chains across entities.

## US-13.4.2.4 Debug Graphs Remotely on a Running Game Instance

**As a** developer (P-15), **I want to** connect the graph debugger to a running game instance
over the network, **so that** I can debug gameplay logic on a live server or remote device.

## US-13.4.2.5 Verify Debugger Does Not Alter Game State

**As an** engine tester (P-27), **I want to** run a game session with and without the debugger
attached and verify identical game state at matching frames, **so that** the debugger has no
side effects on gameplay.

## US-13.4.2.6 Verify Remote Debug Connection Stability

**As an** engine tester (P-27), **I want to** connect and disconnect the remote debugger 50
times during gameplay and verify no crashes or state corruption, **so that** debug sessions
are robust.

## F-13.4.3 Logic Graph Hot Reload

## US-13.4.3.1 Apply Graph Changes Without Restarting

**As a** designer (P-5), **I want to** modify a gameplay logic graph in the editor while the
game is running and see changes applied immediately, **so that** I can iterate on gameplay
logic without restarting.

## US-13.4.3.2 Preserve Graph Instance State Across Reloads

**As a** designer (P-5), **I want to** local variables and coroutine positions to persist across
compatible hot reloads, **so that** I do not lose runtime state when tweaking logic.

## US-13.4.3.3 Detect and Recompile Changed Graphs Automatically

**As a** developer (P-15), **I want to** the hot reload system to detect graph asset changes via
file watcher and recompile through the shared build cache, **so that** reloads happen without
manual steps.

## US-13.4.3.4 Verify Incompatible Changes Trigger Clean Restart

**As an** engine tester (P-27), **I want to** make a breaking graph change (remove a variable)
and verify the affected graph instance restarts cleanly with a warning, **so that**
incompatible reloads are handled gracefully.

## US-13.4.3.5 Verify Hot Reload Under Concurrent Graph Execution

**As an** engine tester (P-27), **I want to** hot-reload a graph while 100 instances are
executing concurrently and verify no crashes or state corruption, **so that** reload is
thread-safe.
