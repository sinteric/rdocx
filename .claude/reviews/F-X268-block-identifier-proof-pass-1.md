# F-X268, block identifier proof, pass 1

**Reviewed**: Codex root reviewed the bounded mechanism 7B3 identifier cache on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The current three-file working diff contains 1801 added and 66 removed lines, including the earlier reviewed prerequisites. This pass covers per-child and frame definitions, the optional identifier hint, its package-scan consumer and the focused evidence.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Evidence examined

The child cache records definitions from exact immutable bytes under matching root declarations, body declarations and body bindings. A failed proof is unavailable, not an empty definition list. Frame proof requires no frame definitions before child vectors can be composed. Nonempty frames and failed or mismatched contexts retain the authoritative whole-part scan. Category and exact-byte checks guard hint consumption. Definitions still pass through the existing ordered define and duplicate bookkeeping, after parts are sorted as before. A failed child scan falls back without substituting a new earlier error.

Counters record actual wrapped input bytes on child and frame misses. They do not count only the output definitions or omit the wrapping frame. Warm neighbors retain their proved vectors. Model copying, vector copying and byte equality checks remain separate work, with no universal constant-time claim.

`/tmp/fx268-m7-identifiers-finish-check.log` records a passing scoped check. `/tmp/fx268-m7-identifiers-finish-test.log` records 20 passed and 0 failed in 35.33s. Warm identifier input is 1308 bytes at both measured sizes, delta 0. The forced whole-parser and scanner reference reads 4874 versus 17524 identifier bytes. Saved bytes, models and identifiers agree. Whole-part parse and namespace bytes remain zero on the optimized path, while canonicalization bytes still grow.

The duplicate-order fixture first performs a valid staged reopen, establishing in-memory eligibility before inserting duplicate bookmarks. It then attaches the invalid later header and compares the optimized and block-disabled reopens. The main-part duplicate error wins on both. The earlier concern that this case stayed on the ZIP path is refuted by that actual warmup and the read_package eligibility rule. The opaque bookmark and preserved drawing control also passes. Existing category, reopen and addressing regressions are in the batch.

Earlier failures were fixture or compilation errors, not product RED. Both implementations dropped the incomplete drawing. The initial duplicate fixture failed during opening before reaching its intended comparison. The missing mutable bindings and Debug-bound error assertions were test-code compile failures.

## Not found

No identifier-value, physical-order, namespace-context, checked-fallback, error-priority, provenance, counter-unit, panic or structural defect was established in this bounded inspection.

## Limits

Current identifier-source release typecheck, clippy and release behavior are pending. The installed Python extension remains the mechanism 6 build. Canonicalization, complete native A10 including section and list cases, Python rebuild, HLD, whole-feature composition, timing, memory, Word and final gates remain pending. This is not mechanism 7 acceptance or a whole-feature microscope.
