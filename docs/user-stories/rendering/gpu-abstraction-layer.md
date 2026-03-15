# GPU Abstraction Layer User Stories

## US-2.1.1 Cross-Platform Shader Development

**As a** graphics programmer, **I want** a unified GPU backend trait with static dispatch
across Metal, D3D12, and Vulkan, **so that** I can write rendering code once and have it run
on all supported platforms without vtable overhead or backend-specific branching.

## US-2.1.2 GPU Memory Profiling

**As a** graphics programmer, **I want** per-pass GPU timestamp queries and memory allocation
statistics exposed through the abstraction layer, **so that** I can identify performance
bottlenecks and optimize VRAM usage without resorting to platform-specific profiling tools.
