# R-9.7 -- Motion Matching Requirements

## Motion Database and Querying

1. **R-9.7.1** -- The engine **SHALL** build offline motion databases from animation clips with
   per-frame feature vectors storing joint positions, velocities, foot contact phases, and future
   trajectory points, stored as compact binary assets.
   - **Rationale:** Pre-computed feature databases enable fast runtime nearest-neighbor search
     without per-frame feature extraction.
   - **Verification:** Build a database from 10 clips. Verify feature vectors are correctly
     extracted. Verify binary asset size stays within the 8 MB mobile budget.

2. **R-9.7.2** -- The engine **SHALL** compute a runtime query feature vector and select the
   best-matching database frame using a weighted distance metric with configurable feature weights.
   - **Rationale:** Weighted matching lets animators prioritize trajectory vs. pose vs. foot contact
     per character type.
   - **Verification:** Query with a known pose and trajectory. Verify the selected frame minimizes
     the weighted distance metric. Verify configurable weights alter selection.

3. **R-9.7.3** -- The engine **SHALL** predict the character's future trajectory from input
   direction, velocity, and turning rate, with optional NavMesh clamping (F-7.1.1).
   - **Rationale:** Trajectory prediction drives motion matching query quality; NavMesh clamping
     prevents predicted paths from crossing obstacles.
   - **Verification:** Predict a trajectory and verify it matches expected future positions. Enable
     NavMesh clamping and verify the trajectory avoids obstacles.

4. **R-9.7.4** -- The engine **SHALL** use inertialization blending for clip transitions, decaying
   pose offsets exponentially instead of cross-fading two simultaneous clips.
   - **Rationale:** Inertialization eliminates blend artifacts and costs half the evaluation of
     cross-fading since only one clip plays at a time.
   - **Verification:** Transition between two clips. Verify no doubled-feet or averaged-pose
     artifacts. Verify only one clip is evaluated per frame.

5. **R-9.7.5** -- The engine **SHALL** accelerate motion database search using a KD-tree built
   offline alongside the database.
   - **Rationale:** KD-tree pruning reduces search from thousands of frames to a small candidate
     set, making motion matching viable at scale.
   - **Verification:** Compare KD-tree search results against brute-force. Verify identical best
     matches. Benchmark and verify speedup scales with database size.

## Tags, Integration, and Overrides

1. **R-9.7.6** -- The engine **SHALL** annotate database frames with designer-defined tags and
   support tag-constrained matching queries.
   - **Rationale:** Tags enable matching queries that respect gameplay context like locomotion style
     and terrain type.
   - **Verification:** Tag frames as "run" and query with a "run" constraint. Verify only tagged
     frames are selected. Verify excluded tags are never matched.

2. **R-9.7.7** -- The engine **SHALL** expose motion matching as a node type within the animation
   state machine (F-9.4.1), using inertialization for transitions into and out of matching nodes.
   - **Rationale:** State machine integration lets animators mix motion-matched locomotion with
     hand-authored combat and interaction states.
   - **Verification:** Create a state machine with motion matching for locomotion and hand-authored
     combat. Verify transitions use inertialization. Verify no blend artifacts.

3. **R-9.7.8** -- The engine **SHALL** support forced transition overrides from gameplay systems
   with priority levels, returning to motion matching via inertialization when the forced animation
   ends.
   - **Rationale:** Hit reactions, abilities, and death must override motion matching with
     guaranteed playback.
   - **Verification:** Force a death animation and verify it plays to completion. Verify a lower-
     priority flinch is interrupted by a higher-priority death. Verify return to matching after a
     forced clip ends.

4. **R-9.7.9** -- The engine **SHALL** lock planted feet via IK during motion matching transitions
   when foot contact flags are active.
   - **Rationale:** Foot locking prevents sliding that occurs from pose discontinuities at match
     boundaries.
   - **Verification:** Transition between two clips with a foot planted. Verify the foot's world
     position does not slide. Verify the lock releases when the contact flag ends.

## Authoring and Performance

1. **R-9.7.10** -- The engine **SHALL** provide an editor workflow for importing clips,
   auto-extracting feature vectors, visualizing feature space, annotating tags, and previewing
   matching results.
   - **Rationale:** Efficient authoring tools are critical for building and validating motion
     databases from large mocap libraries.
   - **Verification:** Import 5 clips and verify auto-extracted features match manual computation.
     Visualize feature space and verify PCA scatter plot renders. Preview matching in viewport.

2. **R-9.7.11** -- The engine **SHALL** reduce motion matching cost for distant characters by
   decreasing matching frequency and feature dimensions per LOD tier, with off-screen characters
   skipping matching entirely.
   - **Rationale:** Motion matching LOD ensures cost scales with screen importance, critical for
     mobile and MMO scenes.
   - **Verification:** Place a character at LOD 2 and verify matching runs at reduced frequency.
     Move a character off-screen and verify zero matching cost.
