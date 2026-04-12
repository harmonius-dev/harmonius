#!/bin/bash
command -v cargo >/dev/null 2>&1 || exit 0
cargo fmt --manifest-path "$MANIFEST" 2>/dev/null
CLIPPY_OUT=$(cargo clippy \
  --manifest-path "$MANIFEST" \
  --fix --allow-dirty --allow-staged \
  --message-format=short 2>&1) || true
REMAINING=$(echo "$CLIPPY_OUT" | grep "^error" | head -10)
if [ -n "$REMAINING" ]; then
  echo "Clippy errors remain after auto-fix:"
  echo "$REMAINING"
fi
exit 0
