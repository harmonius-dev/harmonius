# Coding Standards

Language-specific coding standards for the Harmonius engine.

## Standards Index

| Standard | Language | Scope |
|----------|----------|-------|
| [Rust](rust.md) | Rust | Primary engine language |
| [C++](cpp.md) | C++ | FFI bridges (cxx.rs) |
| [Swift](swift.md) | Swift | macOS platform (cxx.rs) |
| [TypeScript](typescript.md) | TypeScript | Tooling, editor, build |
| [HLSL](hlsl.md) | HLSL | Shader IL (DXC pipeline) |
| [Markdown](markdown.md) | Markdown | Documentation (rumdl) |
| [JSON](json.md) | JSON | Configuration files |
| [TOML](toml.md) | TOML | Cargo.toml, .rumdl.toml |
| [YAML](yaml.md) | YAML | GitHub Actions workflows |

## Core Principles

1. **CPU cache-friendly** — contiguous memory, minimal indirection, hot/cold splitting
2. **Functional first** — immutable data, pure functions, transform pipelines
3. **Data-oriented** — arrays of structs or structs of arrays over object hierarchies
4. **Test-driven** — write tests first, no mocking, use real dependencies or full fakes
5. **100-character lines** — all code, comments, docs (tables exempt)
