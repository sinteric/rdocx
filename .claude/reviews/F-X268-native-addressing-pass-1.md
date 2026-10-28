# F-X268, native addressing, pass 1

**Reviewed**: Codex root reviewed the dirty native addressing implementation in `crates/rdocx/src/document.rs` on worker HEAD `6579e2e0dcc9625eaead146d7c324117d43c3bca`. The file diff is 1309 added and 25 removed lines. This is a bounded native implementation and test review, not the complete F-ID microscope.
**Verdict**: 2 defects, 0 smells, 0 nitpicks

## Defects

### D1, the coverage guard adds a quadratic whole-scan walk

`crates/rdocx/src/document.rs:7739`
`crates/rdocx/src/document.rs:7789`

For a cold document with N paragraph children, scan_body_children extracts N requested children. Each child_scan call walks the complete owner and item arrays to check crossing spans. The item array has at least N entries, and valid input makes every any predicate traverse that array. Coverage validation therefore does N squared metadata comparisons after the full XML scan. The child-scan counter records N and does not reveal those repeated comparisons. This contradicts the bounded-work purpose of the implementation. Prove coverage once for the complete scan, or use a bounded search, while retaining rejection of spans crossing raw block boundaries.

### D2, debug-only samples leave unconstrained vectors in release tests

`crates/rdocx/src/document.rs:43776`
`crates/rdocx/src/document.rs:43863`

The reopen regression declares cache_off and sample_deltas unconditionally with Vec::new. Every use that supplies their element type is under cfg(debug_assertions). With that configuration disabled, the declarations have no inferable element type and the release test target cannot type-check. Gate the declarations with the same configuration while keeping functional assertions in release. This is a source finding. No release compiler result has been recorded for this revision.

## Smells

None.

## Nitpicks

None.

## Not found

No additional correctness, panic, namespace-preservation or structural finding was established within the inspected native addressing and cache-carry paths. Exact serialized frame and child bytes are checked before carrying scans. The current cache stores XML scan facts and does not retain prior document objects.

The five main_body_layout tests passed in `/tmp/fx268-m6-native-tests.log`, with 0 failures in 15.71s. The native check passed in `/tmp/fx268-m6-native-check.log`. Those debug results do not refute D1 or D2. The tests compare with the old scanners, including physical locations, fingerprints, mixed owners and raw boundaries.

Python consumer validation, complete private-provenance and composition coverage, HLD alignment, release execution, final timing and memory measurements, Word qualification, hash harness and final sprint verification remain pending. The serializer prerequisite has its own review and is not reapproved here.
