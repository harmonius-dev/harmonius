#!/usr/bin/env bash
# Sync the Engine Roadmap project Status field from a BL-* issue's status:* labels.
#
# Invoked by sync-project-status.yml on issue label events (labeled, unlabeled,
# opened, reopened) for issues whose title starts with "BL-".
#
# Uses the GITHUB_TOKEN injected automatically by GitHub Actions (projects:write
# permission declared in the workflow). No extra secrets are required.
#
# Environment variables injected by the workflow:
#   PROJECT_ID      — node ID of the Engine Roadmap project v2
#   STATUS_FIELD_ID — single-select field node ID for the "Status" field
#   ISSUE_NODE_ID   — GitHub global node ID of the triggering issue
#   LABELS          — JSON array of the issue's current label names
#
# Hard-coded option IDs match those emitted by seed-project.sh for project #3
# (PVT_kwHOAb6er84BYVxM). If the project is re-created, update all six
# OPTION_ID assignments below and the env block in sync-project-status.yml.
#
# Priority tie-breaking: when an issue carries multiple status:* labels the
# most-progressed one wins (done > review > blocked > in-progress > ready > triage).

set -euo pipefail

# --- label → option ID lookup table ----------------------------------------
declare -A OPTION_ID
OPTION_ID["status:triage"]="9f49c134"
OPTION_ID["status:ready"]="97cf2862"
OPTION_ID["status:in-progress"]="726d5938"
OPTION_ID["status:review"]="de0d8b7e"
OPTION_ID["status:blocked"]="622ad240"
OPTION_ID["status:done"]="b88386fc"

# Priority order for tie-breaking (index 0 = highest priority).
PRIORITY_ORDER=(
  "status:done"
  "status:review"
  "status:blocked"
  "status:in-progress"
  "status:ready"
  "status:triage"
)

# --- Resolve the highest-priority status:* label on the issue ---------------
TARGET_LABEL=""
TARGET_PRIORITY=999
MATCH_COUNT=0

while IFS= read -r LABEL; do
  if [[ -v "OPTION_ID[$LABEL]" ]]; then
    MATCH_COUNT=$((MATCH_COUNT + 1))
    for IDX in "${!PRIORITY_ORDER[@]}"; do
      if [[ "${PRIORITY_ORDER[$IDX]}" == "$LABEL" ]]; then
        if [[ "$IDX" -lt "$TARGET_PRIORITY" ]]; then
          TARGET_PRIORITY="$IDX"
          TARGET_LABEL="$LABEL"
        fi
        break
      fi
    done
  fi
done < <(printf '%s' "$LABELS" | jq -r '.[]')

if [[ "$MATCH_COUNT" -eq 0 ]]; then
  echo "INFO: no status:* label found; nothing to do"
  exit 0
fi

if [[ "$MATCH_COUNT" -gt 1 ]]; then
  echo "WARNING: $MATCH_COUNT status:* labels present;" \
    "using '$TARGET_LABEL' (highest priority)" >&2
fi

OPTION_VALUE="${OPTION_ID[$TARGET_LABEL]}"
echo "INFO: resolved label='$TARGET_LABEL' optionId='$OPTION_VALUE'"

# --- Find the project item ID for this issue --------------------------------
ITEM_JSON="$(gh api graphql \
  -f query='query($issueId: ID!) {
    node(id: $issueId) {
      ... on Issue {
        projectItems(first: 20) {
          nodes { id project { id } }
        }
      }
    }
  }' \
  -F issueId="$ISSUE_NODE_ID")"

ITEM_ID="$(printf '%s' "$ITEM_JSON" \
  | jq -r --arg pid "$PROJECT_ID" \
      '.data.node.projectItems.nodes[]
       | select(.project.id == $pid)
       | .id' \
  | head -1)"

if [[ -z "$ITEM_ID" ]]; then
  echo "INFO: issue '$ISSUE_NODE_ID' is not linked to project '$PROJECT_ID'; nothing to do"
  exit 0
fi

echo "INFO: itemId='$ITEM_ID' fieldId='$STATUS_FIELD_ID'"

# --- Apply the Status field update ------------------------------------------
gh api graphql \
  -f query='mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $optionId: String!) {
    updateProjectV2ItemFieldValue(input: {
      projectId: $projectId
      itemId: $itemId
      fieldId: $fieldId
      value: { singleSelectOptionId: $optionId }
    }) {
      projectV2Item { id }
    }
  }' \
  -F projectId="$PROJECT_ID" \
  -F itemId="$ITEM_ID" \
  -F fieldId="$STATUS_FIELD_ID" \
  -F optionId="$OPTION_VALUE" \
  > /dev/null

echo "INFO: Status updated to '$TARGET_LABEL' on item '$ITEM_ID'"
