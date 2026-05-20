#!/usr/bin/env bash
# Seed the GitHub Project board defined in docs/backlog/projects.md.
#
# Requires: gh authenticated with `read:project` and `project` scopes, jq.
# Note: GitHub Projects v2 is org/user-scoped, not repo-scoped. The script
# creates one project under the authenticated user (or organization if
# `-o owner` is supplied) titled "Engine Roadmap" and links every BL-NNNN
# issue in the target repo to the project.
#
# Idempotent: re-running adds new issues but does not duplicate them.
#
# Usage:
#   bash docs/backlog/scripts/seed-project.sh [-n] [-r owner/repo] [-o owner]
#
# Flags:
#   -n  Dry run.
#   -r  Repo to read issues from (default: derived from `gh repo view`).
#   -o  Project owner (user login or organization). Default: repo owner.
#
# References:
#   docs/backlog/projects.md  — board model (columns, swimlanes, automation)

set -euo pipefail

DRY_RUN=0
REPO=""
OWNER=""
while getopts "nr:o:" opt; do
  case "$opt" in
    n) DRY_RUN=1 ;;
    r) REPO="$OPTARG" ;;
    o) OWNER="$OPTARG" ;;
    *) echo "usage: $0 [-n] [-r owner/repo] [-o owner]" >&2; exit 2 ;;
  esac
done

if [[ -z "$REPO" ]]; then
  REPO="$(gh repo view --json nameWithOwner -q .nameWithOwner)"
fi
if [[ -z "$OWNER" ]]; then
  OWNER="${REPO%/*}"
fi

run() {
  if [[ "$DRY_RUN" -eq 1 ]]; then
    printf 'DRY: %s\n' "$*"
  else
    "$@"
  fi
}

PROJECT_TITLE="Engine Roadmap"

# Find or create the project. `gh project list` may exit non-zero without auth
# or when no projects exist; tolerate either.
PROJECT_NUM=""
PROJECT_LIST_JSON="$(gh project list --owner "$OWNER" --format json 2>/dev/null || true)"
if [[ -n "$PROJECT_LIST_JSON" ]]; then
  PROJECT_NUM="$(printf '%s' "$PROJECT_LIST_JSON" \
    | jq -r --arg t "$PROJECT_TITLE" \
        '.projects // [] | map(select(.title == $t)) | first | .number // empty' \
    || true)"
fi
if [[ -z "$PROJECT_NUM" ]]; then
  echo "Creating project '$PROJECT_TITLE' under $OWNER"
  if [[ "$DRY_RUN" -eq 1 ]]; then
    echo "DRY: gh project create --owner $OWNER --title \"$PROJECT_TITLE\""
    PROJECT_NUM="<dry-run>"
  else
    PROJECT_NUM="$(gh project create --owner "$OWNER" --title "$PROJECT_TITLE" --format json | jq -r .number)"
  fi
else
  echo "Found existing project '$PROJECT_TITLE' (#$PROJECT_NUM) under $OWNER"
fi

# Add every BL-* issue in the target repo to the project.
ADDED=0
SKIPPED=0
while read -r url; do
  [[ -z "$url" ]] && continue
  if [[ "$DRY_RUN" -eq 1 ]]; then
    echo "DRY: gh project item-add $PROJECT_NUM --owner $OWNER --url $url"
    ADDED=$((ADDED + 1))
    continue
  fi
  if gh project item-add "$PROJECT_NUM" --owner "$OWNER" --url "$url" >/dev/null 2>&1; then
    ADDED=$((ADDED + 1))
  else
    SKIPPED=$((SKIPPED + 1))
  fi
done < <(
  gh issue list --repo "$REPO" --state all --limit 1000 \
    --json url,title -q '.[] | select(.title | startswith("BL-")) | .url'
)

echo "Done. Project: $OWNER #$PROJECT_NUM '$PROJECT_TITLE'"
echo "      Issues added: $ADDED  Skipped: $SKIPPED"
echo
echo "Manual follow-up (per docs/backlog/projects.md):"
echo "  - Configure columns: Triage / Ready / In Progress / Review / Blocked / Done"
echo "  - Configure swimlanes by priority (p0 / p1 / p2 / p3)"
echo "  - Wire automations as documented in projects.md"
