#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ -n "$FILE" ]] || exit 0
case "$FILE" in
  *.py) ;;
  *) exit 0 ;;
esac
command -v ruff >/dev/null 2>&1 || exit 0
ruff format "$FILE" >/dev/null 2>&1 || true
LINT=$(ruff check --fix "$FILE" 2>&1) || true
REMAINING=$(echo "$LINT" | grep -E "^[^:]+:[0-9]+" | head -10)
if [ -n "$REMAINING" ]; then
  echo "Ruff lint errors:"
  echo "$REMAINING"
fi
if command -v mypy >/dev/null 2>&1; then
  MYPY_OUT=$(mypy --strict "$FILE" 2>&1) || true
  MYPY_ERRS=$(echo "$MYPY_OUT" | grep ": error:" | head -10)
  if [ -n "$MYPY_ERRS" ]; then
    echo "Mypy type errors:"
    echo "$MYPY_ERRS"
  fi
fi
exit 0
