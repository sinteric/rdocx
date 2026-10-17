---
description: Implement a started feature against its design plan and test stubs.
---

# /implement-feature F-XXX

Turn the failing stubs from `/start-feature` into a passing implementation,
against the design plan as the contract.

## Steps

1. **Re-read the contract.** `.claude/plans/F-XXX-design.md` and its cited
   `docs/hld/` sections. Then `.claude/scratch/F-XXX-progress.md` if resuming.

2. **Confirm the state.** The F-ID is `in-progress`, the branch is `sprint/sNN`,
   and the stubs exist and fail.

3. **Work the checklist in order.** After each item, run the narrowest useful
   test: `cargo test -p <crate> --test <file>`. Not the workspace.

4. **Keep the progress file current.** Update
   `.claude/scratch/F-XXX-progress.md` whenever you stop, using the shape below.
   This is what makes a handoff or a resume cheap.

5. **Stay inside the plan.** If the implementation needs something the plan did
   not anticipate:
   - A small clarification: proceed and record it under "Deviations" for the
     AS_BUILT entry.
   - A change of approach, a new dependency, or a new public type: **stop and
     revise the plan** through `/design F-XXX --revise`. Do not let the code and
     the contract diverge silently.

6. **Update the HLD before freezing.** Execute exactly the approved plan's
   `## HLD impact` list during implementation. Describe implemented intent,
   without claiming pending oracle qualification passed. Source and HLD edits
   must be present for microscope and the final sprint gate.

7. **When the implementation checklist is done**, consume existing applicable
   local observations and run only missing `/verify --scoped F-XXX` checks.
   Obtain `/microscope F-XXX --working`. Do not run a whole suite or external
   oracle here. Record the final sprint checks still due. Source changes after
   review require affected checks and a review of the new material.

## Progress file shape

```markdown
# F-XXX progress notes

## Current state
What works, what does not, what is half-done.

## Changed areas
Files touched so far.

## Last green check
The exact command that last passed, and when.

## Blockers
What is stopping progress, if anything.

## Next action
The single next thing to do. Specific enough to act on without re-deriving it.
```

## Refused situations

- **The F-ID is not `in-progress`.** Run `/start-feature` first.
- **No design plan.** Run `/design`.
- **Changing approach without revising the plan.** The plan is a contract.
- **Committing.** That is `/complete-feature`.
- **Marking a checklist item done while its test fails.**
- **Updating the hash baseline to silence a delta.** Explain it or revert it.
