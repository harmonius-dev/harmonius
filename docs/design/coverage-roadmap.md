# Design Coverage Roadmap

This roadmap lists the mid- and low-level design decompositions that the corpus still needs. Each
entry maps to a backlog issue. Closing every roadmap entry means every domain either has a coherent
set of mid-level docs or has been consciously left as a single top-level doc with a documented
reason.

## Status legend

| Status         | Meaning                                                            |
|----------------|--------------------------------------------------------------------|
| Owned          | Decomposition exists or is intentionally a single doc              |
| Roadmap        | Decomposition planned; backlog issue tracks the work               |
| Deferred       | Acknowledged gap; no backlog issue yet (not on the current cycle)  |

## Monolithic single-file domains

Three domains carry one design file each. Each is a candidate for decomposition.

### Audio (`design/audio/`)

Current: `audio.md` (everything from RT mixer to HRTF). Roadmap split into five files:

| Proposed file                                   | Owns                                                                            | Status   | Backlog                                                |
|-------------------------------------------------|---------------------------------------------------------------------------------|----------|--------------------------------------------------------|
| `audio-runtime.md`                              | RT mixer thread, command queue, lifecycle                                        | Roadmap  | [BL-0050](../backlog/issues/BL-0050-audio-decompose.md) |
| `audio-mixing.md`                               | Mixer graph, buses, ducking                                                     | Roadmap  | [BL-0050](../backlog/issues/BL-0050-audio-decompose.md), [BL-0036](../backlog/issues/BL-0036-audio-ducking.md) |
| `audio-streaming.md`                            | File streaming, decoder, ring buffers                                           | Roadmap  | [BL-0050](../backlog/issues/BL-0050-audio-decompose.md) |
| `audio-spatial.md`                              | HRTF, positional, occlusion                                                     | Roadmap  | [BL-0037](../backlog/issues/BL-0037-hrtf-sofa-loading.md), [BL-0050](../backlog/issues/BL-0050-audio-decompose.md) |
| `audio-codecs.md`                               | Supported codecs, profile loading                                               | Roadmap  | [BL-0050](../backlog/issues/BL-0050-audio-decompose.md) |

After the split, `audio.md` becomes a short overview index pointing at the five sub-docs.

### Input (`design/input/`)

Current: `input.md` (devices through IME). Roadmap split into five files:

| Proposed file                                   | Owns                                                                            | Status   | Backlog                                                |
|-------------------------------------------------|---------------------------------------------------------------------------------|----------|--------------------------------------------------------|
| `input-devices.md`                              | Keyboard, mouse, controller, touch device abstraction                           | Roadmap  | [BL-0051](../backlog/issues/BL-0051-input-decompose.md) |
| `input-actions.md`                              | Action mapping, contexts, runtime rebinding                                     | Roadmap  | [BL-0051](../backlog/issues/BL-0051-input-decompose.md) |
| `input-gestures.md`                             | Touch gestures, multi-touch                                                     | Roadmap  | [BL-0051](../backlog/issues/BL-0051-input-decompose.md) |
| `input-haptics.md`                              | Rumble, force feedback                                                          | Roadmap  | [BL-0051](../backlog/issues/BL-0051-input-decompose.md) |
| `input-ime.md`                                  | IME composition / commit                                                        | Roadmap  | [BL-0035](../backlog/issues/BL-0035-input-ime-support.md) |

### UI (`design/ui/`)

Current: `ui-framework.md` (everything). Roadmap split into four files:

| Proposed file                                   | Owns                                                                            | Status   | Backlog                                                |
|-------------------------------------------------|---------------------------------------------------------------------------------|----------|--------------------------------------------------------|
| `ui-framework.md` (kept; trimmed)               | Widget framework, lifecycle, event routing                                      | Roadmap  | [BL-0052](../backlog/issues/BL-0052-ui-decompose.md)   |
| `ui-layout.md`                                  | Layout engine                                                                   | Roadmap  | [BL-0052](../backlog/issues/BL-0052-ui-decompose.md)   |
| `ui-rendering.md`                               | Text shaping, glyph atlas, batching, scissor, masking                           | Roadmap  | [BL-0052](../backlog/issues/BL-0052-ui-decompose.md)   |
| `ui-accessibility.md`                           | Keyboard nav, screen reader, WCAG plan                                          | Roadmap  | [BL-0039](../backlog/issues/BL-0039-accessibility-wcag.md), [BL-0052](../backlog/issues/BL-0052-ui-decompose.md) |

## Missing integration pairs

The 2026-04-12 design review §3.8 identified missing integration pairs. Three are tracked on this
roadmap; the other items in that section have already been authored or are tracked elsewhere.

| Pair                                      | Status   | Backlog                                                 |
|-------------------------------------------|----------|---------------------------------------------------------|
| `attributes-effects-rendering.md`          | Roadmap  | [BL-0053](../backlog/issues/BL-0053-attributes-rendering-integration.md) |
| `event-logs-animation.md`                  | Roadmap  | [BL-0054](../backlog/issues/BL-0054-event-logs-animation-integration.md) |
| `vfx-audio.md`                             | Roadmap  | [BL-0055](../backlog/issues/BL-0055-vfx-audio-integration.md)            |

## Domains with intentional single-doc shape

Some domains are intentionally one design file because the surface area is small. They are not
decomposition candidates.

| Domain               | File                            | Reason for single-doc                                 |
|----------------------|---------------------------------|-------------------------------------------------------|
| `data-systems/`      | `composition.md`                | Composition is a *guide*, not a subsystem             |

## Coverage summary

| Domain          | Existing design files | Roadmap files | Total target |
|-----------------|----------------------:|--------------:|-------------:|
| Audio           |                     1 |             4 |            5 |
| Input           |                     1 |             4 |            5 |
| UI              |                     1 |             3 |            4 |
| Integration     |                    62 |             3 |           65 |

Closing the roadmap moves Audio + Input + UI from monolithic to mid-level coverage and adds three
integration pairs. The work is sized in the backlog (XL for the three decomposes; M for the three
integration pairs).

## Out-of-scope work

- Splitting other domains (rendering, animation, physics, networking) is not on the
  roadmap. They already have multiple mid-level docs.
- Test-case companion files for the new docs are required per `design/AGENTS.md` rule 2 and
  are part of each backlog issue's acceptance criteria.

## When to update this roadmap

- When a backlog issue closes, change the entry's Status from `Roadmap` to `Owned`.
- When a new monolithic doc is found in audit, add a new section.
- When a domain is decomposed via a different shape (e.g., a sub-folder), update the entry
  to reflect the actual structure.
