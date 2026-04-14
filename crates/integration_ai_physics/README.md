# integration_ai_physics

Reference implementation and CI-runnable tests for the AI ↔ physics integration design
(`docs/design/integration/ai-physics.md`).

This crate provides pure, deterministic helpers plus a small in-memory scene used by unit tests.
It is not wired into a runtime loop yet; it encodes the data contracts and fallback counters (FM-*)
called out in the design.
