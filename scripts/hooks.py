#!/usr/bin/env python3
"""Run language-specific hooks after modifying files.

Maps file extensions to hook scripts in .claude/hooks/ and
invokes them with the same JSON stdin format that Claude Code
PostToolUse hooks expect.

Usage:
    # Single file (used by other scripts):
    from hooks import run_hook
    run_hook("path/to/file.py")

    # All tracked files in the repo:
    python3 scripts/hooks.py --all

    # All staged files (used by git pre-commit):
    python3 scripts/hooks.py --staged

    # Explicit file list:
    python3 scripts/hooks.py file1.py file2.md
"""

import subprocess
import sys
from pathlib import Path

# Map file extensions to hook scripts (relative to repo root)
HOOK_MAP: dict[str, str] = {
    ".json": ".claude/hooks/check-config.sh",
    ".md": ".claude/hooks/fmt-markdown.sh",
    ".py": ".claude/hooks/check-python.sh",
    ".rs": ".claude/hooks/check-rust.sh",
    ".swift": ".claude/hooks/check-swift.sh",
    ".toml": ".claude/hooks/check-config.sh",
    ".yaml": ".claude/hooks/check-config.sh",
    ".yml": ".claude/hooks/check-config.sh",
}


def run_hook(filepath: str | Path) -> bool:
    """Run the language-specific hook for a modified file.

    Returns True if the hook ran and produced error output.
    """
    p = Path(filepath)
    hook = HOOK_MAP.get(p.suffix)
    if not hook:
        return False
    hook_path = Path(hook)
    if not hook_path.exists():
        return False
    stdin_payload = f'{{"tool_input":{{"file_path":"{p.resolve()}"}}}}'
    result = subprocess.run(
        ["bash", str(hook_path)],
        input=stdin_payload,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.stdout.strip():
        print(result.stdout.strip())
        return True
    return False


def _git_file_list(git_args: list[str]) -> list[Path]:
    """Run a git command and return matching file paths."""
    result = subprocess.run(
        ["git", *git_args],
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        return []
    return [Path(f) for f in result.stdout.strip().split("\n") if f and Path(f).suffix in HOOK_MAP]


def get_staged_files() -> list[Path]:
    """Get list of staged files from git."""
    return _git_file_list(["diff", "--cached", "--name-only", "--diff-filter=ACMR"])


def get_all_files() -> list[Path]:
    """Get all files in the repo (tracked and untracked)."""
    # ls-files with -o --exclude-standard includes untracked
    # (non-ignored) files alongside tracked ones
    return _git_file_list(["ls-files", "--cached", "--others", "--exclude-standard"])


def main() -> None:
    """Run hooks on staged, all, or explicit file list."""
    if "--staged" in sys.argv:
        files = get_staged_files()
    elif "--all" in sys.argv:
        files = get_all_files()
    else:
        files = [Path(f) for f in sys.argv[1:] if not f.startswith("-")]

    if not files:
        return

    had_errors = False
    for f in files:
        if run_hook(f):
            had_errors = True
        # Re-stage files that hooks may have auto-fixed
        if "--staged" in sys.argv and f.exists():
            subprocess.run(
                ["git", "add", str(f)],
                check=False,
            )

    if had_errors:
        sys.exit(1)


if __name__ == "__main__":
    main()
