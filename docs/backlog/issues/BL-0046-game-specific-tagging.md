---
blocked_by: []
blocks: []
created: 2026-05-20
domain: meta
effort: M
id: BL-0046
labels: [coverage:audit, domain:meta, p3, status:triage, type:user-story]
priority: P3
status: triage
title: Apply [game-specific] tag consistently
---

## Apply `[game-specific]` tag consistently

### Context

Per `user-stories/AGENTS.md` rule 8, game-specific stories must carry the `[game-specific]` tag. The
2026-05 audit found this tag applied inconsistently — mostly only in `game-framework/`. Many stories
elsewhere are also game-specific (e.g., physics `vehicle-physics.md`, animation `first-person.md`
motion).

### Acceptance criteria

- [ ] Audit pass over all user-story files; tag every game-specific story.
- [ ] Engine-primitive stories have no tag.
- [ ] Audit recorded in `coverage/audits/`.

### Verification

A spot-check of 20 random stories shows correct tagging.

### References

- [docs/user-stories/AGENTS.md](../../user-stories/AGENTS.md) Rule 8
- [docs/coverage/audits/2026-05-audit.md](../../coverage/audits/2026-05-audit.md)
