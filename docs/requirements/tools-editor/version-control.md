# R-15.10 -- Version Control Requirements

## Requirements

1. **R-15.10.1** — The engine **SHALL** embed a Git client using libgit2 with stage, commit, push,
   pull, branch, merge, rebase, and stash operations accessible from the editor UI.
   - **Rationale:** Integrated version control keeps designers in the editor and avoids CLI
     complexity.
   - **Verification:** Commit, push, and pull a change via the editor UI and verify repository state
     matches.

2. **R-15.10.2** — The engine **SHALL** auto-track binary assets via Git LFS based on extension and
   size thresholds, with lock/unlock, bulk operations, and storage quota monitoring.
   - **Rationale:** LFS is required for large binary assets; quota monitoring prevents cost
     surprises.
   - **Verification:** Import a 500 MB texture, verify it is LFS-tracked, and check the quota
     display updates.

3. **R-15.10.3** — The engine **SHALL** register a custom Git merge driver for structural three-way
   merge of graphs, Entity Templates, and data tables, falling back to the visual diff tool for
   unresolvable conflicts.
   - **Rationale:** Binary assets require structural merge; text-based merge produces corrupt files.
   - **Verification:** Create conflicting graph edits on two branches, merge, and verify the
     structural merge resolves non-overlapping changes automatically.

4. **R-15.10.4** — The engine **SHALL** support branch-per-feature workflows with branch creation,
   switching with cache preservation, and pull/merge request creation from the editor.
   - **Rationale:** Branching workflows are standard in team development.
   - **Verification:** Create a branch, make a change, create a PR, and verify it appears on the Git
     hosting provider.

5. **R-15.10.5** — The engine **SHALL** display real-time collaborative presence indicators and
   support pessimistic locking for non-mergeable assets.
   - **Rationale:** Presence prevents conflicting edits; locking protects non-mergeable assets.
   - **Verification:** Lock an asset from one editor instance and verify a second instance cannot
     edit it.

6. **R-15.10.6** — The engine **SHALL** support Git partial clone and sparse checkout with
   role-based patterns and on-demand fetch for missing assets.
   - **Rationale:** Sparse checkout reduces clone time and disk usage for large repositories.
   - **Verification:** Clone with a sparse pattern, verify excluded assets show placeholder
     thumbnails, fetch one on demand, and verify it loads correctly.

7. **R-15.10.7** — The engine **SHALL** provide named shelves for work-in-progress with structural
   diffs, shareable via the shared build cache.
   - **Rationale:** Shelves enable saving and handing off work without committing incomplete
     changes.
   - **Verification:** Create a shelf, share it, and verify a second developer can apply it.

8. **R-15.10.8** — The engine **SHALL** support GitHub, GitLab, Bitbucket, and self-hosted Git
   servers with auto-detection from the remote URL and in-editor PR/review actions.
   - **Rationale:** Multi-provider support avoids vendor lock-in for hosting.
   - **Verification:** Connect to each provider, create a PR, and verify status updates appear in
     the editor.
