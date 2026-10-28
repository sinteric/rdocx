# F-X268, final performance, pass 10

**Reviewed**: Frozen working diff against `461eee97e1aa58b460bce4658f5a58453eaf9351` on `sprint/s95.4`. Six files, 1467 insertions and 146 deletions. F-X268 remains in progress with its approved contract. Read the canonical workflow and contract in the preceding passes and inspected the current remediation against pass 9. Product logic is unchanged from the pass 8 audit.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. This is a source review, not a completed acceptance gate.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Resolved findings

### D1, per-action semantic proof and control coordinate systems

The cell and resize checks continue to inspect every relevant physical occurrence after each sampled action. Comment checks verify both endpoints and run indices. Counter vectors are frozen before the semantic oracles execute.

The inline control oracle at `crates/rdocx/src/document.rs:49751` now resolves the destination paragraph snapshot, checks the same story, and verifies that this paragraph's actual XML contains the selected control tag after the destination text. It no longer interprets flat projected indices as hierarchical paths. The independently selected tag remains unique, and the control's identity, type and payload are compared with the pre-action control. Alternating destination paragraphs and the original distinct host make a skipped move fail the containment check.

The block oracle at `crates/rdocx/src/document.rs:49769` locates the selected control in the actual modeled body. It checks the final direct slot on end moves and the immediately following destination paragraph on before-paragraph moves. These are the correct direct-body coordinates for the two destinations. It no longer compares flat projection indices with modeled body length.

D1 is closed by source inspection.

### D2, cold isolation

The independent byte openings at `crates/rdocx/src/document.rs:49861` remain present for every family. No source-fixture parsed-state or main-body-scan Arc is cloned across families. Own target acquisition stays outside the counter window. D2 remains closed.

### D3, picture identity and immediate insertion semantics

The insertion oracle beginning at `crates/rdocx/src/document.rs:49661` retains the original public operation and captures its returned location. It requires exactly one new body paragraph immediately after the requested destination. Removing that paragraph must restore the entire prior body, which rejects append-at-end and unrelated-body-edit counterexamples.

The returned location must resolve to a paragraph snapshot with the exact inserted direct-body index. The independent picture inventory must contain exactly one picture at that physical body slot, with the requested dimensions and original PNG bytes. The snapshot implementation derives `direct_body_index` from physical span containment at `crates/rdocx/src/document.rs:27685`, so this comparison does not repeat the flat-index error. Inventory count must grow by one on every action. The oracle no longer treats the media filename as drawing-name metadata. D3 is closed by source inspection.

## Other aspects checked

No additional findings in product correctness, contract scope, panic handling, OOXML preservation, error ordering or structure. The unchanged product batch retains the prior review of immutable namespace sharing, borrowed actual parsed children, complete frame and late-child admission, independent writer proof, resource checks, private-field and context fallbacks, and all physical-cell-route consumers.

The test-only pause helper now requires both `test` and `debug_assertions` at `crates/rdocx/src/document.rs:7447`. Its call sites remain in debug-only reference comparisons. Mandatory production work and the empty-frame qualification remain counted when debug assertions are disabled. The seven operation variants, 250/1000 inputs, per-pass five-times bounds and actual old-algorithm sensitivity controls remain unchanged.

## Source identity and evidence limits

- `crates/rdocx/src/document.rs`: `e459e68f9b82ab54325f1103aa3b8a546fb7c71e5534b1221e7bbf9b4b2c08a5`.
- `crates/rdocx/src/content_control.rs`: `baea0de088d90910c7e3e9a3525d8fd11fb7c86b135d386124c6f14f416dfc18`.

At final inspection, `/private/tmp/s954-a9-native-count-pass9.log` had entered controlled-cell warm 1000 and contained no completed full-test result. Partial vectors do not establish a seven-family pass. The current run uses the native test profile with optimization level zero and debug assertions disabled. It is not a release timing measurement. The stopped obsolete release/LTO attempt is not count evidence.

Separate debug agreement checks, completed native count qualification, installed-package release timings, memory measurements, native Word evidence and final integrated gates remain pending. The previous V5 timing misses are not resolved by this review. No builds or tests were run by the reviewer, and no source was changed.
