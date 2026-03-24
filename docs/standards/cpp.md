# C++ Coding Standard

## Scope

Thin FFI bridges only. C++ is used exclusively for wrapping C++ libraries (DXC, Metal Shader
Converter, Direct3D 12) and exposing them to Rust as C ABI functions via `extern "C"`. Rust consumes
the generated C headers through bindgen. No cxx.rs. We do not author C++ application logic.

## Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Class / struct | `PascalCase` | `ShaderCompiler` |
| Function | `snake_case` | `compile_shader` |
| Member variable | `snake_case` | `output_buffer` |
| Constant | `SCREAMING_SNAKE` | `MAX_BUFFER_SIZE` |
| Namespace | `snake_case` | `harmonius::dxc` |
| Macro | `SCREAMING_SNAKE` | `HRM_ASSERT` |
| Template parameter | `PascalCase` | `BackendType` |

## File Organization

- Header: `.h` extension; source: `.cpp` extension
- `#pragma once` for include guards
- One class per file pair (`.h` + `.cpp`)
- C ABI (`extern "C"`) declarations in dedicated header

## Formatting Rules

- 100-character line limit
- 4-space indentation
- Braces on same line (`K&R` style)
- `clang-format` with project `.clang-format`

## Linting Rules

- `clang-tidy` with project `.clang-tidy`
- Treat all warnings as errors (`-Werror`)
- No exceptions across FFI boundaries
- No RTTI (`-fno-rtti`)

## Type Checking

- C++23 standard (`-std=c++23`)
- Compiler warnings at maximum (`-Wall -Wextra`)
- No implicit conversions for numeric types

## How to Check and Fix

| Validation | Check command | Fix command |
|------------|--------------|-------------|
| Format | `clang-format --dry-run` | `clang-format -i` |
| Lint | `clang-tidy` | `clang-tidy --fix-errors` |
| Type-check | compiler errors | (manual) |
| Test | `ctest` | (manual) |

## Project-Specific Rules

1. **Thin bridges only** — no application logic in C++
2. **No STL file I/O** — all I/O through Rust via `IoReactor`
3. **No exceptions across FFI** — catch and convert to error codes at the bridge boundary
4. **No RTTI** — use C enums and integer codes instead
5. **`extern "C"` ABI** — all functions exposed to Rust use `extern "C"` linkage
6. **No cxx.rs** — all FFI through C ABI, not cxx.rs
7. **Minimal headers** — include only what you use; forward-declare where possible
8. **No `new`/`delete`** — use `std::unique_ptr` or Rust-side allocation

## Cache-Friendly Patterns

- **Contiguous buffers** — pass `std::span<T>` or raw pointers to contiguous Rust-allocated memory
- **Avoid virtual dispatch** — use templates or function pointers for compile-time polymorphism
- **Minimize allocations** — prefer stack allocation and pre-allocated buffers in bridge code
- **No `std::map`/`std::unordered_map`** in hot paths — use sorted vectors or Rust-side containers
- **Batch FFI calls** — minimize Rust-C++ boundary crossings by batching operations

## C ABI Memory Safety Patterns

| Category | Rule | Example |
|----------|------|---------|
| Ownership | Rust owns all heap allocations by default | Rust-side `Box<T>` |
| Ownership | C++ receives borrowed pointers | `const T*` for read-only |
| Ownership | Opaque C++ types use handle/ID pattern | `uint64_t` handle |
| Ownership | Never return owning raw pointers across FFI | Return handle or error code |
| Lifetime | C++ objects outlive Rust references | C++ side manages lifetime |
| Lifetime | COM objects: prevent `Release()` on Rust side | C++ retains ownership |
| Exceptions | C++ exceptions must not propagate across FFI | Wrap in try/catch |
| Exceptions | All `extern "C"` functions are `noexcept` | Bridge decl |
| Exceptions | Convert C++ exceptions to error codes | Return `int32_t` status |
| Buffers | Use `const uint8_t*` + `size_t` for buffer views | Read-only slice |
| Buffers | Use caller-allocated buffers for output | Pass `uint8_t*` + capacity |
| Buffers | Never pass `std::vector` across FFI | Copy into C buffer |

Exception-handling pattern:

```cpp
extern "C" int32_t compile_shader(
    const char* source, size_t source_len,
    uint8_t* out_buf, size_t out_cap, size_t* out_len
) {
    try {
        auto result = do_compile(source, source_len);
        // copy result to out_buf...
        return 0; // success
    } catch (...) {
        return -1; // error
    }
}
```

## Testing

1. **Test-driven development** — write tests first, driven by requirements (R-X.Y.Z) and features
   (F-X.Y.Z)
2. **Testability in design** — keep bridges testable by isolating FFI from logic
3. **Use real dependencies** — test against real C++ libraries, not stubs
4. **NO MOCKING** — no gmock, no mock objects; test real bridge behavior
5. **Fakes only when necessary** — write full fakes implementing the same pure virtual interface
6. **Pure virtual interfaces** — define contracts at bridge boundaries
7. **Pure functions** — bridge functions should be stateless where possible
8. **CTest** — use CTest for running C++ test suites

## Best Practices

1. Keep FFI surface area minimal
2. Use `const` everywhere possible
3. Prefer value semantics over pointer semantics
4. Use `[[nodiscard]]` on functions returning status
5. Use `constexpr` for compile-time constants
6. Document memory ownership at every FFI boundary
7. Use `static_assert` for compile-time invariants
8. Prefer `std::array` over C arrays
9. Use `std::string_view` for non-owning strings
10. Keep bridge functions under 20 lines

## Anti-Patterns

1. **Application logic in C++** — all logic belongs in Rust
2. **Raw `new`/`delete`** — use smart pointers or Rust allocation
3. **Exceptions across FFI** — undefined behavior; always catch at boundary
4. **`void*` parameters** — use typed pointers or C-compatible shared types
5. **Global state** — no global or static mutable state
6. **Deep inheritance hierarchies** — use composition
7. **`#define` for constants** — use `constexpr`
8. **Implicit conversions** — use explicit constructors and casts
9. **Large header includes** — forward-declare and include only what you use
10. **`using namespace std`** — always qualify with `std::`
