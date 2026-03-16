# TypeScript Coding Standard

## Scope

Tooling, editor UI, and build scripts. TypeScript is used for the Harmonius editor frontend, build
system scripts, and development tools.

## Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Interface | `PascalCase` | `AssetMetadata` |
| Type alias | `PascalCase` | `EntityId` |
| Class | `PascalCase` | `EditorPanel` |
| Function | `camelCase` | `loadAsset` |
| Variable | `camelCase` | `frameCount` |
| Constant | `SCREAMING_SNAKE` | `MAX_RETRIES` |
| Enum | `PascalCase` | `RenderMode` |
| Enum member | `PascalCase` | `ForwardPlus` |
| File name | `kebab-case` | `asset-browser.ts` |

## File Organization

- One primary export per file
- File name matches primary export (kebab-case)
- Group by feature directory
- Barrel exports via `index.ts` at directory root
- Types in dedicated `types.ts` when shared

## Formatting Rules

- 100-character line limit
- 2-space indentation
- Biome for formatting and import sorting
- Trailing commas in multi-line expressions
- Semicolons required

## Linting Rules

- Biome for all linting
- Strict TypeScript mode (`strict: true`)
- No `any` — use `unknown` and narrow
- No `@ts-ignore` — use `@ts-expect-error` with explanation

## Type Checking

- `tsc --noEmit` must pass with zero errors
- `strictNullChecks` enabled
- `noUncheckedIndexedAccess` enabled

## How to Check and Fix

| Validation | Check command | Fix command |
|------------|--------------|-------------|
| Format | `biome check --formatter-enabled=true` | `biome check --fix` |
| Lint | `biome check --linter-enabled=true` | `biome check --fix` |
| Type-check | `tsc --noEmit` | (manual) |
| Test | `biome check` + test runner | (manual) |

## Project-Specific Rules

1. **No-code editor** — the editor is visual; users never write TypeScript
2. **Biome only** — no ESLint, no Prettier; Biome handles format + lint + imports
3. **Interfaces for contracts** — define interfaces at module boundaries

## Cache-Friendly Patterns

- **Typed arrays** — use `Float32Array`, `Uint32Array` for bulk numeric data
- **Flat data structures** — prefer flat arrays over nested objects for large datasets
- **Object pooling** — reuse objects in hot paths instead of allocating new ones
- **Avoid closures in loops** — pre-bind or use arrow functions outside the loop
- **Immutable by default** — use `readonly` properties and `ReadonlyArray<T>`
- **`const` assertions** — use `as const` for literal types and frozen objects

## Testing

1. **Test-driven development** — write tests first, driven by requirements (R-X.Y.Z) and features
   (F-X.Y.Z)
2. **Testability in design** — design modules for testability from the start
3. **Use real dependencies** — test with real modules, not stubs
4. **NO MOCKING** — no jest mocking, no sinon, no mock libraries
5. **Fakes only when necessary** — write full fakes implementing the same interface
6. **Interfaces for contracts** — define interfaces at module boundaries for substitution
7. **Enums for state** — prefer enums over string literals or booleans
8. **Pure functions** — maximize pure functions with no side effects
9. **Immutable test data** — use `Object.freeze()` or `as const` for test fixtures

## Best Practices

1. Use `const` by default; `let` only when reassignment is required; never `var`
2. Use discriminated unions over class hierarchies
3. Use `readonly` on all properties that should not change
4. Prefer `interface` over `type` for object shapes
5. Use `unknown` instead of `any` for dynamic values
6. Destructure parameters for clarity
7. Use `satisfies` operator for type-safe object literals
8. Keep functions under 40 lines
9. Export only what is needed; internal helpers stay private
10. Use `Map<K, V>` and `Set<T>` over plain objects for dynamic keys

## Anti-Patterns

1. **`any` type** — defeats the purpose of TypeScript
2. **Type assertions (`as`)** — use type guards or discriminated unions instead
3. **Mutable global state** — use module-scoped constants or dependency injection
4. **`class` for data** — use plain interfaces and factory functions
5. **Barrel re-exports of everything** — export only the public API
6. **String enums for IDs** — use branded types or opaque type aliases
7. **Nested callbacks** — use `async`/`await`
8. **Default exports** — use named exports for better refactoring
9. **Index signatures (`[key: string]: T`)** — use `Map<string, T>` or explicit keys
10. **Ignoring `Promise` returns** — always `await` or explicitly handle
