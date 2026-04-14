#!/bin/bash
# Stop hook: verify Rust + regenerate Xcode project

if command -v cargo >/dev/null 2>&1 && [ -f "Cargo.toml" ]; then
  PKG_COUNT=$(cargo metadata --no-deps --format-version 1 2>/dev/null |
    jq '.packages | length' 2>/dev/null || echo 0)
  if [ "${PKG_COUNT:-0}" -gt 0 ]; then
    cargo fmt 2>/dev/null
    CLIPPY_OUT=$(cargo clippy \
      --fix --allow-dirty --allow-staged \
      --message-format=short 2>&1) || true
    REMAINING=$(echo "$CLIPPY_OUT" | grep "^error" | head -10)
    if [ -n "$REMAINING" ]; then
      echo "Clippy errors remain after auto-fix:"
      echo "$REMAINING"
    fi
  fi
fi

if [ -f "project.yml" ] && command -v xcodegen >/dev/null 2>&1; then
  xcodegen generate 2>/dev/null
fi

exit 0
