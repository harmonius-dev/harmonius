# User Stories: Mod Support

## US-15.16.1 Mod Authoring Toolkit

**As a** modder, **I want** a standalone mod SDK with the level editor, material editor,
and logic graph editor that loads the base game's assets as read-only references, **so
that** I can create new content without access to the full engine or source code.

## US-15.16.2 Mod Sandboxing and Safety

**As a** developer, **I want** mods to load into isolated ECS partitions with no filesystem
or network access and automatic deactivation on budget violation, **so that** player-
created content cannot compromise game stability or system security.
