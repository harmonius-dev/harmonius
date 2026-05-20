# Backlog seed scripts

These scripts seed GitHub Issues, Labels, and the Engine Roadmap GitHub Project from
[`../seed.json`](../seed.json). They are one-time bootstrap tools; ongoing changes flow through
GitHub Issues directly. See [`../AGENTS.md`](../AGENTS.md) for the source-of-truth contract.

## Prerequisites

| Tool   | Purpose                                                  |
|--------|----------------------------------------------------------|
| `gh`   | Authenticated against the target repo (`gh auth login`)  |
| `jq`   | Reads `seed.json`                                        |
| `bash` | 4.0 or newer (associative arrays)                        |

The `gh` token must include:

- `repo` scope for label and issue writes.
- `read:project` and `project` scopes for the project script.

## Scripts

### `seed-labels.sh`

Creates or updates every label defined in `seed.json`. Idempotent. Existing labels are edited in
place to match the canonical color and description.

```bash
bash docs/backlog/scripts/seed-labels.sh           # apply
bash docs/backlog/scripts/seed-labels.sh -n        # dry run
bash docs/backlog/scripts/seed-labels.sh -r owner/repo
```

### `seed-issues.sh`

Creates GitHub issues for every entry in `seed.json` whose `BL-NNNN:` title prefix is not already
present in the repo. Issues that already exist (open or closed) are skipped.

```bash
bash docs/backlog/scripts/seed-issues.sh                      # apply
bash docs/backlog/scripts/seed-issues.sh -n                   # dry run
bash docs/backlog/scripts/seed-issues.sh -i BL-0001 -i BL-0002
```

### `seed-project.sh`

Creates the `Engine Roadmap` GitHub Project (v2) under the repo owner if it does not already exist,
then adds every `BL-*`-titled issue from the repo to the project.

Column / swimlane / automation configuration is documented in [`../projects.md`](../projects.md) and
applied through the GitHub UI or the GraphQL API. The script does the structural part only.

```bash
bash docs/backlog/scripts/seed-project.sh                     # apply
bash docs/backlog/scripts/seed-project.sh -n                  # dry run
bash docs/backlog/scripts/seed-project.sh -o my-org -r my-org/my-repo
```

## Recommended order

1. `seed-labels.sh` — labels exist before issues so issue creation can attach them.
2. `seed-issues.sh` — issues are created with their canonical labels.
3. `seed-project.sh` — project picks up the issues just created.

Each step is idempotent; rerunning is safe.

## Dry-run sanity check

```bash
bash docs/backlog/scripts/seed-labels.sh -n  | head
bash docs/backlog/scripts/seed-issues.sh -n  | head
bash docs/backlog/scripts/seed-project.sh -n | head
```

## After seeding

- Confirm the issue catalog at
  [`https://github.com/cjhowe-us/harmonius/issues?q=BL-`](https://github.com/cjhowe-us/harmonius/issues?q=BL-)
- Confirm the project at the GitHub Projects tab on the repo owner.
- Update [`../AGENTS.md`](../AGENTS.md) `Counts (snapshot)` if the seeded counts diverge.

## Re-seeding

Re-running the scripts is safe. They never edit existing issue bodies — once an issue is in GitHub,
GitHub is the source of truth. To roll forward an existing issue, edit it on GitHub. To roll back a
`seed.json` change, edit `seed.json` and rerun `seed-issues.sh`; missing issues will be created,
present issues will be left alone.
