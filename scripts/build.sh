#!/bin/bash
# Build all language targets.
# Exit on first failure.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

FAILED=0

echo "=== Rust ==="
if [ -f src/rust/Cargo.toml ]; then
  cargo build --manifest-path src/rust/Cargo.toml 2>&1 || FAILED=1
else
  echo "  (skipped — no Cargo.toml)"
fi

echo ""
echo "=== C++ ==="
if [ -f CMakeLists.txt ]; then
  if [ "$(uname)" = "Darwin" ]; then
    PRESET="macos-debug"
  elif [ "$(uname)" = "Linux" ]; then
    PRESET="linux-debug"
  else
    PRESET="windows-debug-d3d12"
  fi
  cmake --preset "$PRESET" 2>&1 || FAILED=1
  cmake --build --preset "$PRESET" 2>&1 || FAILED=1
else
  echo "  (skipped — no CMakeLists.txt)"
fi

echo ""
echo "=== Swift ==="
if [ -f src/swift/Package.swift ]; then
  swift build --package-path src/swift 2>&1 || FAILED=1
else
  echo "  (skipped — no Package.swift)"
fi

echo ""
echo "=== TypeScript ==="
if [ -f tsconfig.json ]; then
  if command -v bun >/dev/null 2>&1; then
    bun run tsc --noEmit 2>&1 || FAILED=1
  elif command -v tsc >/dev/null 2>&1; then
    tsc --noEmit 2>&1 || FAILED=1
  else
    echo "  (skipped — no tsc or bun)"
  fi
else
  echo "  (skipped — no tsconfig.json)"
fi

echo ""
echo "=== Python ==="
if command -v ruff >/dev/null 2>&1; then
  ruff check scripts/ 2>&1 || FAILED=1
else
  echo "  (skipped — no ruff)"
fi
if command -v mypy >/dev/null 2>&1; then
  mypy --strict scripts/ 2>&1 || FAILED=1
else
  echo "  (skipped — no mypy)"
fi

if [ "$FAILED" -ne 0 ]; then
  echo ""
  echo "BUILD FAILED"
  exit 1
fi

echo ""
echo "BUILD OK"
