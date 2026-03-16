#!/bin/bash
# PostToolUse hook: after git push, check CI status
INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name // ""')
TOOL_INPUT=$(echo "$INPUT" | jq -r '.tool_input.command // ""')

# Only trigger on Bash calls that contain git push
[ "$TOOL" = "Bash" ] || exit 0
echo "$TOOL_INPUT" | grep -q 'git push' || exit 0

# Wait briefly for GitHub to register the runs
sleep 3

# Get latest run status
RUNS=$(gh run list --limit 4 --json \
  name,status,conclusion,headBranch \
  2>/dev/null)
[ -z "$RUNS" ] && exit 0

FAILING=$(echo "$RUNS" | jq -r \
  '[.[] | select(
    .conclusion == "failure"
  )] | length')
IN_PROGRESS=$(echo "$RUNS" | jq -r \
  '[.[] | select(
    .status == "in_progress" or
    .status == "queued"
  )] | length')

SUMMARY=""
if [ "$FAILING" -gt 0 ]; then
  NAMES=$(echo "$RUNS" | jq -r \
    '[.[] | select(.conclusion == "failure")
    ] | map(.name) | join(", ")')
  SUMMARY="FAILING: $NAMES. "
fi
if [ "$IN_PROGRESS" -gt 0 ]; then
  NAMES=$(echo "$RUNS" | jq -r \
    '[.[] | select(
      .status == "in_progress" or
      .status == "queued"
    )] | map(.name) | join(", ")')
  SUMMARY="${SUMMARY}IN PROGRESS: $NAMES."
fi

if [ "$FAILING" -gt 0 ]; then
  echo "{\"decision\":\"block\",\"reason\":\"CI checks failing after push. ${SUMMARY} Run 'gh run list' and fix failures.\"}"
  exit 2
fi

if [ -n "$SUMMARY" ]; then
  echo "$SUMMARY"
fi
exit 0
