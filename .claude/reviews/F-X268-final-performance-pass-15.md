# F-X268, final performance, pass 15

**Reviewed**: Frozen working diff against `c71560d29d1969ce8079ee3b15dfd2303dddf1ae` on `sprint/s95.4`. Two existing files, 158 insertions and 54 deletions. Final source and evidence review of the comment lookup batch under the canonical workflow and approved F-X268 contract. This continues passes 13 and 14 without expanding product scope.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. The local comment batch is review-clean. Final feature acceptance remains incomplete.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Source assessment

The only algorithm-adjacent change since pass 14 moves the successful read counter after namespace classification and release of the borrowed namespace result at `crates/rdocx/src/document.rs:26884`. The stored `before` and `after` remain the reader positions around the same actual read. Namespace classification does not consume another XML event. The failed-read hook and original error remain unchanged. This resolves the borrow-check failure without omitting read work or changing event processing.

Pass 13 D4 remains resolved: the marker XML reader counts successful and failed attempts, and the actual range qualification, membership, hidden-owner and attribute candidates remain instrumented. The disjoint source-order cursor and overlapping/unordered fallback retain the pass 14 boundary assessment. Pure eligibility guards precede membership without changing attribute errors or marker ordinal processing. The optional batched discovery still falls back to the original scanner and incremental ambiguity ordering.

The matching comment paragraph and companion IDs, independent cold openings, all seven variants, per-step semantic oracles and actual old-prefix sensitivity assertion remain intact. No new findings in correctness, contract, panics, OOXML, error ordering or structure. No public API, cache field, module or source file was added. The HLD matches current intent and does not claim final release acceptance.

## Evidence checked

Consumed existing artifacts without running tests or builds. The driver `/private/tmp/s954-comment-batch-and-symbols.json` records successful count and debug builds, native correctness, integration ranges, scoped clippy and diagnostic symbol build, plus count exit zero and completed status. Its source hashes match this reviewed batch.

The actual log `/private/tmp/s954-a9-native-count-marker-cursor.log` records `body_lookup_work_is_linear_at_250_and_1000` passing: one test, zero failures, 624.91 seconds. Its successful exit includes the final actual old-prefix sensitivity assertion, rather than only the seven operation loops.

Independently checked `/private/tmp/s954-a9-marker-cursor-count-receipt.json` against the raw log. The log SHA-256 matches. All 28 raw vectors match the receipt's 14 small/large cold/warm pairs. Recomputed every unit of the union of counter keys, including zero baselines: none exceeds five times the small value. The largest recorded ratio is approximately 4.464 for block-control warm work. This qualifies the instrumented native lookup count gate, not wall-clock linearity or memory use.

The native correctness log records 21 passes, zero failures and one explicitly ignored external Word UI oracle in 0.70 seconds. The integration log records five passes in 6.07 seconds: atomic invalid moves, nested control endpoints, selected proofing-pair removal, comment endpoint/reference preservation and cross-story reopen endpoints. Scoped clippy completed successfully. `/private/tmp/s954-comment-final-hash.log` now ends with all 49 hash entries matching. Prose, adapter and any separately planned sensitivity checks are not inferred from these logs.

## Limits and exact identity

The count profile uses native tests with package optimization level one, debug assertions disabled and an 8 MiB stack. Separate debug tests supply the correctness observations above. The optimized symbol artifact is diagnostic, not the installed release wheel, and its sample supplies no release timing acceptance.

Installed v6 still has five flat and three linear failures. Native Word evidence, memory measurements and final integrated gates remain pending. This local review does not complete F-X268, authorize a release or supersede those failed timings.

Exact SHA-256 values:

- `crates/rdocx/src/document.rs`: `b8e8f8e8f0b97a7072095f90171a31db6f4f06fec53845210d9dfb25cae00d22`.
- `docs/hld/12-testing-strategy.md`: `3322052c061476cb32cff047276599d9856ef0517cf4a507e2cd46b83f3cfdfa`.

The reviewer changed no source and ran no builds or tests.
