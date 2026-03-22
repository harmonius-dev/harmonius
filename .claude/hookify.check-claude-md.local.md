---
name: check-claude-md
enabled: true
event: file
conditions:
  - field: file_path
    operator: regex_match
    pattern: CLAUDE\.md$
action: warn
---

**Editing agent instructions (CLAUDE.md).**

CLAUDE.md files contain agent rules, coding standards, and project guidelines. Verify that your
edits preserve the existing structure and do not remove critical sections.
