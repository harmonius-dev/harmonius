# User Stories: Engine Launcher

## F-15.15.1 Engine Version Management

## US-15.15.1.1 Developer Installs Multiple Versions

**As a** developer (P-15), **I want** a standalone launcher for installing, updating, and managing
multiple engine versions side-by-side with isolated installations, **so that** I can maintain legacy
projects on older versions while evaluating new releases.

## US-15.15.1.2 DevOps Checks for Updates

**As a** DevOps engineer (P-16), **I want** the launcher to check for updates on startup and display
notifications for new stable and preview releases, **so that** the team is always aware of available
engine updates.

## US-15.15.1.3 Server Admin Manages Disk Space

**As a** server admin (P-22), **I want** per-version storage usage displayed with one-click
uninstall and rollback support, **so that** I can manage disk space and revert to previous versions
if needed.

## US-15.15.1.4 Engine Tester Validates Auto-Update

**As an** engine tester (P-27), **I want** to verify that auto-update uses platform-native
mechanisms (Sparkle on macOS, WinSparkle on Windows, AppImage delta on Linux), **so that** updates
are reliable on each platform.

## F-15.15.2 Automatic Project Upgrades

## US-15.15.2.1 Developer Upgrades Project Incrementally

**As a** developer (P-15), **I want** the launcher to run versioned migration scripts that update
the project file format, re-cook assets, and migrate deprecated APIs in logic graphs when opening a
project with a newer engine version, **so that** upgrades are automated.

## US-15.15.2.2 Designer Reviews Migration Report

**As a** designer (P-5), **I want** a migration report detailing all changes, warnings about manual
review items, and deprecated features, **so that** I understand what changed and what needs
attention after an upgrade.

## US-15.15.2.3 DevOps Creates Pre-Upgrade Backup

**As a** DevOps engineer (P-16), **I want** an automatic pre-upgrade backup created before any
migration runs, **so that** I can restore the project if the upgrade introduces issues.

## US-15.15.2.4 Engine Tester Validates Incremental Migration

**As an** engine tester (P-27), **I want** to verify that incremental migration (e.g., 1.2 to 1.3 to
1.4 to 1.5 in sequence) produces the same result as a fresh project on version 1.5, **so that**
upgrade paths are reliable regardless of starting version.

## F-15.15.3 Project Browser and Creation Wizard

## US-15.15.3.1 Designer Creates Project from Template

**As a** designer (P-5), **I want** a creation wizard that guides new project setup with genre
templates (RPG, FPS, RTS, 2D platformer, VR, empty), target platforms, and starter content,
**so that** projects start with pre-configured gameplay systems and input mappings.

## US-15.15.3.2 Artist Browses Recent Projects

**As an** artist (P-8), **I want** the launcher home screen to display recent projects with
thumbnails, last-modified dates, and engine version, **so that** I can quickly open the project I am
working on.

## US-15.15.3.3 Creative Director Reviews Team Projects

**As a** creative director (P-2), **I want** team member names displayed alongside each project,
**so that** I can see who is working on which project from the launcher.

## US-15.15.3.4 Engine Tester Validates Project Initialization

**As an** engine tester (P-27), **I want** to verify that the creation wizard correctly initializes
the project directory structure, project file, and Git repository, **so that** new projects are
immediately usable.

## F-15.15.4 Project File Format and Association

## US-15.15.4.1 Developer Double-Clicks to Open

**As a** developer (P-15), **I want** `.harmonius` project files registered as the default file type
so double-clicking opens the project in the correct engine version, **so that** I can launch
projects from the file manager.

## US-15.15.4.2 DevOps Version-Controls Project File

**As a** DevOps engineer (P-16), **I want** the project file in human-readable TOML format storing
engine version, modules, platforms, and Git remote URL, **so that** project configuration is
version-controlled and reviewable.

## US-15.15.4.3 Designer Sees Project Metadata

**As a** designer (P-5), **I want** the project file to store name, description, and icon,
**so that** project metadata is displayed correctly in the launcher and file manager.

## US-15.15.4.4 Engine Tester Validates File Association

**As an** engine tester (P-27), **I want** to verify that file association works on all platforms
(Launch Services on macOS, registry on Windows, XDG MIME on Linux), **so that** double-click opening
is reliable.

## F-15.15.5 Cross-Game Preferences and Account Management

## US-15.15.5.1 Developer Links Platform Accounts

**As a** developer (P-15), **I want** to link GitHub, GitLab, and Bitbucket accounts for version
control and AI service credentials, **so that** all engine services authenticate through a single
account.

## US-15.15.5.2 Designer Syncs Preferences Across Machines

**As a** designer (P-5), **I want** global preferences (theme, keybindings, viewport defaults)
synced across machines via the cloud collaboration service, **so that** my workspace feels the same
regardless of which computer I use.

## US-15.15.5.3 Server Admin Switches Between Accounts

**As a** server admin (P-22), **I want** multi-account support for switching between personal and
organization accounts, **so that** I can manage infrastructure for multiple studios.

## US-15.15.5.4 Engine Tester Validates Credential Storage

**As an** engine tester (P-27), **I want** to verify that credentials are stored in platform
keychains (Keychain on macOS, Credential Manager on Windows, libsecret on Linux), **so that**
sensitive tokens are never stored in plaintext.

## F-15.15.6 Collaboration Setup Wizard

## US-15.15.6.1 DevOps Configures Team Collaboration

**As a** DevOps engineer (P-16), **I want** a guided setup wizard that connects Git hosting,
configures LFS storage, sets up the collaboration server, and invites team members with role
assignments, **so that** team infrastructure is ready with minimal manual configuration.

## US-15.15.6.2 Designer Joins Team Automatically

**As a** designer (P-5), **I want** team configuration stored in the project file and shared via
version control, **so that** I inherit the team setup automatically on first clone.

## US-15.15.6.3 Server Admin Validates Connectivity

**As a** server admin (P-22), **I want** the wizard to validate network connectivity, authentication
tokens, and server compatibility before completing setup, **so that** I know the infrastructure
works before the team starts using it.

## US-15.15.6.4 Engine Tester Validates Setup Completeness

**As an** engine tester (P-27), **I want** to verify that the wizard correctly configures all
services (Git, LFS, collaboration server, shared build cache), **so that** no service is left
misconfigured after setup.
