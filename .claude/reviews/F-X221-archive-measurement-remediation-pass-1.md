# F-X221 archive measurement remediation, pass 1

**Reviewed**: Five numeric metadata carriers on HEAD `70f88606a235d6dec18a30917e9e6ed87bd9d39e`,8 insertions and8 deletions. The four README rows and four catalog tuples are the entire working diff. Only this report was written by the reviewer.

**Verdict**: Zero defects, zero smells, zero nitpicks.

## Measurement correctness

The actual clean22 dry-run receipt records exit0 and no upload. Its physical log rehash matches `eefb69b9eb9e0b311c085722cc370bf53fa0b1b81f2f461b5ab4b8968d3a2a44`. The independently supplied member proof rehashes to `0018c541dade531712621d4a2a136a5a8c4723a1be5098ecb283feec4cbcd81c`. It contains22 distinct packages. For every package, the normalized member-size sum and member count reproduce its actual tuple.

The changed member/count pairs are oxml-chart704519/6, oxml-opc434232/12, rdocx51778085/37 and rpptx3831296/16. They match the README rows and catalog. The oxml-chart recorded compressed110676 differs from the clean observed110667 by9 bytes, inside the existing64-byte tolerance. The other changed compressed observations are107730,3702533 and715751. Root's separate application record truthfully selects the actual rdocx3702533 rather than retaining3702480, whose53-byte difference would leave only11 bytes of existing tolerance. No tolerance is increased.

All five edited files retain exactly their prior UTF-8 byte lengths. The README self-member deltas are therefore zero. The actual proof's raw-versus-normalized VCS size delta is zero for the clean archives. `scripts/readme_doctests.py:1293` still measures actual gzip bytes and normalizes only generated VCS dirty state and SHA for member totals. Its strict member/count equality,10 MiB archive ceiling and64-byte compressed tolerance remain unchanged at line1340. Package versions, dates, measurement command, generated manifests, lockfiles and production behavior are unchanged by this diff.

## Evidence boundary

The active four affected-archive rerun has replaced the proof's temporary archive paths with successors. Their current hashes are therefore not represented here as the original clean70f8 archive hashes. This review authenticates the retained original measurement proof and actual clean dry-run log, and the exact numeric application. Current metadata-bearing archive verification remains a separate actual gate. No runtime, full-suite, original100 or native appearance credit follows from numeric metadata edits.

Zero correctness, contract, threshold, provenance-routing, test-strength or structure findings. This review ends here and releases the review-file write lease.
