#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ -n "$FILE" && "$FILE" == *.rs ]] || exit 0
command -v cargo >/dev/null 2>&1 || exit 0
MANIFEST=$(dirname "$FILE")
while [ "$MANIFEST" != "/" ]; do
  [ -f "$MANIFEST/Cargo.toml" ] && break
  MANIFEST=$(dirname "$MANIFEST")
done
[ -f "$MANIFEST/Cargo.toml" ] || exit 0
cargo fmt --manifest-path "$MANIFEST/Cargo.toml" \
  2>/dev/null
CLIPPY_OUT=$(cargo clippy \
  --manifest-path "$MANIFEST/Cargo.toml" \
  --fix --allow-dirty --allow-staged \
  --message-format=short 2>&1) || true
REMAINING=$(echo "$CLIPPY_OUT" | grep "^error" | head -10)
if [ -n "$REMAINING" ]; then
  echo "Clippy errors remain after auto-fix:"
  echo "$REMAINING"
fi
exit 0
