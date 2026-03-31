# User Stories -- 15.16 Mod Support

## Stories

| ID           | Persona                    |
|--------------|----------------------------|
| US-15.16.1.1 | game designer (P-5)        |
| US-15.16.1.2 | extension developer (P-25) |
| US-15.16.2.1 | game designer (P-5)        |
| US-15.16.2.2 | engine developer (P-26)    |
| US-15.16.3.1 | extension developer (P-25) |
| US-15.16.3.2 | build engineer (P-16)      |
| US-15.16.4.1 | engine developer (P-26)    |
| US-15.16.4.2 | game designer (P-5)        |
| US-15.16.5.1 | game designer (P-5)        |
| US-15.16.5.2 | extension developer (P-25) |
| US-15.16.6.1 | build engineer (P-16)      |
| US-15.16.6.2 | game designer (P-5)        |

1. **US-15.16.1.1** — **As a** game designer (P-5), **I want** a standalone mod SDK with level,
   material, logic graph, and VFX editors, **so that** modders create content with professional
   tools.

2. **US-15.16.1.2** — **As a** extension developer (P-25), **I want** the mod SDK to load base game
   assets as read-only, **so that** mods reference but cannot modify originals.

3. **US-15.16.2.1** — **As a** game designer (P-5), **I want** per-project mod constraints for asset
   types, node access, memory, and entity budgets, **so that** mods cannot destabilize the game.

4. **US-15.16.2.2** — **As a** engine developer (P-26), **I want** constraints enforced at authoring
   time (greyed-out features) and runtime (rejection on budget exceed), **so that** policy is
   enforced at both stages.

5. **US-15.16.3.1** — **As a** extension developer (P-25), **I want** mods packaged as signed,
   versioned bundles with compatibility metadata and dependency declarations, **so that** mods
   install and update reliably.

6. **US-15.16.3.2** — **As a** build engineer (P-16), **I want** unsigned mods to display a security
   warning before install, **so that** players are informed of risk.

7. **US-15.16.4.1** — **As a** engine developer (P-26), **I want** mods loaded into isolated ECS
   world partitions with sandboxed logic graph execution, **so that** mods cannot access filesystem,
   network, or engine settings.

8. **US-15.16.4.2** — **As a** game designer (P-5), **I want** budget violations to deactivate the
   offending mod with player notification, **so that** the game remains stable.

9. **US-15.16.5.1** — **As a** game designer (P-5), **I want** mod browser integration with Steam
   Workshop and a self-hosted repository, **so that** players discover and install mods from the
   game menu.

10. **US-15.16.5.2** — **As a** extension developer (P-25), **I want** mod updates detected and
    applied on game launch, **so that** players always have the latest version.

11. **US-15.16.6.1** — **As a** build engineer (P-16), **I want** a web-based moderation dashboard
    with automated scans for budget compliance and policy violations, **so that** review is
    efficient.

12. **US-15.16.6.2** — **As a** game designer (P-5), **I want** to revoke approval and
    force-uninstall mods violating policies, **so that** players are protected.
