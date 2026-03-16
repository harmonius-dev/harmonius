# User Stories -- 9.1 Skeletal Animation

## F-9.1.1

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.1.1.1 | engine developer (P-26) | I want GPU compute skinning that transforms vertices by weighted bone matrices using linear blend and dual-quaternion modes | thousands of skeleton instances are evaluated in a single dispatch without CPU bottlenecks | F-9.1.1 | R-9.1.1 |
| US-9.1.1.2 | character animator (P-11) | I want toggle between linear blend and dual-quaternion skinning per skeleton and see results in the viewport | I can fix candy-wrapping artifacts at twist joints without a code change | F-9.1.1 | R-9.1.1 |
| US-9.1.1.3 | engine tester (P-27) | I want confirm that mobile supports 32-64 bones, Switch supports 96, and desktop supports 256+ bones per skeleton | skeleton complexity respects per-platform GPU budgets | F-9.1.1 | R-9.1.1 |

## F-9.1.2

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.1.2.1 | engine developer (P-26) | I want GPU keyframe evaluation using Hermite interpolation for translation, rotation, and scale tracks in a single compute dispatch | animation curve sampling does not require CPU-side per-joint evaluation | F-9.1.2 | R-9.1.2 |
| US-9.1.2.2 | character animator (P-11) | I want select loop, clamp, or ping-pong playback mode per clip and preview the result in the animation editor | I can author cyclic walk loops and one-shot attack clips with correct end behavior | F-9.1.2 | R-9.1.2 |

## F-9.1.3

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.1.3.1 | character animator (P-11) | I want linear and cubic Hermite blending between up to 8 simultaneous clips | complex locomotion blends (walk + strafe + lean) transition smoothly without popping | F-9.1.3 | R-9.1.3 |
| US-9.1.3.2 | engine tester (P-27) | I want verify that mobile supports 4 simultaneous blend clips, Switch supports 6, and desktop supports 8 | blend complexity scales within per-platform GPU budgets | F-9.1.3 | R-9.1.3 |

## F-9.1.4

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.1.4.1 | character animator (P-11) | I want animation layers with per-bone masks so that upper-body combat animations overlay on lower-body locomotion | a character can swing a sword while running without authoring separate combo-run clips | F-9.1.4 | R-9.1.4 |
| US-9.1.4.2 | game developer (P-15) | I want additive animation layers that encode pose deltas relative to a reference pose | hit reactions, breathing, and weapon sway composite on top of any base animation without per-combination authoring | F-9.1.4 | R-9.1.4 |
| US-9.1.4.3 | engine tester (P-27) | I want verify that mobile supports 2 active animation layers, Switch supports 3, and desktop supports 4+ | additive layers do not exceed per-platform evaluation budgets | F-9.1.4 | R-9.1.4 |

## F-9.1.5

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.1.5.1 | game developer (P-15) | I want instanced skeletal animation that batch-evaluates thousands of skeleton instances in a single compute dispatch using an arena buffer | MMO city hubs render with full animation quality at scale | F-9.1.5 | R-9.1.5 |
| US-9.1.5.2 | engine developer (P-26) | I want benchmark instanced skeletal animation at 50 (mobile), 200 (Switch), and 500+ (desktop) simultaneous instances | I can confirm GPU compute time stays within 1ms per platform tier | F-9.1.5 | R-9.1.5 |

## F-9.1.6

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.1.6.1 | character animator (P-11) | I want root motion extraction to apply root bone translation and rotation deltas to the character's world transform via physics | authored locomotion clips (dodges, vaults, mounts) drive gameplay movement while maintaining capsule collision | F-9.1.6 | R-9.1.6 |
| US-9.1.6.2 | engine tester (P-27) | I want play a root-motion vault clip and verify that the character's capsule follows the root bone path while physics collision is maintained | root motion and gameplay movement integrate correctly | F-9.1.6 | R-9.1.6 |

## F-9.1.7

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.1.7.1 | game developer (P-15) | I want animation compression achieving 10:1+ ratios using smallest-three quaternion encoding and range-reduced fixed-point values | thousands of unique emote, combat, and mount clips can be streamed without exhausting memory | F-9.1.7 | R-9.1.7 |
| US-9.1.7.2 | engine tester (P-27) | I want compare compressed versus uncompressed animation playback for locomotion, facial, and combat clips, verifying that visual quality loss is imperceptible | compression does not introduce visible artifacts | F-9.1.7 | R-9.1.7 |

## F-9.1.8

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.1.8.1 | character animator (P-11) | I want retarget animation clips between skeletons with different bone counts and proportions using a shared canonical pose and per-bone mode (direct copy, scaled translation, rotation only) | human mocap data works on non-human characters | F-9.1.8 | R-9.1.8 |
| US-9.1.8.2 | character animator (P-11) | I want define bone correspondence and per-bone retargeting modes visually in the editor rather than through code | cross-skeleton retargeting is accessible to animators without programmer support | F-9.1.8 | R-9.1.8 |
| US-9.1.8.3 | engine tester (P-27) | I want retarget a human walk cycle onto a quadruped with manual bone remapping and verify that the output is free of NaN transforms and visible deformation errors | cross-species retargeting produces usable results | F-9.1.8 | R-9.1.8 |

## F-9.1.9

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.1.9.1 | character animator (P-11) | I want place named events at specific frames in the animation editor timeline for footstep sounds, hit windows, VFX spawns, and weapon trails | gameplay systems react to animation timing without hardcoded frame numbers | F-9.1.9 | R-9.1.9 |
| US-9.1.9.2 | game developer (P-15) | I want animation events to dispatch as typed event components through the ECS observer system | gameplay systems can subscribe to animation timing events with type-safe handler functions | F-9.1.9 | R-9.1.9 |
| US-9.1.9.3 | engine tester (P-27) | I want verify that window events (active for a frame range) fire correctly during the specified range and do not fire outside it | hit detection windows and weapon trail durations are frame-accurate | F-9.1.9 | R-9.1.9 |

## F-9.1.10

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.1.10.1 | game developer (P-15) | I want animation LOD to automatically transition distant characters through reduced bone sets, lower update rates, and VAT playback | MMO scenes with hundreds of visible characters maintain target frame time | F-9.1.10 | R-9.1.10 |
| US-9.1.10.2 | character animator (P-11) | I want a per-entity LOD bias that keeps hero characters at full animation fidelity regardless of distance | the player character and key NPCs never show reduced animation quality | F-9.1.10 | R-9.1.10 |
| US-9.1.10.3 | engine tester (P-27) | I want verify that mobile uses more aggressive LOD thresholds (earlier transitions, shorter VAT distances) than desktop | animation cost scales with platform capability | F-9.1.10 | R-9.1.10 |
