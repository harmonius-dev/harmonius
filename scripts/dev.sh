#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"

usage() {
  cat <<'MSG'
Usage: scripts/dev.sh <command> [args]

Commands:
  bootstrap [macos|ios|ios-simulator|linux]
  appium-bootstrap
  package-graph
  build <macos|ios|ios-simulator|linux> <debug|release>
  xcodegen
  bundle <macos|ios|ios-simulator> <debug|release>
  test-unit
  test-codegen
  test-render
  build-tests
  test-ui-macos
  test-ui-ios-simulator
  test-ui-record
  test-ui-record-macos
  test-ui-record-ios-simulator
  debug-appium-tests-prep
  test
  run <macos|ios-simulator>
  debug <macos|ios-simulator>
  archive <ios|macos>
  release-validate
  full-check
MSG
}

die() {
  echo "error: $*" >&2
  exit 1
}

have() {
  command -v "$1" >/dev/null 2>&1
}

repo_cd() {
  cd "$repo_root"
}

host_platform() {
  case "$(uname -s)" in
    Darwin) echo "macos" ;;
    Linux) echo "linux" ;;
    *) die "unsupported host $(uname -s)" ;;
  esac
}

triplet_for_platform() {
  case "$1" in
    ios) echo "arm64-ios" ;;
    ios-simulator) echo "arm64-ios-simulator" ;;
    linux) echo "x64-linux" ;;
    macos) echo "arm64-osx" ;;
    *) die "unknown platform $1" ;;
  esac
}

sdk_for_platform() {
  case "$1" in
    ios) echo "iphoneos" ;;
    ios-simulator) echo "iphonesimulator" ;;
    macos) echo "macosx" ;;
    *) die "no Apple SDK for platform $1" ;;
  esac
}

configuration_for_mode() {
  case "$1" in
    debug) echo "Debug" ;;
    release) echo "Release" ;;
    *) die "unknown build mode $1" ;;
  esac
}

swift_configuration_for_mode() {
  case "$1" in
    debug) echo "debug" ;;
    release) echo "release" ;;
    *) die "unknown build mode $1" ;;
  esac
}

destination_for_platform() {
  case "$1" in
    ios) echo "generic/platform=iOS" ;;
    ios-simulator) echo "platform=iOS Simulator,name=iPhone 17 Pro" ;;
    macos) echo "platform=macOS,arch=arm64" ;;
    *) die "no Xcode destination for platform $1" ;;
  esac
}

build_dir_for_platform() {
  echo "$repo_root/build/$1"
}

vcpkg_root() {
  echo "$repo_root/vcpkg"
}

vcpkg_binary() {
  echo "$(vcpkg_root)/vcpkg"
}

vcpkg_install_root_for_platform() {
  echo "$(build_dir_for_platform "$1")/vcpkg_installed"
}

vcpkg_prefix_for_platform() {
  local platform="$1"
  local triplet
  triplet="$(triplet_for_platform "$platform")"
  echo "$(vcpkg_install_root_for_platform "$platform")/$triplet"
}

pkgconfig_dir_for_platform() {
  local platform="$1"
  local triplet
  triplet="$(triplet_for_platform "$platform")"
  echo "$repo_root/build/pkgconfig/$triplet"
}

pkg_config_path_for_platform() {
  local platform="$1"
  local prefix
  local path
  prefix="$(vcpkg_prefix_for_platform "$platform")"
  path="$(printf '%s:%s:%s' \
    "$(pkgconfig_dir_for_platform "$platform")" \
    "$prefix/lib/pkgconfig" \
    "$prefix/share/pkgconfig")"
  if [[ "$platform" == "ios" || "$platform" == "ios-simulator" ]]; then
    path="$path:$(pkg_config_path_for_platform macos)"
  fi
  printf '%s' "$path"
}

runtime_library_path_for_platform() {
  local platform="$1"
  local prefix
  local path
  prefix="$(vcpkg_prefix_for_platform "$platform")"
  path="$(printf '%s:%s' "$prefix/lib" "$prefix/tools/shader-slang")"
  if [[ "$platform" == "ios" || "$platform" == "ios-simulator" ]]; then
    path="$path:$(runtime_library_path_for_platform macos)"
  fi
  printf '%s' "$path"
}

with_pkg_config_path() {
  local platform="$1"
  shift
  PKG_CONFIG_PATH="$(pkg_config_path_for_platform "$platform")" \
    DYLD_LIBRARY_PATH="$(runtime_library_path_for_platform "$platform")" \
    LD_LIBRARY_PATH="$(runtime_library_path_for_platform "$platform")" \
    "$@"
}

ensure_submodules() {
  repo_cd
  git submodule update --init --recursive
}

ensure_vcpkg_bootstrap() {
  local binary
  binary="$(vcpkg_binary)"
  if [[ -x "$binary" ]]; then
    return
  fi
  repo_cd
  "$(vcpkg_root)/bootstrap-vcpkg.sh"
}

install_vcpkg_triplet() {
  local platform="$1"
  local triplet
  local install_root
  triplet="$(triplet_for_platform "$platform")"
  install_root="$(vcpkg_install_root_for_platform "$platform")"
  ensure_vcpkg_bootstrap
  repo_cd
  "$(vcpkg_binary)" install \
    --overlay-triplets="$repo_root/vcpkg/triplets" \
    --overlay-triplets="$repo_root/vcpkg/triplets/community" \
    --triplet="$triplet" \
    --x-install-root="$install_root"
  fix_slang_dylib_install_name "$platform"
}

fix_slang_dylib_install_name() {
  local platform="$1"
  local prefix
  local dylib
  [[ "$(uname -s)" == "Darwin" ]] || return
  [[ "$platform" == "macos" ]] || return
  prefix="$(vcpkg_prefix_for_platform "$platform")"
  dylib="$prefix/lib/libslang-compiler.0.2026.7.1.dylib"
  [[ -f "$dylib" ]] || return
  install_name_tool -id "$prefix/lib/libslang.dylib" "$dylib" || true
  codesign --force --sign - "$dylib" >/dev/null 2>&1 || true
}

ensure_vcpkg_for_platform() {
  local platform="$1"
  install_vcpkg_triplet "$platform"
  if [[ "$platform" == "ios" || "$platform" == "ios-simulator" ]]; then
    install_vcpkg_triplet "macos"
  fi
}

write_pc_file() {
  local path="$1"
  local content="$2"
  mkdir -p "$(dirname "$path")"
  printf '%s\n' "$content" > "$path"
}

generate_pkgconfig_shims() {
  local platform="$1"
  local prefix
  local out_dir
  prefix="$(vcpkg_prefix_for_platform "$platform")"
  out_dir="$(pkgconfig_dir_for_platform "$platform")"
  mkdir -p "$out_dir"

  write_pc_file "$out_dir/cgltf.pc" "\
prefix=$prefix
includedir=\${prefix}/include

Name: cgltf
Description: Header-only glTF loader from vcpkg
Version: 1.15
Cflags: -I\${includedir}
Libs:"

  write_pc_file "$out_dir/meshoptimizer.pc" "\
prefix=$prefix
includedir=\${prefix}/include
libdir=\${prefix}/lib

Name: meshoptimizer
Description: Mesh optimization library from vcpkg
Version: 1.1.1
Cflags: -I\${includedir}
Libs: -L\${libdir} -lmeshoptimizer"

  if [[ "$platform" == "macos" || "$platform" == "linux" ]]; then
    write_pc_file "$out_dir/shader-slang.pc" "\
prefix=$prefix
includedir=\${prefix}/include
libdir=\${prefix}/lib
toolbindir=\${prefix}/tools/shader-slang

Name: shader-slang
Description: Slang shader compiler from vcpkg
Version: 2026.7.1
Cflags: -I\${includedir}
Libs: -L\${libdir} -lslang"
  fi
}

validate_pkgconfig_shims() {
  local platform="$1"
  local dependency
  for dependency in cgltf meshoptimizer shader-slang; do
    with_pkg_config_path "$platform" pkg-config --cflags --libs "$dependency" \
      >/dev/null
  done
}

ensure_pkgconfig_for_platform() {
  local platform="$1"
  generate_pkgconfig_shims "$platform"
  validate_pkgconfig_shims "$platform"
}

bootstrap_platform() {
  local platform="${1:-$(host_platform)}"
  ensure_submodules
  ensure_vcpkg_for_platform "$platform"
  ensure_pkgconfig_for_platform "$platform"
}

appium_bootstrap() {
  if ! have appium; then
    if have npm; then
      npm install -g appium
    else
      die "Appium is missing and npm is unavailable for host-tool install"
    fi
  fi
  ensure_appium_driver mac2
  ensure_appium_driver xcuitest
  appium driver list --installed
}

ensure_appium_driver() {
  local driver="$1"
  if appium driver list --installed --json | grep -q "\"$driver\""; then
    return
  fi
  appium driver install "$driver" >/dev/null
}

swift_package() {
  local platform="${HARMONIUS_SWIFTPM_PLATFORM:-macos}"
  with_pkg_config_path "$platform" swift package "$@"
}

swift_test() {
  local platform="${HARMONIUS_SWIFTPM_PLATFORM:-macos}"
  with_pkg_config_path "$platform" swift test "$@"
}

swift_build() {
  local platform="${1:-$(host_platform)}"
  local mode="${2:-debug}"
  local configuration
  configuration="$(swift_configuration_for_mode "$mode")"
  bootstrap_platform "$platform"
  with_pkg_config_path "$platform" swift build \
    --configuration "$configuration"
}

link_swiftpm_test_executable() {
  local executable
  local bundle
  executable="$(find "$repo_root/.build" \
    -path '*HarmoniusPackageTests.xctest/Contents/MacOS/HarmoniusPackageTests' \
    -type f \
    -print \
    -quit 2>/dev/null)"
  [[ -n "$executable" ]] || return
  bundle="${executable%/Contents/MacOS/HarmoniusPackageTests}"
  ln -sfn "$executable" "$repo_root/build/debug-swiftpm-tests"
  ln -sfn "$bundle" "$repo_root/build/debug-swiftpm-tests.xctest"
}

build_tests() {
  bootstrap_platform "macos"
  with_pkg_config_path "macos" swift build --build-tests
  mkdir -p "$repo_root/build"
  link_swiftpm_test_executable
}

package_graph() {
  bootstrap_platform "macos"
  swift_package resolve
  swift_package describe --type json >/dev/null
}

shader_artifact_path() {
  find "$repo_root/.build" \
    -path '*HarmoniusShaderPlugin*' \
    -name default.metallib \
    -type f \
    -print \
    -quit 2>/dev/null
}

ensure_shader_artifact() {
  local platform="${1:-macos}"
  local mode="${2:-debug}"
  swift_build "$platform" "$mode" >/dev/null
  local artifact
  artifact="$(shader_artifact_path)"
  [[ -n "$artifact" ]] || die "default.metallib was not generated"
  printf '%s\n' "$artifact"
}

run_xcodegen() {
  have xcodegen || die "xcodegen is required"
  repo_cd
  xcodegen generate
}

bundle_app() {
  local platform="$1"
  local mode="$2"
  local configuration
  local destination
  configuration="$(configuration_for_mode "$mode")"
  destination="$(destination_for_platform "$platform")"
  bootstrap_platform "$platform"
  run_xcodegen
  repo_cd
  PKG_CONFIG_PATH="$(pkg_config_path_for_platform "$platform")" xcodebuild build \
    -project Harmonius.xcodeproj \
    -scheme HarmoniusApp \
    -configuration "$configuration" \
    -destination "$destination" \
    -derivedDataPath build/xcodegen \
    -clonedSourcePackagesDirPath build/spm
  if [[ "$platform" == "macos" && "$mode" == "debug" ]]; then
    mkdir -p "$repo_root/build"
    ln -sfn "$(macos_app_path)" "$repo_root/build/debug-macos-app"
  fi
}

macos_app_path() {
  echo "$repo_root/build/xcodegen/Build/Products/Debug/HarmoniusApp.app"
}

ios_simulator_app_path() {
  echo "$repo_root/build/xcodegen/Build/Products/Debug-iphonesimulator/HarmoniusApp.app"
}

run_app() {
  local platform="$1"
  case "$platform" in
    macos)
      bundle_app macos debug
      open "$(macos_app_path)"
      ;;
    ios-simulator)
      bundle_app ios-simulator debug
      xcrun simctl boot "iPhone 17" >/dev/null 2>&1 || true
      xcrun simctl install booted "$(ios_simulator_app_path)"
      xcrun simctl launch booted dev.harmonius.App
      ;;
    *) die "run supports macos or ios-simulator" ;;
  esac
}

debug_app() {
  local platform="$1"
  case "$platform" in
    macos)
      bundle_app macos debug
      ;;
    ios-simulator)
      bundle_app ios-simulator debug
      xcrun simctl boot "iPhone 17" >/dev/null 2>&1 || true
      xcrun simctl install booted "$(ios_simulator_app_path)"
      xcrun simctl launch booted dev.harmonius.App
      ;;
    *) die "debug supports macos or ios-simulator" ;;
  esac
}

test_unit() {
  bootstrap_platform "macos"
  swift_test --filter HarmoniusUnitTests
}

test_codegen() {
  bootstrap_platform "macos"
  swift_test --filter SwiftEmitterTests
}

test_render() {
  bootstrap_platform "macos"
  HARMONIUS_METALLIB_PATH="$(ensure_shader_artifact macos debug)" \
    swift_test --filter HarmoniusRenderTests
}

wait_for_appium() {
  local url="$1"
  local attempts=60
  local i
  for ((i = 1; i <= attempts; i++)); do
    if curl --silent --fail "$url/status" >/dev/null; then
      return
    fi
    sleep 1
  done
  die "Appium server did not become ready at $url"
}

ensure_appium_server_running() {
  local url="${APPIUM_SERVER_URL:-http://127.0.0.1:4723}"
  local log_dir="$repo_root/build/logs/appium"
  if curl --silent --fail "$url/status" >/dev/null; then
    return
  fi
  mkdir -p "$log_dir"
  appium --address 127.0.0.1 --port 4723 \
    --log "$log_dir/debug.log" \
    >/dev/null 2>&1 &
  echo "$!" > "$log_dir/debug.pid"
  wait_for_appium "$url"
}

cleanup_appium_server() {
  if [[ -n "${HARMONIUS_APPIUM_PID:-}" ]]; then
    kill "$HARMONIUS_APPIUM_PID" >/dev/null 2>&1 || true
  fi
}

with_appium_server() {
  local log_name="$1"
  shift
  local url="${APPIUM_SERVER_URL:-http://127.0.0.1:4723}"
  local log_dir="$repo_root/build/logs/appium"
  HARMONIUS_APPIUM_PID=""
  mkdir -p "$log_dir"
  if [[ -z "${APPIUM_SERVER_URL:-}" ]]; then
    appium --address 127.0.0.1 --port 4723 \
      --log "$log_dir/$log_name.log" &
    HARMONIUS_APPIUM_PID="$!"
    trap cleanup_appium_server EXIT
  fi
  wait_for_appium "$url"
  APPIUM_SERVER_URL="$url" "$@"
}

ios_runtime_for_device() {
  local device_name="$1"
  xcrun simctl list devices --json \
    | jq -r --arg name "$device_name" '
      .devices
      | to_entries[]
      | select(any(.value[]; .name == $name and .state == "Booted"))
      | .key
    ' \
    | head -n 1
}

macos_display_scale() {
  system_profiler SPDisplaysDataType -json \
    | jq -r '
      [
        .SPDisplaysDataType[].spdisplays_ndrvs[]?
        | select(.spdisplays_main == "spdisplays_yes")
      ][0]
      | [._spdisplays_pixels, ._spdisplays_resolution]
      | @tsv
    ' \
    | awk -F '\t' '
      {
        split($1, pixels, / x /)
        split($2, resolution, / x | @ /)
        printf "%d\n", pixels[1] / resolution[1]
      }
    '
}

test_appium_platform() {
  local platform="$1"
  local appium_automation_name
  local app_path
  local display_scale=""
  local device_name=""
  local ios_runtime=""
  case "$platform" in
    macos)
      appium_automation_name="mac2"
      bundle_app macos debug
      pkill -x HarmoniusApp >/dev/null 2>&1 || true
      display_scale="$(macos_display_scale)"
      app_path="$(macos_app_path)"
      ;;
    ios-simulator)
      appium_automation_name="XCUITest"
      device_name="${HARMONIUS_APPIUM_DEVICE:-iPhone 17}"
      bundle_app ios-simulator debug
      xcrun simctl boot "$device_name" >/dev/null 2>&1 || true
      xcrun simctl bootstatus "$device_name" -b
      ios_runtime="${HARMONIUS_APPIUM_IOS_RUNTIME:-}"
      [[ -n "$ios_runtime" ]] || ios_runtime="$(ios_runtime_for_device "$device_name")"
      app_path="$(ios_simulator_app_path)"
      ;;
    *) die "unsupported Appium platform $platform" ;;
  esac

  if [[ -n "$display_scale" ]]; then
    HARMONIUS_APPIUM_APP="$app_path" \
      HARMONIUS_APPIUM_AUTOMATION_NAME="$appium_automation_name" \
      HARMONIUS_APPIUM_DEVICE="$device_name" \
      HARMONIUS_APPIUM_DISPLAY_SCALE="$display_scale" \
      HARMONIUS_APPIUM_IOS_RUNTIME="$ios_runtime" \
      HARMONIUS_APPIUM_PLATFORM="$platform" \
      swift_test --filter HarmoniusAppiumTests
  else
    HARMONIUS_APPIUM_APP="$app_path" \
      HARMONIUS_APPIUM_AUTOMATION_NAME="$appium_automation_name" \
      HARMONIUS_APPIUM_DEVICE="$device_name" \
      HARMONIUS_APPIUM_IOS_RUNTIME="$ios_runtime" \
      HARMONIUS_APPIUM_PLATFORM="$platform" \
      swift_test --filter HarmoniusAppiumTests
  fi
}

test_ui_macos() {
  appium_bootstrap
  with_appium_server macos test_appium_platform macos
}

test_ui_ios_simulator() {
  appium_bootstrap
  with_appium_server ios-simulator test_appium_platform ios-simulator
}

test_ui_record_macos() {
  HARMONIUS_RECORD_UI_SNAPSHOTS=1 test_ui_macos
}

test_ui_record_ios_simulator() {
  HARMONIUS_RECORD_UI_SNAPSHOTS=1 test_ui_ios_simulator
}

test_ui_record() {
  test_ui_record_macos
  test_ui_record_ios_simulator
}

debug_appium_tests_prep() {
  appium_bootstrap
  bundle_app macos debug
  build_tests
  ensure_appium_server_running
}

test_all() {
  test_unit
  test_codegen
  test_render
  if [[ "$(uname -s)" == "Darwin" ]]; then
    test_ui_macos
    test_ui_ios_simulator
  fi
}

archive_app() {
  local platform="$1"
  case "$platform" in
    ios)
      bootstrap_platform ios
      run_xcodegen
      PKG_CONFIG_PATH="$(pkg_config_path_for_platform ios)" xcodebuild archive \
        -project Harmonius.xcodeproj \
        -scheme HarmoniusApp \
        -destination 'generic/platform=iOS' \
        -archivePath build/ios/Harmonius.xcarchive \
        -clonedSourcePackagesDirPath build/spm
      ;;
    macos)
      bootstrap_platform macos
      run_xcodegen
      PKG_CONFIG_PATH="$(pkg_config_path_for_platform macos)" xcodebuild archive \
        -project Harmonius.xcodeproj \
        -scheme HarmoniusApp \
        -destination 'generic/platform=macOS' \
        -archivePath build/macos/Harmonius.xcarchive \
        -clonedSourcePackagesDirPath build/spm
      ;;
    *) die "archive supports ios or macos" ;;
  esac
}

release_validate() {
  local required=(
    APPLE_TEAM_ID
    ASC_ISSUER_ID
    ASC_KEY_ID
    ASC_KEY_P8_BASE64
  )
  local name
  for name in "${required[@]}"; do
    [[ -n "${!name:-}" ]] || die "missing release environment $name"
  done
  package_graph
}

format_swift() {
  have swift-format || die "swift-format is required"
  repo_cd
  git ls-files --cached --others --exclude-standard -z '*.swift' \
    | xargs -0 swift-format format --in-place
}

lint_swift() {
  have swift-format || die "swift-format is required"
  repo_cd
  git ls-files --cached --others --exclude-standard -z '*.swift' \
    | xargs -0 swift-format lint --strict
}

lint_no_js() {
  repo_cd
  local matches
  matches="$(find . \
    -path './.build' -prune -o \
    -path './.git' -prune -o \
    -path './build' -prune -o \
    -path './vcpkg' -prune -o \
    -type f \( \
      -name '*.cjs' -o \
      -name '*.js' -o \
      -name '*.mjs' -o \
      -name '*.ts' -o \
      -name 'jest.config.*' -o \
      -name 'mocha.*' -o \
      -name 'playwright.config.*' \
    \) -print)"
  [[ -z "$matches" ]] || die "JavaScript or TypeScript files found: $matches"
}

lint_no_xctest_tests() {
  repo_cd
  local pattern
  local matches
  pattern='import XCTest|XCTestCase|XCTAssert|XCTSkip|import XCUITest'
  pattern+='|XCUIApplication|XCUIElement'
  matches="$(find Tests -type f -name '*.swift' -print0 \
    | xargs -0 grep -nE "$pattern" || true)"
  [[ -z "$matches" ]] || die "XCTest/XCUITest usage found: $matches"
}

lint_line_length() {
  repo_cd
  {
    git diff --name-only -z --diff-filter=ACMR
    git ls-files --others --exclude-standard -z
  } | while IFS= read -r -d '' file; do
    [[ -f "$file" ]] || continue
    case "$file" in
      *.icns|*.icon|*.jpg|*.jpeg|*.pdf|*.png) continue ;;
      vcpkg/*) continue ;;
    esac
    awk 'length($0) > 100 { printf "%s:%d:%d\n", FILENAME, FNR, length($0) }' \
      "$file"
  done | awk 'BEGIN { failed = 0 } { failed = 1; print } END { exit failed }'
}

lint_json_sort() {
  have jq || die "jq is required for JSON key sorting checks"
  repo_cd
  local failed=0
  local file
  while IFS= read -r -d '' file; do
    [[ -f "$file" ]] || continue
    local temp
    temp="$(mktemp)"
    jq -S . "$file" > "$temp"
    if ! cmp -s "$file" "$temp"; then
      echo "$file is not sorted with jq -S" >&2
      failed=1
    fi
    rm -f "$temp"
  done < <(
    git diff --name-only -z --diff-filter=ACMR -- '*.json'
    git ls-files --others --exclude-standard -z '*.json'
  )
  [[ "$failed" -eq 0 ]]
}

lint_shader_sources() {
  repo_cd
  local matches
  matches="$(find Sources/HarmoniusShaders \
    -type f ! -name '*.slang' -print 2>/dev/null || true)"
  [[ -z "$matches" ]] || die "non-Slang shader source files found: $matches"
}

lint_all() {
  lint_no_js
  lint_no_xctest_tests
  lint_shader_sources
  lint_line_length
  lint_json_sort
  lint_swift
}

full_check() {
  lint_no_js
  lint_no_xctest_tests
  lint_shader_sources
  lint_line_length
  lint_json_sort
  lint_swift
  package_graph
  swift_build macos debug
  test_unit
  test_codegen
  test_render
  git diff --check
}

main() {
  local command="${1:-}"
  [[ -n "$command" ]] || {
    usage
    exit 2
  }
  shift || true

  case "$command" in
    appium-bootstrap) appium_bootstrap "$@" ;;
    archive) archive_app "$@" ;;
    bootstrap) bootstrap_platform "${1:-$(host_platform)}" ;;
    build-tests) build_tests "$@" ;;
    bundle) bundle_app "$@" ;;
    build) swift_build "$@" ;;
    debug) debug_app "$@" ;;
    debug-appium-tests-prep) debug_appium_tests_prep "$@" ;;
    format) format_swift "$@" ;;
    full-check) full_check "$@" ;;
    help|-h|--help) usage ;;
    lint) lint_all "$@" ;;
    package-graph) package_graph "$@" ;;
    release-validate) release_validate "$@" ;;
    run) run_app "$@" ;;
    test) test_all "$@" ;;
    test-codegen) test_codegen "$@" ;;
    test-render) test_render "$@" ;;
    test-ui-ios-simulator) test_ui_ios_simulator "$@" ;;
    test-ui-macos) test_ui_macos "$@" ;;
    test-ui-record) test_ui_record "$@" ;;
    test-ui-record-ios-simulator) test_ui_record_ios_simulator "$@" ;;
    test-ui-record-macos) test_ui_record_macos "$@" ;;
    test-unit) test_unit "$@" ;;
    xcodegen) run_xcodegen "$@" ;;
    *) die "unknown command $command" ;;
  esac
}

main "$@"
