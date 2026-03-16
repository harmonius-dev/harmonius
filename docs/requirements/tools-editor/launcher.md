# R-15.15 -- Engine Launcher Requirements

## Installation and Updates

### R-15.15.1 Engine Version Management

The launcher **SHALL** support installing multiple engine versions side by side with full isolation,
rollback to previous versions, update notifications for stable and preview releases, per-version
disk space display with one-click uninstall, and platform-native auto-update (Sparkle, WinSparkle,
AppImage delta).

- **Derived from:**
  [F-15.15.1](../../features/tools-editor/launcher.md)
- **Rationale:** Multiple projects may pin different engine versions; isolation prevents
  cross-version conflicts.
- **Verification:** Integration test: install two versions side by side and verify they are fully
  isolated.

### R-15.15.2 Automatic Project Upgrades

The launcher **SHALL** automatically upgrade projects when opened with a newer engine version,
running incremental versioned migrations with pre-upgrade backup, deprecated API migration, and a
migration report detailing all changes and warnings.

- **Derived from:**
  [F-15.15.2](../../features/tools-editor/launcher.md)
- **Rationale:** Manual project upgrades are error-prone; incremental migration ensures no versions
  are skipped.
- **Verification:** Integration test: upgrade from v1.2 to v1.5 and verify migrations 1.2-1.3,
  1.3-1.4, and 1.4-1.5 run in sequence.

## Project Management

### R-15.15.3 Project Browser and Creation Wizard

The launcher **SHALL** display recent projects with thumbnails and metadata, and provide a creation
wizard with genre template selection (RPG, FPS, RTS, 2D, VR, empty), target platform selection, and
Git repository initialization.

- **Derived from:**
  [F-15.15.3](../../features/tools-editor/launcher.md)
- **Rationale:** Quick project access and guided creation reduce onboarding friction.
- **Verification:** Unit test: create a project from each genre template and verify it produces a
  valid project with expected pre-configured systems.

### R-15.15.4 Project File Format and Association

The launcher **SHALL** use a human-readable TOML .harmonius project file storing engine version pin
and enabled modules, with double-click file association using platform-native mechanisms (Launch
Services, registry, XDG MIME) that launches the pinned engine version.

- **Derived from:**
  [F-15.15.4](../../features/tools-editor/launcher.md)
- **Rationale:** Human-readable project files are diffable in VCS; file association enables
  one-click project opening.
- **Verification:** Unit test: double-click a .harmonius file and verify the pinned engine version
  launches.

## Account and Preferences

### R-15.15.5 Cross-Game Preferences and Account Management

The launcher **SHALL** support global preferences (theme, keybindings) across projects, linked
platform accounts (GitHub, GitLab, Bitbucket), multi-account support, preference sync across
machines, and credentials stored in platform keychain.

- **Derived from:**
  [F-15.15.5](../../features/tools-editor/launcher.md)
- **Rationale:** Consistent settings across projects and machines reduce friction when switching
  contexts.
- **Verification:** Integration test: change a preference on one machine and verify it syncs to
  another.

### R-15.15.6 Collaboration Setup Wizard

The launcher **SHALL** provide guided setup for Git hosting provider connection, LFS storage
configuration, real-time collaboration server setup, team member invitation with role assignment,
shared build cache endpoint configuration, and connectivity validation before completing setup.

- **Derived from:**
  [F-15.15.6](../../features/tools-editor/launcher.md)
- **Rationale:** Guided team setup reduces misconfiguration and accelerates onboarding.
- **Verification:** Integration test: verify team configuration is written to .harmonius and
  inherited by new clones.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/launcher.md](../../user-stories/tools-editor/launcher.md).
Requirements in this document are derived from those
user stories.
