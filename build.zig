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
                "enthrall_command.m",
                "enthrall_descriptor.m",
                "enthrall_device.m",
                "enthrall_instance.m",
                "enthrall_internal.m",
                "enthrall_pipeline.m",
                "enthrall_resource.m",
                "enthrall_sync.m",
            },
            .language = .objective_c,
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
        d3d12_module.addCSourceFiles(.{
            .root = b.path("src/platform/d3d12"),
            .files = &.{
                "enthrall_command.cpp",
                "enthrall_descriptor.cpp",
                "enthrall_device.cpp",
                "enthrall_instance.cpp",
                "enthrall_internal.cpp",
                "enthrall_pipeline.cpp",
                "enthrall_resource.cpp",
                "enthrall_sync.cpp",
            },
            .language = .cpp,
        });

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
                "enthrall_command.cpp",
                "enthrall_descriptor.cpp",
                "enthrall_device.cpp",
                "enthrall_instance.cpp",
                "enthrall_pipeline.cpp",
                "enthrall_resource.cpp",
                "enthrall_surface.cpp",
                "enthrall_sync.cpp",
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
    // Zig unit tests (runs tests from the Metal module on macOS)
    // =========================================================================
    if (target.result.os.tag.isDarwin()) {
        const test_module = b.addModule("test_mod", .{
            .root_source_file = b.path("src/root.zig"),
            .target = target,
            .optimize = optimize,
        });
        test_module.addIncludePath(b.path("src/platform/include"));
        test_module.addIncludePath(b.path("src/platform/metal"));
        test_module.addCSourceFiles(.{
            .root = b.path("src/platform/metal"),
            .files = &.{
                "enthrall_command.m",
                "enthrall_descriptor.m",
                "enthrall_device.m",
                "enthrall_instance.m",
                "enthrall_internal.m",
                "enthrall_pipeline.m",
                "enthrall_resource.m",
                "enthrall_sync.m",
            },
            .language = .objective_c,
        });

        test_module.linkFramework("Metal", .{});
        test_module.linkFramework("Foundation", .{});
        test_module.linkFramework("QuartzCore", .{});

        const unit_tests = b.addTest(.{ .root_module = test_module });
        const run_unit_tests = b.addRunArtifact(unit_tests);
        const test_step = b.step("test", "Run Zig unit tests");
        test_step.dependOn(&run_unit_tests.step);
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
                "enthrall_command.cpp",
                "enthrall_descriptor.cpp",
                "enthrall_device.cpp",
                "enthrall_instance.cpp",
                "enthrall_pipeline.cpp",
                "enthrall_resource.cpp",
                "enthrall_surface.cpp",
                "enthrall_sync.cpp",
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
}
