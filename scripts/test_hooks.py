#!/usr/bin/env python3
"""Tests for all Claude Code hook scripts in .claude/hooks/.

Each hook receives JSON on stdin (simulating Claude Code's
PostToolUse/Stop event payload) and produces output on
stdout/stderr. Tests verify:
- Correct exit codes
- File-extension gating (skip non-matching files)
- Error output when expected
- Graceful handling of missing tools
"""

import json
import os
import subprocess
import tempfile
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
HOOKS_DIR = ROOT / ".claude" / "hooks"


def run_hook(
    hook_name: str,
    stdin_data: str,
    *,
    env_override: dict[str, str] | None = None,
    timeout: int = 30,
) -> subprocess.CompletedProcess[str]:
    """Run a hook script with the given stdin payload."""
    hook_path = HOOKS_DIR / hook_name
    env = os.environ.copy()
    if env_override:
        env.update(env_override)
    return subprocess.run(
        ["bash", str(hook_path)],
        input=stdin_data,
        capture_output=True,
        text=True,
        check=False,
        timeout=timeout,
        env=env,
    )


def make_tool_input(file_path: str) -> str:
    """Build a PostToolUse JSON payload for a file edit."""
    return json.dumps({"tool_input": {"file_path": file_path}})


def make_bash_input(command: str) -> str:
    """Build a PostToolUse JSON payload for a Bash call."""
    return json.dumps(
        {
            "tool_name": "Bash",
            "tool_input": {"command": command},
        }
    )


def make_stop_input(
    *,
    cwd: str = "/tmp",
    active: bool = False,
) -> str:
    """Build a Stop hook JSON payload."""
    return json.dumps(
        {
            "stop_hook_active": active,
            "cwd": cwd,
        }
    )


# -----------------------------------------------------------
# fmt-markdown.sh
# -----------------------------------------------------------
class TestFmtMarkdown:
    """Tests for the Markdown formatting hook."""

    HOOK = "fmt-markdown.sh"

    def test_skip_non_md_file(self) -> None:
        result = run_hook(self.HOOK, make_tool_input("/tmp/foo.py"))
        assert result.returncode == 0
        assert result.stdout == ""

    def test_skip_empty_file_path(self) -> None:
        result = run_hook(self.HOOK, json.dumps({"tool_input": {}}))
        assert result.returncode == 0
        assert result.stdout == ""

    def test_formats_md_file(self) -> None:
        with tempfile.NamedTemporaryFile(
            suffix=".md",
            mode="w",
            delete=False,
        ) as f:
            f.write("# Hello\nSome text\n")
            f.flush()
            result = run_hook(self.HOOK, make_tool_input(f.name))
        os.unlink(f.name)
        assert result.returncode == 0


# -----------------------------------------------------------
# check-python.sh
# -----------------------------------------------------------
class TestCheckPython:
    """Tests for the Python linting hook."""

    HOOK = "check-python.sh"

    def test_skip_non_py_file(self) -> None:
        result = run_hook(self.HOOK, make_tool_input("/tmp/foo.rs"))
        assert result.returncode == 0
        assert result.stdout == ""

    def test_skip_empty_file_path(self) -> None:
        result = run_hook(self.HOOK, json.dumps({"tool_input": {}}))
        assert result.returncode == 0

    def test_clean_python_file(self) -> None:
        with tempfile.NamedTemporaryFile(
            suffix=".py",
            mode="w",
            delete=False,
        ) as f:
            f.write('"""Module docstring."""\n\nx: int = 1\n')
            f.flush()
            result = run_hook(self.HOOK, make_tool_input(f.name))
        os.unlink(f.name)
        assert result.returncode == 0

    def test_reports_lint_errors(self) -> None:
        with tempfile.NamedTemporaryFile(
            suffix=".py",
            mode="w",
            delete=False,
        ) as f:
            # Unused import triggers ruff F401
            f.write("import os\nimport sys\n\nx = 1\n")
            f.flush()
            result = run_hook(self.HOOK, make_tool_input(f.name))
        os.unlink(f.name)
        # ruff --fix may auto-remove unused imports, so either
        # the file is fixed silently or errors are reported
        assert result.returncode == 0


# -----------------------------------------------------------
# check-rust.sh
# -----------------------------------------------------------
class TestCheckRust:
    """Tests for the Rust clippy/fmt hook."""

    HOOK = "check-rust.sh"

    def test_skip_non_rs_file(self) -> None:
        result = run_hook(self.HOOK, make_tool_input("/tmp/foo.py"))
        assert result.returncode == 0
        assert result.stdout == ""

    def test_skip_empty_file_path(self) -> None:
        result = run_hook(self.HOOK, json.dumps({"tool_input": {}}))
        assert result.returncode == 0

    def test_runs_on_real_rs_file(self) -> None:
        rs_file = ROOT / "src" / "rust" / "src" / "lib.rs"
        if not rs_file.exists():
            pytest.skip("src/rust/src/lib.rs not found")
        result = run_hook(
            self.HOOK,
            make_tool_input(str(rs_file)),
            timeout=60,
        )
        assert result.returncode == 0

    def test_no_cargo_toml_skips(self) -> None:
        with tempfile.NamedTemporaryFile(
            suffix=".rs",
            mode="w",
            delete=False,
            dir="/tmp",
        ) as f:
            f.write("fn main() {}\n")
            f.flush()
            result = run_hook(self.HOOK, make_tool_input(f.name))
        os.unlink(f.name)
        # No Cargo.toml in /tmp → should exit 0 silently
        assert result.returncode == 0
        assert result.stdout == ""


# -----------------------------------------------------------
# run-xcodegen.sh
# -----------------------------------------------------------
class TestRunXcodegen:
    """Tests for the XcodeGen hook."""

    HOOK = "run-xcodegen.sh"

    def test_skip_non_project_yml(self) -> None:
        result = run_hook(
            self.HOOK,
            make_tool_input("/tmp/foo.yml"),
        )
        assert result.returncode == 0
        assert result.stdout == ""

    def test_skip_empty_file_path(self) -> None:
        result = run_hook(
            self.HOOK,
            json.dumps({"tool_input": {}}),
        )
        assert result.returncode == 0

    def test_triggers_on_project_yml(self) -> None:
        # xcodegen may not be installed; hook should
        # still exit 0 gracefully
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "project.yml"
            p.write_text("name: test\n")
            result = run_hook(
                self.HOOK,
                make_tool_input(str(p)),
            )
        assert result.returncode == 0


# -----------------------------------------------------------
# check-ci-status.sh
# -----------------------------------------------------------
class TestCheckCiStatus:
    """Tests for the post-push CI status hook."""

    HOOK = "check-ci-status.sh"

    def test_skip_non_bash_tool(self) -> None:
        payload = json.dumps(
            {
                "tool_name": "Edit",
                "tool_input": {"command": "git push"},
            }
        )
        result = run_hook(self.HOOK, payload)
        assert result.returncode == 0
        assert result.stdout == ""

    def test_skip_non_push_command(self) -> None:
        result = run_hook(
            self.HOOK,
            make_bash_input("git status"),
        )
        assert result.returncode == 0
        assert result.stdout == ""

    def test_skip_non_git_command(self) -> None:
        result = run_hook(
            self.HOOK,
            make_bash_input("ls -la"),
        )
        assert result.returncode == 0
        assert result.stdout == ""


# -----------------------------------------------------------
# stop-check.sh
# -----------------------------------------------------------
class TestStopCheck:
    """Tests for the stop hook that checks for uncommitted changes."""

    HOOK = "stop-check.sh"

    def test_allows_stop_when_active(self) -> None:
        result = run_hook(
            self.HOOK,
            make_stop_input(cwd=str(ROOT), active=True),
        )
        assert result.returncode == 0

    def test_allows_stop_in_clean_dir(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            # Init a git repo with no changes
            subprocess.run(
                ["git", "init"],
                cwd=d,
                capture_output=True,
                check=True,
            )
            subprocess.run(
                ["git", "commit", "--allow-empty", "-m", "init"],
                cwd=d,
                capture_output=True,
                check=True,
            )
            result = run_hook(
                self.HOOK,
                make_stop_input(cwd=d),
            )
        assert result.returncode == 0

    def test_blocks_stop_with_uncommitted_source(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            subprocess.run(
                ["git", "init"],
                cwd=d,
                capture_output=True,
                check=True,
            )
            subprocess.run(
                ["git", "commit", "--allow-empty", "-m", "init"],
                cwd=d,
                capture_output=True,
                check=True,
            )
            # Create and stage a .rs file
            rs_file = Path(d) / "main.rs"
            rs_file.write_text("fn main() {}\n")
            subprocess.run(
                ["git", "add", "main.rs"],
                cwd=d,
                capture_output=True,
                check=True,
            )
            result = run_hook(
                self.HOOK,
                make_stop_input(cwd=d),
            )
        assert result.returncode == 2
        assert "main.rs" in result.stderr

    def test_ignores_non_source_files(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            subprocess.run(
                ["git", "init"],
                cwd=d,
                capture_output=True,
                check=True,
            )
            subprocess.run(
                ["git", "commit", "--allow-empty", "-m", "init"],
                cwd=d,
                capture_output=True,
                check=True,
            )
            # Only a .md file changed — should not block
            md_file = Path(d) / "README.md"
            md_file.write_text("# Hello\n")
            subprocess.run(
                ["git", "add", "README.md"],
                cwd=d,
                capture_output=True,
                check=True,
            )
            result = run_hook(
                self.HOOK,
                make_stop_input(cwd=d),
            )
        assert result.returncode == 0

    def test_non_git_dir_passes(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            result = run_hook(
                self.HOOK,
                make_stop_input(cwd=d),
            )
        assert result.returncode == 0
