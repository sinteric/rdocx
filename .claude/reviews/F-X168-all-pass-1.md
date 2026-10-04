# F-X168, all, pass 1

**Reviewed**: working diff against `f9970d3a`, five files, 125 insertions and 14 deletions.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, The Issue 160 evidence row attributes comments-part byte preservation to the matrix

`docs/hld/12-testing-strategy.md:3502`

The producer matrix's no-op assertion compares `word/document.xml` only. It
does not compare the bytes of `word/comments.xml`. The separate
`empty_comments_part_keeps_its_bytes_and_compares_against_its_own_save` test
does that work. The ledger must name the correct test and the final gate must
include it before Issue 160 closes.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: the PR 265 regression checks the reported section ownership across direct and control-owned blocks.
- Contract: the worker diff leaves production section selection unchanged and defers full verification and live closure to the S84 boundary.
- Panics: no production path changed, and fixture parsing uses static input.
- OOXML: no parser or writer changed. The synthetic section properties follow the existing test grammar.
- Tests: the mixed-section gate fails with nine versus seven pages when section selection is forced back to the final section and passes when F-X162 is restored.
- Structure: no new trait, generic, module, crate or source file was added.
