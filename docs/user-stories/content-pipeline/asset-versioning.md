# User Stories — 12.7 Asset Versioning & Collaboration

## US-12.7.1 Collaborate on Binary Assets Without Merge Conflicts Blocking Work
**As an** artist, **I want** a universal binary format, structural diffing, three-way merge,
automatic conflict resolution, visual inspectors, a data table editor, and Git LFS
integration with a custom merge driver, **so that** my team can work on the same assets
concurrently without losing changes or needing to understand Git internals.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Universal binary format with mmap and O(1) sections | F-12.7.1 | R-12.7.1 |
| Compressed bundles with partial decompression | F-12.7.2 | R-12.7.2 |
| Structural diffs shown in the asset editor's visual style | F-12.7.3 | R-12.7.3 |
| Three-way merge auto-succeeds for non-overlapping changes | F-12.7.4 | R-12.7.4 |
| Conservative automatic conflict resolution | F-12.7.5 | R-12.7.5 |
| Spreadsheet data table editor with validation | F-12.7.6 | R-12.7.6 |
| Visual inspector for every asset type (no text fallback) | F-12.7.7 | R-12.7.7 |
| Git LFS tracking with custom merge driver | F-12.7.8 | R-12.7.8 |
