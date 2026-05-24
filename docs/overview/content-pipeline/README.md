# Content Pipeline

How source content becomes runtime-ready assets and reaches the running game.

## Topics

- [import-and-processing](./import-and-processing.md) — bringing in source files, validating,
  and converting them.
- [database-and-versioning](./database-and-versioning.md) — tracking assets, dependencies, and
  changes over time.
- [streaming-and-hot-reload](./streaming-and-hot-reload.md) — loading on demand and applying
  edits without restart.

## Key takeaways

- Source asset import validation (checking format compliance, resolving dependencies) prevents
  broken assets from reaching runtime.
- Serialization formats (compressed image containers for textures, optimized mesh codecs) compress
  size and improve load speed over raw formats.
- Asset dependency graphs enable smart rebuilding: changes to a texture trigger re-importing only
  dependent materials, not unrelated assets.
- Streaming enables progressive load: load low-res version first, stream in high-res over time,
  keeping frame time budgeted.
- Hot reload without restart enables rapid iteration: change material, reload, see result
  immediately without restarting editor.

## Integration risks

- Source format changes (artist updates DCC file, different version incompatible) require migration
  scripts. See [import-and-processing.md](./import-and-processing.md) for backward compatibility.
- Asset dependency cycles (texture A depends on material B, which depends on texture A) cause
  circular rebuilds; cycle detection necessary. See [database-and-versioning.md](./database-and-versioning.md)
  for dependency validation.
- Streaming race conditions (asset requested before loaded) require careful synchronization; loading
  must be non-blocking. See [streaming-and-hot-reload.md](./streaming-and-hot-reload.md)
  for sync patterns.
- Hot reload asset references must be updated; stale references cause crashes or incorrect visuals.
  See [streaming-and-hot-reload.md](./streaming-and-hot-reload.md) for reference patching.
- Import filters (lossy conversion, e.g., 32-bit → 16-bit) can reduce quality; configuration
  necessary per asset. See [import-and-processing.md](./import-and-processing.md) for quality
  profiles.
