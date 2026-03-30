#!/bin/bash
# Run all language test suites.
# Exit on first failure.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

FAILED=0

echo "=== Rust tests ==="
if [ -f Cargo.toml ]; then
  MEMBERS=$(cargo metadata --no-deps --format-version 1 \
    2>/dev/null | jq '.packages | length')
  if [ "${MEMBERS:-0}" -gt 0 ]; then
    cargo test 2>&1 || FAILED=1
  else
    echo "  (skipped — no workspace members)"
  fi
else
  echo "  (skipped — no Cargo.toml)"
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
