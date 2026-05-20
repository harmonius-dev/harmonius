---
blocked_by: []
blocks: []
created: 2026-05-20
domain: input
effort: M
id: BL-0035
labels: [coverage:design, domain:input, p3, status:triage, type:design]
priority: P3
status: triage
title: Add IME support spec to input.md
---

# Add IME support spec to `input.md`

## Context

`input/input.md` has no IME story. CJK text input is broken without IME composition events.
Per-platform IME APIs (TSF on Windows, NSTextInputContext on macOS, IBus on Linux) need
abstraction.

## Acceptance criteria

- [ ] IME composition state model documented (preedit, candidates, commit).
- [ ] Per-platform mapping documented (TSF / NSTextInputContext / IBus / Wayland
      text-input-v3).
- [ ] UI integration: text widgets receive composition + commit events.
- [ ] Test cases cover Japanese, Korean, and Chinese IME flows.

## Verification

A text widget accepts Japanese input via Microsoft IME, macOS Hiragana, and Linux ibus
without dropped composition.

## References

- [docs/design/design-review.md §3.6 / P2 #57](../../design/design-review.md#36-ai--audio--input--ui--networking)
- [docs/design/input/input.md](../../design/input/input.md)
