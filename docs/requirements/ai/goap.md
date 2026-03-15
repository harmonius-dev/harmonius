# R-7.5 -- Goal-Oriented Action Planning User Stories

## US-7.5.1 World State Representation

### US-7.5.1.1
As an **engine dev (P-26)**, I want world state as a fixed-size bitset
so that copy, compare, and diff operations are O(1).

### US-7.5.1.2
As a **designer (P-5)**, I want named boolean and integer properties in world state
so that I can define AI-relevant state declaratively.

### US-7.5.1.3
As an **engine tester (P-27)**, I want to verify state operations complete in constant
time regardless of property count
so that bitset performance is regression-tested.

---

## US-7.5.2 GOAP Forward-Search Planner

### US-7.5.2.1
As a **designer (P-5)**, I want A* search over the action space toward goals
so that NPCs compose behavior sequences dynamically.

### US-7.5.2.2
As a **player (P-23)**, I want NPCs to figure out multi-step plans on their own
so that AI feels intelligent rather than scripted.

### US-7.5.2.3
As an **engine dev (P-26)**, I want the planner to find minimal-cost action sequences
so that plans prefer cheaper actions when available.

### US-7.5.2.4
As an **engine tester (P-27)**, I want to verify the planner returns the cheapest valid
sequence from two alternatives
so that cost optimization is regression-tested.

---

## US-7.5.3 Action Preconditions & Effects

### US-7.5.3.1
As a **designer (P-5)**, I want preconditions and effects declared per action
so that the planner reasons about action applicability automatically.

### US-7.5.3.2
As an **engine dev (P-26)**, I want actions with cost values for plan optimization
so that the planner prefers cheap actions over expensive ones.

### US-7.5.3.3
As an **engine tester (P-27)**, I want to verify an action is only considered when its
preconditions hold
so that precondition enforcement is regression-tested.

---

## US-7.5.4 Plan Caching & Reuse

### US-7.5.4.1
As an **engine dev (P-26)**, I want plans cached by (goal, state-hash)
so that identical requests reuse existing plans.

### US-7.5.4.2
As a **server admin (P-22)**, I want plan caching for same-archetype NPCs
so that server CPU is not wasted on redundant planning.

### US-7.5.4.3
As an **engine tester (P-27)**, I want to verify 10 identical requests produce only
one A* search
so that cache hit behavior is regression-tested.

---

## US-7.5.5 Replanning Triggers

### US-7.5.5.1
As a **designer (P-5)**, I want replanning when action preconditions become invalid
so that NPCs adapt when the world changes.

### US-7.5.5.2
As a **designer (P-5)**, I want a replan cooldown to prevent thrashing
so that rapid state changes do not waste CPU.

### US-7.5.5.3
As a **player (P-23)**, I want NPCs to react when I disrupt their plans
so that AI adapts to player interference.

### US-7.5.5.4
As an **engine tester (P-27)**, I want to verify only one replan occurs during the
cooldown period
so that cooldown throttling is regression-tested.

---

## US-7.5.6 Goal Prioritization

### US-7.5.6.1
As a **designer (P-5)**, I want a scored goal list with dynamic priorities
so that NPCs switch goals based on changing circumstances.

### US-7.5.6.2
As a **player (P-23)**, I want NPCs to abandon combat goals when near death
so that survival instincts feel realistic.

### US-7.5.6.3
As an **engine tester (P-27)**, I want to verify a higher-priority goal causes immediate
replan within one tick
so that goal priority response time is regression-tested.
