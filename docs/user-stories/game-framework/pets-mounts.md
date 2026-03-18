# User Stories -- Pets, Companions, and Mounts (13.15)

## Companion AI Framework (F-13.15.1)

| ID           | Persona        | Features  | Requirements |
|--------------|----------------|-----------|--------------|
| US-13.15.1.1 | player (P-23)  | F-13.15.1 | R-13.15.1    |
| US-13.15.1.2 | player (P-23)  | F-13.15.1 | R-13.15.1    |
| US-13.15.1.3 | player (P-23)  | F-13.15.1 | R-13.15.1    |
| US-13.15.1.4 | player (P-23)  | F-13.15.1 | R-13.15.1    |
| US-13.15.1.5 | designer (P-5) | F-13.15.1 | R-13.15.1    |
| US-13.15.1.6 | designer (P-5) | F-13.15.1 | R-13.15.1    |
| US-13.15.1.7 | tester (P-27)  | F-13.15.1 | R-13.15.1    |
| US-13.15.1.8 | tester (P-27)  | F-13.15.1 | R-13.15.1    |

1. **US-13.15.1.1** — **As a** player (P-23), **I want** to issue follow, guard, assist, stay, and
   patrol commands to my companion, **so that** I can direct their behavior in exploration and
   combat.
2. **US-13.15.1.2** — **As a** player (P-23), **I want** my companion to navigate using pathfinding
   and maintain a configurable follow distance, **so that** they stay near me without blocking my
   movement.
3. **US-13.15.1.3** — **As a** player (P-23), **I want** my companion to teleport to me if they fall
   too far behind, **so that** I never lose my companion due to pathing failures.
4. **US-13.15.1.4** — **As a** player (P-23), **I want** my combat companion to use abilities
   autonomously with AI-controlled activation, **so that** they contribute to fights without
   micromanagement.
5. **US-13.15.1.5** — **As a** designer (P-5), **I want** to configure companion stats, abilities,
   follow distance, and teleport threshold in gameplay databases, **so that** I can balance
   companions per species.
6. **US-13.15.1.6** — **As a** designer (P-5), **I want** companion behavior driven by behavior
   trees with command-selected subtrees, **so that** each command maps to a distinct AI behavior
   set.
7. **US-13.15.1.7** — **As a** tester (P-27), **I want** to verify that issuing "guard" locks the
   companion to a position and they attack approaching enemies, **so that** the guard command works
   correctly.
8. **US-13.15.1.8** — **As a** tester (P-27), **I want** to verify that the companion teleports when
   the distance exceeds the threshold, **so that** the catch-up mechanism triggers reliably.

## Pet Needs and Mood (F-13.15.2)

| ID           | Persona        | Features  | Requirements |
|--------------|----------------|-----------|--------------|
| US-13.15.2.1 | player (P-23)  | F-13.15.2 | R-13.15.2    |
| US-13.15.2.2 | player (P-23)  | F-13.15.2 | R-13.15.2    |
| US-13.15.2.3 | player (P-23)  | F-13.15.2 | R-13.15.2    |
| US-13.15.2.4 | player (P-23)  | F-13.15.2 | R-13.15.2    |
| US-13.15.2.5 | player (P-23)  | F-13.15.2 | R-13.15.2    |
| US-13.15.2.6 | designer (P-5) | F-13.15.2 | R-13.15.2    |
| US-13.15.2.7 | tester (P-27)  | F-13.15.2 | R-13.15.2    |
| US-13.15.2.8 | tester (P-27)  | F-13.15.2 | R-13.15.2    |

1. **US-13.15.2.1** — **As a** player (P-23), **I want** my pet to have hunger, happiness, and
   cleanliness meters displayed in a compact UI panel, **so that** I can monitor their needs at a
   glance.
2. **US-13.15.2.2** — **As a** player (P-23), **I want** a fed and happy pet to fight better and
   follow more responsively, **so that** caring for my pet improves its effectiveness.
3. **US-13.15.2.3** — **As a** player (P-23), **I want** a neglected pet to become sluggish and
   refuse commands, **so that** pet care has meaningful gameplay consequences.
4. **US-13.15.2.4** — **As a** player (P-23), **I want** to feed, pet, play with, and bathe my
   companion through interactions, **so that** I can actively maintain their happiness.
5. **US-13.15.2.5** — **As a** player (P-23), **I want** pet food items to have quality tiers
   affecting hunger restoration and happiness bonus, **so that** food quality matters.
6. **US-13.15.2.6** — **As a** designer (P-5), **I want** to configure need drain rates, care action
   effects, and mood thresholds per pet species, **so that** I can differentiate pet maintenance
   difficulty.
7. **US-13.15.2.7** — **As a** tester (P-27), **I want** to verify that a pet at zero happiness
   refuses commands, **so that** the neglect penalty functions correctly.
8. **US-13.15.2.8** — **As a** tester (P-27), **I want** to verify that extended neglect causes the
   pet to run away, **so that** the abandonment consequence triggers.

## Mount Summoning and Dismissal (F-13.15.3a)

| ID         | Persona        | Features  | Requirements |
|------------|----------------|-----------|--------------|
| US-13.15.3 | player (P-23)  | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | player (P-23)  | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | designer (P-5) | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | tester (P-27)  | F-13.15.3 | R-13.15.3    |

1. **US-13.15.3** — **As a** player (P-23), **I want** to open a collection UI showing all my owned
   mounts, **so that** I can browse and select which mount to summon.
2. **US-13.15.3** — **As a** player (P-23), **I want** summoning to spawn the mount near me and
   dismissal to despawn it on command, **so that** I can quickly access or stow my mount.
3. **US-13.15.3** — **As a** designer (P-5), **I want** to configure mount stats (speed, stamina,
   armor) per species in gameplay databases, **so that** mount variety creates meaningful choices.
4. **US-13.15.3** — **As a** tester (P-27), **I want** to verify that summoning a mount in a
   no-mount zone fails with an appropriate error message, **so that** zone restrictions are
   enforced.

## Mounted Locomotion (F-13.15.3b)

| ID         | Persona        | Features  | Requirements |
|------------|----------------|-----------|--------------|
| US-13.15.3 | player (P-23)  | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | player (P-23)  | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | player (P-23)  | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | designer (P-5) | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | tester (P-27)  | F-13.15.3 | R-13.15.3    |

1. **US-13.15.3** — **As a** player (P-23), **I want** mounting to play an enter animation and
   switch my character controller to mount physics, **so that** riding feels distinct from walking.
2. **US-13.15.3** — **As a** player (P-23), **I want** each mount type to have different speed,
   acceleration, jump height, and turn rate, **so that** mounts feel mechanically distinct.
3. **US-13.15.3** — **As a** player (P-23), **I want** dismounting to play an exit animation and
   restore normal locomotion, **so that** transitions feel smooth.
4. **US-13.15.3** — **As a** designer (P-5), **I want** to configure per-mount physics parameters
   and animation sets, **so that** I can create unique riding experiences per species.
5. **US-13.15.3** — **As a** tester (P-27), **I want** to verify that mounted movement speed matches
   the configured value for each mount type, **so that** mount stats apply correctly.

## Mounted Combat (F-13.15.3c)

| ID         | Persona        | Features  | Requirements |
|------------|----------------|-----------|--------------|
| US-13.15.3 | player (P-23)  | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | player (P-23)  | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | designer (P-5) | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | tester (P-27)  | F-13.15.3 | R-13.15.3    |

1. **US-13.15.3** — **As a** player (P-23), **I want** to use a restricted set of abilities while
   mounted, **so that** I can fight without always dismounting.
2. **US-13.15.3** — **As a** player (P-23), **I want** mount-specific attack animations to replace
   standard combat animations, **so that** combat visuals match the mounted context.
3. **US-13.15.3** — **As a** designer (P-5), **I want** to configure which abilities are allowed per
   mount type, **so that** I can control mounted combat balance.
4. **US-13.15.3** — **As a** tester (P-27), **I want** to verify that a disallowed ability cannot
   activate while mounted, **so that** the ability restriction is enforced.

## Mount Type Variants (F-13.15.3d)

| ID         | Persona        | Features  | Requirements |
|------------|----------------|-----------|--------------|
| US-13.15.3 | player (P-23)  | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | player (P-23)  | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | player (P-23)  | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | designer (P-5) | F-13.15.3 | R-13.15.3    |
| US-13.15.3 | tester (P-27)  | F-13.15.3 | R-13.15.3    |

1. **US-13.15.3** — **As a** player (P-23), **I want** ground mounts for overland travel, flying
   mounts for aerial traversal, and aquatic mounts for underwater movement, **so that** I can
   traverse any terrain.
2. **US-13.15.3** — **As a** player (P-23), **I want** flying mounts to have takeoff, landing, and
   altitude limit rules, **so that** aerial movement feels grounded in physics.
3. **US-13.15.3** — **As a** player (P-23), **I want** aquatic mounts to have dive and surface
   transitions, **so that** underwater navigation feels natural.
4. **US-13.15.3** — **As a** designer (P-5), **I want** to configure movement modes, altitude
   limits, and transition rules per mount type, **so that** I can create diverse mount experiences.
5. **US-13.15.3** — **As a** tester (P-27), **I want** to verify that a flying mount cannot exceed
   the configured altitude ceiling, **so that** altitude restrictions are enforced.

## Creature Taming (F-13.15.4)

| ID           | Persona        | Features  | Requirements |
|--------------|----------------|-----------|--------------|
| US-13.15.4.1 | player (P-23)  | F-13.15.4 | R-13.15.4    |
| US-13.15.4.2 | player (P-23)  | F-13.15.4 | R-13.15.4    |
| US-13.15.4.3 | player (P-23)  | F-13.15.4 | R-13.15.4    |
| US-13.15.4.4 | player (P-23)  | F-13.15.4 | R-13.15.4    |
| US-13.15.4.5 | designer (P-5) | F-13.15.4 | R-13.15.4    |
| US-13.15.4.6 | tester (P-27)  | F-13.15.4 | R-13.15.4    |
| US-13.15.4.7 | tester (P-27)  | F-13.15.4 | R-13.15.4    |

1. **US-13.15.4.1** — **As a** player (P-23), **I want** to tame wild creatures by feeding them over
   multiple attempts with a progress bar, **so that** taming feels like a patient investment.
2. **US-13.15.4.2** — **As a** player (P-23), **I want** taming success to be affected by creature
   level relative to my level, taming skill, and food quality, **so that** higher-level creatures
   are harder to tame.
3. **US-13.15.4.3** — **As a** player (P-23), **I want** failed taming attempts to risk the creature
   fleeing or attacking, **so that** taming involves meaningful risk.
4. **US-13.15.4.4** — **As a** player (P-23), **I want** some creatures to require specific items,
   quests, or reputation to tame, **so that** rare companions feel earned.
5. **US-13.15.4.5** — **As a** designer (P-5), **I want** to configure tameable species, taming
   rules, required items, and success formulas in gameplay databases, **so that** I can balance the
   taming experience.
6. **US-13.15.4.6** — **As a** tester (P-27), **I want** to verify that feeding a creature below the
   required quality tier has zero taming progress, **so that** food quality requirements are
   enforced.
7. **US-13.15.4.7** — **As a** tester (P-27), **I want** to verify that a successfully tamed
   creature becomes a companion or mount as configured, **so that** the taming outcome is correct.

## Pet Life Stages (F-13.15.5a)

| ID         | Persona        | Features  | Requirements |
|------------|----------------|-----------|--------------|
| US-13.15.5 | player (P-23)  | F-13.15.5 | R-13.15.5    |
| US-13.15.5 | player (P-23)  | F-13.15.5 | R-13.15.5    |
| US-13.15.5 | designer (P-5) | F-13.15.5 | R-13.15.5    |
| US-13.15.5 | tester (P-27)  | F-13.15.5 | R-13.15.5    |

1. **US-13.15.5** — **As a** player (P-23), **I want** my companion to grow through baby, juvenile,
   adult, and elder life stages with visible size and appearance changes, **so that** raising a pet
   feels rewarding.
2. **US-13.15.5** — **As a** player (P-23), **I want** each life stage to change the companion's
   stats, **so that** growth has mechanical impact.
3. **US-13.15.5** — **As a** designer (P-5), **I want** to configure growth duration, stat changes,
   and visual transformations per stage per species, **so that** I can differentiate creature
   progression.
4. **US-13.15.5** — **As a** tester (P-27), **I want** to verify that a pet transitions from
   juvenile to adult at the configured time or experience threshold, **so that** stage transitions
   trigger correctly.

## Pet Evolution Branching (F-13.15.5b)

| ID         | Persona        | Features  | Requirements |
|------------|----------------|-----------|--------------|
| US-13.15.5 | player (P-23)  | F-13.15.5 | R-13.15.5    |
| US-13.15.5 | player (P-23)  | F-13.15.5 | R-13.15.5    |
| US-13.15.5 | designer (P-5) | F-13.15.5 | R-13.15.5    |
| US-13.15.5 | tester (P-27)  | F-13.15.5 | R-13.15.5    |

1. **US-13.15.5** — **As a** player (P-23), **I want** my pet's evolution to branch based on diet,
   training focus, or item usage, **so that** I can specialize my companion.
2. **US-13.15.5** — **As a** player (P-23), **I want** evolution choices to produce visually
   distinct specializations, **so that** different builds are recognizable.
3. **US-13.15.5** — **As a** designer (P-5), **I want** to configure branch conditions and resulting
   stat specializations as data-driven assets, **so that** I can add evolution paths without code.
4. **US-13.15.5** — **As a** tester (P-27), **I want** to verify that a wolf pup fed exclusively
   meat evolves into a combat wolf rather than a tracking wolf, **so that** evolution branching
   follows the configured rules.

## Pet Breeding (F-13.15.5c)

| ID         | Persona        | Features  | Requirements |
|------------|----------------|-----------|--------------|
| US-13.15.5 | player (P-23)  | F-13.15.5 | R-13.15.5    |
| US-13.15.5 | player (P-23)  | F-13.15.5 | R-13.15.5    |
| US-13.15.5 | designer (P-5) | F-13.15.5 | R-13.15.5    |
| US-13.15.5 | tester (P-27)  | F-13.15.5 | R-13.15.5    |

1. **US-13.15.5** — **As a** player (P-23), **I want** to breed two compatible pets and see
   offspring with inherited traits from both parents, **so that** breeding is a path to stronger
   companions.
2. **US-13.15.5** — **As a** player (P-23), **I want** breeding to require a suitable environment
   and a gestation timer, **so that** breeding is a deliberate investment.
3. **US-13.15.5** — **As a** designer (P-5), **I want** to configure species compatibility, trait
   inheritance rules, and random variation per species pair, **so that** I can control breeding
   outcomes.
4. **US-13.15.5** — **As a** tester (P-27), **I want** to verify that offspring inherit color and
   stat bonuses from both parents within the configured variation range, **so that** the inheritance
   system works.
