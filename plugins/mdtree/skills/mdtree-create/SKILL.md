---
name: mdtree Create
description: >-
  This skill should be used when the user asks to "create a new document tree",
  "scaffold docs", "initialize an mdtree", "set up a doc hierarchy",
  "create sections for", "generate documentation structure",
  "build a docs skeleton", "new mdtree", or wants to generate a new
  hierarchical numbered markdown document structure from scratch.
  Produces a full directory structure with numbered folders, placeholder
  markdown files, and a root README index.
version: 0.1.0
---

# mdtree Create

Scaffold a new hierarchical numbered markdown document tree. Takes a tree
name, section definitions, and optional configuration (item prefix,
cross-references) and produces the full directory structure with README
and placeholder files.

## Prerequisites

All generated files must follow the mdtree-format convention (numbering,
naming, and document structure). Refer to the mdtree-format skill for
the complete format specification.

## Gathering Requirements

Before creating a tree, collect:

1. **Tree name** — the root directory name (kebab-case, e.g., `features`, `api-docs`)
2. **Parent path** — where to create it (e.g., `docs/`)
3. **Sections** — numbered categories, each with a title and list of subsections
4. **Item prefix** (required) — a short uppercase prefix, e.g., `F-` for features, `R-` for requirements, `API-` for API docs
5. **Cross-references** (optional) — which other mdtree(s) this tree references, and the label to use (e.g., "Requirements")

## Scaffolding Process

### Step 1: Create directory structure

For each section N with subsections, create:

```
{parent-path}/{tree-name}/
├── README.md
├── {N}-{section-kebab}/
│   ├── {N.1}-{subsection-kebab}.md
│   ├── {N.2}-{subsection-kebab}.md
│   └── ...
├── {N+1}-{section-kebab}/
│   └── ...
```

### Step 2: Generate root README

Follow this template:

```markdown
# {Tree Title}

{Description}. Each {item type} has a unique identifier
(e.g., `PREFIX-N.M.K`).{cross-ref-note}

## Sections

### {N} {Section Title}
- [{N.M} {Subsection Title}]({N}-{folder}/{N.M}-{file}.md) — {brief description}
```

If cross-references are configured, add a note:
```markdown
{Item type}s reference {target tree} from [{path}]({relative-path}) by ID.
```

### Step 3: Generate document files

Each subsection file starts with placeholder content:

```markdown
# {N.M} {Subsection Title}

## PREFIX-{N.M.1} {Placeholder Item}

{Description.}
```

Only include one placeholder item per file. The user will fill in real content.

### Step 4: Verify

After scaffolding, verify:
- All folders numbered sequentially starting at 1
- All files within each folder numbered sequentially (N.1, N.2, ...)
- README links resolve correctly
- Cross-reference paths are valid relative paths

## Cross-Reference Setup

When the user specifies cross-references between trees:

1. Determine the relative path from source tree files to target tree files
2. Add the reference note to the source tree's README
3. Include a cross-reference template in each placeholder item:

```markdown
- **{Label}:** [{target-id}]({relative-path-to-target-file}) {description}
```

The relative path depends on directory depth. From a file at
`{tree}/N-section/N.M-file.md`, the path to a sibling tree is typically
`../../{other-tree}/{section-folder}/{file}.md`.

## Example Session

User: "Create an mdtree for API documentation under docs/ with sections:
Authentication (endpoints: oauth, tokens), Users (endpoints: crud, profiles),
Billing (endpoints: invoices, subscriptions). Prefix items with API-. Reference
the requirements tree."

Result:

```
docs/api-docs/
├── README.md
├── 1-authentication/
│   ├── 1.1-oauth.md
│   └── 1.2-tokens.md
├── 2-users/
│   ├── 2.1-crud.md
│   └── 2.2-profiles.md
└── 3-billing/
    ├── 3.1-invoices.md
    └── 3.2-subscriptions.md
```

README excerpt:
```markdown
# API Documentation

API endpoint specification. Each endpoint has a unique identifier
(e.g., `API-2.1.3`). Endpoints reference requirements from
[docs/requirements/](../requirements/) by ID.

## Sections

### 1 Authentication
- [1.1 OAuth](1-authentication/1.1-oauth.md) — OAuth 2.0 flows
- [1.2 Tokens](1-authentication/1.2-tokens.md) — token management
```
