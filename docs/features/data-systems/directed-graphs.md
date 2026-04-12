# 16.4 — Directed Graphs

## Core Topology

| ID       | Feature                          |
|----------|----------------------------------|
| F-16.4.1 | Generic DirectedGraph Primitive |
| F-16.4.2 | Cycle Detection                 |
| F-16.4.3 | Topological Sort                |

1. **F-16.4.1** — `DirectedGraph<N, E>` is a generic graph primitive with typed nodes and edges,
   adjacency list storage, and generational node handles. Quest graphs, dialogue trees, talent
   trees, and ability compositions all compose on top of this primitive.
   - **Deps:** F-1.7.4 (Handle), F-1.7.5 (HandleMap)
2. **F-16.4.2** — Depth-first cycle detection rejects cycle-introducing edges at construction when
   the graph is declared acyclic. Catches authoring errors before runtime.
   - **Deps:** F-16.4.1
3. **F-16.4.3** — Topological sort runs in O(V + E) returning a deterministic node order respecting
   edge dependencies. Used for prerequisite ordering and dataflow compilation.
   - **Deps:** F-16.4.1

## Conditional and Ordered Variants

| ID       | Feature                           |
|----------|-----------------------------------|
| F-16.4.4 | ConditionalGraph with Guards     |
| F-16.4.5 | OrderedGraph with Sibling Order  |

1. **F-16.4.4** — `ConditionalGraph<N, E>` wraps `DirectedGraph` with `ConditionExpr` guards on
   every edge. Traversal evaluates conditions against a `ConditionContext` at runtime, skipping
   gated edges whose conditions fail.
   - **Deps:** F-16.4.1, F-16.1.11 (Condition Expressions)
2. **F-16.4.5** — `OrderedGraph<N, E>` wraps `DirectedGraph` with per-node child ordering for trees
   where sibling order matters. Dialogue choices and talent rows depend on stable sibling order.
   - **Deps:** F-16.4.1

## Queries

| ID        | Feature                           |
|-----------|-----------------------------------|
| F-16.4.6  | Weighted-Edge Shortest Path      |
| F-16.4.7  | BFS and DFS Traversal Strategies |
| F-16.4.8  | Tree Operations                  |
| F-16.4.9  | Dead Node and Transitive Reduction|
| F-16.4.10 | Multi-Graph Parallel Edges       |

1. **F-16.4.6** — Weighted edges support shortest-path (Dijkstra), total cost, and budget-limited
   reachability queries. Used for optimal talent path computation and cost-bounded tree search.
   - **Deps:** F-16.4.1
2. **F-16.4.7** — BFS, DFS pre-order, and DFS post-order traversals accept optional filter
   predicates on nodes and edges. Different operations pick the appropriate order.
   - **Deps:** F-16.4.1
3. **F-16.4.8** — Tree operations on ordered graphs include root, leaves, depth, subtree extraction,
   ancestor path, and lowest common ancestor (LCA).
   - **Deps:** F-16.4.5
4. **F-16.4.9** — Dead node elimination prunes nodes unreachable from live roots. Transitive
   reduction removes edges implied by transitivity. Both operations keep authored graphs bounded.
   - **Deps:** F-16.4.1
5. **F-16.4.10** — Multi-graph support allows multiple edges between the same node pair so parallel
   dialogue choices and alternative quest paths can be expressed without collapsing to a single
   edge.
   - **Deps:** F-16.4.1

## Runtime Integration

| ID        | Feature                           |
|-----------|-----------------------------------|
| F-16.4.11 | 3D Indicator Spawn on Transitions|

1. **F-16.4.11** — Graph traversal state transitions spawn and despawn 3D gameplay indicators (quest
   markers, waypoint beams, minimap icons) via the ECS event channel. Indicator lifecycle is
   authored alongside graph nodes.
   - **Deps:** F-16.4.1, F-1.5.1, F-13.11 (Selection Indicators)
