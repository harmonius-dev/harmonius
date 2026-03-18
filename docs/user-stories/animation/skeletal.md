# User Stories -- 9.1 Skeletal Animation

## F-9.1.1

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.1.1.1 | engine developer (P-26)   | F-9.1.1  | R-9.1.1      |
| US-9.1.1.2 | character animator (P-11) | F-9.1.1  | R-9.1.1      |
| US-9.1.1.3 | engine tester (P-27)      | F-9.1.1  | R-9.1.1      |

1. **US-9.1.1.1** — I want GPU compute skinning that transforms vertices by weighted bone matrices
   using linear blend and dual-quaternion modes
   - **Acceptance:** thousands of skeleton instances are evaluated in a single dispatch without CPU
     bottlenecks
2. **US-9.1.1.2** — I want toggle between linear blend and dual-quaternion skinning per skeleton and
   see results in the viewport
   - **Acceptance:** I can fix candy-wrapping artifacts at twist joints without a code change
3. **US-9.1.1.3** — I want confirm that mobile supports 32-64 bones, Switch supports 96, and desktop
   supports 256+ bones per skeleton
   - **Acceptance:** skeleton complexity respects per-platform GPU budgets

## F-9.1.2

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.1.2.1 | engine developer (P-26)   | F-9.1.2  | R-9.1.2      |
| US-9.1.2.2 | character animator (P-11) | F-9.1.2  | R-9.1.2      |

1. **US-9.1.2.1** — I want GPU keyframe evaluation using Hermite interpolation for translation,
   rotation, and scale tracks in a single compute dispatch
   - **Acceptance:** animation curve sampling does not require CPU-side per-joint evaluation
2. **US-9.1.2.2** — I want select loop, clamp, or ping-pong playback mode per clip and preview the
   result in the animation editor
   - **Acceptance:** I can author cyclic walk loops and one-shot attack clips with correct end
     behavior

## F-9.1.3

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.1.3.1 | character animator (P-11) | F-9.1.3  | R-9.1.3      |
| US-9.1.3.2 | engine tester (P-27)      | F-9.1.3  | R-9.1.3      |

1. **US-9.1.3.1** — I want linear and cubic Hermite blending between up to 8 simultaneous clips
   - **Acceptance:** complex locomotion blends (walk + strafe + lean) transition smoothly without
     popping
2. **US-9.1.3.2** — I want verify that mobile supports 4 simultaneous blend clips, Switch supports
   6, and desktop supports 8
   - **Acceptance:** blend complexity scales within per-platform GPU budgets

## F-9.1.4

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.1.4.1 | character animator (P-11) | F-9.1.4  | R-9.1.4      |
| US-9.1.4.2 | game developer (P-15)     | F-9.1.4  | R-9.1.4      |
| US-9.1.4.3 | engine tester (P-27)      | F-9.1.4  | R-9.1.4      |

1. **US-9.1.4.1** — I want animation layers with per-bone masks so that upper-body combat animations
   overlay on lower-body locomotion
   - **Acceptance:** a character can swing a sword while running without authoring separate
     combo-run clips
2. **US-9.1.4.2** — I want additive animation layers that encode pose deltas relative to a reference
   pose
   - **Acceptance:** hit reactions, breathing, and weapon sway composite on top of any base
     animation without per-combination authoring
3. **US-9.1.4.3** — I want verify that mobile supports 2 active animation layers, Switch supports 3,
   and desktop supports 4+
   - **Acceptance:** additive layers do not exceed per-platform evaluation budgets

## F-9.1.5

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-9.1.5.1 | game developer (P-15)   | F-9.1.5  | R-9.1.5      |
| US-9.1.5.2 | engine developer (P-26) | F-9.1.5  | R-9.1.5      |

1. **US-9.1.5.1** — I want instanced skeletal animation that batch-evaluates thousands of skeleton
   instances in a single compute dispatch using an arena buffer
   - **Acceptance:** MMO city hubs render with full animation quality at scale
2. **US-9.1.5.2** — I want benchmark instanced skeletal animation at 50 (mobile), 200 (Switch), and
   500+ (desktop) simultaneous instances
   - **Acceptance:** I can confirm GPU compute time stays within 1ms per platform tier

## F-9.1.6

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.1.6.1 | character animator (P-11) | F-9.1.6  | R-9.1.6      |
| US-9.1.6.2 | engine tester (P-27)      | F-9.1.6  | R-9.1.6      |

1. **US-9.1.6.1** — I want root motion extraction to apply root bone translation and rotation deltas
   to the character's world transform via physics
   - **Acceptance:** authored locomotion clips (dodges, vaults, mounts) drive gameplay movement
     while maintaining capsule collision
2. **US-9.1.6.2** — I want play a root-motion vault clip and verify that the character's capsule
   follows the root bone path while physics collision is maintained
   - **Acceptance:** root motion and gameplay movement integrate correctly

## F-9.1.7

| ID         | Persona               | Features | Requirements |
|------------|-----------------------|----------|--------------|
| US-9.1.7.1 | game developer (P-15) | F-9.1.7  | R-9.1.7      |
| US-9.1.7.2 | engine tester (P-27)  | F-9.1.7  | R-9.1.7      |

1. **US-9.1.7.1** — I want animation compression achieving 10:1+ ratios using smallest-three
   quaternion encoding and range-reduced fixed-point values
   - **Acceptance:** thousands of unique emote, combat, and mount clips can be streamed without
     exhausting memory
2. **US-9.1.7.2** — I want compare compressed versus uncompressed animation playback for locomotion,
   facial, and combat clips, verifying that visual quality loss is imperceptible
   - **Acceptance:** compression does not introduce visible artifacts

## F-9.1.8

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.1.8.1 | character animator (P-11) | F-9.1.8  | R-9.1.8      |
| US-9.1.8.2 | character animator (P-11) | F-9.1.8  | R-9.1.8      |
| US-9.1.8.3 | engine tester (P-27)      | F-9.1.8  | R-9.1.8      |

1. **US-9.1.8.1** — I want retarget animation clips between skeletons with different bone counts and
   proportions using a shared canonical pose and per-bone mode (direct copy, scaled translation,
   rotation only)
   - **Acceptance:** human mocap data works on non-human characters
2. **US-9.1.8.2** — I want define bone correspondence and per-bone retargeting modes visually in the
   editor rather than through code
   - **Acceptance:** cross-skeleton retargeting is accessible to animators without programmer
     support
3. **US-9.1.8.3** — I want retarget a human walk cycle onto a quadruped with manual bone remapping
   and verify that the output is free of NaN transforms and visible deformation errors
   - **Acceptance:** cross-species retargeting produces usable results

## F-9.1.9

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.1.9.1 | character animator (P-11) | F-9.1.9  | R-9.1.9      |
| US-9.1.9.2 | game developer (P-15)     | F-9.1.9  | R-9.1.9      |
| US-9.1.9.3 | engine tester (P-27)      | F-9.1.9  | R-9.1.9      |

1. **US-9.1.9.1** — I want place named events at specific frames in the animation editor timeline
   for footstep sounds, hit windows, VFX spawns, and weapon trails
   - **Acceptance:** gameplay systems react to animation timing without hardcoded frame numbers
2. **US-9.1.9.2** — I want animation events to dispatch as typed event components through the ECS
   observer system
   - **Acceptance:** gameplay systems can subscribe to animation timing events with type-safe
     handler functions
3. **US-9.1.9.3** — I want verify that window events (active for a frame range) fire correctly
   during the specified range and do not fire outside it
   - **Acceptance:** hit detection windows and weapon trail durations are frame-accurate

## F-9.1.10

| ID          | Persona                   | Features | Requirements |
|-------------|---------------------------|----------|--------------|
| US-9.1.10.1 | game developer (P-15)     | F-9.1.10 | R-9.1.10     |
| US-9.1.10.2 | character animator (P-11) | F-9.1.10 | R-9.1.10     |
| US-9.1.10.3 | engine tester (P-27)      | F-9.1.10 | R-9.1.10     |

1. **US-9.1.10.1** — I want animation LOD to automatically transition distant characters through
   reduced bone sets, lower update rates, and VAT playback
   - **Acceptance:** MMO scenes with hundreds of visible characters maintain target frame time
2. **US-9.1.10.2** — I want a per-entity LOD bias that keeps hero characters at full animation
   fidelity regardless of distance
   - **Acceptance:** the player character and key NPCs never show reduced animation quality
3. **US-9.1.10.3** — I want verify that mobile uses more aggressive LOD thresholds (earlier
   transitions, shorter VAT distances) than desktop
   - **Acceptance:** animation cost scales with platform capability
