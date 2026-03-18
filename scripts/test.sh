#!/bin/bash
# Run all language test suites.
# Exit on first failure.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

FAILED=0

echo "=== Rust tests ==="
if [ -f src/rust/Cargo.toml ]; then
  cargo test --manifest-path src/rust/Cargo.toml 2>&1 || FAILED=1
else
  echo "  (skipped — no Cargo.toml)"
fi

echo ""
echo "=== C++ tests ==="
if [ -f CMakeLists.txt ]; then
  if [ "$(uname)" = "Darwin" ]; then
    PRESET="macos-debug"
  elif [ "$(uname)" = "Linux" ]; then
    PRESET="linux-debug"
  else
    PRESET="windows-debug-d3d12"
  fi
  BUILD_DIR="build/${PRESET}"
  if [ -d "$BUILD_DIR" ]; then
    if cmake --build --preset "$PRESET" 2>/dev/null; then
      # ctest returns 8 when no tests exist
      ctest --test-dir "$BUILD_DIR" \
        --output-on-failure 2>&1
      CTEST_RC=$?
      if [ "$CTEST_RC" -ne 0 ] \
        && [ "$CTEST_RC" -ne 8 ]; then
        FAILED=1
      fi
    else
      echo "  (skipped — build failed)"
      FAILED=1
    fi
  else
    echo "  (skipped — run build.sh first)"
  fi
else
  echo "  (skipped — no CMakeLists.txt)"
fi

echo ""
echo "=== Swift tests ==="
if [ -f src/swift/Package.swift ]; then
  swift test --package-path src/swift 2>&1 || FAILED=1
else
  echo "  (skipped — no Package.swift)"
fi

echo ""
echo "=== TypeScript tests ==="
if [ -f package.json ]; then
  if command -v bun >/dev/null 2>&1; then
    # bun test exits 1 when no test files exist
    TEST_FILES=$(find . -name "*.test.ts" -o -name "*.spec.ts" \
      | head -1)
    if [ -n "$TEST_FILES" ]; then
      bun test 2>&1 || FAILED=1
    else
      echo "  (no test files found)"
    fi
  else
    echo "  (skipped — no bun)"
  fi
else
  echo "  (skipped — no package.json)"
fi

echo ""
echo "=== Python tests ==="
if [ -d scripts ] && command -v python3 >/dev/null 2>&1; then
  TEST_FILES=$(find scripts -name "test_*.py" -o -name "*_test.py" \
    | head -1)
  if [ -n "$TEST_FILES" ]; then
    python3 -m pytest scripts/ -q 2>&1 || FAILED=1
  else
    echo "  (no test files found)"
  fi
else
  echo "  (skipped — no python3)"
fi

if [ "$FAILED" -ne 0 ]; then
  echo ""
  echo "TESTS FAILED"
  exit 1
fi

echo ""
echo "ALL TESTS PASSED"
