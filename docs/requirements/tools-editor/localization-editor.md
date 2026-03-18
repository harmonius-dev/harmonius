# R-15.13 -- Localization Editor Requirements

## String Management

| ID        | Derived From                                                    |
|-----------|-----------------------------------------------------------------|
| R-15.13.1 | [F-15.13.1](../../features/tools-editor/localization-editor.md) |
| R-15.13.2 | [F-15.13.2](../../features/tools-editor/localization-editor.md) |
| R-15.13.3 | [F-15.13.3](../../features/tools-editor/localization-editor.md) |

1. **R-15.13.1** — The editor **SHALL** provide a visual spreadsheet for all localizable strings
   with inline editing, filtering and sorting by key/locale/status, missing translation
   highlighting, plural forms and gendered variants as structured sub-rows, interpolation variable
   display, bulk CSV import/export, and translation memory suggestions.
   - **Rationale:** Centralized string management with structured sub-rows handles complex
     localization patterns without external tools.
   - **Verification:** Unit test: perform a CSV round-trip and verify all data is preserved.
2. **R-15.13.2** — The editor **SHALL** support runtime locale switching with automatic UI
   re-layout, and a validation pass detecting missing translations, text overflow, broken
   interpolation variables, incorrect plural forms, and RTL layout issues, with one-click navigation
   to offending widgets and a pseudo-localization mode.
   - **Rationale:** Preview and validation catch localization issues before shipping.
   - **Verification:** Unit test: enable pseudo-localization and verify all visible text is replaced
     with accented characters.
3. **R-15.13.3** — The editor **SHALL** integrate with external translation services (Crowdin,
   Lokalise, Phrase) for export, import, per-locale progress tracking, in-editor review with
   approved/needs-revision status, string locks on approved translations, and change detection for
   modified source strings.
   - **Rationale:** Professional translation workflows require service integration and quality
     control.
   - **Verification:** Unit test: lock an approved string and verify it cannot be modified.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/localization-editor.md](../../user-stories/tools-editor/localization-editor.md).
Requirements in this document are derived from those user stories.
