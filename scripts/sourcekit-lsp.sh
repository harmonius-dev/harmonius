#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
triplet="arm64-osx"
vcpkg_prefix="$repo_root/build/macos/vcpkg_installed/$triplet"

export PKG_CONFIG_PATH="$repo_root/build/pkgconfig/$triplet"
export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$vcpkg_prefix/lib/pkgconfig"
export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$vcpkg_prefix/share/pkgconfig"

sourcekit_lsp="$(xcrun --find sourcekit-lsp)"
toolchain_root="$(cd "$(dirname "$sourcekit_lsp")/.." && pwd)"
plugin_api="$toolchain_root/lib/swift/pm/PluginAPI"

swift_flags=()
if [[ -d "$plugin_api" ]]; then
  swift_flags=(
    -Xswiftc
    -I
    -Xswiftc
    "$plugin_api"
  )
fi

exec "$sourcekit_lsp" "${swift_flags[@]}" "$@"
