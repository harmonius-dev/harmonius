# Building and Survival Systems Test Cases

Companion test cases for [building-survival.md](building-survival.md).

## Unit Tests

### TC-13.14.1.1 Socket Snap Valid

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |

1. **#1** — Place wall next to foundation with matching socket
   - **Expected:** Ghost snaps to wall socket at 90-degree increment

### TC-13.14.1.2 Socket Snap Invalid

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |

1. **#1** — Place wall with no nearby sockets
   - **Expected:** `PlacementError::NoValidSocket`; ghost shows red

### TC-13.14.1.3 Freeform Ground Align

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |

1. **#1** — Free-form place on 15-degree slope
   - **Expected:** Piece aligned to ground normal; no float or clip

### TC-13.14.1.4 Placement Collision

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |

1. **#1** — Place piece overlapping existing piece
   - **Expected:** `PlacementError::CollisionDetected`

### TC-13.14.1.5 Placement Zone Restrict

| # | Requirement |
|---|-------------|
| 1 | R-13.14.1   |

1. **#1** — Place in restricted zone
   - **Expected:** `PlacementError::RestrictedZone`

### TC-13.14.2.1 Construction Progress

| # | Requirement |
|---|-------------|
| 1 | R-13.14.2   |
| 2 | R-13.14.2   |
| 3 | R-13.14.2   |

1. **#1** — Place scaffold; advance to 33%
   - **Expected:** Visual stage 1 active
2. **#2** — Advance to 66%
   - **Expected:** Visual stage 2 active
3. **#3** — Advance to 100%
   - **Expected:** Construction complete; ConstructionProgress removed

### TC-13.14.2.2 Construction Cancel Refund

| # | Requirement |
|---|-------------|
| 1 | R-13.14.2   |

1. **#1** — Cancel construction at 50% progress
   - **Expected:** 50% of materials refunded to inventory

### TC-13.14.2.3 Incomplete Reduced HP

| # | Requirement |
|---|-------------|
| 1 | R-13.14.2   |

1. **#1** — Building at 50% progress; query HP
   - **Expected:** HP = 50% of max_hp

### TC-13.14.3.1 Stability Decreases With Distance

| # | Requirement |
|---|-------------|
| 1 | R-13.14.3   |

1. **#1** — Build 5-piece tower on foundation
   - **Expected:** Stability decreases at each tier above foundation

### TC-13.14.3.2 Cascade Collapse

| # | Requirement |
|---|-------------|
| 1 | R-13.14.3   |

1. **#1** — Remove foundation from 3-piece tower
   - **Expected:** All dependent pieces marked for collapse and destroyed

### TC-13.14.3.3 Stone Higher Stability

| # | Requirement |
|---|-------------|
| 1 | R-13.14.3   |

1. **#1** — Wood piece at distance 3 vs Stone piece at distance 3
   - **Expected:** Stone stability > Wood stability

### TC-13.14.3.4 Stability Incremental

| # | Requirement |
|---|-------------|
| 1 | R-13.14.3   |

1. **#1** — Add piece; observe recompute
   - **Expected:** Stability recomputes on add/remove, not every frame

### TC-13.14.3.5 Stability Warning Visual

| # | Requirement |
|---|-------------|
| 1 | R-13.14.3   |

1. **#1** — Piece with stability < warning threshold
   - **Expected:** `StabilityWarning` component set with severity

### TC-13.14.4.1 Upgrade Tier

| # | Requirement |
|---|-------------|
| 1 | R-13.14.4   |

1. **#1** — Upgrade wood wall to stone
   - **Expected:** Mesh changes; HP increases; stability increases

### TC-13.14.4.2 Repair Proportional

| # | Requirement |
|---|-------------|
| 1 | R-13.14.4   |

1. **#1** — Damage to 50% HP; repair
   - **Expected:** Material cost = 50% of full construction cost

### TC-13.14.4.3 Decay Over Time

| # | Requirement |
|---|-------------|
| 1 | R-13.14.4   |

1. **#1** — Set decay_rate=0.01/hr; advance 10 game-hours
   - **Expected:** HP reduced by 10% of max

### TC-13.14.4.4 Upgrade Resets Decay

| # | Requirement |
|---|-------------|
| 1 | R-13.14.4   |

1. **#1** — Upgrade piece mid-decay
   - **Expected:** `BuildingDecay.last_maintained` reset to current time

### TC-13.14.5a.1 Housing Permission Public

| # | Requirement |
|---|-------------|
| 1 | R-13.14.5a  |

1. **#1** — Set permission=Public; non-owner attempts entry
   - **Expected:** Entry allowed

### TC-13.14.5a.2 Housing Permission Private

| # | Requirement |
|---|-------------|
| 1 | R-13.14.5a  |

1. **#1** — Set permission=Private; non-owner attempts entry
   - **Expected:** Entry blocked

### TC-13.14.5b.1 Furniture Grid Placement

| # | Requirement |
|---|-------------|
| 1 | R-13.14.5b  |

1. **#1** — Place furniture on interior grid
   - **Expected:** Snaps to valid grid position

### TC-13.14.5b.2 Furniture Overlap Reject

| # | Requirement |
|---|-------------|
| 1 | R-13.14.5b  |

1. **#1** — Place furniture overlapping existing furniture
   - **Expected:** Placement rejected

### TC-13.14.5c.1 Bed Sets Respawn

| # | Requirement |
|---|-------------|
| 1 | R-13.14.5c  |

1. **#1** — Place bed in housing instance
   - **Expected:** Player respawn point updated to bed location

### TC-13.14.5c.2 Chest Extends Inventory

| # | Requirement |
|---|-------------|
| 1 | R-13.14.5c  |

1. **#1** — Place storage chest (extra_slots=20)
   - **Expected:** Inventory capacity increases by 20

### TC-13.14.5c.3 Station Enables Crafting

| # | Requirement |
|---|-------------|
| 1 | R-13.14.5c  |

1. **#1** — Place crafting station
   - **Expected:** Station-gated recipes become accessible

### TC-13.14.6a.1 Hunger Base Drain

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6a  |

1. **#1** — Idle for 60 s; base_drain=0.5/s
   - **Expected:** Hunger reduced by 30

### TC-13.14.6a.2 Hunger Sprint Multiplier

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6a  |

1. **#1** — Sprint for 60 s; sprint_mult=2.0, drain=0.5/s
   - **Expected:** Hunger reduced by 60

### TC-13.14.6a.3 Thirst Hot Biome

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6a  |

1. **#1** — Desert biome (drain_mult=1.5); idle 60 s; drain=0.5/s
   - **Expected:** Thirst reduced by 45

### TC-13.14.6b.1 Warmth Clothing

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6b  |

1. **#1** — Equip insulated clothing (warmth_bonus=0.5) in Arctic
   - **Expected:** Warmth drain reduced by 50%

### TC-13.14.6b.2 Warmth Fire Proximity

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6b  |

1. **#1** — Stand within fire warmth_radius
   - **Expected:** Warmth restores at fire warmth_rate

### TC-13.14.6b.3 Warmth Shelter

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6b  |

1. **#1** — Enter building with roof (sheltered=true)
   - **Expected:** Warmth drain stops

### TC-13.14.6c.1 Stamina Sprint Drain

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6c  |

1. **#1** — Sprint for 10 s; sprint_cost=5.0/s
   - **Expected:** Stamina reduced by 50

### TC-13.14.6c.2 Stamina Rest Recovery

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6c  |

1. **#1** — Rest with recovery_rate=3.0/s for 10 s
   - **Expected:** Stamina increased by 30

### TC-13.14.6c.3 Fatigue Slows Recovery

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6c  |

1. **#1** — Fatigue at 80% of max; recovery base=3.0/s
   - **Expected:** Effective recovery = 3.0 * 0.2 = 0.6/s

### TC-13.14.6d.1 Starvation Debuff

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |

1. **#1** — Hunger below debuff_threshold
   - **Expected:** Max HP reduction effect applied

### TC-13.14.6d.2 Dehydration Debuff

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |

1. **#1** — Thirst below debuff_threshold
   - **Expected:** Movement speed reduction effect applied

### TC-13.14.6d.3 Hypothermia Debuff

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |

1. **#1** — Warmth below debuff_threshold
   - **Expected:** Periodic damage effect applied

### TC-13.14.6d.4 Debuff Removal

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |

1. **#1** — Restore vital above threshold
   - **Expected:** Corresponding debuff removed

### TC-13.14.7a.1 Node Tool Requirement

| # | Requirement |
|---|-------------|
| 1 | R-13.14.7a  |

1. **#1** — Gather ore node without pickaxe
   - **Expected:** Gathering rejected

### TC-13.14.7a.2 Node Depletion

| # | Requirement |
|---|-------------|
| 1 | R-13.14.7a  |

1. **#1** — Gather until node HP reaches 0
   - **Expected:** Node marked depleted; no further gathering

### TC-13.14.7a.3 Node Respawn

| # | Requirement |
|---|-------------|
| 1 | R-13.14.7a  |

1. **#1** — Deplete node; advance by respawn_timer
   - **Expected:** Node respawns with full HP

### TC-13.14.7b.1 Gather Yield Scales

| # | Requirement |
|---|-------------|
| 1 | R-13.14.7b  |

1. **#1** — Gather at skill level 1 vs skill level 10
   - **Expected:** Higher skill produces higher yield_multiplier

### TC-13.14.7b.2 Gather Rare Proc

| # | Requirement |
|---|-------------|
| 1 | R-13.14.7b  |

1. **#1** — Gather 1000 times; rare_chance=0.05
   - **Expected:** Rare procs within [30, 70] range (95% CI)

### TC-13.14.7c.1 PCG Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-13.14.7c  |

1. **#1** — Generate nodes with seed=42 twice
   - **Expected:** Identical node placement both times

### TC-13.14.7c.2 PCG Biome Density

| # | Requirement |
|---|-------------|
| 1 | R-13.14.7c  |

1. **#1** — Generate; count nodes per biome
   - **Expected:** Density matches configured density per biome

### TC-13.14.8.1 Crop Growth Stages

| # | Requirement |
|---|-------------|
| 1 | R-13.14.8   |

1. **#1** — Plant and advance time through 4 stage thresholds
   - **Expected:** Stage transitions at each threshold

### TC-13.14.8.2 Crop Wither

| # | Requirement |
|---|-------------|
| 1 | R-13.14.8   |

1. **#1** — Skip watering for wither_grace_period
   - **Expected:** Crop state = Withered

### TC-13.14.8.3 Crop Fertilizer

| # | Requirement |
|---|-------------|
| 1 | R-13.14.8   |

1. **#1** — Apply fertilizer (multiplier=1.5)
   - **Expected:** Growth speed increased by 50%

### TC-13.14.8.4 Crop Seasonal

| # | Requirement |
|---|-------------|
| 1 | R-13.14.8   |

1. **#1** — Plant summer crop in winter
   - **Expected:** Growth blocked; state unchanged

### TC-13.14.9a.1 Animal Happiness Production

| # | Requirement |
|---|-------------|
| 1 | R-13.14.9a  |

1. **#1** — Feed animal to max happiness
   - **Expected:** Production rate = max configured rate

### TC-13.14.9a.2 Animal Neglect

| # | Requirement |
|---|-------------|
| 1 | R-13.14.9a  |

1. **#1** — Starve animal; happiness drops to 0
   - **Expected:** Production rate = 0

### TC-13.14.9b.1 Animal Housing Capacity

| # | Requirement |
|---|-------------|
| 1 | R-13.14.9b  |

1. **#1** — Fill coop to capacity; add one more
   - **Expected:** Rejection; animal not placed

### TC-13.14.9b.2 Animal Species Compat

| # | Requirement |
|---|-------------|
| 1 | R-13.14.9b  |

1. **#1** — Place cow in chicken coop
   - **Expected:** Rejection; incompatible species

### TC-13.14.9c.1 Breeding Offspring

| # | Requirement |
|---|-------------|
| 1 | R-13.14.9c  |

1. **#1** — Breed two animals with traits A and B
   - **Expected:** Offspring inherits subset of parent traits

### TC-13.14.9c.2 Breeding Gestation

| # | Requirement |
|---|-------------|
| 1 | R-13.14.9c  |

1. **#1** — Breed; advance by gestation_timer
   - **Expected:** Offspring entity spawned

## Integration Tests

### TC-13.14.1.I1 Build Full House

| # | Requirement                     |
|---|---------------------------------|
| 1 | R-13.14.1, R-13.14.2, R-13.14.3 |

1. **#1** — Build foundation, walls, floor, roof
   - **Expected:** All snap valid; construction completes; stability valid

### TC-13.14.3.I1 Siege Destruction

| # | Requirement |
|---|-------------|
| 1 | R-13.14.3   |

1. **#1** — Build structure; destroy foundation via combat damage
   - **Expected:** Cascade collapse with VFX and debris

### TC-13.14.4.I1 Upgrade Full Path

| # | Requirement |
|---|-------------|
| 1 | R-13.14.4   |

1. **#1** — Upgrade wood -> stone -> metal -> reinforced
   - **Expected:** Each tier change updates mesh, HP, stability

### TC-13.14.5.I1 Housing Full Loop

| # | Requirement                        |
|---|------------------------------------|
| 1 | R-13.14.5a, R-13.14.5b, R-13.14.5c |

1. **#1** — Claim plot, build, furnish, set permissions, visit as friend
   - **Expected:** Full lifecycle completes correctly

### TC-13.14.6.I1 Survival Full Day

| # | Requirement                                    |
|---|------------------------------------------------|
| 1 | R-13.14.6a, R-13.14.6b, R-13.14.6c, R-13.14.6d |

1. **#1** — Simulate 24h: drain vitals, eat, drink, shelter, rest
   - **Expected:** Debuffs apply and resolve at correct thresholds

### TC-13.14.7.I1 Gather Craft Loop

| # | Requirement            |
|---|------------------------|
| 1 | R-13.14.7a, R-13.14.7b |

1. **#1** — Gather resources; craft items
   - **Expected:** Inventory contains crafted items

### TC-13.14.8.I1 Farm Full Cycle

| # | Requirement |
|---|-------------|
| 1 | R-13.14.8   |

1. **#1** — Till, plant, water, grow, harvest
   - **Expected:** Yield matches crop config

### TC-13.14.9.I1 Animal Full Lifecycle

| # | Requirement                        |
|---|------------------------------------|
| 1 | R-13.14.9a, R-13.14.9b, R-13.14.9c |

1. **#1** — House, feed, breed, collect production over 3 cycles
   - **Expected:** All systems integrate correctly

## Benchmarks

### TC-NFR-13.14.1.B1 Snap Validity

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Ghost preview with 500 existing pieces | Frame time | < 2 ms | NFR-13.14.1 |

### TC-NFR-13.14.1.B2 Structural Integrity

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Remove piece from 1000-piece structure | Recompute time | < 5 ms | NFR-13.14.1 |

### TC-NFR-13.14.1.B3 Cascade Collapse Frame Rate

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Collapse 50 pieces simultaneously | Frame rate | > 30 fps | NFR-13.14.1 |

### TC-13.14.6.B1 Vital Tick

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 4 vitals on 100 entities | Wall-clock time | < 0.1 ms | R-13.14.6a |

### TC-13.14.7.B1 Node Respawn Scan

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Scan 10,000 resource nodes | Wall-clock time | < 1 ms | R-13.14.7a |

### TC-13.14.8.B1 Crop Growth Tick

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Tick 1,000 planted crops | Wall-clock time | < 0.5 ms | R-13.14.8 |

### TC-13.14.9.B1 Animal Needs Tick

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Tick needs for 500 animals | Wall-clock time | < 0.2 ms | R-13.14.9a |
