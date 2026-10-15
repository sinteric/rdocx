# F-X207, installed fixture lifecycle, pass 7

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

The actual current paired-wheel attempt reports 13 passes and two failures, with no skips. Original Normal, Shape, fit, strict-size, read-only snapshot and both physical-provider controls passed. Source and installed members remained unchanged. The failures stop before exercising the intended acceptance assertions. No typing or complete feature pass is claimed.

## Defects

1. crates/rpptx-py/tests/test_documented_examples.py:7007 retains the first shape handle across add_textbox at line7008. The structural insertion invalidates that handle. The assertion at line7023 raises StaleElementError instead of checking both physical owners. Capture the scalar first shape ID before insertion and use it in the unchanged owner assertions.
2. crates/rpptx-py/tests/test_documented_examples.py:7093 replaces the late frame paragraph graph, invalidating its shape handle. Line7094 reuses that stale handle before the intended late-unfit atomicity assertion. Re-fetch the existing late shape after structural text replacement, preserving the complete original unfit input and atomicity assertions.

## Smells

None.

## Nitpicks

None.

## Not found

No production autofit defect is demonstrated by these failures. Neither finding justifies relaxing the structural revision contract or reducing acceptance. The original failed receipt90de62bf2e6b291cf7f7087934aba296b82a0cf9feffded428f6ed9ec21e8f8e and raw pytest log6741c4ce3c35aaf6c8e336c6ee05324ceec35eb1546df2834579b503d20db95e remain immutable. Only this review file was written during review. Return to separate fixture remediation and acceptance renewal against the unchanged wheel inputs.
