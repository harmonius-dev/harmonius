# R-15.16 -- Mod Support Requirements

## Mod Authoring

### R-15.16.1 Mod SDK and Authoring Tools

The engine **SHALL** provide a standalone mod authoring
toolkit with the level editor, material editor, and logic
graph editor, loading base game assets as read-only
references, with developer-configurable feature
restrictions and distribution alongside the game or as a
separate download.

- **Derived from:**
  [F-15.16.1](../../features/tools-editor/mod-support.md)
- **Rationale:** Modders need a subset of the editor
  without full engine access.
- **Verification:** Integration test: create a mod in the
  SDK and verify it loads in the game runtime.

### R-15.16.2 Developer-Defined Mod Constraints

The engine **SHALL** support developer-configurable per-
project mod constraints for asset types, logic graph
nodes, memory budgets, entity budgets, API surface
restrictions, and world region restrictions, enforced at
both authoring time and load time.

- **Derived from:**
  [F-15.16.2](../../features/tools-editor/mod-support.md)
- **Rationale:** Mods must not degrade game performance or
  access sensitive systems.
- **Verification:** Unit test: create a mod exceeding the
  entity budget and verify it is rejected at load time.

### R-15.16.3 Mod Packaging and Distribution Format

Mods **SHALL** be packaged as signed, versioned bundles
with mod metadata (author, version, description,
compatibility range), dependency declarations, BLAKE3
content hash verification on load, and security warnings
for unsigned mods.

- **Derived from:**
  [F-15.16.3](../../features/tools-editor/mod-support.md)
- **Rationale:** Signed bundles with integrity verification
  protect players from tampered content.
- **Verification:** Unit test: tamper with a mod file and
  verify the integrity check fails.

## Mod Runtime

### R-15.16.4 Mod Loading and Sandboxing

The engine **SHALL** load mods into isolated ECS world
partitions with sandboxed logic graph execution,
restricted filesystem and network access, budget violation
auto-deactivation, dependency-ordered loading, and
ModSource component tagging on mod-spawned entities.

- **Derived from:**
  [F-15.16.4](../../features/tools-editor/mod-support.md)
- **Rationale:** Sandboxing prevents mods from
  compromising game stability or security.
- **Verification:** Unit test: attempt filesystem access
  from a mod and verify it is blocked.

## Distribution

### R-15.16.5 Mod Workshop Integration

The engine **SHALL** support uploading mods to Steam
Workshop, a self-hosted mod repository for non-Steam
platforms, an in-game mod browser with metadata and
ratings, and automatic mod update detection on launch.

- **Derived from:**
  [F-15.16.5](../../features/tools-editor/mod-support.md)
- **Rationale:** Discovery and automatic updates ensure
  players have access to the latest mods.
- **Verification:** Integration test: subscribe to a mod
  and verify automatic download.

### R-15.16.6 Mod Moderation and Review

The engine **SHALL** provide a web-based moderation
dashboard with automated scans for budget compliance and
restricted nodes, approve/reject/revoke actions,
force-uninstall of revoked mods, and audit logging of all
moderation actions.

- **Derived from:**
  [F-15.16.6](../../features/tools-editor/mod-support.md)
- **Rationale:** Moderation prevents policy-violating mods
  from reaching players.
- **Verification:** Unit test: submit a mod using a
  restricted node and verify the automated scan flags it.

---

## User Stories

## US-15.16.1 Mod SDK and Authoring Tools

### US-15.16.1.1
As a **modder (P-24)**, I want a standalone mod authoring toolkit so that I can create mods without
the full engine editor.

### US-15.16.1.2
As a **modder (P-24)**, I want the level editor, material editor, and logic graph available in the
SDK so that I can create levels, materials, and gameplay for my mod.

### US-15.16.1.3
As a **modder (P-24)**, I want base game assets loaded as read-only references so that my mod can
reference original content without modifying it.

### US-15.16.1.4
As a **developer (P-15)**, I want developer-configurable feature restrictions in the SDK so that I
control which editor capabilities modders can access.

### US-15.16.1.5
As a **DevOps (P-16)**, I want the SDK distributed alongside the game or as a separate download so
that modders can obtain it easily.

### US-15.16.1.6
As an **engine tester (P-27)**, I want to verify mods created in the SDK load in the game runtime so
that SDK-to-runtime compatibility is regression-tested.

---

## US-15.16.2 Developer-Defined Mod Constraints

### US-15.16.2.1
As a **developer (P-15)**, I want to restrict which asset types mods can create so that mods cannot
create shaders or other restricted content.

### US-15.16.2.2
As a **developer (P-15)**, I want logic graph node restrictions for mods so that mods cannot use
file I/O or network nodes.

### US-15.16.2.3
As a **developer (P-15)**, I want memory and entity budget limits for mods so that mods cannot
degrade game performance.

### US-15.16.2.4
As a **developer (P-15)**, I want API surface restrictions (which ECS components mods read/write) so
that mods cannot access sensitive game systems.

### US-15.16.2.5
As a **developer (P-15)**, I want world region restrictions for mods so that mods can only modify
designated moddable zones.

### US-15.16.2.6
As a **modder (P-24)**, I want restricted features greyed out in the mod SDK so that I understand my
authoring boundaries clearly.

### US-15.16.2.7
As an **engine tester (P-27)**, I want to verify a mod exceeding entity budget is rejected at load
time so that constraint enforcement is regression-tested.

---

## US-15.16.3 Mod Packaging and Distribution Format

### US-15.16.3.1
As a **modder (P-24)**, I want to package my mod as a signed versioned bundle so that my mod is
distributable and integrity-verified.

### US-15.16.3.2
As a **modder (P-24)**, I want mod metadata (author, version, description, compatibility) in the
package so that players can identify and evaluate my mod.

### US-15.16.3.3
As a **modder (P-24)**, I want dependency declarations on other mods so that required mods are
installed automatically.

### US-15.16.3.4
As a **developer (P-15)**, I want unsigned mods to display a security warning so that players are
informed about unverified content.

### US-15.16.3.5
As a **developer (P-15)**, I want BLAKE3 content hash verification on load so that tampered mods are
detected and rejected.

### US-15.16.3.6
As an **engine tester (P-27)**, I want to verify a tampered mod file fails integrity check so that
hash verification is regression-tested.

---

## US-15.16.4 Mod Loading and Sandboxing

### US-15.16.4.1
As a **developer (P-15)**, I want mods loaded into isolated ECS world partitions so that mods cannot
access core game systems.

### US-15.16.4.2
As a **developer (P-15)**, I want mod logic graphs sandboxed with restricted node access so that
mods cannot perform unauthorized operations.

### US-15.16.4.3
As a **developer (P-15)**, I want budget violations to trigger automatic mod deactivation so that
misbehaving mods are stopped without crashing the game.

### US-15.16.4.4
As a **developer (P-15)**, I want multiple mods loaded in dependency order with conflict detection
so that overlapping asset modifications are identified.

### US-15.16.4.5
As a **modder (P-24)**, I want mod-spawned entities tagged with ModSource component so that I can
identify which entities belong to my mod.

### US-15.16.4.6
As an **engine tester (P-27)**, I want to verify a mod attempting filesystem access is blocked so
that sandbox enforcement is regression-tested.

---

## US-15.16.5 Mod Workshop Integration

### US-15.16.5.1
As a **modder (P-24)**, I want to upload mods to Steam Workshop so that players can discover and
subscribe to my mod.

### US-15.16.5.2
As a **modder (P-24)**, I want a self-hosted mod repository for non-Steam platforms so that my mod
is available to all players.

### US-15.16.5.3
As a **designer (P-5)**, I want an in-game mod browser with metadata and ratings so that players can
discover and install mods from the main menu.

### US-15.16.5.4
As a **developer (P-15)**, I want automatic mod update detection on game launch so that players
always have the latest mod versions.

### US-15.16.5.5
As an **engine tester (P-27)**, I want to verify mod subscription and automatic download so that
workshop integration is regression-tested.

---

## US-15.16.6 Mod Moderation and Review

### US-15.16.6.1
As a **server admin (P-22)**, I want a web-based moderation dashboard so that I can review submitted
mods from any browser.

### US-15.16.6.2
As a **server admin (P-22)**, I want automated scans for budget compliance and restricted nodes so
that policy violations are detected before manual review.

### US-15.16.6.3
As a **server admin (P-22)**, I want approve, reject, and revoke actions on mods so that I can
manage mod availability.

### US-15.16.6.4
As a **server admin (P-22)**, I want force-uninstall of revoked mods from subscribers so that
policy-violating mods are removed from all players.

### US-15.16.6.5
As a **server admin (P-22)**, I want all moderation actions logged for audit so that moderation
decisions are traceable.

### US-15.16.6.6
As an **engine tester (P-27)**, I want to verify automated scan flags a restricted node so that scan
detection is regression-tested.
