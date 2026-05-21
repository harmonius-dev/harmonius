#!/usr/bin/env bash
# Seed the GitHub Project board defined in docs/backlog/projects.md.
#
# What this script does:
#   1. Finds or creates the 'Engine Roadmap' GitHub Project (v2) under the owner.
#   2. Links every BL-* issue from the target repo into the project.
#   3. Replaces the default Status field options with six columns
#      (Triage / Ready / In Progress / Review / Blocked / Done).
#   4. Creates a Priority field with options P0..P3 if it is not present.
#   5. Sets Status=Triage and Priority from each issue's pN label on every item.
#
# What this script does NOT do (GitHub does not expose these via the GraphQL API):
#   - Enable built-in workflow rules (Item added → Triage, Item closed → Done, etc.)
#   - Create custom views (Roadmap, Foundation, Mid-level, etc.)
#
# See docs/backlog/projects.md for the manual UI follow-up.
#
# Requires: gh authenticated with `repo` plus `project` and `read:project` scopes; jq.
#
# Usage:
#   bash docs/backlog/scripts/seed-project.sh [-n] [-r owner/repo] [-o owner]
#
# Flags:
#   -n  Dry run (does not currently dry-run the GraphQL field mutations).
#   -r  Repo to read issues from (default: derived from `gh repo view`).
#   -o  Project owner (user login or organization). Default: repo owner.

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

# 1. Find or create the project.
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

# 2. Resolve the project node ID for GraphQL calls.
PROJECT_ID=""
if [[ "$DRY_RUN" -eq 0 ]]; then
  PROJECT_ID="$(gh project view "$PROJECT_NUM" --owner "$OWNER" --format json | jq -r .id)"
fi

# 3. Add every BL-* issue in the target repo to the project.
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

if [[ "$DRY_RUN" -eq 1 ]]; then
  echo "Done. Project: $OWNER #$PROJECT_NUM '$PROJECT_TITLE'"
  echo "      Issues added: $ADDED  Skipped: $SKIPPED"
  echo
  echo "(dry-run skips field configuration and per-item field updates)"
  exit 0
fi

# 4. Configure the Status field's six column options. Idempotent: passing the
#    same option names re-creates IDs but the column model stays the same.
STATUS_FIELD_ID="$(gh api graphql -f query="{ node(id: \"$PROJECT_ID\") { ... on ProjectV2 { fields(first: 30) { nodes { ... on ProjectV2SingleSelectField { id name } } } } } }" \
  | jq -r '.data.node.fields.nodes[] | select(.name == "Status") | .id')"

if [[ -n "$STATUS_FIELD_ID" ]]; then
  echo "Configuring Status field columns"
  gh api graphql -f query='
mutation($fieldId: ID!) {
  updateProjectV2Field(input: {
    fieldId: $fieldId
    singleSelectOptions: [
      {name: "Triage",      color: GRAY,   description: "Newly filed; not yet sized or scheduled"}
      {name: "Ready",       color: GREEN,  description: "Sized; can be picked up"}
      {name: "In Progress", color: YELLOW, description: "Actively being worked"}
      {name: "Review",      color: BLUE,   description: "Submitted for review"}
      {name: "Blocked",     color: PURPLE, description: "Has unmet blocked_by"}
      {name: "Done",        color: GREEN,  description: "Acceptance criteria met"}
    ]
  }) { projectV2Field { ... on ProjectV2SingleSelectField { id } } }
}' -F fieldId="$STATUS_FIELD_ID" >/dev/null
fi

# 5. Find or create the Priority field.
PRIO_FIELD_ID="$(gh api graphql -f query="{ node(id: \"$PROJECT_ID\") { ... on ProjectV2 { fields(first: 30) { nodes { ... on ProjectV2SingleSelectField { id name } } } } } }" \
  | jq -r '.data.node.fields.nodes[] | select(.name == "Priority") | .id // empty')"
if [[ -z "$PRIO_FIELD_ID" ]]; then
  echo "Creating Priority field (P0/P1/P2/P3)"
  PRIO_FIELD_ID="$(gh project field-create "$PROJECT_NUM" \
      --owner "$OWNER" --name "Priority" --data-type "SINGLE_SELECT" \
      --single-select-options "P0,P1,P2,P3" --format json | jq -r .id)"
fi

# 6. Resolve current option IDs for both fields.
OPTIONS_JSON="$(gh api graphql -f query="{ node(id: \"$PROJECT_ID\") { ... on ProjectV2 { fields(first: 30) { nodes { ... on ProjectV2SingleSelectField { id name options { id name } } } } } }" \
  | jq -c '.data.node.fields.nodes[] | select(.name == "Status" or .name == "Priority")')"

STATUS_TRIAGE="$(printf '%s' "$OPTIONS_JSON" | jq -rs '.[] | select(.name == "Status") | .options[] | select(.name == "Triage") | .id')"
PRIO_P0="$(printf '%s' "$OPTIONS_JSON" | jq -rs '.[] | select(.name == "Priority") | .options[] | select(.name == "P0") | .id')"
PRIO_P1="$(printf '%s' "$OPTIONS_JSON" | jq -rs '.[] | select(.name == "Priority") | .options[] | select(.name == "P1") | .id')"
PRIO_P2="$(printf '%s' "$OPTIONS_JSON" | jq -rs '.[] | select(.name == "Priority") | .options[] | select(.name == "P2") | .id')"
PRIO_P3="$(printf '%s' "$OPTIONS_JSON" | jq -rs '.[] | select(.name == "Priority") | .options[] | select(.name == "P3") | .id')"

# 7. Set Status=Triage and Priority on every item from the issue's pN label.
echo "Setting Status and Priority on items"
ITEM_OK=0
ITEM_FAIL=0
TMP_ITEMS="$(mktemp)"
trap 'rm -f "$TMP_ITEMS"' EXIT

gh api graphql --paginate -f query="
query(\$cursor: String) {
  node(id: \"$PROJECT_ID\") {
    ... on ProjectV2 {
      items(first: 100, after: \$cursor) {
        pageInfo { hasNextPage endCursor }
        nodes {
          id
          content {
            ... on Issue { number labels(first: 30) { nodes { name } } }
          }
        }
      }
    }
  }
}" > "$TMP_ITEMS"

while IFS=$'\t' read -r ITEM PRIORITY; do
  case "$PRIORITY" in
    p0) PVAL="$PRIO_P0" ;;
    p1) PVAL="$PRIO_P1" ;;
    p2) PVAL="$PRIO_P2" ;;
    p3) PVAL="$PRIO_P3" ;;
    *)  PVAL="" ;;
  esac
  [[ -z "$ITEM" ]] && continue
  if gh api graphql -f query='
mutation($p:ID!,$i:ID!,$sf:ID!,$sv:String!,$pf:ID!,$pv:String!) {
  s: updateProjectV2ItemFieldValue(input:{projectId:$p,itemId:$i,fieldId:$sf,value:{singleSelectOptionId:$sv}}) { projectV2Item { id } }
  pr: updateProjectV2ItemFieldValue(input:{projectId:$p,itemId:$i,fieldId:$pf,value:{singleSelectOptionId:$pv}}) { projectV2Item { id } }
}' -F p="$PROJECT_ID" -F i="$ITEM" \
   -F sf="$STATUS_FIELD_ID" -F sv="$STATUS_TRIAGE" \
   -F pf="$PRIO_FIELD_ID"   -F pv="${PVAL:-$PRIO_P2}" >/dev/null 2>&1; then
    ITEM_OK=$((ITEM_OK + 1))
  else
    ITEM_FAIL=$((ITEM_FAIL + 1))
  fi
done < <(jq -rs '
  [.[].data.node.items.nodes[]]
  | map({
      id: .id,
      priority: ([.content.labels.nodes[].name | select(test("^p[0-3]$"))] | first // "")
    })
  | .[] | [.id, .priority] | @tsv
' "$TMP_ITEMS")

echo "Done. Project: $OWNER #$PROJECT_NUM '$PROJECT_TITLE'"
echo "      Issues added: $ADDED  Skipped: $SKIPPED"
echo "      Item field updates: $ITEM_OK ok, $ITEM_FAIL failed"
echo
echo "Manual follow-up (GitHub UI only — no GraphQL mutations exist):"
echo "  - Workflows tab: enable 'Item added to project' → Status: Triage"
echo "  - Workflows tab: enable 'Item closed' → Status: Done"
echo "  - Workflows tab: enable 'Pull request merged' → Status: Done"
echo "  - Create board view 'Roadmap': group by Status, swim-lane by Priority"
echo "  - See docs/backlog/projects.md for the full view recipe"
