# Codegen middleman .dylib + bundled rustc

## Status

Accepted — 2024-10-23 (backfilled 2026-05-20)

## Context

User-authored content in the editor — components, materials, shading models, blueprints, logic
graphs, animation state machines — must be runnable inside the engine. Two industry patterns
exist:

1. **Reflection + bytecode VM.** Define types in the editor; the engine reflects on them at
   runtime; logic compiles to bytecode interpreted by an embedded VM (Unity's IL2CPP-prior,
   Unreal's Blueprint VM, Bevy's `bevy_reflect`).
2. **AOT codegen.** Define types in the editor; codegen produces real source compiled by a
   bundled toolchain; the engine loads compiled code without reflection.

Pattern 1 has runtime cost (interpreter dispatch, reflection lookups), versioning hazards, and
prevents many compile-time optimizations. Pattern 2 needs a bundled toolchain but pays no
runtime cost.

## Decision

All user-authored content compiles to real Rust source through a codegen pipeline. The bundled
`rustc` (shipped inside the engine installer) compiles that source into a single shared library
called the **middleman `.dylib`**. The engine binary loads the middleman at startup via
`libloading`. Hot reload recompiles the middleman and re-loads it; the engine binary itself is
stable across edit sessions.

For shipping builds, the middleman and all plugins are statically linked into a single
executable with LTO. No `.dylib` is shipped.

## Consequences

- Zero runtime reflection. No `dyn Reflect`, no `TypeRegistry`, no runtime `TypeId` dispatch.
- Static dispatch end-to-end. LLVM sees concrete types and inlines aggressively.
- The engine ships a Rust toolchain (`rustc`, `cargo`) inside the installer. Users never
  interact with it.
- Hot reload speed is bounded by `cargo` incremental compilation. Target sub-3-second reload.
- Plugins are data, not DLLs — plugin authors define schemas, the codegen pipeline generates
  Rust source. There is no dynamic plugin loading at runtime; everything compiles together.
- Installer size grows by the size of the bundled toolchain (~200 MB). Acceptable trade for
  the runtime performance and safety.

## Alternatives Considered

- **Bytecode VM** — rejected because runtime cost matters in tight game loops, and reflection
  introduces a class of versioning bugs.
- **WASM as compile target** — promising but adds a second toolchain (wasm-bindgen) and a
  WASM runtime. The marginal benefit over native Rust is small for a desktop engine.
- **AOT C++ codegen** — works for engines already in C++, but would force the engine to
  re-introduce a C++ toolchain on top of the Rust one.

## References

- [docs/design/constraints.md](../../design/constraints.md) "Codegen and Hot-Reload Architecture"
- [docs/design/core-runtime/hot-reload-protocol.md](../../design/core-runtime/hot-reload-protocol.md)
- [docs/architecture.md](../../architecture.md) "Codegen Pipeline"
