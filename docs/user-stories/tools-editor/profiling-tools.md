# User Stories: Profiling Tools

## US-15.5.1 CPU Bottleneck Identification

**As a** developer, **I want** to view a per-frame CPU timeline showing all job system tasks
and subsystem ticks as color-coded swimlanes, **so that** I can identify which system is
causing frame time spikes without instrumenting code manually.

## US-15.5.2 Remote Server Profiling

**As a** DevOps engineer, **I want** to connect the editor profiler to a live game server
over TCP, **so that** I can diagnose performance issues under real player load without
deploying debug builds to production.
