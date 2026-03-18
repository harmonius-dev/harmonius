# R-13.15 -- Pets, Companions, and Mounts Requirements

| ID         | Derived From                                               |
|------------|------------------------------------------------------------|
| R-13.15.1  | [F-13.15.1](../../features/game-framework/pets-mounts.md)  |
| R-13.15.2  | [F-13.15.2](../../features/game-framework/pets-mounts.md)  |
| R-13.15.3a | [F-13.15.3a](../../features/game-framework/pets-mounts.md) |
| R-13.15.3b | [F-13.15.3b](../../features/game-framework/pets-mounts.md) |
| R-13.15.3c | [F-13.15.3c](../../features/game-framework/pets-mounts.md) |
| R-13.15.3d | [F-13.15.3d](../../features/game-framework/pets-mounts.md) |
| R-13.15.4  | [F-13.15.4](../../features/game-framework/pets-mounts.md)  |
| R-13.15.5a | [F-13.15.5a](../../features/game-framework/pets-mounts.md) |
| R-13.15.5b | [F-13.15.5b](../../features/game-framework/pets-mounts.md) |
| R-13.15.5c | [F-13.15.5c](../../features/game-framework/pets-mounts.md) |

1. **R-13.15.1** — The engine **SHALL** provide AI-controlled companion entities that follow the
   player, respond to commands (follow, guard, assist, stay, patrol, passive), navigate via
   pathfinding with configurable follow distance and teleport-on-distance-exceeded, and use the
   ability system for AI-controlled combat with stats defined in gameplay databases.
   - **Rationale:** Command-driven companion AI with database-defined stats lets designers create
     diverse companion archetypes without code changes while ensuring reliable player-following
     behavior.
   - **Verification:** Spawn a combat companion and issue each command type. Verify the companion
     follows at the configured distance, guards a designated position without wandering, assists in
     combat by activating abilities, and teleports to the player when distance exceeds the
     configured threshold. Confirm stats match the gameplay database values.
2. **R-13.15.2** — The engine **SHALL** simulate pet needs meters (hunger, happiness, cleanliness)
   that affect companion behavior and effectiveness, with care interactions restoring specific
   needs, neglect causing command refusal and eventual departure, and mood state displayed in a
   compact UI panel.
   - **Rationale:** Needs-driven mood creates an emotional care loop that deepens the player-pet
     relationship and differentiates pet quality through investment.
   - **Verification:** Spawn a pet and let all needs drain to zero. Verify the pet refuses commands
     and displays sluggish animations. Feed the pet with a high-quality food item and confirm hunger
     and happiness restore by the configured amounts. Verify the UI panel reflects the current mood
     state accurately.
3. **R-13.15.3a** — The engine **SHALL** support summoning mounts from a collection UI and
   dismissing them on command, with mount stats (speed, stamina, armor) defined per species in
   gameplay databases.
   - **Rationale:** Collection-based summoning lets players manage a mount roster and select the
     appropriate mount for each situation.
   - **Verification:** Open the mount collection UI and summon a mount. Verify the mount spawns near
     the player with stats matching the database definition. Dismiss the mount and verify it
     despawns.
4. **R-13.15.3b** — The engine **SHALL** replace player locomotion with mount-specific movement
   parameters (speed, acceleration, jump height, turn rate) on mounting, with enter/exit animation
   transitions.
   - **Rationale:** Mount-specific locomotion enables diverse traversal experiences while animation
     transitions maintain immersion during mount/dismount.
   - **Verification:** Mount a ground mount. Verify enter animation plays, the character controller
     switches to mount physics, and dismounting plays the exit animation and returns normal
     locomotion.
5. **R-13.15.3c** — The engine **SHALL** optionally enable combat while mounted, restricting
   available abilities to a configured allowed set with mount-specific attack animations.
   - **Rationale:** Mounted combat restrictions create tactical choices about when to fight mounted
     versus dismounted.
   - **Verification:** Mount a combat-enabled mount and verify only the configured ability subset is
     available. Mount a non-combat mount and verify all combat abilities are disabled while mounted.
6. **R-13.15.3d** — The engine **SHALL** support ground, flying, and aquatic mount types with
   type-specific movement modes, altitude limits, and transition rules (takeoff, landing, dive,
   surface).
   - **Rationale:** Distinct movement modes per mount type enable varied traversal across terrain,
     air, and water within a unified framework.
   - **Verification:** Mount a ground, flying, and aquatic mount in sequence. Verify each uses its
     configured movement model. Verify flying mounts respect altitude limits and aquatic mounts
     handle dive/surface transitions.
7. **R-13.15.4** — The engine **SHALL** provide a creature taming system with progress-bar-based
   taming over multiple interactions, success probability affected by creature level, player taming
   skill, and food quality, where tamed creatures become companions or mounts based on species
   configuration in gameplay databases.
   - **Rationale:** Probability-based taming with skill scaling creates a rewarding progression loop
     where higher-level creatures require greater player investment to acquire.
   - **Verification:** Attempt to tame a creature at equal level with standard food. Verify the
     progress bar advances on each feeding. Repeat at a 10-level deficit and confirm the success
     probability decreases measurably. Successfully tame a creature and verify it appears in the
     companion roster with stats matching its species definition.
8. **R-13.15.5a** — The engine **SHALL** grow companion creatures through life stages (baby,
   juvenile, adult, elder) with stat changes and visual transformations at each stage, driven by
   configurable time-based or experience-based durations.
   - **Rationale:** Life stages create visible progression that deepens player attachment and
     rewards sustained companion investment.
   - **Verification:** Raise a creature from baby to adult and verify stat increases and visual
     model changes at each stage transition.
9. **R-13.15.5b** — The engine **SHALL** support diet/training/item-based evolution branches that
   determine creature specialization, with branch conditions and resulting forms defined as
   data-driven assets.
   - **Rationale:** Evolution branches create unique companions per player, encouraging
     experimentation with different development paths.
   - **Verification:** Feed one creature meat and another herbs, then confirm they evolve into
     different specializations matching the configured branch conditions.
10. **R-13.15.5c** — The engine **SHALL** produce offspring from breeding pairs that inherit
    parental traits (color, stat bonuses, abilities) with random variation, requiring compatible
    species, a suitable environment, and a gestation timer.
    - **Rationale:** Trait inheritance with variation drives strategic breeding choices and creates
      unique companions through generational investment.
    - **Verification:** Breed two adults and verify the offspring inherits a combination of parental
      traits within the configured variation range. Verify gestation timer elapses before offspring
      appears.

## Non-Functional Requirements

| ID          | Derived From |
|-------------|--------------|
| NFR-13.15.1 |              |
| NFR-13.15.2 |              |

1. **NFR-13.15.1** — Companion behavior tree evaluation, pathfinding, and teleport-on-distance
   checks **SHALL** complete within 0.5ms per companion per frame. The system **SHALL** support up
   to 8 simultaneous active companions (party + summons) without exceeding a total of 4ms companion
   AI budget per frame.
   - **Rationale:** Players may have multiple companions active simultaneously in party-based games.
     AI processing must remain within budget to avoid frame rate impact.
   - **Verification:** Spawn 8 companions with full behavior trees and combat AI. Measure total
     companion AI frame time. Verify it stays under 4ms with all companions actively navigating and
     using abilities.
2. **NFR-13.15.2** — Mounting and dismounting **SHALL** begin the locomotion transition within 1
   frame of input. The character controller swap from player to mount physics **SHALL** complete
   within 100ms with no visible position discontinuity exceeding 0.1 world units.
   - **Rationale:** Delayed mount transitions break gameplay flow, especially in combat situations
     where mounting is used for escape or engagement.
   - **Verification:** Measure time from mount input to locomotion transition start. Verify it is
     within 1 frame. Record character position during mount swap and verify no position jump exceeds
     0.1 world units.
