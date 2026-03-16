#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ -n "$FILE" ]] || exit 0
case "$FILE" in
  *.cpp|*.h|*.hpp|*.cc|*.cxx) ;;
  *) exit 0 ;;
esac
command -v clang-format >/dev/null 2>&1 && \
  clang-format -i "$FILE" 2>/dev/null
command -v clang-tidy >/dev/null 2>&1 || exit 0
TIDY_OUT=$(clang-tidy "$FILE" --fix-errors \
  2>&1 | grep -E "error:" | head -10)
if [ -n "$TIDY_OUT" ]; then
  echo "clang-tidy errors after auto-fix:"
  echo "$TIDY_OUT"
fi
exit 0
