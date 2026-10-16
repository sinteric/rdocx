# F-X240, integration amendment, pass 1

**Reviewed**: Canonical working diff against `9cdbea00f18ca3ec84853a6bf86b2f8c1f6ce9cd`, three files, three insertions and three deletions. Diff SHA-256 `00dbcd1791ce49180bc0310795b9195eb8c017a00c5a39499c73592c1c2c82e3`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Evidence and boundaries

The assertion at `crates/rpptx-render/src/lib.rs:4738` now matches the existing binding dependency at `crates/rpptx-py/Cargo.toml:33` and the approved wrapper feature contract at `.claude/plans/F-X240-design.md:144`. The surrounding native-default, optional-renderer and dependency-direction assertions at `crates/rpptx-render/src/lib.rs:4718` remain intact. This repairs the stale test expectation without changing manifests or production behavior.

The actual predecessor failure is retained at `/private/tmp/S94-full-native-9cdbea-1/changed-default-tests.log:5788`. The focused dependency-direction test and formatting check both exited zero, authenticated against their closed logs in `/private/tmp/S94-native-test-remediation-1/focused-receipt.json`. This is focused evidence, not a successful full native floor.

The independently hashed genuine archives from both `/private/tmp/S94-native-test-remediation-1/archive-measurement-1/receipt.json` and `archive-measurement-2/receipt.json` measure `(84664, 461174, 8)` using the unchanged existing normalization at `scripts/readme_doctests.py:1292`. Round 2 contains the exact current source and README. Its archive SHA-256 is `124dc2a01e8ea95663271e877cb0d62d37973a759424e0820dc8a2610a9075a7`. The README row at `crates/rpptx-render/README.md:21` and catalog at `scripts/readme_doctests.py:417` match. Only those numeric values changed, with the original compression tolerance, exact-member checks and size ceiling preserved. These were actual no-verify package measurements. Verified publication of the successor package remains pending in the full gate.

The complete production source prefix before the test module is byte identical to HEAD, SHA-256 `0d589a2c68d04ef93ae37224e9fb534a29f89c69209c57f22799ae3d817f5ac4`. All other changes are archive measurement metadata. This establishes unchanged relevant non-test source inputs, not newly built binary identity. Prior hosted, installed, math, corpus and CLI evidence remains evidence of its original producer SHA, with narrowly unchanged runtime applicability. It is not fresh execution at the amendment SHA. Any earlier whole-file closure containing this test-bearing source needs this explicit test-only qualification. The old affected renderer archive cannot carry forward. Fresh full native verification, current catalog checks and verified package publication remain required after the amendment is committed.

## Not found

- **Correctness**: 0 findings. Exact manifest expectation and independently measured carrier values agree.
- **Contract**: 0 findings. Approved wrapper feature admission is represented without changing native defaults.
- **Panics**: 0 findings. No production input handling or panic path changed.
- **OOXML**: 0 findings. No parsing, serialization, namespace or child-order change.
- **Tests**: 0 findings. Actual predecessor RED and focused successor GREEN support the repaired assertion. Full verification remains pending.
- **Structure**: 0 findings. No new construct, dependency, feature, module or product file.

Review complete. No remediation was performed.
