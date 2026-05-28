#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  cat <<'MSG'
Usage: ./scripts/setup_environment.sh [--dry-run]

Provision Harmonius development prerequisites for the current host.

Options:
  --dry-run  Print commands without installing packages or changing files.
MSG
}

dry_run=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run)
      dry_run=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

run() {
  if [[ "$dry_run" -eq 1 ]]; then
    printf '[dry-run]'
    printf ' %q' "$@"
    printf '\n'
  else
    "$@"
  fi
}

have() {
  command -v "$1" >/dev/null 2>&1
}

run_with_sudo() {
  if [[ "${EUID}" -eq 0 ]]; then
    run "$@"
  elif have sudo; then
    run sudo "$@"
  else
    echo "sudo is required to install packages as a non-root user." >&2
    exit 1
  fi
}

install_macos() {
  if ! have brew; then
    echo "Homebrew is required. Install Homebrew first: https://brew.sh/" >&2
    exit 1
  fi

  local packages=(cmake ninja xcodegen)
  for pkg in "${packages[@]}"; do
    if brew list --versions "$pkg" >/dev/null 2>&1; then
      echo "[ok] $pkg already installed"
    else
      echo "[install] $pkg"
      run brew install "$pkg"
    fi
  done
}

install_debian_linux() {
  local packages=(
    build-essential
    ca-certificates
    cmake
    git
    libvulkan-dev
    libx11-dev
    libxcb1-dev
    libxcursor-dev
    libxext-dev
    libxfixes-dev
    libxi-dev
    libxinerama-dev
    libxkbcommon-dev
    libxkbcommon-x11-dev
    libxrandr-dev
    libxrender-dev
    mesa-vulkan-drivers
    ninja-build
    pkg-config
    vulkan-tools
    vulkan-validationlayers
  )

  run_with_sudo apt-get update
  run_with_sudo apt-get install -y "${packages[@]}"
}

install_fedora_linux() {
  local packages=(
    cmake
    gcc-c++
    git
    libX11-devel
    libXcursor-devel
    libXext-devel
    libXfixes-devel
    libXi-devel
    libXinerama-devel
    libXrandr-devel
    libXrender-devel
    libxcb-devel
    libxkbcommon-devel
    libxkbcommon-x11-devel
    mesa-vulkan-drivers
    ninja-build
    pkgconf-pkg-config
    vulkan-headers
    vulkan-loader-devel
    vulkan-tools
    vulkan-validation-layers-devel
  )

  run_with_sudo dnf install -y "${packages[@]}"
}

install_arch_linux() {
  local packages=(
    base-devel
    cmake
    git
    libx11
    libxcb
    libxcursor
    libxext
    libxfixes
    libxi
    libxinerama
    libxkbcommon
    libxkbcommon-x11
    libxrandr
    libxrender
    ninja
    pkgconf
    vulkan-devel
    vulkan-tools
    vulkan-validation-layers
  )

  run_with_sudo pacman -Syu --needed --noconfirm "${packages[@]}"
}

install_linux() {
  if [[ -r /etc/os-release ]]; then
    # shellcheck disable=SC1091
    source /etc/os-release
  else
    echo "Cannot detect Linux distribution: /etc/os-release is missing." >&2
    exit 1
  fi

  case "${ID}" in
    debian|ubuntu|linuxmint|pop)
      install_debian_linux
      ;;
    fedora)
      install_fedora_linux
      ;;
    arch|endeavouros|manjaro)
      install_arch_linux
      ;;
    *)
      case "${ID_LIKE:-}" in
        *debian*) install_debian_linux ;;
        *fedora*) install_fedora_linux ;;
        *arch*) install_arch_linux ;;
        *)
          echo "Unsupported Linux distribution: ${PRETTY_NAME:-$ID}" >&2
          echo "Install CMake, Ninja, Vulkan SDK packages, and X11 dev headers." >&2
          exit 1
          ;;
      esac
      ;;
  esac
}

check_cmake_version() {
  if ! have cmake; then
    echo "cmake was not found after setup." >&2
    exit 1
  fi

  local version major
  version="$(cmake --version | awk 'NR==1 {print $3}')"
  major="${version%%.*}"
  if [[ "$major" =~ ^[0-9]+$ && "$major" -lt 4 ]]; then
    cat >&2 <<MSG
CMake ${version} is installed, but Harmonius requires CMake 4.0 or newer.
Install a newer CMake from Kitware, Homebrew, pipx, or your distro backports.
MSG
    exit 1
  fi
}

initialize_repo() {
  cd "$repo_root"

  if [[ ! -d .git ]]; then
    echo "Not in the Harmonius repository root: $repo_root" >&2
    exit 1
  fi

  run git submodule update --init --recursive

  if [[ "$(uname -s)" == "Darwin" ]]; then
    run xcodegen generate
  else
    echo "[info] Linux Vulkan/X11 prerequisites are installed."
    echo "[info] Xcode project generation is macOS-only and was skipped."
  fi
}

case "$(uname -s)" in
  Darwin)
    install_macos
    ;;
  Linux)
    install_linux
    ;;
  *)
    echo "Unsupported host OS: $(uname -s)" >&2
    exit 1
    ;;
esac

if [[ "$dry_run" -eq 0 ]]; then
  check_cmake_version
fi
initialize_repo

echo "Environment setup complete."
