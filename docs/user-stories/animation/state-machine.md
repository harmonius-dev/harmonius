# User Stories -- 9.4 Animation State Machine

## State Graph and Transitions

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.4.1.1  | character animator (P-11)  |
| US-9.4.1.2  | engine developer (P-26)    |
| US-9.4.1.3  | engine developer (P-26)    |
| US-9.4.2.1  | character animator (P-11)  |
| US-9.4.2.2  | character animator (P-11)  |
| US-9.4.3.1  | character animator (P-11)  |
| US-9.4.3.2  | technical artist (P-13)    |

1. **US-9.4.1.1** -- **As a** character animator (P-11), **I want** a declarative state graph where
   each node represents a pose source, **so that** I can author character animation flow without
   code.

2. **US-9.4.1.2** -- **As an** engine developer (P-26), **I want** per-character graph instances to
   share graph definitions, **so that** memory overhead is minimized across many animated entities.

3. **US-9.4.1.3** -- **As an** engine developer (P-26), **I want** per-instance state memory under 1
   KB, **so that** 1000 animated entities evaluate their state graphs within 1 ms.

4. **US-9.4.2.1** -- **As a** character animator (P-11), **I want** transitions with configurable
   blend duration, blend curve, and per-bone blend profiles, **so that** state changes look natural
   and polished.

5. **US-9.4.2.2** -- **As a** character animator (P-11), **I want** sync markers in clips that align
   footfalls across source and destination states during transitions, **so that** foot contact
   timing is preserved.

6. **US-9.4.3.1** -- **As a** character animator (P-11), **I want** to encapsulate related states
   into reusable sub-state machines, **so that** complex behaviors are modular and shareable across
   character archetypes.

7. **US-9.4.3.2** -- **As a** technical artist (P-13), **I want** sub-state machines composable
   hierarchically, **so that** I can build layered animation systems from reusable building blocks.

## Layers, Variables, and Sync

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.4.4.1  | character animator (P-11)  |
| US-9.4.4.2  | engine developer (P-26)    |
| US-9.4.4.3  | game developer (P-15)      |
| US-9.4.5.1  | character animator (P-11)  |
| US-9.4.5.2  | engine developer (P-26)    |
| US-9.4.6.1  | character animator (P-11)  |
| US-9.4.6.2  | engine developer (P-26)    |

1. **US-9.4.4.1** -- **As a** character animator (P-11), **I want** to run multiple state machine
   instances in parallel on separate layers with per-bone masks and blend modes, **so that**
   upper-body and lower-body animations are independent.

2. **US-9.4.4.2** -- **As an** engine developer (P-26), **I want** layer weights to be dynamically
   adjustable at runtime, **so that** gameplay code can fade layers in and out smoothly.

3. **US-9.4.5.1** -- **As a** character animator (P-11), **I want** named parameters that gameplay
   code sets to drive state transitions via boolean expressions, **so that** animation responds to
   game state changes.

4. **US-9.4.5.2** -- **As an** engine developer (P-26), **I want** trigger parameters that
   auto-reset after consumption, **so that** one-shot events like jump or attack fire exactly once.

5. **US-9.4.4.3** -- **As a** game developer (P-15), **I want** to fade a layer weight to zero at
   runtime, **so that** disabled layers have no effect on the final pose.

6. **US-9.4.5.1** -- **As a** character animator (P-11), **I want** named parameters that gameplay
   code sets to drive state transitions via boolean expressions, **so that** animation responds to
   game state changes.

7. **US-9.4.5.2** -- **As an** engine developer (P-26), **I want** trigger parameters that
   auto-reset after consumption, **so that** one-shot events like jump or attack fire exactly once.

8. **US-9.4.6.1** -- **As a** character animator (P-11), **I want** sync groups that keep walk and
   run cycles phase-synchronized regardless of playback rates, **so that** locomotion blends
   maintain consistent foot contact timing.

9. **US-9.4.6.2** -- **As an** engine developer (P-26), **I want** sync groups to advance all member
   clips by normalized time, **so that** foot contact markers align at all blend weights.

## Montages, Blend Spaces, and Integration

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.4.7.1   | character animator (P-11)  |
| US-9.4.7.2   | engine developer (P-26)    |
| US-9.4.7.3   | game developer (P-15)      |
| US-9.4.8.1   | character animator (P-11)  |
| US-9.4.8.2   | technical artist (P-13)    |
| US-9.4.8.3   | engine developer (P-26)    |
| US-9.4.9.1   | character animator (P-11)  |
| US-9.4.9.2   | engine developer (P-26)    |
| US-9.4.10.1  | engine developer (P-26)    |
| US-9.4.10.2  | technical artist (P-13)    |
| US-9.4.10.3  | game developer (P-15)      |

1. **US-9.4.7.1** -- **As a** character animator (P-11), **I want** animation montages that
   temporarily override state machine output on specific bone groups, **so that** emotes, combat
   abilities, and interaction animations play without disrupting the base state.

2. **US-9.4.7.2** -- **As an** engine developer (P-26), **I want** montages to support branching
   sections and notify events with blend-in/blend-out curves, **so that** complex one-shot sequences
   are expressible as data.

3. **US-9.4.8.1** -- **As a** character animator (P-11), **I want** 1D and 2D blend spaces that
   interpolate between clips based on continuous variables, **so that** locomotion speed and
   direction produce smooth animation blends.

4. **US-9.4.8.2** -- **As a** technical artist (P-13), **I want** blend space sample points
   triangulated with pre-computed barycentric weights, **so that** runtime evaluation is fast and
   predictable.

5. **US-9.4.9.1** -- **As a** character animator (P-11), **I want** additive aim offset layers
   parameterized by pitch and yaw, **so that** weapon aiming produces smooth bone rotations on top
   of locomotion.

6. **US-9.4.10.1** -- **As an** engine developer (P-26), **I want** behavior trees and GOAP planners
   to trigger animation state transitions through the Logic Graph system (F-15.8.4), **so that** AI
   agents drive animation from blackboard variables.

7. **US-9.4.7.3** -- **As a** game developer (P-15), **I want** montage blend-out to return control
   to the state machine smoothly, **so that** one-shot animations end without visible pops or
   discontinuities.

8. **US-9.4.8.3** -- **As an** engine developer (P-26), **I want** blend space triangulation
   pre-computed at asset import time, **so that** runtime evaluation requires only barycentric
   weight lookup without per-frame triangulation.

9. **US-9.4.9.1** -- **As a** character animator (P-11), **I want** additive aim offset layers
   parameterized by pitch and yaw, **so that** weapon aiming produces smooth bone rotations on top
   of locomotion.

10. **US-9.4.9.2** -- **As an** engine developer (P-26), **I want** aim offsets integrated with the
IK system for precise weapon alignment, **so that** the weapon muzzle tracks the aim point
accurately.

11. **US-9.4.10.1** -- **As an** engine developer (P-26), **I want** behavior trees and GOAP
    planners
to trigger animation state transitions through the Logic Graph system, **so that** AI agents drive
animation from blackboard variables.

12. **US-9.4.10.2** -- **As a** technical artist (P-13), **I want** query APIs for current state,
remaining clip time, root motion displacement, and montage status, **so that** gameplay systems
synchronize with animation progress.

13. **US-9.4.10.3** -- **As a** game developer (P-15), **I want** 500 AI agents evaluating animation
    state machines within 2 ms, **so that** large NPC crowds animate within budget.

## Transition Modes and Pose Sources

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.4.11.1  | character animator (P-11)  |
| US-9.4.12.1  | technical artist (P-13)    |
| US-9.4.13.1  | game developer (P-15)      |
| US-9.4.14.1  | engine developer (P-26)    |

1. **US-9.4.11.1** -- **As a** character animator (P-11), **I want** inertialization transitions
   that sample only the target clip and decay the source offset, **so that** motion matching and
   state graph transitions look smoother and cost half as much to evaluate.
2. **US-9.4.12.1** -- **As a** technical artist (P-13), **I want** a Multiply layer blend mode that
   scales bone transforms, **so that** size pulsing, muscle flex, and magnification effects can be
   expressed without custom blend code.
3. **US-9.4.13.1** -- **As a** game developer (P-15), **I want** the same state machine to drive 2D
   sprite-sheet animation via a SpriteSheet pose source, **so that** 2D games reuse state machine
   transitions, conditions, layers, and sync groups without a separate system.
4. **US-9.4.14.1** -- **As an** engine developer (P-26), **I want** motion matching as a first-class
   pose source inside the state machine with inertialization transitions into and out of matching
   regions, **so that** authored states can contain motion matching without losing blend fidelity.

## Montages, Parameters, and Feedback

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.4.15.1  | game developer (P-15)      |
| US-9.4.16.1  | game developer (P-15)      |
| US-9.4.17.1  | game developer (P-15)      |

1. **US-9.4.15.1** -- **As a** game developer (P-15), **I want** montage priority resolution with
   last-started-wins tiebreaking, **so that** gameplay and AI systems requesting montages on the
   same bones resolve deterministically without visual artifacts.
2. **US-9.4.16.1** -- **As a** game developer (P-15), **I want** gameplay and AI systems to drive
   the same `AnimationParams` component read by the state machine, **so that** the same state graph
   works for both player and AI-controlled instances of a character.
3. **US-9.4.17.1** -- **As a** game developer (P-15), **I want** to read current state, elapsed
   time, remaining time, and active montage from `AnimationQuery`, **so that** AI can wait for
   attack animations to complete before choosing the next action.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-9.4.1 | character animator (P-11) |
| US-9.4.10 | engine developer (P-26) |
| US-9.4.11 | character animator (P-11) |
| US-9.4.12 | technical artist (P-13) |
| US-9.4.13 | game developer (P-15) |
| US-9.4.14 | engine developer (P-26) |
| US-9.4.15 | game developer (P-15) |
| US-9.4.16 | game developer (P-15) |
| US-9.4.17 | game developer (P-15) |
| US-9.4.2 | character animator (P-11) |
| US-9.4.3 | character animator (P-11) |
| US-9.4.4 | character animator (P-11) |
| US-9.4.5 | character animator (P-11) |
| US-9.4.6 | character animator (P-11) |
| US-9.4.7 | character animator (P-11) |
| US-9.4.8 | character animator (P-11) |
| US-9.4.9 | character animator (P-11) |

1. **US-9.4.1** -- **As a** character animator (P-11), **I want** the capabilities defined in
   sub-stories US-9.4.1.1 through US-9.4.1.3 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-9.4.10** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-9.4.10.1 through US-9.4.10.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-9.4.11** -- **As a** character animator (P-11), **I want** the capabilities defined in
   sub-stories US-9.4.11.1 through US-9.4.11.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-9.4.12** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-9.4.12.1 through US-9.4.12.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

5. **US-9.4.13** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-9.4.13.1 through US-9.4.13.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

6. **US-9.4.14** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-9.4.14.1 through US-9.4.14.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

7. **US-9.4.15** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-9.4.15.1 through US-9.4.15.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

8. **US-9.4.16** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-9.4.16.1 through US-9.4.16.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

9. **US-9.4.17** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-9.4.17.1 through US-9.4.17.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

10. **US-9.4.2** -- **As a** character animator (P-11), **I want** the capabilities defined in
    sub-stories
US-9.4.2.1 through US-9.4.2.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

11. **US-9.4.3** -- **As a** character animator (P-11), **I want** the capabilities defined in
    sub-stories
US-9.4.3.1 through US-9.4.3.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

12. **US-9.4.4** -- **As a** character animator (P-11), **I want** the capabilities defined in
    sub-stories
US-9.4.4.1 through US-9.4.4.3 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

13. **US-9.4.5** -- **As a** character animator (P-11), **I want** the capabilities defined in
    sub-stories
US-9.4.5.1 through US-9.4.5.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

14. **US-9.4.6** -- **As a** character animator (P-11), **I want** the capabilities defined in
    sub-stories
US-9.4.6.1 through US-9.4.6.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

15. **US-9.4.7** -- **As a** character animator (P-11), **I want** the capabilities defined in
    sub-stories
US-9.4.7.1 through US-9.4.7.3 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

16. **US-9.4.8** -- **As a** character animator (P-11), **I want** the capabilities defined in
    sub-stories
US-9.4.8.1 through US-9.4.8.3 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

17. **US-9.4.9** -- **As a** character animator (P-11), **I want** the capabilities defined in
    sub-stories
US-9.4.9.1 through US-9.4.9.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
