---
name: block-mocking
enabled: true
event: file
pattern: \b(mockall|MockAll|MOCK_METHOD|mockito|unittest\.mock|jest\.mock|vi\.mock)\b
action: block
---

**Mocking is explicitly forbidden.**

No mock libraries, no mock objects in any language. Use real dependencies. Only use full fakes that
implement the same interface/trait when no other option exists.
