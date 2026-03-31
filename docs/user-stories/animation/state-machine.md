# User Stories -- 9.4 Animation State Machine

## State Graph and Transitions

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.4.1.1  | character animator (P-11)  |
| US-9.4.1.2  | engine developer (P-26)    |
| US-9.4.2.1  | character animator (P-11)  |
| US-9.4.2.2  | character animator (P-11)  |
| US-9.4.3.1  | character animator (P-11)  |
| US-9.4.3.2  | technical artist (P-13)    |

1. **US-9.4.1.1** -- **As a** character animator (P-11), **I want** a declarative state graph where
   each node represents a pose source, **so that** I can author character animation flow without
   code.

2. **US-9.4.1.2** -- **As an** engine developer (P-26), **I want** per-character graph instances to
   share graph definitions, **so that** memory overhead is minimized across many animated entities.

3. **US-9.4.2.1** -- **As a** character animator (P-11), **I want** transitions with configurable
   blend duration, blend curve, and per-bone blend profiles, **so that** state changes look natural
   and polished.

4. **US-9.4.2.2** -- **As a** character animator (P-11), **I want** sync markers in clips that align
   footfalls across source and destination states during transitions, **so that** foot contact
   timing is preserved.

5. **US-9.4.3.1** -- **As a** character animator (P-11), **I want** to encapsulate related states
   into reusable sub-state machines, **so that** complex behaviors are modular and shareable across
   character archetypes.

6. **US-9.4.3.2** -- **As a** technical artist (P-13), **I want** sub-state machines composable
   hierarchically, **so that** I can build layered animation systems from reusable building blocks.

## Layers, Variables, and Sync

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.4.4.1  | character animator (P-11)  |
| US-9.4.4.2  | engine developer (P-26)    |
| US-9.4.5.1  | character animator (P-11)  |
| US-9.4.5.2  | engine developer (P-26)    |
| US-9.4.6.1  | character animator (P-11)  |

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

5. **US-9.4.6.1** -- **As a** character animator (P-11), **I want** sync groups that keep walk and
   run cycles phase-synchronized regardless of playback rates, **so that** locomotion blends
   maintain consistent foot contact timing.

## Montages, Blend Spaces, and Integration

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.4.7.1   | character animator (P-11)  |
| US-9.4.7.2   | engine developer (P-26)    |
| US-9.4.8.1   | character animator (P-11)  |
| US-9.4.8.2   | technical artist (P-13)    |
| US-9.4.9.1   | character animator (P-11)  |
| US-9.4.10.1  | engine developer (P-26)    |
| US-9.4.10.2  | technical artist (P-13)    |

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

7. **US-9.4.10.2** -- **As a** technical artist (P-13), **I want** query APIs for current state,
   remaining clip time, root motion displacement, and montage status, **so that** gameplay systems
   synchronize with animation progress.
