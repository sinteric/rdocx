---
description: Integrate a prepared worker branch into the canonical sprint branch. Validates the handoff first and never resolves a semantic conflict on its own.
---

# /integrate-feature F-XXX work/<fid-lower>-<agent> [--batch]

Bring one worker's finished F-ID onto the sprint branch. Run this **only from
the canonical sprint worktree**, after the worker has run
`/complete-feature F-XXX --prepare`.

`--batch` is the mode `/run-sprint` uses. It creates the local F-ID commit and
defers the sprint ledgers, the `done` status, consolidated verification and the
push to the end of the sprint run.

## Acceptance reconciliation

Before consuming a worker's handoff, compare its A-ID evidence with the approved
consumer map and test matrix. Verify the integrated callers still supply the
same context and defaults. Qualify interactions introduced by neighboring
stories with focused checks rather than assuming individually green workers
compose. Keep unmet native, installed and corpus criteria pending in the
existing records. A contributed patch's test count never substitutes for its
linked issue's whole acceptance inventory.

Compare the worker's acknowledged plan/instruction revisions with the current
canonical contract before squash integration. If they differ, reconcile the
material changes and affected evidence with the owner first. Do not let an
older worker plan overwrite a newer approved plan merely because Git merges it
without a conflict. Preserve current decisions and re-review semantic changes.

## Preconditions

1. **Clean tree on `sprint/sNN`**, matching `docs/sprints/CURRENT_SPRINT.md`.

2. **F-XXX is `in-progress`** in `CURRENT_SPRINT.md` and `BACKLOG.md`, with an
   `Owner` matching the agent suffix on the named branch.

3. **The branch name matches `work/<fid-lower>-<agent>`** and its merge base is
   an ancestor of the current sprint branch. A worker that rebased onto
   something else is not integrable without a decision.

4. **The handoff validates.** Extract `.claude/handoffs/F-XXX-ready.md` from the
   worker branch to a temporary path and run:

   ```bash
   git show work/<fid-lower>-<agent>:.claude/handoffs/F-XXX-ready.md > <tmp>
   python3 scripts/sprint_workflow.py validate-handoff <tmp> --fid F-XXX
   ```

   Then confirm what the script cannot: its `Base` is the sha the claim
   recorded, its `Head` is an ancestor of the branch tip, and **no change to
   `crates/` appears after that head**. Evidence recorded before further edits
   is not evidence.

5. **The branch actually contains** the design plan, the implementation, the
   tests, and a `.claude/reviews/F-XXX-*` pass reporting zero defects and zero
   smells.

6. **Read the branch diff before merging.** Stop for anything unrelated to
   F-XXX, any edit to another active F-ID's files, and any worker edit to
   `AS_BUILT.md` or `SPRINT_TRACKER.md`. Those totals belong to the integrator.

## Integration

1. `git merge --squash work/<fid-lower>-<agent>`. Do not commit yet.

2. **Resolve no semantic conflict automatically.** A conflict between two
   sprint features is reconciled against **both approved design plans**, then
   re-reviewed with `/microscope F-XXX --working`, and the reconciliation is
   recorded. Any other conflict stops for a human decision.

3. **Consume applicable local evidence.** Run only focused regressions earned
   by integration reconciliation or invalidated source. Do not invoke a second
   general `/verify`, full suite, oracle or risk rider. A semantic reconciliation
   still needs independent microscope of its new material.

4. **Complete only after actual final acceptance.** When final checks remain,
   integrate in batch as reviewed, preserving the approved plan, pending checks
   and in-progress delivery status, except an explicitly approved local
   provider-prefix completion through run-sprint. That route alone can set
   provider run/dependency status completed while plan/rider debt stays pending.
   Source/HLD intent is already reviewed. Delivery ledgers wait for actual final
   observations, with one entry per integrated story.

5. **Consume the handoff.** Its durable facts, the harness result and the test
   gate, move into the AS_BUILT entry. Then delete
   `.claude/handoffs/F-XXX-ready.md` from the staged integration. A handoff left
   behind blocks `close-preflight`, which is deliberate.

6. **Run the generated-skill drift check** if anything under
   `.claude/commands/` or `.claude/skills/` changed:

   ```bash
   python3 scripts/sync_agent_skills.py --check
   ```

7. **Commit once** using the `/complete-feature` message format.

8. **Outside `--batch`, push `sprint/sNN`** and nothing else.

9. Record the result:

   ```bash
   python3 scripts/sprint_workflow.py mark-feature SNN F-XXX reviewed \
     --head <worker-head-sha> --handoff consumed \
     --integration-commit <new-sha>
   ```

10. **Save disk space.** After the integration commit is recorded, confirm the
    worker path is still registered to that branch and its status is empty.
    Remove that clean worktree with `git worktree remove` without `--force`.
    Keep the worker branch for final verification and review. Leave a dirty,
    carried or unrelated worktree intact and report why it remains.

## What `--batch` changes

1. Commit only feature-local work: code, tests, the design plan, the review
   files, the HLD files the plan listed, and the consumed handoff.
2. Leave F-XXX `in-progress` with its `Owner` intact. Do not append `AS_BUILT.md`
   or `SPRINT_TRACKER.md`.
3. **Do not claim consolidated verification** in the commit message or anywhere
   else. It has not run yet. Write `Harness, deferred to sprint gate` when the
   worker's harness result has not been re-established on the integrated tree.
4. Do not push. `/run-sprint` pushes once, after its gate and its review loop.

### Final sprint acceptance handoff

A `sprint-final-acceptance` handoff preserves named final checks in existing
plan/progress/run state. Consume its local scoped evidence, integrate as
reviewed and retain in-progress status. Do not mark a pending oracle passed or
unblock formal dependencies through this marker alone. Only run-sprint's
explicitly approved genuine LOCAL provider-prefix completion can satisfy that
edge while named final rider debt remains. Complete after the union of
actual final checks passes. This is distinct from external source availability.

### External CI source availability handoff

A handoff marked `external-ci-source-availability` uses the explicit
preparation exception in `/complete-feature`. Validate its local evidence and
named pending external gates instead of claiming complete scoped acceptance.
Integrate it in batch as reviewed, keeping its plan approved and its feature
in-progress. Preserve its pending gates in the existing plan, progress record
and run state before consuming the handoff. It does not permit a worker push
or feature completion. The orchestrator owns the source availability
checkpoint described in `/run-sprint`.

## Refused situations

- **Integrating without a validating handoff.** No exceptions.
- **Auto-resolving a semantic conflict.** Two plausible merges of a parser and
  a renderer produce a file that compiles and renders the wrong thing.
- **Deleting the worker branch or an unclean worktree.** Report the retained
  branch and any worktree that cannot be safely removed. `/close-sprint`
  removes completed worker branches after the sprint is pushed.
- **Pushing in `--batch`.**
- **Merging to `main` or tagging.** `/close-sprint` owns `main` and sprint
  tags. `/release` owns release tags.
