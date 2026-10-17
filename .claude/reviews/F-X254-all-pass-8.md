# F-X254, all, pass 8

**Reviewed**: bounded test-only amendment in crates/rpptx-py/tests/test_documented_examples.py, against e02c90ff381f549b73008d216ec7da44cd0088f5.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Assessment

The invalid zero-height expectation at `crates/rpptx-py/tests/test_documented_examples.py:2011` now requires the approved precise ValueError and message, retaining saved-byte atomicity and all valid dimension/held-owner checks.

The opaque fixture at `crates/rpptx-py/tests/test_documented_examples.py:9561` wraps only shape and slide payloads in their valid qualified extLst/ext hooks. The actual foreign opaque payload, entity spelling and namespace-bearing attribute remain unchanged. Existing alias/default-root, text/table preservation, saved/reopened payload, exact no-op and foreign-root refusal assertions remain. The other seven parameter kinds are untouched. This reconciles the older fixture with the newly approved direct-frontier grammar without weakening source custody or admitting a foreign direct child.

The retained original installed failure stays failed. Actual focused JUnit records exactly3PASS/0FAIL/0SKIP for dimension, shape and slide. Installed before/after snapshots match. No product/stub/native/wheel change belongs to this F254 amendment. The separately frozen Word stub amendment is excluded from this review's scope.

Authenticated the readiness5 artifact set, original receipt/log pins, current combined patch and all6combined frozen entries. Only this report was written. No tests, imports, builds or native work occurred. Review ends and returns its writer lease to root after the paired review completes.
