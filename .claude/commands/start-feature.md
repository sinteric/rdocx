---
description: Begin work on a designed feature. Marks it in-progress and creates test stubs.
---

# /start-feature F-XXX [--claimed]

Transition an approved design into an in-progress feature with failing test
stubs. **No implementation in this command.**

`--claimed` is what a worker runs first in a worktree that `/claim-feature`
created. The claim already set the trackers and cut the branch, so this becomes
the stub-creation half of the command.

## Steps

1. **Confirm the branch.** The current branch must be `sprint/sNN` matching the
   sprint in `docs/sprints/CURRENT_SPRINT.md`. If it is `main`, create the
   sprint branch first through `/sync-sprint`.

   Under `--claimed`, the branch must instead be `work/<fid-lower>-<agent>`,
   in the worktree the claim created, with F-XXX already `in-progress` and
   owned by that agent. If the branch is `sprint/sNN`, you are in the canonical
   worktree and the claim did not happen. Stop.

2. **Read the design plan.** `.claude/plans/F-XXX-design.md` must exist and its
   status must be `approved`.

3. **Validate the spec reference.** The plan's `## Spec reference` must name
   specific sections. **Refuse a whole-document citation** such as
   "see docs/hld/". This check exists because a vague reference is how a story
   silently drifts from its specification.

4. **Check the dependencies** are `done` in `docs/sprints/BACKLOG.md`.
   An approved provider design is insufficient. Inspect the plan's dependency
   readiness record and any required dependency-prefix checkpoint evidence.

5. **Check readiness and create the test stubs** from the plan's `## Test plan`. Each is a real test
   function that **fails for the right reason**, not `todo!()`. Confirm the plan's
   consumer map, A-ID acceptance rows and test matrix are concrete first. A
   new or revised plan must also satisfy `/design`'s validation checklist,
   including applicable behavior contracts and answered blocking decisions. A
   repaired contradiction must have an actual discriminator across its affected
   consumers or modes. Include empty cases and valid/refusal controls where
   applicable. Record the expected assertion before running old source. A stub that
   passes vacuously is worse than no stub.

   Prepare related cases sharing a test binary together when practical, then
   retain their individual outcomes from one focused baseline batch. A new API's
   absence at compile time is not a semantic RED. Use existing public providers
   to expose the underlying failure and keep additive API checks distinct.

6. **Update the trackers.** Set `in-progress` in both
   `docs/sprints/BACKLOG.md` and `docs/sprints/CURRENT_SPRINT.md`, and
   regenerate the AUTOGEN counts.

   **Skip this under `--claimed`.** The claim commit already did it, on the
   sprint branch. Doing it again here puts a tracker edit on the worker branch,
   which then conflicts with every other worker at integration time.

7. **Record the source and local RED.** Record the Base and actual failing
   regression commands in `.claude/scratch/F-XXX-progress.md`. Reuse existing
   baseline identities. Do not run hash/pixel corpora or external oracles here.
   Expected behavioral deltas remain declared and reviewed for the final gate.

## Refused situations

- **No design plan**, or its status is `draft`. Run `/design F-XXX` first.
- **The spec reference names a whole document.** Fix the plan.
- **A dependency is not `done`.** Name it and stop.
- **The F-ID is already `in-progress` or `done`.** If resuming, read
  `.claude/scratch/F-XXX-progress.md` and continue rather than restarting.
  Under `--claimed`, `in-progress` is the expected state, not a refusal.
- **On `main`.** Never start a feature on the default branch.
- **`--claimed` from the canonical sprint worktree.** Run `/claim-feature`
  first, from there, and then this from the new worktree.
