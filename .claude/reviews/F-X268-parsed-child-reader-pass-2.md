# F-X268, parsed child reader, pass 2

**Reviewed**: Codex root reviewed D1 and S1 remediation on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The current three-file working diff contains 1084 added and 39 removed lines, including the separately reviewed serializer and write prerequisites. This is the bounded mechanism 7B1 reader review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Resolved findings

D1 is resolved. The empty-body frame helper no longer increments the child-read counter. The six slots retain their units. Frame parsing, shell copying and shell serialization remain separate costs. The new empty-body regression observed actual pre-fix RED, 1 child read against 0, and now passes.

S1 is resolved. The two proved safe children require exactly zero warm reads. Two successive field-bearing reads require exactly two child attempts while the safe neighbor stays cached. Functional model, saved-byte and fallback comparisons remain active in release.

## Evidence and limits

`/tmp/fx268-m7-read-model-remediation-red.log` records 1 passed and 1 failed before the counter fix. The warm safe assertion had passed. The field exact-2 assertion was not reached before that failure, so it has no historical RED claim.

`/tmp/fx268-m7-read-model-remediation-test.log` records 12 passed and 0 failed in 19.01s. Each measured table-row edit records one child write and one child read at both 50 and 200 rows. Reopen ZIP writes and main-document parses are zero. Parsed, namespace-scanned, canonicalized and identifier byte work still grows. The scoped all-target check and clippy pass. Release test targets type-check, with the earlier addressing code's unused child_count warning. This is not release behavior execution.

## Not found

No additional parsed-model, context-selection, output-byte, checked-fallback, panic or structural finding was established in the remediation diff. The actual retained frame is an empty-body shell and its comments now say so.

Namespace, identifier and canonicalization optimization, complete native A10, first-four byte bounds, Python rebuild, HLD, whole-feature composition, release behavior, timing, memory, Word and final gates remain pending. This is not mechanism 7 acceptance or a whole-feature microscope.
