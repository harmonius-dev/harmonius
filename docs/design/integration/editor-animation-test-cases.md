# Editor ↔ Animation Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.3.1.1 | Timeline shows keyframes | Clip with 30 keyframes on 3 tracks | 3 tracks visible, 30 markers | IR-5.3.1 |
| TC-IR-5.3.1.2 | Timeline scrub updates pose | Scrub to t=0.5 | Skeleton at mid-animation pose | IR-5.3.1 |
| TC-IR-5.3.2.1 | Curve editor displays tangents | BoneTrack with Hermite curves | Bezier handles visible and draggable | IR-5.3.2 |
| TC-IR-5.3.2.2 | Curve edit updates clip | Drag tangent handle | AnimationClip modified, undo recorded | IR-5.3.2 |
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

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.3.1.B1 | Timeline render 100 tracks | < 16 ms per frame | IR-5.3.1 |
| TC-IR-5.3.5.B1 | Preview latency scrub-to-display | < 2 frames | IR-5.3.5 |
| TC-IR-5.3.6.B1 | Bone pick raycast 200-bone skeleton | < 1 ms | IR-5.3.6 |
