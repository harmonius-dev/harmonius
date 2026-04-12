# User Stories -- 15.13 Localization Editor

## Stories

| ID           | Persona                        |
|--------------|--------------------------------|
| US-15.13.1.1 | localization specialist (P-18) |
| US-15.13.1.2 | game designer (P-5)            |
| US-15.13.2.1 | localization specialist (P-18) |
| US-15.13.2.2 | game designer (P-5)            |
| US-15.13.3.1 | localization specialist (P-18) |
| US-15.13.3.2 | build engineer (P-16)          |

1. **US-15.13.1.1** — **As a** localization specialist (P-18), **I want** a spreadsheet editor for
   all localizable strings with inline editing, bulk CSV import/export, and translation memory,
   **so that** I manage translations efficiently.

2. **US-15.13.1.2** — **As a** game designer (P-5), **I want** missing translations highlighted and
   context annotations with screenshots, **so that** translators have full context.

3. **US-15.13.2.1** — **As a** localization specialist (P-18), **I want** to preview any locale by
   switching the active language at runtime, **so that** I see how translations look in context.

4. **US-15.13.2.2** — **As a** game designer (P-5), **I want** validation checking for text
   overflow, broken variables, and RTL layout issues, **so that** localization bugs are caught
   before shipping.

5. **US-15.13.3.1** — **As a** localization specialist (P-18), **I want** integration with external
   translation management systems (Crowdin, Lokalise, Phrase), **so that** professional workflows
   connect to the editor.

6. **US-15.13.3.2** — **As a** build engineer (P-16), **I want** change detection flagging strings
   modified since last export, **so that** only new strings are sent for re-translation.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.13.1 | localization specialist (P-18) |
| US-15.13.2 | localization specialist (P-18) |
| US-15.13.3 | localization specialist (P-18) |

1. **US-15.13.1** -- **As a** localization specialist (P-18), **I want** the capabilities defined in
   sub-stories US-15.13.1.1 through US-15.13.1.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-15.13.2** -- **As a** localization specialist (P-18), **I want** the capabilities defined in
   sub-stories US-15.13.2.1 through US-15.13.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.13.3** -- **As a** localization specialist (P-18), **I want** the capabilities defined in
   sub-stories US-15.13.3.1 through US-15.13.3.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.
