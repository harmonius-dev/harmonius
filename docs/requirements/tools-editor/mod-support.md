# R-15.16 -- Mod Support Requirements

## R-15.16.1 Mod SDK and Authoring Tools
The engine **SHALL** provide a standalone mod authoring toolkit derived from the game editor with
developer-configurable feature restrictions. The mod SDK SHALL load the base game's assets as
read-only references. Mods created in the SDK SHALL be compatible with the runtime mod loader
without modification.
- **Derived from:** [F-15.16.1](../../features/tools-editor/mod-support.md)
- **Rationale:** A dedicated mod SDK enables community content creation while maintaining developer
  control over what can be modified.
- **Verification:** Create a mod in the SDK that references base game assets; verify it loads in the
  game runtime; verify restricted features are inaccessible in the SDK.

## R-15.16.2 Developer-Defined Mod Constraints
The engine **SHALL** enforce mod constraints at both authoring time (restricted features greyed out
in the mod SDK) and load time (rejecting mods that exceed budgets). Constraints SHALL cover: asset
type restrictions, logic graph node restrictions, memory budget, entity budget, API surface
restrictions, and world region restrictions. Constraint violations SHALL produce specific error
messages identifying the violated constraint.
- **Derived from:** [F-15.16.2](../../features/tools-editor/mod-support.md)
- **Rationale:** Constraints prevent mods from destabilizing the game, accessing sensitive systems,
  or degrading performance.
- **Verification:** Create a mod that exceeds the entity budget; verify it is rejected at load time
  with a descriptive error. Verify restricted nodes are not available in the mod SDK.

## R-15.16.3 Mod Packaging and Distribution Format
The engine **SHALL** package mods as signed, versioned bundles using the same format as DLC packs
with additional mod metadata (author, version, compatibility range, dependencies). Unsigned mods
SHALL display a security warning before installation. Mod integrity SHALL be verified via BLAKE3
content hashes on load.
- **Derived from:** [F-15.16.3](../../features/tools-editor/mod-support.md)
- **Rationale:** A standardized mod format with signing and versioning enables safe distribution and
  dependency management.
- **Verification:** Package a mod; verify the manifest contains all required metadata. Load an
  unsigned mod; verify the security warning appears. Tamper with a mod file; verify the integrity
  check fails.

## R-15.16.4 Mod Loading and Sandboxing
The engine **SHALL** load mods into isolated ECS world partitions. Mod logic graphs SHALL execute in
a sandboxed runtime with no filesystem access, no network access, and no ability to modify engine
settings. Budget violations (memory, entity count) SHALL trigger automatic mod deactivation with a
player notification. Multiple mods SHALL load in dependency order with conflict detection for
overlapping asset modifications.
- **Derived from:** [F-15.16.4](../../features/tools-editor/mod-support.md)
- **Rationale:** Sandboxing prevents malicious or buggy mods from compromising game stability,
  player data, or system security.
- **Verification:** Load a mod that attempts filesystem access; verify the operation is blocked. Load
  a mod that exceeds its entity budget; verify it is deactivated. Load two mods modifying the same
  asset; verify a conflict prompt appears.

## R-15.16.5 Mod Workshop Integration
The engine **SHALL** integrate with Steam Workshop (via ISteamUGC) for mod upload, subscription,
download, rating, and reporting. A self-hosted mod repository SHALL be available for non-Steam
platforms using a REST API. The in-game mod browser SHALL display available mods with metadata,
ratings, and compatibility status. Mod updates SHALL be detected and applied automatically on game
launch.
- **Derived from:** [F-15.16.5](../../features/tools-editor/mod-support.md)
- **Rationale:** Workshop integration provides discoverability, automatic updates, and community
  feedback for mods.
- **Verification:** Upload a mod to Steam Workshop staging; verify it appears in the in-game
  browser; subscribe and verify automatic download and installation.

## R-15.16.6 Mod Moderation and Review
The engine **SHALL** provide a web-based moderation dashboard for reviewing submitted mods. Automated
scans SHALL check: budget compliance, restricted node usage, content policy keywords, and binary
asset integrity. Moderators SHALL be able to approve, reject, or revoke mods. Revoked mods SHALL be
force-uninstalled from all subscribers. All moderation actions SHALL be logged for audit.
- **Derived from:** [F-15.16.6](../../features/tools-editor/mod-support.md)
- **Rationale:** Moderation ensures community content meets quality and safety standards.
- **Verification:** Submit a mod containing a restricted node; verify the automated scan flags it.
  Approve a mod; verify it is marked as verified in the browser. Revoke a mod; verify it is removed
  from subscriber installations.
