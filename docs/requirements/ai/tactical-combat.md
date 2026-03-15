# R-7.8 -- Tactical Combat AI User Stories

## US-7.8.1 Cover Evaluation and Scoring

### US-7.8.1.1
As a **designer (P-5)**, I want cover scored by protection angle, sight lines, flanking exposure,
and distance so that AI selects tactically optimal positions.

### US-7.8.1.2
As a **designer (P-5)**, I want cover re-evaluation on target movement or cover destruction so that
AI adapts to changing tactical situations.

### US-7.8.1.3
As a **designer (P-5)**, I want scoring weights configurable per AI archetype so that cautious AI
prioritizes protection while aggressive AI prioritizes sight lines.

### US-7.8.1.4
As a **player (P-23)**, I want enemies to use cover intelligently so that combat encounters feel
tactically challenging.

### US-7.8.1.5
As an **engine tester (P-27)**, I want to verify full-cover positions rank higher than partial-cover
so that cover scoring order is regression-tested.

---

## US-7.8.2 Flanking and Pincer Behavior

### US-7.8.2.1
As a **designer (P-5)**, I want coordinated multi-angle approaches when multiple AI engage the same
target so that flanking creates challenging tactical encounters.

### US-7.8.2.2
As a **designer (P-5)**, I want flanking paths avoiding the target's line of sight so that flankers
approach undetected.

### US-7.8.2.3
As a **designer (P-5)**, I want flankers to synchronize at staging positions before simultaneous
attack so that the pincer is coordinated.

### US-7.8.2.4
As a **player (P-23)**, I want enemies to try to surround me so that combat requires positional
awareness.

### US-7.8.2.5
As an **engine tester (P-27)**, I want to verify flanking angles differ by at least 60 degrees so
that angle spread is regression-tested.

---

## US-7.8.3 Squad Formation and Communication

### US-7.8.3.1
As a **designer (P-5)**, I want squad formations (line, wedge, column, diamond) with configurable
spacing so that squads move tactically.

### US-7.8.3.2
As a **designer (P-5)**, I want context-adaptive formation selection (wedge in open, column in
corridors) so that squads adapt to terrain.

### US-7.8.3.3
As a **designer (P-5)**, I want squad communication (contact calls, flanking calls, retreat orders)
so that AI coordination is both functional and presentational.

### US-7.8.3.4
As a **player (P-23)**, I want to hear enemy squads calling out my position so that combat
communication adds immersion.

### US-7.8.3.5
As an **engine tester (P-27)**, I want to verify formation switches from wedge to column in narrow
terrain so that context-adaptive formation is regression-tested.

---

## US-7.8.4 Suppressive Fire and Pinning

### US-7.8.4.1
As a **designer (P-5)**, I want AI to suppress a zone rather than aim at an entity so that pinning
fire prevents the target from moving.

### US-7.8.4.2
As a **designer (P-5)**, I want a "suppressed" debuff with accuracy penalty and camera effects so
that being suppressed feels impactful.

### US-7.8.4.3
As a **designer (P-5)**, I want suppression duration and ammo consumption configurable per weapon so
that different weapons have different suppression characteristics.

### US-7.8.4.4
As a **player (P-23)**, I want to feel pinned down by enemy fire so that taking cover during
suppression is a compelling tactical choice.

### US-7.8.4.5
As an **engine tester (P-27)**, I want to verify the suppressed debuff is removed within 2 seconds
after fire ceases so that debuff lifecycle is regression-tested.

---

## US-7.8.5 Search and Investigation Patterns

### US-7.8.5.1
As a **designer (P-5)**, I want systematic search patterns from last-known position so that AI
actively hunts for lost targets.

### US-7.8.5.2
As a **designer (P-5)**, I want squad members dividing the search area so that coordinated search
covers more ground.

### US-7.8.5.3
As a **designer (P-5)**, I want search intensity decreasing over configurable timeout so that AI
eventually returns to patrol.

### US-7.8.5.4
As a **player (P-23)**, I want enemies to search methodically when I break contact so that stealth
re-engagement requires skill.

### US-7.8.5.5
As an **engine tester (P-27)**, I want to verify search visits at least 80% of nearby cover points
so that search thoroughness is regression-tested.

---

## US-7.8.6 Retreat and Fallback Behavior

### US-7.8.6.1
As a **designer (P-5)**, I want AI retreat triggered by low health, destroyed cover, or being
outnumbered so that AI self-preservation creates dynamic combat flow.

### US-7.8.6.2
As a **designer (P-5)**, I want retreating agents using smoke or suppressive fire for cover so that
withdrawal is tactically sophisticated.

### US-7.8.6.3
As a **designer (P-5)**, I want morale-based squad retreat when casualties exceed threshold so that
entire squads fall back when outmatched.

### US-7.8.6.4
As a **player (P-23)**, I want enemies to retreat when I am winning so that combat feels responsive
to my success.

### US-7.8.6.5
As a **player (P-23)**, I want retreated enemies to re-engage if conditions improve so that combat
has dynamic ebb and flow.

### US-7.8.6.6
As an **engine tester (P-27)**, I want to verify retreat triggers within 2 ticks of health dropping
below threshold so that retreat response time is regression-tested.
