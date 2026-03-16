#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ "$(basename "$FILE")" == "project.yml" ]] || exit 0
xcodegen generate 2>/dev/null
exit 0
