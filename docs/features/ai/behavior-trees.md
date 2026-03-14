# 7.3 — Behavior Trees

## Node Types

### F-7.3.1 Core Composite & Leaf Nodes

Provides the fundamental behavior tree node types: Sequence (runs children in order, fails on first
failure), Selector (runs children in order, succeeds on first success), Parallel (runs all children
concurrently with configurable success/failure policies), and Leaf (executes a single action or
condition check). These composites form the structural backbone of all NPC behavior.

- **Requirements:** R-7.3.1
- **Dependencies:** None
- **Platform notes:** None

### F-7.3.2 Decorator Nodes

Wraps a single child node with modifiers: Inverter (negates result), Repeater (loops N times or
until failure), Succeeder (always returns success), Rate Limiter (throttles tick frequency), and
Cooldown (blocks re-entry for a duration). Decorators enable reusable behavior patterns without
duplicating subtrees.

- **Requirements:** R-7.3.2
- **Dependencies:** F-7.3.1
- **Platform notes:** None

### F-7.3.3 Conditional Aborts

Allows composite nodes to re-evaluate conditions while a lower-priority branch is running. Supports
self-abort (re-checks own conditions), lower-priority abort (interrupts siblings), and both modes
combined. Critical for responsive raid-boss AI that must immediately react to phase transitions or
threat changes.

- **Requirements:** R-7.3.3
- **Dependencies:** F-7.3.1
- **Platform notes:** None

## Blackboard

### F-7.3.4 Blackboard System

A typed key-value store attached to each AI agent that serves as shared memory between behavior tree
nodes. Supports scoped keys (self, group, global) so squad members can share tactical data like
threat targets or rally points. Observers can register for change notifications to trigger
conditional aborts.

- **Requirements:** R-7.3.4
- **Dependencies:** None
- **Platform notes:** None

## Assets & Serialization

### F-7.3.5 Behavior Tree Assets & Serialization

Behavior trees are authored as data assets in a declarative format (RON or JSON) and loaded at
runtime. Supports hot-reload during development so designers can iterate on NPC behavior without
restarting the server. Trees reference node types by registered name, enabling project-specific
leaf nodes.

- **Requirements:** R-7.3.5
- **Dependencies:** F-7.3.1, F-7.3.4
- **Platform notes:** None

### F-7.3.6 Subtree References & Reuse

A node type that references another behavior tree asset by handle, expanding it inline at load time
or executing it as a nested scope. Enables modular authoring where common patterns (patrol, flee to
safety, call for help) are defined once and reused across many NPC archetypes.

- **Requirements:** R-7.3.6
- **Dependencies:** F-7.3.5
- **Platform notes:** None

## Debugging

### F-7.3.7 Runtime Debugging & Visualization

Provides a server-side trace log of node visits, results, and blackboard mutations per tick for a
selected agent. An optional editor overlay renders the tree structure with color-coded node states
(running, success, failure) and live blackboard contents to accelerate behavior authoring and
debugging.

- **Requirements:** R-7.3.7
- **Dependencies:** F-7.3.1, F-7.3.4
- **Platform notes:** Visualization overlay is editor-only; trace log is available in all builds
