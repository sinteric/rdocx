# F-X195, current remaining runtime, pass61

**Reviewed**: Actual R38 ROOT preflight following pass59. Session20614 terminated1 before attempt creation or child execution.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, raw producer receipt is compared to an annotated validator result

`/private/tmp/S91-F-X195-current-r38-remaining-prepared/run.py:75`

The SVG latest-producer branch loads raw immutable receipt bytes through exact(), then compares the dictionary with prior_receipt() output. Recursive validation enriches prerequisite dictionaries with buildValue, so a valid authenticated original receipt differs from that in-memory result. The actual preflight fails before runtime admission. Authenticate raw receipt identity and raw content separately while retaining recursive prior validation. Do not mutate retained receipt files or weaken producer provenance.

## Smells

None.

## Nitpicks

None.

## Not found

No Cargo, native test, source or environment operation occurred. The assertion precedes attempt creation. Caller19, full104 and nativeSVG remain valid actual successes. This failure supplies no remaining gate credit.

## Handback

Only this report was written. Original proposal and actual admission remain immutable. Return to orchestration for an external incremental correction and fresh independent review.
