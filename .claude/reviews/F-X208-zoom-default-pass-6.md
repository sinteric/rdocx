# F-X208, zoom default remediation, pass 6

**Reviewed**: One immutable timeline carrier against authenticated source19, 13 added lines and 1 removed lines. Source22 uncompressed SHA-256 a53da4b59ef09d5b489c096fdf6d88259725e602eb119c32b0d275412c4a9b56. This is the narrow remediation of pass5 D1, not a complete feature review.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None. Pass5 D1 is resolved. `crates/rpptx-render/src/timeline.rs:334` now selects out for an absent direction, matching the native authoring default. Explicit in and out retain their distinct transformations. Optional direction presence is untouched because this changes only composition.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panic/error handling, OOXML preservation, tests and structure have zero findings in this bounded diff. The existing discriminating test at `crates/rpptx-render/src/timeline.rs:1077` retains the explicit in/out scale assertions and adds an absent-direction frame equal to explicit out at half progress at line1124. Reverting only the default changes which page scales and fails the new comparison.

## Evidence and limits

Authenticated capsule `/private/tmp/S91-F-X208-zoom-correction-source22/receipt.json` and compressed carrier hashes and lengths were checked independently. The actual test receipt SHA-256 db88877ef417b83fe21e30f9a26cb441177074b098737549b4860bf7920b551e and raw log fa6bca6b5d7f55e6658ff75149c8b1296648e8583a2b0e6e005c68e07f4948b0 were authenticated. The named exact control ran once and passed with zero failed or ignored tests. Source guards remained exact and its owned group was naturally gone and reaped. No tests, imports or source changes ran during this audit. Subsequent worker changes, fresh installed native renderer, CLI and Office acceptance remain outside this review.
