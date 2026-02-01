const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Vulkan SDK path resolution:
    // 1. Build option: -Dvulkan-sdk=/path/to/sdk
    // 2. Environment variable: VULKAN_SDK
    // 3. Fallback: empty (no SDK configured)
    const vulkan_sdk = b.option([]const u8, "vulkan-sdk", "Path to Vulkan SDK") orelse
        std.process.getEnvVarOwned(b.allocator, "VULKAN_SDK") catch "";

    // D3D12 SDK paths (Windows only):
    // For DXC (DirectX Shader Compiler), DirectStorage, and Windows SDK
    const dxc_lib_path = b.option([]const u8, "dxc-lib-path", "Path to DXC library directory") orelse "";
    const dxc_include_path = b.option([]const u8, "dxc-include-path", "Path to DXC include directory") orelse "";
    const dstorage_lib_path = b.option([]const u8, "dstorage-lib-path", "Path to DirectStorage library directory") orelse "";
    const dstorage_include_path = b.option([]const u8, "dstorage-include-path", "Path to DirectStorage include directory") orelse "";
    const winsdk_lib_path = b.option([]const u8, "winsdk-lib-path", "Path to Windows SDK um/x64 library directory") orelse "";

    // =========================================================================
    // Metal backend (macOS/iOS)
    // =========================================================================
    if (target.result.os.tag.isDarwin()) {
        const metal_module = b.addModule("gcraft_metal_mod", .{
            .root_source_file = b.path("src/root.zig"),
            .target = target,
            .optimize = optimize,
        });
        metal_module.addIncludePath(b.path("src/platform/include"));
        metal_module.addIncludePath(b.path("src/platform/metal"));
        metal_module.addCSourceFiles(.{
            .root = b.path("src/platform/metal"),
            .files = &.{
                "gcraft_command.m",
                "gcraft_descriptor.m",
                "gcraft_device.m",
                "gcraft_instance.m",
                "gcraft_internal.m",
                "gcraft_pipeline.m",
                "gcraft_resource.m",
                "gcraft_sync.m",
            },
            .flags = &.{"-fno-objc-arc"},
        });

        metal_module.linkFramework("Metal", .{});
        metal_module.linkFramework("Foundation", .{});
        metal_module.linkFramework("QuartzCore", .{});

        const metal_lib = b.addLibrary(.{
            .name = "gcraft_metal",
            .linkage = .static,
            .root_module = metal_module,
        });
        b.installArtifact(metal_lib);
    }

    // =========================================================================
    // D3D12 backend (Windows)
    // =========================================================================
    if (target.result.os.tag == .windows) {
        const d3d12_module = b.addModule("gcraft_d3d12_mod", .{
            .root_source_file = b.path("src/root.zig"),
            .target = target,
            .optimize = optimize,
        });
        d3d12_module.addIncludePath(b.path("src/platform/include"));
        d3d12_module.addIncludePath(b.path("src/platform/d3d12"));

        // Add include paths if provided (DXC, DirectStorage headers)
        if (dxc_include_path.len > 0) {
            d3d12_module.addIncludePath(.{ .cwd_relative = dxc_include_path });
        }
        if (dstorage_include_path.len > 0) {
            d3d12_module.addIncludePath(.{ .cwd_relative = dstorage_include_path });
        }

        // Add library paths if provided (Windows SDK, DXC, DirectStorage)
        if (winsdk_lib_path.len > 0) {
            d3d12_module.addLibraryPath(.{ .cwd_relative = winsdk_lib_path });
        }
        if (dxc_lib_path.len > 0) {
            d3d12_module.addLibraryPath(.{ .cwd_relative = dxc_lib_path });
        }
        if (dstorage_lib_path.len > 0) {
            d3d12_module.addLibraryPath(.{ .cwd_relative = dstorage_lib_path });
        }

        d3d12_module.addCSourceFiles(.{
            .root = b.path("src/platform/d3d12"),
            .files = &.{
                "gcraft_command.cpp",
                "gcraft_descriptor.cpp",
                "gcraft_device.cpp",
                "gcraft_instance.cpp",
                "gcraft_internal.cpp",
                "gcraft_pipeline.cpp",
                "gcraft_resource.cpp",
                "gcraft_sync.cpp",
            },
            .flags = &.{
                "-std=c++20",
                "-DUNICODE",
                "-D_UNICODE",
                "-DWIN32_LEAN_AND_MEAN",
                "-DNOMINMAX",
            },
        });

        d3d12_module.linkSystemLibrary("d3d12", .{});
        d3d12_module.linkSystemLibrary("dxgi", .{});
        d3d12_module.linkSystemLibrary("d3dcompiler", .{});
        d3d12_module.linkSystemLibrary("dxcompiler", .{});
        d3d12_module.linkSystemLibrary("dstorage", .{});
        d3d12_module.linkSystemLibrary("kernel32", .{});
        d3d12_module.linkSystemLibrary("user32", .{});
        d3d12_module.linkSystemLibrary("ole32", .{});

        const d3d12_lib = b.addLibrary(.{
            .name = "gcraft_d3d12",
            .linkage = .static,
            .root_module = d3d12_module,
        });
        b.installArtifact(d3d12_lib);
    }

    // =========================================================================
    // Vulkan backend (all platforms with Vulkan SDK)
    // =========================================================================
    // Available on: Linux (native), macOS (via MoltenVK/KosmicKrisp), Windows (optional)
    const has_vulkan = target.result.os.tag != .windows or vulkan_sdk.len > 0;
    if (has_vulkan) {
        const vulkan_module = b.addModule("gcraft_vulkan_mod", .{
            .root_source_file = b.path("src/root.zig"),
            .target = target,
            .optimize = optimize,
        });
        vulkan_module.addIncludePath(b.path("src/platform/include"));
        vulkan_module.addIncludePath(b.path("src/platform/vulkan"));

        if (vulkan_sdk.len > 0) {
            vulkan_module.addIncludePath(.{ .cwd_relative = b.fmt("{s}/include", .{vulkan_sdk}) });
            vulkan_module.addLibraryPath(.{ .cwd_relative = b.fmt("{s}/lib", .{vulkan_sdk}) });
        }
        vulkan_module.addIncludePath(.{ .cwd_relative = "/usr/local/include" });
        vulkan_module.addLibraryPath(.{ .cwd_relative = "/usr/local/lib" });

        vulkan_module.addCSourceFiles(.{
            .root = b.path("src/platform/vulkan"),
            .files = &.{
                "gcraft_command.cpp",
                "gcraft_descriptor.cpp",
                "gcraft_device.cpp",
                "gcraft_instance.cpp",
                "gcraft_pipeline.cpp",
                "gcraft_resource.cpp",
                "gcraft_surface.cpp",
                "gcraft_sync.cpp",
                "vma_impl.cpp",
            },
            .flags = &.{ "-std=c++20", "-fno-exceptions" },
        });
        vulkan_module.linkSystemLibrary("vulkan", .{});
        vulkan_module.linkSystemLibrary("c++", .{});

        const vulkan_lib = b.addLibrary(.{
            .name = "gcraft_vulkan",
            .linkage = .static,
            .root_module = vulkan_module,
        });
        b.installArtifact(vulkan_lib);
    }

    // =========================================================================
    // Zig unit tests
    // =========================================================================
    const test_mod = b.createModule(.{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
    });
    const lib_unit_tests = b.addTest(.{ .root_module = test_mod });
    const run_lib_unit_tests = b.addRunArtifact(lib_unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_lib_unit_tests.step);

    // =========================================================================
    // Metal integration tests (macOS/iOS)
    // =========================================================================
    if (target.result.os.tag.isDarwin()) {
        const metal_test_mod = b.createModule(.{
            .target = target,
            .optimize = optimize,
        });

        metal_test_mod.addIncludePath(b.path("src/platform/include"));
        metal_test_mod.addIncludePath(b.path("src/platform/metal"));

        // Add Metal backend source files (uses manual retain/release, not ARC)
        metal_test_mod.addCSourceFiles(.{
            .root = b.path("src/platform/metal"),
            .files = &.{
                "gcraft_command.m",
                "gcraft_descriptor.m",
                "gcraft_device.m",
                "gcraft_instance.m",
                "gcraft_internal.m",
                "gcraft_pipeline.m",
                "gcraft_resource.m",
                "gcraft_sync.m",
            },
            .flags = &.{"-fno-objc-arc"},
        });

        // Add test file
        metal_test_mod.addCSourceFiles(.{
            .root = b.path("src/platform/metal"),
            .files = &.{"gcraft_test.m"},
            .flags = &.{"-fno-objc-arc"},
        });

        // Link frameworks
        metal_test_mod.linkFramework("Metal", .{});
        metal_test_mod.linkFramework("Foundation", .{});
        metal_test_mod.linkFramework("QuartzCore", .{});
        if (target.result.os.tag == .macos) {
            metal_test_mod.linkFramework("AppKit", .{});
        } else {
            metal_test_mod.linkFramework("UIKit", .{});
        }
        metal_test_mod.linkSystemLibrary("c", .{});

        const metal_test_exe = b.addExecutable(.{
            .name = "test_metal_integration",
            .root_module = metal_test_mod,
        });

        b.installArtifact(metal_test_exe);

        const run_metal_tests = b.addRunArtifact(metal_test_exe);

        const test_metal_step = b.step("test-metal", "Run Metal integration tests");
        test_metal_step.dependOn(&run_metal_tests.step);
    }

    // =========================================================================
    // Vulkan tests
    // =========================================================================
    if (has_vulkan and target.result.os.tag != .windows) {
        const vulkan_test_module = b.addModule("vulkan_test_mod", .{
            .target = target,
            .optimize = optimize,
        });
        vulkan_test_module.addIncludePath(b.path("src/platform/vulkan"));
        vulkan_test_module.addIncludePath(b.path("src/platform/include"));

        if (vulkan_sdk.len > 0) {
            vulkan_test_module.addIncludePath(.{ .cwd_relative = b.fmt("{s}/include", .{vulkan_sdk}) });
            vulkan_test_module.addLibraryPath(.{ .cwd_relative = b.fmt("{s}/lib", .{vulkan_sdk}) });
        }
        vulkan_test_module.addIncludePath(.{ .cwd_relative = "/usr/local/include" });
        vulkan_test_module.addLibraryPath(.{ .cwd_relative = "/usr/local/lib" });

        vulkan_test_module.addCSourceFiles(.{
            .root = b.path("src/platform/vulkan"),
            .files = &.{
                "gcraft_command.cpp",
                "gcraft_descriptor.cpp",
                "gcraft_device.cpp",
                "gcraft_instance.cpp",
                "gcraft_pipeline.cpp",
                "gcraft_resource.cpp",
                "gcraft_surface.cpp",
                "gcraft_sync.cpp",
                "vma_impl.cpp",
                "test_vulkan.cpp",
            },
            .flags = &.{ "-std=c++20", "-fno-exceptions" },
        });
        vulkan_test_module.linkSystemLibrary("vulkan", .{});
        vulkan_test_module.linkSystemLibrary("c++", .{});

        const vulkan_test = b.addExecutable(.{
            .name = "vulkan_test",
            .root_module = vulkan_test_module,
        });
        b.installArtifact(vulkan_test);

        const run_vulkan_test = b.addRunArtifact(vulkan_test);
        run_vulkan_test.step.dependOn(b.getInstallStep());

        // Set KosmicKrisp ICD on macOS
        if (target.result.os.tag.isDarwin() and vulkan_sdk.len > 0) {
            const icd_path = b.fmt("{s}/share/vulkan/icd.d/libkosmickrisp_icd.json", .{vulkan_sdk});
            run_vulkan_test.setEnvironmentVariable("VK_ICD_FILENAMES", icd_path);
        }

        const vulkan_test_step = b.step("test-vulkan", "Run Vulkan backend tests");
        vulkan_test_step.dependOn(&run_vulkan_test.step);
    }

    // =========================================================================
    // D3D12 tests (Windows)
    // =========================================================================
    if (target.result.os.tag == .windows) {
        const d3d12_test_mod = b.createModule(.{
            .target = target,
            .optimize = optimize,
        });

        d3d12_test_mod.addIncludePath(b.path("src/platform/include"));
        d3d12_test_mod.addIncludePath(b.path("src/platform/d3d12"));

        // Add include paths if provided (DXC, DirectStorage headers)
        if (dxc_include_path.len > 0) {
            d3d12_test_mod.addIncludePath(.{ .cwd_relative = dxc_include_path });
        }
        if (dstorage_include_path.len > 0) {
            d3d12_test_mod.addIncludePath(.{ .cwd_relative = dstorage_include_path });
        }

        // Add library paths if provided (Windows SDK, DXC, DirectStorage)
        if (winsdk_lib_path.len > 0) {
            d3d12_test_mod.addLibraryPath(.{ .cwd_relative = winsdk_lib_path });
        }
        if (dxc_lib_path.len > 0) {
            d3d12_test_mod.addLibraryPath(.{ .cwd_relative = dxc_lib_path });
        }
        if (dstorage_lib_path.len > 0) {
            d3d12_test_mod.addLibraryPath(.{ .cwd_relative = dstorage_lib_path });
        }

        d3d12_test_mod.addCSourceFiles(.{
            .root = b.path("src/platform/d3d12"),
            .files = &.{
                "gcraft_command.cpp",
                "gcraft_descriptor.cpp",
                "gcraft_device.cpp",
                "gcraft_instance.cpp",
                "gcraft_internal.cpp",
                "gcraft_pipeline.cpp",
                "gcraft_resource.cpp",
                "gcraft_sync.cpp",
                "gcraft_test.cpp",
            },
            .flags = &.{
                "-std=c++20",
                "-DUNICODE",
                "-D_UNICODE",
                "-DWIN32_LEAN_AND_MEAN",
                "-DNOMINMAX",
            },
        });

        d3d12_test_mod.linkSystemLibrary("d3d12", .{});
        d3d12_test_mod.linkSystemLibrary("dxgi", .{});
        d3d12_test_mod.linkSystemLibrary("d3dcompiler", .{});
        d3d12_test_mod.linkSystemLibrary("dxcompiler", .{});
        d3d12_test_mod.linkSystemLibrary("dstorage", .{});
        d3d12_test_mod.linkSystemLibrary("kernel32", .{});
        d3d12_test_mod.linkSystemLibrary("user32", .{});
        d3d12_test_mod.linkSystemLibrary("ole32", .{});
        d3d12_test_mod.linkSystemLibrary("c++", .{});

        const d3d12_test_exe = b.addExecutable(.{
            .name = "gcraft_d3d12_test",
            .root_module = d3d12_test_mod,
        });

        b.installArtifact(d3d12_test_exe);

        const run_d3d12_tests = b.addRunArtifact(d3d12_test_exe);
        const d3d12_test_step = b.step("test-d3d12", "Run D3D12 backend tests");
        d3d12_test_step.dependOn(&run_d3d12_tests.step);
    }
}
