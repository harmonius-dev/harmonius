# R-7.3 -- Behavior Trees User Stories

## US-7.3.1 Core Composite & Leaf Nodes

### US-7.3.1.1
As a **designer (P-5)**, I want Sequence, Selector, Parallel, and Leaf node types
so that I can structure NPC behavior as composable trees.

### US-7.3.1.2
As a **player (P-23)**, I want NPCs to exhibit varied behaviors
so that encounters feel dynamic rather than scripted.

### US-7.3.1.3
As an **engine dev (P-26)**, I want Parallel with configurable success/failure policies
so that concurrent behaviors complete based on tunable rules.

### US-7.3.1.4
As an **engine tester (P-27)**, I want to verify Sequence fails on first child failure
so that composite node semantics are regression-tested.

---

## US-7.3.2 Decorator Nodes

### US-7.3.2.1
As a **designer (P-5)**, I want Inverter, Repeater, Succeeder, Rate Limiter, and Cooldown
decorators
so that I can modify child node behavior without duplicating subtrees.

### US-7.3.2.2
As a **designer (P-5)**, I want Rate Limiter to throttle expensive subtrees
so that costly behaviors run at reduced frequency.

### US-7.3.2.3
As an **engine tester (P-27)**, I want to verify Cooldown blocks re-entry for the
configured duration
so that decorator timing is regression-tested.

---

## US-7.3.3 Conditional Aborts

### US-7.3.3.1
As a **designer (P-5)**, I want conditional aborts that interrupt lower-priority branches
so that AI reacts immediately to phase transitions or threats.

### US-7.3.3.2
As a **player (P-23)**, I want boss NPCs to react instantly to phase changes
so that encounters feel responsive and challenging.

### US-7.3.3.3
As an **engine tester (P-27)**, I want to verify lower-priority abort fires within the
same tick when conditions change
so that abort responsiveness is regression-tested.

---

## US-7.3.4 Blackboard System

### US-7.3.4.1
As a **designer (P-5)**, I want a typed key-value store per agent for shared memory
so that behavior tree nodes can pass data without tight coupling.

### US-7.3.4.2
As a **designer (P-5)**, I want scoped keys (self, group, global)
so that squad members can share tactical data like rally points.

### US-7.3.4.3
As an **engine dev (P-26)**, I want change-notification observers on blackboard keys
so that conditional aborts trigger on value changes.

### US-7.3.4.4
As an **engine tester (P-27)**, I want to verify self-scoped keys are invisible to other
agents
so that scope isolation is regression-tested.

---

## US-7.3.5 Behavior Tree Assets & Serialization

### US-7.3.5.1
As a **designer (P-5)**, I want behavior trees authored as data assets (RON or JSON)
so that I can edit NPC behavior without code changes.

### US-7.3.5.2
As a **designer (P-5)**, I want hot-reload during development
so that I can iterate on behavior without restarting the server.

### US-7.3.5.3
As an **engine tester (P-27)**, I want to verify hot-reload applies updated behavior
within 1 second
so that hot-reload latency is regression-tested.

---

## US-7.3.6 Subtree References & Reuse

### US-7.3.6.1
As a **designer (P-5)**, I want to reference shared subtrees (patrol, flee) by handle
so that common patterns are defined once and reused.

### US-7.3.6.2
As a **designer (P-5)**, I want shared subtree updates to propagate to all referencing
trees
so that fixing a behavior fixes it everywhere.

### US-7.3.6.3
As an **engine tester (P-27)**, I want to verify circular subtree references are detected
at load time
so that reference cycle detection is regression-tested.

---

## US-7.3.7 Runtime Debugging & Visualization

### US-7.3.7.1
As a **designer (P-5)**, I want a trace log of node visits and results per tick
so that I can diagnose incorrect NPC behavior.

### US-7.3.7.2
As a **designer (P-5)**, I want an editor overlay with color-coded node states
so that I can see active behavior visually during testing.

### US-7.3.7.3
As a **designer (P-5)**, I want live blackboard contents in the debug overlay
so that I can inspect data values driving behavior decisions.

### US-7.3.7.4
As an **engine tester (P-27)**, I want to verify the trace log records every node visit
with correct return status
so that trace log accuracy is regression-tested.
