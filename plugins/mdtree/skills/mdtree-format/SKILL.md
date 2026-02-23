---
name: mdtree Format Convention
description: >-
  This skill should be used when the user asks to "add a section",
  "rename a folder", "restructure docs", "create a numbered document tree",
  "organize docs into sections", "update the document hierarchy",
  "move a section", or when creating, editing, or renaming files and
  folders that follow a hierarchical numbered markdown tree pattern
  (e.g., 1-rendering/, 1.1-core-rendering.md, F-1.1.1). Provides the
  naming conventions, numbering rules, and document format for mdtree
  hierarchies.
version: 0.1.0
---

# mdtree Format Convention

Hierarchical sequentially numbered markdown document trees. This convention
organizes documentation into numbered folders containing numbered markdown
files, each holding sequentially numbered items.

## Three-Tier Numbering

```
Tier 1 (Section):    N           → folder:  N-{kebab-name}/
Tier 2 (Subsection): N.M         → file:    N.M-{kebab-name}.md
Tier 3 (Item):       N.M.K       → heading: ## PREFIX-N.M.K {Title}
```

- Numbering is sequential within each tier (1, 2, 3... never skip)
- Files inside folder `N-name/` always start with `N.` (e.g., folder `2-advanced/` contains `2.1-foo.md`, `2.2-bar.md`)
- Item headings inside file `N.M-name.md` always start with `PREFIX-N.M.` (e.g., file `3.1-meshlet.md` contains `## F-3.1.1`, `## F-3.1.2`)

## Item Prefix

Every mdtree requires a short uppercase prefix for its items. The prefix
is declared in the root README (e.g., `F-` for features → `F-1.1.1`,
`R-` for requirements → `R-1.1.1`). All item headings must include the
prefix: `## PREFIX-N.M.K {Title}`.

## Folder Naming

```
{N}-{kebab-case-name}/
```

Examples: `1-rendering/`, `2-advanced-rendering/`, `3-quality/`

## File Naming

```
{N.M}-{kebab-case-name}.md
```

Examples: `1.1-core-rendering.md`, `2.3-data-constraints.md`

## Root README Format

Every mdtree has a `README.md` at its root that serves as a navigation index:

```markdown
# {Tree Title}

{One-line description}. Each {item type} has a unique identifier
(e.g., `PREFIX-N.M.K`).

## Sections

### {N} {Section Title}
- [{N.M} {Subsection Title}]({N}-{folder-name}/{N.M}-{file-name}.md) — brief description
```

## Document File Format

Each `.md` file contains a title heading and sequentially numbered items:

```markdown
# {N.M} {Subsection Title}

## PREFIX-{N.M.1} {Item Title}

{Description paragraph.}

## PREFIX-{N.M.2} {Item Title}

{Description paragraph.}
```

## Cross-References

Trees may reference items in other trees using inline markdown links:

```markdown
- **{Label}:** [{ID}](../../{other-tree}/{section-folder}/{file}.md) {description}
```

Example from a features tree referencing requirements:

```markdown
- **Requirements:** [R-1.1.2](../../requirements/1-architecture/1.1-core-constraints.md) GPU-driven rendering
```

Multiple references are comma-separated:

```markdown
- **Requirements:** [R-1.1.2](../../requirements/1-architecture/1.1-core-constraints.md) GPU-driven, [R-2.2.1](../../requirements/2-nonfunctional/2.2-hardware.md) bindless
```

## Adding Content

### Adding a new item to an existing file

Append with the next sequential number. If the last item in `2.1-ray-tracing.md`
is `## F-2.1.4`, the next is `## F-2.1.5`.

### Adding a new file to an existing section

Use the next subsection number. If folder `1-rendering/` has files up to
`1.3-shadows.md`, the next file is `1.4-{name}.md`. Add a corresponding
link entry under the relevant `### N` heading in the root README.

### Adding a new section

Use the next section number. If sections go up to `6-tooling/`, create
`7-{name}/`. Update the root README.

### Renaming

When renaming a file or folder, preserve its number. Change only the
kebab-case name portion. Update all references (README links, cross-references
from other files).

## Reference

For a complete worked example of an mdtree hierarchy, see:
- **`references/example-tree.md`** — full directory listing and sample content
