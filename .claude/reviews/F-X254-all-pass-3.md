# F-X254, all, pass 3

**Reviewed**: final integrated remediation against e6fbff4ecb8ae00b63f3b28a94b8d1416778057c, 3 changed files, 109 insertions and 13 deletions. This is a separate review of new final-gate remediation after the clean pass2.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, nested selected Fallback loses the existing modeled owner traversal

`crates/rpptx/src/lib.rs:1776` admits only qualified PresentationML shape kinds beneath an immediate MC Fallback. It omits a nested qualified MC AlternateContent. The existing provider explicitly supports that edge: `crates/rpptx-oxml/src/shape_tree.rs:2618` allows descent from Fallback, and `crates/rpptx-oxml/src/shape_tree.rs:2623` descends into its immediate AlternateContent.

For an existing shape-tree AlternateContent whose Fallback contains another AlternateContent and a selected fallback shape, the nested wrapper becomes unmodeled and all its descendants stay unmodeled. Adding an invalid direct `a:bogus` child to that actual selected `p:sp` then produces no finite-frontier island. The unchanged wrapper routes, policy attributes and namespace meaning still satisfy retained MC metadata comparisons. Thus a supported nested fallback owner can escape both the new checked-replacement admission and imported frontier diagnostics, contrary to the approved four-frontier contract and the new HLD's promise to follow the existing provider.

Preserve opaque Choice delegation while following the provider's nested selected-Fallback AlternateContent edge. A meaningful regression should retain the existing nested wrapper metadata, demonstrate that the selected nested shape's new invalid child refuses, and pair it with a valid same-owner typed edit. This does not require general MC interpretation or a recursive schema validator.

## Smells

0.

## Nitpicks

0.

## Other aspects checked

The rich fixture now supplies an actual namespace declaration for its original Requires=p14 token, instead of relaxing genuine unbound-prefix refusal. The ordinary unsupported Choice remains delegated. Root transition wrappers are no longer flattened into ordinary slide particles. Existing MC policy route, physical-owner and resolved namespace comparisons remain in place, along with recognized-namespace invalid-child refusal.

The F216 fixture installs the same actual custom definitions before invoking checked style selection. Payload-based table lookup selects the intended second source table and first destination table despite layout placeholders. Original collision, default, repeated-import, image-backed, relationship and opaque-source assertions remain. No style availability rule is weakened.

The added focused regression covers one-level fallback admission and invalid-child refusal, opaque Choice, copied/rebound MC metadata, root transition delegation and genuine unbound Requires. It does not exercise D1's nested selected-provider route. No additional panic, structural or public-surface finding arose from this three-file diff. No new module, dependency or generic framework is introduced. HLD06 remains an appropriate impact file, subject to D1.

## Evidence and custody

Frozen manifest SHA256 85f057830d4177499f10542c1734bc74585f9e2bc31c15274d60438b2abb7cd9, actual binary diff SHA256 24720a95161dc9b0f92a681ebd9359a171167b4796471b55d4ca21d0bfc7fb2f and readiness SHA256 afa6b4afe500118c7d5df98b12a3bd9e78030006ba22eb812de738b77dcb3649 match the assigned snapshot. All 7 frozen source/reference entries matched before and after review. All 6 referenced evidence receipts and the integration, style, native and clippy raw-log hashes authenticate.

The first affected integration command honestly exits101 with24PASS and1FAIL. The corrected table-fixture-only command exits0 with1PASS. The focused native command records6PASS, clippy exits0, and format/diff receipts report success. Runtime production bytes are frozen across the applicable observations while the two fixture epochs are explicitly distinguished. This is inspected author evidence, not reviewer execution and not a simultaneous25PASS rerun or final workspace/oracle acceptance.

No tests, builds, native applications or product helpers were executed. No source, test, HLD or progress changes were made. Only this report was written. The review pass ends here and the canonical write lease returns to root for a distinct remediation phase.
