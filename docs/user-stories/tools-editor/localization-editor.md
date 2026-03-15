# User Stories: Localization Editor

## F-15.13.1 String Table Editor

## US-15.13.1.1 Designer Manages Strings in Spreadsheet
**As a** designer (P-5), **I want** a visual spreadsheet editor displaying string keys, source text,
and translations per locale in a filterable, sortable table with inline editing, **so that** I can
manage all localizable strings without editing raw files.

## US-15.13.1.2 Artist Sees Missing Translations
**As an** artist (P-8), **I want** missing translations highlighted in the string table with plural
forms, gendered variants, and interpolation variables displayed in structured sub-rows, **so that**
I can identify incomplete localization at a glance.

## US-15.13.1.3 Developer Imports and Exports CSV
**As a** developer (P-15), **I want** bulk import/export in CSV format, **so that** I can exchange
string data with external translation management tools and spreadsheet workflows.

## US-15.13.1.4 Creative Director Adds Context Annotations
**As a** creative director (P-2), **I want** to attach screenshots and notes as context annotations
for translators, **so that** translators understand where and how each string appears in-game.

## F-15.13.2 Localization Preview and Validation

## US-15.13.2.1 Designer Previews Any Locale
**As a** designer (P-5), **I want** to switch the active language at runtime and see the UI
re-layout with the selected locale's text, **so that** I can preview how the game looks in every
supported language.

## US-15.13.2.2 Developer Runs Validation Pass
**As a** developer (P-15), **I want** a validation pass checking for missing translations, text
overflow, broken interpolation variables, incorrect plural forms, and RTL layout issues, **so that**
localization defects are caught before testing.

## US-15.13.2.3 Tech Artist Uses Pseudo-Localization
**As a** tech artist (P-13), **I want** pseudo-localization mode that replaces text with accented
characters and padded strings, **so that** I can expose hardcoded text and layout assumptions
without waiting for real translations.

## US-15.13.2.4 Engine Tester Validates RTL Layout
**As an** engine tester (P-27), **I want** to verify that RTL locales render correctly with proper
text direction and mirrored UI layout, **so that** Arabic and Hebrew players have a correct
experience.

## F-15.13.3 Translation Workflow Integration

## US-15.13.3.1 DevOps Connects to TMS
**As a** DevOps engineer (P-16), **I want** the localization editor to connect to external
translation management systems (Crowdin, Lokalise, Phrase), **so that** I can integrate professional
translation workflows without manual file exchange.

## US-15.13.3.2 Designer Tracks Translation Progress
**As a** designer (P-5), **I want** to track translation progress per locale and per string set,
**so that** I can identify which languages are falling behind and prioritize accordingly.

## US-15.13.3.3 Creative Director Reviews Translations
**As a** creative director (P-2), **I want** translation review within the editor where reviewers
mark strings as approved, needs-revision, or rejected with comments, **so that** I can maintain
quality control over localized content.

## US-15.13.3.4 Engine Tester Validates Change Detection
**As an** engine tester (P-27), **I want** to verify that change detection correctly flags strings
modified since the last export for re-translation, **so that** updated source strings are always
sent for re-translation.
