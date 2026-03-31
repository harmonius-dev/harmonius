# R-15.15 -- Engine Launcher Requirements

## Requirements

1. **R-15.15.1** — The engine **SHALL** provide a standalone launcher for installing, updating, and
   managing multiple engine versions with per-version disk space display, rollback, and update
   notifications.
   - **Rationale:** Side-by-side version management enables project-pinned engine versions.
   - **Verification:** Install two versions, switch a project between them, and verify correct
     behavior.

2. **R-15.15.2** — The engine **SHALL** orchestrate automatic project upgrades with versioned
   migration scripts, pre-upgrade backup, and a migration report.
   - **Rationale:** Safe migration prevents data loss when upgrading engine versions.
   - **Verification:** Upgrade a project from version N to N+2 and verify migrations run
     sequentially.

3. **R-15.15.3** — The engine **SHALL** provide a project browser with thumbnails and a creation
   wizard with genre templates pre-configuring gameplay systems and starter content.
   - **Rationale:** Templates accelerate project setup for common game genres.
   - **Verification:** Create a project from the RPG template and verify the expected systems are
     enabled.

4. **R-15.15.4** — The engine **SHALL** use a human-readable TOML project file as the default file
   handler for the .harmonius extension on all platforms.
   - **Rationale:** File association enables double-click project opening in the correct engine
     version.
   - **Verification:** Double-click a .harmonius file and verify the correct engine version
     launches.

5. **R-15.15.5** — The engine **SHALL** provide global preferences synced across machines,
   multi-account support, and credential storage in the platform keychain.
   - **Rationale:** Synced preferences and multi-account support enable seamless multi-device
     workflows.
   - **Verification:** Change a preference on one machine, sync, and verify it appears on a second
     machine.

6. **R-15.15.6** — The engine **SHALL** provide a collaboration setup wizard connecting Git hosting,
   LFS storage, CRDT server, and shared cache, with team setup stored in the project file.
   - **Rationale:** Guided setup reduces collaboration infrastructure complexity.
   - **Verification:** Run the wizard, verify all services connect, and confirm a new team member
     inherits the setup.
