# R-15.15 -- Engine Launcher Requirements

## Installation and Updates

### R-15.15.1 Engine Version Management

The launcher **SHALL** support installing multiple engine
versions side by side with full isolation, rollback to
previous versions, update notifications for stable and
preview releases, per-version disk space display with
one-click uninstall, and platform-native auto-update
(Sparkle, WinSparkle, AppImage delta).

- **Derived from:**
  [F-15.15.1](../../features/tools-editor/launcher.md)
- **Rationale:** Multiple projects may pin different engine
  versions; isolation prevents cross-version conflicts.
- **Verification:** Integration test: install two versions
  side by side and verify they are fully isolated.

### R-15.15.2 Automatic Project Upgrades

The launcher **SHALL** automatically upgrade projects when
opened with a newer engine version, running incremental
versioned migrations with pre-upgrade backup, deprecated
API migration, and a migration report detailing all
changes and warnings.

- **Derived from:**
  [F-15.15.2](../../features/tools-editor/launcher.md)
- **Rationale:** Manual project upgrades are error-prone;
  incremental migration ensures no versions are skipped.
- **Verification:** Integration test: upgrade from v1.2 to
  v1.5 and verify migrations 1.2-1.3, 1.3-1.4, and
  1.4-1.5 run in sequence.

## Project Management

### R-15.15.3 Project Browser and Creation Wizard

The launcher **SHALL** display recent projects with
thumbnails and metadata, and provide a creation wizard
with genre template selection (RPG, FPS, RTS, 2D, VR,
empty), target platform selection, and Git repository
initialization.

- **Derived from:**
  [F-15.15.3](../../features/tools-editor/launcher.md)
- **Rationale:** Quick project access and guided creation
  reduce onboarding friction.
- **Verification:** Unit test: create a project from each
  genre template and verify it produces a valid project
  with expected pre-configured systems.

### R-15.15.4 Project File Format and Association

The launcher **SHALL** use a human-readable TOML .harmonius
project file storing engine version pin and enabled
modules, with double-click file association using
platform-native mechanisms (Launch Services, registry,
XDG MIME) that launches the pinned engine version.

- **Derived from:**
  [F-15.15.4](../../features/tools-editor/launcher.md)
- **Rationale:** Human-readable project files are diffable
  in VCS; file association enables one-click project
  opening.
- **Verification:** Unit test: double-click a .harmonius
  file and verify the pinned engine version launches.

## Account and Preferences

### R-15.15.5 Cross-Game Preferences and Account Management

The launcher **SHALL** support global preferences (theme,
keybindings) across projects, linked platform accounts
(GitHub, GitLab, Bitbucket), multi-account support,
preference sync across machines, and credentials stored
in platform keychain.

- **Derived from:**
  [F-15.15.5](../../features/tools-editor/launcher.md)
- **Rationale:** Consistent settings across projects and
  machines reduce friction when switching contexts.
- **Verification:** Integration test: change a preference
  on one machine and verify it syncs to another.

### R-15.15.6 Collaboration Setup Wizard

The launcher **SHALL** provide guided setup for Git hosting
provider connection, LFS storage configuration, real-time
collaboration server setup, team member invitation with
role assignment, shared build cache endpoint
configuration, and connectivity validation before
completing setup.

- **Derived from:**
  [F-15.15.6](../../features/tools-editor/launcher.md)
- **Rationale:** Guided team setup reduces misconfiguration
  and accelerates onboarding.
- **Verification:** Integration test: verify team
  configuration is written to .harmonius and inherited by
  new clones.

---

## User Stories

## US-15.15.1 Engine Version Management

### US-15.15.1.1
As a **developer (P-15)**, I want to install multiple engine versions side by side so that different
projects can use different engine versions.

### US-15.15.1.2
As a **developer (P-15)**, I want to rollback to a previous engine version so that I can revert if a
new version introduces regressions.

### US-15.15.1.3
As a **developer (P-15)**, I want update notifications for stable and preview releases so that I am
aware of new versions without checking manually.

### US-15.15.1.4
As a **developer (P-15)**, I want per-version disk space display with one-click uninstall so that I
can manage storage for installed engine versions.

### US-15.15.1.5
As a **DevOps (P-16)**, I want each installed version isolated so projects pin to a specific version
so that version upgrades are project-by-project decisions.

### US-15.15.1.6
As an **engine dev (P-26)**, I want platform-native auto-update (Sparkle, WinSparkle, AppImage
delta) so that launcher updates use the platform's standard mechanism.

### US-15.15.1.7
As an **engine tester (P-27)**, I want to verify two side-by-side versions are fully isolated so
that version isolation is regression-tested.

---

## US-15.15.2 Automatic Project Upgrades

### US-15.15.2.1
As a **developer (P-15)**, I want automatic project upgrade when opening with a newer engine version
so that migration happens without manual steps.

### US-15.15.2.2
As a **developer (P-15)**, I want a pre-upgrade backup created automatically so that I can restore
the project if migration fails.

### US-15.15.2.3
As a **developer (P-15)**, I want incremental migration across intermediate versions so that
skipping versions does not break the upgrade path.

### US-15.15.2.4
As a **developer (P-15)**, I want a migration report detailing all changes and warnings so that I
understand what changed and what needs manual review.

### US-15.15.2.5
As a **developer (P-15)**, I want deprecated logic graph APIs migrated to replacements so that my
graphs work on the new version without manual editing.

### US-15.15.2.6
As an **engine tester (P-27)**, I want to verify incremental migration from v1.2 to v1.5 runs
1.2-1.3, 1.3-1.4, and 1.4-1.5 in sequence so that incremental migration is regression-tested.

---

## US-15.15.3 Project Browser and Creation Wizard

### US-15.15.3.1
As a **designer (P-5)**, I want recent projects displayed with thumbnails and metadata so that I can
open projects quickly from the launcher.

### US-15.15.3.2
As a **designer (P-5)**, I want a creation wizard with genre template selection so that new projects
start with pre-configured gameplay systems.

### US-15.15.3.3
As a **designer (P-5)**, I want target platform selection during project creation so that the
project is configured for my deployment targets from the start.

### US-15.15.3.4
As a **developer (P-15)**, I want Git repository initialization during project creation so that
version control is set up from the beginning.

### US-15.15.3.5
As a **creative director (P-2)**, I want genre templates (RPG, FPS, RTS, 2D, VR, empty) so that
teams start with appropriate defaults for their game type.

### US-15.15.3.6
As an **engine tester (P-27)**, I want to verify each genre template creates a valid project with
expected pre-configured systems so that template correctness is regression-tested.

---

## US-15.15.4 Project File Format and Association

### US-15.15.4.1
As a **developer (P-15)**, I want a human-readable TOML .harmonius project file so that project
configuration is diffable and mergeable in VCS.

### US-15.15.4.2
As a **developer (P-15)**, I want the project file to store engine version pin and enabled modules
so that project configuration is explicit and reproducible.

### US-15.15.4.3
As a **developer (P-15)**, I want double-clicking .harmonius files to open the project in the
correct engine version so that file association works natively on all platforms.

### US-15.15.4.4
As an **engine dev (P-26)**, I want file association via Launch Services, registry, and XDG MIME so
that association uses platform-native mechanisms.

### US-15.15.4.5
As an **engine tester (P-27)**, I want to verify double-clicking a .harmonius file launches the
pinned engine version so that file association is regression-tested.

---

## US-15.15.5 Cross-Game Preferences and Account Management

### US-15.15.5.1
As a **developer (P-15)**, I want global preferences (theme, keybindings) across projects so that my
editor settings are consistent regardless of which project I open.

### US-15.15.5.2
As a **developer (P-15)**, I want to link GitHub, GitLab, and Bitbucket accounts so that version
control authentication is configured once.

### US-15.15.5.3
As a **developer (P-15)**, I want multi-account support for personal and organization accounts so
that I can switch contexts without re-authenticating.

### US-15.15.5.4
As a **developer (P-15)**, I want preference sync across machines via cloud service so that my
settings follow me when I switch workstations.

### US-15.15.5.5
As an **engine dev (P-26)**, I want credentials stored in platform keychain so that secrets are
secured natively per platform.

### US-15.15.5.6
As an **engine tester (P-27)**, I want to verify preferences sync between two machines so that cloud
sync is regression-tested.

---

## US-15.15.6 Collaboration Setup Wizard

### US-15.15.6.1
As a **developer (P-15)**, I want guided setup for Git hosting provider connection so that version
control is configured correctly without manual steps.

### US-15.15.6.2
As a **developer (P-15)**, I want LFS storage configuration in the wizard so that large file
handling is set up from the start.

### US-15.15.6.3
As a **developer (P-15)**, I want real-time collaboration server setup in the wizard so that team
collaboration is configured in a single flow.

### US-15.15.6.4
As a **creative director (P-2)**, I want to invite team members with role assignment so that
onboarding is a guided process with correct permissions.

### US-15.15.6.5
As a **DevOps (P-16)**, I want shared build cache endpoint configuration in the wizard so that the
cache is connected as part of team setup.

### US-15.15.6.6
As a **DevOps (P-16)**, I want connectivity and authentication validation before completing setup so
that misconfigurations are caught during the wizard.

### US-15.15.6.7
As an **engine tester (P-27)**, I want to verify team configuration is written to .harmonius and
inherited by new clones so that configuration inheritance is regression-tested.
