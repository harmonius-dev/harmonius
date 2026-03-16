# User Stories: Engine Launcher

## F-15.15.1 Engine Version Management

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.15.1.1 | developer (P-15) | a standalone launcher for installing, updating, and managing multiple engine versions side-by-side with isolated installations | I can maintain legacy projects on older versions while evaluating new releases |  |  |
| US-15.15.1.2 | DevOps engineer (P-16) | the launcher to check for updates on startup and display notifications for new stable and preview releases | the team is always aware of available engine updates |  |  |
| US-15.15.1.3 | server admin (P-22) | per-version storage usage displayed with one-click uninstall and rollback support | I can manage disk space and revert to previous versions if needed |  |  |
| US-15.15.1.4 | engine tester (P-27) | to verify that auto-update uses platform-native mechanisms (Sparkle on macOS, WinSparkle on Windows, AppImage delta on Linux) | updates are reliable on each platform |  |  |

## F-15.15.2 Automatic Project Upgrades

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.15.2.1 | developer (P-15) | the launcher to run versioned migration scripts that update the project file format, re-cook assets, and migrate deprecated APIs in logic graphs when opening a project with a newer engine version | upgrades are automated |  |  |
| US-15.15.2.2 | designer (P-5) | a migration report detailing all changes, warnings about manual review items, and deprecated features | I understand what changed and what needs attention after an upgrade |  |  |
| US-15.15.2.3 | DevOps engineer (P-16) | an automatic pre-upgrade backup created before any migration runs | I can restore the project if the upgrade introduces issues |  |  |
| US-15.15.2.4 | engine tester (P-27) | to verify that incremental migration (e.g., 1.2 to 1.3 to 1.4 to 1.5 in sequence) produces the same result as a fresh project on version 1.5 | upgrade paths are reliable regardless of starting version |  |  |

## F-15.15.3 Project Browser and Creation Wizard

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.15.3.1 | designer (P-5) | a creation wizard that guides new project setup with genre templates (RPG, FPS, RTS, 2D platformer, VR, empty), target platforms, and starter content | projects start with pre-configured gameplay systems and input mappings |  |  |
| US-15.15.3.2 | artist (P-8) | the launcher home screen to display recent projects with thumbnails, last-modified dates, and engine version | I can quickly open the project I am working on |  |  |
| US-15.15.3.3 | creative director (P-2) | team member names displayed alongside each project | I can see who is working on which project from the launcher |  |  |
| US-15.15.3.4 | engine tester (P-27) | to verify that the creation wizard correctly initializes the project directory structure, project file, and Git repository | new projects are immediately usable |  |  |

## F-15.15.4 Project File Format and Association

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.15.4.1 | developer (P-15) | `.harmonius` project files registered as the default file type so double-clicking opens the project in the correct engine version | I can launch projects from the file manager |  |  |
| US-15.15.4.2 | DevOps engineer (P-16) | the project file in human-readable TOML format storing engine version, modules, platforms, and Git remote URL | project configuration is version-controlled and reviewable |  |  |
| US-15.15.4.3 | designer (P-5) | the project file to store name, description, and icon | project metadata is displayed correctly in the launcher and file manager |  |  |
| US-15.15.4.4 | engine tester (P-27) | to verify that file association works on all platforms (Launch Services on macOS, registry on Windows, XDG MIME on Linux) | double-click opening is reliable |  |  |

## F-15.15.5 Cross-Game Preferences and Account Management

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.15.5.1 | developer (P-15) | to link GitHub, GitLab, and Bitbucket accounts for version control and AI service credentials | all engine services authenticate through a single account |  |  |
| US-15.15.5.2 | designer (P-5) | global preferences (theme, keybindings, viewport defaults) synced across machines via the cloud collaboration service | my workspace feels the same regardless of which computer I use |  |  |
| US-15.15.5.3 | server admin (P-22) | multi-account support for switching between personal and organization accounts | I can manage infrastructure for multiple studios |  |  |
| US-15.15.5.4 | engine tester (P-27) | to verify that credentials are stored in platform keychains (Keychain on macOS, Credential Manager on Windows, libsecret on Linux) | sensitive tokens are never stored in plaintext |  |  |

## F-15.15.6 Collaboration Setup Wizard

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.15.6.1 | DevOps engineer (P-16) | a guided setup wizard that connects Git hosting, configures LFS storage, sets up the collaboration server, and invites team members with role assignments | team infrastructure is ready with minimal manual configuration |  |  |
| US-15.15.6.2 | designer (P-5) | team configuration stored in the project file and shared via version control | I inherit the team setup automatically on first clone |  |  |
| US-15.15.6.3 | server admin (P-22) | the wizard to validate network connectivity, authentication tokens, and server compatibility before completing setup | I know the infrastructure works before the team starts using it |  |  |
| US-15.15.6.4 | engine tester (P-27) | to verify that the wizard correctly configures all services (Git, LFS, collaboration server, shared build cache) | no service is left misconfigured after setup |  |  |
