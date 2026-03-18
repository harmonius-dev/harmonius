#!/usr/bin/env python3
"""Rewrap markdown files to 100 character line length.

DEPRECATED: Use `rumdl fmt .` instead. rumdl handles prose
rewrapping natively and also enforces table line lengths.

Preserves tables, code blocks, headings, horizontal rules,
mermaid blocks, and other structural elements. Only rewraps
prose paragraphs and list items, paragraph by paragraph.
Always fills lines to 100 characters.
"""

import re
import sys
import textwrap

from hooks import run_hook


def is_table_line(line: str) -> bool:
    stripped = line.strip()
    return stripped.startswith("|") and stripped.endswith("|")


def is_heading(line: str) -> bool:
    return line.lstrip().startswith("#")


def is_hr(line: str) -> bool:
    stripped = line.strip()
    return bool(re.match(r"^[-*_]{3,}\s*$", stripped))


def is_code_fence(line: str) -> bool:
    stripped = line.strip()
    return stripped.startswith(("```", "~~~"))


def is_list_item(line: str) -> bool:
    return bool(re.match(r"^(\s*)([-*+]|\d+\.)\s", line))


def is_blockquote(line: str) -> bool:
    return line.lstrip().startswith(">")


def is_blank(line: str) -> bool:
    return line.strip() == ""


def is_html_tag(line: str) -> bool:
    stripped = line.strip()
    return stripped.startswith("<") and not stripped.startswith("<http")


def is_link_definition(line: str) -> bool:
    return bool(re.match(r"^\[.+\]:\s", line.strip()))


def wrap_text(
    text: str,
    width: int = 100,
    initial_indent: str = "",
    subsequent_indent: str = "",
) -> str:
    """Wrap a single paragraph of text to fill to width."""
    wrapper = textwrap.TextWrapper(
        width=width,
        initial_indent=initial_indent,
        subsequent_indent=subsequent_indent,
        break_long_words=False,
        break_on_hyphens=False,
    )
    return wrapper.fill(text)


def rewrap_file(filepath: str) -> None:
    """Rewrap a single markdown file in place."""
    with open(filepath, encoding="utf-8") as f:
        content = f.read()

    lines = content.split("\n")
    if lines and lines[-1] == "":
        lines = lines[:-1]
        had_trailing_newline = True
    else:
        had_trailing_newline = False

    result: list[str] = []
    i = 0
    in_code_block = False
    in_frontmatter = False
    seen_frontmatter_start = False

    while i < len(lines):
        line = lines[i]

        # Handle YAML frontmatter (--- ... ---)
        if line.strip() == "---" and not in_code_block:
            if not seen_frontmatter_start and i == 0:
                seen_frontmatter_start = True
                in_frontmatter = True
                result.append(line)
                i += 1
                continue
            if in_frontmatter:
                in_frontmatter = False
                result.append(line)
                i += 1
                continue

        if in_frontmatter:
            result.append(line)
            i += 1
            continue

        if is_code_fence(line):
            in_code_block = not in_code_block
            result.append(line)
            i += 1
            continue

        if in_code_block:
            result.append(line)
            i += 1
            continue

        if is_table_line(line):
            result.append(line)
            i += 1
            continue

        if is_heading(line):
            result.append(line)
            i += 1
            continue

        if is_hr(line):
            result.append(line)
            i += 1
            continue

        if is_blank(line):
            result.append(line)
            i += 1
            continue

        if is_html_tag(line):
            result.append(line)
            i += 1
            continue

        if is_link_definition(line):
            result.append(line)
            i += 1
            continue

        if is_blockquote(line):
            result.append(line)
            i += 1
            continue

        # Handle list items
        if is_list_item(line):
            m = re.match(r"^(\s*)([-*+]|\d+\.)\s", line)
            assert m is not None
            marker_prefix = line[: m.end()]
            cont_indent = " " * len(marker_prefix)
            full_text = line[m.end() :].strip()
            i += 1
            while i < len(lines):
                next_line = lines[i]
                if (
                    not is_blank(next_line)
                    and not is_list_item(next_line)
                    and not is_heading(next_line)
                    and not is_table_line(next_line)
                    and not is_code_fence(next_line)
                    and not is_blockquote(next_line)
                    and not is_hr(next_line)
                    and not is_html_tag(next_line)
                    and (
                        next_line.startswith(cont_indent)
                        or (next_line[0:1] == " " and not is_list_item(next_line))
                    )
                ):
                    full_text += " " + next_line.strip()
                    i += 1
                else:
                    break

            wrapped = wrap_text(
                full_text,
                width=100,
                initial_indent=marker_prefix,
                subsequent_indent=cont_indent,
            )
            result.append(wrapped)
            continue

        # Regular prose paragraph
        m = re.match(r"^(\s*)", line)
        assert m is not None
        leading = m.group(1)
        para_lines = [line.strip()]
        i += 1
        while i < len(lines):
            next_line = lines[i]
            if (
                is_blank(next_line)
                or is_heading(next_line)
                or is_table_line(next_line)
                or is_code_fence(next_line)
                or is_list_item(next_line)
                or is_blockquote(next_line)
                or is_hr(next_line)
                or is_html_tag(next_line)
                or is_link_definition(next_line)
            ):
                break
            para_lines.append(next_line.strip())
            i += 1

        full_para = " ".join(para_lines)
        wrapped = wrap_text(
            full_para,
            width=100,
            initial_indent=leading,
            subsequent_indent=leading,
        )
        result.append(wrapped)

    output = "\n".join(result)
    if had_trailing_newline:
        output += "\n"

    with open(filepath, "w", encoding="utf-8") as f:
        f.write(output)
    run_hook(filepath)


if __name__ == "__main__":
    changed = 0
    for path in sys.argv[1:]:
        try:
            with open(path) as f:
                before = f.read()
            rewrap_file(path)
            with open(path) as f:
                after = f.read()
            if before != after:
                changed += 1
        except Exception as e:
            print(f"Error processing {path}: {e}", file=sys.stderr)
    print(f"Changed {changed} of {len(sys.argv) - 1} files")
