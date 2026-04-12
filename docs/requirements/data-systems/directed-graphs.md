# R-16.4 — Directed Graphs Requirements

## Core Topology

1. **R-16.4.1** — The engine **SHALL** provide a generic `DirectedGraph<N, E>` primitive with typed
   nodes and edges, adjacency list storage, and generational node handles.
   - **Rationale:** Quest graphs, dialogue trees, talent trees, and ability compositions all share a
     graph topology; one generic primitive avoids duplicating graph code per feature.
   - **Verification:** Unit test: construct a graph with 10 nodes and 15 edges; enumerate neighbors;
     assert adjacency matches inputs.
2. **R-16.4.2** — The engine **SHALL** detect cycles in directed graphs via depth-first search and
   reject cycle-introducing edges at construction time when the graph is declared acyclic.
   - **Rationale:** Quest progression and talent trees must be acyclic; cycle detection prevents
     authoring errors that cause infinite traversal loops.
   - **Verification:** Unit test: build a DAG; attempt to add a back edge; assert rejection and
     error naming the offending cycle.
3. **R-16.4.3** — The engine **SHALL** compute topological sort of directed acyclic graphs in O(V +
   E) time, returning a deterministic node order respecting edge dependencies.
   - **Rationale:** Dependency-respecting orders are required for prerequisites, staged unlock, and
     dataflow compilation.
   - **Verification:** Unit test: build a known DAG; compute topological sort; assert result is a
     valid linear extension.

## Conditional and Ordered Variants

1. **R-16.4.4** — The engine **SHALL** provide a `ConditionalGraph<N, E>` variant that wraps
   `DirectedGraph` with `ConditionExpr` guards on every edge, where traversal queries evaluate
   conditions against a `ConditionContext` at runtime.
   - **Rationale:** Branching quests and dialogue trees gate edges by runtime state (player level,
     flags, inventory); condition evaluation must be first-class.
   - **Verification:** Unit test: build a graph with a guarded edge; traverse with context failing
     the guard; assert edge skipped. Satisfy guard; assert edge taken.
2. **R-16.4.5** — The engine **SHALL** provide an `OrderedGraph<N, E>` variant that wraps
   `DirectedGraph` with per-node child ordering for trees where sibling order matters.
   - **Rationale:** Dialogue choices, talent rows, and menu sequences depend on stable sibling order
     that plain adjacency lists do not preserve.
   - **Verification:** Unit test: insert children in order A, B, C; reorder to C, A, B; assert
     iteration returns the new order.

## Queries

1. **R-16.4.6** — The engine **SHALL** support weighted edges on directed graphs with shortest-path
   (Dijkstra), total cost, and budget-limited reachability queries.
   - **Rationale:** Talent path optimization, quest cost planning, and budget-bound tree search all
     require weighted-edge queries.
   - **Verification:** Unit test: build a weighted graph; query shortest path between two nodes;
     assert cost and node sequence match expected. Query budget-limited reachability; assert exactly
     the nodes within budget are returned.
2. **R-16.4.7** — The engine **SHALL** provide BFS, DFS pre-order, and DFS post-order traversal
   strategies on directed graphs with optional filter predicates on nodes and edges.
   - **Rationale:** Different operations need different traversal orders (BFS for nearest-reach,
     post-order DFS for dependency tear-down).
   - **Verification:** Unit test: build a fixed DAG; run BFS, DFS pre, DFS post; assert each returns
     the expected node sequence.
3. **R-16.4.8** — The engine **SHALL** provide tree operations on ordered graphs including root,
   leaves, depth, subtree extraction, ancestor path, and lowest common ancestor (LCA).
   - **Rationale:** Tree-shaped graphs (dialogue, talent trees) need these operations frequently;
     reimplementing them per subsystem is wasteful.
   - **Verification:** Unit test: construct a known tree; call each operation; assert results match
     hand-computed expectations.
4. **R-16.4.9** — The engine **SHALL** provide dead node elimination (prune nodes unreachable from
   live roots) and transitive reduction (remove edges implied by transitivity) on directed graphs.
   - **Rationale:** Authored graphs accumulate dead nodes and redundant edges; cleanup passes keep
     graph size and traversal cost bounded.
   - **Verification:** Unit test: construct a graph with one unreachable node; run dead node
     elimination; assert node removed. Run transitive reduction; assert redundant edges removed.
5. **R-16.4.10** — The engine **SHALL** support multiple edges between the same node pair
   (multi-graph) so that parallel dialogue choices, quest alternatives, and duplicate prerequisite
   paths can be expressed without collapsing to a single edge.
   - **Rationale:** Dialogue branches often present multiple lines that go to the same next node;
     each must be addressable as its own edge.
   - **Verification:** Unit test: insert two edges between nodes A and B with distinct payloads;
     iterate A's outgoing edges; assert both returned with distinct edge handles.

## Runtime Integration

1. **R-16.4.11** — The engine **SHALL** spawn and despawn 3D gameplay indicators (quest markers,
   waypoint beams, minimap icons) in response to graph traversal state transitions through the ECS
   event channel.
   - **Rationale:** Players need visual feedback when quest nodes become active; indicator lifecycle
     must be authored alongside the graph, not in custom per-game systems.
   - **Verification:** Integration test: configure a graph node with a marker; traverse to make it
     active; assert indicator entity spawned. Leave node; assert indicator despawned.
