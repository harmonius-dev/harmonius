# User Stories: Localization Editor

## F-15.13.1 String Table Editor

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.13.1.1 | designer (P-5)          |          |              |
| US-15.13.1.2 | artist (P-8)            |          |              |
| US-15.13.1.3 | developer (P-15)        |          |              |
| US-15.13.1.4 | creative director (P-2) |          |              |

1. **US-15.13.1.1** — a visual spreadsheet editor displaying string keys, source text, and
   translations per locale in a filterable, sortable table with inline editing
   - **Acceptance:** I can manage all localizable strings without editing raw files
2. **US-15.13.1.2** — missing translations highlighted in the string table with plural forms,
   gendered variants, and interpolation variables displayed in structured sub-rows
   - **Acceptance:** I can identify incomplete localization at a glance
3. **US-15.13.1.3** — bulk import/export in CSV format
   - **Acceptance:** I can exchange string data with external translation management tools and
     spreadsheet workflows
4. **US-15.13.1.4** — to attach screenshots and notes as context annotations for translators
   - **Acceptance:** translators understand where and how each string appears in-game

## F-15.13.2 Localization Preview and Validation

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.13.2.1 | designer (P-5)       |          |              |
| US-15.13.2.2 | developer (P-15)     |          |              |
| US-15.13.2.3 | tech artist (P-13)   |          |              |
| US-15.13.2.4 | engine tester (P-27) |          |              |

1. **US-15.13.2.1** — to switch the active language at runtime and see the UI re-layout with the
   selected locale's text
   - **Acceptance:** I can preview how the game looks in every supported language
2. **US-15.13.2.2** — a validation pass checking for missing translations, text overflow, broken
   interpolation variables, incorrect plural forms, and RTL layout issues
   - **Acceptance:** localization defects are caught before testing
3. **US-15.13.2.3** — pseudo-localization mode that replaces text with accented characters and
   padded strings
   - **Acceptance:** I can expose hardcoded text and layout assumptions without waiting for real
     translations
4. **US-15.13.2.4** — to verify that RTL locales render correctly with proper text direction and
   mirrored UI layout
   - **Acceptance:** Arabic and Hebrew players have a correct experience

## F-15.13.3 Translation Workflow Integration

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.13.3.1 | DevOps engineer (P-16)  |          |              |
| US-15.13.3.2 | designer (P-5)          |          |              |
| US-15.13.3.3 | creative director (P-2) |          |              |
| US-15.13.3.4 | engine tester (P-27)    |          |              |

1. **US-15.13.3.1** — the localization editor to connect to external translation management systems
   (Crowdin, Lokalise, Phrase)
   - **Acceptance:** I can integrate professional translation workflows without manual file exchange
2. **US-15.13.3.2** — to track translation progress per locale and per string set
   - **Acceptance:** I can identify which languages are falling behind and prioritize accordingly
3. **US-15.13.3.3** — translation review within the editor where reviewers mark strings as approved,
   needs-revision, or rejected with comments
   - **Acceptance:** I can maintain quality control over localized content
4. **US-15.13.3.4** — to verify that change detection correctly flags strings modified since the
   last export for re-translation
   - **Acceptance:** updated source strings are always sent for re-translation
