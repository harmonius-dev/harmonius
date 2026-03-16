# User Stories -- Pets, Companions, and Mounts (13.15)

## Companion AI Framework (F-13.15.1)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.15.1.1 | player (P-23) | **As a** player (P-23), **I want** to issue follow, guard, assist, stay, and patrol commands to my companion, **so that** I can direct their behavior in exploration and combat. |  | F-13.15.1 | R-13.15.1 |
| US-13.15.1.2 | player (P-23) | **As a** player (P-23), **I want** my companion to navigate using pathfinding and maintain a configurable follow distance, **so that** they stay near me without blocking my movement. |  | F-13.15.1 | R-13.15.1 |
| US-13.15.1.3 | player (P-23) | **As a** player (P-23), **I want** my companion to teleport to me if they fall too far behind, **so that** I never lose my companion due to pathing failures. |  | F-13.15.1 | R-13.15.1 |
| US-13.15.1.4 | player (P-23) | **As a** player (P-23), **I want** my combat companion to use abilities autonomously with AI-controlled activation, **so that** they contribute to fights without micromanagement. |  | F-13.15.1 | R-13.15.1 |
| US-13.15.1.5 | designer (P-5) | **As a** designer (P-5), **I want** to configure companion stats, abilities, follow distance, and teleport threshold in gameplay databases, **so that** I can balance companions per species. |  | F-13.15.1 | R-13.15.1 |
| US-13.15.1.6 | designer (P-5) | **As a** designer (P-5), **I want** companion behavior driven by behavior trees with command-selected subtrees, **so that** each command maps to a distinct AI behavior set. |  | F-13.15.1 | R-13.15.1 |
| US-13.15.1.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that issuing "guard" locks the companion to a position and they attack approaching enemies, **so that** the guard command works correctly. |  | F-13.15.1 | R-13.15.1 |
| US-13.15.1.8 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the companion teleports when the distance exceeds the threshold, **so that** the catch-up mechanism triggers reliably. |  | F-13.15.1 | R-13.15.1 |
## Pet Needs and Mood (F-13.15.2)
| US-13.15.2.1 | player (P-23) | **As a** player (P-23), **I want** my pet to have hunger, happiness, and cleanliness meters displayed in a compact UI panel, **so that** I can monitor their needs at a glance. |  | F-13.15.2 | R-13.15.2 |
| US-13.15.2.2 | player (P-23) | **As a** player (P-23), **I want** a fed and happy pet to fight better and follow more responsively, **so that** caring for my pet improves its effectiveness. |  | F-13.15.2 | R-13.15.2 |
| US-13.15.2.3 | player (P-23) | **As a** player (P-23), **I want** a neglected pet to become sluggish and refuse commands, **so that** pet care has meaningful gameplay consequences. |  | F-13.15.2 | R-13.15.2 |
| US-13.15.2.4 | player (P-23) | **As a** player (P-23), **I want** to feed, pet, play with, and bathe my companion through interactions, **so that** I can actively maintain their happiness. |  | F-13.15.2 | R-13.15.2 |
| US-13.15.2.5 | player (P-23) | **As a** player (P-23), **I want** pet food items to have quality tiers affecting hunger restoration and happiness bonus, **so that** food quality matters. |  | F-13.15.2 | R-13.15.2 |
| US-13.15.2.6 | designer (P-5) | **As a** designer (P-5), **I want** to configure need drain rates, care action effects, and mood thresholds per pet species, **so that** I can differentiate pet maintenance difficulty. |  | F-13.15.2 | R-13.15.2 |
| US-13.15.2.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a pet at zero happiness refuses commands, **so that** the neglect penalty functions correctly. |  | F-13.15.2 | R-13.15.2 |
| US-13.15.2.8 | tester (P-27) | **As a** tester (P-27), **I want** to verify that extended neglect causes the pet to run away, **so that** the abandonment consequence triggers. |  | F-13.15.2 | R-13.15.2 |
## Mount Summoning and Dismissal (F-13.15.3a)
| US-13.15.3 | player (P-23) | **As a** player (P-23), **I want** to open a collection UI showing all my owned mounts, **so that** I can browse and select which mount to summon. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | player (P-23) | **As a** player (P-23), **I want** summoning to spawn the mount near me and dismissal to despawn it on command, **so that** I can quickly access or stow my mount. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure mount stats (speed, stamina, armor) per species in gameplay databases, **so that** mount variety creates meaningful choices. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that summoning a mount in a no-mount zone fails with an appropriate error message, **so that** zone restrictions are enforced. |  | F-13.15.3 | R-13.15.3 |
## Mounted Locomotion (F-13.15.3b)
| US-13.15.3 | player (P-23) | **As a** player (P-23), **I want** mounting to play an enter animation and switch my character controller to mount physics, **so that** riding feels distinct from walking. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | player (P-23) | **As a** player (P-23), **I want** each mount type to have different speed, acceleration, jump height, and turn rate, **so that** mounts feel mechanically distinct. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | player (P-23) | **As a** player (P-23), **I want** dismounting to play an exit animation and restore normal locomotion, **so that** transitions feel smooth. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure per-mount physics parameters and animation sets, **so that** I can create unique riding experiences per species. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that mounted movement speed matches the configured value for each mount type, **so that** mount stats apply correctly. |  | F-13.15.3 | R-13.15.3 |
## Mounted Combat (F-13.15.3c)
| US-13.15.3 | player (P-23) | **As a** player (P-23), **I want** to use a restricted set of abilities while mounted, **so that** I can fight without always dismounting. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | player (P-23) | **As a** player (P-23), **I want** mount-specific attack animations to replace standard combat animations, **so that** combat visuals match the mounted context. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure which abilities are allowed per mount type, **so that** I can control mounted combat balance. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a disallowed ability cannot activate while mounted, **so that** the ability restriction is enforced. |  | F-13.15.3 | R-13.15.3 |
## Mount Type Variants (F-13.15.3d)
| US-13.15.3 | player (P-23) | **As a** player (P-23), **I want** ground mounts for overland travel, flying mounts for aerial traversal, and aquatic mounts for underwater movement, **so that** I can traverse any terrain. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | player (P-23) | **As a** player (P-23), **I want** flying mounts to have takeoff, landing, and altitude limit rules, **so that** aerial movement feels grounded in physics. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | player (P-23) | **As a** player (P-23), **I want** aquatic mounts to have dive and surface transitions, **so that** underwater navigation feels natural. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure movement modes, altitude limits, and transition rules per mount type, **so that** I can create diverse mount experiences. |  | F-13.15.3 | R-13.15.3 |
| US-13.15.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a flying mount cannot exceed the configured altitude ceiling, **so that** altitude restrictions are enforced. |  | F-13.15.3 | R-13.15.3 |
## Creature Taming (F-13.15.4)
| US-13.15.4.1 | player (P-23) | **As a** player (P-23), **I want** to tame wild creatures by feeding them over multiple attempts with a progress bar, **so that** taming feels like a patient investment. |  | F-13.15.4 | R-13.15.4 |
| US-13.15.4.2 | player (P-23) | **As a** player (P-23), **I want** taming success to be affected by creature level relative to my level, taming skill, and food quality, **so that** higher-level creatures are harder to tame. |  | F-13.15.4 | R-13.15.4 |
| US-13.15.4.3 | player (P-23) | **As a** player (P-23), **I want** failed taming attempts to risk the creature fleeing or attacking, **so that** taming involves meaningful risk. |  | F-13.15.4 | R-13.15.4 |
| US-13.15.4.4 | player (P-23) | **As a** player (P-23), **I want** some creatures to require specific items, quests, or reputation to tame, **so that** rare companions feel earned. |  | F-13.15.4 | R-13.15.4 |
| US-13.15.4.5 | designer (P-5) | **As a** designer (P-5), **I want** to configure tameable species, taming rules, required items, and success formulas in gameplay databases, **so that** I can balance the taming experience. |  | F-13.15.4 | R-13.15.4 |
| US-13.15.4.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that feeding a creature below the required quality tier has zero taming progress, **so that** food quality requirements are enforced. |  | F-13.15.4 | R-13.15.4 |
| US-13.15.4.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a successfully tamed creature becomes a companion or mount as configured, **so that** the taming outcome is correct. |  | F-13.15.4 | R-13.15.4 |
## Pet Life Stages (F-13.15.5a)
| US-13.15.5 | player (P-23) | **As a** player (P-23), **I want** my companion to grow through baby, juvenile, adult, and elder life stages with visible size and appearance changes, **so that** raising a pet feels rewarding. |  | F-13.15.5 | R-13.15.5 |
| US-13.15.5 | player (P-23) | **As a** player (P-23), **I want** each life stage to change the companion's stats, **so that** growth has mechanical impact. |  | F-13.15.5 | R-13.15.5 |
| US-13.15.5 | designer (P-5) | **As a** designer (P-5), **I want** to configure growth duration, stat changes, and visual transformations per stage per species, **so that** I can differentiate creature progression. |  | F-13.15.5 | R-13.15.5 |
| US-13.15.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a pet transitions from juvenile to adult at the configured time or experience threshold, **so that** stage transitions trigger correctly. |  | F-13.15.5 | R-13.15.5 |
## Pet Evolution Branching (F-13.15.5b)
| US-13.15.5 | player (P-23) | **As a** player (P-23), **I want** my pet's evolution to branch based on diet, training focus, or item usage, **so that** I can specialize my companion. |  | F-13.15.5 | R-13.15.5 |
| US-13.15.5 | player (P-23) | **As a** player (P-23), **I want** evolution choices to produce visually distinct specializations, **so that** different builds are recognizable. |  | F-13.15.5 | R-13.15.5 |
| US-13.15.5 | designer (P-5) | **As a** designer (P-5), **I want** to configure branch conditions and resulting stat specializations as data-driven assets, **so that** I can add evolution paths without code. |  | F-13.15.5 | R-13.15.5 |
| US-13.15.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a wolf pup fed exclusively meat evolves into a combat wolf rather than a tracking wolf, **so that** evolution branching follows the configured rules. |  | F-13.15.5 | R-13.15.5 |
## Pet Breeding (F-13.15.5c)
| US-13.15.5 | player (P-23) | **As a** player (P-23), **I want** to breed two compatible pets and see offspring with inherited traits from both parents, **so that** breeding is a path to stronger companions. |  | F-13.15.5 | R-13.15.5 |
| US-13.15.5 | player (P-23) | **As a** player (P-23), **I want** breeding to require a suitable environment and a gestation timer, **so that** breeding is a deliberate investment. |  | F-13.15.5 | R-13.15.5 |
| US-13.15.5 | designer (P-5) | **As a** designer (P-5), **I want** to configure species compatibility, trait inheritance rules, and random variation per species pair, **so that** I can control breeding outcomes. |  | F-13.15.5 | R-13.15.5 |
| US-13.15.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that offspring inherit color and stat bonuses from both parents within the configured variation range, **so that** the inheritance system works. |  | F-13.15.5 | R-13.15.5 |
