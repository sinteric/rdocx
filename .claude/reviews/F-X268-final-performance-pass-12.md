# F-X268, final performance, pass 12

**Reviewed**: Frozen working diff against `461eee97e1aa58b460bce4658f5a58453eaf9351` on `sprint/s95.4`. Six files, 1487 insertions and 147 deletions. Focused product remediation after the pass 11 source review, against the same approved contract and canonical workflow. The changed logic passes existing normalized writer pieces separately into staged opening-proof construction.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Runtime qualification remains pending.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Safety assessment

`crates/rdocx/src/document.rs:23287` adds a private `writer_blocks` parameter to the existing opening helper. Its ordinary opening caller supplies `None`. The staged in-memory caller at `crates/rdocx/src/document.rs:24059` supplies the existing `carried_scans.serialized` record. No new cache field, public API, source file or module is introduced.

Source/read authority remains separate. The actual opened document still comes from `blocks.proved_document`, a qualified unchanged opening, or the full XML parser. Namespace-owner facts and identifier hints still consume the source record. The new writer argument is used only for constructing the normalized serialization proof at `crates/rdocx/src/document.rs:23492`. It cannot establish that producer bytes parse to a model or that source bytes equal normalized output.

Resource validation still precedes proof construction. `MainPartBlocks::serialize` receives the actual newly opened document and its actual body content, with the writer record as an optional reuse hint. The existing serializer at `crates/rdocx/src/document.rs:8838` checks staged child alignment, requires `owner_facts_reusable_by_value`, excludes field-bearing bytes, and writes every unqualified child through `body_item_xml`. That writer performs model admission and current root binding setup. Private tab, bookmark and field cases do not acquire new value-only eligibility.

The serializer builds the current frame, compares the retained root when any bytes were reused, and falls back to serialization without retained pieces if that comparison fails. Changed head or tail gives fresh context-dependent facts. The carried writer record is the same existing layer used by `serialized_main_part` at `crates/rdocx/src/document.rs:24099`, rather than an unqualified source record relabeled as writer proof. A stale optional record therefore does not grant unconditional reuse.

The optional proof still converts resource or writer failure into absence of a proof, preserving the prior opening behavior. Package-class validation, namespace parsing, related-part loads and identifier checks retain their existing order. The ZIP fallback does not receive this hint. Later staged publication and carried-record installation retain their guards.

Separating the two hints addresses the stated failure mechanism: a producer child can have a successful actual read but no staged writer equivalence, while a separate existing writer memo has independently generated normalized child bytes. Discarding that writer memo forced a whole-child rewrite during opening-proof construction. The new parameter permits reuse only through the established writer qualification, without weakening source admission.

## Tests and evidence limits

The two counted regressions are not loosened. The seven A9 variants, independent byte openings, per-step semantic oracles, canonical-block-proof-off reference insertion and per-pass bounds remain intact. The `is_multiple_of` edits preserve the alternating step behavior.

At inspection, `/private/tmp/s954-a9-native-count-writer-carry.log` was compiling and contained no completed test result. The current test configuration uses `cfg(test)`, debug assertions disabled and a package-specific optimization level of one. It is not installed-release timing evidence. Separate debug checks with the enlarged stack are still pending. The reported 52/202 and 27/102 serialized-child regressions motivate this repair, but source inspection does not establish that their reruns pass.

Completed native count and debug controls, installed-package release timings, memory, native oracles and final integrated gates remain required. No previous failed timing result is superseded by this review.

## Exact source identity

- `crates/rdocx/src/document.rs`: `42681ae506bef41193558e42c27b554bdc7faf3efb9670c96b9959d4aab3d74c`.
- `crates/rdocx/src/content_control.rs`: `baea0de088d90910c7e3e9a3525d8fd11fb7c86b135d386124c6f14f416dfc18`.

The reviewer ran no builds or tests and changed no source.
