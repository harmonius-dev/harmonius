# R-7.4 -- Utility AI Requirements

## Scoring and Selection

### R-7.4.1 Scoring Functions and Response Curves

The engine **SHALL** evaluate each action consideration by mapping an input value to a 0-1 score via
a configurable response curve (linear, quadratic, logistic, step, custom piecewise), defined as data
assets, with all curve types producing outputs clamped to [0.0, 1.0].

- **Derived from:**
  [F-7.4.1](../../features/ai/utility-ai.md)
- **Rationale:** Normalized scoring enables fair comparison across considerations with different
  input ranges; data- driven curves let designers tune NPC priorities without code changes.
- **Verification:** Evaluate each curve type (linear, quadratic, logistic, step, piecewise) with
  input values at 0.0, 0.5, and 1.0. Verify all outputs are clamped to [0.0, 1.0]. Verify a custom
  piecewise curve with 5 segments produces the expected output at each breakpoint.

### R-7.4.2 Action Selection and Compensation

The engine **SHALL** multiply consideration scores per action, apply a compensation factor based on
consideration count to avoid penalizing actions with more inputs, and select the winning action via
a configurable strategy (highest score, weighted random among top N, or threshold-based filtering).

- **Derived from:**
  [F-7.4.2](../../features/ai/utility-ai.md)
- **Rationale:** Without compensation, actions with more considerations are systematically penalized
  by score multiplication; configurable selection strategies enable varied NPC personality
  (deterministic vs. varied).
- **Verification:** Create two actions: one with 2 considerations averaging 0.8, another with 5
  considerations averaging 0.8. Verify the compensation factor adjusts their final scores to be
  comparable (within 10%). Verify weighted-random selection among top 3 produces non-deterministic
  results across 100 evaluations.

### R-7.4.3 Considerations and Input Axes

The engine **SHALL** provide built-in considerations (distance to target, line of sight, health
ratio, threat level, time since last action, resource availability) and support registering custom
considerations via a trait interface, each returning a valid 0-1 score.

- **Derived from:**
  [F-7.4.3](../../features/ai/utility-ai.md)
- **Rationale:** Built-in considerations cover common AI evaluation needs; the trait interface
  enables project- specific inputs without modifying engine code.
- **Verification:** Evaluate each built-in consideration and verify it returns a value in
  [0.0, 1.0]. Register a custom consideration via the trait interface and verify it is invoked
  during action evaluation. Verify a custom consideration returning a value outside [0.0, 1.0] is
  clamped.

## Advanced Reasoning

### R-7.4.4 Dual Utility System

The engine **SHALL** support two-axis scoring where one axis ranks action categories (combat,
social, survival) by priority and a second axis ranks specific actions within the chosen category,
ensuring critical categories are never outscored by high-scoring actions in non-urgent categories.

- **Derived from:**
  [F-7.4.4](../../features/ai/utility-ai.md)
- **Rationale:** Single-axis scoring allows a highly-scored socializing action to outrank a
  moderately-scored survival action, producing unrealistic behavior under threat.
- **Verification:** Configure survival category at priority 1.0 and social category at priority 0.3.
  Set a survival action score of 0.5 and a social action score of 0.9. Verify the survival action is
  selected because its category outranks social. Verify category ranking takes precedence over
  within-category scores.

### R-7.4.5 Context-Based Reasoning

The engine **SHALL** group actions into context sets (in-combat, exploring, fleeing, socializing)
and evaluate only the active context's action pool each tick, with hysteresis thresholds on context
transitions to prevent rapid switching.

- **Derived from:**
  [F-7.4.5](../../features/ai/utility-ai.md)
- **Rationale:** Evaluating all actions every tick is wasteful when most are irrelevant to the
  current situation; hysteresis prevents oscillation between contexts at boundary conditions.
- **Verification:** Activate the "combat" context and verify only combat actions are evaluated per
  tick. Set a hysteresis threshold of 0.2 and verify the context does not switch when the competing
  context's score is within the threshold. Verify the context switches when the competing score
  exceeds the threshold.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/ai/utility-ai.md](../../user-stories/ai/utility-ai.md). Requirements in this document
are derived from those user stories.
