# Rust as primary language

## Status

Accepted — 2024-08-12 (backfilled 2026-05-20)

## Context

Game engines are most commonly built in C++ (Unreal, CryEngine, custom AAA stacks) or C#
runtime atop a C++ core (Unity). Both choices carry significant ongoing costs: undefined
behavior, slow build times, manual memory management, and weak concurrency primitives.
Modern alternatives (Rust, Zig, Carbon) offer different trade-offs. The project needs a single
language for engine, editor, codegen output, headless game server, and most tooling.

## Decision

Rust (stable channel only — no nightly features) is the primary language for every component:
engine, editor, headless game server, asset pipeline, codegen targets, and command-line tools.
Visual editor output codegens to Rust source compiled by a bundled `rustc` into a middleman
`.dylib` (see [ADR-0005](ADR-0005-codegen-middleman-dylib.md)).

## Consequences

- The build pipeline is `cargo` everywhere; no CMake, no MSBuild, no Bazel as primary driver.
- Bundling `rustc` and `cargo` inside the engine installer is mandatory for hot reload and
  shipping codegen.
- Crate availability constrains library choice. Some platform APIs require thin Rust wrappers
  (`windows-rs`, `objc2`, `ash`, `x11rb`, `wayland-client`).
- No nightly features means slower adoption of language conveniences in exchange for shipping
  stability.
- ECS-primary architecture ([ADR-0003](ADR-0003-ecs-primary-architecture.md)) plays naturally
  with Rust's borrow checker.

## Alternatives Considered

- **C++** — broadest game industry mindshare, but the project explicitly aims to avoid the UB
  / build-time / memory-safety costs that motivated this engine.
- **C#** — accessible authoring language, but introduces a managed runtime dependency in the
  shipping binary that conflicts with the no-runtime philosophy.
- **Zig** — fast and minimal, but smaller crate ecosystem and less mature tooling at the time
  of decision.

## References

- [docs/design/constraints.md](../../design/constraints.md) "Language and Toolchain"
- [docs/architecture.md](../../architecture.md) "Engine Overview"
- [README.md philosophy](../../../README.md)
