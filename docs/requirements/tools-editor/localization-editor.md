# R-15.13 Localization Editor

## R-15.13.1 String Table Editor
The engine **SHALL** provide a visual spreadsheet editor for managing all localizable strings,
displaying string keys, source text, and per-locale translations in a filterable and sortable table,
supporting inline editing, bulk CSV import/export, translation memory suggestions, context
annotations, plural/gendered/interpolation sub-rows, and integration with the asset database for
versioning and search.
- **Derived from:** [F-15.13.1](../../features/tools-editor/localization-editor.md)
- **Rationale:** A centralized string table editor with structured sub-rows and translation memory
  eliminates the error-prone workflow of editing raw localization files and reduces duplicate
  translation effort across the project.
- **Verification:** Create string entries with plural forms, gendered variants, and interpolation
  variables; add translations for multiple locales; filter and sort the table; bulk-export to CSV,
  modify externally, re-import, and confirm all translations are updated correctly; confirm missing
  translations are highlighted.

## R-15.13.2 Localization Preview and Validation
The engine **SHALL** support runtime locale switching in the editor with automatic UI re-layout, and
run a validation pass that detects missing translations, text overflow, broken interpolation
variables, incorrect plural forms, and RTL layout issues, reporting results with one-click
navigation to offending widgets or string entries, and provide a pseudo-localization mode that
replaces text with accented and padded characters.
- **Derived from:** [F-15.13.2](../../features/tools-editor/localization-editor.md)
- **Rationale:** Preview and validation catch localization defects before shipping, preventing
  truncated text, broken layouts, and missing strings that degrade the player experience in
  non-primary locales.
- **Verification:** Switch the editor locale and confirm all UI widgets re-layout with the selected
  locale's text; introduce a missing translation, a text overflow, a broken interpolation variable,
  an incorrect plural form, and an RTL issue, then run validation and confirm each is reported;
  enable pseudo-localization and confirm all visible text is replaced with accented/padded strings.

## R-15.13.3 Translation Workflow Integration
The engine **SHALL** support connecting to external translation management systems (Crowdin,
Lokalise, Phrase, or custom APIs) for exporting strings, importing completed translations, tracking
per-locale progress, and providing in-editor review with approved/needs-revision/rejected status,
string locks on approved translations, and change detection for re-translation flagging.
- **Derived from:** [F-15.13.3](../../features/tools-editor/localization-editor.md)
- **Rationale:** Integration with professional translation services streamlines the handoff between
  development and localization teams, reducing turnaround time and preventing accidental overwrites
  of approved translations.
- **Verification:** Connect to an external translation service; export strings and confirm they
  appear in the external system; import completed translations and confirm they appear in the string
  table; mark a string as approved and confirm it is locked against modification; modify a source
  string and confirm the change detection flags it for re-translation.
