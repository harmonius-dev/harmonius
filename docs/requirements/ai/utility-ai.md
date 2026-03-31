# R-7.4 -- Utility AI Requirements

## Scoring and Selection

1. **R-7.4.1** -- The engine **SHALL** evaluate candidate actions using configurable response curves
   (linear, quadratic, logistic, step, piecewise) that map input values to normalized 0-1 scores,
   defined as data assets.
   - **Rationale:** Data-driven curves let designers tune NPC priorities without code changes;
     normalized scores enable consistent blending across considerations.
   - **Verification:** Verify all curve types produce 0-1 output for any valid input. Test each
     curve type against known input-output pairs. Benchmark evaluation cost per agent across curve
     types.

2. **R-7.4.2** -- The engine **SHALL** select actions by multiplying consideration scores with a
   compensation factor for consideration count, supporting highest-score, weighted-random-top-N, and
   threshold-filter selection strategies.
   - **Rationale:** Compensation prevents actions with more considerations from scoring lower;
     multiple selection strategies enable varied AI personality.
   - **Verification:** Verify compensation corrects count-dependent bias. Verify weighted random
     selection produces a probability distribution matching score ratios. Benchmark selection time
     with maximum action pool size.

3. **R-7.4.3** -- The engine **SHALL** provide reusable consideration types for distance, LOS,
   health, threat, time, and resources, with a trait interface for registering custom
   considerations.
   - **Rationale:** Built-in considerations cover common needs; the extension interface enables
     game-specific inputs without engine modification.
   - **Verification:** Verify all built-in types produce valid 0-1 scores across their input range.
     Register a custom consideration and verify it is evaluated during action scoring.

## Advanced Reasoning

1. **R-7.4.4** -- The engine **SHALL** support dual-axis scoring where one axis ranks action
   categories and a second ranks actions within the chosen category, with hysteresis on category
   transitions.
   - **Rationale:** Category prioritization prevents low-urgency actions in critical categories from
     being drowned out; hysteresis prevents oscillation.
   - **Verification:** Verify a survival action always outranks a social action regardless of score.
     Verify hysteresis prevents category switching near the threshold. Verify mobile low-LOD agents
     correctly use single-axis fallback.

2. **R-7.4.5** -- The engine **SHALL** group actions into context sets and evaluate only the active
   context's action pool each tick, with hysteresis thresholds on context transitions.
   - **Rationale:** Context filtering reduces per-tick evaluation cost for NPCs with large action
     repertoires.
   - **Verification:** Verify actions outside the active context are never evaluated. Verify
     hysteresis prevents context oscillation. Compare per-tick cost with and without context
     filtering for NPCs with 30+ actions.
