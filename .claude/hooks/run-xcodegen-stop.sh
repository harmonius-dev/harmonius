#!/bin/bash
# Stop hook: regenerate Xcode project if project.yml exists
[ -f "project.yml" ] || exit 0
command -v xcodegen >/dev/null 2>&1 || exit 0
xcodegen generate 2>/dev/null
exit 0
