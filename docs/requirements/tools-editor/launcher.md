# R-15.15 -- Engine Launcher Requirements

## Installation and Updates

| ID        | Derived From                                         |
|-----------|------------------------------------------------------|
| R-15.15.1 | [F-15.15.1](../../features/tools-editor/launcher.md) |
| R-15.15.2 | [F-15.15.2](../../features/tools-editor/launcher.md) |

1. **R-15.15.1** — The launcher **SHALL** support installing multiple engine versions side by side
   with full isolation, rollback to previous versions, update notifications for stable and preview
   releases, per-version disk space display with one-click uninstall, and platform-native
   auto-update (Sparkle, WinSparkle, AppImage delta).
   - **Rationale:** Multiple projects may pin different engine versions; isolation prevents
     cross-version conflicts.
   - **Verification:** Integration test: install two versions side by side and verify they are fully
     isolated.
2. **R-15.15.2** — The launcher **SHALL** automatically upgrade projects when opened with a newer
   engine version, running incremental versioned migrations with pre-upgrade backup, deprecated API
   migration, and a migration report detailing all changes and warnings.
   - **Rationale:** Manual project upgrades are error-prone; incremental migration ensures no
     versions are skipped.
   - **Verification:** Integration test: upgrade from v1.2 to v1.5 and verify migrations 1.2-1.3,
     1.3-1.4, and 1.4-1.5 run in sequence.

## Project Management

| ID        | Derived From                                         |
|-----------|------------------------------------------------------|
| R-15.15.3 | [F-15.15.3](../../features/tools-editor/launcher.md) |
| R-15.15.4 | [F-15.15.4](../../features/tools-editor/launcher.md) |

1. **R-15.15.3** — The launcher **SHALL** display recent projects with thumbnails and metadata, and
   provide a creation wizard with genre template selection (RPG, FPS, RTS, 2D, VR, empty), target
   platform selection, and Git repository initialization.
   - **Rationale:** Quick project access and guided creation reduce onboarding friction.
   - **Verification:** Unit test: create a project from each genre template and verify it produces a
     valid project with expected pre-configured systems.
2. **R-15.15.4** — The launcher **SHALL** use a human-readable TOML .harmonius project file storing
   engine version pin and enabled modules, with double-click file association using platform-native
   mechanisms (Launch Services, registry, XDG MIME) that launches the pinned engine version.
   - **Rationale:** Human-readable project files are diffable in VCS; file association enables
     one-click project opening.
   - **Verification:** Unit test: double-click a .harmonius file and verify the pinned engine
     version launches.

## Account and Preferences

| ID        | Derived From                                         |
|-----------|------------------------------------------------------|
| R-15.15.5 | [F-15.15.5](../../features/tools-editor/launcher.md) |
| R-15.15.6 | [F-15.15.6](../../features/tools-editor/launcher.md) |

1. **R-15.15.5** — The launcher **SHALL** support global preferences (theme, keybindings) across
   projects, linked platform accounts (GitHub, GitLab, Bitbucket), multi-account support, preference
   sync across machines, and credentials stored in platform keychain.
   - **Rationale:** Consistent settings across projects and machines reduce friction when switching
     contexts.
   - **Verification:** Integration test: change a preference on one machine and verify it syncs to
     another.
2. **R-15.15.6** — The launcher **SHALL** provide guided setup for Git hosting provider connection,
   LFS storage configuration, real-time collaboration server setup, team member invitation with role
   assignment, shared build cache endpoint configuration, and connectivity validation before
   completing setup.
   - **Rationale:** Guided team setup reduces misconfiguration and accelerates onboarding.
   - **Verification:** Integration test: verify team configuration is written to .harmonius and
     inherited by new clones.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/launcher.md](../../user-stories/tools-editor/launcher.md). Requirements
in this document are derived from those user stories.
