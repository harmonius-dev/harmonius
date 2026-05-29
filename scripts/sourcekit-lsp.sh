#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
triplet="arm64-osx"
vcpkg_prefix="$repo_root/build/macos/vcpkg_installed/$triplet"

export PKG_CONFIG_PATH="$repo_root/build/pkgconfig/$triplet"
export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$vcpkg_prefix/lib/pkgconfig"
export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$vcpkg_prefix/share/pkgconfig"

sourcekit_lsp="$(xcrun --find sourcekit-lsp)"
exec "$sourcekit_lsp" "$@"
