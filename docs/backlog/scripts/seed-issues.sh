#!/usr/bin/env bash
# Seed GitHub issues from docs/backlog/seed.json.
#
# Idempotent: an issue is created only if no existing issue (open or closed) has a
# matching `BL-NNNN: ` title prefix. Existing issues are left alone — updates flow
# through GitHub directly.
#
# Requires: gh authenticated against the repo, jq.
#
# Usage:
#   bash docs/backlog/scripts/seed-issues.sh [-n] [-r owner/repo] [-i BL-0001]
#
# Flags:
#   -n  Dry run; print the gh commands instead of executing them.
#   -r  Override the target repo (default: derived from `gh repo view`).
#   -i  Seed only one issue by ID. Repeat to seed multiple.
#
# References:
#   docs/backlog/seed.json  — issue catalog
#   docs/backlog/AGENTS.md  — issue ID and title-prefix convention

set -euo pipefail

DRY_RUN=0
REPO=""
ONLY=()
while getopts "nr:i:" opt; do
  case "$opt" in
    n) DRY_RUN=1 ;;
    r) REPO="$OPTARG" ;;
    i) ONLY+=("$OPTARG") ;;
    *) echo "usage: $0 [-n] [-r owner/repo] [-i BL-NNNN]..." >&2; exit 2 ;;
  esac
done

if [[ -z "$REPO" ]]; then
  REPO="$(gh repo view --json nameWithOwner -q .nameWithOwner)"
fi

SEED_JSON="$(cd "$(dirname "$0")"/.. && pwd)/seed.json"
if [[ ! -f "$SEED_JSON" ]]; then
  echo "seed.json not found at $SEED_JSON" >&2
  exit 1
fi

run() {
  if [[ "$DRY_RUN" -eq 1 ]]; then
    printf 'DRY: %s\n' "$*"
  else
    "$@"
  fi
}

WANTED_FILE="$(mktemp)"
trap 'rm -f "$WANTED_FILE" "${EXISTING_TITLES_FILE:-/dev/null}"' EXIT
if [[ ${#ONLY[@]} -gt 0 ]]; then
  for i in "${ONLY[@]}"; do printf '%s\n' "$i" >> "$WANTED_FILE"; done
fi

echo "Seeding issues into $REPO from $SEED_JSON"

# Pre-fetch existing issue titles so we can skip ones already created.
EXISTING_TITLES_FILE="$(mktemp)"
gh issue list --repo "$REPO" --state all --limit 1000 --json title -q '.[].title' \
  > "$EXISTING_TITLES_FILE" 2>/dev/null || true

CREATED=0
SKIPPED=0
TOTAL="$(jq '.issues | length' "$SEED_JSON")"

idx=0
while [[ "$idx" -lt "$TOTAL" ]]; do
  ID="$(jq -r ".issues[$idx].id" "$SEED_JSON")"
  TITLE="$(jq -r ".issues[$idx].title" "$SEED_JSON")"
  LABELS_CSV="$(jq -r ".issues[$idx].labels | join(\",\")" "$SEED_JSON")"

  if [[ -s "$WANTED_FILE" ]] && ! grep -qxF "$ID" "$WANTED_FILE"; then
    idx=$((idx + 1))
    continue
  fi

  if grep -qxF "$TITLE" "$EXISTING_TITLES_FILE"; then
    SKIPPED=$((SKIPPED + 1))
    idx=$((idx + 1))
    continue
  fi

  BODY_FILE="$(mktemp)"
  jq -r ".issues[$idx].body" "$SEED_JSON" > "$BODY_FILE"

  if [[ "$DRY_RUN" -eq 1 ]]; then
    printf 'DRY: gh issue create -R %s -t %q -F %s -l %q\n' \
      "$REPO" "$TITLE" "$BODY_FILE" "$LABELS_CSV"
  else
    gh issue create \
      --repo "$REPO" \
      --title "$TITLE" \
      --body-file "$BODY_FILE" \
      --label "$LABELS_CSV" >/dev/null
    printf 'created %s\n' "$TITLE"
  fi
  rm -f "$BODY_FILE"
  CREATED=$((CREATED + 1))
  idx=$((idx + 1))
done

echo "Done. Created: $CREATED  Skipped (already exist): $SKIPPED"
