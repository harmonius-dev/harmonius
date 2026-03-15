# R-15.15 Engine Launcher

## R-15.15.1 Engine Version Management
The engine **SHALL** provide a standalone launcher application that installs, updates, and manages
multiple engine versions side-by-side with isolated installations, version-pinned projects, update
notifications for stable and preview releases, rollback to previous versions, and per-version disk
space management with one-click uninstall.
- **Derived from:** [F-15.15.1](../../features/tools-editor/launcher.md)
- **Rationale:** Side-by-side version management enables teams to maintain projects on different
  engine versions without conflicts and to evaluate new releases before committing to an upgrade.
- **Verification:** Install two engine versions side-by-side and confirm they are isolated; pin a
  project to a specific version and confirm the correct engine launches; roll back to a previous
  version and confirm the project opens correctly; uninstall a version and confirm disk space is
  reclaimed; confirm update notifications appear for new releases.

## R-15.15.2 Automatic Project Upgrades
The engine **SHALL** automatically upgrade projects opened with a newer engine version by running
versioned migration scripts that update the .harmonius project file format, re-cook assets, migrate
deprecated logic graph APIs, create a pre-upgrade backup, execute migrations incrementally across
intermediate versions, and produce a migration report detailing all changes, warnings, and
deprecations.
- **Derived from:** [F-15.15.2](../../features/tools-editor/launcher.md)
- **Rationale:** Automatic incremental migration with backup and reporting reduces the manual effort
  and risk of upgrading projects across multiple engine versions.
- **Verification:** Create a project on an older engine version; open it with a newer version and
  confirm the migration runs automatically; confirm a pre-upgrade backup exists; confirm the
  .harmonius project file is updated to the new format; confirm migrations run incrementally through
  each intermediate version; confirm the migration report lists all changes and warnings.

## R-15.15.3 Project Browser and Creation Wizard
The engine **SHALL** display recent projects on the launcher home screen with thumbnails,
last-modified dates, engine versions, and team members, and provide a creation wizard that guides
project setup through genre template selection (RPG, FPS, RTS, 2D platformer, VR, empty), target
platform selection, initial asset pack selection, and Git repository initialization.
- **Derived from:** [F-15.15.3](../../features/tools-editor/launcher.md)
- **Rationale:** A guided creation wizard with genre templates accelerates project bootstrapping by
  pre-configuring gameplay systems, input mappings, and starter content appropriate to the game
  type.
- **Verification:** Confirm recent projects appear on the home screen with correct metadata; create
  a new project using each genre template and confirm the project directory structure, .harmonius
  file, and pre-configured systems match the template; confirm Git repository is initialized when
  selected; confirm the editor opens after wizard completion.

## R-15.15.4 Project File Format and Association
The engine **SHALL** use a human-readable TOML-format .harmonius project file at the project root
storing engine version pin, enabled modules, target platforms, team configuration, Git remote URL,
and project metadata, and register as the default handler for .harmonius files on all platforms so
double-clicking opens the project in the correct engine version.
- **Derived from:** [F-15.15.4](../../features/tools-editor/launcher.md)
- **Rationale:** A human-readable, version-controlled project file ensures project configuration is
  transparent, diffable, and mergeable across team members using standard version control tools.
- **Verification:** Create a project and confirm a .harmonius file exists at the root in valid TOML
  format containing all specified fields; modify the file in a text editor, reopen the project, and
  confirm changes are respected; double-click the .harmonius file from the OS file manager and
  confirm the launcher opens the project with the pinned engine version.

## R-15.15.5 Cross-Game Preferences and Account Management
The engine **SHALL** maintain global preferences (editor theme, keybindings, viewport defaults,
recent projects, telemetry opt-in) and account management (GitHub/GitLab/Bitbucket linking, team
collaboration accounts, AI service credentials) with multi-account support and cloud-based
preference syncing across machines.
- **Derived from:** [F-15.15.5](../../features/tools-editor/launcher.md)
- **Rationale:** Global preferences and account management reduce repetitive per-project
  configuration and enable seamless transitions between workstations.
- **Verification:** Set global preferences (theme, keybindings) and confirm they apply across
  multiple projects; link a platform account and confirm authentication succeeds; switch between
  personal and organization accounts and confirm correct context; modify preferences on one machine
  and confirm they sync to another machine via the cloud service.

## R-15.15.6 Collaboration Setup Wizard
The engine **SHALL** provide a guided collaboration setup wizard that connects a Git hosting
provider, configures LFS storage, sets up the real-time collaboration server, invites team members
with role assignment (admin, developer, artist, designer), configures shared build cache endpoints,
validates connectivity and authentication, and stores team configuration in the .harmonius project
file for automatic inheritance by new team members.
- **Derived from:** [F-15.15.6](../../features/tools-editor/launcher.md)
- **Rationale:** A guided setup wizard reduces the complexity of configuring multi-service team
  collaboration, ensuring all team members share a consistent and validated configuration.
- **Verification:** Run the collaboration wizard; connect a Git provider and confirm authentication
  succeeds; configure LFS and confirm large file storage works; invite a team member and confirm
  they receive the invitation with the assigned role; confirm the team configuration is written to
  the .harmonius project file; clone the project as a new team member and confirm the collaboration
  configuration is inherited automatically.
