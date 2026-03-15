# R-13.12 — Character Progression Requirements

## Race and Class

### R-13.12.1a Data-Driven Race Definitions

The engine **SHALL** define races as data-driven assets specifying stat modifiers, racial abilities,
cosmetic constraints, and lore text, authored in the visual editor and gameplay databases.

- **Derived from:** [F-13.12.1a](../../features/game-framework/progression.md)
- **Rationale:** Data-driven race definitions enable designers to create and balance racial
  archetypes without code changes.
- **Verification:** Define a race with +10 strength and cosmetic constraints. Create a character
  with that race and verify the stat modifier applies and only allowed cosmetics are available.

### R-13.12.1b Data-Driven Class Definitions

The engine **SHALL** define classes as data-driven assets specifying starting ability sets, level-up
unlocks, equipment restrictions, and class-specific resources, authored in the visual editor and
gameplay databases.

- **Derived from:** [F-13.12.1b](../../features/game-framework/progression.md)
- **Rationale:** Data-driven class definitions enable designers to iterate on class archetypes and
  balance ability progression without code changes.
- **Verification:** Define a class with a starting ability set and mana resource. Create a character
  with that class; verify the ability set is available and mana is tracked.

### R-13.12.1c Multi-Class Switching and Hybrid Classes

The engine **SHALL** support class switching at designated NPCs and hybrid classes that combine
abilities from two parent classes when prerequisite conditions are met.

- **Derived from:** [F-13.12.1c](../../features/game-framework/progression.md)
- **Rationale:** Multi-class and hybrid support covers the progression patterns expected across RPG
  sub-genres without forcing a single-class model.
- **Verification:** Switch classes at an NPC and verify the old ability set is removed and the new
  one applied. Configure a hybrid class with two parent prerequisites; verify it unlocks only when
  both are met.

### R-13.12.1d Prestige and Rebirth System

The engine **SHALL** allow max-level characters to restart at level 1 with configurable permanent
bonuses that accumulate across prestige cycles.

- **Derived from:** [F-13.12.1d](../../features/game-framework/progression.md)
- **Rationale:** Prestige systems extend end-game engagement by rewarding repeated progression with
  permanent power growth.
- **Verification:** Reach max level, trigger prestige, and verify the character restarts at level 1
  with the configured permanent bonus applied.

## Talent Trees

### R-13.12.2a Talent Tree Data Model

The engine **SHALL** define talent trees as DAGs with typed nodes (passive bonuses, active
abilities, ability modifications), prerequisite edges enforcing unlock order, and tier gating by
total points spent.

- **Derived from:** [F-13.12.2a](../../features/game-framework/progression.md)
- **Rationale:** DAG-based talent trees with tier gating create meaningful progression choices and
  prevent sequence-breaking.
- **Verification:** Define a talent tree with 3 tiers and prerequisite edges. Attempt to allocate a
  tier-2 node without sufficient total points; verify rejection. Allocate the tier-1 prerequisite
  and then the tier-2 node; verify success.

### R-13.12.2b Talent Allocation and Respec

The engine **SHALL** support point allocation per level with prerequisite and tier-gating
validation, and respec for currency cost that resets all allocations and refunds points.

- **Derived from:** [F-13.12.2b](../../features/game-framework/progression.md)
- **Rationale:** Respec allows players to experiment without permanent commitment while currency
  cost prevents trivial reallocation.
- **Verification:** Allocate a passive stat bonus node and verify the stat increases. Respec for
  currency cost and verify all allocations are reset and points refunded.

### R-13.12.2c Talent Tree Visual Editor

The engine **SHALL** provide a visual editor for authoring talent trees as graph assets with node
creation, edge drawing, tier lane assignment, and prerequisite chain preview.

- **Derived from:** [F-13.12.2c](../../features/game-framework/progression.md)
- **Rationale:** Visual authoring of DAG-based talent trees is essential for designers to build and
  iterate on talent structures without manual data entry.
- **Verification:** Author a talent tree in the visual editor. Export the graph asset and load it at
  runtime. Verify all nodes, edges, and tier assignments match the authored layout.

## Professions

### R-13.12.3a Profession Data Model

The engine **SHALL** define professions with independent skill levels, XP curves, recipe threshold
unlocks, and configurable profession slot limits, where XP is earned from crafts, gathers, and quest
turn-ins.

- **Derived from:** [F-13.12.3a](../../features/game-framework/progression.md)
- **Rationale:** Independent profession leveling with configurable XP curves and recipe thresholds
  gives designers full control over trade skill pacing.
- **Verification:** Learn a profession and verify XP is tracked independently. Level up the
  profession and verify a new recipe unlocks at the threshold. Attempt to learn a profession beyond
  the slot limit and verify rejection.

### R-13.12.3b Gathering Profession Integration

The engine **SHALL** integrate gathering professions with world resource nodes so that gathering
skill level determines yield quantity and rare proc rate.

- **Derived from:** [F-13.12.3b](../../features/game-framework/progression.md)
- **Rationale:** Skill-scaled yields reward profession investment and create meaningful progression
  for gathering-focused players.
- **Verification:** Interact with a resource node at two different gathering skill levels and verify
  that higher skill produces greater yield quantities and higher rare proc rates.

### R-13.12.3c Crafting Profession Integration

The engine **SHALL** integrate crafting professions with the recipe system so that recipe
availability is gated by profession level and successful crafts award profession XP.

- **Derived from:** [F-13.12.3c](../../features/game-framework/progression.md)
- **Rationale:** Level-gated recipes create crafting progression depth and ensure players advance
  through the profession before accessing high-tier outputs.
- **Verification:** Complete a craft using gathered materials and verify profession XP is awarded.
  Attempt to access a recipe above the current profession level and verify it is locked.

### R-13.12.4 Crafting Station Gating

The engine **SHALL** gate recipe access by crafting station type and quality tier, display available
recipes filtered by profession level with material counts and success probability, and support
station placement in player housing and fixed world locations.

- **Derived from:** [F-13.12.4](../../features/game-framework/progression.md)
- **Rationale:** Station-gated recipes create spatial gameplay incentives (visiting specific
  locations) and tiered stations provide progression depth beyond character level alone.
- **Verification:** Interact with a basic forge and verify only basic blacksmithing recipes appear.
  Interact with a master forge and verify higher-tier recipes are also available. Attempt to access
  alchemy recipes at a forge and verify they are absent. Place a crafting station in player housing
  and verify it functions identically to a world station. Verify the crafting UI shows material
  counts from inventory and the success probability.

## Reputation

### R-13.12.5 Faction Reputation with Tiered Standing

The engine **SHALL** track per-character, per-faction reputation with data-driven standing tiers
that gate vendor access, quest availability, zone access, and cosmetic rewards, supporting
asymmetric faction relationships where gaining reputation with one faction may reduce another.

- **Derived from:** [F-13.12.5](../../features/game-framework/progression.md)
- **Rationale:** Tiered reputation with gated rewards provides long-term engagement goals, and
  asymmetric faction relationships create meaningful faction choice consequences.
- **Verification:** Complete a quest for faction A and verify reputation increases. Verify
  reputation with rival faction B decreases if asymmetric loss is configured. Reach "honored"
  standing and verify the faction vendor unlocks exclusive items. Reach "hostile" standing with an
  enemy faction and verify zone entry triggers hostile NPC behavior. Verify standing tier thresholds
  and names are fully configurable in gameplay databases.

## Achievements

### R-13.12.6a Achievement Definition and Tracking

The engine **SHALL** fire achievements through the ECS observer system when data-driven trigger
conditions are met, tracking incremental or boolean progress and recording completion timestamps.

- **Derived from:** [F-13.12.6a](../../features/game-framework/progression.md)
- **Rationale:** Observer-driven achievements decouple tracking logic from gameplay systems and
  enable event-driven completion without polling.
- **Verification:** Define an achievement "Kill 100 Enemies" with incremental tracking. Kill 50
  enemies and verify progress is 50/100. Kill 50 more and verify the achievement fires and the
  completion timestamp is recorded. Define a hidden achievement and verify it is not visible until
  completed.

### R-13.12.6b Achievement Rewards and Display

The engine **SHALL** award configurable rewards (items, titles, currency, cosmetics) on achievement
completion, display a notification popup, and accumulate achievement points into a player score.

- **Derived from:** [F-13.12.6b](../../features/game-framework/progression.md)
- **Rationale:** Visible rewards and score accumulation incentivize achievement pursuit and provide
  tangible recognition of player accomplishments.
- **Verification:** Complete an achievement and verify the configured reward is granted, a
  notification popup displays, and the achievement points increase by the configured value.

### R-13.12.6c Platform Achievement Sync

The engine **SHALL** sync achievements with platform APIs (Steam, PlayStation, Xbox) by mapping
engine achievement IDs to platform-specific achievement identifiers.

- **Derived from:** [F-13.12.6c](../../features/game-framework/progression.md)
- **Rationale:** Platform API sync ensures achievements appear in the player's platform profile and
  contribute to platform-level gamerscore/trophy lists.
- **Verification:** Complete an achievement and verify it syncs to the platform API (Steam
  Achievement unlocked). Verify the platform achievement ID mapping is correct.

## Item Enhancement

### R-13.12.7 Item Enhancement with Success/Failure Probability

The engine **SHALL** enhance equipment through configurable enhancement levels (e.g., +0 to +15)
with per-level stat bonuses, material costs, decreasing success probability, configurable failure
consequences (no change, level decrease, or destruction), and protection items that prevent
destruction.

- **Derived from:** [F-13.12.7](../../features/game-framework/progression.md)
- **Rationale:** Probabilistic enhancement with failure consequences creates high-stakes progression
  decisions, and protection items provide a risk-mitigation mechanism that can be monetized or
  earned through gameplay.
- **Verification:** Enhance an item from +0 to +1 with 100% success rate; verify stat bonus applies.
  Enhance from +14 to +15 with 10% success rate; on failure, verify the configured consequence
  occurs (level decrease or no change). Use a protection item on a +14 enhance attempt; on failure,
  verify the item is not destroyed. Verify the item name displays the enhancement level (e.g., "+7
  Sword of Fire").

### R-13.12.8a Item Rarity Tier System

The engine **SHALL** assign item rarity tiers (common through mythic) with color-coded display and
randomize base stats within per-rarity ranges on drop.

- **Derived from:** [F-13.12.8a](../../features/game-framework/progression.md)
- **Rationale:** Rarity tiers with bounded stat ranges create item diversity and hunt motivation
  while keeping drops within designer-controlled bounds.
- **Verification:** Drop a rare sword 100 times and verify damage rolls fall within the configured
  50-70 range. Drop an epic sword and verify 65-90 range. Verify rarity tier colors display
  correctly in the UI.

### R-13.12.8b Affix System

The engine **SHALL** apply random prefix/suffix affixes from configurable affix pools, scaling the
number of affixes with item rarity.

- **Derived from:** [F-13.12.8b](../../features/game-framework/progression.md)
- **Rationale:** Random affixes create combinatorial item variety beyond base stats, increasing loot
  diversity and replayability.
- **Verification:** Verify each drop receives the correct number of random affixes for its rarity.
  Verify affixes are drawn from the configured pool and grant the expected stat bonuses.

### R-13.12.8c Stat Re-Rolling Mechanics

The engine **SHALL** support affix re-rolling for currency cost, preserving the base item and rarity
tier while re-randomizing affixes from the same pool.

- **Derived from:** [F-13.12.8c](../../features/game-framework/progression.md)
- **Rationale:** Re-rolling provides a controlled path to optimal stats without requiring repeated
  drops of the same base item.
- **Verification:** Re-roll an item's affixes for currency and verify the base item is preserved
  while affixes change. Verify the currency cost is deducted correctly.

### R-13.12.9 Equipment Set Bonuses

The engine **SHALL** apply escalating gameplay effects when multiple items from a defined equipment
set are equipped, displaying set progress (collected vs. missing pieces) in the character equipment
panel.

- **Derived from:** [F-13.12.9](../../features/game-framework/progression.md)
- **Rationale:** Set bonuses incentivize collecting complete equipment sets and create build
  diversity through set-specific proc effects and ability unlocks.
- **Verification:** Define a 6-piece set with bonuses at 2, 4, and 6 pieces. Equip 1 piece and
  verify no bonus. Equip 2 pieces and verify the 2-piece bonus effect is applied. Equip 4 pieces and
  verify the 4-piece bonus is added. Unequip 1 piece (back to 3) and verify the 4-piece bonus is
  removed. Verify the equipment panel shows 3/6 collected with missing pieces listed.

### R-13.12.10 Item Durability and Repair

The engine **SHALL** track per-item durability as a percentage that decreases on damage dealt,
damage taken, and death, rendering items non-functional at 0% without destroying them, with repair
available at NPC vendors (cost scaling with item level and rarity) or via consumable repair kits.

- **Derived from:** [F-13.12.10](../../features/game-framework/progression.md)
- **Rationale:** Durability creates an ongoing resource sink that drives economy engagement (repair
  costs, repair kit crafting) without permanent item loss.
- **Verification:** Attack with a weapon and verify durability decreases. Take damage and verify
  armor durability decreases. Reduce weapon durability to 0% and verify it deals no damage. Repair
  at an NPC vendor and verify durability restores to 100% with cost scaling by item level. Use a
  repair kit and verify partial durability restoration. Verify warnings display at 25% and 10%
  thresholds.

## Non-Functional Requirements

### R-13.12.NF1 Talent Tree Evaluation Latency

The engine **SHALL** evaluate talent tree allocation validation (prerequisite checks, tier gating,
point totals) within 1 ms per allocation operation, ensuring responsive UI feedback during talent
point spending.

- **Derived from:** F-13.12.2
- **Rationale:** Talent allocation is a frequent interaction during leveling; perceptible delay in
  validation feedback breaks the reward loop of spending newly earned points.
- **Verification:** Define a talent tree with 100 nodes across 5 tiers. Allocate all points
  sequentially, measuring per-allocation validation time. Verify the 99th percentile is under 1 ms.

### R-13.12.NF2 Achievement Tracking Overhead

The engine **SHALL** track at least 1,000 achievement definitions with incremental progress counters
without exceeding 0.1 ms per frame of observer evaluation overhead.

- **Derived from:** F-13.12.6
- **Rationale:** Achievement tracking runs every frame for event-driven achievements; excessive
  observer overhead at scale would consume frame budget better spent on gameplay systems.
- **Verification:** Register 1,000 achievements with varied trigger conditions. Generate 100
  gameplay events per frame. Measure total observer evaluation time and verify it stays under 0.1 ms
  per frame.
