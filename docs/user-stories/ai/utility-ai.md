# User Stories -- 7.4 Utility AI

## Scoring and Selection

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.4.1.1  | game designer (P-5)     |
| US-7.4.1.2  | game designer (P-5)     |
| US-7.4.1.3  | engine developer (P-26) |
| US-7.4.1.4  | engine developer (P-26) |
| US-7.4.2.1  | game designer (P-5)     |
| US-7.4.2.2  | game designer (P-5)     |
| US-7.4.2.3  | engine developer (P-26) |
| US-7.4.2.4  | engine developer (P-26) |
| US-7.4.3.1  | game designer (P-5)     |
| US-7.4.3.2  | game developer (P-15)   |
| US-7.4.3.3  | engine developer (P-26) |

1. **US-7.4.1.1** -- **As a** game designer (P-5), **I want** to define response curves as data
   assets with linear, quadratic, logistic, step, and piecewise types, **so that** I can tune NPC
   priorities without code changes.

2. **US-7.4.1.2** -- **As a** game designer (P-5), **I want** to preview each response curve's shape
   in the editor with a graph visualization, **so that** I can see how input maps to output before
   testing in-game.

3. **US-7.4.1.3** -- **As an** engine developer (P-26), **I want** to implement a curve evaluator
   supporting all standard curve types, **so that** any consideration shape is expressible.

4. **US-7.4.1.4** -- **As an** engine developer (P-26), **I want** low-LOD agents on mobile to use
   linear-only curves, **so that** piecewise evaluation cost is avoided on constrained devices.

5. **US-7.4.2.1** -- **As a** game designer (P-5), **I want** to select the action selection
   strategy per agent archetype, **so that** different NPCs have varying degrees of predictability.

6. **US-7.4.2.2** -- **As a** game designer (P-5), **I want** to view the ranked list of scored
   actions with final values in a debug panel, **so that** I can see why a specific action was
   chosen.

7. **US-7.4.2.3** -- **As an** engine developer (P-26), **I want** to implement highest-score,
   weighted-random-top-N, and threshold-filter selection strategies, **so that** designers have
   multiple selection modes.

8. **US-7.4.2.4** -- **As an** engine developer (P-26), **I want** a compensation factor based on
   consideration count applied during score multiplication, **so that** actions with more
   considerations are not unfairly penalized.

9. **US-7.4.3.1** -- **As a** game designer (P-5), **I want** a standard library of reusable
   consideration types like distance, health ratio, and threat level, **so that** common inputs are
   available without custom code.

10. **US-7.4.3.2** -- **As a** game developer (P-15), **I want** to register project-specific
    considerations through a trait interface, **so that** game-specific inputs extend the utility
    system.

11. **US-7.4.3.3** -- **As an** engine developer (P-26), **I want** expensive considerations like
    LOS raycasts to be evaluated less frequently on mobile via the perception budget, **so that**
    scoring stays fast on constrained platforms.

## Advanced Reasoning

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.4.4.1  | game designer (P-5)     |
| US-7.4.4.2  | game designer (P-5)     |
| US-7.4.4.3  | engine developer (P-26) |
| US-7.4.4.4  | engine developer (P-26) |
| US-7.4.5.1  | game designer (P-5)     |
| US-7.4.5.2  | game designer (P-5)     |
| US-7.4.5.3  | engine developer (P-26) |
| US-7.4.5.4  | engine developer (P-26) |

1. **US-7.4.4.1** -- **As a** game designer (P-5), **I want** to define action categories with a
   first-axis priority ranking, **so that** critical categories like survival are not drowned out by
   high-scoring social actions.

2. **US-7.4.4.2** -- **As a** game designer (P-5), **I want** to see both category rankings and
   within-category action rankings in the debug panel, **so that** I can diagnose selection across
   both axes.

3. **US-7.4.4.3** -- **As an** engine developer (P-26), **I want** to implement dual-axis scoring
   that first ranks categories then ranks actions within the winning category, **so that**
   low-urgency actions in critical categories are not suppressed.

4. **US-7.4.4.4** -- **As an** engine developer (P-26), **I want** low-LOD agents on mobile to fall
   back to single-axis scoring, **so that** evaluation cost is halved on constrained devices.

5. **US-7.4.5.1** -- **As a** game designer (P-5), **I want** to define context sets and assign
   actions to each set, **so that** only the active context's action pool is evaluated each tick.

6. **US-7.4.5.2** -- **As a** game designer (P-5), **I want** hysteresis thresholds on context
   transitions, **so that** NPCs do not switch contexts rapidly at boundary conditions.

7. **US-7.4.5.3** -- **As an** engine developer (P-26), **I want** to evaluate only the active
   context's action pool each tick, **so that** NPCs with large action repertoires have bounded
   evaluation cost.

8. **US-7.4.5.4** -- **As an** engine developer (P-26), **I want** platform-specific context groups
   with fewer actions per set on mobile, **so that** evaluation cost scales down appropriately.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-7.4.1 | game designer (P-5) |
| US-7.4.2 | game designer (P-5) |
| US-7.4.3 | game designer (P-5) |
| US-7.4.4 | game designer (P-5) |
| US-7.4.5 | game designer (P-5) |

1. **US-7.4.1** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.4.1.1 through US-7.4.1.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-7.4.2** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.4.2.1 through US-7.4.2.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

3. **US-7.4.3** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.4.3.1 through US-7.4.3.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-7.4.4** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.4.4.1 through US-7.4.4.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-7.4.5** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.4.5.1 through US-7.4.5.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.
