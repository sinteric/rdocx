# F-X163, Plain-line trailing-space fit

**Status**: completed
**Sprint**: S84
**Size**: M
**Depends on**: F-X146, F-X162

## Problem

The plain line fit still counts terminal U+0020 spaces at a word boundary
(`crates/oxml-layout/src/line.rs:442`). It wraps a word early at Issue 226's
2.10-inch width. Integrated PR 222 already fixes the reported explicit
`w:rtl=false` rich-line punctuation path.

## Spec reference

- `docs/hld/08-rendering-spec.md`, "Word revision views" and "The renderer's input".
- `docs/hld/14-development-backlog.md`, "F-X163, Plain-line trailing-space fit".

## Approach

Replay only PR 242's plain-path increment after F-X162, since its PR 222 base
is already integrated. Exclude terminal U+0020 advances from plain fit and
hyphenation decisions, while retaining those spaces as hung source content for
alignment and decoration. Keep NBSP, tabs, markers and RTL cases distinct.
Check both Issue 226 paths against pinned Word and LibreOffice output.

## Rejected alternatives

- Trim trailing spaces from output. That loses source text and spacing facts.
- Apply the plain-path rule to NBSP. It has different break behavior.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| golden | Issue 226 at 1.70 and 2.10 inches, plain and explicit `rtl=false` | Both paths match reported lines with no leading space or punctuation |
| unit | One or three trailing spaces, NBSP, ligature, justification and decoration | Hanging changes fit without losing source spans or changing guarded cases |
| regression | PowerPoint distributed alignment and Word plain line cases | Shared line-breaking code does not regress presentation output |
| harness | Hash and golden pixel checks after labelled baseline review | Only the declared seven PDF entries and two PNG cases move |

**Test gate**: golden. Both paths break at Word's reported widths without a
leading space or punctuation error, and all expected deltas are reviewed.

## HLD impact

- `docs/hld/08-rendering-spec.md`, plain and rich line fit behavior.

## Risk routing

- Layout and line breaking: read `docs/hld/08-rendering-spec.md`. Use
  deterministic fonts for every baseline and deliberately record changes.
- External oracle: read `.claude/skills/differential-testing.md`. Pin Word,
  LibreOffice and Poppler versions, DPI and any pixel tolerance.

## Hash harness

Expect seven PDF entries and the contract and invoice golden PNGs from PR 242,
remeasured after F-X162. No OOXML part delta is expected. Keep this change in
its own labelled baseline commit.

## Implementation checklist

- [x] Review only PR 242's increment beyond integrated PR 222.
- [x] Correct plain fit and hyphenation while preserving hung source spaces.
- [x] Run both Issue 226 reproducers and guarded formatting cases.
- [x] Review the separate hash and golden pixel changes.
- [x] Pass focused tests, risk riders, scoped verification and microscope.

## Open questions

None. Rich-path decoration and unrelated shaping differences are outside
Issue 226's two stated symptoms unless its exact line gate finds them.
