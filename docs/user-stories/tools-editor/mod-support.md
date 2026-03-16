# User Stories: Mod Support

## F-15.16.1 Mod SDK and Authoring Tools

## US-15.16.1.1 Modder Authors Content in Mod SDK

**As a** modder (P-24), **I want** a standalone mod authoring toolkit with the level editor,
material editor, logic graph editor, VFX editor, and asset import pipeline, **so that** I can create
new content without access to the full engine or source code.

## US-15.16.1.2 Developer Configures SDK Feature Set

**As a** developer (P-15), **I want** to define which editor features are available to modders
through a mod configuration asset, **so that** I can restrict access to engine internals and
sensitive game systems.

## US-15.16.1.3 Modder References Base Game Assets

**As a** modder (P-24), **I want** the mod SDK to load the base game's assets as read-only
references, **so that** I can create content that references but cannot modify original game assets.

## US-15.16.1.4 Engine Tester Validates SDK Isolation

**As an** engine tester (P-27), **I want** to verify that the mod SDK runs as a standalone
application with a mod-mode flag restricting features correctly, **so that** modders cannot access
restricted editor capabilities.

## F-15.16.2 Developer-Defined Mod Constraints

## US-15.16.2.1 Developer Defines Asset Type Restrictions

**As a** developer (P-15), **I want** to configure per-project constraints limiting which asset
types mods can create, which logic graph nodes they can use, and which ECS components they can
read/write, **so that** mods stay within safe boundaries.

## US-15.16.2.2 Designer Sets Memory and Entity Budgets

**As a** designer (P-5), **I want** to set memory budget and entity budget limits per mod,
**so that** no single mod can consume resources that degrade game performance.

## US-15.16.2.3 Server Admin Restricts Moddable Zones

**As a** server admin (P-22), **I want** world region restrictions so mods can only modify
designated moddable zones, **so that** core game areas remain unmodified by player content.

## US-15.16.2.4 Engine Tester Validates Authoring Enforcement

**As an** engine tester (P-27), **I want** to verify that constraints are enforced at authoring time
(greyed-out restricted features) and at load time (rejecting over-budget mods), **so that**
constraint violations are caught at both stages.

## F-15.16.3 Mod Packaging and Distribution Format

## US-15.16.3.1 Modder Packages Signed Mod Bundle

**As a** modder (P-24), **I want** to package my mod as a signed, versioned bundle with metadata
(name, author, version, compatibility range, dependencies, preview images, change log), **so that**
my mod is distributable and verifiable.

## US-15.16.3.2 Developer Verifies Mod Integrity

**As a** developer (P-15), **I want** mod packages integrity-verified via content hashes on load,
**so that** corrupted or tampered mods are detected before they execute.

## US-15.16.3.3 DevOps Validates Mod Format

**As a** DevOps engineer (P-16), **I want** mod packages to use the engine's binary asset format
with a mod manifest file, **so that** the existing asset pipeline handles mod content without
special-case code.

## US-15.16.3.4 Engine Tester Validates Security Warning

**As an** engine tester (P-27), **I want** unsigned mods to display a security warning before
installation, **so that** players are informed when installing unverified content.

## F-15.16.4 Mod Loading and Sandboxing

## US-15.16.4.1 Developer Isolates Mod ECS

**As a** developer (P-15), **I want** mods to load into isolated ECS world partitions with
mod-spawned entities tagged with a `ModSource` component, **so that** mod content is traceable and
cannot access core game systems.

## US-15.16.4.2 Server Admin Enforces Sandbox Restrictions

**As a** server admin (P-22), **I want** mod logic graphs to execute in a sandboxed runtime with no
filesystem, network, or engine settings access, **so that** player-created content cannot compromise
system security.

## US-15.16.4.3 Modder Gets Budget Violation Feedback

**As a** modder (P-24), **I want** budget violations (memory, entity count) to trigger mod
deactivation with a player-facing notification, **so that** players understand why a mod was
disabled rather than experiencing a silent failure.

## US-15.16.4.4 Engine Tester Validates Load-Order Resolution

**As an** engine tester (P-27), **I want** to verify that two mods modifying the same asset trigger
a load-order resolution prompt, **so that** mod conflicts are detected and resolved rather than
causing undefined behavior.

## F-15.16.5 Mod Workshop Integration

## US-15.16.5.1 Modder Uploads to Steam Workshop

**As a** modder (P-24), **I want** to upload mods to Steam Workshop and a self-hosted mod repository
directly from the mod SDK, **so that** I can distribute my content without manual file hosting.

## US-15.16.5.2 Designer Browses Mod Browser

**As a** designer (P-5), **I want** a mod browser UI displaying available mods with thumbnails,
descriptions, ratings, download counts, and compatibility status, **so that** I can evaluate
community content.

## US-15.16.5.3 Server Admin Manages Self-Hosted Repository

**As a** server admin (P-22), **I want** a self-hosted mod repository served by the cloud
infrastructure with a REST API, **so that** non-Steam platforms have mod distribution.

## US-15.16.5.4 Engine Tester Validates Auto-Update

**As an** engine tester (P-27), **I want** to verify that mod updates are detected and applied on
game launch automatically, **so that** players always have the latest version of subscribed mods.

## F-15.16.6 Mod Moderation and Review

## US-15.16.6.1 Server Admin Reviews Submitted Mods

**As a** server admin (P-22), **I want** a moderation dashboard displaying submitted mods with
automated scan results (budget compliance, restricted node usage, content policy keywords, malware
scanning), **so that** I can review mods efficiently.

## US-15.16.6.2 DevOps Automates Moderation Scans

**As a** DevOps engineer (P-16), **I want** automated moderation scans to run on every mod
submission before it reaches the review queue, **so that** obvious violations are flagged
automatically.

## US-15.16.6.3 Creative Director Approves Verified Mods

**As a** creative director (P-2), **I want** approved mods flagged as verified in the mod browser,
**so that** players can distinguish curated content from unreviewed submissions.

## US-15.16.6.4 Engine Tester Validates Revocation

**As an** engine tester (P-27), **I want** to verify that revoking approval force-uninstalls mods
from all players and that moderation actions are logged for audit, **so that** policy violations can
be enforced post-publication.
