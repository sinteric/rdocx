# F-X254, all, pass 4

**Reviewed**: cumulative integrated remediation against e6fbff4ecb8ae00b63f3b28a94b8d1416778057c, 3 changed files, 147 insertions and 14 deletions. This distinct pass reviews pass3 D1 and the final affected material.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0. Pass3 D1 is resolved.

## Smells

0.

## Nitpicks

0.

## Remediation assessment

`crates/rpptx/src/lib.rs:1768` now admits the qualified AlternateContent edge from either a supported shape-tree/group owner or an already modeled selected Fallback. The complete condition remains subordinate to the parent's modeled status. Therefore nested selected fallback shapes retain finite admission and physical-owner bookkeeping, while an opaque Choice does not acquire modeled descendants. This matches the existing provider's selected-Fallback nesting at `crates/rpptx-oxml/src/shape_tree.rs:2618` and `crates/rpptx-oxml/src/shape_tree.rs:2623`.

The new regression at `crates/rpptx/src/lib.rs:40278` pairs a valid same-owner text edit with newly invalid direct shape-child refusal and imported-frontier detection inside two existing compatibility wrappers. Wrapper metadata is unchanged, so the negative demonstrates finite shape validation rather than merely triggering the new-complex-MC refusal. The retained old-source log fails at this precise invalid-child assertion after its positive edit passes. Current focused controls pass.

The rest of the cumulative remediation retains the previously inspected intent: root transition MC is delegated, unsupported Choice remains opaque, selected supported fallback shapes are checked, genuinely unbound Requires still refuses, and copied/rebound metadata remains subject to physical-owner/route/namespace custody. Recognized p:/a: children cannot acquire authority through Ignorable. There is no new recursive grammar, public API, dependency or source-owner bypass.

The rich fixture's added actual p14 namespace and the F216 definition-before-selection construction remain unchanged from pass3. The latter retains original collision/default/repeated-import/image-backed relationships and source assertions. Payload-based table lookup still selects the intended actual table owners. HLD06 accurately includes nested selected fallback traversal. No further correctness, contract, panic, OOXML, test or structural finding arose from the cumulative three-file diff.

## Source-qualified evidence

Readiness SHA256 45f902ce220feb338cf17c6455287500976d71294b02ee023a9075cbc60632e9, freeze SHA256 316c03c18db2b40cc636ba1cc9ec109ca790c5a7d20f5f6e327038f9e6a94321 and actual binary patch SHA256 0fc701a4a8f5949302d97f48aa5961473f168994ac1a8d67eb79a99aa8098558 match the assigned final snapshot. All 8 frozen entries and all 30 declared private evidence artifacts authenticate. Frozen entries remain unchanged after review.

Retained raw logs show the newly added regression actually fails on the previous production epoch, then current focused native7PASS and selected corpus1PASS. Scoped clippy and format exit0, with the diff check retained separately. Source and selected proxy/resolved toolchain identities are recorded before and after the scoped commands. These are inspected author observations, not reviewer reruns.

Earlier24integration PASS at fixture epoch1 plus1F216 PASS at corrected fixture epoch2 retain their original source attribution. The current nested traversal change is qualified by the focused7 and corpus1 observations. Neither this report nor the author progress relabels earlier observations as a fresh current25PASS command. Full workspace, installed wheel, CLI, publication, rendering and external/native appearance acceptance remain the final integrated union's pending obligations.

No tests, builds, compilers, native applications or product helpers were executed. No source/test/HLD/progress changes were made. Only this report was written. The review ends and the canonical writer lease returns to root. No confirmation pass is needed absent new material.
