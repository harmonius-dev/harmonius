# 15.16 — Mod Support

## Mod Authoring

### F-15.16.1 Mod SDK and Authoring Tools

A subset of the game engine editor packaged as a standalone mod authoring toolkit. The mod SDK
includes: the level editor with placement and terrain tools, the material editor, the logic
graph editor for gameplay scripting, the VFX effect graph editor, the UI widget editor, and
the asset import pipeline. Developers define which editor features are available to modders
through a mod configuration asset — restricting access to engine internals, core game systems,
or sensitive data. The mod SDK loads the base game's assets as read-only references, allowing
mods to create new content that references but cannot modify original assets. The SDK is
distributed alongside the game or as a separate free download.

- **Requirements:** R-15.16.1
- **Dependencies:** F-15.1.1 (Editor Framework), F-15.8.1 (Logic Graph Runtime),
  F-15.15.4 (Project File Format)
- **Platform notes:** The mod SDK ships as a standalone application using the same binary as
  the editor with a mod-mode flag that restricts available features.

### F-15.16.2 Developer-Defined Mod Constraints

Developers configure per-project constraints that limit what mods can do. Constraint
categories: asset type restrictions (mods can create materials but not shaders), logic graph
node restrictions (mods can use gameplay nodes but not file I/O or network nodes), memory
budget (maximum total mod asset size), entity budget (maximum entities a mod can spawn), API
surface restrictions (which ECS components mods can read/write), and world region restrictions
(mods can only modify designated moddable zones). Constraints are defined in a mod policy
asset edited in the visual editor. The mod SDK enforces constraints at authoring time
(greyed-out restricted features) and the runtime enforces them at load time (rejecting mods
that exceed budgets).

- **Requirements:** R-15.16.2
- **Dependencies:** F-15.16.1, F-13.1.9 (Modular System Registration)
- **Platform notes:** Mod authoring is desktop only. Runtime mod loading is available on all
  platforms but console mods require platform holder approval.

### F-15.16.3 Mod Packaging and Distribution Format

Mods are packaged as signed, versioned bundles in the same format as DLC packs (F-15.14.6)
with additional mod-specific metadata: mod name, author, description, version, base game
version compatibility range, dependencies on other mods, preview images, and change log. The
mod package format uses the engine's binary asset format (F-12.7.1) with an additional mod
manifest file. Mod packages are integrity-verified via content hashes on load. Unsigned mods
display a security warning to players before installation.

- **Requirements:** R-15.16.3
- **Dependencies:** F-15.14.6 (Asset Bundles), F-12.7.1 (Binary Asset Format)
- **Platform notes:** Desktop only. Mod packaging tools are not available on mobile or
  console.

## Mod Runtime

### F-15.16.4 Mod Loading and Sandboxing

Load mod packages at runtime into an isolated ECS world partition that prevents mods from
accessing or modifying the base game's core systems. Mod-spawned entities are tagged with a
`ModSource` component identifying their origin mod. Mod logic graphs execute in a sandboxed
runtime (F-15.8.1) with restricted node access per the mod policy (F-15.16.2). Mods cannot:
access the filesystem, make network requests, modify engine settings, or exceed their
memory/entity budgets. Budget violations trigger mod deactivation with a player-facing
notification. Multiple mods load in dependency order with conflict detection — two mods
modifying the same asset trigger a load-order resolution prompt.

- **Requirements:** R-15.16.4
- **Dependencies:** F-15.16.2, F-15.8.1 (Logic Graph Runtime),
  F-1.1.34 (Multiple World Support)
- **Platform notes:** Mobile enforces tighter mod memory and entity budgets. Console mod
  loading requires platform holder certification.

### F-15.16.5 Mod Workshop Integration

Integrate with platform mod distribution services: Steam Workshop (upload, subscribe, download,
rate, report), and a self-hosted mod repository for non-Steam platforms. The mod browser UI
(accessible from the game's main menu) displays available mods with thumbnails, descriptions,
ratings, download counts, and compatibility status. Players subscribe to mods; the engine
downloads, verifies, and installs them automatically. Mod updates are detected and applied on
game launch. The self-hosted repository uses a REST API served by the same cloud infrastructure
as the collaboration service (F-15.12.7).

- **Requirements:** R-15.16.5
- **Dependencies:** F-15.16.3 (Mod Packaging), F-15.12.7 (Cloud Service),
  F-14.5.1 (Platform Services)
- **Platform notes:** Steam Workshop uses the ISteamUGC API. Self-hosted repository requires
  the Harmonius cloud service deployment.

### F-15.16.6 Mod Moderation and Review

Developer-side tools for reviewing, approving, and managing player-created mods. A moderation
dashboard (web-based, served by the cloud service) displays submitted mods with automated scan
results: asset budget compliance, restricted node usage detection, content policy keyword
scanning, and virus/malware scanning of binary assets. Moderators approve, reject (with
reason), or request changes. Approved mods are flagged as verified in the mod browser.
Developers can revoke approval and force-uninstall mods that violate policies
post-publication. Player reporting feeds into the moderation queue. Moderation actions are
logged for audit.

- **Requirements:** R-15.16.6
- **Dependencies:** F-15.16.5, F-15.16.2 (Mod Constraints), F-15.12.7 (Cloud Service)
- **Platform notes:** Desktop only. Moderation dashboard is a web-based tool accessed from
  desktop browsers.
