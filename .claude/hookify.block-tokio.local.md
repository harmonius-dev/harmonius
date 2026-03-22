---
name: block-tokio
enabled: true
event: file
pattern: \btokio\b|async.std\b|smol::
action: block
---

**External async runtime detected.**

This project forbids tokio, async-std, smol, and all external async runtimes. All async I/O must use
the custom IoReactor built on platform primitives (io_uring, GCD, IOCP).
