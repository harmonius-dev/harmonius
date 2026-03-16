#!/bin/bash
INPUT=$(cat)
ACTIVE=$(echo "$INPUT" | jq -r '.stop_hook_active // false')
[ "$ACTIVE" = "true" ] && exit 0
CHANGED=$(cd "$(echo "$INPUT" | jq -r '.cwd')" && \
  git diff --name-only HEAD 2>/dev/null | \
  grep -E '\.(rs|cpp|swift|h|ts|tsx)$' | head -5)
if [ -n "$CHANGED" ]; then
  cat <<EOFMSG
{"decision":"block","reason":"Source files modified: $CHANGED. Verify tests are up to date before stopping."}
EOFMSG
  exit 2
fi
exit 0
