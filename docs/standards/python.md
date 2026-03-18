# Python Coding Standard

Based on the [Google Python Style Guide](https://google.github.io/styleguide/pyguide.html) with
project-specific overrides noted below.

## Scope

Build scripts, asset pipeline utilities, and development tools. All Python lives in `scripts/`.

## Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Package | `snake_case` | `pipeline_tools` |
| Module | `snake_case` | `fix_tables` |
| Class | `PascalCase` | `TableParser` |
| Function / method | `snake_case` | `process_file` |
| Variable | `snake_case` | `line_count` |
| Constant | `SCREAMING_SNAKE` | `LINE_LIMIT` |
| Type variable | `PascalCase` | `T`, `RowData` |
| Private | `_leading_underscore` | `_parse_row` |

Avoid single-character names except loop counters (`i`, `j`), exception identifiers (`e`), file
handles (`f`), and type variables (`T`).

## File Organization

- All scripts in `scripts/` directory
- One script per task (`fix_tables.py`, `rewrap.py`)
- Shebang `#!/usr/bin/env python3` on all scripts
- `__main__` guard for all entry points
- Module-level docstring on every file

## Import Rules

Order (separated by blank lines):

1. `from __future__ import` (if needed)
2. Standard library
3. Third-party packages
4. Local project imports

Within each group, sort lexicographically. One import per line. Never use relative imports — always
use full package names. Never use star imports.

## Formatting Rules

- **100-character line limit** (override: Google uses 80)
- 4-space indentation, never tabs
- Ruff formatter exclusively (no black, no autopep8)
- Trailing commas in multi-line expressions
- Double quotes for strings
- Implicit line continuation (parentheses, brackets) — no backslash continuation
- No spaces before punctuation; spaces after commas/colons

## Linting Rules

- Ruff with project-configured rule set in `pyproject.toml`
- All warnings treated as errors
- No `# type: ignore` without specific mypy error code
- No `# noqa` without specific ruff rule code

## Type Checking

- **Static types required on all functions** — every parameter and return type must be annotated
- Do not annotate `self`, `cls`, or `__init__` return
- Use built-in generics: `list[str]`, `dict[str, int]`, `tuple[int, ...]` (PEP 585) — not
  `typing.List`
- Use union syntax: `X | None` (PEP 604) — not `Optional[X]` or `Union[X, Y]`
- Use `collections.abc.Sequence` over concrete `list` in parameter types when mutation is not needed
- Mypy strict mode must pass with zero errors

## Docstrings

Follow Google-style docstrings (PEP 257):

```python
def process_file(
    filepath: Path,
    limit: int = 100,
) -> bool:
    """Process a markdown file for table violations.

    Reads the file, identifies tables exceeding the line
    limit, and rewrites them with detail lists.

    Args:
        filepath: Path to the markdown file.
        limit: Maximum line length in characters.

    Returns:
        True if the file was modified.

    Raises:
        FileNotFoundError: If filepath does not exist.
    """
```

Required on all public functions, classes, and modules. One-line summary ending with a period.
Include `Args`, `Returns`/`Yields`, and `Raises` sections when applicable.

## How to Check and Fix

| Validation | Check command | Fix command |
|------------|--------------|-------------|
| Format | `ruff format --check scripts/` | `ruff format scripts/` |
| Lint | `ruff check scripts/` | `ruff check --fix scripts/` |
| Type-check | `mypy scripts/` | (manual) |
| All | `ruff check scripts/ && mypy scripts/` | `ruff check --fix scripts/` |

## Project-Specific Rules

1. **uv for package management** — use `uv` for dependency management, tool installation
   (`uv tool install ruff`), and virtual environments
2. **Ruff only** — no black, no isort, no flake8, no pylint, no autopep8
3. **Mypy strict** — `mypy --strict` must pass with zero errors on all Python
4. **Latest Python** — require Python 3.13+ for PEP 585, 604, and 695 syntax
5. **Scripts are utilities** — not production runtime code; keep them focused and single-purpose

## Cache-Friendly Patterns

- **Generators over lists** — use generators for large file iteration to avoid loading everything
  into memory
- **In-place file processing** — read, transform, write back; avoid holding two copies
- **`Path` over string ops** — use `pathlib.Path` for all filesystem operations
- **`str.join()` over `+`** — never concatenate strings with `+` in loops (quadratic complexity)

## Exception Handling

1. Use built-in exceptions (`ValueError`, `TypeError`)
2. Never use bare `except:` — always catch specific types
3. Minimize code in `try` blocks
4. Use `finally` or `with` for cleanup
5. Custom exceptions inherit from `Exception` and end with `Error`
6. Never use `assert` for API contracts — assertions can be stripped

## Testing

1. **Test-driven development** — write tests first, driven by requirements
2. **NO MOCKING** — no `unittest.mock`, no `pytest-mock`, no `MagicMock`
3. **Real files** — test with real temp files via `tempfile`, not mock file objects
4. **Pure functions** — maximize pure functions that take input and return output
5. **Immutable test data** — use tuples and frozensets for test fixtures

## Best Practices

1. Type annotations on every function signature
2. `pathlib.Path` instead of `os.path`
3. f-strings for string formatting
4. `enumerate()` instead of manual counters
5. `with` statements for all file and resource I/O
6. Comprehensions over `map()`/`filter()` with lambdas
7. Functions under 40 lines — break up longer functions
8. `dataclasses` or `NamedTuple` for structured data
9. `sys.exit()` with meaningful exit codes
10. Module-level docstrings on all files
11. `None` default + assignment for mutable arguments
12. `contextlib.closing()` for non-context-manager resources
13. `queue.Queue` for inter-thread communication

## Anti-Patterns

1. **`Any` type** — defeats the purpose of type checking
2. **Bare `except`** — always catch specific exceptions
3. **Mutable default arguments** — use `None` and assign inside the function
4. **Global mutable state** — use function parameters and constants
5. **`os.path` string manipulation** — use `pathlib.Path`
6. **`print()` for errors** — use `sys.stderr` or `logging`
7. **Star imports** — always import specific names
8. **`type: ignore` without code** — specify the mypy error code
9. **String concatenation with `+` in loops** — use `str.join()` or `io.StringIO`
10. **Untyped functions** — every function needs parameter and return type annotations
11. **Relative imports** — always use full package names
12. **`assert` for validation** — use exceptions for API contracts
13. **Custom metaclasses** — avoid power features
14. **Double underscore mangling** — use single `_` prefix
