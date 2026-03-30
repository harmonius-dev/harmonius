#!/bin/bash
INPUT=$(cat)
read -r ACTIVE CWD <<< \
  "$(echo "$INPUT" | jq -r \
  '[.stop_hook_active // false, .cwd] | @tsv')"
[ "$ACTIVE" = "true" ] && exit 0
CHANGED=$(cd "$CWD" && \
  git diff --diff-filter=d --name-only HEAD 2>/dev/null | \
  grep -E '\.(rs|py)$' | head -5)
if [ -n "$CHANGED" ]; then
  cat >&2 <<EOFMSG
Source files modified: $CHANGED. Verify tests are up to date before stopping.
EOFMSG
  exit 2
fi
exit 0
