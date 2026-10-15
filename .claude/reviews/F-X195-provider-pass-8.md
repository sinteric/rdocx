# F-X195, incremental provider, pass 8

**Reviewed**: One existing Word test function, 98 additions and 60 deletions.
Exact test-only diff SHA256
57866735fc441d872728b6c687ad563534e053b6614d2bb83f814edf27766676.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Runtime retry remains pending.

## Identity and scope

Worker work/f-x195-codex, HEAD
2a9bf14072231e46fcb34df70f88cf7aa79fbd79. F-X195 remains in-progress.
Read repository workflow, microscope contract, approved provider acceptance,
cache and retention implementation, progress and prior provider pass6/core
pass7. Only this incremental Word test correction is reviewed here.

Proposal receipt SHA256
665791a917fc6832734095554aaa74ab0a3a01c430e18c2f0215a7c53a999006,
applied receipt SHA256
34679bae99ed381942eda515e3a19ca4318c53c1b4a8667520dd73234cdbf8b8.
Current engine.rs matches the proposed candidate exactly, SHA256
eaf2de84c7ddbd2b8b5efb9f9aa5da9f8d9a14694f2d38fa7a4851559484b403.
Independent inverse application in memory recovers the exact prior engine
SHA256 6e8c0f5da03181ddcb7d78df9c1329a1a2a83f12ffc099758eaeb3a6de17e9b0.
The production prefix, suffix and eviction policy are unchanged. No source,
formatting, scratch, Cargo or runtime action occurred in this review.

## Assessment

The prior single-request layouts did not keep the other request-specific
FontIds active. Existing pruning at crates/rdocx-layout/src/engine.rs:3827
and retain_current_fonts at crates/oxml-layout/src/font.rs:1346 remove
inactive entries and faces. One loaded face per request key remains the
established policy at crates/oxml-layout/src/font.rs:1674. A warm revisit
cannot require retention of the original inactive working set.

The correction at crates/rdocx-layout/src/engine.rs:21568 keeps three actual
Latin paragraphs active in both Aptos/Calibri/Aptos Display orders, using
one engine per order. It selects cache entries by actual paragraph equality,
requires one entry and nonempty contributing events per original request,
and compares real physical face data without conflating transient request
IDs. Same-fallback identity and request inequality remain distinguishing
assertions at crates/rdocx-layout/src/engine.rs:21642.

Both warm revisits require exactly three additional hits and no additional
builds at crates/rdocx-layout/src/engine.rs:21657. This is an actual counter
incremented by the validated replay branch at
crates/rdocx-layout/src/engine.rs:5132. Cache cardinality and current-font
inventory are also checked. Full layout equality uses the existing helper
at crates/rdocx-layout/src/engine.rs:18456, including final debug equality.
Exact ordered event carriers, selected IDs and original request attribution
must match the cold capture at crates/rdocx-layout/src/engine.rs:21670.
The controls do not add a multilingual paragraph-cache hit claim or weaken
cache ceilings, eviction or overflow behavior.

Original Word child101, six controls with five passing and one failing,
remains retained in the R8 remaining receipt and log. The corrected control
has not run. This review supplies source acceptance for the bounded fixture
correction, not feature completion or a retrospective passing result.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

- correctness: 0 findings. The active working set agrees with unchanged retention.
- contract: 0 findings. Both request orders retain genuine warm/cold and attribution controls.
- panics: 0 findings. Fixed three-paragraph construction and checked entry count precede indexing.
- ooxml: 0 findings. Existing typed paragraph builders suffice and no parser or serializer changes.
- tests: 0 findings. Actual hits, unchanged builds, ordered events and full output equality distinguish reuse.
- structure: 0 findings. One existing test function and existing helpers, with no new module or dependency.

This ends the review pass. Runtime, historical reversal and complete wiring
remain separate gates.
