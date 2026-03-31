#!/usr/bin/env python3
"""Fix relative links in docs/ to include domain subdirectories.

Handles two patterns:
1. docs/design/**/*.md: links like ../../features/X.md
   -> ../../features/{domain}/X.md
2. docs/architecture.md: links like features/X.md
   -> features/{domain}/X.md

Edge cases:
- cross-cutting.md stays at requirements/ root (no subdirectory)
- personas.md stays at user-stories/ root (no subdirectory)
"""

import re
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

# Domain mapping: filename (without .md) -> domain subdirectory
DOMAIN_MAP: dict[str, str] = {}

DOMAINS: dict[str, list[str]] = {
    "core-runtime": [
        "async-io",
        "entity-component-system",
        "events-and-messaging",
        "memory-management",
        "plugin-system",
        "reflection-and-type-system",
        "scene-and-transforms",
        "serialization",
        "spatial-indexing",
    ],
    "rendering": [
        "advanced-materials",
        "advanced-rendering",
        "anti-aliasing-upscaling",
        "character-rendering",
        "core-rendering",
        "environment",
        "first-person-rendering",
        "gpu-abstraction-layer",
        "gpu-abstraction",
        "lighting",
        "post-processing",
        "render-graph",
        "scene-rendering-pipeline",
        "stylized-effects",
    ],
    "geometry-world": [
        "foliage",
        "meshlet-pipeline",
        "procedural-generation",
        "sky-atmosphere",
        "terrain",
        "water",
    ],
    "physics": [
        "collision-detection",
        "constraints-and-joints",
        "destruction-and-fracture",
        "fluid-simulation",
        "rigid-body-dynamics",
        "soft-body-and-cloth",
        "spatial-queries",
        "vehicle-physics",
    ],
    "audio": [
        "audio-engine",
        "dsp-and-effects",
        "music-system",
        "spatial-audio",
        "voice-and-speech",
    ],
    "input": [
        "device-abstraction",
        "gestures",
        "haptics-and-feedback",
        "input-actions-and-mapping",
        "vr-input",
    ],
    "ai": [
        "behavior-trees",
        "crowd-simulation",
        "goap",
        "navigation",
        "perception",
        "steering-avoidance",
        "tactical-combat",
        "utility-ai",
    ],
    "networking": [
        "anti-cheat",
        "communication",
        "mmo-infrastructure",
        "non-functional",
        "prediction-and-rollback",
        "remote-procedure-calls",
        "replay-system",
        "session-management",
        "state-replication",
        "transport-layer",
    ],
    "animation": [
        "cloth-hair",
        "first-person",
        "morph",
        "motion-matching",
        "procedural",
        "skeletal",
        "state-machine",
    ],
    "ui-2d": [
        "2d-games",
        "accessibility",
        "common-widgets",
        "hud-and-game-ui",
        "ui-rendering",
        "widget-framework",
    ],
    "vfx": [
        "decals",
        "destruction",
        "effect-graph",
        "particles",
        "screen-effects",
        "weather-environmental",
    ],
    "content-pipeline": [
        "asset-database",
        "asset-import",
        "asset-processing",
        "asset-versioning",
        "content-plugins",
        "dcc-plugins",
        "hot-reload",
        "streaming-io",
    ],
    "game-framework": [
        "abilities",
        "block-voxel",
        "building-survival",
        "camera-system",
        "character-customization",
        "cinematics",
        "fog-of-war",
        "game-modes-misc",
        "gameplay-databases",
        "gameplay-primitives",
        "inventory",
        "minigames",
        "monetization",
        "npc-simulation",
        "pets-mounts",
        "progression",
        "quest-dialogue",
        "racing",
        "save-system",
        "scripting",
        "selection-system",
        "social",
        "stealth-cover",
        "traversal-interaction",
        "turn-based",
        "weapon-system",
        "world-management",
    ],
    "platform": [
        "crash-reporting",
        "filesystem",
        "os-integration",
        "platform-services",
        "sdk-integration",
        "threading-async",
        "window-display",
    ],
    "tools-editor": [
        "ai-assistant",
        "ai-cloud-backend",
        "ai-governance",
        "animation-editor",
        "asset-store",
        "cloud-build",
        "deployment",
        "documentation",
        "editor-framework",
        "editor-plugins",
        "launcher",
        "level-editor",
        "localization-editor",
        "logic-graph",
        "material-editor",
        "mod-support",
        "profiling-tools",
        "remote-editing",
        "server-infrastructure",
        "shared-cache",
        "specialized-editors",
        "version-control",
        "vr-editor",
        "world-building",
    ],
}

# Root-level files (no subdirectory needed)
ROOT_FILES = {"cross-cutting", "personas"}

# Build reverse lookup
for domain, files in DOMAINS.items():
    for f in files:
        DOMAIN_MAP[f] = domain


def fix_design_links(text: str) -> str:
    """Fix ../../(features|requirements|user-stories)/X.md links."""

    def replacer(m: re.Match[str]) -> str:
        prefix = m.group(1)  # everything before the category
        category = m.group(2)  # features|requirements|user-stories
        filename = m.group(3)  # e.g. navigation
        suffix = m.group(4)  # .md possibly with anchor
        if filename in ROOT_FILES:
            return m.group(0)  # no change
        domain = DOMAIN_MAP.get(filename)
        if domain is None:
            print(f"  WARNING: no domain for '{filename}' in link {m.group(0)}")
            return m.group(0)
        return f"{prefix}{category}/{domain}/{filename}{suffix}"

    return re.sub(
        r"(\.\./\.\./)("
        r"features|requirements|user-stories"
        r")/([a-zA-Z0-9_-]+)(\.md(?:#[^\s)]*)?)",
        replacer,
        text,
    )


def fix_architecture_links(text: str) -> str:
    """Fix (features|requirements|user-stories)/X.md links."""

    def replacer(m: re.Match[str]) -> str:
        category = m.group(1)
        filename = m.group(2)
        suffix = m.group(3)
        if filename in ROOT_FILES:
            return m.group(0)
        domain = DOMAIN_MAP.get(filename)
        if domain is None:
            print(f"  WARNING: no domain for '{filename}' in link {m.group(0)}")
            return m.group(0)
        return f"{category}/{domain}/{filename}{suffix}"

    # Match links in markdown: (features/X.md) or (requirements/X.md)
    # Use lookbehind for ( to ensure we're in a markdown link
    return re.sub(
        r"(?<=\()("
        r"features|requirements|user-stories"
        r")/([a-zA-Z0-9_-]+)(\.md(?:#[^\s)]*)?)"
        r"(?=\))",
        replacer,
        text,
    )


def process_design_files() -> int:
    """Process all .md files under docs/design/."""
    design_dir = REPO / "docs" / "design"
    count = 0
    for md_file in sorted(design_dir.rglob("*.md")):
        original = md_file.read_text()
        updated = fix_design_links(original)
        if updated != original:
            md_file.write_text(updated)
            count += 1
            print(f"  Updated: {md_file.relative_to(REPO)}")
    return count


def process_architecture() -> int:
    """Process docs/architecture.md."""
    arch_file = REPO / "docs" / "architecture.md"
    if not arch_file.exists():
        print("  docs/architecture.md not found")
        return 0
    original = arch_file.read_text()
    updated = fix_architecture_links(original)
    if updated != original:
        arch_file.write_text(updated)
        print(f"  Updated: {arch_file.relative_to(REPO)}")
        return 1
    return 0


def main() -> None:
    print("Fixing domain links in docs/design/ ...")
    design_count = process_design_files()
    print(f"  {design_count} file(s) updated in docs/design/")

    print("\nFixing domain links in docs/architecture.md ...")
    arch_count = process_architecture()
    print(f"  {arch_count} file(s) updated")

    total = design_count + arch_count
    print(f"\nTotal: {total} file(s) updated")
    if total == 0:
        print("No changes needed.")


if __name__ == "__main__":
    main()
