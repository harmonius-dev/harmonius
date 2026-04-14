#!/usr/bin/env bash
# Link rumdl + harmonize from the workflow repo into Cursor local plugins.
# Mirrors Claude Code: enabledPlugins harmonize@cjhowe-us-workflow, rumdl@cjhowe-us-workflow
# (see .claude/settings.json).
#
# Team marketplace equivalent: import https://github.com/cjhowe-us/workflow in
# Cursor Dashboard -> Settings -> Plugins -> Team Marketplaces (when that repo
# has .cursor-plugin/marketplace.json at the root).
#
# Usage: from repo root, run: .cursor/install-workflow-plugins.sh
# Override workflow path: HARMONIUS_WORKFLOW_ROOT=/path/to/workflow .cursor/install-workflow-plugins.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && git rev-parse --show-toplevel)"
WORKFLOW_ROOT="${HARMONIUS_WORKFLOW_ROOT:-${REPO_ROOT}/../workflow}"
LOCAL_ROOT="${HOME}/.cursor/plugins/local"

if [[ ! -d "${WORKFLOW_ROOT}" ]]; then
  echo "error: workflow repo not found at ${WORKFLOW_ROOT}" >&2
  echo "Set HARMONIUS_WORKFLOW_ROOT to your cjhowe-us/workflow checkout." >&2
  exit 1
fi

mkdir -p "${LOCAL_ROOT}"

for name in rumdl harmonize; do
  src="${WORKFLOW_ROOT}/${name}"
  if [[ ! -d "${src}" ]]; then
    echo "error: missing plugin directory ${src}" >&2
    exit 1
  fi
  if [[ ! -f "${src}/.cursor-plugin/plugin.json" ]]; then
    echo "error: ${src} has no .cursor-plugin/plugin.json (Cursor will not load it)" >&2
    exit 1
  fi
  ln -sfn "${src}" "${LOCAL_ROOT}/${name}"
  echo "Linked ${LOCAL_ROOT}/${name} -> ${src}"
done

echo "Done. Reload Cursor (Developer: Reload Window) if plugins do not appear."
