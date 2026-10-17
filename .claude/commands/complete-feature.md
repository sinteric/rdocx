---
description: Finish a feature. Updates every tracker, the HLD, and commits to the sprint branch.
---

# /complete-feature F-XXX [--prepare]

Close out an F-ID. Every tracker is updated in one commit so the delivery record
cannot drift from the code.

`--prepare` is the parallel-work variant a worker runs in its own worktree. It
stops short of the sprint ledgers and writes a handoff instead. See "Prepare
mode" below.

## Steps

1. **Check the preconditions.**
   - The F-ID is `in-progress`.
   - `.claude/plans/F-XXX-design.md` exists and every local implementation item
     is ticked. Final sprint items may remain explicitly pending as below.
   - The latest `.claude/reviews/F-XXX-*` pass reports **zero defects and zero
     smells**.
   - Actual `/verify --scoped F-XXX` local evidence and clean microscope apply
     to the final feature source. Consume these observations, do not rerun them.
     Named final sprint checks remain due at the integrated gate.

   If any fails, say which and stop. Before normal completion steps4 onward,
   require actual final checks to have passed, except the narrow approved local
   provider-prefix checkpoint below. Otherwise prepare only the explicit final
   sprint acceptance checkpoint, without delivery completion.


2. **Confirm the test gate evidence is real.** Read the retained named local
   old-source RED and fixed-source GREEN observations. Run only a missing or
   invalidated check. Do not repeat a passing test just because this command
   was invoked. A gate that passes against unmodified code is not a gate.

3. **Confirm the HLD is current.** Its exact approved impact files must already
   be included in implementation and review. Do not edit source intent after
   final verification. An unlisted contradiction requires a plan revision.

4. **Append to `docs/sprints/AS_BUILT.md`** using the template in that file.
   The `**Hash harness**` field is mandatory for M1 through M6, recording either
   "unchanged" or the delta and its justification.

5. **Append to `docs/sprints/SPRINT_TRACKER.md`**: one row with the estimate and
   the actual.

6. **Set `done`** in `docs/sprints/BACKLOG.md` and
   `docs/sprints/CURRENT_SPRINT.md`, and regenerate the AUTOGEN counts.

7. **Set the design plan status** to `completed`.

8. **Delete** `.claude/scratch/F-XXX-progress.md` if present. It is in-flight
   memory and its job is over.

9. **Commit** to the sprint branch:

   ```
   F-XXX, short title

   One paragraph: what was built, why, and any non-obvious choices.

   Tests, <the gate plus any others>
   Harness, unchanged | <delta and justification>
   ```

   **No `Co-Authored-By` trailer.**

10. **Report** what changed, and the next pending F-ID in the sprint.

## Prepare mode

`--prepare` runs in a worker worktree on `work/<fid-lower>-<agent>`. The worker
owns its feature. It does not own the sprint's totals, and two workers appending
to `AS_BUILT.md` in parallel is a conflict in the one file that must not have
one.

Do steps 1 through 3. Set the plan completed only after actual final acceptance.
When final checks remain, use the checkpoint below and keep the plan approved.
Instead of steps 4 to 6 and 9:

1. **Write `.claude/handoffs/F-XXX-ready.md`**, exactly these fields:

   ```markdown
   # F-XXX ready for integration

   **F-ID**: F-XXX
   **Owner**: claude | codex
   **Branch**: work/f-xxx-<agent>
   **Worktree**: <path>
   **Base**: <the sha the claim recorded>
   **Head**: <the sha this commit will be>
   **Design plan**: .claude/plans/F-XXX-design.md
   **Microscope**: .claude/reviews/F-XXX-<aspect>-pass-N.md, 0 defects, 0 smells
   **Verify**: /verify --scoped F-XXX, pass
   **Hash harness**: unchanged | <delta and its justification>
   **Test gate**: <test name>, pass
   ```

2. **Validate it before committing.** A handoff that does not validate is not
   a handoff:

   ```bash
   python3 scripts/sprint_workflow.py validate-handoff \
     .claude/handoffs/F-XXX-ready.md --fid F-XXX
   ```

3. **Commit feature-local work only** to the worker branch: the code, the
   tests, the design plan, the review files, the HLD files the plan listed, and
   the handoff. Use the normal message format.

4. **Do not** touch `AS_BUILT.md`, `SPRINT_TRACKER.md`, `BACKLOG.md` or
   `CURRENT_SPRINT.md`. F-XXX stays `in-progress` with its `Owner`.

5. **Do not** delete `.claude/scratch/F-XXX-progress.md`. Integration has not
   happened, so the in-flight memory is still live.

6. **Do not push.**

Report the branch, the head sha, and the exact integration command:

```text
/integrate-feature F-XXX work/<fid-lower>-<agent>
```

### Final sprint acceptance preparation

When implementation, scoped local regressions and independent microscope pass,
prepare integration without pretending final oracles ran. Keep the plan
approved, final checklist items unticked and the feature in-progress. Use:

```
**Checkpoint**: sprint-final-acceptance
**Verify**: /verify --scoped F-XXX, local checks pass, final sprint checks pending
**Test gate**: <named actual local regression checks>, pass
**Final checks**: <named cases/commands from the approved plan>, pending
**Hash harness**: deferred to final sprint gate, expected unchanged | <declared delta>
```

Preserve this debt in the existing plan, progress and run state before consuming
its handoff. Integrate as reviewed, not completed. Final integrated acceptance
must actually pass before delivery ledgers, completed status or successful
oracle claims. Do not combine this marker with the external CI checkpoint.

### Approved local provider-prefix completion

Only an explicitly approved dependency-prefix checkpoint may record provider
implementation completion before final qualification. Require its named genuine
LOCAL provider gate to have actually passed, applicable scoped evidence and a
clean independent microscope, and the integrated source commit to be an ancestor
of the consumer's Base. Mark the provider completed in the existing sprint run
state and done in BACKLOG/CURRENT_SPRINT to satisfy the formal dependency edge.
Keep its plan approved and preserve every named final rider in the existing
plan/progress record. Do not append AS_BUILT or SPRINT_TRACKER, complete its
plan or claim final oracle acceptance here. The actual final union supplies
those records exactly once for every integrated provider, including prefixes.
A provider whose required dependency gate is itself an unperformed oracle
cannot use this exception. Reviewed status alone never supplies completion.

### Release preparation

A release F-ID whose gate requires real publication has one narrow exception.
Its worker may prepare the reviewed release machinery before the irreversible
approval and publication checklist items are complete. All implementation
items, focused workflow regressions, scoped local evidence and `/microscope`
must pass. Hash and oracle obligations remain in the final sprint gate. Leave the plan `approved`, record the handoff test gate as
`deferred to /release vX.Y.Z`, and integrate it in batch as `reviewed` rather
than `completed`.

After the integrated sprint passes `/verify --full` and `/sprint-review`,
`/release` obtains the separate final approval and proves the external gate.
Only then tick the remaining checklist items, set the plan to completed and
write the delivery ledgers. This exception does not apply to a dry-run-only
test gate or to any non-release F-ID.

### External CI source availability preparation

An approved feature whose new workflow must be available remotely to execute
its actual external CI gate may prepare a source availability handoff. This
is preparation for acceptance, not feature completion. All implementation,
focused local checks and a zero-finding independent microscope must pass.
Preserve final sprint riders separately without combining checkpoint markers. Leave the plan approved, its external CI
checklist items unticked and the feature in-progress. Do not write delivery
ledgers or unblock dependent work.

Use the ordinary handoff fields and add these exact fields:

```
**Checkpoint**: external-ci-source-availability
**External gates**: <named actual external CI gates>, pending
**Verify**: /verify --scoped F-XXX, local checks pass, external CI pending
**Test gate**: <actual local regression checks>, pass, external CI pending
```

The validator and integrator must refuse pending external CI without this
marker and named gates, an unknown checkpoint, or a claim that the complete
scoped gate passed. Preserve exact local evidence and pending obligations in
the existing plan, progress record and sprint run state. Follow the checkpoint
in `/run-sprint` after batch integration. Only actual external evidence at the
reviewed remote SHA permits ordinary feature completion.

## Refused situations

- **Any precondition in step 1 fails.** Name it and stop.
- **The latest review still has defects or smells.** Fix them and re-run
  `/microscope`.
- **The test gate does not fail against reverted code.** The gate is wrong. Fix
  it before completing.
- **Updating an HLD file the design plan did not list.** Either the plan was
  incomplete or the implementation drifted. Resolve it, do not paper over it.
- **Pushing, merging or tagging.** This command commits to the sprint branch
  only. Only `/close-sprint` touches `main`.
- **Completing an F-ID whose hash-harness delta was never declared.**
- **`--prepare` from the canonical sprint worktree.** It exists for a claimed
  worker branch. Run the normal command instead.
- **A `--prepare` handoff without `/verify --scoped F-XXX`.** `--fast` is the
  inner loop. `validate-handoff` refuses it, and so should you.
- **Using release preparation to claim publication succeeded.** The F-ID stays
  reviewed until `/release` verifies the registry and GitHub release.
