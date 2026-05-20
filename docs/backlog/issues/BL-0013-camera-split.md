---
blocked_by: []
blocks: []
created: 2026-05-20
domain: game-framework
effort: M
id: BL-0013
labels: [coverage:design, domain:game-framework, domain:rendering, p1, status:triage, type:design]
priority: P1
status: triage
title: Camera split (brain vs lens / render concerns)
---

# Camera split (brain vs lens / render concerns)

## Context

`game-framework/camera.md` historically contained spring-arm collision retraction, cine
camera, picture-in-picture, and camera sequencer. The 2026-04-12 review §2.9 P1 #19 found
that the last three are rendering or sequencing concerns, not gameplay.

`rendering/camera-rendering.md` was created to host the rendering concerns, but the split is
incomplete: `game-framework/camera.md` still owns some lens parameters that should live in
the rendering doc.

## Acceptance criteria

- [ ] `game-framework/camera.md` owns the camera **brain**: priorities, blends, target
      tracking, feel parameters.
- [ ] `rendering/camera-rendering.md` owns the **lens / render concerns**: spring arm
      collision, cine camera, PiP, viewport composition, camera sequencer.
- [ ] No type or property is defined in both files.
- [ ] Cross-references between the two files are explicit.
- [ ] Editor camera UI shows both brain and lens panels with clear separation.

## Verification

`grep -rE 'spring_arm|picture_in_picture|cine_camera' docs/design/game-framework/` returns
zero matches; same terms appear only in `rendering/camera-rendering.md`.

## References

- [docs/design/design-review.md §2.9 / P1 #19](../../design/design-review.md#29-game-framework-scope-creep)
- [docs/design/game-framework/camera.md](../../design/game-framework/camera.md)
- [docs/design/rendering/camera-rendering.md](../../design/rendering/camera-rendering.md)
