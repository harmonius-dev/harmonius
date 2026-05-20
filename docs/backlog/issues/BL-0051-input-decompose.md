---
blocked_by: []
blocks: []
created: 2026-05-20
domain: input
effort: XL
id: BL-0051
labels: [coverage:design, domain:input, p2, status:triage, type:design]
priority: P2
status: triage
title: Decompose input domain into mid-level designs
---

## Decompose input domain into mid-level designs

### Context

`design/input/` has only one file (`input.md`) covering devices, action mapping, gestures, haptics,
VR input, and IME (BL-0035). The 2026-05 audit and the
[design coverage roadmap](../../design/coverage-roadmap.md) propose splitting into:

- `input-devices.md` — keyboard, mouse, controller, touch device abstraction
- `input-actions.md` — action mapping, contexts, runtime rebinding
- `input-gestures.md` — touch gestures, multi-touch
- `input-haptics.md` — rumble, force feedback
- `input-ime.md` — IME composition / commit (BL-0035)

### Acceptance criteria

- [ ] Five new design docs replace the monolithic `input.md`.
- [ ] Each has a companion `*-test-cases.md`.
- [ ] `architecture.md` Input section updated.
- [ ] Existing cross-references updated.

### Verification

`input.md` becomes a short overview pointing at the five sub-docs.

### References

- [docs/design/coverage-roadmap.md](../../design/coverage-roadmap.md)
- [BL-0035 Input IME support](BL-0035-input-ime-support.md)
