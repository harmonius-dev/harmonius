# User Stories — 7.4 Utility AI

## F-7.4.1 — Scoring Functions & Response Curves

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.4.1.1  | designer (P-5)          | F-7.4.1  | R-7.4.1      |
| US-7.4.1.2  | designer (P-5)          | F-7.4.1  | R-7.4.1      |
| US-7.4.1.3  | designer (P-5)          | F-7.4.1  | R-7.4.1      |
| US-7.4.1.4  | player (P-23)           | F-7.4.1  | R-7.4.1      |
| US-7.4.1.5  | player (P-23)           | F-7.4.1  | R-7.4.1      |
| US-7.4.1.6  | player (P-23)           | F-7.4.1  | R-7.4.1      |
| US-7.4.1.7  | engine developer (P-26) | F-7.4.1  | R-7.4.1      |
| US-7.4.1.8  | engine developer (P-26) | F-7.4.1  | R-7.4.1      |
| US-7.4.1.9  | engine developer (P-26) | F-7.4.1  | R-7.4.1      |
| US-7.4.1.10 | engine tester (P-27)    | F-7.4.1  | R-7.4.1      |
| US-7.4.1.11 | engine tester (P-27)    | F-7.4.1  | R-7.4.1      |
| US-7.4.1.12 | engine tester (P-27)    | F-7.4.1  | R-7.4.1      |

1. **US-7.4.1.1** — I want to define response curves (linear, quadratic, logistic, step, custom
   piecewise) as data assets
   - **Acceptance:** I can tune NPC priorities without code changes
2. **US-7.4.1.2** — I want to map any input value (health %, distance, ammo count) to a 0-1 score
   via a selected curve
   - **Acceptance:** considerations produce normalized outputs for blending
3. **US-7.4.1.3** — I want to preview the shape of each response curve in the editor with a graph
   visualization
   - **Acceptance:** I can see how input maps to output before testing in-game
4. **US-7.4.1.4** — I want NPCs to prioritize healing or fleeing when their health is low
   - **Acceptance:** AI shows self-preservation instincts
5. **US-7.4.1.5** — I want enemies to switch from ranged to melee when their ammo runs low
   - **Acceptance:** resource-aware AI feels intelligent
6. **US-7.4.1.6** — I want NPC decisions to visibly weigh multiple factors (health, distance,
   threat)
   - **Acceptance:** behavior feels nuanced rather than binary
7. **US-7.4.1.7** — I want to implement a curve evaluator supporting linear, quadratic, logistic,
   step, and piecewise curve types
   - **Acceptance:** any consideration shape is expressible
8. **US-7.4.1.8** — I want to load response curve definitions from data assets
   - **Acceptance:** curves are hot-reloadable during development
9. **US-7.4.1.9** — I want low-LOD agents on mobile to use linear-only curves
   - **Acceptance:** piecewise evaluation cost is avoided on constrained devices
10. **US-7.4.1.10** — I want to verify that all curve types produce output values clamped to the 0-1
    range for any valid input
    - **Acceptance:** consideration scores are normalized
11. **US-7.4.1.11** — I want to test each curve type against known input-output pairs
    - **Acceptance:** curve math is correct
12. **US-7.4.1.12** — I want to benchmark curve evaluation cost per agent for all curve types
    - **Acceptance:** scoring overhead is within the per-tick AI budget. ---

## F-7.4.2 — Action Selection & Compensation

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.4.2.1  | designer (P-5)          | F-7.4.2  | R-7.4.2      |
| US-7.4.2.2  | designer (P-5)          | F-7.4.2  | R-7.4.2      |
| US-7.4.2.3  | designer (P-5)          | F-7.4.2  | R-7.4.2      |
| US-7.4.2.4  | player (P-23)           | F-7.4.2  | R-7.4.2      |
| US-7.4.2.5  | player (P-23)           | F-7.4.2  | R-7.4.2      |
| US-7.4.2.6  | player (P-23)           | F-7.4.2  | R-7.4.2      |
| US-7.4.2.7  | engine developer (P-26) | F-7.4.2  | R-7.4.2      |
| US-7.4.2.8  | engine developer (P-26) | F-7.4.2  | R-7.4.2      |
| US-7.4.2.9  | engine developer (P-26) | F-7.4.2  | R-7.4.2      |
| US-7.4.2.10 | engine tester (P-27)    | F-7.4.2  | R-7.4.2      |
| US-7.4.2.11 | engine tester (P-27)    | F-7.4.2  | R-7.4.2      |
| US-7.4.2.12 | engine tester (P-27)    | F-7.4.2  | R-7.4.2      |

1. **US-7.4.2.1** — I want to select the action selection strategy per agent archetype (highest
   score, weighted random top N, threshold filter)
   - **Acceptance:** different NPCs have varying degrees of predictability
2. **US-7.4.2.2** — I want to enable score compensation to prevent penalizing actions with more
   considerations
   - **Acceptance:** action evaluation is fair regardless of complexity
3. **US-7.4.2.3** — I want to view the ranked list of scored actions with their final values in a
   debug panel
   - **Acceptance:** I can see why a specific action was chosen
4. **US-7.4.2.4** — I want NPCs to make varied choices (not always the same action) while still
   being reasonable
   - **Acceptance:** AI behavior has personality without being random
5. **US-7.4.2.5** — I want AI under pressure to reliably choose the most urgent action (heal, flee,
   fight)
   - **Acceptance:** critical decisions feel decisive
6. **US-7.4.2.6** — I want different NPC archetypes in the same situation to choose different
   actions (brave warrior attacks, cautious merchant flees)
   - **Acceptance:** personality shows
7. **US-7.4.2.7** — I want to multiply consideration scores per action and apply a compensation
   factor based on consideration count
   - **Acceptance:** selection is fair
8. **US-7.4.2.8** — I want to implement highest-score, weighted-random-top-N, and threshold-filter
   selection strategies
   - **Acceptance:** designers have multiple selection modes
9. **US-7.4.2.9** — I want to limit the candidate action pool to 8 on mobile (vs. 32 on desktop) for
   low-LOD agents
   - **Acceptance:** per-tick evaluation cost is bounded
10. **US-7.4.2.10** — I want to verify that compensation corrects the bias where actions with more
    considerations score lower
    - **Acceptance:** evaluation is count-independent
11. **US-7.4.2.11** — I want to verify that weighted random selection produces a probability
    distribution matching action score ratios over many samples
    - **Acceptance:** randomization is correctly weighted
12. **US-7.4.2.12** — I want to benchmark selection time with the maximum action pool size
    - **Acceptance:** worst-case evaluation fits within the per-tick budget. ---

## F-7.4.3 — Considerations & Input Axes

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.4.3.1  | designer (P-5)          | F-7.4.3  | R-7.4.3      |
| US-7.4.3.2  | designer (P-5)          | F-7.4.3  | R-7.4.3      |
| US-7.4.3.3  | designer (P-5)          | F-7.4.3  | R-7.4.3      |
| US-7.4.3.4  | player (P-23)           | F-7.4.3  | R-7.4.3      |
| US-7.4.3.5  | player (P-23)           | F-7.4.3  | R-7.4.3      |
| US-7.4.3.6  | player (P-23)           | F-7.4.3  | R-7.4.3      |
| US-7.4.3.7  | engine developer (P-26) | F-7.4.3  | R-7.4.3      |
| US-7.4.3.8  | engine developer (P-26) | F-7.4.3  | R-7.4.3      |
| US-7.4.3.9  | engine developer (P-26) | F-7.4.3  | R-7.4.3      |
| US-7.4.3.10 | engine tester (P-27)    | F-7.4.3  | R-7.4.3      |
| US-7.4.3.11 | engine tester (P-27)    | F-7.4.3  | R-7.4.3      |
| US-7.4.3.12 | engine tester (P-27)    | F-7.4.3  | R-7.4.3      |

1. **US-7.4.3.1** — I want to use a standard library of considerations (distance to target, health
   ratio, threat level, time since last action)
   - **Acceptance:** common inputs are available without custom code
2. **US-7.4.3.2** — I want to compose multiple considerations per action evaluation in the editor
   - **Acceptance:** decisions factor in several simultaneous inputs
3. **US-7.4.3.3** — I want to register project-specific considerations through a trait interface
   - **Acceptance:** game-specific inputs extend the utility system
4. **US-7.4.3.4** — I want AI to consider both distance and threat when choosing actions
   - **Acceptance:** behavior shows multi-factor reasoning
5. **US-7.4.3.5** — I want AI to change behavior when resources run low (switch weapons, seek
   supplies)
   - **Acceptance:** resource availability visibly affects decisions
6. **US-7.4.3.6** — I want AI to avoid repeating the same action back-to-back due to "time since
   last action" consideration
   - **Acceptance:** behavior has variety over time
7. **US-7.4.3.7** — I want to implement reusable consideration types for distance, LOS, health,
   threat, time, and resources
   - **Acceptance:** common inputs are built-in
8. **US-7.4.3.8** — I want to define a trait interface for registering custom considerations
   - **Acceptance:** gameplay code extends the utility system
9. **US-7.4.3.9** — I want expensive considerations (LOS raycasts) to be evaluated less frequently
   on mobile via the perception budget
   - **Acceptance:** scoring stays fast
10. **US-7.4.3.10** — I want to verify that all built-in consideration types produce valid 0-1
    scores for their full input range
    - **Acceptance:** no consideration outputs invalid values
11. **US-7.4.3.11** — I want to register a mock custom consideration and verify it is evaluated
    during action scoring
    - **Acceptance:** the extension interface works end-to-end
12. **US-7.4.3.12** — I want to benchmark actions with 4-8 considerations and measure per-action
    evaluation time
    - **Acceptance:** multi-factor scoring is performant. ---

## F-7.4.4 — Dual Utility System

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.4.4.1  | designer (P-5)          | F-7.4.4  | R-7.4.4      |
| US-7.4.4.2  | designer (P-5)          | F-7.4.4  | R-7.4.4      |
| US-7.4.4.3  | designer (P-5)          | F-7.4.4  | R-7.4.4      |
| US-7.4.4.4  | player (P-23)           | F-7.4.4  | R-7.4.4      |
| US-7.4.4.5  | player (P-23)           | F-7.4.4  | R-7.4.4      |
| US-7.4.4.6  | player (P-23)           | F-7.4.4  | R-7.4.4      |
| US-7.4.4.7  | engine developer (P-26) | F-7.4.4  | R-7.4.4      |
| US-7.4.4.8  | engine developer (P-26) | F-7.4.4  | R-7.4.4      |
| US-7.4.4.9  | engine developer (P-26) | F-7.4.4  | R-7.4.4      |
| US-7.4.4.10 | engine tester (P-27)    | F-7.4.4  | R-7.4.4      |
| US-7.4.4.11 | engine tester (P-27)    | F-7.4.4  | R-7.4.4      |
| US-7.4.4.12 | engine tester (P-27)    | F-7.4.4  | R-7.4.4      |

1. **US-7.4.4.1** — I want to define action categories (combat, social, survival) with a first-axis
   priority ranking
   - **Acceptance:** critical categories are not drowned out by less important high-scoring actions
2. **US-7.4.4.2** — I want the second axis to rank specific actions within the chosen category
   - **Acceptance:** fine-grained selection occurs after category prioritization
3. **US-7.4.4.3** — I want to see both category rankings and within-category action rankings in the
   debug panel
   - **Acceptance:** I can diagnose selection across both axes
4. **US-7.4.4.4** — I want an NPC to stop chatting and run for cover when combat breaks out
   - **Acceptance:** survival priorities override social behavior
5. **US-7.4.4.5** — I want merchants to stop trading and flee or fight when attacked
   - **Acceptance:** category prioritization produces believable emergency responses
6. **US-7.4.4.6** — I want NPCs to resume social or idle actions after a threat is resolved
   - **Acceptance:** category de-escalation is visible
7. **US-7.4.4.7** — I want to implement a dual-axis scoring model that first ranks categories and
   then ranks actions within the winning category
   - **Acceptance:** low-urgency actions in critical categories are not suppressed
8. **US-7.4.4.8** — I want to support hysteresis on category transitions
   - **Acceptance:** NPCs do not oscillate rapidly between categories at boundary scores
9. **US-7.4.4.9** — I want low-LOD agents on mobile to fall back to single-axis scoring
   - **Acceptance:** evaluation cost is halved on constrained devices
10. **US-7.4.4.10** — I want to verify that a high-scoring survival action always outranks a
    high-scoring social action
    - **Acceptance:** category priority ordering works
11. **US-7.4.4.11** — I want to verify that hysteresis prevents category switching when scores are
    near the threshold
    - **Acceptance:** NPCs do not flicker between behaviors
12. **US-7.4.4.12** — I want to confirm that low-LOD agents on mobile correctly use single-axis
    scoring and never invoke dual-axis evaluation
    - **Acceptance:** the fallback works. ---

## F-7.4.5 — Context-Based Reasoning

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.4.5.1  | designer (P-5)          | F-7.4.5  | R-7.4.5      |
| US-7.4.5.2  | designer (P-5)          | F-7.4.5  | R-7.4.5      |
| US-7.4.5.3  | designer (P-5)          | F-7.4.5  | R-7.4.5      |
| US-7.4.5.4  | player (P-23)           | F-7.4.5  | R-7.4.5      |
| US-7.4.5.5  | player (P-23)           | F-7.4.5  | R-7.4.5      |
| US-7.4.5.6  | player (P-23)           | F-7.4.5  | R-7.4.5      |
| US-7.4.5.7  | engine developer (P-26) | F-7.4.5  | R-7.4.5      |
| US-7.4.5.8  | engine developer (P-26) | F-7.4.5  | R-7.4.5      |
| US-7.4.5.9  | engine developer (P-26) | F-7.4.5  | R-7.4.5      |
| US-7.4.5.10 | engine tester (P-27)    | F-7.4.5  | R-7.4.5      |
| US-7.4.5.11 | engine tester (P-27)    | F-7.4.5  | R-7.4.5      |
| US-7.4.5.12 | engine tester (P-27)    | F-7.4.5  | R-7.4.5      |

1. **US-7.4.5.1** — I want to define context sets (in-combat, exploring, fleeing, socializing) and
   assign actions to each set
   - **Acceptance:** only relevant actions are evaluated
2. **US-7.4.5.2** — I want to set hysteresis thresholds for context transitions
   - **Acceptance:** NPCs do not switch contexts rapidly at boundary conditions
3. **US-7.4.5.3** — I want to create coarser context sets with fewer actions for mobile
   - **Acceptance:** per-tick evaluation cost fits the mobile AI budget
4. **US-7.4.5.4** — I want an NPC to shift from exploration behavior (looking around, investigating)
   to combat behavior (attacking, taking cover) in a smooth transition
   - **Acceptance:** context changes feel motivated
5. **US-7.4.5.5** — I want fleeing NPCs to focus entirely on escape actions without stopping to chat
   or trade
   - **Acceptance:** context filtering makes panic behavior convincing
6. **US-7.4.5.6** — I want socializing NPCs to ignore distant threats unless they escalate
   - **Acceptance:** context hysteresis prevents twitchy overreaction
7. **US-7.4.5.7** — I want to evaluate only the active context's action pool each tick
   - **Acceptance:** NPCs with large action repertoires have bounded evaluation cost
8. **US-7.4.5.8** — I want to apply hysteresis thresholds to context transitions
   - **Acceptance:** rapid switching is prevented at boundary scores
9. **US-7.4.5.9** — I want to support platform-specific context groups with fewer actions per set on
   mobile
   - **Acceptance:** evaluation cost scales down appropriately
10. **US-7.4.5.10** — I want to verify that actions outside the active context set are never
    evaluated
    - **Acceptance:** context filtering eliminates unnecessary work
11. **US-7.4.5.11** — I want to verify that hysteresis prevents context oscillation when input
    values fluctuate near the transition threshold
    - **Acceptance:** behavior is stable
12. **US-7.4.5.12** — I want to compare per-tick evaluation cost with and without context filtering
    for NPCs with 30+ actions
    - **Acceptance:** the performance benefit is validated
