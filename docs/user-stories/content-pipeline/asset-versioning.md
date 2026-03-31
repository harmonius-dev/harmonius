# User Stories -- 12.7 Asset Versioning & Collaboration

## Binary Asset Format

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.7.1 | engine developer (P-26)    |
| US-12.7.2 | build engineer (P-16)      |
| US-12.7.3 | engine developer (P-26)    |

1. **US-12.7.1** — **As an** engine developer (P-26), **I want** all asset types stored in a single
   binary format with a common header and O(1) section access via a table-of-contents, **so that**
   loading is mmap-based with no parsing overhead.
2. **US-12.7.2** — **As a** build engineer (P-16), **I want** related assets grouped into compressed
   bundles with LZ4 for runtime and Zstd for distribution, **so that** storage and bandwidth are
   minimized without sacrificing load speed.
3. **US-12.7.3** — **As an** engine developer (P-26), **I want** individual assets extractable from
   a bundle without decompressing the entire archive, **so that** partial streaming and patching are
   efficient.

## Diffing and Merging

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.7.4 | technical artist (P-13)    |
| US-12.7.5 | environment artist (P-8)   |
| US-12.7.6 | technical artist (P-13)    |
| US-12.7.7 | build engineer (P-16)      |

1. **US-12.7.4** — **As a** technical artist (P-13), **I want** to diff two asset versions at the
   structural level showing added, removed, and modified elements, **so that** I understand what
   changed without inspecting raw bytes.
2. **US-12.7.5** — **As an** environment artist (P-8), **I want** two divergent asset versions
   merged automatically when changes are to non-overlapping regions, **so that** my team can work on
   the same asset concurrently.
3. **US-12.7.6** — **As a** technical artist (P-13), **I want** conflicting merge changes presented
   in the visual diff tool with per-region accept/reject options, **so that** I can resolve
   conflicts without leaving the editor.
4. **US-12.7.7** — **As a** build engineer (P-16), **I want** automatic merge conflict resolution
   using last-writer-wins and union strategies for non-ambiguous cases, **so that** CI merges
   succeed without manual intervention when possible.

## Visual Editors

| ID        | Persona                    |
|-----------|----------------------------|
| US-12.7.8 | game designer (P-5)        |
| US-12.7.9 | technical artist (P-13)    |

1. **US-12.7.8** — **As a** game designer (P-5), **I want** a spreadsheet-style editor for gameplay
   data tables with sorting, filtering, search, and bulk operations, **so that** I can edit large
   data sets efficiently.
2. **US-12.7.9** — **As a** technical artist (P-13), **I want** every asset type to have a dedicated
   visual inspector with no text or code fallback, **so that** all assets are viewed and edited in
   their native visual form.

## Version Control Integration

| ID         | Persona                    |
|------------|----------------------------|
| US-12.7.10 | environment artist (P-8)   |
| US-12.7.11 | build engineer (P-16)      |
| US-12.7.12 | environment artist (P-8)   |

1. **US-12.7.10** — **As an** environment artist (P-8), **I want** binary assets tracked via Git LFS
   with a custom merge driver that invokes structural merge, **so that** version control works
   seamlessly for binary files.
2. **US-12.7.11** — **As a** build engineer (P-16), **I want** optional lock-before-edit per asset
   type for teams preferring pessimistic locking, **so that** large binary assets are protected from
   concurrent edits.
3. **US-12.7.12** — **As an** environment artist (P-8), **I want** lock owner, modified state, and
   merge conflict indicators visible in the asset browser, **so that** I know the collaboration
   status of every asset at a glance.
