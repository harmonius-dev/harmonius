# Building and Survival Systems Test Cases

Companion test cases for [building-survival.md](building-survival.md).

## Unit Tests

### TC-13.14.1.1 Socket Snap Valid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place wall next to foundation with matching socket | Ghost snaps to wall socket at 90-degree increment | R-13.14.1 |

### TC-13.14.1.2 Socket Snap Invalid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place wall with no nearby sockets | `PlacementError::NoValidSocket`; ghost shows red | R-13.14.1 |

### TC-13.14.1.3 Freeform Ground Align

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Free-form place on 15-degree slope | Piece aligned to ground normal; no float or clip | R-13.14.1 |

### TC-13.14.1.4 Placement Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place piece overlapping existing piece | `PlacementError::CollisionDetected` | R-13.14.1 |

### TC-13.14.1.5 Placement Zone Restrict

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place in restricted zone | `PlacementError::RestrictedZone` | R-13.14.1 |

### TC-13.14.2.1 Construction Progress

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place scaffold; advance to 33% | Visual stage 1 active | R-13.14.2 |
| 2 | Advance to 66% | Visual stage 2 active | R-13.14.2 |
| 3 | Advance to 100% | Construction complete; ConstructionProgress removed | R-13.14.2 |

### TC-13.14.2.2 Construction Cancel Refund

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cancel construction at 50% progress | 50% of materials refunded to inventory | R-13.14.2 |

### TC-13.14.2.3 Incomplete Reduced HP

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Building at 50% progress; query HP | HP = 50% of max_hp | R-13.14.2 |

### TC-13.14.3.1 Stability Decreases With Distance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build 5-piece tower on foundation | Stability decreases at each tier above foundation | R-13.14.3 |

### TC-13.14.3.2 Cascade Collapse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove foundation from 3-piece tower | All dependent pieces marked for collapse and destroyed | R-13.14.3 |

### TC-13.14.3.3 Stone Higher Stability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wood piece at distance 3 vs Stone piece at distance 3 | Stone stability > Wood stability | R-13.14.3 |

### TC-13.14.3.4 Stability Incremental

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add piece; observe recompute | Stability recomputes on add/remove, not every frame | R-13.14.3 |

### TC-13.14.3.5 Stability Warning Visual

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Piece with stability < warning threshold | `StabilityWarning` component set with severity | R-13.14.3 |

### TC-13.14.4.1 Upgrade Tier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upgrade wood wall to stone | Mesh changes; HP increases; stability increases | R-13.14.4 |

### TC-13.14.4.2 Repair Proportional

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Damage to 50% HP; repair | Material cost = 50% of full construction cost | R-13.14.4 |

### TC-13.14.4.3 Decay Over Time

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set decay_rate=0.01/hr; advance 10 game-hours | HP reduced by 10% of max | R-13.14.4 |

### TC-13.14.4.4 Upgrade Resets Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upgrade piece mid-decay | `BuildingDecay.last_maintained` reset to current time | R-13.14.4 |

### TC-13.14.5a.1 Housing Permission Public

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set permission=Public; non-owner attempts entry | Entry allowed | R-13.14.5a |

### TC-13.14.5a.2 Housing Permission Private

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set permission=Private; non-owner attempts entry | Entry blocked | R-13.14.5a |

### TC-13.14.5b.1 Furniture Grid Placement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place furniture on interior grid | Snaps to valid grid position | R-13.14.5b |

### TC-13.14.5b.2 Furniture Overlap Reject

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place furniture overlapping existing furniture | Placement rejected | R-13.14.5b |

### TC-13.14.5c.1 Bed Sets Respawn

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place bed in housing instance | Player respawn point updated to bed location | R-13.14.5c |

### TC-13.14.5c.2 Chest Extends Inventory

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place storage chest (extra_slots=20) | Inventory capacity increases by 20 | R-13.14.5c |

### TC-13.14.5c.3 Station Enables Crafting

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place crafting station | Station-gated recipes become accessible | R-13.14.5c |

### TC-13.14.6a.1 Hunger Base Drain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Idle for 60 s; base_drain=0.5/s | Hunger reduced by 30 | R-13.14.6a |

### TC-13.14.6a.2 Hunger Sprint Multiplier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sprint for 60 s; sprint_mult=2.0, drain=0.5/s | Hunger reduced by 60 | R-13.14.6a |

### TC-13.14.6a.3 Thirst Hot Biome

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Desert biome (drain_mult=1.5); idle 60 s; drain=0.5/s | Thirst reduced by 45 | R-13.14.6a |

### TC-13.14.6b.1 Warmth Clothing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip insulated clothing (warmth_bonus=0.5) in Arctic | Warmth drain reduced by 50% | R-13.14.6b |

### TC-13.14.6b.2 Warmth Fire Proximity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stand within fire warmth_radius | Warmth restores at fire warmth_rate | R-13.14.6b |

### TC-13.14.6b.3 Warmth Shelter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enter building with roof (sheltered=true) | Warmth drain stops | R-13.14.6b |

### TC-13.14.6c.1 Stamina Sprint Drain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sprint for 10 s; sprint_cost=5.0/s | Stamina reduced by 50 | R-13.14.6c |

### TC-13.14.6c.2 Stamina Rest Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rest with recovery_rate=3.0/s for 10 s | Stamina increased by 30 | R-13.14.6c |

### TC-13.14.6c.3 Fatigue Slows Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fatigue at 80% of max; recovery base=3.0/s | Effective recovery = 3.0 * 0.2 = 0.6/s | R-13.14.6c |

### TC-13.14.6d.1 Starvation Debuff

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hunger below debuff_threshold | Max HP reduction effect applied | R-13.14.6d |

### TC-13.14.6d.2 Dehydration Debuff

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Thirst below debuff_threshold | Movement speed reduction effect applied | R-13.14.6d |

### TC-13.14.6d.3 Hypothermia Debuff

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Warmth below debuff_threshold | Periodic damage effect applied | R-13.14.6d |

### TC-13.14.6d.4 Debuff Removal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Restore vital above threshold | Corresponding debuff removed | R-13.14.6d |

### TC-13.14.7a.1 Node Tool Requirement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gather ore node without pickaxe | Gathering rejected | R-13.14.7a |

### TC-13.14.7a.2 Node Depletion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gather until node HP reaches 0 | Node marked depleted; no further gathering | R-13.14.7a |

### TC-13.14.7a.3 Node Respawn

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deplete node; advance by respawn_timer | Node respawns with full HP | R-13.14.7a |

### TC-13.14.7b.1 Gather Yield Scales

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gather at skill level 1 vs skill level 10 | Higher skill produces higher yield_multiplier | R-13.14.7b |

### TC-13.14.7b.2 Gather Rare Proc

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gather 1000 times; rare_chance=0.05 | Rare procs within [30, 70] range (95% CI) | R-13.14.7b |

### TC-13.14.7c.1 PCG Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate nodes with seed=42 twice | Identical node placement both times | R-13.14.7c |

### TC-13.14.7c.2 PCG Biome Density

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate; count nodes per biome | Density matches configured density per biome | R-13.14.7c |

### TC-13.14.8.1 Crop Growth Stages

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plant and advance time through 4 stage thresholds | Stage transitions at each threshold | R-13.14.8 |

### TC-13.14.8.2 Crop Wither

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Skip watering for wither_grace_period | Crop state = Withered | R-13.14.8 |

### TC-13.14.8.3 Crop Fertilizer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply fertilizer (multiplier=1.5) | Growth speed increased by 50% | R-13.14.8 |

### TC-13.14.8.4 Crop Seasonal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Plant summer crop in winter | Growth blocked; state unchanged | R-13.14.8 |

### TC-13.14.9a.1 Animal Happiness Production

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Feed animal to max happiness | Production rate = max configured rate | R-13.14.9a |

### TC-13.14.9a.2 Animal Neglect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Starve animal; happiness drops to 0 | Production rate = 0 | R-13.14.9a |

### TC-13.14.9b.1 Animal Housing Capacity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill coop to capacity; add one more | Rejection; animal not placed | R-13.14.9b |

### TC-13.14.9b.2 Animal Species Compat

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place cow in chicken coop | Rejection; incompatible species | R-13.14.9b |

### TC-13.14.9c.1 Breeding Offspring

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Breed two animals with traits A and B | Offspring inherits subset of parent traits | R-13.14.9c |

### TC-13.14.9c.2 Breeding Gestation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Breed; advance by gestation_timer | Offspring entity spawned | R-13.14.9c |

## Integration Tests

### TC-13.14.1.I1 Build Full House

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build foundation, walls, floor, roof | All snap valid; construction completes; stability valid | R-13.14.1, R-13.14.2, R-13.14.3 |

### TC-13.14.3.I1 Siege Destruction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build structure; destroy foundation via combat damage | Cascade collapse with VFX and debris | R-13.14.3 |

### TC-13.14.4.I1 Upgrade Full Path

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upgrade wood -> stone -> metal -> reinforced | Each tier change updates mesh, HP, stability | R-13.14.4 |

### TC-13.14.5.I1 Housing Full Loop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Claim plot, build, furnish, set permissions, visit as friend | Full lifecycle completes correctly | R-13.14.5a, R-13.14.5b, R-13.14.5c |

### TC-13.14.6.I1 Survival Full Day

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate 24h: drain vitals, eat, drink, shelter, rest | Debuffs apply and resolve at correct thresholds | R-13.14.6a, R-13.14.6b, R-13.14.6c, R-13.14.6d |

### TC-13.14.7.I1 Gather Craft Loop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gather resources; craft items | Inventory contains crafted items | R-13.14.7a, R-13.14.7b |

### TC-13.14.8.I1 Farm Full Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Till, plant, water, grow, harvest | Yield matches crop config | R-13.14.8 |

### TC-13.14.9.I1 Animal Full Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | House, feed, breed, collect production over 3 cycles | All systems integrate correctly | R-13.14.9a, R-13.14.9b, R-13.14.9c |

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
