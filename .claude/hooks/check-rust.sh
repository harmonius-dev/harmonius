#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ "$FILE" == *.rs ]] || exit 0
cargo fmt 2>/dev/null
cargo clippy --fix --allow-dirty --allow-staged \
  2>/dev/null || true
REMAINING=$(cargo clippy --message-format=short \
  2>&1 | grep "^error" | head -10)
if [ -n "$REMAINING" ]; then
  echo "Clippy errors remain after auto-fix:"
  echo "$REMAINING"
fi
exit 0
