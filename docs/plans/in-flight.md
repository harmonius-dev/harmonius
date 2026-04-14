---
in_flight:
  - last_seen: 2026-04-14T03:44:25Z
    parent_task_id: 88C465A7-A7D5-4E23-A5D8-BDCEDD74D7A2
    phase: plan
    pr_number: null
    started_at: 2026-04-14T03:44:25Z
    subsystem: all
    task_id: 683ce271-c336-469a-968c-99f0998d4345
    topic: dispatch-only
    worker_agent: plan-orchestrator
  - last_seen: 2026-04-14T03:44:25Z
    parent_task_id: 88C465A7-A7D5-4E23-A5D8-BDCEDD74D7A2
    phase: specify
    pr_number: null
    started_at: 2026-04-14T03:44:25Z
    subsystem: all
    task_id: bc490826-a91b-47fa-a887-98bcea5d5665
    topic: ready-pass
    worker_agent: specify-orchestrator
  - last_seen: 2026-04-14T03:44:25Z
    parent_task_id: 88C465A7-A7D5-4E23-A5D8-BDCEDD74D7A2
    phase: design
    pr_number: null
    started_at: 2026-04-14T03:44:25Z
    subsystem: all
    task_id: 1da8effe-bf8d-46d0-a654-1da994f88dfb
    topic: ready-pass
    worker_agent: design-orchestrator
---

# In-Flight Background Tasks

Registry of background Claude Code tasks currently running in the SDLC. The harmonize master agent
and phase orchestrators write to this file on dispatch and remove entries when tasks complete.
Sub-skills read this file to decide whether to call `TaskStop` before claiming a coarse lock.

## Entry schema

```yaml
in_flight:
  - task_id: {Claude Code task id from Agent run_in_background}
    worker_agent: {agent name, e.g., feature-author}
    phase: {specify | design | plan | review | release}
    subsystem: {subsystem identifier}
    topic: {optional fine-grained topic for logging}
    pr_number: {draft PR number if the worker has opened one}
    started_at: {ISO 8601 UTC timestamp}
    last_seen: {ISO 8601 UTC timestamp — updated on each orchestrator pass}
    parent_task_id: {orchestrator task that dispatched this worker}
```

## Reconciliation loop

Every harmonize master agent pass performs these steps:

1. Read this file
2. For each entry, call `TaskList` and check whether `task_id` is still running
3. If completed, call `TaskOutput(task_id)` and update the corresponding phase progress file
4. If stopped or errored, append a warning to the phase progress file
5. If still running, update `last_seen` to the current UTC timestamp
6. Remove completed and stopped entries from this file

## Stop-before-lock protocol

When an interactive sub-skill claims a coarse lock on `(phase, subsystem)`, it MUST:

1. Read this file
2. Find every entry where `phase == target_phase && subsystem == target_subsystem`
3. Call `TaskStop(task_id)` for each match
4. Remove those entries from this file
5. Append the new lock entry to `locks.md`
6. Begin interactive work

Background workers MUST skip any `(phase, subsystem)` with an active lock. Locks always win over
in-flight tasks — the user's interactive session takes precedence.
