#!/usr/bin/env bash
set -euo pipefail

exec "$(xcrun --find lldb-dap)" "$@"
