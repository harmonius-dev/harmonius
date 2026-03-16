#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
case "$FILE" in
  *.cpp|*.h|*.hpp|*.cc|*.cxx) ;;
  *) exit 0 ;;
esac
clang-format -i "$FILE" 2>/dev/null
TIDY_OUT=$(clang-tidy "$FILE" --fix-errors \
  2>&1 | grep -E "error:" | head -10)
if [ -n "$TIDY_OUT" ]; then
  echo "clang-tidy errors after auto-fix:"
  echo "$TIDY_OUT"
fi
exit 0
