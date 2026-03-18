# User Stories -- 9.4 Animation State Machine

## F-9.4.1

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.4.1.1 | character animator (P-11) | F-9.4.1  | R-9.4.1      |
| US-9.4.1.2 | game developer (P-15)     | F-9.4.1  | R-9.4.1      |
| US-9.4.1.3 | engine tester (P-27)      | F-9.4.1  | R-9.4.1      |

1. **US-9.4.1.1** — I want a declarative state graph where each node is a pose source (clip, blend
   tree, sub-state machine, montage) evaluated CPU-side with GPU-uploaded blend descriptors
   - **Acceptance:** I can build complex animation behavior visually without writing code
2. **US-9.4.1.2** — I want per-character graph instances that share graph definitions
   - **Acceptance:** thousands of MMO characters use minimal memory overhead per instance
3. **US-9.4.1.3** — I want spawn 1,000 characters each with active state graph instances and verify
   that CPU evaluation time stays within 1ms
   - **Acceptance:** state graph overhead is acceptable at MMO density

## F-9.4.2

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.4.2.1 | character animator (P-11) | F-9.4.2  | R-9.4.2      |
| US-9.4.2.2 | engine tester (P-27)      | F-9.4.2  | R-9.4.2      |

1. **US-9.4.2.1** — I want transitions with configurable blend duration, blend curve, and per-bone
   blend profiles that allow different body parts to transition at different rates
   - **Acceptance:** locomotion transitions keep feet planted via sync marker alignment
2. **US-9.4.2.2** — I want transition between walk and run cycles with sync markers and measure foot
   sliding distance, verifying it stays below 1cm
   - **Acceptance:** locomotion transitions produce grounded foot contacts

## F-9.4.3

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.4.3.1 | character animator (P-11) | F-9.4.3  | R-9.4.3      |
| US-9.4.3.2 | game developer (P-15)     | F-9.4.3  | R-9.4.3      |

1. **US-9.4.3.1** — I want package a combat state machine (idle, attack, dodge, stagger) as a
   reusable sub-graph with entry and exit points
   - **Acceptance:** multiple character archetypes share the combat sub-graph without duplication
2. **US-9.4.3.2** — I want nest a combat sub-state machine inside a top-level locomotion graph
   - **Acceptance:** complex character behavior is decomposed into manageable, composable modules

## F-9.4.4

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.4.4.1 | character animator (P-11) | F-9.4.4  | R-9.4.4      |
| US-9.4.4.2 | game developer (P-15)     | F-9.4.4  | R-9.4.4      |
| US-9.4.4.3 | engine tester (P-27)      | F-9.4.4  | R-9.4.4      |

1. **US-9.4.4.1** — I want multiple state machine layers running in parallel with per-bone masks and
   blend modes (override, additive)
   - **Acceptance:** upper-body aiming operates independently from lower-body locomotion
2. **US-9.4.4.2** — I want dynamically adjust layer weights at runtime to smoothly activate and
   deactivate overlay behaviors (facial expressions, hit reactions)
   - **Acceptance:** layer transitions feel natural rather than snapping on/off
3. **US-9.4.4.3** — I want verify that mobile supports 2 parallel layers, Switch 3, and desktop 4+
   - **Acceptance:** layer complexity respects per-platform animation budgets

## F-9.4.5

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-9.4.5.1 | game designer (P-5)  | F-9.4.5  | R-9.4.5      |
| US-9.4.5.2 | engine tester (P-27) | F-9.4.5  | R-9.4.5      |

1. **US-9.4.5.1** — I want named parameters (booleans, floats, triggers) that I set from logic
   graphs to drive state transitions via boolean expressions
   - **Acceptance:** I can tune animation behavior without programmer support
2. **US-9.4.5.2** — I want fire a trigger parameter (jump, attack) and verify it auto-resets after
   the transition consumes it
   - **Acceptance:** one-shot events fire exactly once without manual reset

## F-9.4.6

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.4.6.1 | character animator (P-11) | F-9.4.6  | R-9.4.6      |
| US-9.4.6.2 | engine tester (P-27)      | F-9.4.6  | R-9.4.6      |

1. **US-9.4.6.1** — I want sync groups that advance multiple clips by normalized time with aligned
   sync markers
   - **Acceptance:** blending between walk and run speeds keeps feet in consistent contact timing
     without sliding
2. **US-9.4.6.2** — I want sweep blend space speed from walk to sprint and verify that foot contact
   markers stay aligned within 1 frame tolerance
   - **Acceptance:** locomotion blending is temporally correct

## F-9.4.7

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.4.7.1 | character animator (P-11) | F-9.4.7  | R-9.4.7      |
| US-9.4.7.2 | game developer (P-15)     | F-9.4.7  | R-9.4.7      |
| US-9.4.7.3 | engine tester (P-27)      | F-9.4.7  | R-9.4.7      |

1. **US-9.4.7.1** — I want montages that temporarily override state machine output on specific bone
   groups with branching sections and blend curves
   - **Acceptance:** emotes, combat abilities, and interaction animations play cleanly on top of
     locomotion
2. **US-9.4.7.2** — I want concurrent montage playback to scale to hundreds of simultaneous players
   triggering unique ability montages
   - **Acceptance:** MMO raid encounters run without animation system bottlenecks
3. **US-9.4.7.3** — I want play a montage with embedded notify events and verify that montage-scoped
   notifies fire only during montage playback and not during state machine evaluation
   - **Acceptance:** event scoping is correct

## F-9.4.8

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.4.8.1 | character animator (P-11) | F-9.4.8  | R-9.4.8      |
| US-9.4.8.2 | character animator (P-11) | F-9.4.8  | R-9.4.8      |
| US-9.4.8.3 | engine tester (P-27)      | F-9.4.8  | R-9.4.8      |

1. **US-9.4.8.1** — I want 1D and 2D blend spaces where I place animation samples in parameter space
   (speed, direction) and the runtime interpolates using barycentric weights
   - **Acceptance:** locomotion covers all movement angles smoothly
2. **US-9.4.8.2** — I want see the triangulated sample layout in the blend space editor and adjust
   sample positions visually
   - **Acceptance:** I can optimize clip placement for smooth interpolation across the full
     parameter range
3. **US-9.4.8.3** — I want verify that mobile supports 6-8 blend space samples, Switch 12, and
   desktop 16+
   - **Acceptance:** blend space complexity scales per platform

## F-9.4.9

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.4.9.1 | character animator (P-11) | F-9.4.9  | R-9.4.9      |
| US-9.4.9.2 | game developer (P-15)     | F-9.4.9  | R-9.4.9      |

1. **US-9.4.9.1** — I want aim offsets as 2D blend spaces parameterized by pitch and yaw that
   produce additive bone rotations on top of locomotion
   - **Acceptance:** characters aim weapons toward a target while locomotion continues independently
2. **US-9.4.9.2** — I want aim offsets integrated with the IK system for precise weapon-to-target
   alignment
   - **Acceptance:** the gun barrel points accurately at the aim target in third-person shooters

## F-9.4.10

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-9.4.10.1 | game designer (P-5)   | F-9.4.10 | R-9.4.10     |
| US-9.4.10.2 | game developer (P-15) | F-9.4.10 | R-9.4.10     |
| US-9.4.10.3 | engine tester (P-27)  | F-9.4.10 | R-9.4.10     |

1. **US-9.4.10.1** — I want behavior trees and GOAP planners to set blackboard variables that drive
   blend space parameters and trigger state transitions
   - **Acceptance:** AI agents animate responsively based on their decision-making without hardcoded
     animation calls
2. **US-9.4.10.2** — I want AI systems to query current animation state, remaining clip time, and
   root motion displacement
   - **Acceptance:** AI decisions synchronize with animation timing (wait for attack wind-up, dodge
     after stagger recovery)
3. **US-9.4.10.3** — I want run 500 AI agents each driving animation state via behavior trees and
   verify that combined CPU cost for AI + animation evaluation stays within 2ms
   - **Acceptance:** AI animation scales to MMO population density
