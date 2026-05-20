---
blocked_by: []
blocks: []
created: 2026-05-20
domain: ui
effort: M
id: BL-0039
labels: [coverage:design, domain:ui, p3, status:triage, type:design]
priority: P3
status: triage
title: Define WCAG compliance test plan
---

# Define WCAG compliance test plan

## Context

The 2026-04-12 review §3.6 noted WCAG compliance is aspirational with no automated checks
defined. Accessibility duplicates core UI event routing via `AccessibilityTree` instead of
hooking into it.

## Acceptance criteria

- [ ] `ui/ui-framework.md` (or new `ui/accessibility.md`) documents the accessibility model:
      hooks into core UI event routing rather than parallel structure.
- [ ] WCAG 2.2 AA conformance tests listed: contrast ratios, keyboard navigation, focus
      indicators, ARIA-equivalent semantic exposure to screen readers.
- [ ] Automated accessibility check integration documented (axe-core equivalent or custom).
- [ ] Companion test cases cover keyboard-only workflows.

## Verification

A keyboard-only walkthrough of an editor scene reaches every interactive element with
visible focus indicators. Screen reader announces every label correctly.

## References

- [docs/design/design-review.md §3.6 / P2 #57](../../design/design-review.md#36-ai--audio--input--ui--networking)
- [docs/design/ui/ui-framework.md](../../design/ui/ui-framework.md)
