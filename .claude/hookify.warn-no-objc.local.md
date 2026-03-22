---
name: warn-no-objc
enabled: true
event: file
pattern: objc2.metal|@objc\b|#import\s*<objc/|\.mm?$
action: warn
---

**Avoid Objective-C.**

Use metal-cpp for Metal API access and Swift C++ interop via cxx.rs for platform bridges. Do not
write Objective-C or Objective-C++ code.
