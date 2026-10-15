# F-X206 union adoption, pass 56

**Reviewed:** Incremental rollback correction in `/private/tmp/S91-F-X206-reviewed-union-adoption-r2-prepared`, receipt SHA-256 `67bc27138f38e37ca571d51260ff41f7ff13b42ec7f3496ad60148f51c599b90`. Corrected adopt.py SHA-256 `5cebb99bfdc57ef5503b4d8cbe58806ccb924bce0bec0b4f24fc8bca3fb5dc99`.

**Verdict:** 0 defects, 0 smells, 0 nitpicks in the incremental correction.

## Closure

Pass 55 D1 is closed at `/private/tmp/S91-F-X206-reviewed-union-adoption-r2-prepared/adopt.py:35`. The current carrier is now registered for restoration before either its byte write or chmod. A partial write or subsequent chmod error therefore includes that carrier in the existing preimage and mode restoration loop. The final before-map guard remains mandatory on the exceptional path. Restoration failure still propagates rather than claiming success.

The exact executable delta is this ordering change. All candidate bytes, preimages, manifest, prospective source map, prerequisite predicates and source/HEAD/index guards are unchanged. Every sealed artifact hash matches. All 2714 actual current source records still match the a9 baseline, including size and mode.

## Aspects

- Correctness: 0 findings. The failed current mutation is covered by restoration.
- Contract: 0 findings. Highlight and picture2 genuine Base predicates remain unchanged, with no production or acceptance credit inferred.
- Panics: 0 findings. The existing exception path now covers the in-flight carrier.
- OOXML: 0 findings. No candidate byte changes.
- Tests: 0 findings. No control or deciding assertion changes.
- Structure: 0 findings. No new API, dependency or abstraction.

## Authority and limits

This review authorizes the corrected R2 script only. The copied COMMANDS.md remains the parent recipe and names the immutable R1 directory. ROOT must substitute the R2 adopt.py path when invoking adoption, rather than execute the superseded R1 script. No packet file was corrected by this review.

Pass 55's unaffected incremental checks and the prior independently reviewed union semantics remain the parent authority. No adoption, fault injection, Cargo, source, cache, environment or runtime operation was performed. Future picture evidence and ROOT source admission remain required. Only this report was written. Return the review-only lease to ROOT.
