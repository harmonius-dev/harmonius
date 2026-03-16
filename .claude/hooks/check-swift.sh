#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ "$FILE" == *.swift ]] || exit 0
swift-format format -i "$FILE" 2>/dev/null
swiftlint --fix "$FILE" 2>/dev/null || true
ERRORS=$(swiftc -typecheck "$FILE" 2>&1) || true
if [ -n "$ERRORS" ]; then
  echo "Swift type-check errors:"
  echo "$ERRORS" | head -20
fi
exit 0
