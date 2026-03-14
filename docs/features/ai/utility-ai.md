# 7.4 — Utility AI

## Scoring & Selection

### F-7.4.1 Scoring Functions & Response Curves

Each candidate action exposes one or more considerations, each of which maps an input value (health
percentage, distance to target, ammo count) to a 0-1 score via a configurable response curve
(linear, quadratic, logistic, step, custom piecewise). Curves are defined as data assets so
designers can tune NPC priorities without code changes.

- **Requirements:** R-7.4.1
- **Dependencies:** None
- **Platform notes:** None

### F-7.4.2 Action Selection & Compensation

Multiplies consideration scores for each action, applies a compensation factor based on the number
of considerations to avoid penalizing actions with more inputs, and selects the highest-scoring
action. Supports configurable selection strategies: highest score, weighted random among top N, and
threshold-based filtering.

- **Requirements:** R-7.4.2
- **Dependencies:** F-7.4.1
- **Platform notes:** None

### F-7.4.3 Considerations & Input Axes

Defines a standard library of reusable consideration types (distance to target, line of sight,
health ratio, threat level, time since last action, resource availability) that can be composed into
action evaluations. Custom considerations are registered by gameplay code through a trait interface.

- **Requirements:** R-7.4.3
- **Dependencies:** F-7.4.1
- **Platform notes:** None

## Advanced Reasoning

### F-7.4.4 Dual Utility System

Extends single-score selection with a two-axis model: one axis ranks action categories (combat,
social, survival) and a second axis ranks specific actions within the chosen category. Prevents
low-urgency actions in critical categories from being drowned out by high-scoring but less
important actions in other categories.

- **Requirements:** R-7.4.4
- **Dependencies:** F-7.4.2
- **Platform notes:** None

### F-7.4.5 Context-Based Reasoning

Groups actions into context sets (in-combat, exploring, fleeing, socializing) and evaluates only
the active context's action pool each tick. Context transitions are governed by hysteresis
thresholds to avoid rapid switching. Reduces per-tick evaluation cost for NPCs with large action
repertoires.

- **Requirements:** R-7.4.5
- **Dependencies:** F-7.4.2
- **Platform notes:** None
