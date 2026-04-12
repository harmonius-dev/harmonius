# Directed Graphs User Stories

## Core Topology

| ID        | Persona                 |
|-----------|-------------------------|
| US-16.4.1 | engine developer (P-26) |
| US-16.4.2 | QA lead (P-20)          |
| US-16.4.3 | engine developer (P-26) |

1. **US-16.4.1** — **As an** engine developer (P-26), **I want** a generic DirectedGraph primitive
   with typed nodes and edges plus adjacency list storage, **so that** quest graphs, dialogue trees,
   and talent trees all compose on one shared type.
2. **US-16.4.2** — **As a** QA lead (P-20), **I want** cycle detection to reject cycle-introducing
   edges in acyclic graphs at construction, **so that** authoring errors never lead to
   infinite-traversal crashes at runtime.
3. **US-16.4.3** — **As an** engine developer (P-26), **I want** O(V+E) topological sort on DAGs,
   **so that** prerequisite ordering and dataflow compilation have a fast engine-level primitive.

## Variants

| ID        | Persona                      |
|-----------|------------------------------|
| US-16.4.4 | story writer (P-17)          |
| US-16.4.5 | narrative designer (P-4)     |

1. **US-16.4.4** — **As a** story writer (P-17), **I want** ConditionalGraph edges guarded by
   boolean expressions over runtime state, **so that** dialogue and quest branches can depend on
   player level, flags, and inventory.
2. **US-16.4.5** — **As a** narrative designer (P-4), **I want** OrderedGraph to preserve sibling
   order for dialogue choices and talent rows, **so that** players see options in the order I
   authored them.

## Queries

| ID         | Persona                 |
|------------|-------------------------|
| US-16.4.6  | game designer (P-5)     |
| US-16.4.7  | engine developer (P-26) |
| US-16.4.8  | game developer (P-15)   |
| US-16.4.9  | QA lead (P-20)          |
| US-16.4.10 | narrative designer (P-4)|

1. **US-16.4.6** — **As a** game designer (P-5), **I want** weighted shortest path and
   budget-limited reachability queries on graphs, **so that** the engine can compute the cheapest
   path through a talent tree to a desired node.
2. **US-16.4.7** — **As an** engine developer (P-26), **I want** BFS and DFS (pre-order and
   post-order) traversal strategies with filter predicates, **so that** I can pick the appropriate
   order per use case.
3. **US-16.4.8** — **As a** game developer (P-15), **I want** tree operations (root, leaves, depth,
   subtree, ancestor path, LCA) on ordered graphs, **so that** I do not reimplement them per
   subsystem.
4. **US-16.4.9** — **As a** QA lead (P-20), **I want** dead node elimination and transitive
   reduction passes, **so that** authored graphs do not accumulate unreachable nodes and redundant
   edges.
5. **US-16.4.10** — **As a** narrative designer (P-4), **I want** multiple edges between the same
   node pair for alternative dialogue choices, **so that** I can offer parallel lines leading to the
   same outcome without collapsing them to one edge.

## Runtime Integration

| ID         | Persona      |
|------------|--------------|
| US-16.4.11 | gamer (P-23) |

1. **US-16.4.11** — **As a** gamer (P-23), **I want** quest markers to appear above NPCs when
   objectives become available and disappear when completed, **so that** I always know where to go
   next without checking a menu.
