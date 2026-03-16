# R-15.16 -- Mod Support Requirements

## Mod Authoring

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.16.1 | The engine **SHALL** provide a standalone mod authoring toolkit with the level editor, material editor, and logic graph editor, loading base game assets as read-only references, with developer-configurable feature restrictions and distribution alongside the game or as a separate download. | [F-15.16.1](../../features/tools-editor/mod-support.md) | Modders need a subset of the editor without full engine access. | Integration test: create a mod in the SDK and verify it loads in the game runtime. |
| R-15.16.2 | The engine **SHALL** support developer-configurable per-project mod constraints for asset types, logic graph nodes, memory budgets, entity budgets, API surface restrictions, and world region restrictions, enforced at both authoring time and load time. | [F-15.16.2](../../features/tools-editor/mod-support.md) | Mods must not degrade game performance or access sensitive systems. | Unit test: create a mod exceeding the entity budget and verify it is rejected at load time. |
| R-15.16.3 | Mods **SHALL** be packaged as signed, versioned bundles with mod metadata (author, version, description, compatibility range), dependency declarations, BLAKE3 content hash verification on load, and security warnings for unsigned mods. | [F-15.16.3](../../features/tools-editor/mod-support.md) | Signed bundles with integrity verification protect players from tampered content. | Unit test: tamper with a mod file and verify the integrity check fails. |

## Mod Runtime

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.16.4 | The engine **SHALL** load mods into isolated ECS world partitions with sandboxed logic graph execution, restricted filesystem and network access, budget violation auto-deactivation, dependency-ordered loading, and ModSource component tagging on mod-spawned entities. | [F-15.16.4](../../features/tools-editor/mod-support.md) | Sandboxing prevents mods from compromising game stability or security. | Unit test: attempt filesystem access from a mod and verify it is blocked. |

## Distribution

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.16.5 | The engine **SHALL** support uploading mods to Steam Workshop, a self-hosted mod repository for non-Steam platforms, an in-game mod browser with metadata and ratings, and automatic mod update detection on launch. | [F-15.16.5](../../features/tools-editor/mod-support.md) | Discovery and automatic updates ensure players have access to the latest mods. | Integration test: subscribe to a mod and verify automatic download. |
| R-15.16.6 | The engine **SHALL** provide a web-based moderation dashboard with automated scans for budget compliance and restricted nodes, approve/reject/revoke actions, force-uninstall of revoked mods, and audit logging of all moderation actions. | [F-15.16.6](../../features/tools-editor/mod-support.md) | Moderation prevents policy-violating mods from reaching players. | Unit test: submit a mod using a restricted node and verify the automated scan flags it. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/mod-support.md](../../user-stories/tools-editor/mod-support.md).
Requirements in this document are derived from those user stories.
