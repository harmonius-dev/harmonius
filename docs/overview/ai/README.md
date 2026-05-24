# AI

How non-player characters perceive the world, decide what to do, move through space, and behave in
groups.

## Topics

- [decision-making](./decision-making.md) — how agents pick actions: behavior trees, utility
  scoring, and goal planning.
- [perception](./perception.md) — what agents can see, hear, and remember about the world.
- [navigation](./navigation.md) — how agents find paths and steer around obstacles.
- [crowds-and-tactics](./crowds-and-tactics.md) — group movement, formations, and squad combat.

## Key takeaways

- Behavior trees hierarchically decompose decisions into composite nodes (sequence, select, parallel)
  and action nodes, enabling reactive behavior without global state machines.
- Blackboards provide shared mutable decision context (nearby threats, goal) accessible to all tree
  nodes, enabling decoupled behavior branches.
- Goal-oriented action planning (GOAP) searches for action sequences reaching goal state,
  automatically replanning when conditions change, producing flexible behavior.
- Line-of-sight checks with memory and decay maintain character awareness without continuous
  visibility queries, balancing responsiveness vs CPU cost.
- Local steering (velocity steering around obstacles) combined with global pathfinding on navigation
  meshes produces natural movement with minimal overhead.

## Integration risks

- Behavior tree traversal order (evaluate conditions every frame) must account for action frame lag;
  premature condition re-evaluation causes action interruption. See [decision-making.md](./decision-making.md)
  for evaluation scheduling.
- Blackboard variable typing (ensuring correct types written/read) requires validation; type
  mismatches cause AI freezes or silent failures. See [decision-making.md](./decision-making.md)
  for type-safe blackboard patterns.
- GOAP planner state space explosion (too many actions or goal combinations) causes planning latency.
  See [decision-making.md](./decision-making.md) for state space reduction.
- Memory decay rates (forgetting enemies over time) must match game pacing; decay too fast, AI seems
  confused; decay too slow, enemies remember forever. See [perception.md](./perception.md) for
  tuning guidance.
- Navigation mesh inaccuracy (edges too narrow for characters, missing connections) causes pathfinding
  failure or characters stuck at boundaries. See [navigation.md](./navigation.md) for nav mesh
  validation.
