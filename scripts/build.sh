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
