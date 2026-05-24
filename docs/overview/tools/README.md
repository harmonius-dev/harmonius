# Tools

Editor framework, visual editors, profiling, and cloud deployment.

## Topics

- [editor-framework](./editor-framework.md) — editor core, plugins, and extensibility.
- [visual-editors](./visual-editors.md) — level editor, material editor, animation editor,
  graph editor, and specialized tools.
- [profiling-and-collaboration](./profiling-and-collaboration.md) — performance profilers,
  version control, and team workflows.
- [cloud-and-deployment](./cloud-and-deployment.md) — cloud build, deployment pipelines, and
  asset stores.
- [content-services](./content-services.md) — documentation, localization, modding, and
  governance.

## Key takeaways

- Property inspector with custom editors (color pickers, sliders, dropdowns) enables non-programmer
  artist friendly authoring without console scripting.
- Gizmos (visual move/rotate/scale handles in viewport) provide intuitive spatial manipulation
  more discoverable than numeric text input.
- Undo/redo system recording every change enables risk-free experimentation; artists confidently
  iterate without save checkpoints.
- Node-based editors (material graphs, shader graphs, animation state machines) enable visual
  authoring avoiding complex syntax while remaining expressive.
- Performance profilers (CPU, GPU, memory) with per-function breakdown identify bottlenecks;
  frame analysis guides optimization priority.

## Integration risks

- Undo history memory (storing all changes) can grow unbounded; memory limits necessary. See
  [editor-framework.md](./editor-framework.md) for undo buffer management.
- Node graph compilation (material/shader graphs to executable code) must handle invalid graphs
  gracefully; poor error messages confuse artists. See [visual-editors.md](./visual-editors.md)
  for validation and error reporting.
- Profiler overhead (instrumentation cost) can skew results; CPU profiling itself consumes CPU. See
  [profiling-and-collaboration.md](./profiling-and-collaboration.md) for overhead quantification.
- CI/CD pipeline failures (build failure blocking merge) require rapid diagnosis; vague error
  messages stall development. See [cloud-and-deployment.md](./cloud-and-deployment.md) for
  error communication.
- Mod sandbox escape vulnerabilities (untrusted code accessing filesystem/network) compromise
  security; strict sandboxing necessary. See [content-services.md](./content-services.md)
  for sandbox design.
