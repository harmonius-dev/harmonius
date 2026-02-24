# mdtree

Hierarchical sequentially numbered markdown document trees with cross-references.

## What it does

Enforces a consistent convention for organizing documentation into numbered
folders (`1-rendering/`), numbered files (`1.1-core-rendering.md`), and
numbered items (`## F-1.1.1 Title`). Automatically reminds Claude to follow
the convention when editing matching files.

## Components

- **mdtree-format** (skill) — naming conventions, numbering rules, and
  document format
- **mdtree-create** (skill) — scaffolds a new mdtree hierarchy from a description
- **PreToolUse hook** — detects numbered file/folder patterns and loads
  format conventions

## Installation

First, register the project marketplace (one-time):

```text
/plugin marketplace add .
```

Then install the plugin:

```text
/plugin install mdtree@harmonius
```

Or load directly without the marketplace:

```bash
claude --plugin-dir ./plugins/mdtree
```

## Marketplace Setup

This plugin is registered in the root `marketplace.json`. The marketplace
uses the schema:

```json
{
  "name": "marketplace-name",
  "owner": { "name": "author" },
  "plugins": [
    {
      "name": "plugin-name",
      "source": "./plugins/plugin-name",
      "description": "What the plugin does"
    }
  ]
}
```

To add a new plugin, append an entry to the `plugins` array with `name`,
`source`, and `description`. Then users install with
`/plugin install plugin-name@marketplace-name`.

## Usage

The format skill activates automatically when creating or editing files
matching the numbered pattern. To scaffold a new tree:

> "Create an mdtree for API docs under docs/ with sections: Auth, Users,
> Billing"
