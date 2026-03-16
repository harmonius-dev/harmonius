#!/bin/bash
FILE=$(cat | jq -r '.tool_input.file_path // empty')
[[ -n "$FILE" ]] || exit 0
[[ "$(basename "$FILE")" == "project.yml" ]] || exit 0
command -v xcodegen >/dev/null 2>&1 || exit 0
xcodegen generate 2>/dev/null
exit 0
