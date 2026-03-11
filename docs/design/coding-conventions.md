# C++26 Coding Conventions

Coding conventions for the Harmonius GPU graphics framework. Based on the
[Google C++ Style Guide](https://google.github.io/styleguide/cppguide.html) with adaptations
for C++26, modern GPU programming, and project-specific constraints.

---

## Contents

- [C++26 Coding Conventions](#c26-coding-conventions)
  - [Contents](#contents)
  - [Deviations from Google C++ Style Guide](#deviations-from-google-c-style-guide)
  - [Module Organization](#module-organization)
    - [File layout within the source tree](#file-layout-within-the-source-tree)
  - [Namespace Rules](#namespace-rules)
  - [Type Design](#type-design)
    - [Value types](#value-types)
    - [Strongly-typed handles](#strongly-typed-handles)
    - [Structs vs. classes](#structs-vs-classes)
    - [Construction and copy/move](#construction-and-copymove)
    - [Other rules](#other-rules)
  - [Error Handling](#error-handling)
  - [Memory and Ownership](#memory-and-ownership)
  - [Concurrency](#concurrency)
  - [Templates and Concepts](#templates-and-concepts)
  - [Formatting](#formatting)
  - [Comments and Documentation](#comments-and-documentation)
  - [Imports and Includes](#imports-and-includes)
  - [Code Examples](#code-examples)
    - [Concept definition](#concept-definition)
    - [Class with proper naming conventions](#class-with-proper-naming-conventions)
    - [Enum class with error handling via `std::expected`](#enum-class-with-error-handling-via-stdexpected)
    - [Struct with designated initializer usage](#struct-with-designated-initializer-usage)
    - [Module file layout](#module-file-layout)

---

## Deviations from Google C++ Style Guide

The following table summarizes where this project deliberately departs from the Google C++
Style Guide.

| Area             | Google C++ Style Guide      | Harmonius               | Rationale                                                                                                  |
| ---------------- | --------------------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------- |
| Language version | C++20                       | C++26                   | Use the latest standard; C++26 features are encouraged.                                                    |
| Line length      | 80 characters               | 120 characters          | Better readability for template-heavy and concept-heavy code.                                              |
| Modules          | Headers with include guards | C++20 modules (`.cppm`) | Modules eliminate the need for include guards, reduce compilation times, and enforce better encapsulation. |
| Exceptions       | Prohibited                  | Prohibited              | Same policy. Use `std::expected<T, E>` for fallible operations instead.                                    |
| RTTI             | Prohibited                  | Prohibited              | Same policy. Use concepts for Compile-time type Dispatch.                                                  |
| Streams          | Discouraged                 | Prohibited              | No `std::iostream` in production code. Use `std::format` and `std::print` for output.                      |
| File IO          | `std::fstream`              | Prohibited              | No C++ stdlib file IO. Use platform-native async IO: IOCP (Windows), dispatch_io (macOS), io_uring (Linux). |

---

## Module Organization

Each namespace maps to a named C++20 module. Module files use the `.cppm` extension.

- **Module naming:** `harmonius.<subsystem>` or `harmonius.<subsystem>.<component>`
  - `harmonius.rg` -- render graph core types
  - `harmonius.gpu` -- GPU backend interface
  - `harmonius.gpu_runtime.memory` -- GPU runtime memory management
- **Module partitions** for implementation details: `harmonius.rg.core:internal`
- **Re-export** commonly used types from the parent module so consumers need only a single
  `import` statement.
- **One module per file.** Do not place multiple module declarations in a single file.

### File layout within the source tree

Module files live under `src/` and mirror the namespace hierarchy:

```
src/
  gpu/
    harmonius.gpu.cppm
    harmonius.gpu.concepts.cppm
    metal/
      harmonius.gpu.metal.cppm
    vulkan/
      harmonius.gpu.vulkan.cppm
    d3d12/
      harmonius.gpu.d3d12.cppm
  gpu_runtime/
    harmonius.gpu_runtime.cppm
    harmonius.gpu_runtime.memory.cppm
  rg/
    core/
      harmonius.rg.cppm
    builder/
      harmonius.rg.builder.cppm
    compiler/
      harmonius.rg.compiler.cppm
```

---

## Namespace Rules

- **Root namespace:** `harmonius`
- **Subsystem namespaces:** `harmonius::rg`, `harmonius::gpu`, `harmonius::gpu_runtime`,
  `harmonius::asset`
- **Component namespaces:** `harmonius::rg::builder`, `harmonius::gpu_runtime::memory`
- **Internal implementation namespaces:** `harmonius::rg::detail`,
  `harmonius::gpu_runtime::detail`
- **No `using namespace` directives.** Always qualify names explicitly.
- **No anonymous namespaces in module interface units.** Use module partitions or `detail`
  namespaces for internal symbols.

---

## Type Design

### Value types

Prefer value types over pointer types. Pass small types by value, larger types by `const&`
or by move.

### Strongly-typed handles

Use `enum class` with an explicit underlying type for opaque GPU handles. This prevents accidental implicit conversions
between unrelated handle types.

```cpp
enum class TextureHandle : std::uint64_t { kInvalid = 0 };
enum class BufferHandle : std::uint64_t { kInvalid = 0 };
```

### Structs vs. classes

- **Structs** are passive data aggregates with public fields and no invariants. Use aggregate initialization with
  designated initializers.
- **Classes** are types with invariants maintained by private data and public methods.

### Construction and copy/move

- Mark single-argument constructors `explicit`.
- Explicitly default or delete copy and move operations. Never rely on implicit generation
  when the type manages resources.
- Prefer move-only types for resource ownership (`std::unique_ptr`,
  `std::move_only_function`).

### Other rules

- Use `[[nodiscard]]` on all functions returning non-void values.
- Use `constexpr` wherever possible.
- Prefer `auto` with trailing return types for function declarations when the return type is
  complex or template-dependent.

---

## Error Handling

- **No exceptions anywhere in the codebase.** Compile with `-fno-exceptions` or equivalent.
- Use `std::expected<T, E>` for operations that can fail.
- Define error types as `enum class` with descriptive values.
- Mark all functions returning `std::expected` with `[[nodiscard]]`.
- Use `assert()` for programmer errors and invariant violations in debug builds.
- Do not use error codes conveyed through return values or output parameters; use
  `std::expected` exclusively.

```cpp
enum class ResourceError : std::uint8_t {
  kOutOfMemory,
  kInvalidFormat,
  kInvalidDimensions,
  kUnsupportedUsage,
};

[[nodiscard]] auto CreateTexture(const TextureDesc& desc) -> std::expected<TextureHandle, ResourceError>;
```

---

## Memory and Ownership

- **No raw `new`/`delete`** in user-facing code.
- `std::unique_ptr` for exclusive ownership.
- `std::span` for non-owning views of contiguous data.
- `std::string_view` for non-owning string references.
- Stack allocation preferred; use the heap only when necessary.
- **No `std::shared_ptr`** unless strongly justified. If used, document the rationale in a
  comment at the declaration site.
- Use RAII for all resource management. Destructors must release resources deterministically.

---

## Concurrency

- No mutable shared state without explicit synchronization.
- Prefer lock-free algorithms: atomic operations, ring buffers, single-producer /
  single-consumer queues.
- Use thread-local storage for per-thread caches and scratch allocators.
- No `std::mutex` on hot paths. If a mutex is needed, it must be off the critical rendering
  path.
- Document thread-safety guarantees on every public type with a `@threadsafety` Doxygen tag.

---

## Templates and Concepts

- Use C++20 concepts for **all** compile-time polymorphism.
- **No SFINAE.** Replace all `std::enable_if` and SFINAE tricks with concepts and `requires`
  clauses.
- **No CRTP.** Use concepts to express interface requirements instead.
- **No virtual methods, vtables, or abstract base classes.** All dispatch is static via
  concepts. Runtime polymorphism is not used.
- Keep concept definitions minimal and focused. Each concept should express a single
  capability or interface contract.
- Place concept definitions in dedicated module files (e.g., `harmonius.gpu.concepts.cppm`).

---

## Formatting

- **Formatter:** clang-format with the project `.clang-format` configuration.
- **Indentation:** 4 spaces. No tabs.
- **Line length:** 120 characters. May be exceeded only when a line cannot be split without
  changing semantics (e.g., a long string literal). The limit does not apply to table rows
  in Markdown documents.
- **Braces:** opening brace on the same line as the statement. Closing brace on its own line.
- **No trailing whitespace.**
- **Single blank line** between function definitions and between logical sections.
- **No `#pragma once`** -- modules eliminate the need for include guards.
- **Trailing return types** are encouraged when they improve readability, especially for
  template-heavy or `auto`-deduced return types.

```cpp
// Opening brace on same line
auto Allocate(std::uint64_t size_bytes) -> std::expected<BufferHandle, ResourceError> {
  // 4-space indentation
  if (size_bytes == 0) {
    return std::unexpected(ResourceError::kInvalidDimensions);
  }
  // ...
}
```

---

## Comments and Documentation

- **Documentation comments** (`///` or `/** */`) on all public API surfaces. Use
  Doxygen-compatible format.
- **Implementation comments** sparingly -- only for complex algorithms, non-obvious logic,
  or workarounds.
- **No commented-out code.** Remove dead code; version control preserves history.
- **TODO format:** `// TODO(username): description`
- **Section separators** within a file use a line of dashes inside a comment block:

```cpp
// ---------------------------------------------------------------------------
// Section name
// ---------------------------------------------------------------------------
```

---

## Imports and Includes

- Use `import` for C++20 modules.
- Use `#include` only for C system headers and third-party libraries that are not yet
  modularized.
- **Import order** (each group separated by a blank line):
  1. Own module partitions
  2. Harmonius modules
  3. Standard library modules (`import std;`)
  4. Third-party includes

```cpp
export module harmonius.rg.compiler;

import harmonius.rg;
import harmonius.gpu;

import std;
```

---

## Code Examples

### Concept definition

```cpp
/// A GPU device capable of creating resources, recording commands, and submitting work.
template <typename D>
concept GpuDevice = GpuCommandPool<typename D::CommandPoolType> &&
                    requires(D d, const D cd, QueueType qt, const TextureDesc& tex_desc, TextureHandle tex,
                             const BufferDesc& buf_desc, BufferHandle buf) {
                      { cd.Capabilities() } -> std::same_as<DeviceCapabilities>;
                      { cd.QueueCount(qt) } -> std::same_as<std::uint32_t>;
                      { d.CreateTexture(tex_desc) } -> std::same_as<std::expected<TextureHandle, ResourceError>>;
                      { d.DestroyTexture(tex) } -> std::same_as<void>;
                      { d.CreateBuffer(buf_desc) } -> std::same_as<std::expected<BufferHandle, ResourceError>>;
                      { d.DestroyBuffer(buf) } -> std::same_as<void>;
                    };
```

### Class with proper naming conventions

```cpp
/// Manages a pool of GPU memory blocks for sub-allocation.
///
/// Thread-safety: instances are not thread-safe. Each thread should use its own
/// allocator or synchronize externally.
class BlockAllocator {
 public:
  /// Configuration for a block allocator.
  struct Config {
    std::uint64_t block_size_bytes = 64 * 1024 * 1024;
    std::uint32_t max_blocks = 16;
    HeapType heap_type = HeapType::kDeviceLocal;
  };

  explicit BlockAllocator(Config config);

  BlockAllocator(const BlockAllocator&) = delete;
  auto operator=(const BlockAllocator&) -> BlockAllocator& = delete;

  BlockAllocator(BlockAllocator&& other) noexcept;
  auto operator=(BlockAllocator&& other) noexcept -> BlockAllocator&;

  ~BlockAllocator();

  /// Allocate a region of the requested size and alignment.
  [[nodiscard]] auto Allocate(std::uint64_t size_bytes, std::uint64_t alignment)
      -> std::expected<AllocationHandle, ResourceError>;

  /// Free a previously allocated region.
  auto Free(AllocationHandle handle) -> void;

  /// Returns the total number of bytes currently allocated.
  [[nodiscard]] auto AllocatedBytes() const -> std::uint64_t;

 private:
  Config config_;
  std::uint64_t allocated_bytes_ = 0;
  std::vector<Block> blocks_;
};
```

### Enum class with error handling via `std::expected`

```cpp
/// Errors that can occur during pipeline creation.
enum class PipelineError : std::uint8_t {
  kCompilationFailed,
  kUnsupported,
  kInvalidState,
};

/// Create a compute pipeline from the given descriptor.
[[nodiscard]] auto CreateComputePipeline(const ComputePipelineDesc& desc)
    -> std::expected<PipelineHandle, PipelineError> {
  if (desc.compute_shader.data == nullptr) {
    return std::unexpected(PipelineError::kInvalidState);
  }

  auto handle = CompileShader(desc);
  if (!handle) {
    return std::unexpected(PipelineError::kCompilationFailed);
  }

  return *handle;
}
```

### Struct with designated initializer usage

```cpp
/// Describes a 2D texture to be created on the GPU.
struct TextureDesc {
  std::string_view name;
  TextureDimension dimension = TextureDimension::kTex2d;
  Format format = Format::kRgba8Unorm;
  std::uint32_t width = 1;
  std::uint32_t height = 1;
  std::uint32_t depth_or_layers = 1;
  std::uint32_t mip_levels = 1;
  SampleCount samples = SampleCount::kX1;
  TextureUsageFlags usage = {};
};

// Usage with designated initializers:
auto desc = TextureDesc{
    .name = "gbuffer_albedo",
    .format = Format::kRgba8Srgb,
    .width = 1920,
    .height = 1080,
    .usage = TextureUsageFlagBits::kColorAttachment | TextureUsageFlagBits::kShaderRead,
};

auto result = device.CreateTexture(desc);
```

### Module file layout

```cpp
/// @file harmonius.rg.cppm
/// @brief Core types for the Harmonius render graph.

export module harmonius.rg;

import std;
import harmonius.gpu;

export namespace harmonius::rg {

// ---------------------------------------------------------------------------
// Handle types
// ---------------------------------------------------------------------------

enum class PassHandle : std::uint32_t { kInvalid = std::numeric_limits<std::uint32_t>::max() };

enum class ResourceHandle : std::uint32_t { kInvalid = std::numeric_limits<std::uint32_t>::max() };

// ---------------------------------------------------------------------------
// Classification enums
// ---------------------------------------------------------------------------

enum class PassType : std::uint8_t {
  kRasterization,
  kCompute,
  kRayTracingDispatch,
  kTransfer,
  kPresent,
};

// ---------------------------------------------------------------------------
// Shared vocabulary structs
// ---------------------------------------------------------------------------

struct ResourceBinding {
  ResourceHandle resource;
  AccessMode access;
  UsageType usage;
  std::uint32_t array_layer = 0;
  std::uint32_t mip_level = 0;
  bool is_history = false;
};

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

enum class ValidationErrorKind : std::uint8_t {
  kCycleDetected,
  kTypeMismatch,
  kUndeclaredResource,
  kQueueIncompatibility,
};

struct ValidationError {
  ValidationErrorKind kind;
  PassHandle pass;
  ResourceHandle resource;
  std::string message;
};

}  // namespace harmonius::rg
```
