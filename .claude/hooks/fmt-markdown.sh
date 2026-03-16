#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ "$FILE" == *.md ]] || exit 0
rumdl fmt "$FILE" 2>/dev/null
exit 0
