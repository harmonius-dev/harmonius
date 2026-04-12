# User Stories -- 9.7 Motion Matching

## Motion Database and Querying

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.7.1.1  | character animator (P-11)  |
| US-9.7.1.2  | engine developer (P-26)    |
| US-9.7.2.1  | engine developer (P-26)    |
| US-9.7.2.2  | character animator (P-11)  |
| US-9.7.3.1  | engine developer (P-26)    |
| US-9.7.4.1  | engine developer (P-26)    |
| US-9.7.5.1  | engine developer (P-26)    |

1. **US-9.7.1.1** -- **As a** character animator (P-11), **I want** a motion database built from
   animation clips with per-frame feature vectors storing joint data and trajectory points,
   **so that** motion matching can select the best continuation for any pose.

2. **US-9.7.1.2** -- **As an** engine developer (P-26), **I want** databases built offline as
   compact binary assets with multiple databases per character type, **so that** database memory
   stays within platform budgets.

3. **US-9.7.2.1** -- **As an** engine developer (P-26), **I want** a runtime feature matching query
   that compares the current pose and desired trajectory against all database entries, **so that**
   the best-matching frame becomes the playback origin.

4. **US-9.7.2.2** -- **As a** character animator (P-11), **I want** configurable feature weights per
   character to prioritize trajectory vs. pose vs. foot contact, **so that** matching behavior is
   tunable per character type.

5. **US-9.7.3.1** -- **As an** engine developer (P-26), **I want** trajectory prediction from input
   direction, velocity, and turning rate with optional NavMesh clamping, **so that** predicted
   trajectories respect obstacles.

6. **US-9.7.4.1** -- **As an** engine developer (P-26), **I want** inertialization blending that
   decays pose offsets exponentially instead of cross-fading two clips, **so that** transitions cost
   half the evaluation and avoid blend artifacts.

7. **US-9.7.5.1** -- **As an** engine developer (P-26), **I want** KD-tree search acceleration for
   the motion database, **so that** nearest neighbor queries prune thousands of frames to a small
   candidate set efficiently.

## Tags, Integration, and Overrides

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.7.6.1  | character animator (P-11)  |
| US-9.7.6.2  | character animator (P-11)  |
| US-9.7.7.1  | character animator (P-11)  |
| US-9.7.7.2  | technical artist (P-13)    |
| US-9.7.8.1  | engine developer (P-26)    |
| US-9.7.9.1  | engine developer (P-26)    |

1. **US-9.7.6.1** -- **As a** character animator (P-11), **I want** to annotate database frames with
   tags for locomotion style, action type, and terrain context, **so that** matching queries can
   constrain or exclude frames by tag.

2. **US-9.7.6.2** -- **As a** character animator (P-11), **I want** to paint tag ranges in the
   animation editor and preview matching results in the viewport, **so that** I can verify database
   quality visually.

3. **US-9.7.7.1** -- **As a** character animator (P-11), **I want** motion matching exposed as a
   node in the animation state machine, **so that** I can mix motion-matched locomotion with
   hand-authored combat states.

4. **US-9.7.7.2** -- **As a** technical artist (P-13), **I want** transitions into and out of motion
   matching nodes to use inertialization, **so that** transitions between hand-authored and
   motion-matched states are seamless.

5. **US-9.7.8.1** -- **As an** engine developer (P-26), **I want** gameplay systems to force
   specific clips or poses overriding motion matching with priority levels, **so that** hit
   reactions, abilities, and death animations take precedence.

6. **US-9.7.9.1** -- **As an** engine developer (P-26), **I want** a post-process IK pass that locks
   planted feet during matching transitions, **so that** foot sliding from pose discontinuities is
   eliminated.

## Authoring and Performance

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.7.10.1  | character animator (P-11)  |
| US-9.7.10.2  | character animator (P-11)  |
| US-9.7.11.1  | engine developer (P-26)    |
| US-9.7.11.2  | technical artist (P-13)    |

1. **US-9.7.10.1** -- **As a** character animator (P-11), **I want** an editor workflow that imports
   mocap clips, auto-extracts feature vectors, and visualizes the feature space as a scatter plot,
   **so that** I can build and validate motion databases efficiently.

2. **US-9.7.10.2** -- **As a** character animator (P-11), **I want** to preview matching results in
   the viewport with trajectory visualization, **so that** I can verify database coverage before
   exporting.

3. **US-9.7.11.1** -- **As an** engine developer (P-26), **I want** motion matching LOD that
   decreases matching frequency and feature dimensions for distant characters, **so that** motion
   matching cost scales with screen importance.

4. **US-9.7.11.2** -- **As a** technical artist (P-13), **I want** off-screen characters to skip
   matching entirely and use a simple locomotion heuristic, **so that** CPU cost is zero for
   non-visible characters.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-9.7.1 | character animator (P-11) |
| US-9.7.10 | character animator (P-11) |
| US-9.7.11 | engine developer (P-26) |
| US-9.7.2 | engine developer (P-26) |
| US-9.7.3 | engine developer (P-26) |
| US-9.7.4 | engine developer (P-26) |
| US-9.7.5 | engine developer (P-26) |
| US-9.7.6 | character animator (P-11) |
| US-9.7.7 | character animator (P-11) |
| US-9.7.8 | engine developer (P-26) |
| US-9.7.9 | engine developer (P-26) |

1. **US-9.7.1** -- **As a** character animator (P-11), **I want** the capabilities defined in
   sub-stories US-9.7.1.1 through US-9.7.1.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-9.7.10** -- **As a** character animator (P-11), **I want** the capabilities defined in
   sub-stories US-9.7.10.1 through US-9.7.10.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-9.7.11** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-9.7.11.1 through US-9.7.11.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-9.7.2** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-9.7.2.1 through US-9.7.2.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

5. **US-9.7.3** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-9.7.3.1 through US-9.7.3.1 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

6. **US-9.7.4** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-9.7.4.1 through US-9.7.4.1 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

7. **US-9.7.5** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-9.7.5.1 through US-9.7.5.1 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

8. **US-9.7.6** -- **As a** character animator (P-11), **I want** the capabilities defined in
   sub-stories US-9.7.6.1 through US-9.7.6.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

9. **US-9.7.7** -- **As a** character animator (P-11), **I want** the capabilities defined in
   sub-stories US-9.7.7.1 through US-9.7.7.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

10. **US-9.7.8** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-9.7.8.1 through US-9.7.8.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

11. **US-9.7.9** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-9.7.9.1 through US-9.7.9.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
