const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const root_module = b.addModule("gcraft", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
    });
    root_module.addIncludePath(b.path("src/platform/include"));

    if (target.result.os.tag.isDarwin()) {
        root_module.addIncludePath(b.path("src/platform/metal"));
        root_module.addCSourceFiles(.{
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
    } else if (target.result.os.tag == .windows) {
        root_module.addIncludePath(b.path("src/platform/d3d12"));
        root_module.addCSourceFiles(.{
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
            .flags = &.{
                "-std=c++20",
                "-DUNICODE",
                "-D_UNICODE",
                "-DWIN32_LEAN_AND_MEAN",
                "-DNOMINMAX",
            },
        });
        // Link Windows SDK libraries for D3D12, DXGI, DirectStorage
        root_module.linkSystemLibrary("d3d12", .{});
        root_module.linkSystemLibrary("dxgi", .{});
        root_module.linkSystemLibrary("d3dcompiler", .{});
        root_module.linkSystemLibrary("dxcompiler", .{});
        root_module.linkSystemLibrary("dstorage", .{});
        root_module.linkSystemLibrary("kernel32", .{});
        root_module.linkSystemLibrary("user32", .{});
        root_module.linkSystemLibrary("ole32", .{});
    } else {
        root_module.addIncludePath(b.path("src/platform/vulkan"));
        root_module.addCSourceFiles(.{
            .root = b.path("src/platform/vulkan"),
            .files = &.{
                "enthrall_command.c",
                "enthrall_descriptor.c",
                "enthrall_device.c",
                "enthrall_instance.c",
                "enthrall_internal.c",
                "enthrall_pipeline.c",
                "enthrall_resource.c",
                "enthrall_sync.c",
            },
            .language = .c,
        });
    }

    const root_library = b.addLibrary(.{
        .name = "gcraft",
        .linkage = .static,
        .root_module = root_module,
    });
    b.installArtifact(root_library);

    const root_mod_tests = b.addTest(.{
        .root_module = root_library.root_module,
    });
    const run_root_mod_tests = b.addRunArtifact(root_mod_tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_root_mod_tests.step);

    // Metal integration tests (macOS/iOS only)
    if (target.result.os.tag.isDarwin()) {
        const metal_test_mod = b.createModule(.{
            .target = target,
            .optimize = optimize,
        });

        // Add include paths
        metal_test_mod.addIncludePath(b.path("src/platform/include"));
        metal_test_mod.addIncludePath(b.path("src/platform/metal"));

        // Add Metal backend source files (uses manual retain/release, not ARC)
        metal_test_mod.addCSourceFiles(.{
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
            .flags = &.{"-fno-objc-arc"},
        });

        // Add test file
        metal_test_mod.addCSourceFiles(.{
            .root = b.path("tests"),
            .files = &.{"test_metal_integration.m"},
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
        run_metal_tests.step.dependOn(b.getInstallStep());

        const test_metal_step = b.step("test-metal", "Run Metal integration tests");
        test_metal_step.dependOn(&run_metal_tests.step);
    }

    // D3D12 backend C++ tests (Windows only)
    if (target.result.os.tag == .windows) {
        const d3d12_test_exe = b.addExecutable(.{
            .name = "enthrall_d3d12_test",
            .target = target,
            .optimize = optimize,
        });

        d3d12_test_exe.addIncludePath(b.path("src/platform/include"));
        d3d12_test_exe.addIncludePath(b.path("src/platform/d3d12"));

        // Add all D3D12 source files plus the test file
        d3d12_test_exe.addCSourceFiles(.{
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
                "enthrall_test.cpp",
            },
            .flags = &.{
                "-std=c++20",
                "-DUNICODE",
                "-D_UNICODE",
                "-DWIN32_LEAN_AND_MEAN",
                "-DNOMINMAX",
            },
        });

        // Link Windows SDK libraries
        d3d12_test_exe.linkSystemLibrary("d3d12");
        d3d12_test_exe.linkSystemLibrary("dxgi");
        d3d12_test_exe.linkSystemLibrary("d3dcompiler");
        d3d12_test_exe.linkSystemLibrary("dxcompiler");
        d3d12_test_exe.linkSystemLibrary("dstorage");
        d3d12_test_exe.linkSystemLibrary("kernel32");
        d3d12_test_exe.linkSystemLibrary("user32");
        d3d12_test_exe.linkSystemLibrary("ole32");
        d3d12_test_exe.linkLibCpp();

        b.installArtifact(d3d12_test_exe);

        const run_d3d12_tests = b.addRunArtifact(d3d12_test_exe);
        const d3d12_test_step = b.step("test-d3d12", "Run D3D12 backend tests");
        d3d12_test_step.dependOn(&run_d3d12_tests.step);

        // Also make the main test step depend on D3D12 tests on Windows
        test_step.dependOn(&run_d3d12_tests.step);
    }
}
