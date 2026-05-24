# Decision-Making

Behavior trees, goal-oriented action planning, and utility-based AI decision structures.

## What it covers

- Behavior trees: hierarchical task trees with composite nodes (sequence, select, parallel).
- Tree traversal: evaluating conditions and executing actions.
- Blackboards: shared mutable state for decision context.
- Goal-oriented action planning (GOAP): state-based planning toward goals.
- Utility systems: scoring actions by weighted criteria.
- Decision contexts: character state, perceived threats, opportunities.
- Replanning: detecting state changes invalidating current plan.
- Interruption and priority: aborting lower-priority plans for urgent goals.
- Scripted sequences and cinematics: deterministic action sequences.
- Behavior variants: different AI personalities with different trees.

## Concepts

### Behavior Trees

Behavior trees model decision-making as a hierarchy: root node represents the character's overall
goal; children represent substeps. Composite nodes combine children: Sequence succeeds only if all
children succeed; Select succeeds if any child succeeds; Parallel runs all children concurrently.
Decorator nodes modify child behavior: Inverter flips success/failure; Repeat retries until success.
Action nodes execute behaviors (move, attack, play animation).

### Blackboards and Context

Blackboards store decision context: character position, health, nearby enemies, goal. Actions and
conditions read and write blackboards. This shared state enables communication between tree nodes
without tight coupling. Multiple trees can read the same blackboard, enabling coordination.

### Goal-Oriented Action Planning

GOAP systems define states (at target, enemy dead) and actions (move to target, shoot). Given
current state and a goal state, the planner searches for an action sequence reaching the goal.
Replanning activates when conditions change: if a closer enemy appears, replan toward the new
target. GOAP produces flexible behavior without hard-coded sequences.

### Utility Systems

Utility AI scores each candidate action by weighted criteria. Scoring factors include distance to
target, health danger, ammo, and opportunity. Actions score high when conditions favor them: chase
when enemy is nearby and weak; retreat when health is low. Re-scoring every frame enables smooth
transitions as circumstances change.

### Interruption and Hierarchies

Characters maintain current plans and goals. Urgent goals (taking damage) interrupt lower-priority
goals (exploring). Hierarchical planning enables parallel plans: high-priority plan (fight) runs
simultaneously with lower-priority plan (maintain formation). When interrupted, plans save state
for resumption.

## How it fits

- See [perception.md](./perception.md) for sensing and awareness context.
- See [navigation.md](./navigation.md) for pathfinding and movement execution.
- See [../core-runtime/authoring-runtime.md](../core-runtime/authoring-runtime.md) for graph
  systems and hot reload.
