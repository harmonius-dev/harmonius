# 15.13 — Localization Editor

## String Management

| ID        | Feature                             | Requirements |
|-----------|-------------------------------------|--------------|
| F-15.13.1 | String Table Editor                 | R-15.13.1    |
| F-15.13.2 | Localization Preview and Validation | R-15.13.2    |
| F-15.13.3 | Translation Workflow Integration    | R-15.13.3    |

1. **F-15.13.1** — A visual spreadsheet editor for managing all localizable strings in the project.
   Displays string keys, source language text, and translations for each target locale in a
   filterable, sortable table. Supports inline editing, bulk import/export (CSV), translation memory
   (suggest translations from previously translated strings), and context annotations (screenshots,
   notes for translators). Missing translations are highlighted. Plural forms, gendered variants,
   and interpolation variables are displayed in structured sub-rows. Integrates with the asset
   database (F-12.3.1) for versioning and search.
   - **Deps:** F-15.1.1 (Editor Framework), F-10.1.9 (Localization Hooks)
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.13.2** — Preview any locale in the editor by switching the active language at runtime. The
   UI system (F-10.1.9) re-layouts all widgets with the selected locale's text. A validation pass
   checks for: missing translations, text overflow (strings that exceed widget bounds), broken
   interpolation variables, incorrect plural forms, and right-to-left (RTL) layout issues.
   Validation results are displayed in a report panel with one-click navigation to the offending
   widget or string entry. Pseudo-localization mode replaces all text with accented characters and
   padded strings to expose hardcoded text and layout assumptions.
   - **Deps:** F-15.13.1, F-10.1.9 (Localization Hooks), F-10.1.4 (Layout)
   - **Platform:** Desktop only. Not available on mobile or console runtime.
3. **F-15.13.3** — Connect the localization editor to external translation management systems
   (Crowdin, Lokalise, Phrase, or custom APIs) for professional translation workflows. Export
   strings for translation, import completed translations, and track translation progress per
   locale. Supports translation review within the editor — reviewers mark strings as approved,
   needs-revision, or rejected with comments. String locks prevent accidental modification of
   approved translations. Change detection flags strings modified since last export for
   re-translation.
   - **Deps:** F-15.13.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.
