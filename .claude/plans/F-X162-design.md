# F-X162, Per-paragraph section width and pagination

**Status**: completed
**Sprint**: S84
**Size**: M
**Depends on**: F-X161

## Problem

`layout_transaction` can choose the prior or final section width for a body
paragraph instead of its governing section
(`crates/rdocx-layout/src/engine.rs:1998`). Mixed-width sections can therefore
wrap text and tables at the wrong right edge.

## Spec reference

- `docs/hld/08-rendering-spec.md`, "Word section geometry and page numbering".
- `docs/hld/14-development-backlog.md`, "F-X162, Per-paragraph section width and pagination".

## Approach

Review PR 241 after F-X161. Determine each direct body item's governing
`sectPr` before layout, including paragraphs, tables and body content controls.
A paragraph's section break governs that paragraph and preceding content since
the last break. The final body `sectPr` governs the last section. Do not let
section properties inside cells or text boxes replace the main story's width.

## Rejected alternatives

- Use the final section width for the whole document. It reproduces the bug.
- Change unit conversion while fixing section selection. Truncation is pinned
  separately and would make the output delta ambiguous.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| unit | Mixed-section paragraph, table and body-control cases | Each item uses its governing width and stays within the right edge |
| golden | Three sections at 144, 252 and 360 pt with deterministic fonts | Line counts and page boundaries match pinned Word output |
| golden | Continuous break and two-column cases | Section transition and column measure remain correct |
| harness | `python3 scripts/hash_harness.py --check` after its own baseline review | Only the declared section-layout output changes |

**Test gate**: golden. Mixed-section paragraphs, tables and page boundaries
match the pinned Word render with deterministic fonts and declared hashes.

## HLD impact

- `docs/hld/08-rendering-spec.md`, section width selection during layout.

## Risk routing

- Layout and pagination: read `docs/hld/08-rendering-spec.md`. Use
  deterministic fonts and record the baseline deliberately.
- External oracle: read `.claude/skills/differential-testing.md`. Pin the Word
  version and record the comparison measure and tolerance.

## Hash harness

Expect PR 241's section-dependent `feature_showcase` PDF changes, subject to
remeasurement after F-X161. Do not move the F-X161 baseline in this commit.
No golden PNG change is expected from this increment.

## Implementation checklist

- [x] Rebase and review PR 241's incremental section change after F-X161.
- [x] Assign the governing section to every main body item before layout.
- [x] Run mixed-section, continuous-break and two-column focused cases.
- [x] Review the separate deterministic hash delta.
- [x] Pass focused tests, risk riders, scoped verification and microscope.

## Open questions

None. Mirror-gutter width and other unreported section semantics remain
outside this F-ID unless its stated golden gate exposes them.
