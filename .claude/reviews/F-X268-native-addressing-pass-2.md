# F-X268, native addressing, pass 2

**Reviewed**: Codex root reviewed the pass 1 remediation and the additional unsupported-shape discriminator on worker HEAD `6579e2e0dcc9625eaead146d7c324117d43c3bca`. The native file diff is 1491 added and 25 removed lines. This remains a bounded native review, not the complete F-ID microscope.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D3, multiple physical body owners are collapsed by the cached layout

`crates/rdocx/src/document.rs:7804`
`crates/rdocx/src/document.rs:7829`

The full scan selects the first body for item extraction, while child_scan removes every body owner from child records. The layout then supplies only its frame body. It never establishes that the original scan had exactly that one body owner.

The native writer accepts paragraph A, preserved raw bytes `</w:body><w:body>`, and paragraph B. The old scanner reports body index 0 at 369..430 and body index 1 at 430..656. The cached layout instead reports one body at 369..656 using the first fingerprint. In debug the agreement assertion panics. Release omits the second owner.

`main_body_layout_multiple_body_owners_use_full_reader` is an actual failing discriminator in `/tmp/fx268-m6-shape-probe.log`, exit 101, 0 passed and 1 failed in 0.02s. Product code was unchanged for that probe.

This shape is outside the cache's single-body representation. Establish body-owner and frame correspondence before accepting either whole or independent block scans. An unsupported shape must take the old full path and preserve its accepted owners, order, fingerprints and errors. Do not add raw XML admission validation or turn the old successful reader result into a refusal.

## Resolved findings

D1 is resolved. Coverage is checked once for ordered owners and once for ordered items. Child extraction uses partition points. The test-private work counter measures sweep iterations at execution sites, and the 250/1000 regression meets the 5 times bound. The iterator helper has two existing concrete instantiations, the owner iterator and the item iterator.

D2 is resolved. Counter-only declarations are configuration-gated, and the actual release group compiled and ran. The initial release check reported E0282 for cache_off and for an earlier recorded vector in the same F-ID. The latter was also corrected. The compiler did not name sample_deltas in that recorded failure.

The retained remediation evidence is scoped clippy exit 0, debug 6 passed in 20.71s, and release 6 passed in 1.01s. The later shape probe changed only a test and does not invalidate the product source underlying those six cases. It adds the separate failing D3 observation.

## Smells

None.

## Nitpicks

None.

## Not found

No additional finding was established in the inspected sweep, serialized-fact invalidation or exact-byte cache-carry paths. Python validation, HLD alignment, complete provenance and composition coverage, mechanisms 7 and 8, final measurements, Word qualification, hash harness and full sprint gates remain pending. No complete feature acceptance is claimed.
