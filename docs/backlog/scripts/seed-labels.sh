#!/usr/bin/env bash
# Seed GitHub labels from docs/backlog/seed.json.
#
# Idempotent: existing labels are updated in place; missing labels are created.
# Requires: gh authenticated against the repo, jq.
#
# Usage:
#   bash docs/backlog/scripts/seed-labels.sh [-n] [-r owner/repo]
#
# Flags:
#   -n  Dry run; print the gh commands instead of executing them.
#   -r  Override the target repo (default: derived from `gh repo view`).
#
# References:
#   docs/backlog/labels.md  — normative taxonomy
#   docs/backlog/seed.json  — label catalog (sorted by name)

set -euo pipefail

DRY_RUN=0
REPO=""
while getopts "nr:" opt; do
  case "$opt" in
    n) DRY_RUN=1 ;;
    r) REPO="$OPTARG" ;;
    *) echo "usage: $0 [-n] [-r owner/repo]" >&2; exit 2 ;;
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

echo "Seeding labels into $REPO from $SEED_JSON"

# Existing labels written one per line so we can grep for membership without
# requiring bash 4 associative arrays.
EXISTING_FILE="$(mktemp)"
trap 'rm -f "$EXISTING_FILE"' EXIT
gh label list --repo "$REPO" --limit 200 --json name -q '.[].name' \
  > "$EXISTING_FILE" 2>/dev/null || true

CREATED=0
UPDATED=0

# Stream label rows from jq one at a time as TSV: name<TAB>color<TAB>desc.
while IFS=$'\t' read -r name color desc; do
  [[ -z "$name" ]] && continue
  if grep -qxF "$name" "$EXISTING_FILE"; then
    run gh label edit "$name" --repo "$REPO" --color "$color" --description "$desc"
    UPDATED=$((UPDATED + 1))
  else
    run gh label create "$name" --repo "$REPO" --color "$color" --description "$desc"
    CREATED=$((CREATED + 1))
  fi
done < <(jq -r '.labels[] | [.name, .color, .description] | @tsv' "$SEED_JSON")

echo "Done. Created: $CREATED  Updated: $UPDATED"
