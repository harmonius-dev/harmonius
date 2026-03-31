# 9.7 — Motion Matching

## Motion Database and Querying

| ID      | Feature                      |
|---------|------------------------------|
| F-9.7.1 | Motion Database              |
| F-9.7.2 | Feature Matching Query       |
| F-9.7.3 | Trajectory Prediction        |
| F-9.7.4 | Inertialization Blending     |
| F-9.7.5 | KD-Tree Search Acceleration  |

1. **F-9.7.1** — Database of animation clips decomposed into per-frame entries. Each entry stores a
   feature vector containing joint positions, joint velocities, foot contact phases, and future
   trajectory points (position + facing at configurable lookahead times). The database is built
   offline from source animation clips and stored as a compact binary asset. Multiple databases can
   be assigned per character type (humanoid, quadruped, custom skeleton).
   - **Deps:** F-9.1.1 (Skeletal Animation), F-12.2.1 (Asset Processing)
   - **Platform:** All platforms. Database memory footprint scales with clip count; mobile budgets
     target under 8 MB per character.
2. **F-9.7.2** — At runtime, the system computes a query feature vector from the character's current
   pose (joint positions, velocities) and desired future trajectory (from F-9.7.3). A weighted
   distance metric compares the query against all database entries and selects the best match.
   Feature weights are configurable per character to prioritize trajectory vs. pose vs. foot
   contact. The selected frame becomes the playback origin.
   - **Deps:** F-9.7.1, F-9.7.3
   - **Platform:** All platforms. Distance computation is SIMD optimized. Mobile: reduced feature
     vector dimensions allowed.
3. **F-9.7.3** — Predicts the character's future trajectory from current input direction, velocity,
   acceleration, and turning rate. Trajectory points are sampled at configurable lookahead intervals
   (e.g., 0.2 s, 0.4 s, 0.8 s). The prediction accounts for obstacles via navmesh clamping (F-7.1.1)
   when enabled. Trajectory prediction updates every matching tick.
   - **Deps:** F-6.2.1 (Input Actions), F-4.1.8 (Character Controller)
   - **Platform:** All platforms. Navmesh clamping is optional on mobile to reduce query cost.
4. **F-9.7.4** — When switching from one matched clip to another, pose transitions use
   inertialization: the offset between the old and new pose decays exponentially over a configurable
   half-life rather than cross-fading between two simultaneous clip evaluations. This eliminates
   blend artifacts (doubled feet, averaged poses) and costs half the evaluation of a traditional
   cross-fade since only one clip plays at a time.
   - **Deps:** F-9.7.2, F-9.1.5 (Animation Blending)
   - **Platform:** All platforms. Inertialization is cheaper than cross-fading.
5. **F-9.7.5** — Spatial search acceleration for the motion database using a KD-tree (or VP-tree for
   non-Euclidean metrics). The tree is built offline alongside the database. At runtime, nearest
   neighbor queries prune the search space from thousands of frames to a small candidate set.
   Configurable search depth trades accuracy for speed.
   - **Deps:** F-9.7.1
   - **Platform:** All platforms. KD-tree traversal is cache friendly and branch-predictor friendly.

## Tags, Integration, and Overrides

| ID      | Feature                            |
|---------|------------------------------------|
| F-9.7.6 | Tag and Annotation System          |
| F-9.7.7 | State Machine Integration          |
| F-9.7.8 | Forced Transition Override         |
| F-9.7.9 | Foot Lock Post-Process             |

1. **F-9.7.6** — Frames in the motion database are annotated with designer-defined tags: locomotion
   style (walk, run, sprint), action type (idle, start, stop, turn, jump), terrain context (stairs,
   slope), and custom gameplay tags. Matching queries can constrain search to frames with required
   tags or exclude frames with forbidden tags. Tags are authored in the animation editor (F-15.4.1)
   and stored in the database asset.
   - **Deps:** F-9.7.1, F-15.4.1 (Animation Editor)
   - **Platform:** All platforms. Tag filtering is a bitmask check per candidate.
2. **F-9.7.7** — Motion matching is exposed as a node type within the animation state machine
   (F-9.4.1). A motion matching node replaces a traditional blend space or clip reference. The state
   machine can mix motion-matched states with hand-authored states (e.g., motion matching for
   locomotion, hand-authored for combat). Transitions into and out of motion matching nodes use
   inertialization (F-9.7.4).
   - **Deps:** F-9.4.1 (Animation State Machine), F-9.7.2, F-9.7.4
   - **Platform:** All platforms. No additional platform-specific considerations.
3. **F-9.7.8** — Gameplay systems can force the animation to a specific clip or pose, overriding
   motion matching. Forced transitions are used for hit reactions, ability casts, death animations,
   and other gameplay-driven moments. When the forced animation ends, control returns to motion
   matching via inertialization from the forced pose. Priority levels allow higher-priority forces
   (death) to interrupt lower-priority ones (flinch).
   - **Deps:** F-9.7.4, F-9.4.1 (Animation State Machine)
   - **Platform:** All platforms.
4. **F-9.7.9** — Post-process IK pass that locks planted feet to the ground during motion matching
   transitions. When a matched frame has a foot contact flag, the foot's world position is pinned
   and IK (F-9.3.1) adjusts the leg chain to maintain contact. The lock releases when the contact
   flag ends. This prevents foot sliding that would otherwise occur from pose discontinuities at
   match boundaries.
   - **Deps:** F-9.3.1 (IK Solvers), F-9.3.5 (Foot Placement), F-9.7.2
   - **Platform:** All platforms. IK cost is the same as existing foot placement.

## Authoring and Performance

| ID       | Feature                          |
|----------|----------------------------------|
| F-9.7.10 | Motion Database Authoring        |
| F-9.7.11 | Motion Matching LOD              |

1. **F-9.7.10** — Animation editor (F-15.4.1) workflow for building motion matching databases.
   Import mocap or hand-keyed clips, auto extract feature vectors from joint data, visualize the
   feature space as a 2D/3D scatter plot (PCA reduced), annotate tags by painting frame ranges,
   preview matching results in the viewport with trajectory visualization, and export the compiled
   database asset.
   - **Deps:** F-15.4.1 (Animation Editor), F-9.7.1, F-9.7.6
   - **Platform:** Editor only (desktop).
2. **F-9.7.11** — Reduce motion matching cost for distant or off-screen characters. LOD tiers
   decrease matching frequency (e.g., every frame at LOD 0, every 4th frame at LOD 1, every 8th at
   LOD 2) and optionally use reduced feature vector dimensions at lower LODs. Off-screen characters
   may skip matching entirely and use a simple locomotion heuristic until they become visible. LOD
   thresholds are configurable per character type.
   - **Deps:** F-9.7.2, F-9.7.5
   - **Platform:** All platforms. Critical for mobile and MMO scenes with many animated characters.
