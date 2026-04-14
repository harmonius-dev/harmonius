#!/usr/bin/env python3
"""Enforce SC-11 for integration design docs (shared-conventions.md).

Within each Mermaid ``classDiagram`` block, any ``class`` body that uses the ``<<enumeration>>``
or ``<<enum>>`` stereotype must not use ``...`` or ``/* ... */`` elision on subsequent lines until
the closing brace of that class — see docs/design/integration/shared-conventions.md (SC-11).
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
INTEGRATION_DIR = REPO_ROOT / "docs" / "design" / "integration"

SKIP_EXACT = frozenset(
    {
        "shared-conventions.md",
        "PROMPT.md",
        "PROMPT-test-cases.md",
    }
)

MERMAID_OPEN = re.compile(r"^```\s*mermaid\s*$")
MERMAID_CLOSE = re.compile(r"^```\s*$")
CLASS_START = re.compile(r"^\s*class\s+\w+\s*\{\s*$")
ENUM_STEREOTYPE = re.compile(r"<<enumeration>>|<<enum>>")
COMMENT_LINE = re.compile(r"^\s*%%")


def iter_mermaid_blocks(lines: list[str]) -> list[tuple[int, int, list[str]]]:
    """Return (start_idx, end_idx_exclusive, block_lines) for each mermaid fence."""
    blocks: list[tuple[int, int, list[str]]] = []
    i = 0
    n = len(lines)
    while i < n:
        if MERMAID_OPEN.match(lines[i]):
            i += 1
            start = i
            while i < n and not MERMAID_CLOSE.match(lines[i]):
                i += 1
            blocks.append((start, i, lines[start:i]))
            if i < n:
                i += 1
            continue
        i += 1
    return blocks


def violations_in_mermaid(
    mermaid_lines: list[str],
    file_rel: str,
    base_line_one_indexed: int,
) -> list[str]:
    if not any("classDiagram" in ln for ln in mermaid_lines):
        return []

    out: list[str] = []
    i = 0
    while i < len(mermaid_lines):
        if not CLASS_START.match(mermaid_lines[i]):
            i += 1
            continue

        depth = 1
        i += 1
        after_enum_stereo = False

        while i < len(mermaid_lines) and depth > 0:
            ln = mermaid_lines[i]
            phys = base_line_one_indexed + i

            if ENUM_STEREOTYPE.search(ln):
                after_enum_stereo = True
            elif after_enum_stereo:
                if not COMMENT_LINE.match(ln):
                    if "..." in ln:
                        out.append(
                            f"{file_rel}:{phys}: SC-11: `...` not allowed after "
                            f"<<enumeration>>/<<enum>> in classDiagram: {ln.strip()!r}"
                        )
                    if re.search(r"/\*.*\.\.\..*\*/", ln):
                        out.append(
                            f"{file_rel}:{phys}: SC-11: `/* ... */` elision not allowed after "
                            f"<<enumeration>>/<<enum>> in classDiagram: {ln.strip()!r}"
                        )

            depth += ln.count("{") - ln.count("}")
            i += 1

    return out


def lint_file(path: Path) -> list[str]:
    rel = str(path.relative_to(REPO_ROOT))
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()
    all_v: list[str] = []
    for start, _end, block in iter_mermaid_blocks(lines):
        base = start + 1
        all_v.extend(violations_in_mermaid(block, rel, base))
    return all_v


def main() -> int:
    if not INTEGRATION_DIR.is_dir():
        print(f"lint_integration_mermaid_sc11: missing {INTEGRATION_DIR}", file=sys.stderr)
        return 2

    violations: list[str] = []
    for path in sorted(INTEGRATION_DIR.glob("*.md")):
        name = path.name
        if name in SKIP_EXACT or name.endswith("-test-cases.md"):
            continue
        violations.extend(lint_file(path))

    if violations:
        print("SC-11 Mermaid enumeration lint failed:\n", file=sys.stderr)
        for v in violations:
            print(v, file=sys.stderr)
        return 1

    print("SC-11 Mermaid enumeration lint: OK", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
