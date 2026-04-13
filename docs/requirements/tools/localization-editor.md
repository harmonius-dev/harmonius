# R-15.13 -- Localization Editor Requirements

## Requirements

1. **R-15.13.1** — The engine **SHALL** provide a string table editor with filterable columns,
   inline editing, CSV import/export, translation memory, and context annotations.
   - **Rationale:** Centralized string management is essential for multi-locale projects.
   - **Verification:** Import a CSV with 1000 strings, filter by missing translations, and verify
     the correct subset is displayed.

2. **R-15.13.2** — The engine **SHALL** support runtime locale switching with validation for text
   overflow, broken interpolation variables, incorrect plural forms, and RTL layout issues, with
   pseudo-localization mode.
   - **Rationale:** In-editor preview catches localization bugs early in development.
   - **Verification:** Switch to a locale with longer strings and verify overflow warnings appear
     for affected widgets.

3. **R-15.13.3** — The engine **SHALL** integrate with external translation management systems via
   configurable API connectors, with export, import, review, and change detection.
   - **Rationale:** Professional studios use external translation services requiring system
     integration.
   - **Verification:** Export modified strings to Crowdin, import translations, and verify they
     appear in the string table.
