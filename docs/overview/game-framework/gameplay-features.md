# Gameplay Features

Abilities, combat, progression systems, and game loops.

## What it covers

- Ability systems: skills with cooldowns and resource costs.
- Cooldown management: tracking ability availability.
- Resource systems: mana, stamina, ammo as limiting factors.
- Damage and healing: applying and recovering health.
- Status effects: buffs and debuffs modifying character properties.
- Loot systems: items dropped on enemy death.
- Crafting: combining items to create new items.
- Progression rewards: experience, unlocks, cosmetics.
- Difficulty scaling: adjusting challenge based on player level.
- Game loop: core gameplay cycle (explore, encounter, overcome).

## Concepts

### Ability Systems

Abilities (skills, spells) have costs (mana, cooldown), effects (damage, heal, buff), and targets
(self, enemy, area). Ability queuing buffers input: rapid ability presses queue for execution.
Cooldowns prevent spamming: ability has N-second cooldown before use again. Channeled abilities lock
character in place during casting; interruption cancels. Global cooldowns (brief cooldown after any
ability) add strategic pausing.

### Resources and Limiting Factors

Resources (mana, stamina, ammo) gate ability usage. Abilities cost resources; spending resources
limits spam. Regeneration passively restores resources. Consuming all resources (out of mana)
prevents spellcasting until regenerated. This creates tactical resource management.

### Status Effects

Status effects modify character properties: burn (damage over time), slow (reduced speed), haste
(increased speed). Effects stack or replace: two haste effects don't exceed 2× speed. Effects have
duration; expiration removes the effect. Cleansing abilities remove effects. Effect prioritization
(e.g., stun overrides slow) determines visual feedback.

### Loot and Progression

Defeated enemies drop items (loot). Loot rarity (common, rare, legendary) affects value and power.
Rarity distribution follows a curve: common loot frequently, legendary rarely. Loot adapts to
player level: defeating level 50 enemies drops level 50 loot. Crafting combines items: ore + coal
= smelted bar. Crafting enables vertical progression: better gear via effort.

### Difficulty and Scaling

Difficulty modes adjust challenge: easy reduces enemy health and damage; hard increases both.
Scaling adjusts based on player level: level 10 dungeon is harder at level 10, easier at level 50.
Scaling keeps challenge appropriate. Scaling factor (1.2× per level above minimum) balances
advancement vs difficulty.

## How it fits

- See [genres-and-modes.md](./genres-and-modes.md) for genre-specific gameplay.
- See [authoring-and-scripting.md](./authoring-and-scripting.md) for ability and progression
  scripting.
- See [../data-systems/attributes-and-effects.md](../data-systems/attributes-and-effects.md)
  for attribute and effect data.
