# F-X268, opened staging identifiers, pass 2

**Reviewed**: Native D2 repair against 68dc8b67a10844b8b9a65a613a9448db56bc49da, one source and test file, 102 insertions and no deletions. Root inspected the full diff and confirmed it stayed unchanged through the reported checks.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None. D2 is closed. A nonempty pending set for the current main owner now misses the unchanged-main shortcut. The unchanged whole canonicalization performs the cleanup. The three native controls distinguish an expired hold, a still-live unreferenced hold and a now-referenced header hold. They require equal results, model, serialized XML, package and identifiers, and explicitly check which holds survive. No public API reproducer is claimed.

## Smells

None in this bounded repair.

## Nitpicks

None.

## Evidence and limits

`/tmp/fx268-d2-native.log` records eight passing native cases in 29.61 seconds, including warm reuse, cache-off growth, serialization admission, byte equivalence, related parts and ordered resource errors. Format and affected clippy passed. The fresh Python extension has SHA-256 92a324caca6d0e5f1cbc31bb3cd4dacea93fc627085d537be223c1fa1e42e25f. Four Python cases passed in 132.75 seconds in `/tmp/fx268-d2-pytest.log`. Warm staging samples remained zero and repeated add_run piece reads stayed one at both sizes. The archive is `/private/tmp/s95.4-fx268-d2-pending-miss-_rdocx.abi3.so`.

This closes the identifier finding, not the full F-ID microscope. Mechanisms 5 through 8, full A3 to A5 composition, release fallback proof, memory, timing and the final integrated union remain pending. Whole-model serialization and copying are not zero-work claims.
