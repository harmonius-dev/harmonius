# R-13.14 -- Building and Survival Requirements

| ID         | Derived From                                                     |
|------------|------------------------------------------------------------------|
| R-13.14.1  | [F-13.14.1](../../features/game-framework/building-survival.md)  |
| R-13.14.2  | [F-13.14.2](../../features/game-framework/building-survival.md)  |
| R-13.14.3  | [F-13.14.3](../../features/game-framework/building-survival.md)  |
| R-13.14.4  | [F-13.14.4](../../features/game-framework/building-survival.md)  |
| R-13.14.5a | [F-13.14.5a](../../features/game-framework/building-survival.md) |
| R-13.14.5b | [F-13.14.5b](../../features/game-framework/building-survival.md) |
| R-13.14.5c | [F-13.14.5c](../../features/game-framework/building-survival.md) |
| R-13.14.6a | [F-13.14.6a](../../features/game-framework/building-survival.md) |
| R-13.14.6b | [F-13.14.6b](../../features/game-framework/building-survival.md) |
| R-13.14.6c | [F-13.14.6c](../../features/game-framework/building-survival.md) |
| R-13.14.6d | [F-13.14.6d](../../features/game-framework/building-survival.md) |
| R-13.14.7a | [F-13.14.7a](../../features/game-framework/building-survival.md) |
| R-13.14.7b | [F-13.14.7b](../../features/game-framework/building-survival.md) |
| R-13.14.7c | [F-13.14.7c](../../features/game-framework/building-survival.md) |
| R-13.14.8  | [F-13.14.8](../../features/game-framework/building-survival.md)  |
| R-13.14.9a | [F-13.14.9a](../../features/game-framework/building-survival.md) |
| R-13.14.9b | [F-13.14.9b](../../features/game-framework/building-survival.md) |
| R-13.14.9c | [F-13.14.9c](../../features/game-framework/building-survival.md) |

1. **R-13.14.1** — The engine **SHALL** provide a snap-based building placement system where modular
   pieces define attachment sockets, display a ghost preview mesh with green/red validity
   indication, and snap to compatible adjacent sockets at configurable rotation increments, with all
   placement rules authored as data-driven building piece assets.
   - **Rationale:** Socket-driven snap placement enables intuitive construction without manual
     alignment, and data-driven rules let designers add new building pieces without code changes.
   - **Verification:** Place a wall piece adjacent to a foundation. Verify the ghost preview snaps
     to the foundation's wall socket at 90-degree increments, shows green when valid and red when
     colliding with existing geometry, and commits placement only in the valid state. Confirm that
     free-form mode allows unconstrained positioning with physics-based ground alignment.
2. **R-13.14.2** — The engine **SHALL** place buildings as scaffold blueprints that require
   resources and time to complete, display a progress bar, transition through visual stages keyed to
   progress thresholds, and support pause, cancel (with partial resource refund), and
   worker-accelerated construction.
   - **Rationale:** Phased construction creates meaningful resource investment and visual feedback
     that communicates build state to players without UI dependency.
   - **Verification:** Place a building requiring 100 wood. Verify that 100 wood is deducted from
     inventory, the scaffold transitions through at least three visual stages as progress increases,
     incomplete buildings have proportionally reduced HP, and cancellation refunds a configured
     fraction of the spent resources.
3. **R-13.14.3** — The engine **SHALL** compute stability values for building pieces based on
   distance from grounded foundations, display visual warnings on low-stability pieces, and trigger
   cascade collapse when stability reaches zero, with stability recomputed incrementally on piece
   addition, removal, or damage.
   - **Rationale:** Stability propagation from foundations creates meaningful structural constraints
     that reward thoughtful building design and enable destruction-based gameplay.
   - **Verification:** Build a five-piece-tall tower on a single foundation. Verify stability
     decreases with each tier. Remove the foundation and confirm all pieces cascade-collapse in
     order. Verify that stone pieces carry higher stability values than wood pieces at the same
     distance from the foundation.
4. **R-13.14.4** — The engine **SHALL** support in-place upgrade of building pieces across material
   tiers, repair that consumes materials proportional to damage, and configurable time-based decay
   when structures are not maintained, with per-tier costs and decay rates defined in gameplay
   databases.
   - **Rationale:** Upgrade paths and decay create an ongoing resource loop that sustains engagement
     and differentiates early-game from late-game structures.
   - **Verification:** Upgrade a wood wall to stone. Verify the mesh, HP, and stability bonus change
     and the correct material cost is deducted. Damage the wall to 50% HP, repair it, and confirm
     material consumption equals the proportional repair cost. Enable decay and verify HP decreases
     at the configured rate over simulated time.
5. **R-13.14.5a** — The engine **SHALL** support designated housing plots or instanced housing zones
   with configurable visitor permissions (public, friends, guild, private) persisted through the
   save system.
   - **Rationale:** Permission-gated housing instances protect player investment from unauthorized
     access and provide persistent personal space.
   - **Verification:** Create a housing instance. Set permissions to friends-only and confirm a
     non-friend player cannot enter. Save and reload the session and verify permissions persist.
6. **R-13.14.5b** — The engine **SHALL** support furniture placement within housing interiors using
   a separate grid or free-placement system for beds, tables, chests, stations, and decorations.
   - **Rationale:** A dedicated interior placement system enables intuitive furniture arrangement
     distinct from the exterior building system.
   - **Verification:** Place furniture items in a housing instance using grid snap and
     free-placement modes. Save and reload the session and verify all furniture positions persist.
7. **R-13.14.5c** — The engine **SHALL** apply data-driven gameplay effects from functional
   furniture: beds set respawn points, storage chests extend inventory, and crafting stations enable
   profession crafting.
   - **Rationale:** Functional furniture ties housing investment to tangible gameplay benefits,
     incentivizing interior customization beyond cosmetics.
   - **Verification:** Place a bed and verify it sets the respawn point. Place a storage chest and
     verify it extends inventory capacity. Place a crafting station and verify profession recipes
     are accessible.
8. **R-13.14.6a** — The engine **SHALL** simulate hunger and thirst meters that drain over time at
   configurable base rates, accelerated by physical activity and biome conditions, restored by
   consuming food and drink items.
   - **Rationale:** Hunger and thirst create core resource management decisions that drive
     gathering, cooking, and exploration gameplay loops.
   - **Verification:** Simulate a character sprinting for 60 seconds. Verify hunger drains faster
     than baseline. Move to a hot biome and verify thirst drains faster. Eat food and verify hunger
     restores by the configured amount.
9. **R-13.14.6b** — The engine **SHALL** simulate a warmth meter affected by clothing insulation,
   fire proximity, shelter detection, and weather conditions, draining on cold exposure.
   - **Rationale:** Temperature-based survival adds environmental awareness and rewards preparation
     through clothing and shelter.
   - **Verification:** Expose a character to cold weather without insulation and verify warmth
     drains. Equip insulated clothing and verify drain slows. Stand near a fire and verify warmth
     restores.
10. **R-13.14.6c** — The engine **SHALL** simulate stamina that depletes from sprinting, jumping,
    and combat actions and recovers during rest, with fatigue accumulation slowing recovery after
    prolonged exertion.
    - **Rationale:** Stamina creates pacing in combat and exploration, preventing indefinite
      sprinting and rewarding strategic rest.
    - **Verification:** Sprint until stamina depletes and verify the character cannot sprint. Rest
      and verify stamina recovers at the configured rate. Verify prolonged exertion increases
      fatigue and slows recovery.
11. **R-13.14.6d** — The engine **SHALL** impose configurable debuffs when vitals reach critically
    low thresholds: starvation reduces max HP, dehydration slows movement, hypothermia applies
    periodic damage.
    - **Rationale:** Vital debuffs create urgency and consequence for neglecting survival needs,
      differentiating survival from standard gameplay.
    - **Verification:** Reduce hunger to the critical threshold and confirm max HP is reduced by the
      configured starvation penalty. Reduce thirst to critical and verify movement slows. Reduce
      warmth to critical and verify periodic damage applies.
12. **R-13.14.7a** — The engine **SHALL** define harvestable resource nodes with configurable
    resource type, gather time, tool requirements, node HP, respawn timer, and rare yield chance.
    - **Rationale:** Data-driven resource node definitions allow designers to add and balance node
      types without code changes.
    - **Verification:** Define an ore node requiring a pickaxe. Verify gathering fails without a
      pickaxe, succeeds with one, depletes the node after the configured HP, and respawns after the
      configured timer.
13. **R-13.14.7b** — The engine **SHALL** execute gathering as an interaction loop with animation,
    skill-based yield scaling, and inventory deposit, where skill level determines yield quantity
    and rare proc rate.
    - **Rationale:** Skill-scaled gathering rewards profession investment and creates meaningful
      progression for gathering-focused players.
    - **Verification:** Gather at two different skill levels and confirm that higher skill produces
      greater yield quantities. Verify gathered resources appear in inventory and the gathering
      animation loops until completion or cancellation.
14. **R-13.14.7c** — The engine **SHALL** distribute resource nodes procedurally via PCG integration
    with configurable per-biome density, type distribution, and clustering rules.
    - **Rationale:** Procedural node distribution scales to open worlds without requiring manual
      placement of every node.
    - **Verification:** Generate a world region and verify resource nodes are placed according to
      biome-specific density and type rules. Regenerate and confirm variation within configured
      bounds.
15. **R-13.14.8** — The engine **SHALL** implement a crop growth pipeline with configurable stages,
    watering requirements, soil quality modifiers, seasonal constraints, and wither timers, where
    harvested crops are inventory items usable in crafting recipes.
    - **Rationale:** Multi-stage crop growth with environmental modifiers creates a meaningful
      agricultural loop that integrates with crafting and survival systems.
    - **Verification:** Plant a seed, water it, and advance simulated time through all growth
      stages. Verify the crop transitions through each visual stage at configured durations. Skip
      watering and confirm the crop withers after the grace period. Apply fertilizer and verify
      growth speed increases by the configured multiplier.
16. **R-13.14.9a** — The engine **SHALL** simulate animal needs (hunger, happiness) that affect
    resource production rates, with care interactions restoring needs and production scaling with
    happiness.
    - **Rationale:** Needs-based production rewards sustained player investment in animal care and
      creates ongoing engagement with the husbandry system.
    - **Verification:** Feed an animal to full happiness and verify production rate matches the
      configured maximum. Starve the animal and confirm production rate decreases proportionally.
17. **R-13.14.9b** — The engine **SHALL** support buildable animal housing structures (coops, barns,
    stables) placed in housing zones, each with species-specific capacity limits.
    - **Rationale:** Dedicated housing structures create spatial investment and capacity constraints
      that add depth to animal management.
    - **Verification:** Build a coop in a housing zone and place a chicken. Verify the coop enforces
      its capacity limit. Attempt to place an incompatible species and verify rejection.
18. **R-13.14.9c** — The engine **SHALL** produce offspring from breeding pairs with inherited trait
    variations (color, size, productivity), requiring compatible species, suitable housing, and a
    gestation timer.
    - **Rationale:** Breeding with trait inheritance creates emergent genetic variety that rewards
      selective breeding strategies.
    - **Verification:** Breed two animals and verify the offspring inherits traits from both parents
      with random variation within configured bounds. Verify gestation timer elapses before
      offspring appears.

## Non-Functional Requirements

| ID          | Derived From |
|-------------|--------------|
| NFR-13.14.1 |              |
| NFR-13.14.2 |              |

1. **NFR-13.14.1** — The building placement system **SHALL** evaluate snap validity and render ghost
   preview meshes within 2ms per frame for up to 500 placed building pieces in the active area.
   Structural integrity recomputation on piece addition, removal, or damage **SHALL** complete
   within 5ms for structures of up to 1,000 pieces. Cascade collapse propagation **SHALL NOT** cause
   frame drops below 30 fps.
   - **Rationale:** Players build large structures in survival games. Placement preview and
     structural integrity must remain responsive to avoid breaking the building flow.
   - **Verification:** Place 500 building pieces and measure ghost preview evaluation time. Verify
     it stays under 2ms. Build a 1,000-piece structure, remove a foundation, and measure integrity
     recomputation time. Verify it stays under 5ms. Trigger cascade collapse and verify frame rate
     stays above 30 fps.
2. **NFR-13.14.2** — All survival meter rates, thresholds, debuffs, resource node yields, crop
   growth durations, and animal husbandry parameters **SHALL** be configurable through gameplay
   database entries authored in the visual editor without code changes or recompilation.
   - **Rationale:** Survival balance requires frequent iteration. Data-driven configuration enables
     designers to tune all survival parameters without engineering support.
   - **Verification:** Modify hunger drain rate, crop growth duration, and animal production rate in
     gameplay databases. Verify changes take effect at runtime without restarting the game session.
