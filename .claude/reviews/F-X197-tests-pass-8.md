# F-X197, tests, pass 8

**Reviewed**: R14 to R8 scalar-getter migration in one existing PowerPoint test carrier, three additions and three deletions. The author is independent of this reviewer.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Read the approved live colour getter and scalar RGB migration contract, HLD10, the actual R14 terminal traceback, the full test and concrete native getters. All 318 current input hashes, sizes and modes match the frozen receipt. The complete three-line inverse reproduces the R14 carrier hash. All other 317 guarded inputs are unchanged. No production, manifest, installed package or wheel changed.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract: the getter at crates/rpptx-py/src/text.rs:1440 returns the approved colour format. Its scalar getter at crates/rpptx-py/src/dml.rs:525 returns None when unset and constructs RGBColor for direct RGB. The initial and reopened property lists at crates/rpptx-py/tests/test_documented_examples.py:1359 and crates/rpptx-py/tests/test_documented_examples.py:1379 now inspect that scalar and retain all seven other property reads. The expected reopened RGBColor matches the assigned channels and saved XML.

Tests: all eight attributes, direct assignment, save/reopen, exact colour and font XML, five underline variants and the direct None-clearing loop remain. The final empty-property XML assertion at crates/rpptx-py/tests/test_documented_examples.py:1405 remains unchanged. This migrates the approved API expectation without removing its source assertions.

Panics, OOXML and structure: this test-only delta adds no production indexing, parsing, XML mutation, dependency, module or abstraction. Zero findings in these categories.

## Execution qualification

Actual R14 Word passed all 354 cases. Actual R14 PowerPoint passed 582 and failed this legacy unset-getter assertion. The original failure remains immutable. This review grants no corrected runtime or completion credit. Reuse unaffected Word evidence only through explicit unchanged-source and tool equivalence. Rerun the affected PowerPoint checks and retain the original R10 wheel identity. Only this report was written and this review ends here.
