# R-7.4 -- Utility AI User Stories

## US-7.4.1 Scoring Functions & Response Curves

### US-7.4.1.1
As a **designer (P-5)**, I want configurable response curves (linear, quadratic,
logistic, step, custom)
so that I can tune NPC priorities as data assets without code.

### US-7.4.1.2
As a **designer (P-5)**, I want each consideration to map inputs to a 0-1 score
so that different considerations are comparable.

### US-7.4.1.3
As a **player (P-23)**, I want NPCs to make contextually appropriate decisions
so that AI behavior feels intelligent and varied.

### US-7.4.1.4
As an **engine tester (P-27)**, I want to verify all curve types produce outputs clamped
to [0.0, 1.0]
so that score normalization is regression-tested.

---

## US-7.4.2 Action Selection & Compensation

### US-7.4.2.1
As an **engine dev (P-26)**, I want compensation factors based on consideration count
so that actions with more inputs are not penalized unfairly.

### US-7.4.2.2
As a **designer (P-5)**, I want configurable selection strategies (highest, weighted
random, threshold)
so that I can control action variety per NPC archetype.

### US-7.4.2.3
As an **engine tester (P-27)**, I want to verify compensation adjusts 2-consideration
and 5-consideration actions to comparable final scores
so that compensation fairness is regression-tested.

---

## US-7.4.3 Considerations & Input Axes

### US-7.4.3.1
As a **designer (P-5)**, I want built-in considerations (distance, LOS, health, threat)
so that common evaluations are available without custom code.

### US-7.4.3.2
As an **engine dev (P-26)**, I want custom considerations registered via a trait interface
so that project-specific inputs are supported.

### US-7.4.3.3
As an **engine tester (P-27)**, I want to verify each built-in consideration returns
valid 0-1 scores
so that consideration correctness is regression-tested.

---

## US-7.4.4 Dual Utility System

### US-7.4.4.1
As a **designer (P-5)**, I want two-axis scoring (category priority + action score)
so that critical categories are not outscored by non-urgent actions.

### US-7.4.4.2
As a **player (P-23)**, I want NPCs to prioritize survival over socializing when hurt
so that AI decisions reflect appropriate urgency.

### US-7.4.4.3
As an **engine tester (P-27)**, I want to verify survival category outranks social even
when social actions score higher within-category
so that dual-axis priority is regression-tested.

---

## US-7.4.5 Context-Based Reasoning

### US-7.4.5.1
As a **designer (P-5)**, I want actions grouped into context sets (combat, exploring,
fleeing)
so that only the active context is evaluated per tick.

### US-7.4.5.2
As an **engine dev (P-26)**, I want hysteresis thresholds for context transitions
so that rapid switching between contexts is prevented.

### US-7.4.5.3
As an **engine tester (P-27)**, I want to verify only the active context's actions are
evaluated per tick
so that context filtering is regression-tested.
