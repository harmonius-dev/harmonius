# AI Behavior ↔ Scripting Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.4.1.1 | BT graph compiles | Visual BT asset | GraphProgram with on_tick | IR-2.4.1 |
| TC-IR-2.4.1.2 | BT graph runs as system | Entity + GraphInstance | BT evaluated each frame | IR-2.4.1 |
| TC-IR-2.4.2.1 | Utility curve codegen | Linear curve graph | fn score → f32 in .dylib | IR-2.4.2 |
| TC-IR-2.4.2.2 | Custom consideration | Graph with ECS read | Score from component val | IR-2.4.2 |
| TC-IR-2.4.3.1 | BT leaf invokes graph | Leaf with fn_idx=0 | GraphFn called, Success | IR-2.4.3 |
| TC-IR-2.4.3.2 | BT leaf returns failure | GraphFn returns error | Leaf returns Failure | IR-2.4.3 |
| TC-IR-2.4.4.1 | GOAP executes graph | Plan step fn_idx=2 | Action body runs | IR-2.4.4 |
| TC-IR-2.4.4.2 | GOAP multi-step plan | 3-step plan, each graph | All 3 fns invoked in order | IR-2.4.4 |
| TC-IR-2.4.5.1 | Hot reload preserves | Change graph, reload .dylib | Variables preserved | IR-2.4.5 |
| TC-IR-2.4.5.2 | Hot reload version bump | New .dylib version | needs_reload() = true | IR-2.4.5 |
| TC-IR-2.4.6.1 | Coroutine suspends | Patrol with 3 waypoints | Yields between waypoints | IR-2.4.6 |
| TC-IR-2.4.6.2 | Coroutine resumes | Suspended + next frame | Resumes at saved variant | IR-2.4.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.4.1.B1 | 1000 BT graph ticks | < 4 ms | IR-2.4.1 |
| TC-IR-2.4.2.B1 | 500 utility score calls | < 0.5 ms | IR-2.4.2 |
| TC-IR-2.4.5.B1 | Hot reload turnaround | < 1 s | IR-2.4.5 |
| TC-IR-2.4.6.B1 | 200 coroutine resumes | < 0.2 ms | IR-2.4.6 |
