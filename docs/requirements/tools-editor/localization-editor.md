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

## User Stories

## US-15.13.1 String Table Editor

### US-15.13.1.1

As a **designer (P-5)**, I want a visual spreadsheet for all localizable strings so that I can
manage string keys and translations in one place.

### US-15.13.1.2

As a **designer (P-5)**, I want inline editing of source text and translations so that I can update
strings without external tools.

### US-15.13.1.3

As a **designer (P-5)**, I want filtering and sorting by key, locale, or status so that I can find
specific strings quickly.

### US-15.13.1.4

As a **designer (P-5)**, I want missing translations highlighted visually so that I can identify
untranslated strings at a glance.

### US-15.13.1.5

As a **designer (P-5)**, I want plural forms and gendered variants as structured sub-rows so that
complex localization patterns are represented clearly.

### US-15.13.1.6

As a **designer (P-5)**, I want interpolation variables displayed in context so that I can verify
variable names match the source format.

### US-15.13.1.7

As a **developer (P-15)**, I want bulk CSV import/export so that I can exchange data with external
translation services.

### US-15.13.1.8

As a **developer (P-15)**, I want translation memory suggestions so that previously translated
phrases are reused automatically.

### US-15.13.1.9

As a **artist (P-8)**, I want context annotations (screenshots, notes) so that translators
understand the visual context of each string.

### US-15.13.1.10

As an **engine tester (P-27)**, I want to verify CSV round-trip preserves all data so that
import/export fidelity is regression-tested.

---

## US-15.13.2 Localization Preview and Validation

### US-15.13.2.1

As a **designer (P-5)**, I want to switch the active locale at runtime in the editor so that I can
preview any language without rebuilding.

### US-15.13.2.2

As a **designer (P-5)**, I want automatic UI re-layout when switching locales so that widgets adjust
to different text lengths.

### US-15.13.2.3

As a **designer (P-5)**, I want a validation pass detecting missing translations so that incomplete
locales are flagged before shipping.

### US-15.13.2.4

As a **designer (P-5)**, I want validation detecting text overflow so that strings exceeding widget
bounds are caught.

### US-15.13.2.5

As a **designer (P-5)**, I want validation detecting broken interpolation variables so that format
string mismatches are caught.

### US-15.13.2.6

As a **designer (P-5)**, I want validation detecting RTL layout issues so that right-to-left
languages render correctly.

### US-15.13.2.7

As a **designer (P-5)**, I want pseudo-localization mode so that I can expose hardcoded text and
layout assumptions.

### US-15.13.2.8

As a **developer (P-15)**, I want one-click navigation from validation results to offending widgets
so that I can fix issues without searching manually.

### US-15.13.2.9

As an **engine tester (P-27)**, I want to verify pseudo-localization replaces all visible text with
accented characters so that hardcoded text detection is regression-tested.

---

## US-15.13.3 Translation Workflow Integration

### US-15.13.3.1

As a **developer (P-15)**, I want integration with Crowdin, Lokalise, and Phrase so that strings
flow to professional translation services.

### US-15.13.3.2

As a **developer (P-15)**, I want to export strings for translation and import results so that the
translation handoff is automated.

### US-15.13.3.3

As a **developer (P-15)**, I want per-locale translation progress tracking so that I can monitor
completion rates across languages.

### US-15.13.3.4

As a **developer (P-15)**, I want in-editor review with approved/needs-revision status so that
translation quality is managed without external tools.

### US-15.13.3.5

As a **developer (P-15)**, I want string locks on approved translations so that approved strings are
protected from accidental modification.

### US-15.13.3.6

As a **developer (P-15)**, I want change detection for modified source strings so that updated
strings are flagged for re-translation.

### US-15.13.3.7

As an **engine tester (P-27)**, I want to verify a locked approved string cannot be modified so that
string lock enforcement is regression-tested.
