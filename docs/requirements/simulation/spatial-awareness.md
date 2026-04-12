# R-17.3 — Spatial Awareness Requirements

## Sense Definitions

1. **R-17.3.1** — The engine **SHALL** provide a `SenseDefinition` primitive as a named spatial
   query with shape, range, falloff curve, and filter tag set, data-driven from the editor.
   - **Rationale:** Sight, hearing, smell, and custom senses share a common spatial query structure;
     a single definition type avoids per-sense hardcoded systems.
   - **Verification:** Unit test: define a "sight" sense with a cone shape; query against targets;
     assert only targets within the cone return.
2. **R-17.3.2** — The engine **SHALL** provide 2D sense shapes (circle, cone, rectangle) for spatial
   awareness queries in 2D and 2.5D games using `Transform2D`.
   - **Rationale:** Top-down stealth games, 2D RTS, and platformers need awareness without paying
     the cost of 3D volumes.
   - **Verification:** Unit test: query a 2D cone sense; assert targets in the cone are returned and
     targets outside are not.

## Awareness State Machine

1. **R-17.3.3** — The engine **SHALL** track per-entity awareness state as a state machine with
   states Unaware, Suspicious, Alert, Tracking, and Lost, transitioning based on sense query results
   and elapsed time.
   - **Rationale:** Detection progression (enemies gradually becoming aware) is a universal
     stealth/AI pattern that must be engine-level, not per-game.
   - **Verification:** Unit test: start in Unaware; feed stimuli above the suspicion threshold;
     advance time; assert transitions through Suspicious to Alert in the configured time.
2. **R-17.3.4** — The engine **SHALL** emit `AwarenessTransitionEvent` through the ECS event channel
   on every state change so that UI, audio, and AI systems can react without polling.
   - **Rationale:** Event-driven integration decouples awareness from consumers; polling per frame
     wastes cycles.
   - **Verification:** Integration test: subscribe to the event channel; drive a state change;
     assert exactly one event fires with the old and new state.

## Spatial Queries

1. **R-17.3.5** — The engine **SHALL** run 100 concurrent awareness queries, each scanning up to
   1,000 candidate targets, within 2 ms per frame on desktop hardware by routing through the shared
   BVH spatial index (F-1.9.1).
   - **Rationale:** Crowds of AI entities each evaluating awareness must fit in the per-frame
     budget; reusing the shared spatial index avoids duplicate structures.
   - **Verification:** Benchmark: spawn 100 observers and 1,000 targets; run awareness queries;
     assert total time under 2 ms.
2. **R-17.3.6** — The engine **SHALL** provide selection queries (raycast, box select, nearest-N)
   routing through the shared spatial index for player-facing picking with latency under 0.5 ms per
   query batch of up to 50 picks.
   - **Rationale:** Player input (click to select, box drag, snap-to-nearest) must feel
     instantaneous and share infrastructure with AI awareness.
   - **Verification:** Benchmark: issue 50 concurrent raycast picks on a scene of 10,000 entities;
     assert batch under 0.5 ms.

## GPU Scale

1. **R-17.3.7** — The engine **SHALL** support GPU compute spatial awareness evaluation for
   ultra-scale scenarios (one million-plus entities) with one-frame latency readback and hybrid
   CPU/GPU threshold selection based on entity count.
   - **Rationale:** Massive RTS battles and simulation games exceed CPU throughput; GPU compute
     scales to millions of entities with acceptable latency.
   - **Verification:** Integration test: spawn 1,000,000 entities; run GPU awareness; assert results
     match CPU baseline within tolerance and frame rate stays above 30 fps.

## Visual Indicators

1. **R-17.3.8** — The engine **SHALL** spawn awareness-driven 3D visual indicators (detection state
   icons, threat direction arrows, stealth meters) on entities in response to
   `AwarenessTransitionEvent` emissions.
   - **Rationale:** Players need clear feedback on enemy detection state; indicator lifecycle must
     be declarative and engine-managed.
   - **Verification:** Integration test: drive an entity into Alert; assert a detection icon
     indicator entity spawns as child of the target.
2. **R-17.3.9** — The engine **SHALL** render sense volumes (vision cones, hearing spheres) and
   awareness state icons as debug gizmos in the editor viewport, filtered by render layer bitmask.
   - **Rationale:** Debugging AI perception requires visualizing query shapes; layer filtering
     prevents debug clutter from overwhelming the viewport.
   - **Verification:** Manual test in editor: enable the sense debug layer; assert vision cones
     render around AI entities. Disable; assert cones hidden.

## Latency

1. **R-17.3.10** — The engine **SHALL** report awareness state changes within one frame of the
   triggering stimulus being observable in the shared spatial index.
   - **Rationale:** Stale awareness feels unresponsive; latency must be bounded to a single frame
     for all cases except GPU-evaluated ultra-scale.
   - **Verification:** Integration test: spawn a target within sense range; assert
     AwarenessTransitionEvent fires within one frame of the target becoming observable.
