# F-X268, in-memory reopen, pass 1

**Reviewed**: Mechanism 5 working diff against 7fc95c22b68d1761cb2c4c042eb55b5033b5913a, five files, 1044 insertions and 79 deletions. Root inspected the complete native implementation and test changes, the counter exports, and the Python consumer regression. This is a bounded mechanism review.
**Verdict**: 0 defects, 1 smell, 0 nitpicks

## Defects

None found in the bounded production diff. Eligibility requires an opened-byte baseline and literal unbounded read limits. The OPC trusted-baseline check remains in reread. Main bytes, name and namespace frame guard carried model and owner facts, and current relationships additionally guard the canonicalization record. Related models and identifier categories are checked from the current package. Identifier definitions replay in sorted-part and document order, with preceding duplicates retaining priority over a later parse error. The new opening owns concrete current records rather than a chain of prior openings. The pending-hold miss remains.

## Smells

### S1, the Python work gate depends on another parameterized case

`crates/rdocx-py/tests/test_core.py:14035`

The module-global sample dictionary makes the 20-paragraph node return before any work-cost assertion unless the 80-paragraph case has also run. Running the 80-paragraph node alone instead fails because 20 is absent. A selected healthy test can fail, while the small selected test can miss a work regression. Collect both workloads inside one test with local samples, assert each warm sample, and then compare the matching identifier-byte ordinals. Keep functional checks for both sizes when debug counters are absent. Do not require global test order.

## Nitpicks

None.

## Evidence and limits

The corrected always-ZIP baseline and the separate namespace-scan RED are retained. The native namespace batch passed 11 cases, atomic refusal passed one, the four fallback, error-order, identifier and cache-off cases passed, and the strengthened retype test passed. A retyped header now demonstrably exposes comment ID 9 in the release-capable functional assertions. Affected clippy passed. The fresh extension SHA-256 is 8d3b0c03b401d2f22ec385a2b86d45bfafac0bfc42427ede460aaeb307c2d42b. Six Python cases passed in 189.46 seconds. Both header workloads produced reopen samples [0, 0, 645] and staging samples [0, 0, 0, 0, 0, 0]. The last two staging slots remain unimplemented child-counter debt. The Python test exercises set_story_text, not paragraph assignment. Configurable read limits are native coverage, not a new Python API.

Fix S1 and verify only the affected test before the local checkpoint. HLD and actual evidence-name adoption remain pending. This is not the full F-ID microscope. Forms, settings and mail-merge composition, full A3 and A5, mechanisms 6 through 8, release fallback execution, memory, timing and the final integrated gates remain pending. Byte equality proofs, model serialization, copying and other XML validation are separate work.
