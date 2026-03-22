---
name: block-vendor-edits
enabled: true
event: file
action: block
conditions:
  - field: file_path
    operator: regex_match
    pattern: vendor/
---

**Vendor file edit blocked.**

Files under `vendor/` are third-party vendored dependencies (e.g., metal-cpp) and must not be
modified directly. If you need changes, update the upstream source and re-vendor.
