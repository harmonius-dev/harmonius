# R-7.4 -- Utility AI Requirements

## R-7.4.1 Scoring Functions and Response Curves

The engine **SHALL** evaluate each candidate action's considerations by mapping input values
(health percentage, distance, ammo count) to a 0-1 score via configurable response curves
(linear, quadratic, logistic, step, custom piecewise) defined as data assets.

- **Derived from:** [F-7.4.1](../../features/ai/utility-ai.md)
- **Rationale:** Data-driven response curves let designers tune NPC priorities without code
  changes, and the 0-1 normalization enables consistent cross-consideration comparison.
- **Verification:** Configure a logistic curve mapping health percentage to score. Verify that
  health=100% produces score >= 0.95, health=50% produces score ~0.5 (within 0.05), and
  health=0% produces score <= 0.05. Swap to a step curve with threshold at 30% and verify
  score is 0.0 above 30% and 1.0 at or below 30%. Verify all curve types produce outputs
  clamped to [0.0, 1.0].

## R-7.4.2 Action Selection and Compensation

The engine **SHALL** multiply consideration scores for each action, apply a compensation factor
based on the number of considerations to avoid penalizing actions with more inputs, and select
the winning action using a configurable strategy (highest score, weighted random among top N,
or threshold-based filtering).

- **Derived from:** [F-7.4.2](../../features/ai/utility-ai.md)
- **Rationale:** Raw score multiplication penalizes actions with many considerations;
  compensation ensures fair comparison between simple and complex actions.
- **Verification:** Create two actions: action A with 2 considerations (scores 0.8, 0.8 =
  raw 0.64) and action B with 5 considerations (scores 0.8 each = raw 0.328). Verify the
  compensation factor adjusts both to comparable final scores (within 10% of each other).
  Test weighted-random selection over 1000 trials and verify the top-3 actions are selected
  with frequency proportional to their scores.

## R-7.4.3 Considerations and Input Axes

The engine **SHALL** provide a standard library of reusable consideration types (distance to
target, line of sight, health ratio, threat level, time since last action, resource
availability) and allow registration of custom considerations through a trait interface.

- **Derived from:** [F-7.4.3](../../features/ai/utility-ai.md)
- **Rationale:** A shared library of considerations avoids reimplementing common evaluations
  per NPC type and ensures consistency across AI archetypes.
- **Verification:** Instantiate each built-in consideration type and verify it returns a valid
  0-1 score given known test inputs. Register a custom "magic_mana_ratio" consideration via
  the trait interface and verify it is evaluated alongside built-in considerations during
  action scoring. Verify that an unregistered consideration name produces a descriptive error
  at load time.

## R-7.4.4 Dual Utility System

The engine **SHALL** support a two-axis utility model where one axis ranks action categories
(combat, social, survival) and a second axis ranks specific actions within the chosen category,
preventing low-urgency actions in critical categories from being outscored by high-scoring
actions in less important categories.

- **Derived from:** [F-7.4.4](../../features/ai/utility-ai.md)
- **Rationale:** Single-axis scoring can cause survival-critical actions to lose to high-scoring
  but non-urgent social actions; dual-axis preserves category priority.
- **Verification:** Configure a survival category (score 0.9) containing "heal" (score 0.4)
  and a social category (score 0.3) containing "chat" (score 0.95). Verify the system selects
  "heal" because survival outranks social, even though "chat" has a higher within-category
  score. Swap category scores and verify the selection reverses.

## R-7.4.5 Context-Based Reasoning

The engine **SHALL** group actions into context sets (in-combat, exploring, fleeing, socializing)
and evaluate only the active context's action pool each tick, with context transitions governed
by hysteresis thresholds to prevent rapid switching.

- **Derived from:** [F-7.4.5](../../features/ai/utility-ai.md)
- **Rationale:** Evaluating all actions every tick is wasteful for NPCs with large repertoires;
  context grouping reduces per-tick cost and prevents erratic behavior switching.
- **Verification:** Configure an NPC with 20 actions across 4 contexts. Verify only the active
  context's actions (5) are evaluated per tick, not all 20. Set a hysteresis threshold of 0.1
  and verify that a context switch requires the new context's score to exceed the current
  context's score by at least 0.1. Oscillate the input signal near the threshold and confirm
  no context switch occurs for at least 10 consecutive ticks.
