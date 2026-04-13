# 15.15 — Engine Launcher

## Installation and Updates

| ID | Feature |
| ----------- | ---------------------------- |
| F-15.15.1 | Engine Version Management |
| F-15.15.2 | Automatic Project Upgrades |

1. **F-15.15.1** — A standalone launcher application for installing, updating, and managing multiple
   engine versions side-by-side. Displays available engine versions with release notes, downloads
   and installs selected versions, and allows rollback to previous versions. Each installed version
   is isolated — projects pin to a specific engine version and can be upgraded independently. The
   launcher checks for updates on startup and displays notifications for new stable and preview
   releases. Disk space management shows per-version storage usage with one-click uninstall. macOS,
   WinSparkle on Windows, AppImage delta updates on Linux).
   - **Deps:** None (standalone application)
   - **Platform:** Auto-update uses platform-native update mechanisms where available (Sparkle on
2. **F-15.15.2** — When a project is opened with a newer engine version than it was created with,
   the launcher orchestrates an automatic upgrade pipeline. The pipeline runs versioned migration
   scripts that update the project file format, re-cook assets for the new engine, migrate
   deprecated APIs in logic graphs to their replacements, and validate the result. A pre-upgrade
   backup is created automatically. Migration is incremental — a project on version 1.2 upgrading to
   1.5 runs the 1.2-to-1.3, 1.3-to-1.4, and 1.4-to-1.5 migrations in sequence. A migration report
   details all changes made, warnings about manual review items, and any features deprecated or
   removed.
   - **Deps:** F-15.15.1

## Project Management

| ID | Feature |
| ----------- | ------------------------------------- |
| F-15.15.3 | Project Browser and Creation Wizard |
| F-15.15.4 | Project File Format and Association |

1. **F-15.15.3** — The launcher's home screen displays recent projects with thumbnails,
   last-modified dates, engine version, and team members. A creation wizard guides new project
   setup: select genre template (RPG, FPS, RTS, 2D platformer, VR experience, empty), target
   platforms, initial asset packs, and Git repository initialization. Genre templates pre-configure
   enabled gameplay systems (F-13.1.8), default input mappings, and starter content. The wizard
   creates the project directory structure, initializes the project file, and opens the editor.
   - **Deps:** F-15.15.1
2. **F-15.15.4** — A `.harmonius` project file at the root of every project directory. The file
   stores: engine version pin, enabled modules, target platform list, team configuration, Git remote
   URL, and project metadata (name, description, icon). The launcher registers as the default
   handler for `.harmonius` files on all platforms so double-clicking opens the project in the
   correct engine version. The project file is human-readable (TOML format) and version-controlled
   alongside the project. on Linux.
   - **Deps:** F-15.15.1
   - **Platform:** File association uses Launch Services on macOS, registry on Windows, XDG MIME

## Account and Preferences

| ID | Feature |
| ----------- | ----------------------------------------------- |
| F-15.15.5 | Cross-Game Preferences and Account Management |
| F-15.15.6 | Collaboration Setup Wizard |

1. **F-15.15.5** — Global preferences that apply across all projects: editor theme, keybindings,
   viewport defaults, recent project list, and telemetry opt-in. Account management supports linking
   multiple platform accounts (GitHub, GitLab, Bitbucket) for version control, team collaboration
   accounts for the real-time collaboration service (F-15.12.7), and AI service credentials
   (F-15.9.1). Multi-account support allows switching between personal and organization accounts.
   Preferences sync across machines via the cloud collaboration service when logged in. on Windows,
   libsecret on Linux).
   - **Deps:** F-15.15.1, F-15.12.7 (Cloud Collaboration Service)
   - **Platform:** Credentials stored in platform keychain (Keychain on macOS, Credential Manager
2. **F-15.15.6** — Guided setup for team collaboration features: connect Git hosting provider
   (GitHub, GitLab, Bitbucket), configure LFS storage, set up the real-time collaboration server
   (F-15.12.7), invite team members by email or platform account, assign roles (admin, developer,
   artist, designer), and configure shared build cache endpoints (F-15.11.1). The wizard validates
   network connectivity, authentication tokens, and server compatibility before completing setup.
   Team configuration is stored in the project file and shared via version control so new team
   members inherit the setup automatically. (Shared Cache)
   - **Deps:** F-15.15.1, F-15.10.8 (Multi-Provider Git), F-15.12.7 (Cloud Service), F-15.11.1
