#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ -n "$FILE" && "$FILE" == *.swift ]] || exit 0
command -v swift-format >/dev/null 2>&1 && \
  swift-format format -i "$FILE" 2>/dev/null
command -v swiftlint >/dev/null 2>&1 && \
  swiftlint --fix "$FILE" 2>/dev/null || true
PKG_DIR=$(dirname "$FILE")
while [ "$PKG_DIR" != "/" ]; do
  [ -f "$PKG_DIR/Package.swift" ] && break
  PKG_DIR=$(dirname "$PKG_DIR")
done
if [ -f "$PKG_DIR/Package.swift" ]; then
  ERRORS=$(swift build --package-path "$PKG_DIR" \
    2>&1) || true
  if echo "$ERRORS" | grep -q "error:"; then
    echo "Swift build errors:"
    echo "$ERRORS" | grep "error:" | head -20
  fi
else
  command -v swiftc >/dev/null 2>&1 || exit 0
  ERRORS=$(swiftc -typecheck "$FILE" 2>&1) || true
  if [ -n "$ERRORS" ]; then
    echo "Swift type-check errors:"
    echo "$ERRORS" | head -20
  fi
fi
exit 0
