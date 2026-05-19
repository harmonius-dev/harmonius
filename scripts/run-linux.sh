#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
command -v glslangValidator >/dev/null || {
  echo "glslangValidator not found on PATH" >&2
  exit 1
}
cargo run -p harmonius_app --release "$@"
