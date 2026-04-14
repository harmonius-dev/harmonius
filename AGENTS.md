# Agents

## Cursor Cloud specific instructions

### Project state

Harmonius is a design-phase Rust game engine. The `Cargo.toml`
workspace has `members = []` (no crates yet). `cargo build` / `cargo
check` will error with "no package"; this is expected. Use `cargo
metadata --no-deps` to verify the workspace parses correctly.

### Installed tools

Commands and versions are documented in `CLAUDE.md` (see Prerequisites
and Commands tables). All tools below are pre-installed on the VM:

| Tool | Location | Notes |
|------|----------|-------|
| `rustc` / `cargo` | rustup stable | Must be latest stable (>=1.94) for `resolver = "3"` / edition 2024 |
| `rumdl` | `~/.cargo/bin` | Markdown linter; config in `.rumdl.toml` |
| `taplo` | `~/.cargo/bin` | TOML formatter |
| `jq` | system | JSON processor |
| `yq` | system | YAML processor |
| `dxc` | `/usr/local/bin` | Microsoft DXC (HLSL compiler); libs in `/usr/local/lib` |
| `spirv-tools` | system apt | SPIR-V validator (`spirv-val`) and optimizer |
| `spirv-cross` | system apt | SPIR-V to GLSL/MSL/HLSL cross-compiler |
| `glslangValidator` | system apt | GLSL/ESSL front-end and validator |
| Vulkan dev | system apt | `libvulkan-dev`, validation layers, utility libs |

### Lint / build / test

| Task | Command | Expected outcome |
|------|---------|------------------|
| Markdown lint | `rumdl check .` | Exit 1 with existing warnings (normal) |
| Markdown fix | `rumdl fmt .` | Auto-fixes most warnings |
| TOML check | `taplo check Cargo.toml` | Exit 0 |
| Cargo parse | `cargo metadata --no-deps --format-version 1` | Exit 0, 0 packages |
| HLSL to SPIR-V | `dxc -T ps_6_0 -E main -spirv file.hlsl -Fo out.spv` | Exit 0 |
| Validate SPIR-V | `spirv-val out.spv` | Exit 0 |

### Platform-specific tools

- **Metal / Metal Shader Converter**: macOS only, not applicable on
  Linux VMs.
- **Direct3D 12**: Windows only, not applicable on Linux VMs.
- **Vulkan**: fully functional via Mesa LLVMpipe (software Vulkan
  1.4.318). Both `vkcube` and `vulkaninfo` work with `DISPLAY=:1`.
  Use the LunarG SDK `vulkan-tools` (>=1.4.313); the Ubuntu default
  (1.3.275) has a BadMatch bug on X11. Force the lavapipe ICD on
  this VM:
  `VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`.

### Gotchas

- The default rustup toolchain may be pinned to an older version
  (1.83). Run `rustup default stable` if `cargo` rejects
  `resolver = "3"`.
- `dxc` requires `libdxcompiler.so` at runtime. It is installed in
  `/usr/local/lib`; ensure `ldconfig` has been run after install.
- `rumdl check .` currently reports ~228 pre-existing lint issues.
  These are in the documentation, not in agent changes.
- The LunarG Vulkan SDK apt repo (`packages.lunarg.com`) is added
  at `/etc/apt/sources.list.d/lunarg-vulkan-noble.list`. This
  provides newer `vulkan-tools` that fix the X11 BadMatch bug in
  `vulkaninfo`.
