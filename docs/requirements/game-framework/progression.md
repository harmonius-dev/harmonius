# R-13.12 — Character Progression Requirements

## Archetype Templates

1. **R-13.12.1** — The engine **SHALL** support data-driven archetype templates specifying stat
   modifiers, starting abilities, level-up unlocks, equipment restrictions, and cosmetic
   constraints, with archetype switching preserving previous ECS component history.
   - **Rationale:** Archetype templates enable character differentiation without code; state
     preservation enables respec mechanics.
   - **Verification:** Define two archetypes with different stat modifiers. Create a character with
     archetype A and verify modifiers apply. Switch to archetype B and verify A's progress is
     preserved and B's modifiers activate.

2. **R-13.12.2** — The engine **SHALL** support a prestige/rebirth system where max-progression
   entities restart at base level with configurable permanent bonuses accumulating across cycles.
   - **Rationale:** Prestige provides long-term goals beyond the progression cap.
   - **Verification:** Reach max level, trigger prestige, verify level resets to 1 with permanent
     bonus applied. Complete a second cycle and verify bonuses accumulate.

## DAG Progression Graph

3. **R-13.12.3** — The engine **SHALL** provide directed acyclic graph progression with nodes
   (passive bonuses, active abilities, ability modifications), prerequisite edges, tier gating, and
   point allocation with currency-cost respec.
   - **Rationale:** DAG graphs enable structured progression paths across all game genres.
   - **Verification:** Define a 3-tier tree with prerequisites. Allocate points and verify unlock
     order. Attempt skipping a prerequisite and verify rejection. Respec and verify all points
     refunded at currency cost.

4. **R-13.12.4** — The engine **SHALL** provide a visual graph editor for DAG progression trees with
   drag-and-drop node creation, edge drawing, and tier assignment.
   - **Rationale:** Visual editing enables designers to author progression without code.
   - **Verification:** Create a tree in the editor with 10 nodes across 3 tiers. Draw prerequisite
     edges. Export and load at runtime and verify the structure matches.

## Professions and Crafting

5. **R-13.12.5** — The engine **SHALL** support profession definitions with skill level, XP curve,
   recipe thresholds, and profession limits per entity, with gathering professions interacting with
   world resource nodes.
   - **Rationale:** Data-driven professions enable economy design without code.
   - **Verification:** Define a profession with 3 recipe thresholds. Gain XP and verify recipe
     unlock at each threshold. Verify profession limit prevents learning additional professions.

6. **R-13.12.6** — The engine **SHALL** support in-world crafting stations with quality tiers gating
   recipe access, displaying available recipes filtered by profession level with material counts and
   success probability.
   - **Rationale:** Station-gated crafting ties progression to world exploration.
   - **Verification:** Place a basic and master station. Verify basic station shows tier-1 recipes
     only. Verify master station shows all recipes. Attempt a craft with insufficient materials and
     verify rejection.

## Reputation and Achievements

7. **R-13.12.7** — The engine **SHALL** support per-faction reputation meters with data-driven
   tiered standings, asymmetric faction relationships, and standing thresholds gating vendors,
   quests, and zone access.
   - **Rationale:** Data-driven reputation drives content gating without hardcoded faction logic.
   - **Verification:** Define two rival factions. Gain reputation with faction A and verify faction
     B decreases. Cross a threshold and verify vendor access unlocks.

8. **R-13.12.8** — The engine **SHALL** support data-driven achievement definitions with trigger
   conditions (observer-based), incremental or boolean progress, point values, rewards, and platform
   achievement synchronization via platform services (F-14.5.1).
   - **Rationale:** Observer-based triggers integrate with the ECS without polling.
   - **Verification:** Define a "kill 10 enemies" achievement. Kill 10 and verify completion fires
     through the observer. Verify the platform achievement unlocks via the SDK.

## Item Enhancement

9. **R-13.12.9** — The engine **SHALL** support item enhancement with configurable success
   probability, failure consequences (no change, level decrease, destruction), protection items,
   rarity tiers with stat ranges, random affixes from pools, and affix re-rolling at currency cost.
   - **Rationale:** Configurable enhancement and affixes create item progression depth.
   - **Verification:** Enhance an item and verify success/failure follows configured probability.
     Verify protection item prevents destruction. Re-roll affixes and verify new values from the
     same pool.

10. **R-13.12.10** — The engine **SHALL** support equipment set definitions with escalating bonuses
    per equipped piece count, and per-item durability with configurable drain rates, repair costs,
    and non-functional state at zero durability.
    - **Rationale:** Set bonuses incentivize collecting complete sets; durability creates economic
      sinks.
    - **Verification:** Equip 2 of a 4-piece set and verify the 2-piece bonus activates. Equip 4 and
      verify the 4-piece bonus. Reduce durability to 0 and verify the item becomes non-functional.
      Repair and verify restoration.
