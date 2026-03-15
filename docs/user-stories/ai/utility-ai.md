# User Stories — 7.4 Utility AI

## F-7.4.1 — Scoring Functions & Response Curves

## US-7.4.1.1 Define Response Curves as Data Assets
**As a** designer (P-5), **I want to** define response curves (linear, quadratic, logistic, step,
custom piecewise) as data assets, **so that** I can tune NPC priorities without code changes.

## US-7.4.1.2 Map Input Values to 0-1 Scores
**As a** designer (P-5), **I want to** map any input value (health %, distance, ammo count) to a 0-1
score via a selected curve, **so that** considerations produce normalized outputs for blending.

## US-7.4.1.3 Preview Curve Shape in Editor
**As a** designer (P-5), **I want to** preview the shape of each response curve in the editor with a
graph visualization, **so that** I can see how input maps to output before testing in-game.

## US-7.4.1.4 See NPCs Prioritize Healing When Low Health
**As a** player (P-23), **I want** NPCs to prioritize healing or fleeing when their health is low,
**so that** AI shows self-preservation instincts.

## US-7.4.1.5 Watch Enemies Change Tactics Based on Ammo
**As a** player (P-23), **I want** enemies to switch from ranged to melee when their ammo runs low,
**so that** resource-aware AI feels intelligent.

## US-7.4.1.6 See NPCs Weigh Multiple Factors in Decisions
**As a** player (P-23), **I want** NPC decisions to visibly weigh multiple factors (health,
distance, threat), **so that** behavior feels nuanced rather than binary.

## US-7.4.1.7 Implement Configurable Response Curve Evaluator
**As an** engine developer (P-26), **I want to** implement a curve evaluator supporting linear,
quadratic, logistic, step, and piecewise curve types, **so that** any consideration shape is
expressible.

## US-7.4.1.8 Load Curves from Data Assets at Runtime
**As an** engine developer (P-26), **I want to** load response curve definitions from data assets,
**so that** curves are hot-reloadable during development.

## US-7.4.1.9 Use Linear-Only Curves for Low-LOD on Mobile
**As an** engine developer (P-26), **I want** low-LOD agents on mobile to use linear-only curves,
**so that** piecewise evaluation cost is avoided on constrained devices.

## US-7.4.1.10 Verify Curve Output Stays in 0-1 Range
**As an** engine tester (P-27), **I want to** verify that all curve types produce output values
clamped to the 0-1 range for any valid input, **so that** consideration scores are normalized.

## US-7.4.1.11 Test All Curve Types Against Reference Values
**As an** engine tester (P-27), **I want to** test each curve type against known input-output pairs,
**so that** curve math is correct.

## US-7.4.1.12 Benchmark Curve Evaluation Per Agent
**As an** engine tester (P-27), **I want to** benchmark curve evaluation cost per agent for all
curve types, **so that** scoring overhead is within the per-tick AI budget.

---

## F-7.4.2 — Action Selection & Compensation

## US-7.4.2.1 Configure Selection Strategy Per Agent
**As a** designer (P-5), **I want to** select the action selection strategy per agent archetype
(highest score, weighted random top N, threshold filter), **so that** different NPCs have varying
degrees of predictability.

## US-7.4.2.2 Enable Score Compensation
**As a** designer (P-5), **I want to** enable score compensation to prevent penalizing actions with
more considerations, **so that** action evaluation is fair regardless of complexity.

## US-7.4.2.3 View Scored Action Rankings in Debug
**As a** designer (P-5), **I want to** view the ranked list of scored actions with their final
values in a debug panel, **so that** I can see why a specific action was chosen.

## US-7.4.2.4 See NPCs Make Varied but Reasonable Choices
**As a** player (P-23), **I want** NPCs to make varied choices (not always the same action) while
still being reasonable, **so that** AI behavior has personality without being random.

## US-7.4.2.5 Watch AI Select Optimal Actions Under Pressure
**As a** player (P-23), **I want** AI under pressure to reliably choose the most urgent action
(heal, flee, fight), **so that** critical decisions feel decisive.

## US-7.4.2.6 See Different NPCs Choose Differently in Same Situation
**As a** player (P-23), **I want** different NPC archetypes in the same situation to choose
different actions (brave warrior attacks, cautious merchant flees), **so that** personality shows.

## US-7.4.2.7 Implement Score Multiplication with Compensation
**As an** engine developer (P-26), **I want to** multiply consideration scores per action and apply
a compensation factor based on consideration count, **so that** selection is fair.

## US-7.4.2.8 Implement Selection Strategies
**As an** engine developer (P-26), **I want to** implement highest-score, weighted-random-top-N, and
threshold-filter selection strategies, **so that** designers have multiple selection modes.

## US-7.4.2.9 Limit Action Pool on Mobile
**As an** engine developer (P-26), **I want to** limit the candidate action pool to 8 on mobile (vs.
32 on desktop) for low-LOD agents, **so that** per-tick evaluation cost is bounded.

## US-7.4.2.10 Verify Compensation Factor Corrects Multi-Input Bias
**As an** engine tester (P-27), **I want to** verify that compensation corrects the bias where
actions with more considerations score lower, **so that** evaluation is count-independent.

## US-7.4.2.11 Test Weighted Random Produces Expected Distribution
**As an** engine tester (P-27), **I want to** verify that weighted random selection produces a
probability distribution matching action score ratios over many samples, **so that** randomization
is correctly weighted.

## US-7.4.2.12 Benchmark Selection with Maximum Action Pool
**As an** engine tester (P-27), **I want to** benchmark selection time with the maximum action pool
size, **so that** worst-case evaluation fits within the per-tick budget.

---

## F-7.4.3 — Considerations & Input Axes

## US-7.4.3.1 Use Built-In Considerations
**As a** designer (P-5), **I want to** use a standard library of considerations (distance to target,
health ratio, threat level, time since last action), **so that** common inputs are available without
custom code.

## US-7.4.3.2 Compose Multiple Considerations Per Action
**As a** designer (P-5), **I want to** compose multiple considerations per action evaluation in the
editor, **so that** decisions factor in several simultaneous inputs.

## US-7.4.3.3 Register Custom Considerations via Trait
**As a** designer (P-5), **I want to** register project-specific considerations through a trait
interface, **so that** game-specific inputs extend the utility system.

## US-7.4.3.4 See AI Decisions Reflect Multiple Factors
**As a** player (P-23), **I want** AI to consider both distance and threat when choosing actions,
**so that** behavior shows multi-factor reasoning.

## US-7.4.3.5 Watch AI React to Resource Scarcity
**As a** player (P-23), **I want** AI to change behavior when resources run low (switch weapons,
seek supplies), **so that** resource availability visibly affects decisions.

## US-7.4.3.6 See AI Timing Vary with Recent Actions
**As a** player (P-23), **I want** AI to avoid repeating the same action back-to-back due to "time
since last action" consideration, **so that** behavior has variety over time.

## US-7.4.3.7 Implement Standard Consideration Library
**As an** engine developer (P-26), **I want to** implement reusable consideration types for
distance, LOS, health, threat, time, and resources, **so that** common inputs are built-in.

## US-7.4.3.8 Define Trait Interface for Custom Considerations
**As an** engine developer (P-26), **I want to** define a trait interface for registering custom
considerations, **so that** gameplay code extends the utility system.

## US-7.4.3.9 Budget Expensive Considerations on Mobile
**As an** engine developer (P-26), **I want** expensive considerations (LOS raycasts) to be
evaluated less frequently on mobile via the perception budget, **so that** scoring stays fast.

## US-7.4.3.10 Verify All Built-In Considerations Produce Valid Scores
**As an** engine tester (P-27), **I want to** verify that all built-in consideration types produce
valid 0-1 scores for their full input range, **so that** no consideration outputs invalid values.

## US-7.4.3.11 Test Custom Consideration Registration
**As an** engine tester (P-27), **I want to** register a mock custom consideration and verify it is
evaluated during action scoring, **so that** the extension interface works end-to-end.

## US-7.4.3.12 Benchmark Multi-Consideration Actions
**As an** engine tester (P-27), **I want to** benchmark actions with 4-8 considerations and measure
per-action evaluation time, **so that** multi-factor scoring is performant.

---

## F-7.4.4 — Dual Utility System

## US-7.4.4.1 Define Action Categories
**As a** designer (P-5), **I want to** define action categories (combat, social, survival) with a
first-axis priority ranking, **so that** critical categories are not drowned out by less important
high-scoring actions.

## US-7.4.4.2 Rank Actions Within Categories
**As a** designer (P-5), **I want** the second axis to rank specific actions within the chosen
category, **so that** fine-grained selection occurs after category prioritization.

## US-7.4.4.3 Preview Dual-Axis Rankings in Debug Panel
**As a** designer (P-5), **I want to** see both category rankings and within-category action
rankings in the debug panel, **so that** I can diagnose selection across both axes.

## US-7.4.4.4 See NPC Prioritize Survival Over Social Actions
**As a** player (P-23), **I want** an NPC to stop chatting and run for cover when combat breaks out,
**so that** survival priorities override social behavior.

## US-7.4.4.5 Watch Merchants Stop Trading When Attacked
**As a** player (P-23), **I want** merchants to stop trading and flee or fight when attacked, **so
that** category prioritization produces believable emergency responses.

## US-7.4.4.6 See NPCs Return to Normal After Danger Passes
**As a** player (P-23), **I want** NPCs to resume social or idle actions after a threat is resolved,
**so that** category de-escalation is visible.

## US-7.4.4.7 Implement Two-Axis Scoring Model
**As an** engine developer (P-26), **I want to** implement a dual-axis scoring model that first
ranks categories and then ranks actions within the winning category, **so that** low-urgency actions
in critical categories are not suppressed.

## US-7.4.4.8 Support Category Transition Hysteresis
**As an** engine developer (P-26), **I want to** support hysteresis on category transitions, **so
that** NPCs do not oscillate rapidly between categories at boundary scores.

## US-7.4.4.9 Fall Back to Single-Axis on Mobile Low-LOD
**As an** engine developer (P-26), **I want** low-LOD agents on mobile to fall back to single-axis
scoring, **so that** evaluation cost is halved on constrained devices.

## US-7.4.4.10 Verify Category Prioritization Order
**As an** engine tester (P-27), **I want to** verify that a high-scoring survival action always
outranks a high-scoring social action, **so that** category priority ordering works.

## US-7.4.4.11 Test Hysteresis Prevents Rapid Category Switching
**As an** engine tester (P-27), **I want to** verify that hysteresis prevents category switching
when scores are near the threshold, **so that** NPCs do not flicker between behaviors.

## US-7.4.4.12 Validate Mobile Fallback to Single-Axis
**As an** engine tester (P-27), **I want to** confirm that low-LOD agents on mobile correctly use
single-axis scoring and never invoke dual-axis evaluation, **so that** the fallback works.

---

## F-7.4.5 — Context-Based Reasoning

## US-7.4.5.1 Define Context Sets in Editor
**As a** designer (P-5), **I want to** define context sets (in-combat, exploring, fleeing,
socializing) and assign actions to each set, **so that** only relevant actions are evaluated.

## US-7.4.5.2 Configure Context Transition Hysteresis
**As a** designer (P-5), **I want to** set hysteresis thresholds for context transitions, **so
that** NPCs do not switch contexts rapidly at boundary conditions.

## US-7.4.5.3 Assign Fewer Actions to Mobile Contexts
**As a** designer (P-5), **I want to** create coarser context sets with fewer actions for mobile,
**so that** per-tick evaluation cost fits the mobile AI budget.

## US-7.4.5.4 See NPC Behavior Shift Between Exploration and Combat
**As a** player (P-23), **I want** an NPC to shift from exploration behavior (looking around,
investigating) to combat behavior (attacking, taking cover) in a smooth transition, **so that**
context changes feel motivated.

## US-7.4.5.5 Watch Fleeing NPCs Focus Only on Escape
**As a** player (P-23), **I want** fleeing NPCs to focus entirely on escape actions without stopping
to chat or trade, **so that** context filtering makes panic behavior convincing.

## US-7.4.5.6 See Socializing NPCs Ignore Minor Threats
**As a** player (P-23), **I want** socializing NPCs to ignore distant threats unless they escalate,
**so that** context hysteresis prevents twitchy overreaction.

## US-7.4.5.7 Implement Context Set Evaluation Filter
**As an** engine developer (P-26), **I want to** evaluate only the active context's action pool each
tick, **so that** NPCs with large action repertoires have bounded evaluation cost.

## US-7.4.5.8 Implement Hysteresis for Context Transitions
**As an** engine developer (P-26), **I want to** apply hysteresis thresholds to context transitions,
**so that** rapid switching is prevented at boundary scores.

## US-7.4.5.9 Support Coarser Context Groups on Mobile
**As an** engine developer (P-26), **I want to** support platform-specific context groups with fewer
actions per set on mobile, **so that** evaluation cost scales down appropriately.

## US-7.4.5.10 Verify Only Active Context Actions Are Evaluated
**As an** engine tester (P-27), **I want to** verify that actions outside the active context set are
never evaluated, **so that** context filtering eliminates unnecessary work.

## US-7.4.5.11 Test Hysteresis Prevents Context Oscillation
**As an** engine tester (P-27), **I want to** verify that hysteresis prevents context oscillation
when input values fluctuate near the transition threshold, **so that** behavior is stable.

## US-7.4.5.12 Benchmark Per-Tick Cost with Context Filtering
**As an** engine tester (P-27), **I want to** compare per-tick evaluation cost with and without
context filtering for NPCs with 30+ actions, **so that** the performance benefit is validated.
