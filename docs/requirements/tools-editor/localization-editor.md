# R-15.13 -- Localization Editor Requirements

## String Management

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.13.1 | The editor **SHALL** provide a visual spreadsheet for all localizable strings with inline editing, filtering and sorting by key/locale/status, missing translation highlighting, plural forms and gendered variants as structured sub-rows, interpolation variable display, bulk CSV import/export, and translation memory suggestions. | [F-15.13.1](../../features/tools-editor/localization-editor.md) | Centralized string management with structured sub-rows handles complex localization patterns without external tools. | Unit test: perform a CSV round-trip and verify all data is preserved. |
| R-15.13.2 | The editor **SHALL** support runtime locale switching with automatic UI re-layout, and a validation pass detecting missing translations, text overflow, broken interpolation variables, incorrect plural forms, and RTL layout issues, with one-click navigation to offending widgets and a pseudo-localization mode. | [F-15.13.2](../../features/tools-editor/localization-editor.md) | Preview and validation catch localization issues before shipping. | Unit test: enable pseudo-localization and verify all visible text is replaced with accented characters. |
| R-15.13.3 | The editor **SHALL** integrate with external translation services (Crowdin, Lokalise, Phrase) for export, import, per-locale progress tracking, in-editor review with approved/needs-revision status, string locks on approved translations, and change detection for modified source strings. | [F-15.13.3](../../features/tools-editor/localization-editor.md) | Professional translation workflows require service integration and quality control. | Unit test: lock an approved string and verify it cannot be modified. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/localization-editor.md](../../user-stories/tools-editor/localization-editor.md).
Requirements in this document are derived from those user stories.
