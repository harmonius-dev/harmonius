# R-7.2 -- Steering & Avoidance User Stories

## US-7.2.1 RVO/ORCA Local Avoidance

### US-7.2.1.1
As an **engine dev (P-26)**, I want ORCA-based collision avoidance so that agents compute
deadlock-free velocities in dense areas.

### US-7.2.1.2
As a **player (P-23)**, I want crowd NPCs flowing smoothly through chokepoints so that city streets
feel alive and believable.

### US-7.2.1.3
As a **designer (P-5)**, I want deadlock-free crowd flow so that agents do not get stuck in
face-to-face standoffs.

### US-7.2.1.4
As an **engine tester (P-27)**, I want to verify zero agent overlap at chokepoints with 200+ agents
so that collision avoidance is regression-tested under load.

---

## US-7.2.2 Obstacle Avoidance (Static Geometry)

### US-7.2.2.1
As an **engine dev (P-26)**, I want feeler rays against static geometry so that agents steer away
from walls and terrain edges.

### US-7.2.2.2
As a **player (P-23)**, I want NPCs to avoid walking into walls and pillars so that AI movement
looks spatially aware.

### US-7.2.2.3
As an **engine tester (P-27)**, I want to verify agents maintain radius distance from all static
surfaces so that obstacle avoidance is regression-tested.

---

## US-7.2.3 Core Steering Behaviors

### US-7.2.3.1
As a **designer (P-5)**, I want seek, flee, arrive, wander, pursuit, and evade primitives so that I
can compose varied AI movement patterns.

### US-7.2.3.2
As a **player (P-23)**, I want NPCs to pursue with predictive interception so that enemies
anticipate my movement.

### US-7.2.3.3
As a **player (P-23)**, I want NPCs to decelerate smoothly near destinations so that AI does not
overshoot waypoints.

### US-7.2.3.4
As an **engine tester (P-27)**, I want to verify arrive behavior stops within tolerance of the
destination so that arrival precision is regression-tested.

---

## US-7.2.4 Steering Behavior Blending & Priority

### US-7.2.4.1
As a **designer (P-5)**, I want weighted blending of multiple steering behaviors so that agents
combine pursuit and avoidance smoothly.

### US-7.2.4.2
As an **engine dev (P-26)**, I want priority pipeline for steering magnitude allocation so that
high-priority behaviors are satisfied first.

### US-7.2.4.3
As an **engine tester (P-27)**, I want to verify conflicting forces do not produce zero net movement
so that priority blending prevents stalling.

---

## US-7.2.5 Formation Movement

### US-7.2.5.1
As a **designer (P-5)**, I want parameterized formation shapes (line, wedge, column) so that squads
maintain tactical arrangements.

### US-7.2.5.2
As a **player (P-23)**, I want squads to move in formation so that military units look organized.

### US-7.2.5.3
As a **designer (P-5)**, I want formations adjusting spacing in narrow terrain so that squads adapt
to environment constraints.

### US-7.2.5.4
As an **engine tester (P-27)**, I want to verify formation offsets stay within 0.5m tolerance so
that formation coherence is regression-tested.

---

## US-7.2.6 Group Steering & Cohesion

### US-7.2.6.1
As a **designer (P-5)**, I want group cohesion so members share velocity goals so that named groups
move together without fragmenting.

### US-7.2.6.2
As a **player (P-23)**, I want groups of NPCs to stay together during turns so that group movement
looks coordinated.

### US-7.2.6.3
As an **engine tester (P-27)**, I want to verify group bounding radius does not exceed 1.5x initial
radius after turns so that group cohesion is regression-tested.
