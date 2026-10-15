# F-X206, run-link relationship pruning, pass33

**Reviewed**: Bounded native setter slice of the external run-target proposal.
**Verdict**: 1 defect,0 smells,0 nitpicks. Unadopted and unexecuted.

## Defects

### D1, clearing a click without r:id skips its sound relationship

`/private/tmp/S91-F-X206-run-target-slide-production-r1-prepared/candidates/crates/rpptx/src/lib.rs:4343`
gates before/after relationship pruning on old.is_some(). At line4269 old
is only the optional nonempty hlinkClick relationship_id, not presence of
the click. A valid imported action-only hlinkClick can omit r:id and contain
an a:snd that references an audio relationship. Clearing that click removes
the sound reference, but old is None and pruning never runs. The audio
relationship remains unreferenced, contradicting the setter contract and
approved owner-relationship acceptance.

Track whether an old click existed independently of its primary r:id, then
perform the existing staged before/after reference comparison when necessary.
Add an existing-API control with canonical namespace, no primary r:id and
a valid click sound. Establish positive package/read/source setup before
clearing, verify the sound-only relationship is removed, preserve any shared
sound relationship, and retain no-op and failed-owner atomicity controls.
Namespace provenance repair remains a separate prerequisite.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

No additional finding is claimed outside this bounded relationship-pruning
slice. The whole core and namespace proposal remain unsealed and unadopted.
Compilation, Base, GREEN, full imported preservation and scoped acceptance
remain mandatory. No code was changed during this review.

## Review boundary

Only this review was written. Return to orchestration for separate remediation.
