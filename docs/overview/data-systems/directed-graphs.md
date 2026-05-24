# Directed Graphs

Nodes and edges for quest paths, dialogue trees, talent progression, and ability compositions.

## What it covers

- Typed directed graphs with nodes and edges; cycle detection and topological sorting.
- Conditional edges that gate traversal based on boolean expressions evaluated at runtime.
- Ordered graphs preserving stable sibling order for branching paths (dialogue choices, talent
  rows).
- Weighted edge queries: shortest path, total cost, and budget-limited reachability.
- Multiple distinct edges between the same nodes for parallel paths.
- Dead-node elimination and transitive reduction to clean up redundant content.
- Tree operations: root, leaves, depth, subtree extraction, ancestor paths, lowest common
  ancestor.
- 3D visual bindings: traversal state spawns/despawns quest markers, waypoint beams, and
  minimap icons.

## Concepts

### Graph Primitives

Nodes and edges are strongly typed and stored in adjacency lists with generational handles. A
conditional graph wraps nodes and edges with boolean guard expressions; traversal evaluates guards
at runtime. An ordered graph tracks stable sibling order per node, ensuring dialogue choices and
talent trees present options deterministically.

### Traversal and Queries

BFS, DFS (pre/post-order), and shortest-path queries all accept optional filters (per node or per
edge). Weighted-edge queries compute optimal paths via Dijkstra. Reachability queries stop after a
budget is exhausted, useful for bounded best-effort search.

### Visual Integration

Quest graphs spawn in-world markers (waypoint beams, minimap icons) when nodes become relevant and
despawn when no longer needed. State transitions trigger these events through the ECS.

## How it fits

- See [data-tables](./data-tables.md) for row references embedded in graph nodes.
- See [attributes-and-effects](./attributes-and-effects.md) for condition expressions used to
  gate edges.
- See [containers-and-slots](./containers-and-slots.md) for quest reward expressions.
- Integrates with [game-framework](../game-framework/gameplay-features.md) for quests, dialogue,
  and ability composition.
