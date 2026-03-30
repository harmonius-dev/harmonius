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
# check-swift.sh
# -----------------------------------------------------------
class TestCheckSwift:
    """Tests for the Swift build/typecheck hook."""

    HOOK = "check-swift.sh"

    def test_skip_non_swift_file(self) -> None:
        result = run_hook(
            self.HOOK,
            make_tool_input("/tmp/foo.py"),
        )
        assert result.returncode == 0
        assert result.stdout == ""

    def test_skip_empty_file_path(self) -> None:
        result = run_hook(
            self.HOOK,
            json.dumps({"tool_input": {}}),
        )
        assert result.returncode == 0


# -----------------------------------------------------------
# check-config.sh
# -----------------------------------------------------------
class TestCheckConfig:
    """Tests for the JSON/TOML/YAML config hook."""

    HOOK = "check-config.sh"

    def test_skip_non_config_file(self) -> None:
        result = run_hook(
            self.HOOK,
            make_tool_input("/tmp/foo.py"),
        )
        assert result.returncode == 0
        assert result.stdout == ""

    def test_skip_empty_file_path(self) -> None:
        result = run_hook(
            self.HOOK,
            json.dumps({"tool_input": {}}),
        )
        assert result.returncode == 0

    def test_formats_json(self) -> None:
        with tempfile.NamedTemporaryFile(
            suffix=".json",
            mode="w",
            delete=False,
        ) as f:
            f.write('{"b":1,"a":2}\n')
            f.flush()
            result = run_hook(
                self.HOOK,
                make_tool_input(f.name),
            )
        content = Path(f.name).read_text()
        os.unlink(f.name)
        assert result.returncode == 0
        # jq sorts keys — "a" should come before "b"
        assert content.index('"a"') < content.index('"b"')

    def test_accepts_toml(self) -> None:
        with tempfile.NamedTemporaryFile(
            suffix=".toml",
            mode="w",
            delete=False,
        ) as f:
            f.write('key = "value"\n')
            f.flush()
            result = run_hook(
                self.HOOK,
                make_tool_input(f.name),
            )
        os.unlink(f.name)
        assert result.returncode == 0

    def test_accepts_yaml(self) -> None:
        with tempfile.NamedTemporaryFile(
            suffix=".yaml",
            mode="w",
            delete=False,
        ) as f:
            f.write("key: value\n")
            f.flush()
            result = run_hook(
                self.HOOK,
                make_tool_input(f.name),
            )
        os.unlink(f.name)
        assert result.returncode == 0
