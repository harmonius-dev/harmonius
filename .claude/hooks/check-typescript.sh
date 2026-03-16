#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ -n "$FILE" ]] || exit 0
case "$FILE" in
  *.ts|*.tsx) ;;
  *) exit 0 ;;
esac
command -v biome >/dev/null 2>&1 && \
  biome check --fix "$FILE" 2>/dev/null || true
command -v tsc >/dev/null 2>&1 || exit 0
ERRORS=$(tsc --noEmit 2>&1 | grep "error TS" | head -10)
if [ -n "$ERRORS" ]; then
  echo "TypeScript type errors:"
  echo "$ERRORS"
fi
exit 0
