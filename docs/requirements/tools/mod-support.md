# R-15.16 -- Mod Support Requirements

## Requirements

1. **R-15.16.1** — The engine **SHALL** provide a standalone mod SDK with level, material, logic
   graph, and VFX editors, loading base game assets as read-only references.
   - **Rationale:** Modders need professional tools with safe boundaries.
   - **Verification:** Open the mod SDK, verify base assets are visible but read-only.

2. **R-15.16.2** — The engine **SHALL** support per-project mod constraints for asset types, node
   access, memory, and entity budgets, enforced at both authoring and runtime.
   - **Rationale:** Constraints prevent mods from destabilizing the base game.
   - **Verification:** Set an entity budget, create a mod exceeding it, and verify rejection at both
     authoring and runtime.

3. **R-15.16.3** — The engine **SHALL** package mods as signed, versioned bundles with compatibility
   metadata, dependencies, and content hash verification.
   - **Rationale:** Signed bundles prevent tampering; versioning enables dependency resolution.
   - **Verification:** Package a mod, verify the signature, tamper with the bundle, and verify load
     fails.

4. **R-15.16.4** — The engine **SHALL** load mods into isolated ECS partitions with sandboxed logic
   graph execution, deactivating mods that exceed budgets.
   - **Rationale:** Isolation prevents mods from accessing filesystem, network, or engine internals.
   - **Verification:** Load a mod attempting filesystem access and verify the operation is blocked.

5. **R-15.16.5** — The engine **SHALL** integrate with Steam Workshop and a self-hosted mod
   repository for browsing, subscribing, downloading, and auto-updating mods.
   - **Rationale:** Mod distribution requires platform integration for discoverability and updates.
   - **Verification:** Subscribe to a mod, verify it downloads and loads on game launch.

6. **R-15.16.6** — The engine **SHALL** provide a web-based moderation dashboard with automated
   compliance scans, approval workflow, and force-uninstall capability.
   - **Rationale:** Moderation protects players from malicious or non-compliant mods.
   - **Verification:** Submit a non-compliant mod and verify the automated scan flags the violation.
