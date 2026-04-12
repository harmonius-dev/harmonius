# User Stories -- Pets, Companions, and Mounts (13.15)

## Companions

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.15.1.1  | game designer (P-5)     |
| US-13.15.1.2  | game developer (P-15)   |
| US-13.15.1.3  | player (P-23)           |
| US-13.15.1.4  | player (P-23)           |
| US-13.15.2.1  | game designer (P-5)     |
| US-13.15.2.2  | player (P-23)           |
| US-13.15.2.3  | player (P-23)           |

1. **US-13.15.1.1** -- **As a** game designer (P-5), **I want** to configure companion stats,
   abilities, follow distance, and teleport threshold in gameplay databases, **so that** I can
   balance companions per species.

2. **US-13.15.1.2** -- **As a** game developer (P-15), **I want** companion behavior driven by
   behavior trees with command-selected subtrees, **so that** each command maps to a distinct AI
   behavior set.

3. **US-13.15.1.3** -- [game-specific] **As a** player (P-23), **I want** to issue follow, guard,
   assist, stay, and patrol commands to my companion, **so that** I direct their behavior in
   exploration and combat.

4. **US-13.15.1.4** -- [game-specific] **As a** player (P-23), **I want** my companion to teleport
   to me if they fall too far behind, **so that** I never lose my companion due to pathing failures.

5. **US-13.15.2.1** -- **As a** game designer (P-5), **I want** to configure need drain rates, care
   action effects, and mood thresholds per pet species, **so that** I differentiate pet maintenance
   difficulty.

6. **US-13.15.2.2** -- [game-specific] **As a** player (P-23), **I want** my pet to have hunger,
   happiness, and cleanliness meters displayed in a compact UI panel, **so that** I monitor their
   needs at a glance.

7. **US-13.15.2.3** -- [game-specific] **As a** player (P-23), **I want** a fed and happy pet to
   fight better and follow more responsively, **so that** caring for my pet improves its
   effectiveness.

## Mounts

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.15.3.1  | game designer (P-5)     |
| US-13.15.3.2  | game developer (P-15)   |
| US-13.15.3.3  | player (P-23)           |
| US-13.15.3.4  | player (P-23)           |
| US-13.15.3.5  | player (P-23)           |

1. **US-13.15.3.1** -- **As a** game designer (P-5), **I want** to configure per-mount physics
   parameters, animation sets, and combat ability restrictions, **so that** each mount type feels
   mechanically distinct.

2. **US-13.15.3.2** -- **As a** game developer (P-15), **I want** mounting to switch the character
   controller to mount-specific physics with distinct speed, acceleration, and turn rate,
   **so that** riding feels different from walking.

3. **US-13.15.3.3** -- [game-specific] **As a** player (P-23), **I want** to summon mounts from a
   collection UI and dismiss them on command, **so that** I quickly access or stow my mount.

4. **US-13.15.3.4** -- [game-specific] **As a** player (P-23), **I want** ground, flying, and
   aquatic mount types with type-specific movement modes, **so that** I can traverse any terrain.

5. **US-13.15.3.5** -- [game-specific] **As a** player (P-23), **I want** to use a restricted set of
   abilities while mounted with mount-specific attack animations, **so that** I can fight without
   always dismounting.

## Taming, Growth, and Breeding

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.15.4.1  | game designer (P-5)     |
| US-13.15.4.2  | player (P-23)           |
| US-13.15.4.3  | player (P-23)           |
| US-13.15.5.1  | game designer (P-5)     |
| US-13.15.5.2  | player (P-23)           |
| US-13.15.5.3  | player (P-23)           |
| US-13.15.5.4  | player (P-23)           |

1. **US-13.15.4.1** -- **As a** game designer (P-5), **I want** to configure tameable species,
   taming rules, required items, and success formulas in gameplay databases, **so that** taming is
   balanced via data.

2. **US-13.15.4.2** -- [game-specific] **As a** player (P-23), **I want** to tame wild creatures by
   feeding them over multiple attempts with a progress bar, **so that** taming feels like a patient
   investment.

3. **US-13.15.4.3** -- [game-specific] **As a** player (P-23), **I want** failed taming attempts to
   risk the creature fleeing or attacking, **so that** taming involves meaningful risk.

4. **US-13.15.5.1** -- **As a** game designer (P-5), **I want** to configure life stage durations,
   stat changes, evolution branch conditions, and breeding genetics per species, **so that**
   creature progression is data-driven.

5. **US-13.15.5.2** -- [game-specific] **As a** player (P-23), **I want** my companion to grow
   through baby, juvenile, adult, and elder stages with visible changes, **so that** raising a pet
   feels rewarding.

6. **US-13.15.5.3** -- [game-specific] **As a** player (P-23), **I want** my pet's evolution to
   branch based on diet, training, or item usage, **so that** I can specialize my companion.

7. **US-13.15.5.4** -- [game-specific] **As a** player (P-23), **I want** to breed two compatible
   pets and see offspring with inherited traits from both parents, **so that** breeding is a path to
   stronger companions.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-13.15.1 | game designer (P-5) |
| US-13.15.2 | game designer (P-5) |
| US-13.15.3 | game designer (P-5) |
| US-13.15.4 | game designer (P-5) |
| US-13.15.5 | game designer (P-5) |

1. **US-13.15.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.15.1.1 through US-13.15.1.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-13.15.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.15.2.1 through US-13.15.2.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-13.15.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.15.3.1 through US-13.15.3.5 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-13.15.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.15.4.1 through US-13.15.4.3 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-13.15.5** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-13.15.5.1 through US-13.15.5.4 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
