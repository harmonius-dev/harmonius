# User Stories — 12.5 Streaming & I/O

## US-12.5.1 Explore an Open World Without Loading Screens or Pop-In
**As a** player, **I want** seamless asset streaming with platform-native async I/O, GPU
direct storage, texture and mesh LOD streaming, priority scheduling, memory pressure
management, compressed archives, and download-on-demand patching, **so that** I can traverse
the world continuously without loading screens or visible pop-in.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Virtual file system over loose files, paks, and HTTP | F-12.5.1 | R-12.5.1 |
| Platform-native async I/O (IOCP/GCD/io_uring) | F-12.5.2 | R-12.5.2 |
| GPU direct storage bypassing CPU for bulk transfers | F-12.5.3 | R-12.5.3 |
| Mip-level texture streaming with sparse binding | F-12.5.4 | R-12.5.4 |
| Mesh LOD streaming with dithered cross-fade | F-12.5.5 | R-12.5.5 |
| Priority queue with aging to prevent starvation | F-12.5.6 | R-12.5.6 |
| Memory pressure response with progressive eviction | F-12.5.7 | R-12.5.7 |
| Seekable pak archives with spatial locality | F-12.5.8 | R-12.5.8 |
| LZ4 and Zstd per-chunk compression | F-12.5.9 | R-12.5.9 |
| CDN download-on-demand with hash verification | F-12.5.10 | R-12.5.10 |
| Async I/O at 80%+ of raw disk bandwidth | F-12.5.2 | R-X.15.1 |
| No stdlib file I/O anywhere in the engine | F-12.5.2 | R-14.6.1 |
