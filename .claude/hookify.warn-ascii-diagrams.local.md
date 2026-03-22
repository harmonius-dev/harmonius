---
name: warn-ascii-diagrams
enabled: true
event: file
conditions:
  - field: file_path
    operator: regex_match
    pattern: \.md$
  - field: new_text
    operator: regex_match
    pattern: "[+|][-+]{4,}[+|]"
action: warn
---

**ASCII diagram detected in Markdown.**

Never create ASCII diagrams. Use Mermaid diagrams instead and render them with MCP to verify
correctness.
