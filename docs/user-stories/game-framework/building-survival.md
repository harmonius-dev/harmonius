# User Stories -- Building and Survival (13.14)

## Building Placement (F-13.14.1)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.14.1.1 | player (P-23) | **As a** player (P-23), **I want** building pieces to snap to valid attachment sockets on adjacent pieces with green/red ghost previews, **so that** I can align walls, floors, and roofs without manual positioning. |  | F-13.14.1 | R-13.14.1 |
| US-13.14.1.2 | player (P-23) | **As a** player (P-23), **I want** to rotate snap-placed pieces in 90-degree increments for square grids or 60-degree for hex grids, **so that** I can orient each piece to fit my layout. |  | F-13.14.1 | R-13.14.1 |
| US-13.14.1.3 | player (P-23) | **As a** player (P-23), **I want** a free-form placement mode that aligns pieces to the ground using physics, **so that** I can position structures on uneven terrain without grid constraints. |  | F-13.14.1 | R-13.14.1 |
| US-13.14.1.4 | designer (P-5) | **As a** designer (P-5), **I want** to define attachment sockets, rotation increments, and placement rules per building piece as data assets, **so that** I can create new building pieces without code changes. |  | F-13.14.1 | R-13.14.1 |
| US-13.14.1.5 | designer (P-5) | **As a** designer (P-5), **I want** placement validity checks for collision, slope, and adjacency to be configurable per piece, **so that** I can tune building rules for different game modes. |  | F-13.14.1 | R-13.14.1 |
| US-13.14.1.6 | level designer (P-6) | **As a** level designer (P-6), **I want** to restrict placement in designated zones using trigger volumes, **so that** players cannot build in quest-critical or performance-sensitive areas. |  | F-13.14.1 | R-13.14.1 |
| US-13.14.1.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a piece with no valid socket turns red and cannot be placed, **so that** invalid placements are correctly rejected. |  | F-13.14.1 | R-13.14.1 |
| US-13.14.1.8 | tester (P-27) | **As a** tester (P-27), **I want** to verify that free-form placement correctly aligns to sloped terrain, **so that** ground-level structures do not float or clip. |  | F-13.14.1 | R-13.14.1 |
## Construction Phase (F-13.14.2)
| US-13.14.2.1 | player (P-23) | **As a** player (P-23), **I want** placed buildings to start as scaffolds that require resources and time to complete, **so that** construction feels like a meaningful investment. |  | F-13.14.2 | R-13.14.2 |
| US-13.14.2.2 | player (P-23) | **As a** player (P-23), **I want** to see a progress bar and visual stages during construction, **so that** I know how close a building is to completion. |  | F-13.14.2 | R-13.14.2 |
| US-13.14.2.3 | player (P-23) | **As a** player (P-23), **I want** to pause, cancel, or accelerate construction with partial resource refunds on cancellation, **so that** I have control over my building projects. |  | F-13.14.2 | R-13.14.2 |
| US-13.14.2.4 | player (P-23) | **As a** player (P-23), **I want** incomplete buildings to have reduced HP and functionality proportional to progress, **so that** unfinished structures are vulnerable. |  | F-13.14.2 | R-13.14.2 |
| US-13.14.2.5 | designer (P-5) | **As a** designer (P-5), **I want** to configure construction duration, required materials, visual stage thresholds, and refund percentages per piece, **so that** I can balance building economy without code. |  | F-13.14.2 | R-13.14.2 |
| US-13.14.2.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that cancelling a 50%-complete building refunds the configured fraction of materials, **so that** the refund system is accurate. |  | F-13.14.2 | R-13.14.2 |
## Structural Integrity (F-13.14.3)
| US-13.14.3.1 | player (P-23) | **As a** player (P-23), **I want** unsupported structures to display cracks and wobble before collapsing, **so that** I receive visual warnings about instability. |  | F-13.14.3 | R-13.14.3 |
| US-13.14.3.2 | player (P-23) | **As a** player (P-23), **I want** destroying a foundation to cause cascade collapse of all pieces that depended on it, **so that** structural strategy matters during raids. |  | F-13.14.3 | R-13.14.3 |
| US-13.14.3.3 | player (P-23) | **As a** player (P-23), **I want** stone and metal pieces to have higher stability than wood, **so that** material choice affects structural reach. |  | F-13.14.3 | R-13.14.3 |
| US-13.14.3.4 | designer (P-5) | **As a** designer (P-5), **I want** to configure stability values, material bonuses, and cascade thresholds per building piece, **so that** I can tune structural behavior per game mode. |  | F-13.14.3 | R-13.14.3 |
| US-13.14.3.5 | level designer (P-6) | **As a** level designer (P-6), **I want** to preview stability propagation in the editor before play-testing, **so that** I can verify puzzle or raid encounter designs. |  | F-13.14.3 | R-13.14.3 |
| US-13.14.3.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that removing a middle support causes only the dependent pieces above threshold distance to collapse, **so that** cascade logic is correct. |  | F-13.14.3 | R-13.14.3 |
| US-13.14.3.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that stability is recomputed incrementally on piece add, remove, or damage rather than every frame, **so that** performance is not degraded by large structures. |  | F-13.14.3 | R-13.14.3 |
## Building Upgrade and Repair (F-13.14.4)
| US-13.14.4.1 | player (P-23) | **As a** player (P-23), **I want** to upgrade building pieces from wood to stone to metal by paying material costs, **so that** I can strengthen my base over time. |  | F-13.14.4 | R-13.14.4 |
| US-13.14.4.2 | player (P-23) | **As a** player (P-23), **I want** upgrading a piece to change its visual mesh, HP, and stability bonus, **so that** upgrades have visible and mechanical impact. |  | F-13.14.4 | R-13.14.4 |
| US-13.14.4.3 | player (P-23) | **As a** player (P-23), **I want** to repair damaged pieces by consuming materials proportional to the damage, **so that** I can restore my base after an attack. |  | F-13.14.4 | R-13.14.4 |
| US-13.14.4.4 | player (P-23) | **As a** player (P-23), **I want** unrepaired buildings to decay over real time at a configurable rate, **so that** abandoned bases deteriorate naturally. |  | F-13.14.4 | R-13.14.4 |
| US-13.14.4.5 | designer (P-5) | **As a** designer (P-5), **I want** to configure upgrade tiers, material costs, decay rates, and repair costs per material in gameplay databases, **so that** I can balance the building economy without code. |  | F-13.14.4 | R-13.14.4 |
| US-13.14.4.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that upgrading a piece mid-decay resets the decay timer, **so that** maintained structures do not decay prematurely. |  | F-13.14.4 | R-13.14.4 |
## Housing Plot and Instance (F-13.14.5a)
| US-13.14.5 | player (P-23) | **As a** player (P-23), **I want** to claim a designated housing plot as my personal building space, **so that** I have a persistent home location. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | player (P-23) | **As a** player (P-23), **I want** to set visitor permissions to public, friends, guild, or private, **so that** I control who can enter my housing instance. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | player (P-23) | **As a** player (P-23), **I want** my housing state to persist through saves and be visible to visitors, **so that** my home is a permanent part of the game world. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | designer (P-5) | **As a** designer (P-5), **I want** to configure plot sizes, instance limits, and permission tiers per housing zone, **so that** I can balance housing availability. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | level designer (P-6) | **As a** level designer (P-6), **I want** to place designated housing zones in the world with configurable boundaries, **so that** player housing integrates with the world layout. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a visitor set to "private" cannot enter the housing instance, **so that** permission enforcement works correctly. |  | F-13.14.5 | R-13.14.5 |
## Furniture Placement (F-13.14.5b)
| US-13.14.5 | player (P-23) | **As a** player (P-23), **I want** to place furniture using a separate grid or free placement system inside my home, **so that** I can arrange beds, tables, and decorations. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | player (P-23) | **As a** player (P-23), **I want** to place crafting stations inside my home, **so that** I can perform crafting from my personal space. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | designer (P-5) | **As a** designer (P-5), **I want** to define furniture placement rules, grid sizes, and valid surface types per interior space, **so that** I can control how players furnish different rooms. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that furniture cannot be placed overlapping existing furniture or outside interior bounds, **so that** placement validation is correct. |  | F-13.14.5 | R-13.14.5 |
## Functional Furniture (F-13.14.5c)
| US-13.14.5 | player (P-23) | **As a** player (P-23), **I want** beds to set my respawn point, **so that** I revive at home after death. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | player (P-23) | **As a** player (P-23), **I want** storage chests to extend my inventory capacity, **so that** I can store surplus items at home. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | player (P-23) | **As a** player (P-23), **I want** crafting stations to enable profession crafting, **so that** I can craft advanced items from my home. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | designer (P-5) | **As a** designer (P-5), **I want** to define functional effects per furniture type in gameplay databases, **so that** I can add new furniture benefits without code. |  | F-13.14.5 | R-13.14.5 |
| US-13.14.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that placing a bed updates the player's respawn point, **so that** functional furniture effects apply correctly. |  | F-13.14.5 | R-13.14.5 |
## Hunger and Thirst (F-13.14.6a)
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** hunger to drain at a base rate accelerated by physical activity, **so that** I must find food regularly. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** eating food to restore hunger and optionally provide temporary stat buffs, **so that** food quality matters. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** thirst to drain faster in hot biomes, **so that** I must plan water supplies for desert exploration. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | designer (P-5) | **As a** designer (P-5), **I want** to configure drain rates, restoration values, and buff effects per food item in gameplay databases, **so that** I can balance the food economy without code. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that sprinting accelerates hunger drain by the configured multiplier, **so that** activity-based drain is correct. |  | F-13.14.6 | R-13.14.6 |
## Temperature and Warmth (F-13.14.6b)
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** clothing insulation, fire proximity, and shelter to protect against cold, **so that** I can prepare for harsh environments. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** exposure to cold without mitigation to drain warmth over time, **so that** I face consequences for venturing unprepared. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | designer (P-5) | **As a** designer (P-5), **I want** to configure insulation values per clothing item and warmth bonuses per fire source, **so that** I can balance the temperature system. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that entering a shelter stops warmth drain, **so that** shelter detection works correctly. |  | F-13.14.6 | R-13.14.6 |
## Stamina and Fatigue (F-13.14.6c)
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** stamina to deplete from sprinting, jumping, and combat actions, **so that** I must pace my physical exertion. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** stamina to recover during rest at a configurable rate, **so that** resting between activities is rewarded. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** fatigue to accumulate from prolonged exertion and slow recovery, **so that** extended activity without rest has consequences. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | designer (P-5) | **As a** designer (P-5), **I want** to configure stamina costs per action and recovery rates in gameplay databases, **so that** I can tune the stamina economy per game mode. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that fatigue slows stamina recovery by the configured amount, **so that** the fatigue penalty is correct. |  | F-13.14.6 | R-13.14.6 |
## Vital Debuffs (F-13.14.6d)
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** starvation to reduce max HP, **so that** neglecting hunger has a dangerous combat penalty. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** dehydration to slow my movement speed, **so that** I cannot outrun threats while dehydrated. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | player (P-23) | **As a** player (P-23), **I want** hypothermia to apply periodic damage, **so that** prolonged cold exposure is lethal. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | designer (P-5) | **As a** designer (P-5), **I want** to configure debuff thresholds, penalty values, and effects per vital in gameplay databases, **so that** I can balance survival difficulty. |  | F-13.14.6 | R-13.14.6 |
| US-13.14.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that restoring a vital above the debuff threshold removes the associated debuff, **so that** debuff application and removal are correct. |  | F-13.14.6 | R-13.14.6 |
## Resource Node Definition (F-13.14.7a)
| US-13.14.7 | player (P-23) | **As a** player (P-23), **I want** to identify harvestable resource nodes with clear visual types, **so that** I know what each node yields before gathering. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | player (P-23) | **As a** player (P-23), **I want** nodes to require specific tools, **so that** I must equip the right tool before gathering. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | designer (P-5) | **As a** designer (P-5), **I want** to define resource type, gather time, tool requirement, node HP, respawn timer, and rare yield chance per node in data tables, **so that** I can create new resource nodes without code. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | level designer (P-6) | **As a** level designer (P-6), **I want** to hand-place resource nodes in the world with configurable properties, **so that** I can control resource availability in designed areas. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a node without the required tool equipped shows an error and prevents gathering, **so that** tool requirements are enforced. |  | F-13.14.7 | R-13.14.7 |
## Gathering Interaction (F-13.14.7b)
| US-13.14.7 | player (P-23) | **As a** player (P-23), **I want** gathering to trigger an interaction with a looped animation until the node depletes or I cancel, **so that** gathering feels physical and interruptible. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | player (P-23) | **As a** player (P-23), **I want** gathered resources to be added to my inventory automatically, **so that** I receive loot without manual pickup. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | player (P-23) | **As a** player (P-23), **I want** my gathering skill level to increase yield quantity and rare proc rate, **so that** skill progression is rewarded. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | designer (P-5) | **As a** designer (P-5), **I want** to configure yield tables and skill multipliers per node type, **so that** I can balance gathering rewards. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a depleted node stops the gathering animation and respawns after the configured timer, **so that** the depletion and respawn cycle works. |  | F-13.14.7 | R-13.14.7 |
## Resource Node Distribution (F-13.14.7c)
| US-13.14.7 | player (P-23) | **As a** player (P-23), **I want** resource nodes distributed across the world by biome type, **so that** exploration rewards me with new gathering opportunities. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | designer (P-5) | **As a** designer (P-5), **I want** to configure biome-specific node density, type distribution, and clustering rules per world region, **so that** I can control resource placement in procedural worlds. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | level designer (P-6) | **As a** level designer (P-6), **I want** procedural node placement to integrate with the world generation system, **so that** nodes appear naturally in generated terrain. |  | F-13.14.7 | R-13.14.7 |
| US-13.14.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that regenerating the same seed produces identical node placements, **so that** procedural distribution is deterministic. |  | F-13.14.7 | R-13.14.7 |
## Farming and Crops (F-13.14.8)
| US-13.14.8.1 | player (P-23) | **As a** player (P-23), **I want** to till soil, plant seeds, water crops, and harvest produce through a multi-stage growth loop, **so that** farming feels like a rewarding long-term activity. |  | F-13.14.8 | R-13.14.8 |
| US-13.14.8.2 | player (P-23) | **As a** player (P-23), **I want** crops to wither from neglect after a configurable grace period, **so that** I must tend my farm regularly. |  | F-13.14.8 | R-13.14.8 |
| US-13.14.8.3 | player (P-23) | **As a** player (P-23), **I want** fertilizer to speed up growth and increase yield, **so that** investing in soil quality pays off. |  | F-13.14.8 | R-13.14.8 |
| US-13.14.8.4 | player (P-23) | **As a** player (P-23), **I want** seasonal constraints to restrict which crops grow in which seasons, **so that** crop planning has strategic depth. |  | F-13.14.8 | R-13.14.8 |
| US-13.14.8.5 | designer (P-5) | **As a** designer (P-5), **I want** to configure growth stages, watering requirements, seasonal rules, and fertilizer effects per crop type, **so that** I can balance the farming economy. |  | F-13.14.8 | R-13.14.8 |
| US-13.14.8.6 | level designer (P-6) | **As a** level designer (P-6), **I want** to designate farming zones in player housing areas, **so that** farm plots integrate with the world layout. |  | F-13.14.8 | R-13.14.8 |
| US-13.14.8.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that an unwatered crop withers after the grace period, **so that** neglect penalties apply correctly. |  | F-13.14.8 | R-13.14.8 |
## Animal Needs and Happiness (F-13.14.9a)
| US-13.14.9 | player (P-23) | **As a** player (P-23), **I want** domesticated animals to have hunger and happiness meters, **so that** I understand their current needs at a glance. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | player (P-23) | **As a** player (P-23), **I want** fed and happy animals to produce more resources, **so that** caring for animals is rewarded with higher yields. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | player (P-23) | **As a** player (P-23), **I want** to feed, pet, and clean animals through interaction actions, **so that** I can directly influence their happiness. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | designer (P-5) | **As a** designer (P-5), **I want** to configure production rates, happiness decay, and care action effects per animal species, **so that** I can balance the animal husbandry economy. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a neglected animal's production rate drops to the configured minimum, **so that** happiness affects output correctly. |  | F-13.14.9 | R-13.14.9 |
## Animal Housing (F-13.14.9b)
| US-13.14.9 | player (P-23) | **As a** player (P-23), **I want** to build coops, barns, and stables for my animals, **so that** each species has appropriate housing with capacity limits. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | player (P-23) | **As a** player (P-23), **I want** to acquire animals from NPC vendors, wild taming, or breeding, **so that** I have multiple paths to expand my livestock. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | designer (P-5) | **As a** designer (P-5), **I want** to configure housing capacity, species compatibility, and purchase costs per structure type, **so that** I can balance animal housing. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | level designer (P-6) | **As a** level designer (P-6), **I want** animal housing to be buildable only in housing zones, **so that** animal structures integrate with the housing system. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | tester (P-27) | **As a** tester (P-27), **I want** to verify that exceeding a housing structure's capacity prevents adding more animals, **so that** capacity limits are enforced. |  | F-13.14.9 | R-13.14.9 |
## Animal Breeding (F-13.14.9c)
| US-13.14.9 | player (P-23) | **As a** player (P-23), **I want** to breed compatible animal pairs and see offspring with inherited trait variations, **so that** selective breeding is rewarding. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | player (P-23) | **As a** player (P-23), **I want** breeding to require a suitable housing structure and a gestation timer, **so that** breeding is a deliberate investment. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | designer (P-5) | **As a** designer (P-5), **I want** to configure genetic rules, trait inheritance, and random variation per species, **so that** I can control breeding outcomes. |  | F-13.14.9 | R-13.14.9 |
| US-13.14.9 | tester (P-27) | **As a** tester (P-27), **I want** to verify that offspring inherit traits from both parents within the configured variation range, **so that** the genetics system produces correct results. |  | F-13.14.9 | R-13.14.9 |
