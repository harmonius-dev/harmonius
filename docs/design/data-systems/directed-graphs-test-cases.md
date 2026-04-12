# Directed Graphs — Test Cases

Companion to [directed-graphs.md](directed-graphs.md).

Test case IDs use `TC-16.4.Z.N` format. Every row links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                                  | Req       |
|---------------|---------------------------------------|-----------|
| TC-16.4.1.1   | `test_graph_construct_nodes_edges`    | R-16.4.1  |
| TC-16.4.1.2   | `test_graph_neighbors_outgoing`       | R-16.4.1  |
| TC-16.4.1.3   | `test_graph_handle_generation`        | R-16.4.1  |
| TC-16.4.1.4   | `test_graph_remove_node_invalidates`  | R-16.4.1  |
| TC-16.4.2.1   | `test_cycle_detect_self_loop`         | R-16.4.2  |
| TC-16.4.2.2   | `test_cycle_detect_two_node`          | R-16.4.2  |
| TC-16.4.2.3   | `test_cycle_detect_three_node`        | R-16.4.2  |
| TC-16.4.2.4   | `test_dag_acyclic_passes`             | R-16.4.2  |
| TC-16.4.3.1   | `test_topo_sort_linear_chain`         | R-16.4.3  |
| TC-16.4.3.2   | `test_topo_sort_diamond`              | R-16.4.3  |
| TC-16.4.3.3   | `test_topo_sort_deterministic`        | R-16.4.3  |
| TC-16.4.4.1   | `test_conditional_edge_skipped`       | R-16.4.4  |
| TC-16.4.4.2   | `test_conditional_edge_taken`         | R-16.4.4  |
| TC-16.4.5.1   | `test_ordered_insert_sequence`        | R-16.4.5  |
| TC-16.4.5.2   | `test_ordered_reorder_children`       | R-16.4.5  |
| TC-16.4.6.1   | `test_dijkstra_shortest_path`         | R-16.4.6  |
| TC-16.4.6.2   | `test_dijkstra_unreachable`           | R-16.4.6  |
| TC-16.4.6.3   | `test_budget_reachability`            | R-16.4.6  |
| TC-16.4.7.1   | `test_bfs_order`                      | R-16.4.7  |
| TC-16.4.7.2   | `test_dfs_preorder`                   | R-16.4.7  |
| TC-16.4.7.3   | `test_dfs_postorder`                  | R-16.4.7  |
| TC-16.4.7.4   | `test_traversal_with_filter`          | R-16.4.7  |
| TC-16.4.8.1   | `test_tree_root_and_leaves`           | R-16.4.8  |
| TC-16.4.8.2   | `test_tree_depth_and_subtree`         | R-16.4.8  |
| TC-16.4.8.3   | `test_tree_lca`                       | R-16.4.8  |
| TC-16.4.9.1   | `test_dead_node_elimination`          | R-16.4.9  |
| TC-16.4.9.2   | `test_transitive_reduction`           | R-16.4.9  |
| TC-16.4.10.1  | `test_multi_graph_parallel_edges`     | R-16.4.10 |

1. **TC-16.4.1.1** `test_graph_construct_nodes_edges` — Construct a graph with 10 nodes and 15
   directed edges. Assert reported counts.
   - Input: empty `DirectedGraph<NodeData, EdgeData>`; insert 10 nodes, 15 edges
   - Expected: `graph.node_count() == 10`, `graph.edge_count() == 15`

2. **TC-16.4.1.2** `test_graph_neighbors_outgoing` — Node A has outgoing edges to B, C, D. Assert
   `out_neighbors(A)` returns `{B, C, D}`.
   - Input: graph with nodes A, B, C, D; edges `A->B`, `A->C`, `A->D`
   - Expected: `out_neighbors(A).collect::<HashSet<_>>() == {B, C, D}`

3. **TC-16.4.1.3** `test_graph_handle_generation` — Insert node, remove it, insert another. Assert
   stale handle to first does not resolve to the second.
   - Input: insert A → handle h1; remove A; insert B → handle h2
   - Expected: `graph.get(h1) == None`; `graph.get(h2) == Some(&B)`; `h1 != h2` (generation differs)

4. **TC-16.4.1.4** `test_graph_remove_node_invalidates` — Remove a node referenced by edges. Assert
   all incident edges are removed and neighbor lists no longer contain the node.
   - Input: graph `A -> B -> C, A -> C`; remove B
   - Expected: `edge_count == 1` (just `A -> C`); `out_neighbors(A) == {C}`; `B` not present

5. **TC-16.4.2.1** `test_cycle_detect_self_loop` — Add a self-edge `A -> A` to a DAG. Assert cycle
   detection rejects.
   - Input: DAG with one node A; attempt `add_edge(A, A)`
   - Expected: `Err(GraphError::CycleDetected { path: [A] })`; edge not added

6. **TC-16.4.2.2** `test_cycle_detect_two_node` — DAG with edge `A -> B`; attempt to add `B -> A`.
   Assert cycle detection rejects with the offending path.
   - Input: edges `[A -> B]`; attempt `add_edge(B, A)`
   - Expected: `Err(GraphError::CycleDetected { path: [A, B] })`; edge not added

7. **TC-16.4.2.3** `test_cycle_detect_three_node` — DAG `A -> B -> C`; attempt to add `C -> A`.
   Assert cycle detected.
   - Input: edges `[A -> B, B -> C]`; attempt `add_edge(C, A)`
   - Expected: `Err(GraphError::CycleDetected { path: [A, B, C] })`

8. **TC-16.4.2.4** `test_dag_acyclic_passes` — Build a 5-node DAG with 7 edges, no cycles. Assert
   `is_acyclic` returns true and no insertion fails.
   - Input: nodes `[A..E]`, edges `[A->B, A->C, B->D, C->D, B->E, D->E, C->E]`
   - Expected: `graph.is_acyclic() == true`; all 7 inserts return `Ok`

9. **TC-16.4.3.1** `test_topo_sort_linear_chain` — Chain `A -> B -> C -> D`. Assert `toposort`
   returns `[A, B, C, D]`.
   - Input: edges `[A->B, B->C, C->D]`
   - Expected: `toposort() == vec![A, B, C, D]`

10. **TC-16.4.3.2** `test_topo_sort_diamond` — Diamond `A -> {B, C} -> D`. Assert result is a valid
    linear extension.
    - Input: edges `[A->B, A->C, B->D, C->D]`
    - Expected: result starts with A, ends with D; B and C both appear after A and before D

11. **TC-16.4.3.3** `test_topo_sort_deterministic` — Build the same diamond from TC-16.4.3.2 in
    multiple insertion orders. Assert each `toposort` call returns the same sequence.
    - Input: same edges inserted in 3 different orders, all using node id ordering as tiebreak
    - Expected: all three calls return identical `Vec<NodeHandle>` (insertion-independent)

12. **TC-16.4.4.1** `test_conditional_edge_skipped` — Edge guarded by `Leaf(HasFlag("key"))`;
    traverse with context lacking the flag. Assert edge skipped.
    - Input: `ConditionalGraph` with `A -> B` guarded by `HasFlag("key")`;
      `ConditionContext { flags: {} }`
    - Expected: `traverse_from(A, &ctx)` does not visit B; `successors(A, &ctx).count() == 0`

13. **TC-16.4.4.2** `test_conditional_edge_taken` — Same edge with the flag set in context. Assert
    edge taken and B is visited.
    - Input: same graph; `ConditionContext { flags: {"key": true} }`
    - Expected: `successors(A, &ctx).collect() == [B]`; `traverse_from(A, &ctx)` visits B

14. **TC-16.4.5.1** `test_ordered_insert_sequence` — Insert children A, B, C under a parent in
    order. Assert iteration returns `[A, B, C]`.
    - Input: `OrderedGraph` with parent P; `add_child(P, A)`, then B, then C
    - Expected: `children(P).collect() == [A, B, C]`

15. **TC-16.4.5.2** `test_ordered_reorder_children` — After inserting A, B, C, reorder to
    `[C, A, B]`. Assert iteration matches the new order.
    - Input: insert A, B, C; `reorder_children(P, [C, A, B])`
    - Expected: `children(P).collect() == [C, A, B]`

16. **TC-16.4.6.1** `test_dijkstra_shortest_path` — Weighted graph with two paths from A to D.
    Assert returned path is the lower-cost one.
    - Input: edges `[(A, B, 1), (B, D, 5), (A, C, 2), (C, D, 1)]`; `shortest_path(A, D)`
    - Expected: `Some(Path { nodes: [A, C, D], cost: 3.0 })`

17. **TC-16.4.6.2** `test_dijkstra_unreachable` — Two disconnected components. Assert no path found.
    - Input: nodes `[A, B, C, D]`; edges only `[A -> B, C -> D]`; query `shortest_path(A, D)`
    - Expected: `None`

18. **TC-16.4.6.3** `test_budget_reachability` — Weighted graph; query nodes reachable from A within
    cost budget 5.0.
    - Input: edges `[(A, B, 2), (B, C, 2), (C, D, 2), (A, E, 6)]`; `reachable_within(A, 5.0)`
    - Expected: result set `{A, B, C}` (cost 0, 2, 4); D and E excluded

19. **TC-16.4.7.1** `test_bfs_order` — Graph rooted at A with two layers. Assert BFS visits root,
    then all level-1, then all level-2 nodes.
    - Input: edges `[A->B, A->C, B->D, C->E]`; `bfs(A)`
    - Expected: visit order `[A, B, C, D, E]` (level-by-level)

20. **TC-16.4.7.2** `test_dfs_preorder` — Same graph as TC-16.4.7.1; DFS pre-order visits parent
    before children.
    - Input: same graph; `dfs_preorder(A)` with deterministic neighbor order
    - Expected: visit order `[A, B, D, C, E]`

21. **TC-16.4.7.3** `test_dfs_postorder` — Same graph; DFS post-order visits children before parent.
    - Input: same graph; `dfs_postorder(A)` with deterministic neighbor order
    - Expected: visit order `[D, B, E, C, A]`

22. **TC-16.4.7.4** `test_traversal_with_filter` — BFS with filter excluding node C. Assert C and
    its descendants are not visited.
    - Input: same graph; `bfs_filtered(A, |n| n != C)`
    - Expected: visit order `[A, B, D]`; C and E omitted

23. **TC-16.4.8.1** `test_tree_root_and_leaves` — Tree with root R and leaves `[L1, L2, L3]`. Assert
    `root` and `leaves` return expected handles.
    - Input: `OrderedGraph` tree `R -> {N1 -> L1, N1 -> L2, N2 -> L3}`
    - Expected: `tree.root() == R`; `tree.leaves().collect::<HashSet<_>>() == {L1, L2, L3}`

24. **TC-16.4.8.2** `test_tree_depth_and_subtree` — Tree with depth 3; query depth of leaf and
    extract subtree at intermediate node.
    - Input: tree `R -> N1 -> N2 -> L`; `depth(L)` and `subtree(N1)`
    - Expected: `depth(L) == 3`; `subtree(N1).nodes() == {N1, N2, L}`

25. **TC-16.4.8.3** `test_tree_lca` — Tree `R -> {A -> {X, Y}, B -> Z}`; assert `lca(X, Y) == A` and
    `lca(X, Z) == R`.
    - Input: tree as above
    - Expected: `lca(X, Y) == Some(A)`; `lca(X, Z) == Some(R)`; `lca(X, X) == Some(X)`

26. **TC-16.4.9.1** `test_dead_node_elimination` — Graph with node D unreachable from root R. Run
    dead node elimination. Assert D removed and reachable nodes preserved.
    - Input: nodes `[R, A, B, D]`; edges `[R->A, R->B]`; live roots `[R]`
    - Expected: after `prune_unreachable([R])`, `node_count == 3`; D not present; A and B remain

27. **TC-16.4.9.2** `test_transitive_reduction` — Graph with `A -> B`, `B -> C`, and redundant
    `A -> C`. Run transitive reduction. Assert `A -> C` removed.
    - Input: edges `[A->B, B->C, A->C]`
    - Expected: after reduction, edges `{A->B, B->C}`; reachability unchanged (A still reaches C)

28. **TC-16.4.10.1** `test_multi_graph_parallel_edges` — Insert two distinct edges between the same
    node pair. Assert both addressable with distinct handles.
    - Input: edges `add_edge(A, B, EdgeData::Choice("yes"))`,
      `add_edge(A, B, EdgeData::Choice("no"))`
    - Expected: `out_edges(A).count() == 2`; both edges have distinct `EdgeHandle` values; payloads
      preserved

## Integration Tests

| ID            | Name                              | Req        |
|---------------|-----------------------------------|------------|
| TC-16.4.I.1   | `test_quest_graph_traversal`      | R-16.4.4   |
| TC-16.4.I.2   | `test_dialogue_branch_choices`    | R-16.4.10  |
| TC-16.4.I.3   | `test_talent_path_optimization`   | R-16.4.6   |
| TC-16.4.I.4   | `test_indicator_spawn_on_active`  | R-16.4.11  |
| TC-16.4.I.5   | `test_indicator_despawn_on_leave` | R-16.4.11  |
| TC-16.4.I.6   | `test_graph_hot_reload`           | R-16.4.1   |
| TC-16.4.I.7   | `test_graph_serialization`        | R-16.4.1   |

1. **TC-16.4.I.1** `test_quest_graph_traversal` — Build a `ConditionalGraph` quest with branching
   gates on player level. Walk from start to end with two contexts (level too low, level high
   enough). Assert different terminal nodes are reached.
   - Input: quest `Start -> {[Level>=10] BranchA, [else] BranchB} -> End`; contexts level=5 and
     level=15
   - Expected: low-level traversal visits `BranchB`; high-level traversal visits `BranchA`; both
     reach `End`

2. **TC-16.4.I.2** `test_dialogue_branch_choices` — Multi-graph dialogue node with three parallel
   choice edges leading to the same response. Assert all three options enumerable for UI.
   - Input: `OrderedGraph` with `Q -> A` three times with edge payloads `["yes", "maybe", "no"]`
   - Expected: `choices(Q).collect() == [("yes", A), ("maybe", A), ("no", A)]`; all three render

3. **TC-16.4.I.3** `test_talent_path_optimization` — Weighted talent tree with point costs; compute
   the cheapest path from root to a target talent.
   - Input: tree with 20 nodes and varying edge costs; target T at depth 5
   - Expected: returned path is the unique lowest-cost route to T; matches hand-computed result

4. **TC-16.4.I.4** `test_indicator_spawn_on_active` — Quest graph node configured with marker
   prefab. Traverse to make node active. Assert one indicator entity spawned and parented under the
   node's world location.
   - Input: graph node `N { marker: Some(handle) }`; transition `Inactive -> Active`
   - Expected: `IndicatorSpawned` event fired; `query::<QuestMarker>().count() == 1`; transform
     matches node origin

5. **TC-16.4.I.5** `test_indicator_despawn_on_leave` — Continuation of TC-16.4.I.4: transition the
   node out of `Active`. Assert indicator entity despawned.
   - Input: state transition `Active -> Completed` for the same node
   - Expected: `IndicatorDespawned` event fired; `query::<QuestMarker>().count() == 0`

6. **TC-16.4.I.6** `test_graph_hot_reload` — Modify a graph asset file (add a node, remove an edge);
   reload. Assert in-memory graph updates and live traversal state remains valid (no dangling
   handles).
   - Input: graph v1 `[A, B, C, A->B, B->C]`; reload as v2 `[A, B, C, D, A->B, B->D]`
   - Expected: `node_count == 4`; `out_neighbors(B) == {D}`; live traversal at A unaffected

7. **TC-16.4.I.7** `test_graph_serialization` — Save a `DirectedGraph<N, E>` via rkyv and reload it.
   Assert all nodes, edges, payloads, and handle generations round-trip.
   - Input: graph with 100 nodes and 250 edges with non-trivial payloads
   - Expected: deserialized graph equal to original (same node payloads, same edge payloads, same
     adjacency); `node_count`/`edge_count` match

## Benchmarks

| ID            | Benchmark                          | Target    | Req       |
|---------------|------------------------------------|-----------|-----------|
| TC-16.4.B.1   | Topological sort (10k nodes)       | < 1 ms    | R-16.4.3  |
| TC-16.4.B.2   | Cycle detect (10k nodes)           | < 1 ms    | R-16.4.2  |
| TC-16.4.B.3   | Dijkstra shortest path (10k nodes) | < 5 ms    | R-16.4.6  |
| TC-16.4.B.4   | BFS traversal (10k nodes)          | < 500 µs  | R-16.4.7  |
| TC-16.4.B.5   | Conditional graph eval (1k edges)  | < 100 µs  | R-16.4.4  |

1. **TC-16.4.B.1** — Topological sort over a synthetic DAG with 10,000 nodes and ~30,000 edges. Wall
   time for one `toposort` call. Measured with `criterion`. Must remain O(V + E).

2. **TC-16.4.B.2** — Full cycle-detection scan over a 10,000-node DAG. Wall time for one
   `is_acyclic` call across the entire graph.

3. **TC-16.4.B.3** — Dijkstra shortest path between two nodes in a 10,000-node weighted graph
   averaging 3 outgoing edges per node. Wall time for one query.

4. **TC-16.4.B.4** — BFS from one root over a 10,000-node graph. Wall time for full traversal
   collecting visited node handles into a `Vec`.

5. **TC-16.4.B.5** — Evaluate condition guards across 1,000 edges of a `ConditionalGraph` against a
   fixed `ConditionContext`. Wall time for one full `successors`-style sweep.
