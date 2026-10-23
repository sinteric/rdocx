# F-283, all, pass 2

**Reviewed**: Frozen working diff against b0d351dccbcb36ffb36697ee1da3c2403d564708, 16 files, 3249 insertions and 198 deletions. This pass examines the four-path remediation against pass 1 and its integration with the unchanged reviewed contract. Frozen manifest c471065d6661f501f27f0a8b814d99cf2d786bc3ff14d6d1b70a6fcda4e1c550, whole patch 8b1707391f5d7134cde7754fec31d390e50478f1b5644389b138048c3ddd162e and remediation patch f96d9479e7780f0f734efa67065948a309b5e2285d6f9637ef0b40855a416463.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass 1 D1 and D2 are closed.

## Defects

0 found.

## Smells

0 found.

## Nitpicks

0 found.

## Prior findings

### D1, closed

`crates/rdocx/src/field.rs:14338`

Suppressed numbering now supplies the base zero value at line 14346 and reaches common position handling at line 14352. Main-story above/below handling and the incomparable-story refusal are no longer bypassed. The new source-built regression at `crates/rdocx/tests/regression_test.rs:405562` separately checks the main result and related-story refusal while retaining unchanged package bytes. The retained old-candidate observation reports Resolved zero for both, with a real failing assertion. The fixed-source affected regression receipt records success. These are local internal-consistency observations, not native Word parity claims.

### D2, closed

`crates/rdocx/src/field.rs:1711`

The paragraph window advances each cursor monotonically. Per-run partition_point at line 1747 searches ordered result-end positions, then invokes the unchanged ownership predicate on at most one candidate at line 1756. The existing scanner sorts physical starts and rejects overlapping owners at lines 6708-6716, so disjoint spans have the end ordering required by the lookup. Removing CITATION preserves that order. Exclusive result ends, same-paragraph gaps and partial first/last paragraphs retain the original predicate at lines 8703-8725. The prior simple-owner empty interval is not reinterpreted. Packed owners cost logarithmic lookup per accepted run rather than a whole-window scan.

The test-only thread-local counter instruments actual owner comparisons without a production knob. Dispersed and packed controls assert meaningful cache exclusion plus retained genuine headings, alongside work bounds at `crates/rdocx/src/field.rs:17168` and line 17198. Their actual old-candidate failures retain 10272 and 3420 comparisons for 64 owners. The current unit batch passes both bounds. The independent boundary reviewer likewise found no remaining D2 correctness, resource or freshness defect. This closure concerns the newly eager bridge and does not claim that all pre-existing explicit table-rebuild work is linear.

## Not found

Correctness, contract, panic, OOXML, tests and structure were checked for the repair and its unchanged integration context. No additional actionable finding was established. The changes preserve source stamps, regenerated counters, source-qualified joins, existing ownership semantics and public transaction boundaries. No new runtime dependency, module, trait, generic, public field or counter was introduced. The plan and HLD03 now describe the bounded lookup and common zero-value handling without asserting final native acceptance.

## Evidence and scope

Authenticated all seven referenced receipt hashes and their stdout/stderr hashes. Actual current-source observations record 7 unit passes, 10 affected regression passes, rdocx clippy, formatting and prose success. The old-candidate D1 and D2 commands genuinely failed and remain retained. The other twelve files are byte-identical to pass 1. Prior layout checks and unchanged-case observations remain attributed to their actual earlier source epochs. No command was rerun by this reviewer.

All 16 frozen file hashes and the whole patch matched before and after this pass. Pass 1 remains unchanged. The only reviewer write is this report. Actual Word lifecycle and hidden-marker qualification, integrated full suites, installed consumers, WASM, 49 hashes, 7 pixels and package/archive riders remain pending. This clean local review does not award those gates or final feature completion.
