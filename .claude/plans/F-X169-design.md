# F-X169, Reconcile live contributions and open issue contracts

**Status**: completed
**Sprint**: S84
**Size**: L
**Depends on**: F-X160

## Problem

The 2 October intake still has 31 open PRs, although 19 map to completed
contribution stories. Every current head conflicts with main and no PR has a
submitted review (`docs/sprints/SPRINT_PLAN.md:1741`). Eight issues still have
distinct acceptance criteria (`docs/sprints/SPRINT_PLAN.md:1807`). A head's
green CI result does not establish that its increment is absent from main or
that the combined queue is safe to replay.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X169, Reconcile live contributions and open issue contracts".
- `docs/hld/12-testing-strategy.md`, "S82 original issue closure ledger" and "Binding tests".
- `docs/hld/04-opc-and-packaging.md`, "The package".

## Approach

Refresh every open PR and issue from GitHub, including bodies, discussion,
reviews, current head checks, changed files and stacked commits. Compare the
older PR increments with the completed S76 to S83 code and AS_BUILT evidence.
Record for each PR whether its increment is already present, must be replayed,
or requires a contributor rebase. Keep its exact issue coverage and any
unresolved criterion in the single live sprint-plan inventory. Record the
dependency graph and exclusive source, test, documentation and baseline files
before worker claims. Update the existing M23 and M24 schedule assertion in
`scripts/test_sprint_workflow.py` to the revised whole-number sprint range.
This story changes planning, evidence and its schedule test, not product source.

## Rejected alternatives

- Merge every green PR head. The heads conflict with main and stacked branches
  contain earlier work already integrated.
- Treat a closed predecessor issue as proof for an open issue. The open issue's
  own acceptance criteria still need evidence.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| integration | Compare `gh pr list --state open` and `gh issue list --state open` with the sprint-plan inventory | Every live PR and issue has one row and no stale item is treated as current |
| integration | Check each older PR increment against integrated main and its completed F-ID record | Every replay or supersession decision has a cited commit or diff |
| integration | Check stack and overlap edges against current head commits and changed files | No dependent increment starts before its prerequisite, and baseline owners are exclusive |
| regression | `test_m23_m24_roadmap_has_no_duplicate_or_dangling_story` | The revised S85 to S90 assignments agree across the plan, backlog and HLD |
| regression | `python3 scripts/prose_check.py` | The revised tracked records pass voice rules |

**Test gate**: integration. Every open PR and issue has a current disposition,
every stacked dependency has an order, and the sprint plan agrees with the
backlog and live GitHub inventory.

## HLD impact

None. This is intake evidence in the canonical sprint plan, not a product
specification change.

## Risk routing

None. No parser, serializer, public API or source file changes in this F-ID.

## Hash harness

Unchanged. No rendered or serialized output changes in this F-ID.

## Implementation checklist

- [x] Refresh all live PR and issue facts and discussion at the intake cutoff.
- [x] Verify the 19 older PR increments against main and completed F-ID evidence.
- [x] Record dispositions, stack order, overlap and acceptance coverage for all
  31 PRs and eight issues in the single sprint-plan inventory.
- [x] Update the existing roadmap schedule assertion for S85 to S90.
- [x] Confirm one owner per baseline and source or test conflict, then pass the
  integration gate, scoped verification and microscope.

## Open questions

None. The user selected one integrated S84 repair sprint and full closure.
