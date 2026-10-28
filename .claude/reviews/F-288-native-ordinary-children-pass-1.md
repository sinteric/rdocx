# F-288, native ordinary children, pass 1

**Reviewer:** Independent Codex reviewer `/root/review_shared_proof_batch`.
**Reviewed:** Frozen Q25/Q27 working changes against `85bd31e8f99b4dbabbe01e3333b8c2dd13f88b7e`. Eight existing files, 1114 insertions and63 deletions, excluding Root's separate PowerPoint reference-admission hunk in HLD12. Product behavior changes only in `crates/rdocx-oxml/src/text.rs`. Read the canonical microscope guidance, approved Q25/Q27 plan, progress, earlier bookmark reviews and applicable HLD changes. No source changes, builds, tests, rendering or native UI were performed.
**Verdict:** One defect, zero smells, zero nitpicks. Return for a separate remediation phase. This is not full native A6 or release acceptance.

## Defects

### D1, alternate-content drawing owners bypass the new private-owner guard

`crates/rdocx-oxml/src/text.rs:6247`

The new actual-source guard reparses each interior run but inspects only `CT_R.content`. Supported `mc:AlternateContent` drawings are deliberately stored outside that vector in `CT_R.alt_drawings` (`crates/rdocx-oxml/src/text.rs:3735`). Thus an otherwise eligible own-bookmarked FORMTEXT with its one ordinary child and an additional interior run containing a supported AlternateContent drawing passes the new guard, including when that run is in the cached result. The existing valid DrawingML/VML fixture at `crates/rdocx-oxml/src/text.rs:17935` supplies a concrete admitted wrapper with an actual typed shape/text-box owner.

This matters beyond preserving raw ZIP bytes. The complex-field event reader skips the AlternateContent wrapper instead of projecting its drawing (`crates/rdocx-oxml/src/text.rs:5715`), and rebuilt cached runs carry empty `alt_drawings` (`crates/rdocx-oxml/src/text.rs:5910`). Collapsing the original run span therefore removes the typed drawing from the observable model/layout. Layout explicitly consumes these sidecar drawings (`crates/rdocx-layout/src/engine.rs:766`). Before this exception the nested own-bookmarked shape retained the untyped source and its original runs. The approved bounded exception requires additional typed/private interior owners to retain that fallback.

Reject actual nonempty `alt_drawings` alongside the existing content check, without classifying arbitrary opaque raw XML as a drawing. Add the supported AlternateContent representation to the negative matrix for the relevant child regions, and assert the outer remains untyped, the original drawing owner remains discoverable, and serialization preserves the raw wrapper. Retain a compatible opaque unknown-child control. This is a source-derived counterexample, not an executed RED. No runtime failure is claimed by this reviewer.

## Smells

Zero.

## Nitpicks

Zero.

## Examined behavior and evidence

The bounded helper distinguishes exactly one direct result child from exactly one Nested instruction argument, rejects extra operands/nested switch children, restricts ordinary complex leaves and checks legacy ownership and descendants (`crates/rdocx-oxml/src/text.rs:6168`). Existing pure outer boundaries, bookmark uniqueness, actual namespace-frame guards and three-axis adjustment remain in place. The new simple-child raw-source check addresses the demonstrated model omission. D1 is a different supported representation omitted by its new private-owner check.

The canonical fallback refusal is placed after the unchanged/source-preserving writer paths and before reconstruction (`crates/rdocx-oxml/src/text.rs:11167`). Its own-name scan resolves actual nested namespace frames rather than matching foreign local names (`crates/rdocx-oxml/src/text.rs:11060`). The checked legacy setter compares serialized original/candidate bytes before publishing (`crates/rdocx-oxml/src/text.rs:2051`). Existing instruction source identity, defaults, private cache fallback and evaluation policy are unchanged. Result-child literal removal and instruction-child retention are independently asserted before subsequent ordinary updates. No additional correctness or error-order finding was identified in these examined paths.

Consumed actual logs, artifact authentication and raw receipt contents rather than adding the aggregate selections together. The16 native OXML and51 field OXML selections overlap in two tests, so they represent65 unique OXML tests. The facade selection passed56. Python passed37 on the isolated debug module `b12c12ee24df64bece9328932728bacf3bc0b1258c0e0236b2c1cfafa243e139`. The native six-file result has six distinct inputs and ten fresh/reopened entries each. The separate stored-XML oracle matches the restored fields in both native originals, including outer `5.00` and default `=2+3`, without interpreting inner instrText as display. These receipts support that concrete inventory repair, not D1 coverage or evaluated-output parity.

The source-shaped instruction RED, genuine replacement-child/bookmark RED, simple-child negative and prior native seven-of-ten discrepancy remain distinct from compilation/setup failures and the corrected post-update fixture expectation. Scoped three-crate clippy, format/prose and diff receipts passed. No normal-release rebuild, CLI acceptance, full integrated hash result or final native A6 is inferred from the debug artifact. Normal398a remains historical for these new semantics.

All eight source/document hashes match `/private/tmp/s954-q27-frozen-hashes.json` at review. Product `text.rs` SHA-256 is `6f67ff8d1c38d08efa3e77ce9347680bb3ee388ee74eb12654655aeb251a82bc`. Native integration source is `8ea181df4fbf10c075bc0190fd12818ec0d2c2c3e0bc6982f03db82c9523d34f`. Python test source is `59574131a767a6530138f45837e275a9b7b886c215796a2734b7e8c6bef511d3`. The five HLD hunks describe the bounded owner policy and retain qualification debt. Root's PowerPoint changes were excluded.

## Not found

Contract beyond D1: zero additional findings. Panics/indexing/arithmetic: zero. Namespace/schema/opaque preservation beyond D1: zero additional findings. Tests beyond D1: zero additional local findings. Public API, bindings surface and structure: zero. No new module, dependency, persistent cache or evaluator was introduced. Final native A6, dense-form A7 and integrated sprint/release riders remain separate pending obligations.
