# F-283, all, pass 1

**Reviewed**: Working diff against b0d351dccbcb36ffb36697ee1da3c2403d564708, 16 files, 3101 insertions and 198 deletions. Frozen source manifest ec94bf3bf6faa7a652eca2b3ff861db334dc8cd875c3b9c3f34ad684db91b372 and patch 119c6b59a313ac4813bd1ad5a0e7e4783fad0ba80beff94a621a5bed9622f72e authenticated before and after inspection.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, Suppressed STYLEREF numbering bypasses the position switch

`crates/rdocx/src/field.rs:14303`

When the selected styled paragraph has effective numId zero, the numbered branch returns Resolved zero immediately. It never reaches the supported p switch at line 14316. A main-story numId-zero Heading1 followed by STYLEREF Heading1 with n and p therefore evaluates and publishes zero rather than zero followed by above. The ordinary renderer takes the same suppression as a value at `crates/rdocx-layout/src/engine.rs:8212` and then appends the position at line 8219, so saved-cache evaluation and actual painting disagree on this same source. For a related-story caller, the early return also bypasses the incomparable-story refusal at field.rs14317. Keep suppression as the base value and apply the common position handling. A focused zero-number plus position discriminator should cover both the main-story result and related-story refusal. No native equivalence claim is required to establish this internal disagreement.

### D2, Generated exclusions introduce quadratic work into ordinary layout

`crates/rdocx/src/field.rs:1702`

The new eager facade bridge visits every paragraph and searches the complete generated-span vector at lines 1703-1705. For each accepted run it again searches that same vector through toc_source_position_is_owned at lines 1712-1722 and `crates/rdocx/src/field.rs:8668`. N valid disjoint one-paragraph generated owners in physical order cause at least 1+2+...+N span membership comparisons before the run checks. N further ordinary paragraphs each search all N spans as well. These are legal disjoint owners, so overlap or malformed-owner rejection does not bound this case.

This is newly reachable from every ordinary build_layout_input through `crates/rdocx/src/document.rs:36616`, including layout, rendering and the new read-only outline. It is distinct from any pre-existing complexity during an explicit TOC rebuild. There is no small generated-span cardinality bound, and a package byte bound does not prevent quadratic amplification. Reuse the scanner's ordered physical positions to restrict checks to relevant spans instead of repeatedly searching all owners. A bounded-work control over many disjoint owners should distinguish this path without a wall-clock threshold. The independent provenance reviewer confirmed this same resource defect and reported no additional freshness defect.

## Smells

0 found.

## Nitpicks

0 found.

## Not found

No additional actionable correctness, contract, panic, OOXML, test or structural finding was established. The new outline is additive, legacy accessors retain their signatures, and the changes reuse existing numbering, cascade and physical source providers. The generated-exclusion bridge validates the exact parsed main document and physical part before transferring only exclusions or failure status into regenerated sequence facts. Result-local caller IDs join through physical paths. Ordinary malformed cache paint, strict new outline refusal and STYLEREF cache retention remain distinct. No new dependency, module, trait, generic or public struct-field break was introduced.

## Evidence and boundary

Read the final worker plan, progress and cited HLD contracts, including the new doc-hidden hooks, direct marker vanish, generated-owner failure policy and TOC selector distinctions. The frozen tests retain the original regression prefix, and the physical-caller Base control is not described as a defect. Authenticated actual receipts and stdout/stderr hashes record 23 focused regression passes, 5 unit passes, affected two-crate clippy, formatting and prose success. The post-test filter_map-to-filter/map amendment preserves the same predicate, order and success-only allocation. Subsequent plan changes are six checklist markers. These observations were inspected, not rerun.

All 16 frozen source hashes and the exact patch remained unchanged. The only reviewer write is this report. No builds, tests, native UI, product changes or remediation were performed. Actual Word lifecycle and hidden-marker qualification, full integrated suites, installed consumers, WASM, 49 hashes, 7 pixels and package/archive riders remain explicitly pending. Missing historical output bodies are not credited as native evidence. Unperformed final gates are not additional local defects.
