#!/usr/bin/env python3
"""Fix Markdown tables exceeding 100-char line length.

Moves long cell content into numbered detail lists below each table.
Detects table schema from headers and applies category-specific rules.
"""

import re
import sys
import textwrap
from pathlib import Path

from hooks import run_hook

LINE_LIMIT = 100


def parse_row(line: str) -> list[str]:
    """Parse a markdown table row into cell values."""
    line = line.strip()
    if line.startswith("|"):
        line = line[1:]
    if line.endswith("|"):
        line = line[:-1]
    return [c.strip() for c in line.split("|")]


def is_separator(line: str) -> bool:
    """Check if a line is a table separator (|---|---|)."""
    return bool(re.match(r"^\|[\s\-:|]+\|$", line.strip()))


def is_table_row(line: str) -> bool:
    """Check if a line is a table row."""
    return line.strip().startswith("|") and line.strip().endswith("|")


def normalize_header(h: str) -> str:
    """Normalize a header string for matching."""
    return h.strip().lower().replace(" ", "")


def detect_schema(headers: list[str]) -> str | None:
    """Detect table schema from header names."""
    norm = [normalize_header(h) for h in headers]

    # Features: ID | Feature | Description | Requirements |
    #           Dependencies | Platform Notes
    if len(norm) == 6 and "id" in norm[0] and "feature" in norm[1] and "description" in norm[2]:
        return "features"

    # Requirements: ID | Requirement | Derived From | Rationale |
    #               Verification
    if (
        len(norm) == 5
        and "id" in norm[0]
        and ("requirement" in norm[1] or "shall" in norm[1])
        and ("derived" in norm[2] or "from" in norm[2])
    ):
        return "requirements"

    # User Stories: ID | Persona | Story | Acceptance Criteria |
    #               Features | Requirements
    if len(norm) == 6 and "id" in norm[0] and "persona" in norm[1] and "story" in norm[2]:
        return "user-stories"

    # Design test plan: Test | Req | Description
    if len(norm) == 3 and ("test" in norm[0]) and ("req" in norm[1]) and ("description" in norm[2]):
        return "test-plan"

    # Design test cases: # | Input | Expected Output | Requirement
    if (
        len(norm) == 4
        and norm[0] in ("#", "no", "no.", "num", "number")
        and "input" in norm[1]
        and ("expected" in norm[2] or "output" in norm[2])
    ):
        return "test-cases"

    # Req-trace: Feature | Requirement | [User Story |] Description
    if (
        len(norm) >= 3
        and "feature" in norm[0]
        and "requirement" in norm[1]
        and "description" in norm[-1]
    ):
        return "req-trace"

    # Business/generic: 2+ cols with any cell that would overflow
    return None


def make_detail_item(
    num: int,
    key: str,
    main_text: str,
    sub_items: list[tuple[str, str]],
) -> list[str]:
    """Build a numbered detail list item with sub-items."""
    lines = []
    # Indent = length of "N. " prefix (3 for 1-9, 4 for 10-99, etc.)
    prefix_len = len(str(num)) + 2
    indent = " " * prefix_len
    sub_indent = indent + "  "

    header = f"{num}. **{key}**"
    if main_text:
        header += f" \u2014 {main_text}"

    # Wrap the header line
    if len(header) <= LINE_LIMIT:
        lines.append(header)
    else:
        wrapped = textwrap.wrap(
            header,
            width=LINE_LIMIT,
            subsequent_indent=indent,
            break_long_words=False,
            break_on_hyphens=False,
        )
        lines.extend(wrapped)

    for label, value in sub_items:
        if not value or value.lower() in ("none", "n/a", ""):
            continue
        sub_line = f"{indent}- **{label}:** {value}"
        if len(sub_line) <= LINE_LIMIT:
            lines.append(sub_line)
        else:
            wrapped = textwrap.wrap(
                sub_line,
                width=LINE_LIMIT,
                subsequent_indent=sub_indent,
                break_long_words=False,
                break_on_hyphens=False,
            )
            lines.extend(wrapped)

    return lines


def build_table(headers: list[str], separator_cells: list[str], rows: list[list[str]]) -> list[str]:
    """Build a markdown table from headers and rows."""
    # Calculate column widths
    widths = [len(h) for h in headers]
    for row in rows:
        for i, cell in enumerate(row):
            if i < len(widths):
                widths[i] = max(widths[i], len(cell))

    # Check if padded table would exceed line limit
    total = sum(w + 3 for w in widths) + 1  # | col |
    use_padding = total <= LINE_LIMIT

    def fmt_row(cells: list[str]) -> str:
        parts = []
        for i, cell in enumerate(cells):
            if use_padding and i < len(widths):
                parts.append(f" {cell.ljust(widths[i])} ")
            else:
                parts.append(f" {cell} ")
        return "|" + "|".join(parts) + "|"

    def fmt_sep() -> str:
        parts = []
        for i in range(len(headers)):
            w = widths[i] if use_padding else max(3, len(headers[i]))
            parts.append("-" * (w + 2))
        return "|" + "|".join(parts) + "|"

    lines = [fmt_row(headers), fmt_sep()]
    for row in rows:
        # Pad row to match header count
        padded = row + [""] * (len(headers) - len(row))
        lines.append(fmt_row(padded[: len(headers)]))
    return lines


def transform_features(
    headers: list[str],
    sep: list[str],
    rows: list[list[str]],
) -> tuple[list[str], list[str]]:
    """Transform a features table."""
    keep_cols = [0, 1, 3]  # ID, Feature, Requirements
    new_headers = [headers[i] for i in keep_cols]
    new_sep = [sep[i] for i in keep_cols if i < len(sep)]
    new_rows = []
    detail_lines = []

    for num, row in enumerate(rows, 1):
        new_rows.append([row[i] if i < len(row) else "" for i in keep_cols])
        row_id = row[0] if row else ""
        desc = row[2] if len(row) > 2 else ""
        deps = row[4] if len(row) > 4 else ""
        platform = row[5] if len(row) > 5 else ""

        sub_items = []
        if deps and deps.lower() not in ("none", ""):
            sub_items.append(("Deps", deps))
        if platform and platform.lower() not in ("none", ""):
            sub_items.append(("Platform", platform))

        detail_lines.extend(make_detail_item(num, row_id, desc, sub_items))

    table_lines = build_table(new_headers, new_sep, new_rows)
    return table_lines, detail_lines


def transform_requirements(
    headers: list[str],
    sep: list[str],
    rows: list[list[str]],
) -> tuple[list[str], list[str]]:
    """Transform a requirements table."""
    keep_cols = [0, 2]  # ID, Derived From
    new_headers = [headers[i] for i in keep_cols]
    new_sep = [sep[i] for i in keep_cols if i < len(sep)]
    new_rows = []
    detail_lines = []

    for num, row in enumerate(rows, 1):
        new_rows.append([row[i] if i < len(row) else "" for i in keep_cols])
        row_id = row[0] if row else ""
        requirement = row[1] if len(row) > 1 else ""
        rationale = row[3] if len(row) > 3 else ""
        verification = row[4] if len(row) > 4 else ""

        sub_items = []
        if rationale:
            sub_items.append(("Rationale", rationale))
        if verification:
            sub_items.append(("Verification", verification))

        detail_lines.extend(make_detail_item(num, row_id, requirement, sub_items))

    table_lines = build_table(new_headers, new_sep, new_rows)
    return table_lines, detail_lines


def transform_user_stories(
    headers: list[str],
    sep: list[str],
    rows: list[list[str]],
) -> tuple[list[str], list[str]]:
    """Transform a user stories table."""
    keep_cols = [0, 1, 4, 5]  # ID, Persona, Features, Requirements
    new_headers = [headers[i] for i in keep_cols]
    new_sep = [sep[i] for i in keep_cols if i < len(sep)]
    new_rows = []
    detail_lines = []

    for num, row in enumerate(rows, 1):
        new_rows.append([row[i] if i < len(row) else "" for i in keep_cols])
        row_id = row[0] if row else ""
        story = row[2] if len(row) > 2 else ""
        acceptance = row[3] if len(row) > 3 else ""

        sub_items = []
        if acceptance:
            sub_items.append(("Acceptance", acceptance))

        detail_lines.extend(make_detail_item(num, row_id, story, sub_items))

    table_lines = build_table(new_headers, new_sep, new_rows)
    return table_lines, detail_lines


def transform_test_plan(
    headers: list[str],
    sep: list[str],
    rows: list[list[str]],
) -> tuple[list[str], list[str]]:
    """Transform a test plan table."""
    keep_cols = [0, 1]  # Test, Req
    new_headers = [headers[i] for i in keep_cols]
    new_sep = [sep[i] for i in keep_cols if i < len(sep)]
    new_rows = []
    detail_lines = []

    for num, row in enumerate(rows, 1):
        new_rows.append([row[i] if i < len(row) else "" for i in keep_cols])
        test_name = row[0] if row else ""
        description = row[2] if len(row) > 2 else ""

        detail_lines.extend(make_detail_item(num, test_name, description, []))

    table_lines = build_table(new_headers, new_sep, new_rows)
    return table_lines, detail_lines


def transform_test_cases(
    headers: list[str],
    sep: list[str],
    rows: list[list[str]],
) -> tuple[list[str], list[str]]:
    """Transform a test cases table."""
    keep_cols = [0, 3]  # #, Requirement
    new_headers = [headers[i] for i in keep_cols]
    new_sep = [sep[i] for i in keep_cols if i < len(sep)]
    new_rows = []
    detail_lines = []

    for num, row in enumerate(rows, 1):
        new_rows.append([row[i] if i < len(row) else "" for i in keep_cols])
        row_num = row[0] if row else str(num)
        input_text = row[1] if len(row) > 1 else ""
        expected = row[2] if len(row) > 2 else ""

        sub_items = []
        if expected:
            sub_items.append(("Expected", expected))

        detail_lines.extend(make_detail_item(num, f"#{row_num}", input_text, sub_items))

    table_lines = build_table(new_headers, new_sep, new_rows)
    return table_lines, detail_lines


def transform_req_trace(
    headers: list[str],
    sep: list[str],
    rows: list[list[str]],
) -> tuple[list[str], list[str]]:
    """Transform a req-trace table."""
    # Keep all cols except last (Description)
    keep_cols = list(range(len(headers) - 1))
    new_headers = [headers[i] for i in keep_cols]
    new_sep = [sep[i] for i in keep_cols if i < len(sep)]
    new_rows = []
    detail_lines = []

    for num, row in enumerate(rows, 1):
        new_rows.append([row[i] if i < len(row) else "" for i in keep_cols])
        key = row[0] if row else ""
        description = row[-1] if row else ""

        detail_lines.extend(make_detail_item(num, key, description, []))

    table_lines = build_table(new_headers, new_sep, new_rows)
    return table_lines, detail_lines


def split_table(
    headers: list[str],
    sep: list[str],
    rows: list[list[str]],
) -> tuple[list[str], list[str]]:
    """Split a wide table into two tables sharing col 0 as key."""
    # Find the split point: greedily add columns to table 1
    # until adding the next would exceed the limit
    key_col = 0
    t1_cols = [key_col]
    for col_idx in range(1, len(headers)):
        trial_cols = [*t1_cols, col_idx]
        trial_headers = [headers[i] for i in trial_cols]
        trial_sep = [sep[i] for i in trial_cols if i < len(sep)]
        trial_rows = [[r[i] if i < len(r) else "" for i in trial_cols] for r in rows]
        trial_lines = build_table(trial_headers, trial_sep, trial_rows)
        if max(len(ln) for ln in trial_lines) <= LINE_LIMIT:
            t1_cols = trial_cols
        else:
            break

    # If we couldn't fit even 2 cols in table 1, force col 0 + col 1
    if len(t1_cols) == 1:
        t1_cols = [0, 1] if len(headers) > 1 else [0]

    t2_cols = [key_col] + [c for c in range(1, len(headers)) if c not in t1_cols]

    # Build table 1
    t1_headers = [headers[i] for i in t1_cols]
    t1_sep = [sep[i] for i in t1_cols if i < len(sep)]
    t1_rows = [[r[i] if i < len(r) else "" for i in t1_cols] for r in rows]
    t1_lines = build_table(t1_headers, t1_sep, t1_rows)

    # Build table 2
    t2_headers = [headers[i] for i in t2_cols]
    t2_sep = [sep[i] for i in t2_cols if i < len(sep)]
    t2_rows = [[r[i] if i < len(r) else "" for i in t2_cols] for r in rows]
    t2_lines = build_table(t2_headers, t2_sep, t2_rows)

    # Check if table 2 also exceeds limit
    t2_max = max(len(ln) for ln in t2_lines)
    if t2_max > LINE_LIMIT:
        # Table 2 also too wide; move its long cols to detail list
        t2_headers_list = [headers[i] for i in t2_cols]
        t2_sep_list = [sep[i] for i in t2_cols if i < len(sep)]
        t2_rows_list = [[r[i] if i < len(r) else "" for i in t2_cols] for r in rows]
        result = transform_generic_inner(t2_headers_list, t2_sep_list, t2_rows_list)
        if result:
            t2_lines, detail_lines = result
        else:
            # Last resort: detail list only for table 2
            detail_lines = []
            for num, row in enumerate(rows, 1):
                key = row[0] if row else str(num)
                sub_items = []
                for col_idx in t2_cols[1:]:
                    val = row[col_idx] if col_idx < len(row) else ""
                    if val and val.lower() not in ("none", "n/a"):
                        sub_items.append((headers[col_idx], val))
                if sub_items:
                    detail_lines.extend(make_detail_item(num, key, "", sub_items))
            t2_lines = []

        # Combine: table1 + blank + table2 + blank + details
        combined = t1_lines
        if t2_lines:
            combined.append("")
            combined.extend(t2_lines)
        return combined, detail_lines

    # Both tables fit
    combined = [*t1_lines, "", *t2_lines]
    return combined, []


def transform_generic_inner(
    headers: list[str],
    sep: list[str],
    rows: list[list[str]],
) -> tuple[list[str], list[str]] | None:
    """Inner generic transform (no recursion into split)."""
    test_table = build_table(headers, sep, rows)
    max_len = max(len(line) for line in test_table)
    if max_len <= LINE_LIMIT:
        return None

    long_cols = set()
    short_cols = []
    for col_idx in range(len(headers)):
        max_cell = max(
            (len(row[col_idx]) if col_idx < len(row) else 0 for row in rows),
            default=0,
        )
        max_cell = max(max_cell, len(headers[col_idx]))
        if max_cell > 40:
            long_cols.add(col_idx)
        else:
            short_cols.append(col_idx)

    if not long_cols or not short_cols:
        short_cols = [0]
        long_cols = set(range(1, len(headers)))

    new_headers = [headers[i] for i in short_cols]
    new_sep = [sep[i] for i in short_cols if i < len(sep)]
    new_rows = []
    detail_lines = []

    for num, row in enumerate(rows, 1):
        new_rows.append([row[i] if i < len(row) else "" for i in short_cols])
        key = row[0] if row else str(num)
        long_col_list = sorted(long_cols)
        main_text = ""
        sub_items = []
        for idx, col_idx in enumerate(long_col_list):
            val = row[col_idx] if col_idx < len(row) else ""
            if not val or val.lower() in ("none", "n/a"):
                continue
            if idx == 0:
                main_text = val
            else:
                sub_items.append((headers[col_idx], val))
        if main_text or sub_items:
            detail_lines.extend(make_detail_item(num, key, main_text, sub_items))

    table_lines = build_table(new_headers, new_sep, new_rows)
    new_max = max(len(line) for line in table_lines)
    if new_max <= LINE_LIMIT:
        return table_lines, detail_lines
    return None


def transform_generic(
    headers: list[str],
    sep: list[str],
    rows: list[list[str]],
) -> tuple[list[str], list[str]] | None:
    """Transform a generic table by moving long columns to detail list.

    Identifies which columns cause overflow and moves them.
    Returns None if no transformation is needed.
    """
    # Check if any row exceeds the limit
    test_table = build_table(headers, sep, rows)
    max_len = max(len(line) for line in test_table)
    if max_len <= LINE_LIMIT:
        return None

    # Find which columns are "long" (have cells > 40 chars)
    long_cols = set()
    short_cols = []
    for col_idx in range(len(headers)):
        max_cell = max(
            (len(row[col_idx]) if col_idx < len(row) else 0 for row in rows),
            default=0,
        )
        max_cell = max(max_cell, len(headers[col_idx]))
        if max_cell > 40:
            long_cols.add(col_idx)
        else:
            short_cols.append(col_idx)

    if not long_cols or not short_cols:
        # Can't split meaningfully; try keeping first col + short cols
        short_cols = [0]
        long_cols = set(range(1, len(headers)))

    new_headers = [headers[i] for i in short_cols]
    new_sep = [sep[i] for i in short_cols if i < len(sep)]
    new_rows = []
    detail_lines = []

    for num, row in enumerate(rows, 1):
        new_rows.append([row[i] if i < len(row) else "" for i in short_cols])
        key = row[0] if row else str(num)

        # First long column is main text
        long_col_list = sorted(long_cols)
        main_text = ""
        sub_items = []
        for idx, col_idx in enumerate(long_col_list):
            val = row[col_idx] if col_idx < len(row) else ""
            if not val or val.lower() in ("none", "n/a"):
                continue
            if idx == 0:
                main_text = val
            else:
                sub_items.append((headers[col_idx], val))

        if main_text or sub_items:
            detail_lines.extend(make_detail_item(num, key, main_text, sub_items))

    table_lines = build_table(new_headers, new_sep, new_rows)

    # Verify the new table fits
    new_max = max(len(line) for line in table_lines)
    if new_max <= LINE_LIMIT:
        return table_lines, detail_lines

    # Table still too wide. Split into two tables sharing col 0 as key.
    return split_table(headers, sep, rows)


def table_needs_transform(
    schema: str | None,
    headers: list[str],
    sep: list[str],
    rows: list[list[str]],
) -> bool:
    """Check if a table needs transformation."""
    if schema in (
        "features",
        "requirements",
        "user-stories",
        "test-plan",
        "test-cases",
        "req-trace",
    ):
        return True

    # For unknown schemas, check if any line exceeds limit
    test_table = build_table(headers, sep, rows)
    return any(len(line) > LINE_LIMIT for line in test_table)


def process_file(filepath: Path) -> bool:
    """Process a single markdown file. Returns True if modified."""
    content = filepath.read_text(encoding="utf-8")
    lines = content.split("\n")
    result = []
    i = 0
    modified = False

    while i < len(lines):
        line = lines[i]

        # Check if this starts a table
        if is_table_row(line) and i + 1 < len(lines) and is_separator(lines[i + 1]):
            # Collect the entire table
            header_line = line
            sep_line = lines[i + 1]
            headers = parse_row(header_line)
            sep_cells = parse_row(sep_line)
            table_rows = []
            j = i + 2
            while j < len(lines) and is_table_row(lines[j]):
                table_rows.append(parse_row(lines[j]))
                j += 1

            schema = detect_schema(headers)

            if table_needs_transform(schema, headers, sep_cells, table_rows):
                # Apply transformation
                transform_result = None
                if schema == "features":
                    transform_result = transform_features(headers, sep_cells, table_rows)
                elif schema == "requirements":
                    transform_result = transform_requirements(headers, sep_cells, table_rows)
                elif schema == "user-stories":
                    transform_result = transform_user_stories(headers, sep_cells, table_rows)
                elif schema == "test-plan":
                    transform_result = transform_test_plan(headers, sep_cells, table_rows)
                elif schema == "test-cases":
                    transform_result = transform_test_cases(headers, sep_cells, table_rows)
                elif schema == "req-trace":
                    transform_result = transform_req_trace(headers, sep_cells, table_rows)
                else:
                    transform_result = transform_generic(headers, sep_cells, table_rows)

                if transform_result:
                    table_lines, detail_lines = transform_result
                    result.extend(table_lines)
                    if detail_lines:
                        result.append("")
                        result.extend(detail_lines)
                    modified = True
                    i = j
                    continue

            # No transform needed or possible; keep original
            for k in range(i, j):
                result.append(lines[k])
            i = j
            continue

        result.append(line)
        i += 1

    if modified:
        # Ensure file ends with single newline
        output = "\n".join(result)
        if not output.endswith("\n"):
            output += "\n"
        filepath.write_text(output, encoding="utf-8")
        run_hook(filepath)

    return modified


def main() -> None:
    root = Path(".")
    if len(sys.argv) > 1:
        root = Path(sys.argv[1])

    md_files = [root] if root.is_file() else sorted(root.rglob("*.md"))

    # Exclude paths from .rumdl.toml excludes
    excludes = {
        ".git",
        ".github",
        "CLAUDE.md",
        "build",
        "node_modules",
        "vcpkg_installed",
        "vendor",
    }

    total_modified = 0
    for f in md_files:
        parts = f.parts
        if any(ex in parts for ex in excludes):
            continue
        if f.name == "CLAUDE.md":
            continue
        if f.name == "AGENTS.md":
            continue

        if process_file(f):
            total_modified += 1
            print(f"  Modified: {f}")

    print(f"\nTotal files modified: {total_modified}")


if __name__ == "__main__":
    main()
