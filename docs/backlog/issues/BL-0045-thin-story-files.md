---
blocked_by: []
blocks: []
created: 2026-05-20
domain: meta
effort: L
id: BL-0045
labels: [coverage:stories, domain:meta, p2, status:triage, type:user-story]
priority: P2
status: triage
title: Backfill thin user-story files
---

## Backfill thin user-story files

### Context

The 2026-05 audit identified user-story files with fewer than ten stories: `event-logs.md` (8),
`ui-rendering.md` (7), `localization-editor.md` (6), `containers-slots.md` (10 borderline),
`scripting.md` (8), `animation/first-person.md` (8).

### Acceptance criteria

- [ ] Each file expanded to at least 12 stories with persona references.
- [ ] Stories follow the AGENTS rule: granular, "As a … I want … so that …".
- [ ] No umbrella / parent compound stories (also see BL-0047).
- [ ] `user-stories/README.md` counts updated.

### Verification

Each affected file has ≥ 12 numbered stories with persona table.

### References

- [docs/coverage/audits/2026-05-audit.md](../../coverage/audits/2026-05-audit.md)
- [docs/user-stories/AGENTS.md](../../user-stories/AGENTS.md)
