# Editor ↔ Animation Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.3.1.1 | Timeline shows keyframes | Clip with 30 keyframes on 3 tracks | 3 tracks visible, 30 markers | IR-5.3.1 |
| TC-IR-5.3.1.2 | Timeline scrub updates pose | Scrub to t=0.5 | Skeleton at mid-animation pose | IR-5.3.1 |
| TC-IR-5.3.2.1 | Curve editor Hermite tangents | BoneTrack with Hermite curves | Hermite in/out tangents visible and draggable | IR-5.3.2 |
| TC-IR-5.3.2.2 | Curve edit updates clip | Drag tangent handle | AnimationClip modified, undo recorded | IR-5.3.2 |
| TC-IR-5.3.2.3 | Curve editor Bezier tangents | BoneTrack with Bezier curves | Bezier control points visible and draggable | IR-5.3.2 |
| TC-IR-5.3.2.4 | Bezier tangent edit round-trip | Drag Bezier handle, undo | Clip reverts to pre-edit tangent pair | IR-5.3.2 |
| TC-IR-5.3.3.1 | Blend space editor 1D | BlendSpace1D with 3 samples | 1D ruler with 3 sample points | IR-5.3.3 |
| TC-IR-5.3.3.2 | Blend space editor 2D | BlendSpace2D with triangulation | 2D grid with Delaunay triangles | IR-5.3.3 |
| TC-IR-5.3.3.3 | Blend param preview | Set speed=0.5 in blend space | Preview blends walk/run at 50% | IR-5.3.3 |
| TC-IR-5.3.4.1 | State graph renders nodes | StateGraph with 5 states | 5 nodes with transition arrows | IR-5.3.4 |
| TC-IR-5.3.4.2 | Active state highlighted | Preview playing, idle state | Idle node highlighted in green | IR-5.3.4 |
| TC-IR-5.3.4.3 | Transition debug overlay | Transition in progress | Blend weight bar on transition edge | IR-5.3.4 |
| TC-IR-5.3.5.1 | Preview play/pause | Press play in timeline | Animation plays in viewport | IR-5.3.5 |
| TC-IR-5.3.5.2 | Preview step frame | Step forward 1 frame | Pose advances by 1 tick | IR-5.3.5 |
| TC-IR-5.3.6.1 | Bone selection click | Click on forearm bone in viewport | Bone highlighted, properties shown | IR-5.3.6 |
| TC-IR-5.3.6.2 | Bone hierarchy selection | Select parent bone | Children bones dimmed but visible | IR-5.3.6 |
| TC-IR-5.3.7.1 | Add event marker on timeline | Right-click at t=0.75 | AnimEventMarker added at that time | IR-5.3.7 |
| TC-IR-5.3.7.2 | Event fires during preview | Play past event marker at t=0.75 | Event observer fires, log entry | IR-5.3.7 |

## Failure-Mode Tests

These tests cover every row of the design's Failure Modes table. All are negative tests and all are
CI-runnable headless (no GPU required except TC-IR-5.3.F5, which uses the software skinning fake).

| ID | Test | Input | Expected | Failure Row |
|----|------|-------|----------|-------------|
| TC-IR-5.3.F1 | Invalid clip handle | `Play { clip: bogus_handle }` | T-pose placeholder, no panic, warn log | Invalid clip handle |
| TC-IR-5.3.F2 | Blend space missing samples | BlendSpace2D with 0 samples, set param | Nearest-sample clamp (bind pose), warn log | Blend space missing samples |
| TC-IR-5.3.F3 | Bone index out of range | Overlay request for bone index = bone_count | Overlay skipped for that bone, others render | Bone index out of range |
| TC-IR-5.3.F4 | State graph cycle | StateGraphDef with A->B->A cycle both always-true | Cycle broken, warn log, no infinite loop | State graph cycle detected |
| TC-IR-5.3.F5 | GPU skinning dispatch fails | Force dispatch error in software skinning fake | Falls back to bind pose, warn log, no crash | GPU skinning dispatch fails |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.3.1.B1 | Timeline render 100 tracks | < 16 ms per frame | IR-5.3.1 |
| TC-IR-5.3.5.B1 | Preview latency scrub-to-display | < 2 frames | IR-5.3.5 |
| TC-IR-5.3.6.B1 | Bone pick raycast 200-bone skeleton | < 1 ms | IR-5.3.6 |
| TC-IR-5.3.F.B1 | BonePoseSnapshot MPSC throughput 256 entities | < 0.2 ms | IR-5.3.5 |
