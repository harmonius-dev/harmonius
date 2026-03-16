#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ -n "$FILE" ]] || exit 0
[[ "$(basename "$FILE")" == "CLAUDE.md" ]] || exit 0
EXPECTED="Read and follow all instructions in @AGENTS.md before proceeding with any work in this"
CONTENT=$(head -1 "$FILE" 2>/dev/null)
if [[ "$CONTENT" != "$EXPECTED"* ]]; then
  echo "CLAUDE.md must be a simple redirect to @AGENTS.md"
  echo "Expected: $EXPECTED..."
  echo "Got: $CONTENT"
fi
exit 0
