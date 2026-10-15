# F-X203 nested accepted-owner boundaries, pass 3

**Reviewed**: Immutable nested correction only. Four before/after carriers, two changed and two byte-identical. Incremental diff adds 112 lines and removes no lines. Other evolving F-X203 APIs are excluded.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Prior pass2 D1 resolved within this scope.

## Bound evidence

Receipt SHA `544a9b68ca44f2a584b6f57e627b5a6e0a57df8b5e828eb28120a2f69b5e1550` and all 17 declared artifacts authenticated. All four preimages equal immutable-boundary-correction-2 after-images. Incremental diff SHA `526c7ae78fa1b3217f1022d033bdf9d5528ccfaa3881abda9fa0193d0fa28155`. Full 3111-record map SHA `0b0a72b219856d0efd049e29139f22f22578dd788515a25739a027836b581022`.

## D1 disposition

`/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/immutable-nested-correction-3/after/crates/rdocx-oxml/src/text.rs:6039` recursively projects accepted paragraphs with their inner boundary offsets. The insertion/move-to helper at6062 uses this same projection. The control helper at6078 visits nested controls, paragraphs and revisions at their physical positions, retains both scope edges for nonempty accepted content, and removes provisional boundaries for empty output. The caller at6359 uses these helpers only for the boundary-preserving projection. Run indices and hyperlink index-vector lengths remain aligned.

`after/crates/rdocx-html/src/lib.rs:470` independently exercises outer bold A, nested control bold B, accepted insertion bold C and trailing bold D, expecting four separate emphasis spans. The valid insertion includes its required author. Existing whitespace, cached field line/column break and top-level owner assertions remain unchanged. The default accepted-view false branches retain their original helpers and operands. HTML emitter and Markdown carrier bytes are unchanged. No fake source run or XML write is introduced.

The approved structural-boundary contract remains `.claude/plans/F-X203-design.md:547`. These recursive scope edges close the previously demonstrated inner-boundary omission. Block table/row/cell projections remain separately bounded synthetic-inline fallbacks, with ordinary Markdown block visitation unchanged.

## Actual evidence and limits

Authenticated actual-native-api-3 receipt SHA `da7ddd05ae5144574fa87251656eae1e0077958fa916c8905c15df519351d783` records HTML14PASS/0FAIL/0IGNORE and seven native selectors PASS/0FAIL/0IGNORE. Logs match their declared hashes. Source-after matches and both groups naturally reached ESRCH with no signals. The seven-row native result includes later TextReport/Validation selectors, which are outside this source review and receive no independent source-audit claim here.

The prior malformed missing-author fixture failure remains retained in actual-native-api-2: HTML13PASS/1FAIL, with C absent from accepted output. It is not a genuine product regression Base. Its receipt still labels status running even though recorded child rows and cleanup are terminal. The reused four-crate all-target check row is actual0 with cleanup safe and production bytes equivalent across the final fixture-author correction. This review executes no commands and awards no new runtime or whole-feature completion credit.

## Findings

Defects: 0. Smells: 0. Nitpicks: 0.

## Not found

Correctness, contract, panics, OOXML, tests and structure: zero findings within the recursive accepted-owner correction. Default accepted-view consumers retain byte-equivalent implementation paths. Raw XML, metadata and caller-owned source are untouched.
