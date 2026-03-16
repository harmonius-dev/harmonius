# R-15.13 -- Localization Editor Requirements

## String Management

### R-15.13.1 String Table Editor

The editor **SHALL** provide a visual spreadsheet for all localizable strings with inline editing,
filtering and sorting by key/locale/status, missing translation highlighting, plural forms and
gendered variants as structured sub-rows, interpolation variable display, bulk CSV import/export,
and translation memory suggestions.

- **Derived from:**
  [F-15.13.1](../../features/tools-editor/localization-editor.md)
- **Rationale:** Centralized string management with structured sub-rows handles complex localization
  patterns without external tools.
- **Verification:** Unit test: perform a CSV round-trip and verify all data is preserved.

### R-15.13.2 Localization Preview and Validation

The editor **SHALL** support runtime locale switching with automatic UI re-layout, and a validation
pass detecting missing translations, text overflow, broken interpolation variables, incorrect plural
forms, and RTL layout issues, with one-click navigation to offending widgets and a
pseudo-localization mode.

- **Derived from:**
  [F-15.13.2](../../features/tools-editor/localization-editor.md)
- **Rationale:** Preview and validation catch localization issues before shipping.
- **Verification:** Unit test: enable pseudo-localization and verify all visible text is replaced
  with accented characters.

### R-15.13.3 Translation Workflow Integration

The editor **SHALL** integrate with external translation services (Crowdin, Lokalise, Phrase) for
export, import, per-locale progress tracking, in-editor review with approved/needs-revision status,
string locks on approved translations, and change detection for modified source strings.

- **Derived from:**
  [F-15.13.3](../../features/tools-editor/localization-editor.md)
- **Rationale:** Professional translation workflows require service integration and quality control.
- **Verification:** Unit test: lock an approved string and verify it cannot be modified.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/localization-editor.md](../../user-stories/tools-editor/localization-editor.md).
Requirements in this document are derived from those
user stories.
