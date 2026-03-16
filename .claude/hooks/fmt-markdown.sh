#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ -n "$FILE" && "$FILE" == *.md ]] || exit 0
command -v rumdl >/dev/null 2>&1 || exit 0
rumdl fmt "$FILE" 2>/dev/null
exit 0
