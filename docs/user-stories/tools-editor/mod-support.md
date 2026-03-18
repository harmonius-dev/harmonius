# User Stories: Mod Support

## F-15.16.1 Mod SDK and Authoring Tools

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.16.1.1 | modder (P-24)        |          |              |
| US-15.16.1.2 | developer (P-15)     |          |              |
| US-15.16.1.3 | modder (P-24)        |          |              |
| US-15.16.1.4 | engine tester (P-27) |          |              |

1. **US-15.16.1.1** — a standalone mod authoring toolkit with the level editor, material editor,
   logic graph editor, VFX editor, and asset import pipeline
   - **Acceptance:** I can create new content without access to the full engine or source code
2. **US-15.16.1.2** — to define which editor features are available to modders through a mod
   configuration asset
   - **Acceptance:** I can restrict access to engine internals and sensitive game systems
3. **US-15.16.1.3** — the mod SDK to load the base game's assets as read-only references
   - **Acceptance:** I can create content that references but cannot modify original game assets
4. **US-15.16.1.4** — to verify that the mod SDK runs as a standalone application with a mod-mode
   flag restricting features correctly
   - **Acceptance:** modders cannot access restricted editor capabilities

## F-15.16.2 Developer-Defined Mod Constraints

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.16.2.1 | developer (P-15)     |          |              |
| US-15.16.2.2 | designer (P-5)       |          |              |
| US-15.16.2.3 | server admin (P-22)  |          |              |
| US-15.16.2.4 | engine tester (P-27) |          |              |

1. **US-15.16.2.1** — to configure per-project constraints limiting which asset types mods can
   create, which logic graph nodes they can use, and which ECS components they can read/write
   - **Acceptance:** mods stay within safe boundaries
2. **US-15.16.2.2** — to set memory budget and entity budget limits per mod
   - **Acceptance:** no single mod can consume resources that degrade game performance
3. **US-15.16.2.3** — world region restrictions so mods can only modify designated moddable zones
   - **Acceptance:** core game areas remain unmodified by player content
4. **US-15.16.2.4** — to verify that constraints are enforced at authoring time (greyed-out
   restricted features) and at load time (rejecting over-budget mods)
   - **Acceptance:** constraint violations are caught at both stages

## F-15.16.3 Mod Packaging and Distribution Format

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.16.3.1 | modder (P-24)          |          |              |
| US-15.16.3.2 | developer (P-15)       |          |              |
| US-15.16.3.3 | DevOps engineer (P-16) |          |              |
| US-15.16.3.4 | engine tester (P-27)   |          |              |

1. **US-15.16.3.1** — to package my mod as a signed, versioned bundle with metadata (name, author,
   version, compatibility range, dependencies, preview images, change log)
   - **Acceptance:** my mod is distributable and verifiable
2. **US-15.16.3.2** — mod packages integrity-verified via content hashes on load
   - **Acceptance:** corrupted or tampered mods are detected before they execute
3. **US-15.16.3.3** — mod packages to use the engine's binary asset format with a mod manifest file
   - **Acceptance:** the existing asset pipeline handles mod content without special-case code
4. **US-15.16.3.4** — unsigned mods to display a security warning before installation
   - **Acceptance:** players are informed when installing unverified content

## F-15.16.4 Mod Loading and Sandboxing

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.16.4.1 | developer (P-15)     |          |              |
| US-15.16.4.2 | server admin (P-22)  |          |              |
| US-15.16.4.3 | modder (P-24)        |          |              |
| US-15.16.4.4 | engine tester (P-27) |          |              |

1. **US-15.16.4.1** — mods to load into isolated ECS world partitions with mod-spawned entities
   tagged with a `ModSource` component
   - **Acceptance:** mod content is traceable and cannot access core game systems
2. **US-15.16.4.2** — mod logic graphs to execute in a sandboxed runtime with no filesystem,
   network, or engine settings access
   - **Acceptance:** player-created content cannot compromise system security
3. **US-15.16.4.3** — budget violations (memory, entity count) to trigger mod deactivation with a
   player-facing notification
   - **Acceptance:** players understand why a mod was disabled rather than experiencing a silent
     failure
4. **US-15.16.4.4** — to verify that two mods modifying the same asset trigger a load-order
   resolution prompt
   - **Acceptance:** mod conflicts are detected and resolved rather than causing undefined behavior

## F-15.16.5 Mod Workshop Integration

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.16.5.1 | modder (P-24)        |          |              |
| US-15.16.5.2 | designer (P-5)       |          |              |
| US-15.16.5.3 | server admin (P-22)  |          |              |
| US-15.16.5.4 | engine tester (P-27) |          |              |

1. **US-15.16.5.1** — to upload mods to Steam Workshop and a self-hosted mod repository directly
   from the mod SDK
   - **Acceptance:** I can distribute my content without manual file hosting
2. **US-15.16.5.2** — a mod browser UI displaying available mods with thumbnails, descriptions,
   ratings, download counts, and compatibility status
   - **Acceptance:** I can evaluate community content
3. **US-15.16.5.3** — a self-hosted mod repository served by the cloud infrastructure with a REST
   API
   - **Acceptance:** non-Steam platforms have mod distribution
4. **US-15.16.5.4** — to verify that mod updates are detected and applied on game launch
   automatically
   - **Acceptance:** players always have the latest version of subscribed mods

## F-15.16.6 Mod Moderation and Review

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.16.6.1 | server admin (P-22)     |          |              |
| US-15.16.6.2 | DevOps engineer (P-16)  |          |              |
| US-15.16.6.3 | creative director (P-2) |          |              |
| US-15.16.6.4 | engine tester (P-27)    |          |              |

1. **US-15.16.6.1** — a moderation dashboard displaying submitted mods with automated scan results
   (budget compliance, restricted node usage, content policy keywords, malware scanning)
   - **Acceptance:** I can review mods efficiently
2. **US-15.16.6.2** — automated moderation scans to run on every mod submission before it reaches
   the review queue
   - **Acceptance:** obvious violations are flagged automatically
3. **US-15.16.6.3** — approved mods flagged as verified in the mod browser
   - **Acceptance:** players can distinguish curated content from unreviewed submissions
4. **US-15.16.6.4** — to verify that revoking approval force-uninstalls mods from all players and
   that moderation actions are logged for audit
   - **Acceptance:** policy violations can be enforced post-publication
