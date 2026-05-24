# Game Framework

Shared services and gameplay compositions every project uses on top of the generic primitives.

## Topics

- [camera-and-controls](./camera-and-controls.md) — cameras, follow modes, traversal, and
  interaction.
- [save-and-persistence](./save-and-persistence.md) — saving, loading, and persisting world state.
- [authoring-and-scripting](./authoring-and-scripting.md) — visual scripting and gameplay logic
  authoring.
- [world-management](./world-management.md) — streaming worlds, levels, regions, and game modes.
- [gameplay-features](./gameplay-features.md) — abilities, inventory, quests, progression,
  combat, and other compositions of the data primitives.
- [genres-and-modes](./genres-and-modes.md) — how the framework supports specific genres
  (turn-based, racing, building, social, voxel, minigames).

## Key takeaways

- Decoupled camera (separate from character position/rotation) enables third-person orbiting, dynamic
  zoom, and obstacle avoidance without player feeling passive.
- Save file versioning enables backward compatibility; old saves load in new game versions with
  migration logic updating old formats.
- Event-driven scripting (scripts listen for events rather than polling) decouples systems enabling
  modular, reusable behavior without tight coupling.
- Hierarchical streaming (zones → regions → chunks) manages memory by loading only nearby content;
  unloading distant content prevents unbounded growth.
- Ability cooldowns and resource costs (mana, stamina) limit player spamming; progression and
  difficulty scaling adjust challenge with player advancement.

## Integration risks

- Camera collision (preventing clipping into walls) must work with player collision; overlapping
  geometry can trap camera. See [camera-and-controls.md](./camera-and-controls.md) for collision
  resolution.
- Save file corruption detection must have fallback (load previous save or restart); no fallback
  causes progression loss. See [save-and-persistence.md](./save-and-persistence.md) for recovery
  strategies.
- Event listener lifetimes must be managed; listeners outliving event source cause dangling pointers
  or crash. See [authoring-and-scripting.md](./authoring-and-scripting.md) for listener
  lifecycle.
- World streaming chunk boundaries must align; misaligned chunk edges cause gaps or overlaps. See
  [world-management.md](./world-management.md) for chunk boundary validation.
- Ability cooldown synchronization across network clients must match; diverged cooldowns cause
  desync. See [gameplay-features.md](./gameplay-features.md) for cooldown replication.
- Genre-specific difficulty scaling (stat multipliers, behavior tweaks) must not break intended
  challenge; over-scaling trivializes content. See [genres-and-modes.md](./genres-and-modes.md)
  for scaling curves.
